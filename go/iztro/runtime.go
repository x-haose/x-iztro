package iztro

import (
	"context"
	"crypto/sha256"
	_ "embed"
	"encoding/hex"
	"encoding/json"
	"os"
	"path/filepath"
	"runtime"
	"sync"
	"sync/atomic"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

// wasm 运行时：模块编译一次、实例按需铺开。
//
// wasm 模块实例持有自己的线性内存，alloc/free 不能跨实例交叉，因此并发调用
// 各用一个实例而非共享一个加锁。实例数上限为 GOMAXPROCS——wasm 调用是纯计算，
// 并发度超过核数只会增加内存占用。
//
// 编译产物（机器码）落盘缓存在用户缓存目录下，进程再次启动时直接读取，
// 省去每次约百毫秒的编译；缓存目录按 wasm 内容哈希分桶，换了 wasm 自然换桶。

//go:embed x_iztro.wasm
var wasmBytes []byte

// fnSlot 为 wasm 导出函数在实例内的槽位。
type fnSlot int

const (
	// fnBySolar 为阳历排盘入口
	fnBySolar fnSlot = iota
	// fnByLunar 为农历排盘入口
	fnByLunar
	// fnHoroscope 为运限入口
	fnHoroscope
	// fnQuery 为通用查询入口
	fnQuery
	// fnCount 为槽位总数
	fnCount
)

// wasmExports 为各槽位对应的 wasm 导出函数名。
var wasmExports = [fnCount]string{
	fnBySolar:   "iztro_wasm_by_solar",
	fnByLunar:   "iztro_wasm_by_lunar",
	fnHoroscope: "iztro_wasm_horoscope",
	fnQuery:     "iztro_wasm_query",
}

// instance 为一个 wasm 模块实例及其导出函数；单个实例非并发安全。
type instance struct {
	mod   api.Module
	alloc api.Function
	free  api.Function
	fns   [fnCount]api.Function
}

// engine 持有 wazero 运行时、编译产物与实例池。
type engine struct {
	wa       wazero.Runtime
	compiled wazero.CompiledModule
	// idle 为空闲实例队列
	idle chan *instance
	// slots 为实例总数额度，写入一个元素即占用一个实例名额
	slots chan struct{}
	// cacheDir 为编译缓存目录；未启用缓存时为空串
	cacheDir string
}

var (
	activeEngine atomic.Pointer[engine]
	engMu        sync.Mutex
)

// getEngine 取全局引擎，首次调用时编译 wasm 模块。
func getEngine(ctx context.Context) (*engine, error) {
	if e := activeEngine.Load(); e != nil {
		return e, nil
	}
	engMu.Lock()
	defer engMu.Unlock()
	if e := activeEngine.Load(); e != nil {
		return e, nil
	}
	e, err := newEngine(ctx)
	if err != nil {
		return nil, err
	}
	activeEngine.Store(e)
	return e, nil
}

// newEngine 建立 wazero 运行时并编译 wasm 模块。
func newEngine(ctx context.Context) (*engine, error) {
	cfg := wazero.NewRuntimeConfig()
	cacheDir, cache := compilationCache()
	if cache != nil {
		cfg = cfg.WithCompilationCache(cache)
	}
	wa := wazero.NewRuntimeWithConfig(ctx, cfg)
	if _, err := wasi_snapshot_preview1.Instantiate(ctx, wa); err != nil {
		wa.Close(ctx)
		return nil, internalError("instantiate wasi: " + err.Error())
	}
	compiled, err := wa.CompileModule(ctx, wasmBytes)
	if err != nil {
		wa.Close(ctx)
		return nil, internalError("compile wasm: " + err.Error())
	}
	size := runtime.GOMAXPROCS(0)
	if size < 1 {
		size = 1
	}
	return &engine{
		wa:       wa,
		compiled: compiled,
		idle:     make(chan *instance, size),
		slots:    make(chan struct{}, size),
		cacheDir: cacheDir,
	}, nil
}

// compilationCache 打开落盘的编译缓存，返回目录与缓存句柄；
// 取不到用户缓存目录或建目录失败时返回 ("", nil)，编译退化为每进程一次。
func compilationCache() (string, wazero.CompilationCache) {
	base, err := os.UserCacheDir()
	if err != nil {
		return "", nil
	}
	sum := sha256.Sum256(wasmBytes)
	dir := filepath.Join(base, "x-iztro", hex.EncodeToString(sum[:4]))
	if err := os.MkdirAll(dir, 0o755); err != nil {
		return "", nil
	}
	cache, err := wazero.NewCompilationCacheWithDir(dir)
	if err != nil {
		return "", nil
	}
	return dir, cache
}

// newInstance 实例化一个新的 wasm 模块实例。
// 模块名留空，否则同名实例在同一运行时内会冲突。
func (e *engine) newInstance(ctx context.Context) (*instance, error) {
	mod, err := e.wa.InstantiateModule(ctx, e.compiled, wazero.NewModuleConfig().WithName(""))
	if err != nil {
		return nil, internalError("instantiate wasm: " + err.Error())
	}
	ins := &instance{
		mod:   mod,
		alloc: mod.ExportedFunction("iztro_wasm_alloc"),
		free:  mod.ExportedFunction("iztro_wasm_free"),
	}
	for slot, name := range wasmExports {
		ins.fns[slot] = mod.ExportedFunction(name)
	}
	return ins, nil
}

// acquire 取一个可用实例：优先复用空闲实例，额度未满时新建，否则等待归还。
// ctx 已取消时直接失败——池里有空闲实例也不会绕过这道检查。
func (e *engine) acquire(ctx context.Context) (*instance, error) {
	if err := ctx.Err(); err != nil {
		return nil, internalError("acquire wasm instance: " + err.Error())
	}
	select {
	case ins := <-e.idle:
		return ins, nil
	default:
	}
	select {
	case ins := <-e.idle:
		return ins, nil
	case e.slots <- struct{}{}:
		ins, err := e.newInstance(ctx)
		if err != nil {
			<-e.slots
			return nil, err
		}
		return ins, nil
	case <-ctx.Done():
		return nil, internalError("wait for wasm instance: " + ctx.Err().Error())
	}
}

// release 归还实例；idle 与 slots 容量相同，故不会阻塞。
func (e *engine) release(ins *instance) {
	e.idle <- ins
}

// acquireAll 占满全部实例名额，占满即等于没有在飞调用。
// 中途失败时归还已占的，不留半占状态。
func (e *engine) acquireAll(ctx context.Context) ([]*instance, error) {
	held := make([]*instance, 0, cap(e.slots))
	for i := 0; i < cap(e.slots); i++ {
		ins, err := e.acquire(ctx)
		if err != nil {
			e.releaseAll(held)
			return nil, err
		}
		held = append(held, ins)
	}
	return held, nil
}

// releaseAll 批量归还实例。
func (e *engine) releaseAll(held []*instance) {
	for _, ins := range held {
		e.release(ins)
	}
}

// discard 弃用实例并交还额度。wasm 调用失败可能是 trap，trap 会永久损耗该实例
// 的栈空间，继续复用会让后续合法调用也失败，因此关掉换新的。
func (e *engine) discard(ctx context.Context, ins *instance) {
	ins.mod.Close(ctx)
	<-e.slots
}

// invoke 把入参 JSON 写进实例内存、调用指定导出函数并取回结果 JSON 原文。
func (ins *instance) invoke(ctx context.Context, slot fnSlot, payload []byte) ([]byte, error) {
	mem := ins.mod.Memory()

	allocRes, err := ins.alloc.Call(ctx, uint64(len(payload)))
	if err != nil {
		return nil, internalError("alloc: " + err.Error())
	}
	inPtr := allocRes[0]
	defer ins.free.Call(ctx, inPtr, uint64(len(payload)))

	if !mem.Write(uint32(inPtr), payload) {
		return nil, internalError("write input to wasm memory failed")
	}

	callRes, err := ins.fns[slot].Call(ctx, inPtr, uint64(len(payload)))
	if err != nil {
		return nil, internalError("call " + wasmExports[slot] + ": " + err.Error())
	}

	outPtr := uint32(callRes[0] >> 32)
	outLen := uint32(callRes[0] & 0xFFFFFFFF)
	defer ins.free.Call(ctx, uint64(outPtr), uint64(outLen))

	out, ok := mem.Read(outPtr, outLen)
	if !ok {
		return nil, internalError("read result from wasm memory failed")
	}
	result := make([]byte, len(out))
	copy(result, out)
	return result, nil
}

// callWasm 序列化入参、调用 wasm 并返回结果 JSON 原文；
// wasm 侧的错误信封会转成 *Error 返回。
func callWasm(ctx context.Context, slot fnSlot, input any) ([]byte, error) {
	payload, err := json.Marshal(input)
	if err != nil {
		return nil, internalError("marshal input: " + err.Error())
	}
	e, err := getEngine(ctx)
	if err != nil {
		return nil, err
	}
	ins, err := e.acquire(ctx)
	if err != nil {
		return nil, err
	}
	result, err := ins.invoke(ctx, slot, payload)
	if err != nil {
		e.discard(ctx, ins)
		return nil, err
	}
	e.release(ins)

	var probe struct {
		Error string `json:"error"`
		Code  string `json:"code"`
	}
	if err := json.Unmarshal(result, &probe); err != nil {
		return nil, internalError("decode result: " + err.Error())
	}
	if probe.Error != "" {
		return nil, &Error{Code: probe.Code, Message: probe.Error}
	}
	return result, nil
}

// Warmup 预先完成 wasm 编译与实例池铺满，把首次调用的冷启动开销提前到这里。
//
// 不调用也能正常工作——编译与实例化都是按需惰性完成的；服务进程希望第一个
// 请求就走热路径时在启动阶段调用一次即可。
func Warmup(ctx context.Context) error {
	e, err := getEngine(ctx)
	if err != nil {
		return err
	}
	held, err := e.acquireAll(ctx)
	if err != nil {
		return err
	}
	e.releaseAll(held)
	return nil
}

// Close 等在飞调用结束后关闭 wasm 运行时，释放全部实例内存。
//
// 先把引擎摘下——此后发起的调用会新建一个引擎，不再落到正在关闭的这一个；
// 再占满全部实例名额，占满即证明没有调用还在 wasm 里，此时关闭不会打断任何人。
// ctx 在等待期间被取消时引擎原样装回、不关闭，返回 ErrInternal。
//
// 通常不必调用——运行时随进程结束回收；长期驻留但已不再排盘的进程可以用它
// 归还内存。关闭后再次调用本包任何函数会自动重新初始化。
func Close(ctx context.Context) error {
	engMu.Lock()
	defer engMu.Unlock()
	e := activeEngine.Swap(nil)
	if e == nil {
		return nil
	}
	if _, err := e.acquireAll(ctx); err != nil {
		activeEngine.Store(e)
		return err
	}
	if err := e.wa.Close(ctx); err != nil {
		return internalError("close wasm runtime: " + err.Error())
	}
	return nil
}

// CompilationCacheDir 返回 wasm 编译缓存所在目录；未启用落盘缓存时返回空串。
func CompilationCacheDir(ctx context.Context) (string, error) {
	e, err := getEngine(ctx)
	if err != nil {
		return "", err
	}
	return e.cacheDir, nil
}

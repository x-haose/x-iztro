// Package iztro 提供紫微斗数排盘与运限计算的 Go API。
//
// 实现方式：内嵌 rs-iztro 编译出的 WebAssembly 模块（wasm32-wasip1），
// 通过纯 Go 的 wazero 运行时调用——无 cgo，保留 Go 的交叉编译能力，
// `go get` 即用，无需本机安装 Rust 工具链。
//
// 返回值为 JS iztro 兼容 JSON（camelCase 键 + 按语言翻译的值）解析出的
// map[string]any。计算失败时返回 error（对应 JSON 中的 error 字段）。
//
// 更新内嵌 wasm：在仓库根目录执行
//
//	cargo build --release --target wasm32-wasip1
//	cp target/wasm32-wasip1/release/rs_iztro.wasm go/iztro/
package iztro

import (
	"context"
	_ "embed"
	"encoding/json"
	"errors"
	"fmt"
	"sync"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

//go:embed rs_iztro.wasm
var wasmBytes []byte

// Config 为排盘配置；nil 或缺省键取默认值（与 JS iztro 默认一致）。
// 键：YearDivide/HoroscopeDivide "normal"|"exact"、AgeDivide "normal"|"birthday"、
// DayDivide "forward"|"current"、Algorithm "default"|"zhongzhou"。
type Config struct {
	YearDivide      string `json:"yearDivide,omitempty"`
	HoroscopeDivide string `json:"horoscopeDivide,omitempty"`
	AgeDivide       string `json:"ageDivide,omitempty"`
	DayDivide       string `json:"dayDivide,omitempty"`
	Algorithm       string `json:"algorithm,omitempty"`
}

// runtime 持有一次性初始化的 wazero 实例；wasm 模块实例非并发安全，
// 所有调用经互斥锁串行化。
type runtime struct {
	mu      sync.Mutex
	mod     api.Module
	alloc   api.Function
	free    api.Function
	bySolar api.Function
	byLunar api.Function
	horo    api.Function
}

var (
	rtOnce sync.Once
	rt     *runtime
	rtErr  error
)

func getRuntime() (*runtime, error) {
	rtOnce.Do(func() {
		ctx := context.Background()
		r := wazero.NewRuntime(ctx)
		wasi_snapshot_preview1.MustInstantiate(ctx, r)
		mod, err := r.Instantiate(ctx, wasmBytes)
		if err != nil {
			rtErr = fmt.Errorf("iztro: instantiate wasm: %w", err)
			return
		}
		rt = &runtime{
			mod:     mod,
			alloc:   mod.ExportedFunction("iztro_wasm_alloc"),
			free:    mod.ExportedFunction("iztro_wasm_free"),
			bySolar: mod.ExportedFunction("iztro_wasm_by_solar"),
			byLunar: mod.ExportedFunction("iztro_wasm_by_lunar"),
			horo:    mod.ExportedFunction("iztro_wasm_horoscope"),
		}
	})
	return rt, rtErr
}

// call 将入参 JSON 写入 wasm 内存、调用函数并取回结果 JSON。
func (r *runtime) call(fn api.Function, input any) (map[string]any, error) {
	payload, err := json.Marshal(input)
	if err != nil {
		return nil, fmt.Errorf("iztro: marshal input: %w", err)
	}

	r.mu.Lock()
	defer r.mu.Unlock()
	ctx := context.Background()

	allocRes, err := r.alloc.Call(ctx, uint64(len(payload)))
	if err != nil {
		return nil, fmt.Errorf("iztro: alloc: %w", err)
	}
	inPtr := allocRes[0]
	if !r.mod.Memory().Write(uint32(inPtr), payload) {
		return nil, errors.New("iztro: write input to wasm memory failed")
	}

	callRes, err := fn.Call(ctx, inPtr, uint64(len(payload)))
	if _, ferr := r.free.Call(ctx, inPtr, uint64(len(payload))); ferr != nil && err == nil {
		err = ferr
	}
	if err != nil {
		return nil, fmt.Errorf("iztro: call: %w", err)
	}

	outPtr := uint32(callRes[0] >> 32)
	outLen := uint32(callRes[0] & 0xFFFFFFFF)
	out, ok := r.mod.Memory().Read(outPtr, outLen)
	if !ok {
		return nil, errors.New("iztro: read result from wasm memory failed")
	}
	result := make([]byte, len(out))
	copy(result, out)
	if _, err := r.free.Call(ctx, uint64(outPtr), uint64(outLen)); err != nil {
		return nil, fmt.Errorf("iztro: free result: %w", err)
	}

	var decoded map[string]any
	if err := json.Unmarshal(result, &decoded); err != nil {
		return nil, fmt.Errorf("iztro: decode result: %w", err)
	}
	if msg, ok := decoded["error"].(string); ok {
		return nil, fmt.Errorf("iztro: %s", msg)
	}
	return decoded, nil
}

// BySolar 以阳历日期排盘。
//   - solarDate: "YYYY-M-D"，如 "2000-8-16"
//   - timeIndex: 时辰索引 0-12（0=早子时，12=晚子时）
//   - gender: "male" 或 "female"
//   - fixLeap: 是否修正闰月
//   - language: "zh_cn"/"zh_tw"/"en_us"/"ja_jp"/"ko_kr"/"vi_vn"
//   - config: 排盘配置，nil 取默认
func BySolar(solarDate string, timeIndex uint8, gender string, fixLeap bool, language string, config *Config) (map[string]any, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	return r.call(r.bySolar, map[string]any{
		"solarDate": solarDate,
		"timeIndex": timeIndex,
		"gender":    gender,
		"fixLeap":   fixLeap,
		"language":  language,
		"config":    config,
	})
}

// ByLunar 以农历日期排盘；isLeapMonth 在该月没有闰月时不生效。
func ByLunar(lunarDate string, timeIndex uint8, gender string, isLeapMonth bool, fixLeap bool, language string, config *Config) (map[string]any, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	return r.call(r.byLunar, map[string]any{
		"lunarDate":   lunarDate,
		"timeIndex":   timeIndex,
		"gender":      gender,
		"isLeapMonth": isLeapMonth,
		"fixLeap":     fixLeap,
		"language":    language,
		"config":      config,
	})
}

// Horoscope 计算运限（无状态：直接传出生排盘参数与目标日期）。
func Horoscope(solarDate string, timeIndex uint8, gender string, fixLeap bool, language string, config *Config, targetDate string, targetTimeIndex uint8) (map[string]any, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	return r.call(r.horo, map[string]any{
		"solarDate":       solarDate,
		"timeIndex":       timeIndex,
		"gender":          gender,
		"fixLeap":         fixLeap,
		"language":        language,
		"config":          config,
		"targetDate":      targetDate,
		"targetTimeIndex": targetTimeIndex,
	})
}

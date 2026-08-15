// Package iztro 提供紫微斗数排盘与运限计算的 Go API。
//
// 实现方式：内嵌 x-iztro 编译出的 WebAssembly 模块（wasm32-wasip1），
// 通过纯 Go 的 wazero 运行时调用——无 cgo，保留 Go 的交叉编译能力，
// `go get` 即用，无需本机安装 Rust 工具链。
//
// 返回类型化的 Astrolabe/Horoscope 结构体（见 types.go），字段同时携带
// 翻译文本与语言无关标识（keys.go 常量），判断方法在任何输出语言下可用。
// 计算失败时返回 error。
//
// 更新内嵌 wasm：在仓库根目录执行
//
//	cargo build --release --target wasm32-wasip1
//	cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/
package iztro

import (
	"context"
	_ "embed"
	"encoding/json"
	"errors"
	"fmt"
	"sync"
	"time"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

//go:embed x_iztro.wasm
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

	// AstroType 为排盘视角："heaven" 天盘（默认）、"earth" 地盘、"human" 人盘。
	// 地盘以身宫干支、人盘以福德宫干支起五行局重排。
	AstroType string `json:"astroType,omitempty"`

	// Mutagens 为自定义四化表：天干标识 → 四颗星标识（禄、权、科、忌）。
	// 按天干整表替换默认值，未列出的天干仍用默认表。
	Mutagens map[string][]string `json:"mutagens,omitempty"`

	// Brightness 为自定义亮度表：星耀标识 → 十二宫亮度标识（十二项，
	// 空串表示该宫无亮度，索引 0 为寅宫）。按星耀整表替换默认值。
	Brightness map[string][]string `json:"brightness,omitempty"`
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
	query   api.Function
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
			query:   mod.ExportedFunction("iztro_wasm_query"),
		}
	})
	return rt, rtErr
}

// call 将入参 JSON 写入 wasm 内存、调用函数并取回结果 JSON 原文。
func (r *runtime) call(fn api.Function, input any) ([]byte, error) {
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

	var probe struct {
		Error string `json:"error"`
	}
	if err := json.Unmarshal(result, &probe); err != nil {
		return nil, fmt.Errorf("iztro: decode result: %w", err)
	}
	if probe.Error != "" {
		return nil, fmt.Errorf("iztro: %s", probe.Error)
	}
	return result, nil
}

// BySolar 以阳历日期排盘，返回类型化星盘。
//   - solarDate: "YYYY-M-D"，如 "2000-8-16"
//   - timeIndex: 时辰索引 0-12（0=早子时，12=晚子时）
//   - gender: "male" 或 "female"
//   - fixLeap: 是否修正闰月
//   - language: "zh-CN"/"zh-TW"/"en-US"/"ja-JP"/"ko-KR"/"vi-VN"
//   - config: 排盘配置，nil 取默认
func BySolar(solarDate string, timeIndex uint8, gender string, fixLeap bool, language string, config *Config) (*Astrolabe, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	raw, err := r.call(r.bySolar, map[string]any{
		"solarDate": solarDate,
		"timeIndex": timeIndex,
		"gender":    gender,
		"fixLeap":   fixLeap,
		"language":  language,
		"config":    config,
	})
	if err != nil {
		return nil, err
	}
	var out Astrolabe
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("iztro: decode astrolabe: %w", err)
	}
	out.link()
	return &out, nil
}

// Rearranged 以指定干支为命宫重排本盘，返回新盘；本盘不变。
//
// 传入的干支决定五行局，进而决定紫微天府落点、十二宫名、长生十二神与大限小限；
// 辅星、杂耀（天伤天使天才除外）、博士十二神、岁前将前十二神沿用原盘。
//
// 常规的天盘、地盘、人盘用 Config.AstroType 指定即可，本方法用于从任意干支起盘。
// fromStemKey / fromBranchKey 取 Stem* / Branch* 常量。
func (a *Astrolabe) Rearranged(fromStemKey string, fromBranchKey string) (*Astrolabe, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	raw, err := r.call(r.bySolar, map[string]any{
		"solarDate":  a.SolarDate,
		"timeIndex":  a.TimeIndex,
		"gender":     a.GenderKey,
		"fixLeap":    a.FixLeap,
		"language":   a.Language,
		"config":     &a.Config,
		"fromStem":   fromStemKey,
		"fromBranch": fromBranchKey,
	})
	if err != nil {
		return nil, err
	}
	var out Astrolabe
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("iztro: decode astrolabe: %w", err)
	}
	out.link()
	return &out, nil
}

// ByLunar 以农历日期排盘，返回类型化星盘；isLeapMonth 在该月没有闰月时不生效。
func ByLunar(lunarDate string, timeIndex uint8, gender string, isLeapMonth bool, fixLeap bool, language string, config *Config) (*Astrolabe, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	raw, err := r.call(r.byLunar, map[string]any{
		"lunarDate":   lunarDate,
		"timeIndex":   timeIndex,
		"gender":      gender,
		"isLeapMonth": isLeapMonth,
		"fixLeap":     fixLeap,
		"language":    language,
		"config":      config,
	})
	if err != nil {
		return nil, err
	}
	var out Astrolabe
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("iztro: decode astrolabe: %w", err)
	}
	out.link()
	return &out, nil
}

// Horoscope 以本命盘为起点计算目标日期的运限，返回的运限持有本盘。
//
//   - targetDate: 目标阳历日期，"YYYY-M-D"
//   - targetTimeIndex: 目标时辰索引 0-12
//
// 排盘上下文（出生日期、时辰、性别、闰月修正、语言、配置）取自本盘，
// 与 wasm 侧的无状态调用协定一致。
func (a *Astrolabe) Horoscope(targetDate string, targetTimeIndex uint8) (*Horoscope, error) {
	r, err := getRuntime()
	if err != nil {
		return nil, err
	}
	raw, err := r.call(r.horo, map[string]any{
		"solarDate":       a.SolarDate,
		"timeIndex":       a.TimeIndex,
		"gender":          a.GenderKey,
		"fixLeap":         a.FixLeap,
		"language":        a.Language,
		"config":          &a.Config,
		"targetDate":      targetDate,
		"targetTimeIndex": targetTimeIndex,
	})
	if err != nil {
		return nil, err
	}
	var out Horoscope
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, fmt.Errorf("iztro: decode horoscope: %w", err)
	}
	out.astrolabe = a
	return &out, nil
}

// HoroscopeNow 以本命盘为起点计算**此刻**的运限，日期与时辰取本地时钟。
func (a *Astrolabe) HoroscopeNow() (*Horoscope, error) {
	now := time.Now()
	date := fmt.Sprintf("%d-%d-%d", now.Year(), int(now.Month()), now.Day())
	timeIndex, err := TimeToIndex(uint8(now.Hour()))
	if err != nil {
		return nil, err
	}
	return a.Horoscope(date, timeIndex)
}

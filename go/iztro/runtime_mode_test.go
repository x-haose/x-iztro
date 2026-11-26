package iztro

import (
	"context"
	"encoding/json"
	"runtime"
	"testing"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/api"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

// directModule 是在给定执行方式的独立运行时里实例化的本模块，供逐次调用导出函数。
type directModule struct {
	rt  wazero.Runtime
	mod api.Module
}

func newDirectModule(t *testing.T, cfg wazero.RuntimeConfig) *directModule {
	t.Helper()
	ctx := context.Background()
	rt := wazero.NewRuntimeWithConfig(ctx, cfg)
	wasi_snapshot_preview1.MustInstantiate(ctx, rt)
	mod, err := rt.InstantiateWithConfig(ctx, wasmBytes, wazero.NewModuleConfig().WithName(""))
	if err != nil {
		t.Fatal(err)
	}
	t.Cleanup(func() { rt.Close(ctx) })
	return &directModule{rt: rt, mod: mod}
}

// call 调用一次导出函数，返回结果缓冲的原文。
func (d *directModule) call(t *testing.T, export string, payload []byte) string {
	t.Helper()
	ctx := context.Background()
	ptr, err := d.mod.ExportedFunction("iztro_wasm_alloc").Call(ctx, uint64(len(payload)))
	if err != nil {
		t.Fatal(err)
	}
	d.mod.Memory().Write(uint32(ptr[0]), payload)
	res, err := d.mod.ExportedFunction(export).Call(ctx, ptr[0], uint64(len(payload)))
	if err != nil {
		t.Fatalf("%s: %v", export, err)
	}
	out, ok := d.mod.Memory().Read(uint32(res[0]>>32), uint32(res[0]))
	if !ok {
		t.Fatalf("%s: result buffer out of range", export)
	}
	return string(out)
}

// TestCompilerMatchesInterpreter 同一批入参在编译器与解释器下的输出逐字节相同，盯住编译器这条执行路径。
// arm64 的编译器有已知误编译（wazero/wazero#2590），包在 arm64 上改用解释器，此测试在 arm64 上跳过。
func TestCompilerMatchesInterpreter(t *testing.T) {
	if runtime.GOARCH == "arm64" {
		t.Skip("arm64 上编译器有已知误编译（wazero/wazero#2590），本包在 arm64 用解释器")
	}
	charts := []map[string]any{
		{"solarDate": "2000-8-16", "timeIndex": 2, "gender": "female", "fixLeap": true, "language": "zh-CN"},
		{"solarDate": "1997-3-4", "timeIndex": 1, "gender": "male", "fixLeap": true, "language": "zh-CN"},
		{"solarDate": "1985-6-10", "timeIndex": 12, "gender": "female", "fixLeap": false, "language": "en-US"},
	}
	queries := []map[string]any{
		{"kind": "astrolabeToText", "knowledge": "builtin"},
		{"kind": "horoscopeToText", "targetDate": "2025-1-1", "targetTimeIndex": 0},
		{"kind": "patterns"},
		{"kind": "horoscopePatterns", "scope": "yearly", "targetYear": 2026},
		{"kind": "horoscopeTopicPalaces", "topic": "siblings", "scope": "monthly", "targetYear": 2026, "targetMonth": 10, "knowledge": "builtin"},
		{"kind": "horoscopeTopicToText", "topic": "romance", "scope": "yearly", "targetYear": 2026, "knowledge": "builtin"},
		{"kind": "knowledgeForChart", "knowledge": "builtin", "targetDate": "2025-1-1"},
		{"kind": "decadalList"},
		{"kind": "monthlyList", "year": 2025},
	}
	compiler := newDirectModule(t, wazero.NewRuntimeConfigCompiler())
	interpreter := newDirectModule(t, wazero.NewRuntimeConfigInterpreter())
	for _, chart := range charts {
		in, _ := json.Marshal(chart)
		if a, b := compiler.call(t, "iztro_wasm_by_solar", in), interpreter.call(t, "iztro_wasm_by_solar", in); a != b {
			t.Errorf("by_solar %s: 编译器与解释器输出不同", in)
		}
		for _, q := range queries {
			payload := map[string]any{}
			for k, v := range chart {
				payload[k] = v
			}
			for k, v := range q {
				payload[k] = v
			}
			if payload["language"] != "zh-CN" && payload["knowledge"] != nil {
				continue
			}
			in, _ := json.Marshal(payload)
			a := compiler.call(t, "iztro_wasm_query", in)
			b := interpreter.call(t, "iztro_wasm_query", in)
			if a != b {
				t.Errorf("%s: 编译器与解释器输出不同", in)
			}
		}
	}
}

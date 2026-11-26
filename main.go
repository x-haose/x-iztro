// Usage: go run . repro.wasm input.json
// Calls iztro_wasm_alloc, writes input.json into memory, then calls iztro_wasm_query.
// Exit code 0 when the interpreter succeeds and the compiler traps with out of bounds memory access.
package main

import (
	"context"
	"fmt"
	"os"
	"strings"

	"github.com/tetratelabs/wazero"
	"github.com/tetratelabs/wazero/imports/wasi_snapshot_preview1"
)

func call(cfg wazero.RuntimeConfig, wasm, input []byte) (string, error) {
	ctx := context.Background()
	rt := wazero.NewRuntimeWithConfig(ctx, cfg)
	defer rt.Close(ctx)
	wasi_snapshot_preview1.MustInstantiate(ctx, rt)
	mod, err := rt.InstantiateWithConfig(ctx, wasm, wazero.NewModuleConfig().WithName(""))
	if err != nil {
		return "", fmt.Errorf("instantiate: %w", err)
	}
	if q := mod.ExportedFunction("iztro_wasm_query"); q != nil {
		r, err := mod.ExportedFunction("iztro_wasm_alloc").Call(ctx, uint64(len(input)))
		if err != nil {
			return "", err
		}
		mod.Memory().Write(uint32(r[0]), input)
		res, err := q.Call(ctx, r[0], uint64(len(input)))
		if err != nil {
			return "", err
		}
		b, _ := mod.Memory().Read(uint32(res[0]>>32), uint32(res[0]))
		return string(b), nil
	}
	res, err := mod.ExportedFunction("run").Call(ctx)
	if err != nil {
		return "", err
	}
	return fmt.Sprint(res), nil
}

func main() {
	wasm, err := os.ReadFile(os.Args[1])
	if err != nil {
		fmt.Println(err)
		os.Exit(2)
	}
	var input []byte
	if len(os.Args) > 2 {
		input, _ = os.ReadFile(os.Args[2])
	}
	iOut, iErr := call(wazero.NewRuntimeConfigInterpreter(), wasm, input)
	cOut, cErr := call(wazero.NewRuntimeConfigCompiler(), wasm, input)
	short := func(s string) string {
		if len(s) > 120 {
			return s[:120] + "…"
		}
		return s
	}
	fmt.Printf("interpreter: out=%q err=%v\ncompiler:    out=%q err=%v\n", short(iOut), iErr, short(cOut), cErr)
	if iErr == nil && cErr != nil && strings.Contains(cErr.Error(), "out of bounds memory access") {
		os.Exit(0)
	}
	os.Exit(1)
}

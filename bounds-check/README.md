# Second trigger: failed bounds check under the arm64 compiler

`repro.wasm` is a later build of the same library, compiled from source on a GitHub Actions `ubuntu-24.04-arm` runner. Under wazero's compiler on arm64, calling `iztro_wasm_by_solar` aborts (`wasm error: unreachable`) after a Rust bounds check fails inside `get_minor_stars`. The interpreter, the compiler on amd64, and Node.js all return the chart JSON.

```
go run . repro.wasm input.json iztro_wasm_by_solar
```

Exits 0 when the interpreter succeeds and the compiler traps. The same source built on another machine (different embedded file paths, hence a different data layout) does not trigger it.

# wazero arm64 compiler out-of-bounds reproducer

`repro.wasm` traps with `out of bounds memory access` under wazero's compiler on arm64, while the interpreter, the compiler on amd64, and Node.js all return the expected JSON.

```
go run . repro.wasm input.json
```

Exits 0 when the interpreter succeeds and the compiler traps. See the linked wazero issue for details.

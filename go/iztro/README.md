# iztro (Go)

Zi Wei Dou Shu (紫微斗数, Chinese "Purple Star" astrology) chart engine for Go. The Rust core is embedded
as WebAssembly and run by [wazero](https://wazero.io): no cgo, no Rust toolchain, cross-compilation keeps
working. Results match [iztro](https://github.com/SylarLong/iztro) field for field, with the same
capabilities as the Rust, Python and JavaScript/TypeScript packages.

[Documentation](https://x-iztro.x-haose.com/en/docs/go) ·
[中文文档](https://x-iztro.x-haose.com/zh/docs/go) ·
[GitHub](https://github.com/x-haose/x-iztro)

## Install

```bash
go get github.com/x-haose/x-iztro/go/iztro
```

## Your first chart

```go
package main

import (
	"fmt"
	"log"

	"github.com/x-haose/x-iztro/go/iztro"
)

func main() {
	// time index 2 = Tiger hour (03:00–05:00); 0 = early Rat hour … 12 = late Rat hour
	chart, err := iztro.BySolar("2000-8-16", 2, iztro.GenderFemale, true, iztro.LanguageEnUS, nil)
	if err != nil {
		log.Fatal(err)
	}

	fmt.Println(chart.Palace(iztro.PalaceSoul).MajorStars[0].Key) // ziweiMaj

	text, err := chart.ToText() // structured text an LLM can read
	if err != nil {
		log.Fatal(err)
	}
	fmt.Println(text)
}
```

The first call compiles the WebAssembly module (about 200 ms once, cached on disk afterwards); call
`iztro.Warmup(ctx)` at startup if a service's first request should take the hot path. Calls are safe from
multiple goroutines.

## License

[MIT](https://github.com/x-haose/x-iztro/blob/main/LICENSE)

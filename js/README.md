# x-iztro

Zi Wei Dou Shu (紫微斗数, Chinese "Purple Star" astrology) chart engine for JavaScript and TypeScript. The
Rust core runs as WebAssembly; results match [iztro](https://github.com/SylarLong/iztro) field for field.

- Browsers, Node.js 22+ and Web Workers; ESM, zero runtime dependencies.
- Written in TypeScript with complete `.d.ts` declarations — no `@types/` package needed.
- Same capabilities as the Rust, Python and Go packages: charts, the six horoscope levels, 64 named
  patterns, knowledge packs, reverse lookup, and text an LLM can read.

[Documentation](https://x-iztro.x-haose.com/en/docs/js) ·
[中文文档](https://x-iztro.x-haose.com/zh/docs/js) ·
[Bundlers and frameworks](https://x-iztro.x-haose.com/en/docs/js/bundlers) ·
[GitHub](https://github.com/x-haose/x-iztro)

## Install

```bash
npm install x-iztro
```

## Your first chart

```ts
import { init, astro } from 'x-iztro';

await init(); // loads the WebAssembly core once; every call after this is synchronous

// time index 2 = Tiger hour (03:00–05:00); 0 = early Rat hour … 12 = late Rat hour
const chart = astro.bySolar('2000-8-16', 2, 'female', { language: 'en-US' });

chart.palace('soulPalace')!.majorStars.map((s) => s.key); // ['ziweiMaj']
```

## Text for a language model

```ts
console.log(chart.toText({ detail: 'overview' }));
```

```text
# Natal Chart 2000-8-16 Tiger hour female

## Basic Info
- Solar: 2000-8-16 · Lunar: 2000-7-17 · Hour: Tiger hour (03:00~05:00)
- Pillars: geng chen - jia shen - bing woo - geng yin · Zodiac: dragon · Sign: leo
- Five Elements Class: wood 3rd · Soul Star: rebel · Body Star: scholar
...
```

Horoscopes work the same way, e.g. `chart.horoscopeOfYear(2026).toText()`.

## Entry points

| Import | Contents |
| --- | --- |
| `x-iztro` | The whole API plus `init` |
| `x-iztro/types` | DTO types and enum constants only; importing it never loads the WebAssembly core |
| `x-iztro/x_iztro.wasm` | The core file, for self-hosting or prefetching |

`init()` also accepts a URL, a `Response` or the bytes of `x_iztro.wasm` when you host the file yourself.
Vite, webpack and Turbopack pick up the core file automatically; see
[Bundlers and frameworks](https://x-iztro.x-haose.com/en/docs/js/bundlers).

## License

[MIT](./LICENSE)

<p align="center">
  <a href="https://ziwei.x-haose.com/en"><img src="https://ziwei.x-haose.com/banner-en.png" alt="x-iztro — Zi Wei Dou Shu chart engine. Code casts the chart; AI does the reading."></a>
</p>

<p align="center">
  <a href="https://crates.io/crates/x-iztro"><img src="https://img.shields.io/crates/v/x-iztro?style=flat-square&logo=rust&logoColor=white" alt="crates.io"></a>
  <a href="https://docs.rs/x-iztro"><img src="https://img.shields.io/docsrs/x-iztro?style=flat-square" alt="docs.rs"></a>
  <a href="https://pypi.org/project/x-iztro/"><img src="https://img.shields.io/pypi/v/x-iztro?style=flat-square&logo=python&logoColor=white" alt="PyPI"></a>
  <a href="https://pkg.go.dev/github.com/x-haose/x-iztro/go/iztro"><img src="https://img.shields.io/badge/go.dev-reference-00ADD8?style=flat-square&logo=go&logoColor=white" alt="Go Reference"></a>
  <a href="https://github.com/x-haose/x-iztro/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/x-haose/x-iztro/ci.yml?branch=main&style=flat-square&label=CI" alt="CI"></a>
  <a href="https://ziwei.x-haose.com/en/docs/guide/about/accuracy"><img src="https://img.shields.io/badge/golden_cases-716%2C314-2da44e?style=flat-square" alt="716,314 golden cases"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="MIT"></a>
</p>

<p align="center">
  <a href="https://ziwei.x-haose.com/en">Documentation</a> ·
  <a href="https://ziwei.x-haose.com/en/docs/guide/getting-started">Quick start</a> ·
  <a href="#x-iztro-and-iztro">vs iztro</a> ·
  <a href="#accuracy">Accuracy</a> ·
  <a href="https://github.com/x-haose/x-iztro/blob/main/README.zh-CN.md">中文文档</a>
</p>

x-iztro is a Zi Wei Dou Shu (紫微斗数, Chinese "Purple Star" astrology) chart
engine: a Rust core with native Python and Go bindings. It turns a birth date
and hour into a complete, semantically labeled chart — twelve palaces, ~100
stars with brightness and transformations, six horoscope levels, 64 named
patterns, swappable reading texts. Every piece carries a translated name plus a
stable language-independent key, and one call renders any of it as structured
text an LLM can read.

> Casting a chart is arithmetic, not interpretation. A language model gets some
> of the arithmetic right and quietly gets the rest wrong, and from the output
> you cannot tell which. x-iztro does the arithmetic deterministically, then
> hands the model the part it is actually good at: **reading** the chart.

```python
from x_iztro import Astro

astro = Astro()
# time_index 2 = Tiger hour (03:00-05:00); 0 = early Rat hour ... 12 = late Rat hour
chart = astro.by_solar("2000-8-16", 2, "female", language="en-US")
print(chart.to_text())
```

```text
# Natal Chart 2000-8-16 Tiger hour female

## Basic Info
- Solar: 2000-8-16 · Lunar: 二〇〇〇年七月十七 · Hour: Tiger hour (03:00~05:00)
- Pillars: geng chen - jia shen - bing woo - geng yin · Zodiac: dragon · Sign: leo
- Five Elements Class: wood 3rd · Soul Star: rebel · Body Star: scholar
- Soul Palace: woo · Body Palace: xu (career) · Original Palace: chen (spouse)
- Birth-Year Mutagen: sun [A]→children, general [B]→wealth, moon [C]→friends, fortunate [D]→health

## Palace Overview
| Palace | Major Stars | Minor Stars | Decadal |
|---|---|---|---|
| **soul** woo | emperor([+3]) | artist([-3]) | 3-12 |
| siblings si | advisor([-1]) | — | 13-22 |
| spouse chen [Original Palace] | marshal([+3]) | helper, impulsive([-3]) | 23-32 |
(the other nine rows omitted)

## Patterns
- **Empress and Minister Facing the Palace** (soul): empress([+3]), minister([+3])

## Palaces

### soul (ren woo) · Decadal 3-12
- Major Stars: emperor([+3])
- Minor Stars: artist([-3])
- Adjective Stars: refined, lucky, intercepted, instigated, considery(Y)
- Trine & Opposite: Opposite surface · Trine wealth, career
- Stem ren Flying: sage [A]→children, emperor [B]→soul, officer [C]→career, general [D]→wealth
- Twelve Gods: Changsheng·weak, Boshi·dragon, Suiqian·downcast, Jiangqian·disastery
- Age Fortune Years: 5, 17, 29, 41, 53, 65, 77, 89, 101, 113

... (the other eleven palaces)
```

Legend: `([+3])` is brightness on a −3…+3 scale; `[A]`–`[D]` are the four
transformations (四化, "mutagens" in iztro's vocabulary) and `→children` names
the palace the transformed star sits in. The lunar date stays in Chinese
numerals by design. The same chart renders in six
languages; the underlying objects are typed, and `chart.to_json()` emits every
key JS iztro emits, with the same values, plus documented extension keys.

## x-iztro and iztro

Chart math is ported from the JavaScript [iztro](https://github.com/SylarLong/iztro)
and held identical to it; the layers an AI pipeline needs on top are new.

|                                                              | iztro v2.5.8 (JS) | x-iztro                                   |
| ------------------------------------------------------------ | ----------------- | ----------------------------------------- |
| Chart, twelve palaces (body and Original palace included), six horoscope levels | ✅ | ✅ field-for-field identical, 716,314 golden cases |
| Palace queries, surrounded palaces (三方四正), flying stars  | ✅                | ✅                                        |
| Serialized output                                            | `JSON.stringify`  | every iztro key, key for key, plus documented extension keys |
| Pattern judgement (64 rules)                                 | —                 | ✅                                        |
| Knowledge packs (reading texts, school attributes)           | —                 | ✅                                        |
| Reverse lookup (BaZi pillars / chart features → birth dates) | —                 | ✅                                        |
| Semantic text projection (to_text)                           | —                 | ✅                                        |
| Calling languages                                            | JS / TS           | Rust / Python / Go                        |
| Configuration                                                | global singleton  | passed per chart, no global state         |
| Invalid input                                                | throws            | typed errors with machine-readable codes; never panics on invalid input |

On JS or in the browser, use iztro — it is the original. On a backend, or when
the chart is headed for an LLM, use x-iztro.

## Install

**Rust** — MSRV 1.88 (checked by a dedicated CI job), four direct dependencies
(21 crates in the full tree). The core forbids `unsafe` via a crate-level
`#![deny(unsafe_code)]`; only the FFI/wasm boundary modules are exempt.

```toml
[dependencies]
x-iztro = "0.3"
```

**Python** — 3.10+, zero runtime dependencies, typed API (dataclasses +
StrEnum). Prebuilt abi3 wheels for Linux x86_64/aarch64 (manylinux), macOS
universal2 and Windows x64 — no Rust toolchain needed there. Other targets
(musl/Alpine, 32-bit) fall back to the sdist and compile with one.

```bash
pip install x-iztro
```

**Go** — no cgo: the Rust core ships inside the module as a ~1.5 MB
WebAssembly blob run by [wazero](https://wazero.io), a pure-Go runtime.
Cross-compilation works as usual.

```bash
go get github.com/x-haose/x-iztro/go/iztro
```

## Quick start

**Python**

```python
from x_iztro import Astro
from x_iztro.enums import MajorStar, Mutagen, PalaceName

# time_index 2 = Tiger hour; output defaults to zh-CN, pass language="en-US" for English
chart = Astro().by_solar("2000-8-16", 2, "female")
print(chart.chinese_date, chart.soul, chart.five_elements_class)

# Predicates take language-independent keys, so they answer the same
# no matter which language the chart was rendered in.
soul = chart.palace(PalaceName.SOUL)
print(soul.has([MajorStar.ZIWEI]), soul.has_mutagen(Mutagen.LU))

horoscope = chart.horoscope("2024-1-1", 0)
print(horoscope.yearly.heavenly_stem, horoscope.yearly.earthly_branch)
```

**Rust**

```rust
use x_iztro::{by_solar, Config, Gender, IztroError, Language};

fn main() -> Result<(), IztroError> {
    let chart = by_solar(
        "2000-8-16",        // solar birth date
        2,                  // hour index: 0 = early Rat ... 12 = late Rat
                            // (the Rat hour straddles midnight, hence two indices)
        Gender::Female,
        true,               // fix_leap: split leap months at the midpoint
        Language::ZhCN,
        Config::default(),  // boundaries and school; defaults match JS iztro
    )?;

    println!("{} / {}", chart.lunar_date, chart.chinese_date);
    // These fields are language-independent keys; Debug prints the variant name.
    // Use translate_star(chart.soul, lang) etc. for display text.
    println!("{:?} {:?}", chart.soul, chart.five_elements_class);

    let horoscope = chart.horoscope("2024-1-1", 0)?;
    println!("{:?}", horoscope.yearly.mutagen);
    Ok(())
}
```

**Go**

```go
package main

import (
    "fmt"
    "log"

    "github.com/x-haose/x-iztro/go/iztro"
)

func main() {
    // args: date, hour index (2 = Tiger hour), gender, fixLeap, language,
    // config (nil = defaults, same as JS iztro). Context variants: BySolarContext.
    chart, err := iztro.BySolar("2000-8-16", 2, iztro.GenderFemale, true, iztro.LanguageZhCN, nil)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(chart.ChineseDate, chart.Soul, chart.FiveElementsClass)

    soul := chart.Palace(iztro.PalaceSoul)
    fmt.Println(soul.Has(iztro.StarZiweiMaj), soul.HasMutagen(iztro.MutagenLu))

    horoscope, err := chart.Horoscope("2024-1-1", 0)
    if err != nil {
        log.Fatal(err)
    }
    fmt.Println(horoscope.Yearly.HeavenlyStem, horoscope.Yearly.EarthlyBranch)
}
```

All three bindings call the same Rust core, so the same birth data yields the
same answers everywhere — that equivalence is itself under test.

## Beyond iztro

iztro computes the chart. x-iztro also derives what the tradition reads into
it — the semantic layers an AI pipeline (or an app) needs on top of raw star
positions.

### Pattern judgement — 64 rules

```python
chart = astro.by_solar("1985-5-3", 9, "male", language="en-US")

for hit in chart.patterns():
    print(hit.name, "|", hit.palace_name, "|", hit.variant, "|", hit.broken)
```

```text
General and Wolf Together | surface | None | False
Empress and Minister Facing the Palace | soul | soul_empty | False
Marshal, Rebel and Wolf | surface | None | False
Money and Horse Galloping Together | soul | surround | False
Officer and Helper Flanking Life | soul | None | False
Literary Nobility and Brilliance | surface | None | False
Literary Stars Facing Life | soul | None | True
Literary Stars in Hidden Support | soul | opposite | False
Literary Stars in Hidden Support | soul | surround | False
```

Not a lookup table: each hit carries the palace it formed in, the reading
variant that matched, a broken-pattern flag, and the stars that triggered it —
auditable, and language-independent via `hit.key`. The `surface` hits sit in
the travel palace because that is where this chart's body palace is:
soul-or-body patterns record the palace they actually formed in. The two
Hidden Support lines are one pattern forming two distinct ways, told apart by
`variant`. Natal charts and horoscope views share one rule set. How the rules
are evaluated, plus the full table of 64 patterns:
[documentation](https://ziwei.x-haose.com/en/docs/guide/concepts/patterns).

### BaZi reverse — four pillars back to birth dates

```python
from x_iztro import solar_dates_by_bazi

# every solar birth moment in 1900-2100 with the pillars
# geng-chen jia-shen bing-wu geng-yin (庚辰 甲申 丙午 庚寅)
for c in solar_dates_by_bazi(
    ("gengHeavenly", "chenEarthly"), ("jiaHeavenly", "shenEarthly"),
    ("bingHeavenly", "wuEarthly"), ("gengHeavenly", "yinEarthly"),
):
    print(c.solar_date, c.time_index)
```

```text
1940-8-31 2
2000-8-16 2
2060-8-1 2
```

A set of pillars recurs roughly every 60 years, so multiple hits are the
normal case; the pillars are interpreted under the chart configuration's
boundary conventions.

### Chart reverse — chart features back to birth dates

```python
from x_iztro import reverse_chart, ReverseCriteria, StarPosition

# soul palace in 午, body palace in 戌, wood-3rd class,
# Ziwei in the soul palace, birth-year Lu on the Sun
result = reverse_chart(ReverseCriteria(
    soul_branch="wuEarthly",
    body_branch="xuEarthly",
    five_elements_class="wood3rd",
    stars=[StarPosition("ziweiMaj", "wuEarthly")],
    mutagens=("taiyangMaj", None, None, None),
    year_range=(1995, 2005),
))
for c in result.candidates:
    print(c.solar_date, c.time_index)
```

```text
2000-2-11 8
2000-2-19 8
2000-2-21 8
2000-8-6 2
2000-8-14 2
2000-8-16 2
```

The criteria are exactly this README's sample chart — its own birth moment
(2000-8-16, Tiger hour) is among the candidates. Both entry points work by
pruned enumeration, then re-cast every surviving candidate in full, so each
result provably charts back to the target. Constrained queries answer in
milliseconds; sweeps across the whole 200-year range are typically sub-second,
and overly loose criteria stop at a candidate limit with a `truncated` flag.
Details: [documentation](https://ziwei.x-haose.com/en/docs/guide/guides/reverse).

### Knowledge packs — the interpretation layer, swappable

The core only judges facts; how to *interpret* them differs by school (a
lineage of tradition). So reading texts and school-specific star attributes
live in a swappable JSON pack, not in the code. A default pack ships inside —
107 stars, 64 patterns, 12 palaces, 4 transformations, 49 glossary entries,
adapted from [iztro-docs](https://docs.iztro.com) (MIT, Sylar Long) — and any
entry can be replaced with an overlay pack. It doubles as a ready-made RAG
corpus, **in Chinese**: the built-in pack is zh-CN only (note the quote below
stays Chinese even on an English chart), so English products bring an overlay.

```python
from x_iztro import KnowledgePack

pack = KnowledgePack.builtin()
chart = astro.by_solar("2000-8-16", 2, "female", language="en-US")

for hit in chart.patterns():
    print(hit.name, "|", pack.pattern(hit.key).quotes[0])
```

```text
Empress and Minister Facing the Palace | 府相朝垣命必荣
```

Pack format, merge rules, and how to write an overlay:
[documentation](https://ziwei.x-haose.com/en/docs/guide/guides/knowledge-pack).

### LLM output in practice

- `chart.to_text()` projects the whole chart into Markdown (basic info, a
  palace overview table, patterns, the twelve palaces in detail);
  `chart.horoscope("2024-10-1", 0).to_text()` does the same for the horoscope
  at a date; palaces, surrounded palaces and pattern hits each have their own
  to_text. The format is deterministic with a stable field order, and the text
  snapshots guarding it are part of the test suite.
- `chart.to_text(knowledge=True)` inlines readings picked by chart next to the
  facts (each palace's star readings after that palace, pattern readings after
  the pattern list, mutagen notes at the end), sourced from the bundled
  knowledge pack or your own pack merged from overlays — the core assembles,
  it holds no opinion of its own. Rust:
  `to_text_with(&TextOptions::new().knowledge(&pack))`; Go:
  `ToTextWith(iztro.TextOptions{Knowledge: iztro.BuiltinKnowledge()})`.
- A full natal-chart text is ~3.4k characters in zh-CN and ~6.8k in en-US —
  on the order of 2–3k tokens, varying by model.
- Even for an English-facing product, consider feeding the model the **zh-CN**
  chart: models know the Chinese star names, while the English names are
  iztro's own gloss vocabulary (`considery`, `dissipated`) that models mostly
  don't recognize. Feed the Chinese chart and ask for answers in English.
  Rationale and wiring, including exposing the library as a tool call:
  [LLM guide](https://ziwei.x-haose.com/en/docs/guide/guides/llm). No MCP
  server yet — the tool-call recipe there covers the same integration.
- The documentation site is itself LLM-readable:
  [`/llms.txt`](https://ziwei.x-haose.com/llms.txt),
  [`/llms-full.txt`](https://ziwei.x-haose.com/llms-full.txt), and any docs
  page with `.md` appended.

## Accuracy

Every number is checked against the JavaScript
[iztro v2.5.8](https://github.com/SylarLong/iztro) (version-pinned) with zero
tolerance for differences. That is a reproducibility standard, not a claim
that any one school is the only correct one — schools genuinely differ, which
is what the configuration switches and custom transformation/brightness tables
are for. And you can re-run the whole comparison yourself:

```bash
cargo test                                               # every layer except Tier 3, about a minute
cargo test --release --test golden_tier3 -- --ignored    # Tier 3 in full: 586,430 charts, ~70 s
```

716,314 golden cases in nine layers ("60 years" below means 1984–2043, one
sexagenary cycle):

| Layer       | Cases   | Coverage                                                                  |
|-------------|---------|---------------------------------------------------------------------------|
| Tier 1      | 1,560   | 60 years × 13 hours × both genders, every field compared individually     |
| Tier 2      | 37,440  | 60 years × the 1st and 15th of each month × 13 hours × both genders       |
| Tier 3      | 586,430 | **every day** of 60 years × 13 hours × both genders, plus a second `fix_leap` pass on leap-month days; hashed |
| Edge years  | 46,228  | 1583–1983 and 2044–2100 sampled, where leap months and tables strain      |
| Horoscope   | 5,760   | 360 charts × 16 target dates, all six horoscope levels, every field       |
| Variants    | 14,268  | lunar-date charts across leap months, Zhongzhou school, all six languages |
| Config      | 9,696   | each boundary switch at its non-default value, chart and horoscope layers |
| Astro type  | 12,488  | the heaven / earth / human chart perspectives                             |
| 1602 window | 2,444   | the 1602 leap-month correction window, day by day, hashed                 |

On top of that: the serialization contract is compared key-by-key against JS
`JSON.stringify`; translation reverse-lookup is checked against iztro's `kot`
(its key-of-translation helper) on 1,559 entries; and the three bindings are
cross-checked so the same birth data yields the same answers in Rust, Python
and Go.

**One example of what zero tolerance means.** The Rust lunar-calendar
dependency has a self-contradictory month table for the year 1602 (a 31-day
second month), which made one date panic outright and 28 more days silently
produce wrong charts. x-iztro adds a correction layer at its single
lunar-table entry point, with the true values cross-confirmed against three
independent sources — lunar-typescript, the Shou-Xing almanac (sxtwl) and the
Korea Astronomy and Space Science Institute's tables — and the window locked
by 2,444 golden cases. A full-domain scan of every date from 1583 to 9999
(~6.1 million charts) found no second window of the same kind. The layer
probes the dependency at runtime and disables itself once upstream is fixed.

## Running it in production

- **No global state** — configuration and language travel with each call, so
  charts under different schools or locales never interfere.
- **Invalid input is an error, never a panic** — date format and existence are
  validated, solar years 1583–9999, hour index 0–12. Rust returns
  `Err(IztroError)`, Python raises `x_iztro.IztroError` (a `ValueError` with a
  `.code`), Go returns an `error` matchable with `errors.Is`, and the C FFI
  yields error JSON. Every failure carries a machine-readable category.
- **Python** — ~0.5 ms per chart (Apple Silicon dev machine). Charts are
  frozen dataclasses, safe to share across threads, but the compute holds the
  GIL — scale with processes, not threads. Serialize with `to_dict()` /
  `to_json()`, not `dataclasses.asdict()` (charts hold back-references).
- **Go** — concurrency-safe: a pool of wasm instances (capped at
  `GOMAXPROCS`, one linear memory each) serves goroutines in parallel. First
  call compiles the embedded wasm (~100–200 ms, or 20–30 ms with the on-disk
  cache under `os.UserCacheDir()`; without one, each process recompiles).
  Steady state is ~0.5 ms per chart — reproduce with `go test -bench` in
  `go/iztro` — and `Warmup` moves the cold start to boot time.
- **Rust** — MSRV 1.88; direct dependencies are `serde`, `serde_json`,
  `lunar_rust` and `chrono` (clock only, for `horoscope_now`).
- **Pre-1.0 stability** — the serialized JSON contract, the to_text format and
  the error codes are snapshot-guarded by the test suite; breaking changes are
  called out explicitly in the [CHANGELOG](CHANGELOG.md).

## Configuration

Six switches, passed explicitly per chart — there is no global state.

| Switch             | Values                       | Default   | Effect                                                    |
|--------------------|------------------------------|-----------|-----------------------------------------------------------|
| `year_divide`      | `normal` / `exact`           | `normal`  | Year boundary: lunar new year, or 立春 (Lichun, the solar term that begins spring) |
| `horoscope_divide` | `normal` / `exact`           | `normal`  | Horoscope boundary: 1st of the month, or solar term        |
| `age_divide`       | `normal` / `birthday`        | `normal`  | Nominal age: increments at new year, or on the birthday    |
| `day_divide`       | `forward` / `current`        | `forward` | Late Rat hour belongs to the next day, or the current one  |
| `algorithm`        | `default` / `zhongzhou`      | `default` | School of placement rules                                  |
| `astro_type`       | `heaven` / `earth` / `human` | `heaven`  | Chart perspective — a Zhongzhou-school distinction         |

Defaults match JS iztro exactly. Custom 四化 (transformation) and brightness
tables can be supplied alongside them. Chart language is a per-call parameter:
`zh-CN` (default), `zh-TW`, `en-US`, `ja-JP`, `ko-KR`, `vi-VN`.

## Documentation

**<https://ziwei.x-haose.com>** — the documentation site, in English and
Chinese: a guide that starts from zero, the Zi Wei concepts behind the data
model, and per-language API references where every function, type and method
has its own entry with real output and edge-case notes. New to Zi Wei Dou Shu
itself? Start at
[the concepts](https://ziwei.x-haose.com/en/docs/guide/concepts).

Rust API docs are also on [docs.rs/x-iztro](https://docs.rs/x-iztro); inline
doc comments are currently Chinese, the English API reference lives on the
docs site. Runnable projects for all three languages are under `examples/`.

## Building from source

Only needed when changing the Rust core.

```bash
cargo build --release

# Python bindings
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

# Go bindings: rebuild and refresh the embedded wasm
cargo build --release --target wasm32-wasip1
cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/
```

Golden test data is generated from the JS iztro package:
`cd tests/golden && npm ci && npm run gen:all`.

## Credits

Chart math ported from [iztro](https://github.com/SylarLong/iztro); default
knowledge-pack texts adapted from [iztro-docs](https://docs.iztro.com) (both
MIT). iztro's author, SylarLong, maintains an introduction to Zi Wei Dou Shu
at [iztro.com](https://iztro.com/learn/basis.html).

## License

MIT

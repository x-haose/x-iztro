# x-iztro

[![crates.io](https://img.shields.io/crates/v/x-iztro.svg)](https://crates.io/crates/x-iztro)
[![PyPI](https://img.shields.io/pypi/v/x-iztro.svg)](https://pypi.org/project/x-iztro/)
[![Go Reference](https://pkg.go.dev/badge/github.com/x-haose/x-iztro/go/iztro.svg)](https://pkg.go.dev/github.com/x-haose/x-iztro/go/iztro)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**中文文档：[README.zh-CN.md](https://github.com/x-haose/x-iztro/blob/main/README.zh-CN.md)**

Give it a birth date and hour; get back a complete Zi Wei Dou Shu (紫微斗数, Purple
Star Astrology) chart — twelve palaces, every star with its brightness and
transformation, decadal and annual horoscopes — as typed objects in Rust, Python,
or Go, plus a single call that renders the whole chart as text you can hand
straight to an LLM.

## What the LLM-ready output looks like

```python
from x_iztro import Astro

astro = Astro()
chart = astro.by_solar("2000-8-16", 2, "female")
print(astro.astrolabe_to_prompt(chart))
```

```text
=== 基本信息 ===
性别: 女
阳历: 2000-8-16
农历: 二〇〇〇年七月十七
干支: 庚辰 甲申 丙午 庚寅
时辰: 寅时 (03:00~05:00)
星座: 狮子座
生肖: 龙
命宫地支: 午
身宫地支: 戌
命主: 破军
身主: 文昌
五行局: 木三局
生年四化: 太阳禄, 武曲权, 太阴科, 天同忌

=== 十二宫 ===

--- 财帛 ---
天干地支: 戊寅
大限: 43-52
小限虚岁: 9, 21, 33, 45, 57, 69, 81, 93, 105, 117
十二神: 绝, 飞廉, 吊客, 岁驿
主星: 武曲(得)[权], 天相(庙)
辅星: 天马
杂耀: 解神, 三台, 天寿, 天巫, 天厨, 阴煞, 天哭

--- 夫妻 [来因] ---
天干地支: 庚辰
大限: 23-32
小限虚岁: 7, 19, 31, 43, 55, 67, 79, 91, 103, 115
十二神: 死, 将军, 岁建, 华盖
主星: 七杀(庙)
辅星: 右弼, 火星(陷)
杂耀: 封诰, 华盖

... (all twelve palaces)
```

The same text is available in six languages — pass `language="en-US"` and the
stars, palaces and brightness levels come out as `general([+1])[B]`,
`wealth`, `Tiger hour`, `Twelve Gods: dissipated, gossip, …` and so on.
`horoscope_to_prompt` does the same for a horoscope at a given date.

## Why not just ask the LLM to cast the chart?

Casting a chart is arithmetic, not interpretation: lunar/solar conversion, leap
month handling, sexagenary cycle, the placement rules for ~100 stars, and the
四化 transformation table. A language model gets some of it right and quietly
gets the rest wrong, and you cannot tell which from the output. This library does
the arithmetic deterministically and verifiably, then hands the LLM the part it is
actually good at — reading the chart.

## Install

**Rust**

```toml
[dependencies]
x-iztro = "0.2"
```

**Python** — requires 3.10+, ships as an abi3 wheel with zero runtime dependencies.

```bash
pip install x-iztro
```

**Go** — the core library is embedded as WebAssembly and driven by the pure-Go
[wazero](https://wazero.io) runtime: no cgo, no Rust toolchain, cross-compilation
works as usual.

```bash
go get github.com/x-haose/x-iztro/go/iztro
```

## Quick start

**Rust**

```rust
use x_iztro::{by_solar, IztroError};
use x_iztro::data::types::*;

fn main() -> Result<(), IztroError> {
    let chart = by_solar(
        "2000-8-16",        // solar birth date
        2,                  // hour index: 0 = early Rat hour … 12 = late Rat hour
        Gender::Female,
        true,               // fix_leap: split leap months at the midpoint
        Language::ZhCN,
        Config::default(),  // boundaries and school; defaults match JS iztro
    )?;

    // Translated strings; `soul` and `five_elements_class` are language-independent
    // keys (`StarKey::PojunMaj`, `FiveElementsClass::Wood3rd`).
    println!("{} / {}", chart.lunar_date, chart.chinese_date);
    println!("{:?} {:?}", chart.soul, chart.five_elements_class);

    // `horoscope` derefs to the data, so the six levels are plain fields.
    let horoscope = chart.horoscope("2024-1-1", 0)?;
    println!("{:?}", horoscope.yearly.base.mutagen);

    // Bad input is an error, never a panic.
    assert!(by_solar("2000-13-1", 2, Gender::Male, true, Language::ZhCN, Config::default()).is_err());
    Ok(())
}
```

**Python**

```python
from x_iztro import Astro, IztroError
from x_iztro.enums import MajorStar, Mutagen, PalaceName

chart = Astro().by_solar("2000-8-16", 2, "female")
print(chart.chinese_date, chart.soul, chart.five_elements_class)

# Enums are language-independent keys, so these checks give the same answer
# no matter which language the chart was rendered in.
soul = chart.palace(PalaceName.SOUL)
print(soul.has([MajorStar.ZIWEI]), soul.has_mutagen(Mutagen.LU))

horoscope = chart.horoscope("2024-1-1", 0)
print(horoscope.yearly.heavenly_stem, horoscope.yearly.earthly_branch)

# IztroError subclasses ValueError; .code is a machine-readable category.
try:
    Astro().by_solar("2000-13-1", 2, "male")
except IztroError as e:
    print(e.code)  # invalid_date
```

**Go**

```go
package main

import (
    "errors"
    "fmt"
    "log"

    "github.com/x-haose/x-iztro/go/iztro"
)

func main() {
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

    // Errors carry a category you can match with errors.Is.
    _, err = iztro.BySolar("2000-13-1", 2, iztro.GenderMale, true, iztro.LanguageZhCN, nil)
    fmt.Println(errors.Is(err, iztro.ErrInvalidDate)) // true
}
```

## Features

- **Full chart** — twelve palaces, body palace, soul/body stars, five elements
  class, major/minor/adjective stars with brightness and 四化 transformations.
- **Six horoscope levels** — decadal, yearly, monthly, daily, hourly and the
  childhood limit, each with its own palaces and transformations.
- **Chart queries** — locate a palace by name, branch or index; test stars,
  transformations and empty palaces; the 三方四正 surrounded-palace group; and the
  flying-star (飞星) family.
- **Pattern judgement** — 64 named star arrangements (格局), one rule set shared by
  natal charts and horoscope views, each hit carrying the palace it formed in, the
  reading that matched, and the evidencing stars. iztro has no such API.
- **Knowledge packs** — reading texts and school-specific star attributes live in a
  swappable JSON pack, not in the core. A default pack ships inside (107 stars, 64
  patterns, 12 palaces, 4 transformations, 38 glossary entries, zh-CN, from
  iztro-docs by Sylar Long, MIT); write an overlay to replace any entry.
- **Reverse lookup** — `solar_dates_by_bazi` recovers solar birth dates from four
  BaZi pillars (interpreted under the chart Config), and `reverse_chart` recovers
  them from chart features — soul/body palace branches, five elements class, star
  placements, birth-year mutagens. Pruned enumeration verified by full
  re-charting: zero divergence from forward casting. iztro has no such API.
- **Two schools** — the default school and 中州派 (Zhongzhou), selected per chart.
- **Six languages** — zh-CN, zh-TW, en-US, ja-JP, ko-KR, vi-VN, with
  language-independent key constants so your logic never depends on the display
  language.
- **LLM output** — `astrolabe_to_prompt` / `horoscope_to_prompt` render a whole
  chart as structured text.
- **Validated input** — date format and existence, solar years 1583–9999, hour
  index 0–12. Invalid input returns `Err(IztroError)` in Rust, raises
  `x_iztro.IztroError` (a `ValueError`) in Python, returns an `error` matchable
  with `errors.Is` in Go, and yields `{"error":"..."}` JSON over the C FFI.
  Every failure carries a machine-readable category. Nothing panics.

### Pattern judgement

```python
chart = astro.by_solar("1985-5-3", 9, "male", language="en-US")

for hit in chart.patterns():
    print(hit.name, hit.palace_name, hit.broken)
```

```text
General and Wolf Together surface False
Empress and Minister Facing the Palace soul False
Marshal, Rebel and Wolf surface False
Officer and Helper Flanking Life soul False
Literary Nobility and Brilliance surface False
Literary Stars Facing Life soul True
```

The judging principles and the full table of 64 patterns are on the
[documentation site](https://ziwei.x-hoase.com/en/docs/guide/concepts/patterns).

### Knowledge packs

The default pack is Chinese, so pair it with a hit's language-independent `key`
rather than its translated name:

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

The pack format, the merge rules and how to write an overlay are on the
[documentation site](https://ziwei.x-hoase.com/en/docs/guide/guides/knowledge-pack).

### Reverse lookup

```python
from x_iztro import solar_dates_by_bazi

# every solar birth moment in 1900-2100 with the pillars 庚辰 甲申 丙午 庚寅
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

Chart-feature lookup, how the pillars follow the Config boundaries, and the
truncation semantics are on the
[documentation site](https://ziwei.x-hoase.com/en/docs/guide/guides/reverse).

## Accuracy

Every number is checked field-by-field against the JavaScript
[iztro v2.5.8](https://github.com/SylarLong/iztro) (version-pinned), with zero
tolerance for differences. Roughly 710,000 golden cases in eight layers:

| Layer      | Cases   | Coverage                                                                |
|------------|---------|--------------------------------------------------------------------------|
| Tier 1     | 1,560   | 60 years × 13 hours × both genders, every field compared individually     |
| Tier 2     | 37,440  | 60 years × the 1st and 15th of each month × 13 hours × both genders       |
| Tier 3     | 586,430 | **every day** of 60 years × 13 hours × both genders × `fix_leap`, hashed  |
| Edge years | 46,228  | the far ends of the supported range, where leap months and tables strain  |
| Horoscope  | 5,760   | 360 charts × 16 target dates, all six horoscope levels, every field       |
| Variants   | 14,268  | lunar-date charts across leap months, Zhongzhou school, all six languages |
| Config     | 9,696   | each boundary switch at its non-default value                             |
| Astro type | 12,488  | the heaven / earth / human chart perspectives                             |

On top of that: the serialization contract is compared key-by-key against JS
`JSON.stringify`, and the three bindings are cross-checked so that the same birth
data yields the same answers in Rust, Python and Go.

```bash
cargo test                                               # regular layers, ~15s
cargo test --release --test golden_tier3 -- --ignored    # Tier 3 in full, ~20s
```

## Configuration

Six switches, passed explicitly per chart — there is no global state.

| Switch             | Values                  | Default    | Effect                                                |
|--------------------|-------------------------|------------|-------------------------------------------------------|
| `year_divide`      | `normal` / `exact`      | `normal`   | Year boundary: lunar new year, or 立春                 |
| `horoscope_divide` | `normal` / `exact`      | `normal`   | Horoscope boundary: 1st of the month, or solar term    |
| `age_divide`       | `normal` / `birthday`   | `normal`   | Nominal age: increments at new year, or on the birthday |
| `day_divide`       | `forward` / `current`   | `forward`  | Late Rat hour belongs to the next day, or the current one |
| `algorithm`        | `default` / `zhongzhou` | `default`  | School of placement rules                              |
| `astro_type`       | `heaven` / `earth` / `human` | `heaven` | Chart perspective (Zhongzhou)                       |

Custom 四化 and brightness tables can be supplied alongside them.

## Documentation

**<https://ziwei.x-hoase.com>** — the documentation site, in Chinese and English: a guide
that starts from zero, the Zi Wei concepts behind the data model, and per-language API
references where every function, type and method has its own entry with real output and
edge-case notes. LLM-friendly endpoints: [`/llms.txt`](https://ziwei.x-hoase.com/llms.txt),
[`/llms-full.txt`](https://ziwei.x-hoase.com/llms-full.txt), and any page with `.md` appended.

The site lives in `docs/` (`cd docs && npm ci && npm run dev` to run it locally).
Rust API docs are also published at [docs.rs/x-iztro](https://docs.rs/x-iztro).
Runnable projects for all three languages are under `examples/`.

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

```bash
cd tests/golden && npm install && node generate_tier1.mjs   # …and the other generators
```

## Credits

Ported from [iztro](https://github.com/SylarLong/iztro) by SylarLong. New to Zi
Wei Dou Shu? Its author maintains an introduction at
[iztro.com](https://iztro.com/learn/basis.html).

## License

MIT

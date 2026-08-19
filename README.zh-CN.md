# x-iztro

[![crates.io](https://img.shields.io/crates/v/x-iztro.svg)](https://crates.io/crates/x-iztro)
[![PyPI](https://img.shields.io/pypi/v/x-iztro.svg)](https://pypi.org/project/x-iztro/)
[![Go Reference](https://pkg.go.dev/badge/github.com/x-haose/x-iztro/go/iztro.svg)](https://pkg.go.dev/github.com/x-haose/x-iztro/go/iztro)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

**English: [README.md](https://github.com/x-haose/x-iztro/blob/main/README.md)**

给一个出生日期和时辰，拿回一张完整的紫微斗数命盘——十二宫、每颗星的亮度与四化、
大限与流年运限——在 Rust、Python、Go 里都是带类型的对象；再加一个调用，
把整张盘渲染成可以直接喂给大模型的文本。

## 喂给大模型的文本长这样

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

... (十二宫依次列全)
```

六种语言都能出：传 `language="en-US"` 就变成 `general([+1])[B]`、`wealth`、
`Tiger hour`、`Twelve Gods: dissipated, gossip, …`。
`horoscope_to_prompt` 对运限做同样的事。

## 为什么不直接让大模型自己排盘

排盘是算术，不是解读：公农历换算、闰月处理、干支推算、上百颗星的安星规则、
四化表。大模型能算对一部分，剩下的会安静地算错——而你从输出里看不出是哪一部分。
这个库把算术做成确定的、可验证的，再把大模型真正擅长的那部分——读盘——交给它。

## 安装

**Rust**

```toml
[dependencies]
x-iztro = "0.2"
```

**Python** — 要求 3.10+，abi3 轮子，装完零运行期依赖。

```bash
pip install x-iztro
```

**Go** — 核心库以 WebAssembly 形式内嵌，由纯 Go 的
[wazero](https://wazero.io) 运行时驱动：不需要 cgo，不需要本机 Rust 工具链，
交叉编译照常可用。

```bash
go get github.com/x-haose/x-iztro/go/iztro
```

## 快速开始

**Rust**

```rust
use x_iztro::{by_solar, IztroError};
use x_iztro::data::types::*;

fn main() -> Result<(), IztroError> {
    let chart = by_solar(
        "2000-8-16",        // 阳历生日
        2,                  // 时辰索引：0 早子时 … 12 晚子时
        Gender::Female,
        true,               // fix_leap：闰月按中点拆到前后两月
        Language::ZhCN,
        Config::default(),  // 分界点与流派，默认同 JS iztro
    )?;

    // 文本字段已按排盘语言翻译；soul 与 five_elements_class 是语言无关标识
    // （`StarKey::PojunMaj`、`FiveElementsClass::Wood3rd`）。
    println!("{} / {}", chart.lunar_date, chart.chinese_date);
    println!("{:?} {:?}", chart.soul, chart.five_elements_class);

    // horoscope 解引用即得运限数据，六个层级都是普通字段
    let horoscope = chart.horoscope("2024-1-1", 0)?;
    println!("{:?}", horoscope.yearly.base.mutagen);

    // 非法入参返回 Err，不 panic
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

# 枚举取值是语言无关标识，在任何输出语言的星盘上判断结果一致
soul = chart.palace(PalaceName.SOUL)
print(soul.has([MajorStar.ZIWEI]), soul.has_mutagen(Mutagen.LU))

horoscope = chart.horoscope("2024-1-1", 0)
print(horoscope.yearly.heavenly_stem, horoscope.yearly.earthly_branch)

# IztroError 继承 ValueError，.code 是机器可读的错误类别
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

    // 错误带类别，用 errors.Is 判断
    _, err = iztro.BySolar("2000-13-1", 2, iztro.GenderMale, true, iztro.LanguageZhCN, nil)
    fmt.Println(errors.Is(err, iztro.ErrInvalidDate)) // true
}
```

## 功能

- **完整命盘** — 十二宫、身宫、命主身主、五行局，主星辅星杂耀连同亮度与四化。
- **六个运限层级** — 大限（未起运时为童限）、流年、流月、流日、流时，各自带十二宫与四化。
- **盘面查询** — 按宫名、地支或索引定位宫位；判断星耀、四化与空宫；三方四正；飞星族。
- **格局判定** — 64 条格局，本命盘与运限盘共用同一套规则，命中带成格宫位、
  多口径标记与证据星。iztro 无此 API。
- **知识包** — 星耀与格局的解读文本、星耀的门派属性放在可替换的 JSON 包里，不进内核。
  库内嵌一份默认包（107 颗星、64 条格局、12 宫、4 化、38 条术语，zh-CN，
  取自 Sylar Long 的 iztro-docs，MIT），不认同的条目写覆盖包换掉。
- **生辰反推** — `solar_dates_by_bazi` 由八字四柱反查公历生辰（四柱口径随排盘 Config），
  `reverse_chart` 由命宫身宫、五行局、星耀落宫、生年四化等盘面特征反查。
  剪枝枚举 + 正排终验，结果与正向排盘零分歧。iztro 无此 API。
- **两个流派** — 默认算法与中州派，按盘选择。
- **六种语言** — zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN，另有语言无关的标识常量，
  业务判断不依赖显示语言。
- **大模型输出** — `astrolabe_to_prompt` / `horoscope_to_prompt` 把整张盘渲染成结构化文本。
- **入参校验** — 日期格式与存在性、公历 1583–9999、时辰索引 0–12。
  非法输入在 Rust 返回 `Err(IztroError)`，Python 抛 `x_iztro.IztroError`（继承 `ValueError`），
  Go 返回可用 `errors.Is` 匹配的 `error`，C FFI 返回 `{"error":"..."}` JSON。
  每种失败都带机器可读的类别。任何一侧都不 panic。

### 格局判定

```python
chart = astro.by_solar("1985-5-3", 9, "male")

for hit in chart.patterns():
    print(hit.name, hit.palace_name, hit.broken)
```

```text
武贪同行 迁移 False
府相朝垣 命宫 False
杀破狼 迁移 False
左右夹命 命宫 False
文贵文华 迁移 False
文星朝命 命宫 True
```

判定原则与 64 条格局总表见[文档站](https://ziwei.x-hoase.com/zh/docs/guide/concepts/patterns)。

### 知识包

```python
from x_iztro import KnowledgePack

pack = KnowledgePack.builtin()
chart = astro.by_solar("2000-8-16", 2, "female")

for hit in chart.patterns():
    print(hit.name, "|", pack.pattern(hit.key).quotes[0])
    print(pack.pattern_intro(hit.key)[:10])
```

```text
府相朝垣 | 府相朝垣命必荣
不难看出，这是个备受
```

包的格式、合并规则与覆盖包写法见[文档站](https://ziwei.x-hoase.com/zh/docs/guide/guides/knowledge-pack)。

### 生辰反推

```python
from x_iztro import solar_dates_by_bazi

# 庚辰 甲申 丙午 庚寅 这组八字在 1900–2100 间的全部公历生辰
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

星盘特征反查、四柱口径与 Config 的关系、截断语义见[文档站](https://ziwei.x-hoase.com/zh/docs/guide/guides/reverse)。

## 准确性

每一个数值都与 JS [iztro v2.5.8](https://github.com/SylarLong/iztro)（版本锁定）
逐字段对照，零容忍差异。约 71 万条金标用例，分八层：

| 层级        | 用例数     | 覆盖范围                                     |
|-----------|---------|------------------------------------------|
| Tier 1    | 1,560   | 60 年 × 13 时辰 × 男女，全字段逐一比对                 |
| Tier 2    | 37,440  | 60 年 × 每月 1/15 号 × 13 时辰 × 男女             |
| Tier 3    | 586,430 | 60 年**每一天** × 13 时辰 × 男女 × fix_leap，全字段哈希 |
| 边界年代      | 46,228  | 支持范围的两端，闰月与农历表最吃紧的地方                      |
| Horoscope | 5,760   | 360 命盘 × 16 目标日期，六个运限层级全字段                |
| Variants  | 14,268  | 农历排盘逐日（含闰月组合）、中州派、六语言                     |
| Config    | 9,696   | 各分界开关取非默认值                               |
| 中州派盘型     | 12,488  | 天盘 / 地盘 / 人盘三种视角                          |

此外：序列化契约与 JS 的 `JSON.stringify` 逐键逐值对照；三侧绑定交叉验证，
同一组出生数据在 Rust、Python、Go 得到同一份结论。

```bash
cargo test                                               # 常规层，约 15 秒
cargo test --release --test golden_tier3 -- --ignored    # Tier 3 全量，约 20 秒
```

## 配置

六个开关，随每次排盘显式传入，没有全局状态。

| 开关                 | 取值                           | 默认         | 含义                    |
|--------------------|------------------------------|------------|-----------------------|
| `year_divide`      | `normal` / `exact`           | `normal`   | 年分界：正月初一，或立春          |
| `horoscope_divide` | `normal` / `exact`           | `normal`   | 运限分界：初一，或节气           |
| `age_divide`       | `normal` / `birthday`        | `normal`   | 虚岁：跨年即加，或过生日才加        |
| `day_divide`       | `forward` / `current`        | `forward`  | 晚子时归次日，或归当天           |
| `algorithm`        | `default` / `zhongzhou`      | `default`  | 安星流派                  |
| `astro_type`       | `heaven` / `earth` / `human` | `heaven`   | 盘型：天盘 / 地盘 / 人盘（中州派） |

另可传入自定义四化表与亮度表。

## 文档

**<https://ziwei.x-hoase.com>** —— 中英双语文档站：从零开始的指南、数据模型背后的斗数概念、
三种语言各自的 API 参考（每个函数、类型、方法都有独立条目，附真实运行输出与边界说明）。
给 AI 读的端点：[`/llms.txt`](https://ziwei.x-hoase.com/llms.txt)、
[`/llms-full.txt`](https://ziwei.x-hoase.com/llms-full.txt)，以及任意页面追加 `.md`。

站点源码在 `docs/` 下（`cd docs && npm ci && npm run dev` 本地运行）。
Rust 的 API 文档同时发布在 [docs.rs/x-iztro](https://docs.rs/x-iztro)。
三种语言各有一个可直接运行的完整示例项目，在 `examples/` 下。

## 从源码构建

只有改动 Rust 侧代码时才需要。

```bash
cargo build --release

# Python 绑定
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

# Go 绑定：重编并刷新内嵌的 wasm
cargo build --release --target wasm32-wasip1
cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/
```

金标数据由 JS iztro 包生成：

```bash
cd tests/golden && npm install && node generate_tier1.mjs   # 以及其余生成器
```

## 致谢

移植自 SylarLong 的 [iztro](https://github.com/SylarLong/iztro)。
想系统了解紫微斗数本身，作者维护了一份入门材料：
[iztro.com](https://iztro.com/learn/basis.html)。

## License

MIT

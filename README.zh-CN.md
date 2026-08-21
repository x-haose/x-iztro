<p align="center">
  <a href="https://ziwei.x-haose.com/zh"><img src="https://ziwei.x-haose.com/banner.png" alt="x-iztro — 紫微斗数排盘引擎：排盘归它算，解读归 AI"></a>
</p>

<p align="center">
  <a href="https://crates.io/crates/x-iztro"><img src="https://img.shields.io/crates/v/x-iztro?style=flat-square&logo=rust&logoColor=white" alt="crates.io"></a>
  <a href="https://docs.rs/x-iztro"><img src="https://img.shields.io/docsrs/x-iztro?style=flat-square" alt="docs.rs"></a>
  <a href="https://pypi.org/project/x-iztro/"><img src="https://img.shields.io/pypi/v/x-iztro?style=flat-square&logo=python&logoColor=white" alt="PyPI"></a>
  <a href="https://pkg.go.dev/github.com/x-haose/x-iztro/go/iztro"><img src="https://img.shields.io/badge/go.dev-reference-00ADD8?style=flat-square&logo=go&logoColor=white" alt="Go Reference"></a>
  <a href="https://github.com/x-haose/x-iztro/actions/workflows/ci.yml"><img src="https://img.shields.io/github/actions/workflow/status/x-haose/x-iztro/ci.yml?branch=main&style=flat-square&label=CI" alt="CI"></a>
  <a href="https://ziwei.x-haose.com/zh/docs/guide/about/accuracy"><img src="https://img.shields.io/badge/%E9%87%91%E6%A0%87%E7%94%A8%E4%BE%8B-716%2C314-2da44e?style=flat-square" alt="金标用例 716,314"></a>
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="MIT"></a>
</p>

<p align="center">
  <a href="https://ziwei.x-haose.com/zh">文档站</a> ·
  <a href="https://ziwei.x-haose.com/zh/docs/guide/getting-started">快速开始</a> ·
  <a href="#与-iztro-的关系">与 iztro 对照</a> ·
  <a href="#准确性">准确性</a> ·
  <a href="https://github.com/x-haose/x-iztro/blob/main/README.md">English</a>
</p>

x-iztro 把生日算成一张完整的紫微斗数命盘，再变成大模型能直接读的文字——
**排盘归它算，解读归 AI**。十二宫、约百颗星连同亮度与四化、六个运限层级、
64 条格局判定、可替换的解读文本，每一项既有译名又有语言无关的稳定标识。
排盘结果与 JS iztro v2.5.8 逐字段零差异（716,314 例金标守着这条线），
默认口径与 iztro 一致，中州派与各分界点可切换。
Rust 核心，Rust、Python、Go 三种语言直接调用。

> 排盘是算术，不是解读：公农历换算、闰月、干支、上百颗星的安星规则。
> 大模型能算对一部分，剩下的会不声不响地算错——而你从输出里看不出是哪一部分。

不写代码？先看这页：[不写代码怎么用它](https://ziwei.x-haose.com/zh/docs/guide/guides/for-non-developers)。

```python
from x_iztro import Astro

astro = Astro()
# 时辰索引 2 = 寅时 (03:00-05:00)；0 = 早子时 ... 12 = 晚子时
chart = astro.by_solar("2000-8-16", 2, "female")
print(chart.to_text())
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

…（其余十一宫依次列全）
```

样例盘的庚干四化取太阴化科，与 iztro 一致；取天府/天同的门派可用
[自定义四化表](https://ziwei.x-haose.com/zh/docs/guide/guides/config)切换。
同一张盘可以输出成六种语言；底层对象全部带类型，`chart.to_json()` 覆盖
JS iztro 输出的每一个键且逐值一致（另加文档声明的扩展键）。

## 与 iztro 的关系

排盘算法移植自 JavaScript 的 [iztro](https://github.com/SylarLong/iztro)
并与之保持逐字段一致；AI 管线需要的语义层是新增的。

|                                          | iztro v2.5.8 (JS) | x-iztro                              |
| ---------------------------------------- | ----------------- | ------------------------------------ |
| 排盘、十二宫（含身宫、来因宫）、六层运限 | ✅                | ✅ 逐字段零差异，716,314 例金标      |
| 宫位查询、三方四正、飞星                 | ✅                | ✅                                   |
| 序列化输出                               | `JSON.stringify`  | iztro 的每个键逐值一致，另加声明的扩展键 |
| 格局判定（64 条）                        | 无                | ✅                                   |
| 知识包（解读文本、门派属性）             | 无                | ✅                                   |
| 生辰反推（八字四柱 / 盘面特征 → 生辰）   | 无                | ✅                                   |
| 语义化文本投影（to_text）                | 无                | ✅                                   |
| 调用语言                                 | JS / TS           | Rust / Python / Go                   |
| 配置                                     | 全局单例          | 随盘传入，无全局状态                 |
| 非法输入                                 | 抛异常            | 带分类码的错误，非法输入不 panic     |

前端或 Node 项目请用上游 iztro（原作）；后端服务，或要把盘交给大模型解读的场景，
用 x-iztro。

## 安装

**Rust** — MSRV 1.88（有独立的 CI 任务盯着），直接依赖四个（完整依赖树 21 个
crate）。核心以 crate 级 `#![deny(unsafe_code)]` 禁用 `unsafe`，
仅 FFI/wasm 边界模块豁免。

```toml
[dependencies]
x-iztro = "0.3"
```

**Python** — 要求 3.10+，零运行时依赖，类型化 API（dataclass + StrEnum）。
预编译 abi3 轮子覆盖 Linux x86_64/aarch64（manylinux）、macOS（universal2）、
Windows x64，这些平台不需要 Rust 工具链；其余目标（musl/Alpine、32 位）
回落到 sdist 源码编译，需要装一个。

```bash
pip install x-iztro
```

**Go** — 无 cgo：Rust 核心以约 1.5 MB 的 WebAssembly 形式内嵌在模块里，
由纯 Go 运行时 [wazero](https://wazero.io) 驱动。交叉编译照常可用。

```bash
go get github.com/x-haose/x-iztro/go/iztro
```

## 快速开始

**Python**

```python
from x_iztro import Astro
from x_iztro.enums import MajorStar, Mutagen, PalaceName

# 时辰索引 2 = 寅时 (03:00-05:00)；0 = 早子时 ... 12 = 晚子时
chart = Astro().by_solar("2000-8-16", 2, "female")
print(chart.chinese_date, chart.soul, chart.five_elements_class)

# 判断方法收语言无关标识，在任何输出语言的盘上结果一致
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
        "2000-8-16",        // 阳历生日
        2,                  // 时辰索引：0 早子时 ... 12 晚子时
        Gender::Female,
        true,               // fix_leap：闰月按中点拆到前后两月
        Language::ZhCN,
        Config::default(),  // 分界点与流派，默认同 JS iztro
    )?;

    println!("{} / {}", chart.lunar_date, chart.chinese_date);
    // 这两个字段是语言无关标识，Debug 打印的是标识名；
    // 要显示译名用 translate_star(chart.soul, lang) 等翻译函数。
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
    // 参数依次为：日期、时辰索引（2 = 寅时）、性别、fixLeap、语言、
    // config（nil 取默认，同 JS iztro）；Context 变体见 BySolarContext。
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

三套绑定调用同一份 Rust 核心，同一组出生数据在三种语言里得到同一份结论——
这条等价性本身也在测试范围内。

## 在 iztro 之上

iztro 负责把盘算出来；x-iztro 还把「这张盘按传统怎么读」也算出来——
AI 管线（以及应用）在星耀落宫之上需要的语义层，外加把它们喂给大模型的出口。

### 格局判定 — 64 条规则

```python
chart = astro.by_solar("1985-5-3", 9, "male")

for hit in chart.patterns():
    print(hit.name, "|", hit.palace_name, "|", hit.variant, "|", hit.broken)
```

```text
武贪同行 | 迁移 | None | False
府相朝垣 | 命宫 | soul_empty | False
杀破狼 | 迁移 | None | False
禄马交驰 | 命宫 | surround | False
左右夹命 | 命宫 | None | False
文贵文华 | 迁移 | None | False
文星朝命 | 命宫 | None | True
文星暗拱 | 命宫 | opposite | False
文星暗拱 | 命宫 | surround | False
```

不是查表：每次命中都给出成格宫位、所采口径（`variant`）、破格标记（`broken`）
与参与成格的星曜及其落宫——可审计；`hit.key` 是语言无关标识，换输出语言不变。
「迁移」上的命中是这张盘身宫落迁移所致，身命类格局命中哪宫记哪宫；两行
「文星暗拱」是同一格局的两种成立口径（对照 / 夹拱），由 `variant` 区分。
格局名以本项目词表为准，古籍异名（如「文桂文华」）见 64 条总表。本命盘与
运限盘共用同一套规则。判定原则与总表见
[文档站](https://ziwei.x-haose.com/zh/docs/guide/concepts/patterns)。

### 知识包 — 解读层外置，可整包替换

内核只判定事实；怎么解读是门派观点，所以星耀与格局的解读文本、星耀的门派属性
（阴阳五行、化气、别号）放在可替换的 JSON 包里，不写死在代码里。库内嵌一份
zh-CN 默认包——107 颗星、64 条格局、12 宫、4 化、49 条术语，整理自
[iztro-docs](https://docs.iztro.com)（MIT，Sylar Long）。不认同哪一条，
就用覆盖包把它换掉。它同时是一份现成的中文 RAG 语料。

```python
from x_iztro import KnowledgePack

pack = KnowledgePack.builtin()
chart = astro.by_solar("2000-8-16", 2, "female")

for hit in chart.patterns():
    print(hit.name, "|", pack.pattern(hit.key).quotes[0])
```

```text
府相朝垣 | 府相朝垣命必荣
```

包格式、合并规则与覆盖包写法见
[文档站](https://ziwei.x-haose.com/zh/docs/guide/guides/knowledge-pack)。

### 八字反推 — 由四柱反查生辰

```python
from x_iztro import solar_dates_by_bazi

# 庚辰 甲申 丙午 庚寅 这组八字在 1900-2100 间的全部公历生辰
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

一组四柱约每 60 年重复一次，多解是常态；四柱按传入排盘配置的分界口径解释。

### 紫微反推 — 由盘面特征反查生辰

```python
from x_iztro import reverse_chart, ReverseCriteria, StarPosition

# 命宫在午、身宫在戌、木三局、紫微坐命、生年太阳化禄
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

条件正是本文示例盘的特征——它自己的生辰（2000-8-16，寅时）就在候选里。
两个入口都是剪枝枚举，再用正向排盘把每个候选完整复核一遍，结果必然排得回目标盘。
条件明确的查询毫秒级出解；横跨整个 200 年范围的宽查询通常也在一秒上下，
过宽的条件会在候选数上限处截断并置 `truncated`。详见
[文档站](https://ziwei.x-haose.com/zh/docs/guide/guides/reverse)。

### 接大模型的几件实事

- `chart.to_text()` 把整张盘投影成语义化文本（含格局节），
  `chart.horoscope("2024-10-1", 0).to_text()` 对指定日期的运限做同样的事；
  宫位、三方四正、格局命中各有自己的 to_text。
  格式确定、字段顺序稳定，守着它的文本快照就在测试套件里。
- 一张本命盘的完整文本约 1.8 千字（zh-CN）/ 3.6 千字符（en-US），
  折合大约一两千 token，随模型而异。
- 即使做英文产品，也建议**喂模型中文盘**：模型认识中文星名，而英文星名是
  iztro 自造的释义词表（`considery`、`dissipated` 这类），模型多半不认识——
  喂中文盘、让它用英文作答即可。原理与接法（含把库当 tool call 暴露）见
  [LLM 指南](https://ziwei.x-haose.com/zh/docs/guide/guides/llm)。
  暂无 MCP server，tool call 的接法覆盖同样的场景。
- 文档站本身对 LLM 可读：[`/llms.txt`](https://ziwei.x-haose.com/llms.txt)、
  [`/llms-full.txt`](https://ziwei.x-haose.com/llms-full.txt)，任意文档页
  追加 `.md` 即纯文本。

## 准确性

每一个数值都与 JS [iztro v2.5.8](https://github.com/SylarLong/iztro)（版本锁定）
对照，零容忍差异。这是**复现口径**，不是宣称哪一派是唯一正解——流派与口径
本就众多，配置开关与自定义四化表、亮度表就是为此准备的。整套对照你自己就能重跑：

```bash
cargo test                                               # 下表除 Tier 3 外的全部层，约 1 分钟
cargo test --release --test golden_tier3 -- --ignored    # Tier 3 全量：586,430 张盘，约 70 秒
```

716,314 例金标用例，分九层（表中「60 年」均指 1984–2043，一个甲子）：

| 层级      | 用例数  | 覆盖范围                                                 |
|-----------|---------|----------------------------------------------------------|
| Tier 1    | 1,560   | 60 年 × 13 时辰 × 男女，全字段逐一比对                   |
| Tier 2    | 37,440  | 60 年 × 每月 1/15 号 × 13 时辰 × 男女                    |
| Tier 3    | 586,430 | 60 年**每一天** × 13 时辰 × 男女，闰月日期另跑 fix_leap 双份；哈希 |
| 边界年代  | 46,228  | 1583–1983 与 2044–2100 抽样，闰月与历表最吃紧的地方      |
| Horoscope | 5,760   | 360 命盘 × 16 目标日期，六个运限层级全字段               |
| Variants  | 14,268  | 农历排盘逐日（含闰月组合）、中州派、六语言               |
| Config    | 9,696   | 各分界开关取非默认值，排盘层与运限层                     |
| 中州派盘型 | 12,488  | 天盘 / 地盘 / 人盘三种视角                               |
| 1602 窗口 | 2,444   | 1602 闰二月修正窗口逐日全时辰，哈希                      |

此外：序列化契约与 JS 的 `JSON.stringify` 逐键逐值对照；翻译反查与 iztro 的
`kot`（其翻译反查函数）实际取值对照 1,559 例；三侧绑定交叉验证，同一组出生
数据在 Rust、Python、Go 得到同一份结论。

**「零容忍」是什么意思，举一个例子。** Rust 侧的农历依赖库在 1602 年
（明万历三十年）的月表自相矛盾——二月 31 天——导致 1 天直接 panic、另有 28 天
静默排出错盘。x-iztro 在农历月表的唯一读取入口加了修正层：真值经
lunar-typescript、寿星天文历（sxtwl）与韩国天文研究院历表三个独立来源交叉确认，
窗口由 2,444 例金标锁死，另有 1583–9999 全域逐日扫描（约 614 万张盘）未发现
第二个同类窗口。修正层带上游探测：依赖库一旦修好，自动停用。

## 放进生产

- **无全局状态**——配置与语言随每次调用传入，不同流派、不同语言的盘并发互不干扰。
- **非法输入是错误，不是 panic**——日期格式与存在性都校验，公历 1583–9999，
  时辰索引 0–12。Rust 返回 `Err(IztroError)`，Python 抛 `x_iztro.IztroError`
  （继承 `ValueError`，带 `.code`），Go 返回可用 `errors.Is` 匹配的 `error`，
  C FFI 返回错误 JSON。每种失败都带机器可读的类别。
- **Python**——单次排盘约 0.5 ms（Apple Silicon 开发机）。盘对象是 frozen
  dataclass，可跨线程共享，但计算持有 GIL：扩容用多进程而不是多线程。
  序列化用 `to_dict()` / `to_json()`，别用 `dataclasses.asdict()`
  （盘内有回指引用，会无限递归）。
- **Go**——并发安全：wasm 实例池按调用取用（上限 `GOMAXPROCS`，每实例一份
  线性内存），多 goroutine 真并行。首次调用编译内嵌 wasm（约 100–200 ms，
  落盘缓存命中后 20–30 ms；缓存在 `os.UserCacheDir()` 下，取不到时退化为
  每进程编译）。稳态单次排盘约 0.5 ms（`go/iztro` 下 `go test -bench` 可复现），
  `Warmup` 可把冷启动挪到启动阶段。
- **Rust**——MSRV 1.88；直接依赖只有 `serde`、`serde_json`、`lunar_rust` 与
  `chrono`（仅 `horoscope_now` 用到时钟）。
- **1.0 之前的稳定性**——序列化 JSON 契约、to_text 文本格式与错误码都有快照测试
  守着；破坏性变更在 [CHANGELOG](CHANGELOG.md) 里显式列出。

## 配置

六个开关，随每次排盘显式传入，没有全局状态。

| 开关               | 取值                         | 默认      | 含义                              |
|--------------------|------------------------------|-----------|-----------------------------------|
| `year_divide`      | `normal` / `exact`           | `normal`  | 年分界：正月初一，或立春          |
| `horoscope_divide` | `normal` / `exact`           | `normal`  | 运限分界：初一，或节气            |
| `age_divide`       | `normal` / `birthday`        | `normal`  | 虚岁：跨年即加，或过生日才加      |
| `day_divide`       | `forward` / `current`        | `forward` | 晚子时归次日，或归当天            |
| `algorithm`        | `default` / `zhongzhou`      | `default` | 安星流派                          |
| `astro_type`       | `heaven` / `earth` / `human` | `heaven`  | 盘型：天盘 / 地盘 / 人盘（中州派）|

默认值与 JS iztro 完全一致。另可传入自定义四化表与亮度表。盘面语言随调用传入：
`zh-CN`（默认）、`zh-TW`、`en-US`、`ja-JP`、`ko-KR`、`vi-VN`。

## 文档

**<https://ziwei.x-haose.com>**——中英双语文档站：从零开始的指南、数据模型
背后的斗数概念、三种语言各自的 API 参考（每个函数、类型、方法都有独立条目，
附真实运行输出与边界说明）。想先系统看懂命盘本身，从
[紫微斗数概念](https://ziwei.x-haose.com/zh/docs/guide/concepts)开始。

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

金标数据由 JS iztro 包生成：`cd tests/golden && npm ci && npm run gen:all`。

## 致谢

排盘算法移植自 SylarLong 的 [iztro](https://github.com/SylarLong/iztro)；
默认知识包文本整理自 [iztro-docs](https://docs.iztro.com)（两者均为 MIT）。
想系统了解紫微斗数本身，iztro 作者维护了一份入门材料：
[iztro.com](https://iztro.com/learn/basis.html)。

## License

MIT

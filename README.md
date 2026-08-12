# rs-iztro

紫微斗数 Rust 核心库，移植自 [iztro](https://github.com/SylarLong/iztro) v2.5.8。

提供排盘、运限、三方四正、四化飞星等完整功能，支持 Rust / Python / Go 三种语言调用。

## 安装

### Rust

```toml
# Cargo.toml（本仓库尚未发布到 crates.io，用 path 或 git 依赖指向本仓库）
[dependencies]
rs-iztro = { path = "../rs-iztro" }
```

### Python

```bash
pip install maturin
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
```

### Go

Go 包内嵌 wasm、经纯 Go 的 wazero 运行时调用，无 cgo、无需本机 Rust 工具链：

```bash
go get rs-iztro/go/iztro   # 或在 go.mod 中 replace 指向本仓库 go/iztro
```

开发者更新内嵌 wasm：

```bash
cargo build --release --target wasm32-wasip1
cp target/wasm32-wasip1/release/rs_iztro.wasm go/iztro/
```

## 快速开始

### Rust

```rust
use rs_iztro::{IztroError, by_solar, get_horoscope};
use rs_iztro::data::types::*;

// Config 控制分界点与派别：year_divide / horoscope_divide / age_divide /
// day_divide / algorithm，默认值与 JS iztro 一致；
// 入参非法（日期格式/范围、时辰索引）时返回 Err(IztroError)
let astrolabe = by_solar("2000-8-16", 2, Gender::Female, true, Language::ZhCN, Config::default())?;
println!("命主：{:?}", astrolabe.soul);

// 中州派：Config { algorithm: Algorithm::Zhongzhou, ..Config::default() }

let horoscope = get_horoscope(&astrolabe, "2024-1-1", 0, Language::ZhCN)?;
println!("流年四化：{:?}", horoscope.yearly.base.mutagen);
```

### Python

```python
from rs_iztro import Astro, ChartConfig
from rs_iztro.enums import Algorithm, MajorStar, Mutagen, PalaceName

astro = Astro()
result = astro.by_solar("2000-8-16", 2, "female")
# 配置：astro.by_solar(..., config=ChartConfig(algorithm=Algorithm.ZHONGZHOU))

# 枚举基于语言无关 key，在任何输出语言的星盘上判断结果一致
soul = result.palace(PalaceName.SOUL)
print(f"命主：{result.soul}")
print(f"命宫有紫微：{soul.has([MajorStar.ZIWEI])}")
print(f"命宫化禄：{soul.has_mutagen(Mutagen.LU)}")

horoscope = astro.get_horoscope(result, "2024-1-1", 0)
print(f"流年：{horoscope.yearly.heavenly_stem}{horoscope.yearly.earthly_branch}")
print(f"岁前十二神：{horoscope.yearly.yearly_dec_star.suiqian12}")
```

### Go

```go
import "rs-iztro/go/iztro"

// 返回类型化结构体；key 常量在任何输出语言下都有效
result, _ := iztro.BySolar("2000-8-16", 2, "female", true, "zh_cn", nil)
fmt.Println(result.Soul)

soul := result.Palace(iztro.PalaceSoul)
fmt.Println(soul.Has(iztro.StarZiweiMaj), soul.HasMutagen(iztro.MutagenLu))

// 中州派：iztro.BySolar(..., &iztro.Config{Algorithm: "zhongzhou"})
h, _ := iztro.GetHoroscope("2000-8-16", 2, "female", true, "zh_cn", nil, "2024-1-1", 0)
fmt.Println(h.Yearly.HeavenlyStem, h.Yearly.Mutagen)
```

## 错误处理

全部外部输入在核心层前置校验：日期格式与存在性、公历 1583-9999 年（下限为
格里历改革次年、上限为农历表终点）、时辰索引 0-12。非法输入的表现按语言：

- **Rust** — 排盘入口返回 `Result<_, IztroError>`
- **Python** — 抛 `ValueError`（消息含具体原因）
- **Go** — 返回 `error`
- **C FFI** — 返回 `{"error":"..."}` JSON（serde 生成，转义完备）

## 示例项目

`examples/` 下有三个独立可运行的完整项目：

```
examples/rust/     # cargo run
examples/python/   # python main.py
examples/go/       # go run .
```

## 测试

### 运行测试

```bash
# 常规测试（单元 + tier1/tier2/运限/变体/配置/契约金标，~15 秒）
cargo test

# Tier 3 全参数空间（586,430 例，~20 秒）
cargo test --release --test golden_tier3 -- --ignored

# Python / Go 端到端金标
cd python && pytest tests/          # 先 maturin develop
cd go/iztro && go test ./...
```

### 金标覆盖矩阵

全部数据对照 JS [iztro v2.5.8](https://github.com/SylarLong/iztro)（版本锁定）生成，零容忍差异：

| 层级        | 用例数      | 覆盖范围                                            | 数据格式        |
|-----------|----------|-------------------------------------------------|-------------|
| Tier 1    | 780      | 60 年 × 13 时辰，全字段逐一比对（含展示字段与来因宫）                 | 完整 JSON     |
| Tier 2    | 37,440   | 60 年 × 每月 1/15 号 × 13 时辰 × 男女，紧凑比对              | 压缩 JSON     |
| Tier 3    | 586,430  | 60 年**每一天** × 13 时辰 × 男女 × fix_leap（闰月双份），全字段哈希 | SHA-256 CSV |
| Horoscope | 5,760    | 360 命盘 × 16 目标日期（12 流年支/童限/高龄/闰月/晚子时），六层级运限全字段  | 紧凑 JSON     |
| Variants  | 14,268   | by_lunar 闰月逐日（含 is_leap/fix_leap 组合）、中州派、六语言    | CSV/JSON    |
| Config    | 8,544    | yearDivide=exact（立春窗口逐日）、dayDivide=current、ageDivide=birthday、horoscopeDivide=exact | CSV/JSON    |

合计约 66 万对照用例，覆盖排盘与运限的全部参数空间；另有 Python 108 例、Go 金标端到端测试（含 500 次非法输入轰炸）、C FFI 边界安全 16 例与绑定契约 JSON 逐键对照。

### 生成测试基准数据

```bash
cd tests/golden
npm install
node generate_tier1.mjs      # → tier1_data.json (~6MB)
node generate_tier2.mjs      # → tier2/year_*.json (60 files, ~23MB)
node generate_tier3.mjs      # → tier3/year_*.csv (60 files, ~30MB, 约 30 分钟)
node generate_horoscope.mjs  # → horoscope_data.json (~9MB)
node generate_variants.mjs   # → variants_*.csv / variants_languages.json
node generate_config.mjs     # → config_*.csv / config_*.json
```

哈希不一致时用生成器的 `--inspect` 系列参数重放 JS 单例，与 Rust 失败输出中的规范化串 diff 定位字段（格式定义见 `tests/golden/canonical.mjs` 与 `tests/common/mod.rs`）。

## 项目结构

```
src/
  lib.rs              # 公共 API
  error.rs            # IztroError 错误类型
  data/               # 枚举、常量、天干地支、星耀数据
  models/             # Astrolabe、Palace、Star、Horoscope 结构体
  astro/              # 排盘、运限、三方四正算法
  i18n/               # 多语言翻译（zh-CN/zh-TW/en-US/ja-JP/ko-KR/vi-VN）
  dto.rs              # JS 兼容序列化 DTO（三语言绑定共用）
  ffi.rs              # C FFI 导出
  wasm.rs             # wasm32 导出（Go 经 wazero 调用）
  python.rs           # PyO3 原生模块
  prompt.rs           # AI Prompt 生成

python/rs_iztro/      # Python 包（dataclass 类型 + 枚举常量）
go/iztro/             # Go FFI 绑定包
examples/             # Rust / Python / Go 示例项目
tests/golden/         # JS 生成的测试基准数据
```

## 移植说明

- 移植自 iztro v2.5.8，核心排盘逻辑 1:1 对照
- 支持默认算法和中州派算法
- 排盘结果与 JS iztro 完全一致：金标测试覆盖全部时辰（含晚子时）与全部星耀，零容忍差异

## License

MIT

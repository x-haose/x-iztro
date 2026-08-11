# rs-iztro

紫微斗数 Rust 核心库，移植自 [iztro](https://github.com/SylarLong/iztro) v2.5.8。

提供排盘、运限、三方四正、四化飞星等完整功能，支持 Rust / Python / Go 三种语言调用。

## 安装

### Rust

```toml
# Cargo.toml
[dependencies]
rs-iztro = { git = "https://github.com/anthropic/rs-iztro" }
```

### Python

```bash
pip install maturin
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
```

### Go

```bash
cargo build --release
# Go 项目中引用 go/ 目录下的 iztro 包
```

## 快速开始

### Rust

```rust
use rs_iztro::{by_solar, get_horoscope};
use rs_iztro::data::types::*;

let astrolabe = by_solar("2000-8-16", 2, Gender::Female, true, Language::ZhCN, Algorithm::Default);
println!("命主：{}", astrolabe.soul);

let horoscope = get_horoscope( & astrolabe, "2024-1-1", 0, Language::ZhCN);
println!("流年：{}", horoscope.yearly.name);
```

### Python

```python
from rs_iztro import Astro
from rs_iztro.enums import PalaceName, Mutagen

astro = Astro()
result = astro.by_solar("2000-8-16", 2, "female")

soul = result.palace(PalaceName.SOUL)
print(f"命主：{result.soul}")
print(f"命宫化禄：{soul.has_mutagen(Mutagen.LU)}")

horoscope = astro.get_horoscope(result, "2024-1-1", 0)
print(f"流年：{horoscope.yearly.name}")
```

### Go

```go
import "rs-iztro/go/iztro"

result, _ := iztro.BySolar("2000-8-16", 2, "female", true, "zh_cn", "default")
fmt.Println(result["soul"])
```

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
# 常规测试（单元测试 + Tier 1 + Tier 2，~8 秒）
cargo test

# 含 Tier 3 完整回归（~4 分钟，284,895 用例）
cargo test -- --ignored
```

### 测试金字塔

对照 JS [iztro v2.5.8](https://github.com/SylarLong/iztro) 生成的测试基准数据，三层覆盖：

| 层级     | 用例数     | 覆盖范围                   | 数据格式        |
|--------|---------|------------------------|-------------|
| Tier 1 | 780     | 60 年 × 13 时辰，全字段逐一比对   | 完整 JSON     |
| Tier 2 | 37,440  | 60 年 × 月/日/时辰/性别，紧凑比对  | 压缩 JSON     |
| Tier 3 | 284,895 | 60 年 × 每日 × 13 时辰，哈希校验 | SHA-256 CSV |

### 生成测试基准数据

```bash
cd tests/golden
npm install
node generate_tier1.mjs    # → tier1_data.json (~6MB)
node generate_tier2.mjs    # → tier2/year_*.json (60 files, ~23MB)
node generate_tier3.mjs    # → tier3_hashes.csv (~21MB, 需要几分钟)
```

Tier 3 Rust 侧使用 selfcheck baseline 机制——首次运行生成 `selfcheck_baseline.csv`，后续运行对比检测回归。

## 项目结构

```
src/
  lib.rs              # 公共 API
  data/               # 枚举、常量、天干地支、星耀数据
  models/             # Astrolabe、Palace、Star、Horoscope 结构体
  astro/              # 排盘、运限、三方四正算法
  i18n/               # 多语言翻译（zh-CN/zh-TW/en-US/ja-JP/ko-KR/vi-VN）
  ffi.rs              # C FFI 导出（供 Go 等语言调用）
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

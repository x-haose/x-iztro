# CLAUDE.md

## 项目概述

rs-iztro：紫微斗数 Rust 核心库，移植自 JS [iztro](https://github.com/SylarLong/iztro) v2.5.8。
支持 Rust / Python(PyO3) / Go(C FFI) 三语言调用。

## 决策标准

- 代码的功能、质量、架构、干净程度（即「强大」）永远是第一优先。
- 绝不以「最小改动 / 最小破坏」为标准做决定；只做对的决定，不做最小的。
- 某块/某服务/整体到了重写更优的程度，只要收益高就重写，在所不惜。
- 零容忍对付、补丁、将就式编码；要范围内最合适、最高的标准。
- 准确性目标：iztro 自身有的功能与数据 100% 对齐（金标零容忍差异），在此基础上再扩展增值功能。

## 注释规范

- 只解释「当前代码在当前项目的含义」，必须自包含——不依赖对话、设计文档、changelog 才能读懂。
- 禁止出现对话过程、解释性叙述、「以前/现在/后续/待改」之类的修改痕迹或变更日志。
- 覆盖要全：所有字段、方法、函数、类型都写。
- 极其厌恶繁琐、屎山式注释，尤其把对话内容/改动日志写进注释。

## 构建与测试

```bash
cargo build --release                    # 构建（含 cdylib + rlib）
cargo test                               # 常规测试（~8s，含 Tier 1/2 金标测试）
cargo test -- --ignored                  # 含 Tier 3 selfcheck（~4min，284K 用例）

# Python 绑定
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

# 金标测试数据生成（需要 Node.js + iztro）
cd tests/golden && npm install && node generate_tier1.mjs && node generate_tier2.mjs && node generate_tier3.mjs
```

## 项目结构

- `src/` — Rust 核心库
  - `data/` — 枚举、常量、天干地支、星耀数据表
  - `models/` — Astrolabe、Palace、Star、Horoscope 结构体
  - `astro/` — 排盘、运限、三方四正算法
  - `i18n/` — 多语言翻译
  - `ffi.rs` — C FFI（Go 调用）
  - `python.rs` — PyO3 原生模块（`python` feature gate）
- `python/rs_iztro/` — Python 包（dataclass 类型 + StrEnum 枚举，零外部依赖）
- `go/iztro/` — Go FFI 绑定包
- `examples/{rust,python,go}/` — 三个独立示例项目
- `tests/golden/` — JS 生成的金标测试数据（tier1/tier2/tier3）

## 关键约定

### Rust
- Edition 2024，unsafe 用 `#[unsafe(no_mangle)]` 语法
- 排盘入口收 `Config`（year_divide/horoscope_divide/age_divide/day_divide/algorithm），`Config::default()` 与 JS iztro 默认一致；年系杂耀与岁前/将前12按 horoscope_divide 年支、红鸾天喜按 year_divide 年支（复刻 iztro 内部分界矩阵）
- `Cargo.toml` 的 lib crate-type 同时有 `cdylib`（FFI）和 `rlib`（Rust 依赖）
- `python` feature 启用 PyO3 + pythonize

### Python 绑定架构
- Rust 侧（`src/python.rs`）返回 dict（via pythonize），模块名 `_rs_iztro`
- Python 侧（`python/rs_iztro/`）用 dataclass 包装，提供类型化 API
- `pyproject.toml` 配置 `python-source = "python"`, `module-name = "rs_iztro._rs_iztro"`
- 不依赖 pydantic，纯 stdlib（dataclasses + StrEnum）

### Go 绑定
- `go/iztro/iztro.go` 通过 cgo 调用 `src/ffi.rs` 导出的 C 函数
- 返回 JSON 字符串 → `map[string]any`
- `examples/go/go.mod` 用 `replace` 指向 `../../go`

### 测试
- 全部金标数据由 JS iztro v2.5.8（版本锁定）生成，在 `tests/golden/` 下，零容忍差异
- 覆盖矩阵（约 65 万例）：tier1 全字段 780（含 rawDates）/ tier2 紧凑 37K / tier3 全日期×性别×fix_leap 哈希 586K / 运限 5,760 / 变体（by_lunar 闰月逐日、中州派、六语言）14K / Config 四开关非默认值 8.5K
- tier3 与变体哈希基于规范化串（`tests/golden/canonical.mjs` ≡ `tests/common/mod.rs`，逐字节同构）；不一致时用生成器 `--inspect*` 重放 JS 单例与 Rust 输出 diff
- `cargo test` 跑常规层（~15s）；tier3 全量用 `cargo test --release --test golden_tier3 -- --ignored`（~20s）
- 同步 iztro 新版本流程：升级 `tests/golden/package.json` 中的精确版本 → 重新生成金标数据 → `cargo test`，diff 即差异清单

### 多语言
- 支持 6 种语言：zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN
- 翻译在 `src/i18n/` 下，serde 序列化时自动应用

## 常用命令速查

```bash
cargo run --example basic                      # 从根目录跑旧 example（如果有）
cd examples/rust && cargo run                  # Rust 示例
cd examples/go && go run .                     # Go 示例
python examples/python/main.py                 # Python 示例
cargo test --test golden_tier1 -- --nocapture  # 单跑 Tier 1 看输出
nm target/release/librs_iztro.dylib | grep iztro  # 检查 FFI 符号导出
```

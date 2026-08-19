# CLAUDE.md

## 项目概述

x-iztro：紫微斗数 Rust 核心库，移植自 JS [iztro](https://github.com/SylarLong/iztro) v2.5.8。
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
cargo test                               # 常规测试（~1min，含除 Tier 3 外的全部金标）
cargo test --release --test golden_tier3 -- --ignored   # Tier 3 全量（586,430 例，~70s）

# Python 绑定
PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

# Go 用 wasm
cargo build --release --target wasm32-wasip1

# 金标测试数据生成（需要 Node.js + iztro）
cd tests/golden && npm ci && npm run gen:all       # 逐个生成器见 package.json 的 gen:* 脚本
# tier3 / tier_edge 按年跳过已存在的文件（便于断点续跑），重新生成前先删掉对应目录；
# tier3 支持 `node generate_tier3.mjs --range <起> <止>` 分段，多进程并行可跑满 CPU
```

## 项目结构

- `src/` — Rust 核心库
  - `data/` — 枚举、常量、天干地支、星耀数据表
  - `models/` — Astrolabe、Palace、Star、Horoscope 结构体
  - `astro/` — 排盘、运限、三方四正算法
  - `pattern/` — 格局判定引擎（`view.rs` 把本命盘与运限合成盘统一成 `ChartView`，
    `rules/` 五组 64 条规则每条一个函数并注明口径来源，`keys.rs` 语言无关 `PatternKey`）
  - `i18n/` — 多语言翻译
  - `error.rs` — IztroError（入口前置校验的错误类型）
  - `dto.rs` — JS 兼容序列化 DTO（三语言绑定共用）
  - `ffi.rs` — C FFI（错误 JSON + catch_unwind 兜底）
  - `wasm.rs` — wasm32 导出（Go 经 wazero 调用；wasm 上 catch_unwind 无效，防线在核心校验）
  - `python.rs` — PyO3 原生模块（`python` feature gate）
- `python/x_iztro/` — Python 包（dataclass 类型 + StrEnum 枚举，零外部依赖）
- `go/iztro/` — Go FFI 绑定包
- `examples/{rust,python,go}/` — 三个独立示例项目
- `tests/golden/` — JS 生成的金标测试数据（tier1/tier2/tier3）

## 关键约定

### Rust
- Edition 2024，unsafe 用 `#[unsafe(no_mangle)]` 语法
- `by_lunar` 收三态 `LeapMonth`（NotLeap/Leap/LeapFixed）而非 iztro 的 `isLeapMonth`+`fixLeap` 两个相邻布尔（写反不报错）；
  绑定层 JSON 线协议仍是两个布尔键，Go 用 `LeapMonth` 具名常量、Python 用关键字参数；Go 的 `gender`/`language` 也是具名类型
- 排盘入口收 `Config`（year_divide/horoscope_divide/age_divide/day_divide/algorithm），`Config::default()` 与 JS iztro 默认一致；年系杂耀与岁前/将前12按 horoscope_divide 年支、红鸾天喜按 year_divide 年支（复刻 iztro 内部分界矩阵）
- 入口返回 `Result<_, IztroError>`：全部外部输入前置校验（日期格式与存在性、公历 1583-9999、时辰 0-12），非法输入不 panic——wasm 下 panic 即 trap 且累积损耗实例栈空间，绑定层 catch_unwind 仅兜底库内部缺陷
- `Cargo.toml` 的 lib crate-type 同时有 `cdylib`（FFI）和 `rlib`（Rust 依赖）
- `python` feature 启用 PyO3 + pythonize

### 绑定契约（三语言共用）
- `src/dto.rs` 定义 JS iztro 兼容的序列化 DTO：camelCase 键 + 按排盘语言翻译的值，
  附加两类扩展：排盘上下文（genderKey/timeIndex/fixLeap/language/config）与
  语言无关标识（星/宫/干支/四化/亮度/五行局的 `*key`/`*Key(s)`，取值为 iztro i18n key）
- 强类型层基于标识字段：Python 枚举（enums.py）与 Go 常量（keys.go）的值即这些 key，
  判断方法在任何输出语言的星盘上结果一致
- 绑定接口无状态：运限/Prompt/格局 直接收排盘参数（含 config JSON 部分键补丁），不做星盘 JSON 往返
- 契约由 golden_contract 测试与 JS 的 JSON.stringify 输出逐键逐值对照

### Python 绑定架构
- Rust 侧（`src/python.rs`）返回 DTO dict（via pythonize），模块名 `_x_iztro`
- Python 侧（`python/x_iztro/`）用 dataclass 包装，提供类型化 API；config 传 camelCase dict
- `pyproject.toml` 配置 `python-source = "python"`, `module-name = "x_iztro._x_iztro"`
- 不依赖 pydantic，纯 stdlib（dataclasses + StrEnum）
- 端到端金标：`cd python && pytest tests/`（先 maturin develop）

### Go 绑定
- `go/iztro` 内嵌 `x_iztro.wasm`（wasm32-wasip1），经纯 Go 的 wazero 运行时调用，无 cgo
- 内存协定见 `src/wasm.rs`：alloc/free + (ptr<<32)|len 打包返回
- 更新 wasm：`cargo build --release --target wasm32-wasip1 && cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/`
- `examples/go/go.mod` 用 `replace` 指向 `../../go/iztro`；金标测试 `cd go/iztro && go test`

### 与 iztro 的 API 对齐
- 基准是 npm 包的 `lib/**/*.d.ts`（签名）+ `lib/**/*.js`（语义，以它为准）。
  硬要求：iztro 每个公开 API 三侧都要有等价物，且三侧能力完全一致，形式各随语言习惯
- 已全数覆盖。改动后逐条自查用这两条线索——三侧测试都发现不了它们：
  - `src/bridge.rs` 分派了、但 `python/x_iztro/*.py` 或 `go/iztro/*.go` 没写类型化包装，
    等于对外不可用
  - 反查取值（`key_of` ≡ iztro `kot`）依赖扫描顺序：iztro 按语言外层、locale 合并顺序内层，
    8 处同形译名靠它消歧。金标须拿 `kot` 实际取值逐条对照（`tests/golden/i18n_kot.json`），
    写成「反查到某个译文相同的标识」这类松断言查不出顺序分叉
- 换了形状而非照抄的几处，别改回去：
  - `astroType` 收进 `Config`（iztro 放在 `withOptions`，因其 `config()` 是全局单例装不下按盘变化的值）
  - 配置显式随调用传入，无全局单例，故不提供 `getConfig` / `setLanguage`
  - `get_decadals_and_ages` 直接收命宫索引与五行局，比 iztro `getHoroscope` 的 `from` 更一般
  - 插件按语言惯用方式实现：Rust 扩展 trait、Python 类方法注入、Go 嵌入 `*Astrolabe`
- 故意不做的（考察过，不是漏）：`astro/analyzer`（Palace/Surpalaces 方法的自由函数版）、
  `calendar/*` 与 `star/star.js`、`star/decorationStar.js`（iztro v2.5.8 里已是死代码，
  活路径走 lunar-lite）、`initStars`（空盘工厂，类型系统已给定长数组）、
  `astrolabeBySolarDate` / `astrolabeByLunarDate`（v2.0.5 起废弃的别名）、
  `fixEarthlyBranchIndex`（与 `earthlyBranchIndexToPalaceIndex` 同义）、
  `setPalace` / `setAstrolabe`（建链是内部行为）、i18next 实例、`Astrolabe.copyright`
- 复刻的反直觉语义：`fliesTo` 星列表为空返回 false，而 `fliesOneOfTo` / `notFlyTo` 返回 true；
  `getPalace` 额外接受 `bodyPalace` / `originalPalace`；`getMajorStarBySolarDate` 命宫空宫时借对宫；
  iztro 在本命层级把红鸾误写为 `hongluanMin`，x-iztro 用正确的 `hongluan`
- 用户视角的对应关系写在文档站「关于 → 与 iztro 的对应」一页

### 格局引擎（x-iztro 扩展，iztro 无对应 API）
- 规则来源 iztro-docs《格局》页（MIT）63 条，火贪/铃贪分列为 64 个 `PatternKey`；无金标，口径自守：
  每条规则函数的文档注释就是口径与出处，多口径一律以 `PatternHit.variant` 报出、不设 strict/loose 开关，
  「破格/加杀平常」只置 `broken` 不否决，「身命」类命宫身宫各判、命中哪宫 `palace` 记哪宫
- 亮度红线：日月明暗默认按 iztro 亮度表（`BrightnessSource::Table`），页面《日月并明》示例太阴在酉、
  表为「不」故按表不成格；`Positional` 口径复现传统位置判法。`PatternConfig` 只放会改变事实判定的开关
- 本命与运限共用同一套规则：`ChartView::at` 以该层命宫为命宫、合并该层流曜（流曜等同对应本命辅星）
  与该层四化；`Scope::Origin` 等同本命；两条行运格（禄衰马困、风云际会）只在运限视角报，
  风云际会只在大限视角报一次
- 新增/修改规则：改 `src/pattern/rules/<组>.rs` 并加入该组 `RULES`（`rules_cover_every_pattern_key_once`
  强制 64 个 key 都有规则）；正例用 `pattern::testutil::find_chart` 在真实盘上搜；
  规范文档 `docs/plan/2026-08-19-pattern-rules.md` 是本地过程文件（gitignore），不要在代码注释里引用它

### 测试
- 全部金标数据由 JS iztro v2.5.8（版本锁定）生成，在 `tests/golden/` 下，零容忍差异
- 覆盖矩阵（八层合计 713,870 例，约 71 万；另有 i18n 反查 1,559 与契约 13）：tier1 全字段 1,560（60 年 × 13 时辰 × 男女，含 rawDates）/
  tier2 紧凑 37,440 / tier3 全日期×性别×fix_leap 哈希 586,430 /
  边界年代哈希 46,228（1583-1983 与 2044-2100 每 10 年抽样，补 tier1/2/3 只覆盖 1984-2043 的盲区）/
  运限 5,760 / 变体（by_lunar 闰月逐日、中州派、六语言）14,268 /
  Config 开关（四个非默认取值 + 排盘层与运限层的组合）9,696 / 中州派盘型 12,488
- tier3、边界年代、变体、astrotype、Config 排盘层的哈希都基于规范化串
  （`tests/golden/canonical.mjs` ≡ `tests/common/mod.rs`，逐字节同构；
  条目含星名/类型/范围/亮度/四化，排序等价性只在 BMP 内成立，注释里写了这个前提）；
  不一致时用生成器 `--inspect*` 重放 JS 单例与 Rust 输出 diff
- `cargo test` 跑除 tier3 外的全部层（~1min）；tier3 全量用
  `cargo test --release --test golden_tier3 -- --ignored`（~70s）
- 哪条结论由哪个测试守着：
  - 排盘数值 → `golden_tier1/2/3`、`golden_edge`（1583-1983 / 2044-2100 抽样）、
    `golden_variants`、`golden_horoscope`
  - 序列化契约 → `golden_contract`（对 JS 的 `JSON.stringify` 逐键逐值；
    扩展键白名单按 DTO 路径限定，多在别处的键即判为契约偏离）
  - 单盘全接口面 → `regression`（工具函数、十二宫、三方四正、运限查询方法，
    金标由 `generate_regression.mjs` 生成，可复现）
  - 入口错误路径 → `error_paths`（非法日期/越界年份/时辰 13/农历 31 日等返回 Err 不 panic）
  - 绑定不漏接口 → `binding_coverage`（读 `src/bridge.rs` 与 `src/data/stars.rs` 源码文本，
    要求每个 kind 与星耀 key 都出现在 Python/Go 的非测试源码里）
  - Prompt 文本 → `prompt_snapshot`（zh-CN / en-US 固定盘的完整输出快照，
    快照缺失时自动写入基线，有意改动后删掉 `tests/golden/prompt_snapshots/` 重建）
  - 格局 → 规则单测（`src/pattern/rules/*.rs`，真实盘正/负例）+ `pattern_api`（Rust 方法/DTO/FFI 分派/口径入参）
    + `pattern_examples`（iztro-docs 页面 32 张示例盘反查真实盘）+ `pattern_distribution`（tier1 全量分布合理性）
    + `pattern_snapshot`（三侧同解基线 `tests/golden/pattern_snapshots/`，Python `test_patterns.py` 与
    Go `pattern_golden_test.go` 读同一批快照；有意改口径后删掉快照重建）
  - `star` 模块各入口 → `golden_star`（含低层落宫按入参域全覆盖 814 例）
  - 翻译与反查 → `golden_i18n`（`key_lookup_matches_kot` 1,559 例对 `kot` 实际取值）
  - 数据表 → `golden_data`；中州派盘型 → `golden_astrotype`；四开关 → `golden_config`
  - 自定义四化/亮度表 → `config_overrides`；Rust 扩展 trait → `extension`
  - 三侧同盘同解 → `src/models/astrolabe.rs` 单测 + `python/tests/test_parity.py`
    + `go/iztro/parity_check_test.go`；插件三侧同解 → 各自的 `test_plugin` / `plugin_test`
- 同步 iztro 新版本流程：升级 `tests/golden/package.json` 中的精确版本 → 重新生成金标数据
  → `cargo test`，diff 即数据差异清单；API 面的增删改金标不报，另按上面「与 iztro 的 API 对齐」自查

### 多语言
- 支持 6 种语言：zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN
- 翻译在 `src/i18n/` 下，serde 序列化时自动应用

## 常用命令速查

```bash
cd examples/rust && cargo run                  # Rust 示例
cd examples/go && go run .                     # Go 示例
python examples/python/main.py                 # Python 示例
cargo test --test golden_tier1 -- --nocapture  # 单跑 Tier 1 看输出
nm target/release/libx_iztro.dylib | grep iztro  # 检查 FFI 符号导出
```

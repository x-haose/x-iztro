# 更新日志

本项目的所有重要变更记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 修正

- `rust-version` 由 1.85 改为 1.88：代码自 0.2.0 起已使用 let-chains（Rust 1.88 稳定），
  旧声明下 1.85 工具链无法编译；CI 新增 MSRV 编译检查防止再漂移。

### 新增

- **格局判定引擎**：64 条格局的形式化判定，本命盘与运限盘共用同一套规则。
  运限视角以该层命宫为命宫、合并该层流曜与四化后重跑全部规则，
  其中禄衰马困与风云际会两条只在运限视角判定。
  一次命中给出格局标识、判定视角、成格宫位、多口径标记（`variant`）、
  破格标记（`broken`）与参与成格的星及其落宫。
  规则条目、示例盘与古籍引文取自 iztro-docs 的《格局》页（MIT License，作者 Sylar Long），
  每条规则的实现注明所采口径与取舍理由。iztro 本身没有对应 API。
- **三语言 API**：Rust 的 `Astrolabe::patterns` / `patterns_with`、
  `HoroscopeRef::patterns` / `patterns_with` 与模块函数 `patterns_at`；
  Python 的 `Astrolabe.patterns` / `Horoscope.patterns`；
  Go 的 `Astrolabe.Patterns` / `Horoscope.Patterns`（各带 Context 变体）。
- **语言无关格局标识**：Rust `PatternKey`（含 `ALL_PATTERNS`、`as_key`、`from_key`、
  `is_horoscope_only`）、Python `PatternKey` 枚举、Go `PatternXxx` 常量，三侧取值一致。
- **格局名六语言词表**：zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN，
  经 `translate_pattern` 取用。
- **`PatternConfig` 判定口径**：`brightness_source`（日月明暗按 iztro 亮度表或传统位置）、
  `borrow`（空宫是否借对宫主星）、`flow_stars`（运限流曜是否等同本命辅星）。
  同一格局的多种成立形式一律以 `variant` 报出，不在这里加开关。
- **序列化契约**：`PatternHitDto` / `PatternStarDto`（camelCase 键，译名与语言无关标识并存），
  bridge 新增 `patterns` 与 `horoscopePatterns` 两个 kind，入参含部分键的 `patternConfig`。
- **文档站**：概念页《格局》（含 64 条总表）与三语言 API 参考页，中英各一份。
- **测试**：每条规则的正反例单测、来源页 32 张示例盘的真实盘复现、
  tier1 1,560 张盘的批量不变量与命中分布检查，以及三侧共读的 DTO 输出快照
  （4 张盘 × 6 种语言）。

## [0.2.0] - 2026-08-19

### 变更（breaking）

- `by_lunar` 的闰月参数收敛为三态 `LeapMonth`（NotLeap / Leap / LeapFixed），
  取代 iztro 的 `isLeapMonth` + `fixLeap` 两个相邻布尔。
- Go 侧 `gender` 与 `language` 改为具名类型，十二宫改为定长数组。

### 新增

- Rust 核心、Python 绑定与 Go 绑定分别补齐 iztro v2.5.8 的全部公开 API，三侧能力一致。
- 文档站（fumadocs），部署至 Vercel。

### 修复

- 三语言绑定与核心的质量加固：绑定覆盖表失效、错误 code、Go 实例池、金标覆盖盲区。

## [0.1.1] - 2026-08-12

### 变更

- 补全 PyPI 项目元数据：README 描述页、项目链接、作者与 classifiers。

## [0.1.0] - 2026-08-12

首个发布版本。

### 新增

- 紫微斗数排盘核心，与 JS iztro v2.5.8 逐字段对齐，金标测试零容忍差异。
- 六个运限层级、三方四正、飞星族与盘面查询。
- 配置系统：四个分界开关与算法派别（默认派 / 中州派）。
- 三语言绑定：Rust 核心库、PyO3 Python 扩展、经 wasm 的 Go 包，另有 C FFI。
- 六种语言的翻译与语言无关标识体系。
- AI Prompt 生成：`astrolabe_to_prompt` / `horoscope_to_prompt`。
- 排盘入口 Result 化：非法输入返回带分类码的错误而非 panic。

[Unreleased]: https://github.com/x-haose/x-iztro/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/x-haose/x-iztro/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/x-haose/x-iztro/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/x-haose/x-iztro/releases/tag/v0.1.0

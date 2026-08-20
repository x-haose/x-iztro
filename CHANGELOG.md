# 更新日志

本项目的所有重要变更记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [0.3.0] - 2026-08-20

### 变更（breaking）

- **中州派重排盘的行为对齐**：`rearranged` 系列此前只有安星按重排干支起五行局，
  运限查询与 Prompt 生成（以及本版新增的格局判定）仍按原始盘计算；
  现在三者一律按重排后布局——五行局、命宫与大限随重排起点变化，出生数据不变，
  Rust / Python / Go 与 bridge 序列化入口行为一致。

### 修正

- **1602 年闰二月换算修正**：依赖 lunar_rust 1.0.1 推算农历 1602 年（明万历三十年）
  闰二月合朔晚一天，月表自相矛盾（二月 31 天、闰二月 28 天），致
  `by_solar("1602-3-24", …)` 全部时辰 panic（Go/wasm 即 trap）、
  1602-3-25 至 4-21 共 28 天静默错盘。真值经 lunar-typescript（lunar_rust 的移植上游）、
  寿星天文历 sxtwl 与韩国天文研究院历表交叉确认：1602-3-24 为闰二月初一，
  二月 30 天、闰二月 29 天。新增 `lunar_table` 修正层作为仓库读取农历月结构的
  唯一入口，按真值修正并带在场探测（依赖将来修复该表即自动停用）；
  新增金标 tier_1602（受影响窗口 2,444 例与 JS iztro 逐字节一致）与
  1583-9999 全域扫描测试（约 614 万盘无同类缺陷）。
- bridge 报错信息改进：未知 `scope` 列出全部合法值；天干/地支收到非标识串
  （如译名）时提示应传语言无关 key。
- 三处 `% 2 == 0` 改为 `is_multiple_of(2)`，以通过新版 clippy 的 `manual_is_multiple_of`。
- `rust-version` 由 1.85 改为 1.88：代码自 0.2.0 起已使用 let-chains（Rust 1.88 稳定），
  旧声明下 1.85 工具链无法编译；CI 新增 MSRV 编译检查防止再漂移。

### 新增

- **反推求解器的解析域锁定**：星耀条件先按安星几何反解出各维度的最小枚举域
  （任意主星落宫归一化为紫微落宫并做互斥检验——矛盾条件微秒级判空；干系星锁年干、
  支系星锁年支、月系星锁月、时系星锁时辰，三台/八座/恩光/天贵走日层查表通道），
  枚举只在锁剩的域上跑，多条件反推从数百毫秒降至约 10ms；终验与往返保证不变。
- **生辰反推**：由八字四柱或星盘特征反查候选生辰，iztro 无对应 API。
  `solar_dates_by_bazi` 收四柱干支与公历年范围，四柱按传入 `Config` 的分界口径解释
  （`year_divide` 年柱、`horoscope_divide` 月柱、`day_divide` 晚子归属，
  与 `raw_dates.chinese_date` 同一套语义）；一组四柱约每 60 年重复一次故天然多解，
  时柱为子时因早晚子之分可能给出相邻两天的两个候选。
  `reverse_chart` 收条件集（命宫身宫地支、五行局、星耀落宫、生年四化、年份范围、
  闰月修正、候选数上限，条件至少给一个且只收本命盘星耀），
  达到上限（默认 512）即停止搜索并置 `truncated`。
  两者都是「剪枝枚举 + 正排终验」，每个候选用同一 `Config` 正排必满足条件，
  与正向排盘零分歧；星盘布局与性别无关，故不收性别。
- **反推三语言 API**：Rust 的 `solar_dates_by_bazi` / `reverse_chart`
  （类型 `BirthCandidate` / `StarPosition` / `ReverseCriteria` / `ReverseResult`，
  常量 `DEFAULT_REVERSE_LIMIT`）、Python 的同名函数与对应 frozen dataclass、
  Go 的 `SolarDatesByBazi` / `ReverseChart`（各带 Context 变体）与 `Pillar` 类型。
  bridge 新增 `solarDatesByBazi` 与 `reverseChart` 两个 kind，入参一律语言无关标识。
- **`IztroError::InvalidArgument` 新变体**（code 沿用 `invalid_argument`）：
  反推入口的干支阴阳不配、条件为空或含流耀、年份范围非法在核心层报错；
  此前该分类只在绑定层（`BridgeError`）出现。
- **反推的文档与测试**：使用指南《反推》与三语言 API 参考页（中英各一份）；
  往返一致性（任一盘的四柱/特征反查必含原生辰、每个候选正排必满足条件）、
  逐类星耀条件的单星环测（任何一条剪枝臂不许错杀真解）、早晚子两种归属口径、
  节气分界与中州派口径下的闭环、60 年周期多解、截断语义与入参校验由
  `tests/reverse.rs`、`python/tests/test_reverse.py`、`go/iztro/reverse_test.go` 守着。
- **知识包协议与内嵌默认包**：解读文本与星耀的门派属性从内核里分出来，
  做成「语言无关标识 → 文本与属性」的 JSON（格式 v1，规范见 `knowledge/SCHEMA.md`）。
  内核只做事实判定，怎么解读是门派观点，因此可整包替换或用覆盖包逐条合并。
  内嵌的 zh-CN 默认包有 107 颗星（主星 14、辅星 14、杂耀 34、神煞 45，含卡片属性、
  特性正文与主星双星组合解读）、64 条格局（引文、成立条件、解读）、12 宫、4 化与 49 条术语，
  条目自锁定 commit 的 iztro-docs《学习》各页一次性提取（MIT License，作者 Sylar Long），
  文本整理改写为第三人称释义口吻（`source.adapted` 注明）；此后该 JSON 即手工维护的源文件，
  同步上游靠人工对照页面差异改写回填。其余五种语言暂无默认包。
- **知识包三语言 API**：Rust 的 `KnowledgePack`（`builtin` / `builtin_json` / `from_json` /
  `to_json` / `merge` / `merged` / `star` / `pattern` / `palace` / `mutagen` /
  `star_intro` / `pattern_intro`）、Python 的 `KnowledgePack`（另有 `from_dict` / `to_dict` /
  `concept` / `stars` / `patterns`，条目为 frozen dataclass）、
  Go 的 `BuiltinKnowledgePack` / `ParseKnowledgePack` / `Merged`（各带 Context 变体）与
  `Star` / `Pattern` / `Palace` / `Mutagen` / `Concept` 等取值方法。
  bridge 新增 `knowledgePack` 与 `mergeKnowledgePacks` 两个 kind。
- **知识包合并语义**：逐段按键合并，覆盖包的非空字段覆盖同键条目的对应字段，
  `attributes` 与 `combinations` 逐字段合并，数组字段整体替换，显式 `null` 等同缺省；
  合并后 `id` / `version` / `language` / `source` 取覆盖包的。
  合并只在 Rust 内核实现一处，三语言共用，结果逐字节一致；
  `schema` 高于本库支持的版本报 `invalid_argument`。
  Go 内嵌的 wasm 因内置默认包增大约 380 KB。
- **知识包的文档与测试**：使用指南《知识包》与三语言 API 参考页（中英各一份）；
  默认包的完整性与键有效性、合并语义、绑定入口分别由 `tests/knowledge_pack.rs`、
  `python/tests/test_knowledge.py`、`go/iztro/knowledge_test.go` 守着。
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
  tier1 1,560 张盘的全表命中计数金标，以及三侧共读的 DTO 输出快照
  （4 张盘 × 6 种语言 × 本命与五个运限层级；快照缺失即测试失败，
  仅 `UPDATE_PATTERN_SNAPSHOTS=1` 时显式重建）。
- Python 包补 `__version__`（与 PyPI 分发版本一致）。

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

[Unreleased]: https://github.com/x-haose/x-iztro/compare/v0.3.0...HEAD
[0.3.0]: https://github.com/x-haose/x-iztro/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/x-haose/x-iztro/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/x-haose/x-iztro/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/x-haose/x-iztro/releases/tag/v0.1.0

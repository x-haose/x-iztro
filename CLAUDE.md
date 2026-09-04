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
  - `astro/` — 排盘、运限、三方四正算法；`reverse.rs` 反推（八字四柱/星盘特征 → 候选生辰）
  - `knowledge/` — 知识包（`KnowledgePack`：解析/合并/查询），内嵌默认包 `data/knowledge/iztro_docs.zh-CN.json`
  - `pattern/` — 格局判定引擎（`view.rs` 把本命盘与运限合成盘统一成 `ChartView`，
    `rules/` 五组 64 条规则每条一个函数并注明口径来源，`keys.rs` 语言无关 `PatternKey`）
  - `i18n/` — 多语言翻译
  - `text.rs` — 语义化文本投影（to_text：本命/运限/格局/单宫/三方四正的自然语言形态，六语言标签表）
  - `error.rs` — IztroError（入口前置校验的错误类型）
  - `dto.rs` — JS 兼容序列化 DTO（三语言绑定共用）
  - `ffi.rs` — C FFI（错误 JSON + catch_unwind 兜底）
  - `wasm.rs` — wasm32 导出（Go 经 wazero 调用；wasm 上 catch_unwind 无效，防线在核心校验）
  - `python.rs` — PyO3 原生模块（`python` feature gate）
- `python/x_iztro/` — Python 包（dataclass 类型 + StrEnum 枚举，零外部依赖）
- `go/iztro/` — Go FFI 绑定包
- `examples/{rust,python,go}/` — 三个独立示例项目
- `tests/golden/` — JS 生成的金标测试数据（tier1/tier2/tier3）
- `knowledge/` — 知识包格式规范 `SCHEMA.md`（默认包 JSON 在 `src/data/knowledge/`，是手工维护的源文件）

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
- 语义化契约总纲：**一个对象三种投影**——`to_json`/DTO（机器）、`to_text`（自然语言）、译文字段（展示）；
  **key 两条命名规则**——① 有译文的属性 `x` 配套 `xKey`（数组 `xKeys`），② 实体自身标识一律叫 `key`
  （星、格局命中）。少数语义改名：运限层级与宫位的四化星数组统一叫 `mutagenStarKeys`
  （单数 `mutagenKey` 是四化类型 Mutagen 的标识，两者命名空间不同）；`time` 的语义对应是 `timeIndex`。
  守护测试 `semantic_contract`：zh-CN/en-US 双排盘逐字段并行遍历，任何随语言变化的译文字段
  缺配套 key 即失败——新增译文字段必须同时给 key
- `src/dto.rs` 定义 JS iztro 兼容的序列化 DTO：camelCase 键 + 按排盘语言翻译的值，
  附加两类扩展：排盘上下文（genderKey/timeIndex/fixLeap/language/config）与
  语言无关标识（星/宫/干支/四化/亮度/五行局/星座/生肖/运限层级名的 `*key`/`*Key(s)`，
  取值为 iztro i18n key；运限层级 `nameKey` 未起运时为 `childhood`，与 `decadal` 是不同解盘语义）
- 强类型层基于标识字段：Python 枚举（enums.py）与 Go 常量（keys.go）的值即这些 key，
  判断方法在任何输出语言的星盘上结果一致
- 绑定接口无状态：运限/to_text/格局 直接收排盘参数（含 config JSON 部分键补丁），不做星盘 JSON 往返；
  重排盘（rearranged）在语言对象里记 fromStem/fromBranch，patterns/horoscope/各 to_text kind 的
  payload 都要转发——三侧新增「再计算」入口时别漏（bridge 各入口统一 apply_rearrange）
- to_text 输出是**可直接阅读的 Markdown 子集**（`#` 标题、`- 标签: 值`、`**粗体**`、一张总览窄表；
  记号只在 `text.rs` 代码里拼，六语言标签表只放词，有测试守着），只此一种格式、不设 plain/markdown
  开关；本命文本从命宫起顺排十二宫，每宫带三方四正与宫干飞化事实行，四化写全称「化禄」。
  覆盖六个 kind：`astrolabeToText`/`horoscopeToText`/`palaceToText`/`surroundedPalacesToText`
  /`patternsToText`/`horoscopePatternsToText`；宫位寻址收 `palaceKey`（宫名 key 或
  bodyPalace/originalPalace）或 `palaceIndex`，二者必给其一。Python 挂对象方法且 `__str__` 即 to_text，
  Go 是 `ToText()`
- 知识包只作为**参数**融入 to_text：选项对象 `TextOptions`（Rust `to_text_with(&TextOptions::new()
  .knowledge(&pack))`，Go `ToTextWith(TextOptions{Knowledge})`，Python 关键字参数
  `to_text(knowledge=True|pack)`），将来的分层等开关也加在这里。bridge 六个 kind 的可选入参
  `knowledge`（`"builtin"` 取盘语言内嵌包，无该语言包即报错不回退；或直接给包对象）。带包时释义
  **内联**：每宫事实后紧跟该宫星耀释义（同宫主星组合最前，两个方向都查、每对一次；十二神不释义），
  格局列表后跟格局释义，文末附四化释义。
  结构化「按盘取材」在 `knowledge/mod.rs` 的 `for_astrolabe`/`for_horoscope`（bridge kind
  `knowledgeForChart`）：取盘上出现的星（含四组十二神，同宫主星的双星组合只留同宫对方）、
  命中格局、四化四条，不取宫位与术语；释义给全文，裁剪归应用层。文本的内联释义在
  `text.rs` 自行按宫选材（逐宫内联需要星与宫的对应，扁平子包给不出），取材范围是子包的子集——
  差在十二神：子包收、文本不出。两处口径若要改，一起改。
  排盘 DTO 永远不嵌解读
- 轻量查询 `zodiacBySolar`/`signBySolar`/`signByLunar`/`majorStarBySolar`/`majorStarByLunar`
  返回 `{text, keys}` 双轨：text 与 iztro 同名函数一致，keys 是语言无关标识；绑定层的
  iztro 对齐函数仍返回 text，命宫主星另有 keys 形态入口
- `flowStarCounterparts` kind（data 组）导出流耀 → 本命辅星的 50 条对照，源头是
  `astro/horoscope.rs` 的 `flow_star_row` 表（格局引擎的运/流对应是它的子集，有测试锁定不分叉）
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
- 布尔开关字段一律 `*bool` + omitempty（`PatternConfig.Borrow/FlowStars`、
  `ReverseCriteria.FixLeap` 先例）：nil＝省略键让内核取默认；裸 `bool` 的零值会静默
  翻转内核默认开关。便捷构造用 `Bool(v)`，新增此类字段沿用此约定

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

### 农历换算
- `src/astro/lunar_table.rs` 是仓库读取农历月结构（公历↔农历、月天数、中文月/日名）的
  **唯一入口**——lunar_rust 1.0.1 的 1602 年闰二月合朔晚一天（月表自相矛盾：二月 31 天），
  该层按权威历表真值修正并带在场探测（依赖将来修好即自动停用）。新代码不许绕过它
  直读 lunar_rust 的月表——公历↔农历、月天数、闰月归属（`leap_month`）、中文月/日名
  都走这一层；日柱/时柱与节气类取值不经月表，仍直接调 lunar_rust
- 守护：`golden_1602`（受影响窗口 2,444 例 vs JS 逐字节）+ `tests/lunar_table.rs`
  （窗口边界、by_lunar 口径；1583-9999 全域扫描标 `#[ignore]`，改换算层后实跑一遍）

### 反推（x-iztro 扩展）
- 两个入口都是「剪枝枚举 + 正排终验」：终验用与正排完全相同的函数（`four_pillars` / `by_solar`），
  与正向排盘零分歧；剪枝保守（宁多留不错杀），错杀是 bug、漏杀只是慢——
  日层剪枝对时辰敏感，`day_divide=Current` 时须以归一时辰（晚子归 0）参与，八字入口
  晚子同日给 t=0/t=12 双候选；年干候选 Exact 口径含 `year+1`（春节晚于立春年份的立春后段）
- 剪枝几何不许手抄副本：主星落宫反解的偏移取安星表（`star::major` 的
  `ZIWEI_GROUP`/`TIANFU_GROUP`），年层候选按 `year_prefilter` 逐组过滤——手抄第二份
  几何漂移时剪枝静默错杀，v0.4.0 终审清过一轮（`MAJOR_OFFSETS`/`admits_year`），别再引入
- 无外部金标，正确性由往返测试定义（`tests/reverse.rs`）：任取一盘，四柱/特征反查必含原生辰，
  且每个候选正排后必须真的得出目标；全星环测逐类星耀条件核「任何剪枝臂不错杀」
- 四柱按传入 `Config` 的分界口径解释（与 `raw_dates.chinese_date` 同语义）；星盘布局与性别无关，
  反推不收性别；`year_range` 是候选公历日期所属年份的闭区间；`reverse_chart` 条件为空、
  含流耀或含纯十二神 key 报 `InvalidArgument`（咸池/华盖/天德/龙德/大耗兼作宫内杂耀，
  不在拒绝之列），`limit`（默认 512）截断兜住宽条件
- 日柱用「lunar_rust 锚点校准 + 儒略日纯算术」推 60 周期，避免逐日构造农历对象

### 知识包（x-iztro 扩展）
- 边界：内核只做事实判定，星耀解读、格局释义、宫位/四化含义、门派属性（阴阳五行化气别号等）全在知识包；
  核心 `StarInfo` 保持与 iztro 一致，不把卡片属性并进去（卡片与 iztro 表本身有冲突，说明也是观点）
- 协议：语言无关 key（`StarKey`/`PatternKey`/`Palace`/`Mutagen` 的 `as_key`）→ 文本/属性，格式 `knowledge/SCHEMA.md`；
  合并（覆盖包按字段覆盖、数组整体替换）只在 `src/knowledge/mod.rs` 实现一处，Python/Go 经 bridge kind
  `mergeKnowledgePacks` 复用；`knowledgePack` kind 透传内嵌默认包 JSON（解析结果有缓存）。
  条目合并是 JSON 值递归（`merge_entry`），schema 新增字段自动参与——不许再写逐字段
  手抄的合并清单，那种清单在加字段时会静默漏合并且测试全绿
- 默认包只有 zh-CN，文本已由教学博客口吻整理改写为第三人称释义口吻（`source.adapted` 注明；
  只换表达不加减命理内容，唯 adapted 里逐条声明的例外：勘正原文笔误、术语依格局篇归一），
  JSON 即源头、直接手改；口吻红线由 `knowledge_pack` 的
  `builtin_texts_keep_reference_tone` 断言守护（禁「读者/大家/本站/上表/详见/参见」等残留）
- 星耀条目覆盖全部 162 个 `StarKey`（`knowledge_pack` 有反向断言兜底）：iztro-docs 有释义的
  107 条 + x-iztro 自撰的对照性条目 55 条——流耀 50 条（category `flow`，指向对应本命辅星，
  机器可读对照走 `flowStarCounterparts`）与截路/旬中/年解/劫煞/岁破 5 条（只述安放事实与组别，
  不新增解读；劫煞、岁破沿用同名神煞原释义），出处均已写进 `source.adapted`
- 同步上游：人工 diff iztro-docs 新旧 commit 的 learn 页差异，把变化条目改写后写回 JSON 并更新
  `source.commit`；一次性提取脚手架存于 docs/plan/knowledge-scaffold（gitignore，不进仓库）
- 映射类字段允许写 `null`（Go nil map 默认序列化）

### 测试
- 全部金标数据由 JS iztro v2.5.8（版本锁定）生成，在 `tests/golden/` 下，零容忍差异
- 覆盖矩阵（九层合计 716,314 例，约 72 万；另有 i18n 反查 1,559 与契约 13）：tier1 全字段 1,560（60 年 × 13 时辰 × 男女，含 rawDates）/
  tier2 紧凑 37,440 / tier3 全日期×性别×fix_leap 哈希 586,430 /
  边界年代哈希 46,228（1583-1983 与 2044-2100 每 10 年抽样，补 tier1/2/3 只覆盖 1984-2043 的盲区）/
  运限 5,760 / 变体（by_lunar 闰月逐日、中州派、六语言）14,268 /
  Config 开关（四个非默认取值 + 排盘层与运限层的组合）9,696 / 中州派盘型 12,488 /
  1602 闰二月窗口 2,444（lunar_table 修正层专属，逐字段全比对）
- tier3、边界年代、变体、astrotype、Config 排盘层的哈希都基于规范化串
  （`tests/golden/canonical.mjs` ≡ `tests/common/mod.rs`，逐字节同构；
  条目含星名/类型/范围/亮度/四化，排序等价性只在 BMP 内成立，注释里写了这个前提）；
  不一致时用生成器 `--inspect*` 重放 JS 单例与 Rust 输出 diff
- `cargo test` 跑除 tier3 外的全部层（~1min）；tier3 全量用
  `cargo test --release --test golden_tier3 -- --ignored`（~70s）
- 哪条结论由哪个测试守着：
  - 排盘数值 → `golden_tier1/2/3`、`golden_edge`（1583-1983 / 2044-2100 抽样）、
    `golden_variants`、`golden_horoscope`；农历换算修正层 → `golden_1602` + `lunar_table`
  - 序列化契约 → `golden_contract`（对 JS 的 `JSON.stringify` 逐键逐值；
    扩展键白名单按 DTO 路径限定，多在别处的键即判为契约偏离）
  - 单盘全接口面 → `regression`（工具函数、十二宫、三方四正、运限查询方法，
    金标由 `generate_regression.mjs` 生成，可复现）
  - 入口错误路径 → `error_paths`（非法日期/越界年份/时辰 13/农历 31 日等返回 Err 不 panic）
  - 绑定不漏接口 → `binding_coverage`（读 `src/bridge.rs` 与 `src/data/stars.rs` 源码文本，
    要求每个 kind 与星耀 key 都出现在 Python/Go 的非测试源码里）
  - 语义化文本 → `text_snapshot`（Markdown 形态的本命/运限/格局/单宫/三方四正五类输出 × zh-CN/en-US，
    外加五类带释义输出 × zh-CN 的固定盘完整快照 `tests/golden/text_snapshots/`；Python
    `test_parity.py` / Go `parity_check_test.go` 读事实快照，Python `test_knowledge.py` /
    Go `knowledge_test.go` 读带释义快照，均逐字节比对；缺失即失败，
    有意改动后用 `UPDATE_TEXT_SNAPSHOTS=1` 重跑该测试重建基线）
  - 语义 key 契约 → `semantic_contract`（译文字段必有配套语言无关 key，规则见「绑定契约」节）
  - 知识包 → `knowledge_pack`（默认包完整性/键与内核标识一致/FFI 合并语义）+ Python `test_knowledge.py`
    + Go `knowledge_test.go`
  - 反推 → `reverse`（八字/特征往返、甲子周期解数、Exact 口径、中州派、晚子双归属、
    全星环测 34 剪枝臂、共享 key 十二神可用、1602 窗口、limit 截断、错误路径、FFI kind）
  - 格局 → 规则单测（`src/pattern/rules/*.rs`，真实盘正/负例）+ `pattern_api`（Rust 方法/DTO/FFI 分派/口径入参）
    + `pattern_examples`（iztro-docs 页面 32 张示例盘反查真实盘）+ `pattern_distribution`（tier1 全量分布合理性）
    + `pattern_snapshot`（三侧同解基线 `tests/golden/pattern_snapshots/`，本命与五个运限层，
    Python `test_patterns.py` 与 Go `pattern_golden_test.go` 读同一批快照；缺失即失败，
    有意改口径后用 `UPDATE_PATTERN_SNAPSHOTS=1` 重跑该测试重建基线，`pattern_distribution`
    的 tally 金标随之更新）
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

## 发版

1. `git cliff --unreleased --tag v<版本>` 生成草稿（配置在 `cliff.toml`，按 conventional
   commits 归类为中文节名），逐条润色成带背景说明的条目并入 CHANGELOG.md——
   未发布功能的开发期修正并进该功能条目的最终描述，只有对已发布版本的行为修正才单列；
   `[Unreleased]` 下插入版本节，文件尾链接行同步更新。
2. `Cargo.toml` 与 `pyproject.toml` 版本号同改（release.yml 开头断言两者相等），
   `cargo check` 与 `cd examples/rust && cargo check` 分别刷新两处 Cargo.lock——发版走
   `cargo publish --locked`，lock 里本包版本不同步会挂；示例经 `path` 依赖本包，
   它的 lock 不同步则首次运行示例即产生脏工作区。
   **版本号忘了改不会报错**：release.yml 见 `v<版本>` 标签已存在就打印一行跳过并成功退出，
   PR 合并、CI 全绿、什么都没发。合并后按第 4 步核验实际产物。
3. dev → main 发 PR，正文按模板写「## 发布说明」节（GitHub Release 描述取自该节）；
   合并即自动发版：金标门禁 → crates.io（OIDC）→ `v*` 与 `go/iztro/v*` 双标签
   → GitHub Release → 派发 wheels.yml 发 PyPI。
4. 合并后核验四样实际产物到位：crates.io 新版本、PyPI wheels、`v*` 与 `go/iztro/v*` 两个标签、
   GitHub Release。CI 绿不等于发出去了。

## 常用命令速查

```bash
cd examples/rust && cargo run                  # Rust 示例
cd examples/go && go run .                     # Go 示例
python examples/python/main.py                 # Python 示例
cargo test --test golden_tier1 -- --nocapture  # 单跑 Tier 1 看输出
nm target/release/libx_iztro.dylib | grep iztro  # 检查 FFI 符号导出
```

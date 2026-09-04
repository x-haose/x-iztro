# 更新日志

本项目的所有重要变更记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

## [0.5.0] - 2026-09-04

### 变更（breaking）

- **to_text 输出改为 Markdown 子集**，事实与释义同处一屏：`# 命盘` 标题 → `## 基本信息`
  （生年四化带落宫、身宫与来因宫带宫名）→ `## 十二宫总览`（宫位 | 主星 | 辅星 | 大限）→
  `## 格局` → `## 十二宫`（从命宫起顺排，每宫 `### 宫名 (干支) · 大限` 与事实行）。
  按「标签: 值」逐行解析或用 `=== 节 ===`/`--- 宫名 ---` 切段的下游需按新记号调整。
  运限文本同步改为各层 `## 大限 · 命宫: 本命X (干支)` 节，大限与流年展开十二宫表。
- **四化改写全称**：`天同(平)化权` 取代 `天同(平)[权]`，与禄存的「禄」不再同形；英文为 `[A]`。
  破格由 `[破格]` 改为格局括号内标注 `(命宫, 破格)`。
- **`to_text_with` 收 `TextOptions`**：Rust `to_text_with(&TextOptions::new().knowledge(&pack)
  .pattern_config(&cfg))`、Go `ToTextWith(TextOptions{Knowledge, PatternConfig})`；Python
  `to_text(knowledge=…, config=…)`。格局口径从此对文本的格局节、释义与 bridge 的 `patternConfig`
  入参一并生效。`palace_to_text` 收 `&PalaceRef`（单宫的三方四正与飞化行需要全盘），
  `SurroundedPalaces::to_text` 不再收语言参数（按所属星盘语言）。
- **带释义文本不再为四组十二神出释义**：十二神只保留在每宫的事实行（标组名），其一句话条目
  仍在知识包与按盘取材的子包里，需要时按 key 自取。
- **运限的宫名行改为成对映射**：小限与流月及以下不展开十二宫表，此前只写一串该层宫名，
  且按宫位索引排——既非命宫起也非本命宫序，读者无从还原对应关系。现从该层命宫起写
  `命宫→本命官禄, 兄弟→本命田宅, …`，与大限、流年十二宫表的两列同序同义。
- **格局文本带文档标题**：`patternsToText` / `horoscopePatternsToText` 输出以 `# 格局` 起，
  零命中时为标题加 `—`。此前零命中返回空串，调用方分不清「确实没有格局」与「参数没生效」。
- **Go 的 `PatternsToTextWith` 去掉位置参数 `config`**：格局口径统一取 `opts.PatternConfig`，
  与 Go 侧其余 `ToTextWith` 家族一致。此前两处都能给口径而 `opts` 静默压掉位置参数。
- **`SurroundedPalaces` 不再可由结构体字面量构造**：新增私有字段回指所属星盘（单宫与三方四正
  文本要按全盘算飞化与会照）。自建三方四正视图的下游改用 `Palace::surrounded_palaces()`。

### 新增

- **知识包融入 to_text**：`TextOptions` 带知识包时，每宫事实之后紧跟该宫星耀的释义（同宫主星的
  组合解读在前，包里只记在一方名下也查得到），格局列表之后紧跟释义（含成立条件），文末附四化释义；
  运限各层附流耀与格局释义（跨层去重）。bridge 六个 to_text kind 增可选入参 `knowledge`
  （`"builtin"` 取盘语言内嵌包、无该语言包即报错；或直接给包对象），新 kind `knowledgeForChart`。
- **按盘取材**：`KnowledgePack::for_astrolabe(_with)` / `for_horoscope(_with)` 裁出本盘相关子包
  （盘上的星含四组十二神、同宫主星组合、按口径命中的格局、四化四条；不含宫位与术语），
  返回仍是标准包；Python `pack.for_astrolabe(chart, config=None)`、Go `pack.ForAstrolabe(chart, cfg)`
  与 `Knowledge.ForAstrolabe`（内嵌包哨兵不搬整包）。
- **事实层补齐**：每宫新增三方四正行（对宫 · 三合）与宫干飞化行（四化星带落宫）；十二神标组名
  （长生·/博士·/岁前·/将前·）；空宫写明「空宫」；运限流耀行标注落宫（`流魁→命宫`）。
- to_text 快照金标扩为 15 份（五类 × zh-CN/en-US + 五类带释义 × zh-CN），三侧读同一批逐字节比对。

### 修正

- **反推在地盘 / 人盘（`astroType`）下静默错杀真解**：地盘与人盘在排盘末尾按身宫、福德宫干支
  整体重排，星耀落宫、命宫身宫与五行局随之移位，而反推的四层剪枝一律按天盘安星表几何计算，
  终验读的却是重排后的盘——两端口径不一致，真解在到达终验前就被剪掉。表现分两档且都无声：
  给命宫地支或五行局时候选常为空，给星耀落宫时返回非空但残缺的列表（每条单独看都满足条件，
  唯独原生辰不在其中）。重排偏移取决于身宫位置、本身是待求量，无法在剪枝前反解，故非天盘
  一律跳过几何剪枝，退化为逐日枚举加终验：慢，但不错杀。0.4.0 及更早版本均受影响。
- **知识包里五条兼作十二神与宫内杂耀的条目释义语境错位**：华盖、咸池、天德、大耗（连同只作
  十二神的病符、小耗）的正文按源站分节拼接，首段限定于十二神语境。本版起十二神不出释义，
  这些条目只剩杂耀出口，於是出现「事实行说华盖坐本命夫妻宫、释义却说仅影响流月流日」的
  自相矛盾。现改写为一段：杂耀语境在前、十二神语境作补充，并去掉与释义标题同形的源站分节标题。
- 内嵌知识包的 JSON 值缓存改为按语言分槽并移到内核数据源旁——此前单槽不分键，
  今天只有 zh-CN 一份包故无碍，一旦新增第二个内嵌语言包即会静默串包。
- `for_astrolabe` / `for_horoscope` 的四化条目改为按 key 取，与星、格局同一口径；
  此前整段照抄，包里的非法键会混进子包。

### 内部

以下四项不改变任何输出，是防漂移与提速：

- 反推按年枚举先查 `lunar_table::leap_month` 跳过无效闰月试探——v0.4.0 为消除对 lunar_rust 月表的
  直读，把这层预筛一并删了，现经修正视图加回；枚举结果不变，反推测试 debug 构建 14.3s → 约 9s。
- 反推剪枝的主星落宫偏移改由安星表（`ZIWEI_GROUP` / `TIANFU_GROUP`）派生，年层锁域去掉手抄副本——
  手抄的第二份几何一旦与安星表漂移，剪枝会静默错杀候选而测试全绿。
- 知识包解析结果加缓存，内嵌包由 `const` 改 `static`（wasm 不再嵌两份 JSON）。
- 知识包条目合并改为 JSON 值递归，替代逐字段手抄的合并清单——schema 新增字段自动参与合并。

## [0.4.0] - 2026-08-21

### 变更（breaking）

- **Prompt 更名 to_text，定位为「语义化文本投影」**：文本输出不再叫「AI 提示词」——
  它是与 `to_json`（机器格式）、译文字段（展示）并列的第三种投影：同一份盘面事实的
  自然语言形态，提示词只是用途之一。Rust 模块 `prompt` 更名 `text`，
  `astrolabe_to_prompt` / `horoscope_to_prompt` 更名 `astrolabe_to_text` /
  `horoscope_to_text`；bridge kind `astrolabeToPrompt` / `horoscopeToPrompt` 更名
  `astrolabeToText` / `horoscopeToText`；Python 删除 `Astro.astrolabe_to_prompt` /
  `horoscope_to_prompt`，改为对象方法 `Astrolabe.to_text()` / `Horoscope.to_text()`
  （`str()` 即 to_text）；Go 的 `AstrolabeToPrompt` / `HoroscopeToPrompt` 更名
  `Astrolabe.ToText` / `Horoscope.ToText`。一律不留旧名别名。
- **运限层级的 `mutagenKeys` 更名 `mutagenStarKeys`**（Python `mutagen_star_keys`、
  Go `MutagenStarKeys`）：与宫位的同名字段同义对齐。此前单数 `mutagenKey` 装四化类型
  标识（`sihuaLu` 等）、复数 `mutagenKeys` 却装被化四星的星耀标识——差一个字母就换
  命名空间；更名后「`mutagenKey`＝四化类型、`mutagenStarKeys`＝被化的星」泾渭分明。
- **轻量查询返回双轨对象**：bridge 的 `zodiacBySolar` / `signBySolar` / `signByLunar` /
  `majorStarBySolar` / `majorStarByLunar` 由裸译文串改为 `{text, keys}`——`text` 与
  iztro 同名函数一致，`keys` 是同一结果的语言无关标识（命宫主星从此接得上知识包）。
  Python / Go 的同名函数签名与返回不变（取 `text`），直连 FFI/wasm 的消费方需按新形状解析。
- **Rust 的 `rearranged` 返回 `Result`**：此前对非法 `raw_dates`（反序列化或手工构造的盘把
  农历月改成月表中不存在的月份）直接 panic——wasm 上 panic 即 trap 且损耗共享实例；
  现返回 `IztroError::Internal`。排盘入口产出的盘重排必成功；Python / Go 走 bridge 不受影响。

### 修正

- **反推条件校验漏放流月/流日流耀**：v0.3.0 的 `reverse_chart` 只拒绝运/流/时三个层级的
  流曜，条件含 `yuechang`（月昌）等流月/流日流曜时通过校验并静默返回空结果，
  看起来像真的无解。现按全量流曜对照表判定，五个层级共 50 颗一律报 `invalid_argument`。
- 反推枚举的闰月判定不再直读 lunar_rust 月表，统一经 `lunar_table` 修正视图——
  月表读取的唯一入口约定从此无例外。

### 新增

- **语义化 key 契约成文并全量补齐**：契约两条规则——有译文的属性 `x` 配套 `xKey`
  （数组 `xKeys`）、实体自身标识叫 `key`。补上最后三处缺口：星盘的 `signKey`（星座）与
  `zodiacKey`（生肖）；运限层级的 `nameKey`——大限层未起运时为 `childhood`（童限），
  与 `decadal` 是不同的解盘语义，此前只能比对译文区分。新增 `semantic_contract` 测试：
  同一张盘按 zh-CN 与 en-US 双排盘逐字段并行遍历，任何随语言变化的译文字段缺配套
  key 即失败——契约从文档承诺变成 CI 强制。
- **to_text 覆盖五类对象**：本命盘、运限之外新增格局命中（`patterns_to_text`）、
  单宫（`palace_to_text`）与三方四正（`surrounded_palaces_to_text`）的文本投影；
  三侧同名动词（Rust 方法 / Python 对象方法 / Go `ToText` 与 Context 变体），
  bridge 新增 `palaceToText` / `surroundedPalacesToText` / `patternsToText` /
  `horoscopePatternsToText` 四个 kind，宫位寻址收 `palaceKey`（宫名 key 或
  `bodyPalace` / `originalPalace`）或 `palaceIndex`，必须显式给出，缺省报错。
- **文本内容升级**：本命文本带「格局」节；运限各层带格局行（同名同宫命中去重）；
  流月/流日/流时补流耀行（标注落宫）；小限补重排宫名与四化；各层四化行逐星标注
  禄权科忌（「太阳禄, 武曲权, …」，与生年四化同款，不再要求读者记顺序约定）；
  未起运的盘大限段正确标注「童限」（此前误标「大限」）。
- **文本语言与排盘语言解耦**：`text` 模块的自由函数传任意语言都输出纯该语言文本——
  星名、时辰、星座、干支、流耀一律按语义 key 现翻，zh-CN 盘按 en-US 渲染与原生
  en-US 盘的输出逐字节一致（有等价测试锁定）。
- **知识包星耀条目补至 162 全覆盖**：新增流耀 50 条（新类别 `flow`，指向对应本命辅星，
  按随天干/随地支分写安放依据）与截路、旬中、年解、劫煞、岁破 5 条（只述安放事实与
  组别归属，不新增解读；劫煞、岁破沿用同名神煞原释义，出处逐条写进 `source.adapted`）。
  测试补反向断言：内核每个星耀标识必有条目，漏写即失败。
- **流耀对照表**：`flowStarCounterparts` kind（Rust `flow_star_counterparts` /
  `natal_counterpart_of_flow_star`，Python `flow_star_counterparts`，
  Go `FlowStarCounterparts`）导出 50 条「流耀 → 对应本命辅星」，如 `liuchang` →
  `wenchangMin`；安星与格局引擎共用同一张源表，两处一致性有测试锁定。
- **命宫主星 keys 轨**：Rust `major_star_keys_of_soul_palace`、Python
  `get_major_star_keys_by_*`、Go `MajorStarKeysBy*`——借宫规则与译文轨共享同一实现。
- **to_text 快照金标**：五类输出 × zh-CN / en-US 十份完整快照
  （`tests/golden/text_snapshots/`），Rust / Python / Go 读同一批逐字节比对；
  仅 `UPDATE_TEXT_SNAPSHOTS=1` 时显式重建。

### 工程

- 核心库以 `#![deny(unsafe_code)]` 锁定 unsafe 只出现在 FFI / wasm 边界，
  由编译期保证而非约定。
- CI 的 Python 矩阵作业加聚合门，分支保护可按固定作业名等待。
- README（中英）、文档站首页与视觉资产全面重写；文档站按语义化契约对齐
  （《AI 提示词》页更名《语义化文本》，数据模型页收录契约总纲）。

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

[Unreleased]: https://github.com/x-haose/x-iztro/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/x-haose/x-iztro/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/x-haose/x-iztro/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/x-haose/x-iztro/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/x-haose/x-iztro/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/x-haose/x-iztro/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/x-haose/x-iztro/releases/tag/v0.1.0

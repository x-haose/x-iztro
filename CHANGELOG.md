# 更新日志

本项目的所有重要变更记录于此。

格式遵循 [Keep a Changelog](https://keepachangelog.com/zh-CN/1.1.0/)，
版本号遵循 [语义化版本](https://semver.org/lang/zh-CN/)。

## [Unreleased]

### 新增

- **主题选宫结果带上格局**：`TopicSelection` 新增 `patterns`，即主题文本「格局」一节逐条所列的那批命中（所选层视角成格、
  成格宫在选中宫内；只读大限时不含风云际会），文本与结构化结果由同一次选宫产出。每条只含语言无关标识与宫位索引：
  `key`、`scope`、`palaceIndex`、`palaceNameKey`（所选层的宫名）、`variant`、`broken`、`stars`。格局口径可指定：
  Rust `Astrolabe::topic_palaces_with` / `HoroscopeRef::topic_palaces_with`，Go `TopicPalacesWith`（另有 Context 变体），
  Python、JavaScript 的 `topic_palaces` / `topicPalaces` 收 `config`，bridge kind `topicPalaces` / `horoscopeTopicPalaces` 收 `patternConfig`。
- **附加四化星**：有的流派一干化出多于四颗星（如庚干天同、天相同化忌），四槽的四化表放不下。`Config` 新增
  `extraMutagens`（天干标识 → 四化标识 → 星耀标识列表，如 `{"gengHeavenly": {"sihuaJi": ["tianxiangMaj"]}}`），
  加在该天干生效的四槽（自定义或默认）之外。附加星与四槽之星同等进入星耀的四化标记、宫干飞化、运限各层的四化、
  格局判定、文本（生年四化、宫干飞化、各层四化与一览表）、主题选宫的飞化与四化落宫，以及反推的生年四化条件。
  四槽结构与 DTO 的四化数组不变；宫位与运限层另有 `extraMutagenStarKeys`（按四化分组），只在有附加星时出现。
  一类四化化出不止一颗星时，`flies_to` / `self_mutaged` 有任一颗落在目标宫即算这一类飞入，`mutaged_places` 只按四槽。
  同一天干里一颗星在四槽与附加星合计至多出现一次，否则排盘与反推报 `invalid_argument`（四槽本身有重复的自定义表也一并拒绝）。
  Rust `Config::with_extra_mutagens` / `stem_mutagens` / `mutagen_of` / `check`、`StemMutagens`、`HoroscopeItem::extra_mutagen`
  与 `mutagens()`；Python `ChartConfig.extra_mutagens`，Go `Config.ExtraMutagens`，JavaScript `ChartConfig.extraMutagens`。
- **截空、旬空分正傍**：`Config` 新增开关 `voidPlacement`（Rust `void_placement: VoidPlacement`）。默认 `default` 与 iztro 相同；
  `zhengPang` 时截空、旬空各安正空、傍空两宫（阳年阳宫为正空、阴年阴宫为正空，王亭之《安星法及推断实例》p56–57）：
  两派都在正空安截空、傍空安新星「傍截空」（`pangJiekong`），默认派不再安截路、空亡；旬空的傍空安新星「傍旬空」（`pangXunkong`）。
  格局引擎的空亡星集合把两颗傍空星算上，空曜集合只取正空。星盘 DTO 的 `config` 只在取非默认值时带这个键。
  新星曜六种语言都有译名，默认知识包补两条只述安放事实的对照性条目（`source.adapted` 已注明），星耀标识由 162 个增至 164 个、
  可翻译标识由 260 个增至 263 个。
- **亮度「闲」**：亮度新增一级 `xian2`（闲，王亭之《安星法及推断实例》六级、《紫微斗数一本通》第七章七级里排在平之下、陷之上），
  标识取带声调数字的拼音以与陷（`xian`）区分。iztro 没有这一级，默认亮度表不出现，只能经自定义亮度表使用；
  格局判定的日月明暗不把它算作「暗」。Python `Brightness.XIAN2`，Go `BrightnessXian2`，JavaScript `Brightness.XIAN2`。
- **杂耀可带亮度**：自定义亮度表对杂耀同样生效（含重排时随命宫挪位的天伤、天使、天才）。默认亮度表不给杂耀亮度，
  因此默认输出不变；杂耀有亮度时 DTO 才带 `brightness` / `brightnessKey`，文本随之写出亮度。
- **更多流派口径开关**：`Config` 新增八个开关，`dayDivide` 新增一个取值；默认值都等于原行为，星盘 DTO 的 `config`
  只在取非默认值时带这些键。Python `ChartConfig`、Go `Config`、JavaScript `ChartConfig` 同名字段（Python 为 snake_case），
  取值枚举 Python `enums.py`、Go `keys.go`、JavaScript `enums.ts` 同步。
  - `dayDivide: "forwardCarry"`：晚子时整个日期归次日，按次日早子时排盘，月底的晚子进下月、除夕的晚子进下一年
    （李铁笔《紫微斗数命运学》p105–106）；农历生日、生月、年干支、生肖与四柱都按次日，`solarDate`、时辰与星座按入参；
    运限目标的晚子时同样按次日早子时。
  - `leapDivide`（`default` / `nextMonth`）：`nextMonth` 时闰月整月按下月安星，`fix_leap` 不起作用
    （《紫微斗数讲义》p33–34、《紫微斗数精成》p11）；斗君的生月随之。
  - `tianmaPlacement`（`yearly` / `monthly` / `both`）：天马按年支、按生月（正五九申、二六十巳、三七十一寅、四八十二亥，
    李铁笔《紫微斗数命运学》p20），或年马之外另在月马之位安新杂耀「月天马」（`yuemaAdj`，《紫微斗数一本通》p80、p84、p91）。
  - `changshengDirection`（`yinYang` / `gender` / `clockwise`）：长生十二神男顺女逆（《紫微斗数精成》p22、李 p29）或一律顺行
    （《紫微斗数讲义》p64）。
  - `tianshangTianshi`（`default` / `tianshiFriends`）：天使在仆役、天伤在疾厄，不分男女（李 p105、p135）。
  - `tianwuPlacement`（`siShenYinHai` / `siShenHaiYin`）：天巫三七十一亥、四八十二寅（《紫微斗数精成》p20、p26）。
  - `yearly12Placement`（`natalAndYearly` / `yearlyOnly`）：岁前、将前十二神只作流年星，本命不安（《紫微斗数一本通》p44、
    李 p115）。本命不安时宫位 DTO 的 `suiqian12`、`jiangqian12` 及其标识为空串，字段与类型不变；文本的十二神行只写长生、博士。
  - `originalPalace`（`primary` / `withSecondary`）：辛年丑宫、壬年子宫另标辅来因宫（许铨仁《紫微斗数命理学正解（二）》p12
    以寅卯为主、子丑为辅）。宫位新增 `isSecondaryOriginalPalace`（只在为真时出现），文本标「辅来因宫」；
    宫位寻址的 `originalPalace` 仍指寅卯那一宫。
  - `douJunMonth`（`default` / `natal`）：`natal` 时推斗君的生月与本命盘同一规则，即 `dayDivide=forward` 的晚子时不把闰月
    十六日起作下月。这是内核自身的一致性开关（iztro 本命与斗君对晚子时取生月的做法不同），不是哪一家的口径，流派预设不设它。
- **格局口径 `pangKong`**：`PatternConfig` 新增 `pangKong`（Rust `pang_kong`，默认 `false`），打开时空曜（禄衰马困、
  两重华盖的宽口径）连傍截空、傍旬空一并算（王亭之《安星法及推断实例》p56–57 正空、傍空两宫都算）。取默认值时序列化不写出该键。
  Python `PatternConfig.pang_kong`（`to_dict()` 恒写出 `pangKong`），Go `PatternConfig.PangKong`，JavaScript `PatternConfig.pangKong`。
- 新星曜「月天马」六种语言都有译名（书中称月马；iztro 的流月天马已叫「月马」，本命这颗译作「月天马」以免同名），
  默认知识包补一条只述安放事实的对照性条目（`source.adapted` 已注明）；星耀标识由 164 个增至 165 个、可翻译标识由 263 个增至 264 个。
- 四侧共用用例 `tests/data/school_option_cases.json`：每个新开关一张盘、格局口径一组，运限口径另有 `horoscopeCases`（各层落宫、干、四化所用天干与四化星），Python、Go、JavaScript 经各自的类型化配置比对。
- **运限流派口径开关**：`Config` 新增九个开关，`horoscopeDivide` 新增一个取值；默认值都等于原行为，星盘 DTO 的 `config`
  只在取非默认值时带这些键。各层的落宫、宫名、四化、文本、主题选宫与运限格局都按所选口径算。Python `ChartConfig`、
  Go `Config`、JavaScript `ChartConfig` 同名字段，取值枚举 Python `enums.py`、Go `keys.go`、JavaScript `enums.ts` 同步。
  - `horoscopeDivide: "exactMonth"`：流月干与月柱按节气换月，流年仍按正月初一换年（令东来《紫微斗数命理学》p160–161；
    依婷《如何看大限流年》p215）。iztro 的 `exact` 把两者绑在一起；流月命宫三种取值都按农历月。
  - `decadalStem`（`palace` / `continuous`）：大限天干以命宫宫干起，按天干次序随大限方向连续排（《紫微斗数精成》p398）；
    宫位上的大限干、大限四化与运曜随之。
  - `childhoodPalace`（`sequence` / `spouse`）：起运前的童限一律落夫妻宫，以夫妻宫为命宫推排宫名、取夫妻宫干起四化
    （依婷系统班 02 讲 p14–17）。
  - `ageMutagens`（`palaceStem` / `none`）：小限不起四化（《紫微斗数精成》p433）。
  - `yearlyPalace`（`taisui` / `age`）：流年命宫取小限宫（《天纪紫微斗数笔记》p13，书中 p35、p59 命例核对得上）；
    流年干支、流耀、岁前将前与斗君仍按流年年支与太岁宫。
  - `scopeMutagenStem`（`scope` / `scopeSoulPalace`）：运限各层四化取该层命宫的本命宫干（周星飞《梁若瑜飞星派紫微斗数课程》p21，
    方外人《河洛紫微斗数》p28，蔡明宏《华山钦天四化紫微斗数飞星秘仪》书页 50、PDF p37 甲法）。
  - `monthlyPalace`（`douJun` / `monthBranch`）：流月正月固定在寅宫（蔡明宏《秘仪》书页 51–52、PDF p37–38 乙法）。
  - `dailyPalace`（`fromMonthly` / `afterMonthly`）：流日初一落在流月命宫的下一宫（《梁若瑜飞星派紫微斗数课程》p264）。
  - `leapDaily`（`default` / `continuous`）：闰月前半月的流日接同号本月的日数连续数（王亭之《安星法及推断实例》p85）；
    原文对闰月后半月读不出确定算法，十六日起仍按默认。
  - `hourlyPalace`（`fromDaily` / `branch`）：流时按时支落宫，子时在子宫（许铨仁《学习紫微斗数的第一本书》p52–53，
    《紫微斗数讲义》p79）。
- **运限层标出四化所用的天干**：运限各层新增 `mutagenStem` / `mutagenStemKey`（Rust `HoroscopeItem::mutagen_stem`），
  只在与该层天干不同（`scopeMutagenStem=scopeSoulPalace`）或不起四化（小限 `ageMutagens=none`，为空串）时出现，默认输出不带。
  文本的四化行此时注明所用天干（`- 四化 (辛干): …`），不起四化时不写四化行。主题选宫的 `scopeMutagenStemSource` 随配置取
  `scope` 或新值 `scopeSoulPalace`，逐条的 `stemKey` 即实际所用的天干。Go 新增 `StemSource*` 常量。
- **亮度「地」**：亮度新增一级 `di`（地），照录王亭之《安星法及推断实例》p101–102 庙陷总表（贪狼卯、天梁酉、化禄巳三格）。
  该书 p107 的术语只释入庙、坐旺、落陷、平闲，没有给「地」下定义，所以不把它并入得或别的哪一级；默认亮度表不出现，
  只能经自定义亮度表使用。格局判定的明暗只分庙旺（明）与陷不（暗），得、利、平、闲、地都在两者之间，「地」按「得」
  还是按「平、闲」处理结果相同，所以没有另设格局口径。六种语言都有译名（英文、韩文记作 `[di]`、`[지]`），
  可翻译标识由 264 个增至 265 个。Python `Brightness.DI`，Go `BrightnessDi`，JavaScript `Brightness.DI`。

### 变更

- **Go 绑定在 arm64 上改用 wazero 解释器**，其余架构仍用编译器：wazero（v1.8.2 至 v1.12.0）的 arm64 编译器在不同的代码形状下
  会把本库的 wasm 编错，表现为越界访问或越界检查失败而中止（[wazero#2590](https://github.com/wazero/wazero/issues/2590)），
  同一份 wasm 用解释器或在 amd64 上用编译器都正确。输出不变，arm64 上变慢：每次排盘约 9 毫秒（编译器约 0.2 毫秒），
  实例化约 40 毫秒；arm64 不再建编译缓存，`CompilationCacheDir` 返回空串。wazero 修复后换回编译器。
- Go 内嵌的 wasm 编译时把工具链、cargo 缓存与仓库目录重映射成 `/rust`、`/cargo`、`/src`，产物里不再带构建机的本机路径
  （此前各版本的内嵌 wasm 在 panic 位置串里带构建机目录）。
- Rust 的 `Config` 多一个公有字段 `void_placement`、`HoroscopeItem` 多一个公有字段 `extra_mutagen`、`YearlyStars` 多
  `pang_jiekong` / `pang_xunkong`：用结构体字面量构造的代码需补上。`Brightness` 新增变体 `Xian2` 并标为 `#[non_exhaustive]`，
  crate 外的 `match` 需加兜底分支。`TopicPalace::flying` 由 `[Flight; 4]` 改为 `Vec<Flight>`（有附加四化星时多于四条）。
  `star::adjective::get_adjective_stars` 的末两个参数由 `algorithm, lang` 改为 `lang, config`。
- Rust 的 `Config` 多八个公有字段，`PalaceData` 多 `is_secondary_original_palace`，`suiqian12` / `jiangqian12` 由 `StarKey`
  改为 `Option<StarKey>`（本命不安时为 `None`），`PatternConfig` 多 `pang_kong`：用结构体字面量构造的代码需补上。
  `DayDivide` 新增变体 `ForwardCarry` 并标为 `#[non_exhaustive]`，八个新枚举同样 `#[non_exhaustive]`。
  安星构件的签名随开关改变：`get_changsheng12` 多一个方向参数，`get_monthly_star_index` 多天巫安法参数，
  `get_tianshang_tianshi_index` 与 `get_yearly_star_index` 的末参数由 `Algorithm` 改为 `&Config`，`get_adjective_stars` 多月天马位置参数。
- Rust 的 `Config` 多九个运限口径公有字段，`HoroscopeItem` 多 `mutagen_stem`：用结构体字面量构造的代码需补上。
  `HoroscopeDivide` 新增变体 `ExactMonth` 并标为 `#[non_exhaustive]`，`Brightness` 新增变体 `Di`，九个新枚举同样
  `#[non_exhaustive]`，`StemSource` 新增 `ScopeSoulPalace`。`astro::palace::get_decadals_and_ages` 多一个 `DecadalStem` 参数。
- 运限取出生的农历日期改读星盘的 `raw_dates`（排盘入口产出的盘与按 `solar_date` 重算的结果相同），
  不再解析 `solar_date`；`get_horoscope` 因此不再因星盘的 `solar_date` 被改成非法值而报错。
- DTO 里非四化候选星（十四主星与左辅右弼文昌文曲之外）在自定义四化表让它化出四化时，`mutagen` 译文键随 `mutagenKey` 一起出现，
  此前只有标识没有译文。默认四化表只化四化候选星，默认输出不变。

### 修正

- **斗君的生月按星盘的 `fix_leap` 取**：推流月宫（斗君）时生月一律按「闰月十六起作下月」，不看星盘的 `fix_leap`；
  `fix_leap=false` 的盘本命按闰月整月本月安星，流月却按下月起，两处口径不一。现在 `fix_leap=false` 时闰月整月按本月计；
  只影响 `fix_leap=false` 且生在闰月十六日及以后的盘，这些盘的流月宫（及其下的流日、流时宫）顺移一宫。
  `fix_leap=true`（默认）的盘不变，与 iztro 一致。仍有一处例外：本命盘取生月时晚子时（时辰 12）出生不按下月计，
  斗君不排除晚子时（iztro 现行行为，金标锁定）；`fix_leap=true`、闰月十六日及以后晚子时出生的盘，两处生月相差一个月。

## [0.10.0] - 2026-11-27

### 新增

- **在运限某一层读主题**：此前主题只按本命盘选宫，选了流月、流年或大限去读「兄弟」「财运」，读到的仍是本命兄弟宫、财帛宫。
  现在可以在大限、流年、流月、流日、流时任一层读主题：主题里的宫名按该层的宫名落到盘上（流月兄弟宫即以流月命宫起排的兄弟宫），
  三方四正以这一宫为本宫起算；`bodyPalace` / `originalPalace` 在任何一层都落在本命那一宫（身宫、来因宫是本命盘上的标记，
  运限只轮换宫名，不重算它们）。层级不能比运限的查询精度细。
  - 文本：Rust `HoroscopeRef::topic_to_text` / `text::horoscope_topic_to_text`，Python、JavaScript `Horoscope.topic_to_text` /
    `topicToText`，Go `Horoscope.TopicToText`，bridge kind `horoscopeTopicToText`。标题写运限时间与命盘，标题下逐层写大限至该层的
    命宫位置与四化；每宫段落写该层宫名并补本命宫名（`### 本宫 · 流月兄弟 (本命福德 · 癸卯)`），宫内星曜照本命，另写各层落在本宫的
    四化与流耀；三方四正行用该层宫名、宫干飞化的落宫标明本命；格局取该层视角在选中宫内的命中。`origin` 层与本命主题文本逐字相同，
    本命主题文本本身不变。
  - 选中的宫与三类四化：Rust `Astrolabe::topic_palaces` / `HoroscopeRef::topic_palaces`（`TopicSelection`，`to_dto()`），
    Python `topic_palaces`，Go `TopicPalaces`，JavaScript `topicPalaces`，bridge kind `topicPalaces` / `horoscopeTopicPalaces`。
    返回各组位置、每个选中宫的角色、该层宫名与本命宫名、宫干飞化（禄权科忌四条，自化单独标出），以及落在选中宫里的生年四化和
    大限至该层各层的四化（每条标明层级，未起运的大限层为 `childhood`），全部是语言无关标识与宫位索引，供界面高亮、逐步阅读与
    画飞化连线；与主题文本同一套选宫，`palaces` 的顺序即文本的成段顺序。天干取法标明在结果里：`palaceStemSource`（宫干飞化，现为本命宫干 `natalPalace`）、
    `scopeMutagenStemSource`（运限四化，现为该层自身的干 `scope`），每宫与每条运限四化另带所用天干的 `stemKey`。
  - 四侧共用用例 `tests/data/topic_palaces_cases.json` 覆盖五个运限层、童限、身宫与来因宫，期望由 Rust 测试按各层宫名另行推得核对；
    文本快照新增流月层婚恋主题。

- **知识包主题的显示顺序**：主题条目新增可选字段 `order`（0 至 4294967295 的整数）。列出全部主题时按 `order` 升序，
  没有 `order` 的排在最后，相同或都没有时按 slug 的字节序。此前格式里没有主题顺序，各语言按 slug 字母序列出，
  应用只能自己维护一份排序表。键在 JSON 里的书写顺序不算数：JSON 对象成员无序，内核解析与合并、Go 的 map、
  JavaScript 对纯数字键的属性排序都会重排。`order` 是普通标量字段，覆盖包可以单独改某个主题的位置，或给新增的主题插位。
  Rust 新增 `KnowledgePack::topics_in_order`，Go 新增 `TopicEntry.Order`（`*uint32`）与 `KnowledgePack.TopicSlugs`，
  Python、JavaScript 的 `TopicEntry` 新增 `order`；`order` 不是 0 至 4294967295 的整数时校验报 `wrong_type`（error）。
  默认包 13 个主题按总论、性格、事业、财运、婚恋、健康、父母、子女、兄弟、交友、田宅、迁移、精神以 10 为步长编号。
- **默认包术语「亮度」**（`concepts.liang-du`）：亮度概念的由来、四级（庙旺利陷）与七级（庙旺得利平不陷）两种分法、
  庙最亮陷最暗，以及亮度如何影响星曜的表现。取自同一锁定 commit 的 iztro-docs《学习》星曜篇「星曜的亮度」一节，
  按默认包的口吻改写，`source.adapted` 已注明；该节的亮度表由内核排盘给出，不收。默认包术语由 50 条增至 51 条。

### 变更（breaking）

逐条的改法见[从 0.9 升级到 0.10](https://x-iztro.x-haose.com/zh/docs/guide/about/upgrade-0-10)。

- Python、JavaScript 的 `KnowledgePack.topics()` 按上述显示顺序返回，此前按 slug 字母序；没有写 `order` 的包结果不变。
  默认包因此按总论、性格、事业……的顺序返回。
- Rust 的 `TopicEntry` 多一个公有字段 `order`：用结构体字面量构造它的代码需补上该字段或 `..Default::default()`。
  Go 的 `TopicEntry` 多一个字段 `Order`，不带字段名的结构体字面量需补上。

### 变更

- 默认包婚恋、精神两个主题的说明由「命宫、身宫看本人」改为「命宫看本人，身宫看后天的着力之处」，与总论、性格两条一致：
  在运限层读主题时命宫随层、身宫留在本命，两宫不再写成同一个角色。主题文本的标题下说明随之改变。

## [0.9.0] - 2026-11-23

### 变更（breaking）

逐条的改法与各语言写法见[从 0.8 升级到 0.9](https://x-iztro.x-haose.com/zh/docs/guide/about/upgrade-0-9)。

- **知识包构造按校验结果拒绝不合规范的包**：此前构造入口只查 `schema`，缺 `id`、未知语言、未知星耀标识（如 `ziweiMjj`）、
  字段类型不符、正文含 HTML、主题里写错的宫位等都能通过，进了包只会让条目静默失效，或在合并、出文本时才报第一处错且没有路径。
  现在 Rust `KnowledgePack::from_json` / `from_value`、Python `from_dict` / `from_json`、Go `ParseKnowledgePack`、
  JavaScript `fromObject` / `fromJSON`，以及 `merged` 收到的覆盖包对象，遇到 error 级问题即报 `invalid_argument`，
  信息逐条列出全部 error 级问题及其路径；Python、JavaScript 的异常与 Go 的 `*Error` 另带 `issues`。
  规范外的字段、空字符串、摘要写法等 warning 级问题不影响构造。SCHEMA.md §3 的实现行为随之改为「未知标识即拒绝」。
- **构造出的包是内核规范化后的同一份**：此前各语言各自解析，同一输入得到的包不同——Go 用 `encoding/json` 解原文，
  键不分大小写（`"Intro"` 被收下，同时出现 `intro` 与 `INTRO` 取后者），Python、JavaScript 持有传入对象的引用、
  保留规范外字段。现在构造入口一律持有内核解析再序列化的包（新增 bridge kind `parseKnowledgePack`）：键区分大小写、
  规范外字段丢弃。Python `from_dict`、JavaScript `fromObject` 改为持有副本；Python `from_json`、JavaScript `fromJSON`
  的 JSON 语法错误由 `JSONDecodeError` / `SyntaxError` 改为 `IztroError`（一条 `invalid_json` 问题）；Go 的
  `ParseKnowledgePack` 改为进 wasm 内核，另有 `ParseKnowledgePackContext`。

### 新增

- **知识包校验**：`KnowledgePack.check(raw, base?)` 逐条返回包里不合 `knowledge/SCHEMA.md` 的地方，每条带级别（error / warning）、
  JSON 路径、代码与参数，一次返回全部问题；输入不是对象、JSON 非法也作为问题返回，不抛异常。27 种代码各自注明出处
  （SCHEMA.md 的条款或默认包测试的断言），未知标识、字段、语言与取值给出编辑距离最近的建议（相邻换位计 1，上限为词长的三分之一）。
  给了底包时另报相对底包的提醒（空串覆盖底包的值、语言或 `extends` 与底包不符），摘要规则按合并后的条目判断。
  默认包口吻红线里指向页面上下文的那部分按 §5 对所有包生效（warning），其余是默认包自己的写法要求，不查。
  Rust `knowledge::check` / `check_json`，Python `KnowledgePack.check`，Go `CheckKnowledgePack`，JavaScript `KnowledgePack.check`，
  bridge kind `checkKnowledgePack`。HTML、图片、站外链接与页面引用只查 SCHEMA §5 列出的正文类字段（现明确为各条目的
  `intro`、`summary`、`conditions` 与两个组合映射的值；古籍引文 `quotes` 是逐字转录的纯文本，不在其列），行内代码里的内容不算
  （按 CommonMark：转义的反引号不作开头，不跨空行）。Python、JavaScript 的包对象含 NaN / ±Infinity 时报 `invalid_argument`，
  不悄悄写成 null。
  内嵌默认包校验结果为空。

### 修正

- **组合映射里的 null 值**：`"combinations": {"pojunMaj": null}`（`combinationSummaries` 同理）按 SCHEMA §4「写 null 等同省略」
  处理为没有这个键。此前 Rust 解析报错、Go 解成空串变成空文本、Python 与 JavaScript 构造成功但用到时才报错。

## [0.8.0] - 2026-11-23

### 新增

- **历法查询**：不排盘直接查历法，供日期选择器、日历视图这类按数值取用的场合。此前只能借排盘接口凑
  （排一张盘取 `rawDates` 做公农互换、取流月列表算月长、按农历年取运限拿年干支）。四个入口，
  入参都是整数年月日，与排盘同一份历法内核，结果与星盘的农历日期、年柱逐日一致：
  - `lunar_months(year)`：农历某年的全部月份，按先后排列，每项给月份、是否闰月、天数（29 或 30）
    与初一的公历日期；闰月紧跟在所闰的月之后。个别古代年份与 lunar-typescript 一致地少一个月
    （农历 237 年没有三月）。
  - `solar_to_lunar(year, month, day)`：公历换农历，返回与星盘 `raw_dates.lunar_date` 同一类型。
    公历 1 年年初的几十天属农历 0 年。
  - `lunar_to_solar(year, month, day, is_leap)`：农历换公历。该年没有这个闰月时报错，
    不像排盘入口（与 iztro、lunar-lite 的 `lunar2solar` 相同）那样退回同号的常规月——退回得到的日期
    差一个月却看不出错。排盘的 `fix_leap` 只影响星曜按哪个月安，不出现在日期换算里。
    农历 9999 年腊月初三起的 27 天落在公历 10000 年，返回的公历年为 10000。
  - `lunar_year_pillar(year)`：农历年的年干支。干支只由年号决定；正月初一与立春两种分界决定的是
    这一柱从哪天起算，排盘年柱在两种分界下都按此取。

  范围为公历与农历 1–9999 年，1582-10-15 以前按儒略历。给完整日期的换算在日期不存在时报
  `invalid_date`（同 `by_solar` / `by_lunar`），只给年份的查询在年份越界时报 `invalid_argument`
  （同按农历年取运限）。Python `x_iztro.calendar`（四个函数另从包根导出）、Go `LunarMonths` /
  `SolarToLunar` / `LunarToSolar` / `LunarYearPillar`、JavaScript / TypeScript `calendar` 命名空间，
  bridge kind `lunarMonths` / `solarToLunar` / `lunarToSolar` / `lunarYearPillar`。
  新金标 `calendar_api.csv` 由 lunar-typescript 生成，按农历 1–9999 年逐日核对月份表、年干支、
  农历换公历，以及公历换回原农历日期。文档站各语言新增「历法查询」页。

## [0.7.1] - 2026-11-23

### 修正

- **npm 包补上说明与许可证**：0.7.0 的 npm 包里没有 README 与 LICENSE，npm 页面显示「This package does not have a README」，
  MIT 要求随副本附带的许可证文本也缺失。现在包内附带两者，README 写安装、第一张盘、给模型读的文本与入口说明。
- **Go 模块补上 README**：`go/iztro` 是独立模块，模块目录里没有 README，pkg.go.dev 上没有说明。
- **npm 包的描述与关键词写明 TypeScript**：包本身用 TypeScript 写、自带完整类型声明，原来的描述与关键词里没有体现。

内核与各语言 API 没有变化。

## [0.7.0] - 2026-11-23

### 变更（breaking）

逐条的改法与各语言前后对照见[从 0.6 升级到 0.7](https://x-iztro.x-haose.com/zh/docs/guide/about/upgrade-0-7)。

- **Rust 公开类型的 serde 表示统一为语言无关标识**：此前约 45 个公开枚举的派生 serde 用 Rust 变体名
  （`"Male"`、`"Normal"`），与绑定线协议、DTO 用的语言无关标识（`male`、`normal`）是两套 JSON。
  现在这些枚举按标识序列化与反序列化。星盘、宫位、星耀、运限这些模型结构不再派生 serde，
  序列化一律走 DTO；`PatternConfig`、反推结果、知识包本身就是线协议，保留。
  `Language::as_code` 去掉，改用 `as_key`；`from_code` 仍收宽松写法。
- **配置 JSON 里的未知键由忽略改为报错**（Rust `Config` 的 serde 与各绑定的 config 入参同一实现）：
  此前拼错的键被静默忽略、按默认配置排盘，结果看上去正常却不是想要的配置。`Config` 的派生 serde
  此前还会丢掉自定义四化表与亮度表，现在往返不丢，报错统一以 `config:` 开头。
- **运限按宫查询改为可失败**：按农历年、年月取的运限只精确到流年、流月（见「新增」），
  在这样的运限上按更细的层级查宫位，此前会静默答出内核替调用方挑的代表日那一层。现在比查询精度细
  即报 `invalid_argument`，签名随之改变：
  - Rust `HoroscopeRef` 的 `palace`、`surround_palaces`、`has_horoscope_mutagen`、`has_horoscope_stars`、
    `has_one_of_horoscope_stars`、`not_have_horoscope_stars` 与 `to_text_with`、`patterns`、`patterns_with`
    返回 `Result`；`KnowledgePack::for_horoscope` / `for_horoscope_with` 改收 `HoroscopeRef`，
    按它的查询精度取材，不再有 `through` 参数。`HoroscopeData` 上带星盘参数的同名方法不变。
  - Go `Horoscope.Palace`、`SurroundPalaces` 返回 `(*T, error)`，`HasHoroscopeMutagen`、`HasHoroscopeStars`、
    `HasOneOfHoroscopeStars`、`NotHaveHoroscopeStars` 返回 `(bool, error)`。
  - Python 这几个方法抛 `IztroError`；层级标识拼错也报错，不再返回 `None`。
  按日期取的运限精确到流时，任何层级都不会报错。
- **日文、韩文、越南文译文对齐 iztro v2.6.3**，共 52 处：日文改用新字体（陷→陥、龍→竜、祿→禄、德→徳，
  交友宫僕役→奴僕）；韩文天德 복덕→천덕（原与福德宫同形）、铃星 령성→영성、截空 절중→절공、
  岁破 태파→세파等；越南文陷 Hạn→Hãm、钺 Nguyệt→Việt 等。按这几种语言的译名展示或反查的下游需同步。
- **星盘的 `lunarDate` 随排盘语言**（iztro v2.6.3 起）：简繁中文仍为「二〇〇〇年七月十七」，
  英日韩越为「农历」一词加数字的写法（`Lunar 2000-7-17`、`旧暦2000年7月17日`…），闰月带前缀。
  运限的 `lunarDate` 仍恒为中文写法。
- **运限的 `solarDate` 规范为不补零、不带时间的 `Y-M-D`**（如 `2025-1-5`）：日期入参可以补零、带时间部分，
  运限不回显原串；星盘的 `solarDate` 仍原样回显输入。

### 新增

- **JavaScript / TypeScript 绑定**（npm 包 `x-iztro`）：Rust 内核编译为 WebAssembly，在浏览器、Node.js 22+
  与 Web Worker 里运行，与 Python、Go 走同一个 bridge 分派，能力一一对应——星盘、宫位、星耀、三方四正、
  夹宫、运限（含按农历年、月取运限与查询精度）、全部 to_text 入口与分层开关、知识包读取 / 合并 /
  按盘取材、格局、八字与星盘特征反推、插件。模块划分与函数名随 iztro（`astro` / `util` / `star` / `data`），
  `toJSON()` 即 iztro 的 JSON；与 iztro 不同的是可选参数收在选项对象里，且启动时要先 `await init()`。
  - ESM 包，自带类型声明（TypeScript 5.0 起，消费方 `lib` 只需 `ES2022`）。`init()` 是唯一的异步调用，
    之后全部同步；内核按 `new URL('./x_iztro.wasm', import.meta.url)` 取，Vite、webpack、Turbopack 直接识别，
    也可显式传 URL、路径、字节、`Response` 或已编译的 `WebAssembly.Module`。
  - 纯数据入口 `x-iztro/types`：只有 DTO 类型与枚举常量，不装载内核，给只渲染排盘结果的组件用。
  - 内核调用中断（trap、越界等）时实例作废、抛 `internal`，再 `init()` 重建，不会带着损坏的内存继续算；
    数字入参拒绝 `NaN` 与 `±Infinity`。
  - 测试读与 Python、Go 相同的金标与文本快照逐值对照；在 React Router 8（预渲染）、Vite 8、Next.js 16
    （Turbopack 与 webpack）与 Tauri 2 打包后的应用里实测引用。文档站新增 JavaScript 栏目与打包器配置说明
    （Next.js webpack 的 `serverExternalPackages`、Vite Worker 的 `worker.format: 'es'`、CSP 的
    `'wasm-unsafe-eval'`），指南各页补 JavaScript 代码标签。
- **to_text 分层**：带释义的本命文本约 2 万字，调用方无论问什么都只能拿全部，而 AI 客户端对工具返回有篇幅上限。
  `TextOptions` 新增两个互相独立的开关，默认都是完整，输出与此前逐字节相同：
  - 事实详略 `TextDetail`（bridge `detail`）：`Overview` 时本命只写基本信息、十二宫总览表与格局，
    运限各层只写宫名对照、四化、流耀与格局，单宫、三方四正、夹宫只写三组星。
  - 释义深度 `KnowledgeDepth`（bridge `knowledgeDepth`）：`Summary` 取条目摘要。
  基准盘（2000-8-16 寅时女，zh-CN）本命：完整事实 3,389 字、总览 756 字、总览加摘要 1,054 字、
  完整加全文释义 20,552 字。
- **知识包摘要字段**：星耀、格局、宫位、四化条目新增 `summary`，星耀另有与 `combinations` 成对的
  `combinationSummaries`。正文显示宽度超过 240（中文 120 字）的条目必须带摘要，
  `KnowledgePack::missing_summaries` 按整包校验；覆盖包换了正文却没给摘要时去掉底包的旧摘要，
  免得摘要层说的是被换掉的内容。默认包 134 条长正文配了摘要。
- **按主题取材**：知识包新增 `topics.<slug>`（标题、连同三方四正取的宫位、只取本宫的宫位、主题说明），
  `topic_to_text` 按主题取出相关宫位成文。「婚恋看哪些宫」各家不一，所以主题定义在知识包里，
  门派可用覆盖包改写。默认包 13 个主题：总论、性格、财运、事业、婚恋、子女、父母、兄弟、交友、
  田宅、迁移、精神、健康。Rust / Python `topic_to_text`、Go `TopicToText`、bridge kind `topicToText`。
- **按农历年、农历月取运限**：`horoscope_of_year(year)` / `horoscope_of_month(year, month)`
  （Go `HoroscopeOfYear` / `HoroscopeOfMonth`，bridge 运限目标 `targetYear` 可配 `targetMonth`）。
  公历元旦到正月初一之间仍属上一个农历年，按 1 月 1 日取会得到上一年的流年。
  内核替调用方挑一个代表日（农历年取六月初一；农历月取初一，节气分界下取该节令月的交节次日），
  运限的查询精度 `precision()` 记为流年或流月，to_text、格局、按盘取材都只写到这一层，
  文本标题写年份与干支（如「# 运限 2027 (丁未)」）而不写代表日。改历年缺这个农历月时报 `invalid_argument`。
- **运限文本写到哪一层**：`TextOptions::horoscope_through(scope)`（Python `through=`、Go `TextOptions.Through`、
  bridge `through`），写到大限 / 流年 / 流月 / 流日 / 流时为止。只写到大限时不列风云际会——它从大限视角判定，
  却要读小限与流年。
- **流月一览与流年一览文本**：`monthly_list_to_text(year)` 某农历年每个流月一行（闰月拆前后两行），
  `yearly_overview_to_text(from, to)` 连续若干农历年每年一行（干支、虚岁、大限与小限宫、流年命宫与四化落宫）。
  回答「接下来几年」「哪个月适合」这类问题时一次拿到，不必逐年逐月取运限。带知识包时表后附四化星释义。
  bridge kind `monthlyListToText` / `yearlyOverviewToText`。
- **运限时辰可省略**（对齐 iztro v2.6.3）：省略时按目标日期串里的小时换算（23 点为晚子），没有时间即早子。
  Rust 的时辰参数改收 `impl Into<Option<u8>>`，传数字的调用不变；Python `horoscope()` 收 `str` / `datetime` / `date`；
  Go 新增 `HoroscopeAt(date)` 与 `HoroscopeAtTime(t)`；bridge `targetTimeIndex` 可省略。
- **日期写法对齐 iztro v2.6.3**：分隔符可用 `-`、`/`、`.`（同一串内一致），月日可补零，可带 ` HH:mm` 或
  ` HH:mm:ss`；公历、农历、运限共用同一套校验。
- **支持年份扩到 1-9999**（对齐 iztro v2.6.3）：此前为 1583-9999。1582-10-15 以前按儒略历，
  改历空缺的 1582-10-05 至 10-14 报 `invalid_date`。
- Rust 新增可选 feature `schemars`：带标识的枚举实现 `JsonSchema`（列出全部标识），供下游从内核类型派生入参 schema。

### 修正

- **历法换成自有内核，修正 lunar_rust 的移植错误**：以 iztro 所用的 lunar-typescript 1.8.6 为准，
  对公历 1-9999 年逐日比对，原依赖 lunar_rust 1.0.1 有两类错误落在此前支持的 1583-9999 年内：
  - 19 处农历月界错位、569 天农历日期不同，原修正层只盖住 1602 年的 29 天，其余 540 天排盘出错，
    如 1925-05-22 起一个月闰四月被标成四月；
  - 2012-2149 年 4,080 个节气时刻差 1-41 秒，其中 23 个跨过整点后 30 分，这些时辰
    `horoscope_divide=Exact` 的月柱判反。

  根因是公式移植错误（`t -= A + B` 译成了 `t = t - A + B` 等），上游仓库自 2023-12 无提交。
  新内核逐函数移植 lunar-typescript（MIT），金标逐日覆盖公历 1-9999 年的农历日期、年月日柱与星座，
  逐年覆盖月表与节气时刻。单盘耗时由约 15 µs 降到约 4.5 µs，wasm 由 1.80 MB 降到 1.65 MB。
- **Python `horoscope()` 只给日期不给时辰时取了此刻的时辰**：iztro 此时按早子时算。
  运限对象再发起计算（to_text、格局、按盘取材）时也改为转发调用方原始的目标日期与时辰，
  不再用已去掉时间部分的 `solarDate`，避免省略时辰时静默换成早子时。
- **流月列表对 1-9999 以外的年份先推算月表、再间接报 `invalid_date`**：现在入口即报 `invalid_argument`，
  与 iztro 2.6.3 一致。

### 其他

- 文档站迁到 x-iztro.x-haose.com，只讲把库嵌进自己的程序；面向 AI 客户端用户的站点在 ziwei.x-haose.com。
  新增「从 0.6 升级到 0.7」一页。
- 默认知识包内容修订：禄存条目末尾与禄存无关的辅星同宫提示移入术语「辅星同宫」（术语 49 → 50），
  三台、八座按各自视角表述同一段对星释义，龙德补述中州派的安放。

## [0.6.1] - 2026-09-08

### 修正

- 文档站页脚的分隔线在文档版式下横穿侧栏：侧栏是绝对定位、占满视口高度，而页脚横跨整行，
  滚到底部时页脚的上边框正落在侧栏中部，两者又都是透明背景，线便从侧栏中间透了出来。
  分隔线改为按 `--fd-sidebar-width` 起始，窄屏与首页版式下该变量为 0 或未定义，线回到最左。
- 文档站「童限」一节的表列到虚岁 8 并标「循环」：童限不存在循环——起运最晚的火六局
  虚岁 6 即入第一个大限，童限最多到虚岁 5，口诀里的「六官禄」永远取不到。表改到 5 为止。
  `CHILDHOOD_PALACES` 与取模照 iztro 原样保留，只在注释里点明末项不可达。

## [0.6.0] - 2026-09-08

### 变更（breaking）

对齐上游 iztro v2.6.1（金标基准从 v2.5.8 升级，全部 716,314 例重新生成）。以下四项会改变
既有盘的输出，升级后同一生辰的结果与 v0.5.0 不同：

- **太阳、太阴、七杀在酉宫的亮度修正**（iztro v2.6.1 修）：太阳「陷」→「平」、太阴「不」→「旺」、
  七杀「庙」→「旺」。连带改变日月类格局的判定——日月并明与丹墀桂墀命中增多、日月反背与
  日月藏辉减少（tier1 全量分布：102/56/60/24，原为 76/40/80/30）。iztro-docs《日月并明》
  《丹墀桂墀》两张示例盘从此在默认亮度表口径下即成格，与页面把它们列为该格局示例一致。
- **武曲的英文译名 `general` → `warrior`，岁破的英文译名 `wastrel` → `breaker`**
  （iztro v2.6.1 改，避免与将军 `general`、大耗 `wastrel` 重名）。按英文译名做展示或反查的
  下游需同步。语言无关标识 `wuquMaj` / `suipo` 不变。
- **来因宫的韩文译名 `라인` → `래인`**（iztro v2.6.1 修）。
- **`age_divide=Birthday` 的虚岁修正**（iztro v2.6.1 修）：此前判断「是否过了农历生日」时把
  出生年也纳入相等比较，导致出生当年之后的每一年里，落在生日同月且已过生日的日期少算一岁。
  现只比农历月日。

### 新增

- **夹宫**（iztro v2.6.0 新增）：`flanking_palaces(target)` 取某宫前后相邻的两宫，
  带 `have` / `not_have` / `have_one_of` / `have_mutagen` / `not_have_mutagen` 五个判定。
  十二宫首尾相连，故首宫的前一宫是末宫；星耀判定在**两宫合计**的集合上做，不要求同在一宫。
  Rust `flanking_palaces`、Python `flanking_palaces`、Go `FlankingPalaces`，
  bridge kind `flankingPalaces`。
- **三个运限列表**（iztro v2.6.0 新增）：
  - `decadal_list()` 按起运先后排列的十二个大限，每项带该限所在的本命宫名、起止虚岁与
    起止农历年份，以及以该宫为命宫推排的十二宫名、该限四化与大限流曜。
  - `yearly_list(target)` 某个大限内的十个流年；`target` 可写大限序号（0 起）或本命宫名。
  - `monthly_list(year, fix_leap)` 某农历年的流月：无闰月 12 项；有闰月且 `fix_leap` 为真时
    闰月拆成前后半月共 14 项，为假时闰月整月一项共 13 项。
  三个列表都经与逐层查询相同的 `horoscope()` 算出，故列表值与单层查询零分歧；
  列表用的时辰取自**时柱地支**而非出生入参——晚子时两者差 12，用入参会让晚子盘的列表
  与逐层查询对不上。bridge kind `decadalList` / `yearlyList` / `monthlyList`。
- **两个新的 to_text 入口**（x-iztro 扩展，to_text 家族从六个扩到八个）：
  - **夹宫文本** `flanking_palaces_to_text(_with)` / Python `FlankingPalaces.to_text` /
    Go `Astrolabe.FlankingPalacesToText`，与三方四正文本同构：前后两宫各一段，
    角色标题写「前宫」「后宫」。bridge kind `flankingPalacesToText`。
  - **大限一览** `decadal_list_to_text(_with)` / Python `Astrolabe.decadal_list_to_text` /
    Go `Astrolabe.DecadalListToText`：十二行一张表（序 / 本命宫 / 虚岁 / 年份 / 干支 / 四化），
    一眼看完一生的十二个十年。不展开每限的流年——十二限各十年会撑到一百二十行，
    某限的流年用 `yearly_list` 单取。带知识包时表后附各限四化星的释义。
    bridge kind `decadalListToText`。
  文本快照扩为 21 份（七类 × zh-CN/en-US + 七类带释义 × zh-CN），三侧读同一批。
- **星曜反查别名**（iztro v2.6.0 起）：`key_of` / `key_of_in` 先查 14 条带汉字的限定别名
  （`천상(天相)` / `Kiếp Sát(劫煞)` 之类），命中即返回且不受标识名限定影响，用于消歧韩文与
  越南语中若干完全同形的星曜译名。金标 `i18n_kot.json` 已纳入这 14 条。

### 修正

- **`horoscope_divide=Exact` 的月柱在节气当天静默算错**：构造日期对象用的是整点 0 分，
  而 lunar-lite 用的是整点后 30 分（`Solar.fromYmdHms(…, 30, 0)`）。按节气取月柱是时刻级
  比较，节气落在 `HH:00`~`HH:30` 的日子两边判到节气两侧——1984-2043 间有 196 个
  (日期, 时辰) 组合受影响。影响面不止展示字段：运限流月的干支与四化整组错，而流月宫名
  与流曜不变、肉眼看不出；反推走同一条路径，用标准排盘软件算出的八字在这些日子会漏解。
  年柱按立春是日期级比较、日柱分界在 23:00，两个分钟数同侧，均不受影响。
  金标新增按节气时刻取样的 `config_jieqi.json`（1,404 例）——此前 `horoscopeDivide=exact`
  只按固定日期抽样，结构上抓不到这类日子。0.5.0 及更早版本均受影响。
- **运限查询未遵守晚子时日期分界**（iztro v2.6.1 同修）：`day_divide=Current` 时目标时辰未做
  归一，晚子时（23:00 起）的日柱与随之推算的时柱进位到次日，流日与流时两层的干支和宫位
  因此落错。农历年月日不受影响（23:00 仍属同一公历日）。四处时辰归一（排盘上下文、运限、
  重排、反推剪枝）合并为 `builder::effective_time_index` 一份，不再各留副本。

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

[Unreleased]: https://github.com/x-haose/x-iztro/compare/v0.9.0...HEAD
[0.10.0]: https://github.com/x-haose/x-iztro/compare/v0.9.0...v0.10.0
[0.9.0]: https://github.com/x-haose/x-iztro/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/x-haose/x-iztro/compare/v0.7.1...v0.8.0
[0.7.1]: https://github.com/x-haose/x-iztro/compare/v0.7.0...v0.7.1
[0.7.0]: https://github.com/x-haose/x-iztro/compare/v0.6.1...v0.7.0
[0.6.1]: https://github.com/x-haose/x-iztro/compare/v0.6.0...v0.6.1
[0.6.0]: https://github.com/x-haose/x-iztro/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/x-haose/x-iztro/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/x-haose/x-iztro/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/x-haose/x-iztro/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/x-haose/x-iztro/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/x-haose/x-iztro/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/x-haose/x-iztro/releases/tag/v0.1.0

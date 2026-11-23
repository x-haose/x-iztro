/**
 * 内核收发的 JSON 形状与调用选项。
 *
 * `*Data` 是内核 DTO 的类型：camelCase 键、按排盘语言翻译的值，外加语言无关标识（`key` / `*Key`），
 * 与 iztro 的 `JSON.stringify` 输出逐键对应；各对象的 `toJSON()` 返回的就是它。
 */

import type {
  AgeDivide,
  Algorithm,
  AstroType,
  Brightness,
  BrightnessSource,
  DayDivide,
  EarthlyBranch,
  FiveElementsClass,
  Gender,
  HeavenlyStem,
  HoroscopeDivide,
  KnowledgeIssueCode,
  KnowledgeDepth,
  Language,
  MonthPart,
  Mutagen,
  PalaceName,
  Scope,
  StarType,
  TextDetail,
  TimeIndex,
  YearDivide,
} from './enums.js';
import type { KnowledgePack } from './knowledge.js';

/**
 * 排盘配置。六个开关决定算法分界与盘型，两张覆盖表替换内置的四化与亮度数据；
 * 每一项都可省略，省略取与 iztro 一致的默认值。
 */
export interface ChartConfig {
  /** 年分界点：normal=正月初一（默认）/ exact=立春 */
  yearDivide?: YearDivide | undefined;
  /** 运限分界点：normal=初一（默认）/ exact=节气 */
  horoscopeDivide?: HoroscopeDivide | undefined;
  /** 虚岁分界点：normal=跨年即加（默认）/ birthday=过生日才加 */
  ageDivide?: AgeDivide | undefined;
  /** 晚子时归属：forward=归次日（默认）/ current=归当天 */
  dayDivide?: DayDivide | undefined;
  /** 算法派别：default（默认）/ zhongzhou */
  algorithm?: Algorithm | undefined;
  /** 排盘视角：heaven=天盘（默认）/ earth=地盘 / human=人盘 */
  astroType?: AstroType | undefined;
  /**
   * 自定义四化表：天干标识 → 四颗星标识（禄、权、科、忌）。
   * 按天干整表替换默认值，未列出的天干仍用默认表。
   */
  mutagens?: Partial<Record<HeavenlyStem, readonly [string, string, string, string]>> | undefined;
  /**
   * 自定义亮度表：星耀标识 → 十二宫亮度标识（十二项，索引 0 为寅宫，空串表示该宫无亮度）。
   * 按星耀整表替换默认值，未列出的星耀仍用默认表。
   */
  brightness?: Partial<Record<string, readonly (Brightness | '')[]>> | undefined;
}

/** 星盘上记录的排盘配置：六个开关的实际取值（自定义四化与亮度表不回显） */
export interface ChartConfigData {
  /** 年分界点 */
  yearDivide: YearDivide;
  /** 运限分界点 */
  horoscopeDivide: HoroscopeDivide;
  /** 虚岁分界点 */
  ageDivide: AgeDivide;
  /** 晚子时归属 */
  dayDivide: DayDivide;
  /** 算法派别 */
  algorithm: Algorithm;
  /** 排盘视角 */
  astroType: AstroType;
}

/** 格局判定口径。多口径格局一律以 `PatternHit.variant` 报出，这里只放会改变事实判定的开关 */
export interface PatternConfig {
  /** 日月明暗依据：table=按 iztro 亮度表（默认）/ positional=按传统位置 */
  brightnessSource?: BrightnessSource | undefined;
  /** 空宫是否借对宫主星参与判定，默认 true */
  borrow?: boolean | undefined;
  /** 运限视角下流曜（运禄/流禄等）是否等同对应本命辅星参与判定，默认 true */
  flowStars?: boolean | undefined;
}

/**
 * 释义材料：`true` 取排盘语言的内嵌包（目前只有 zh-CN），`KnowledgePack` 用该包（自定义或合并后的包）；
 * `false` / `null` / 省略只输出盘面事实。
 */
export type Knowledge = boolean | KnowledgePack | null;

/** 释义类文本（大限、流年、流月一览）的选项 */
export interface KnowledgeOptions {
  /** 释义材料；给出时表后附四化星的释义 */
  knowledge?: Knowledge | undefined;
  /** 释义深度：summary=摘要层（条目摘要，或本身够短的正文）/ full=全文（默认）；summary 须同时给 knowledge */
  knowledgeDepth?: KnowledgeDepth | undefined;
}

/** 盘面语义化文本的选项 */
export interface TextOptions extends KnowledgeOptions {
  /** 事实详略：overview=总览 / full=全部事实（默认） */
  detail?: TextDetail | undefined;
  /** 格局判定口径，与 `patterns(config)` 同形态；省略取默认口径 */
  config?: PatternConfig | undefined;
}

/** 运限语义化文本的选项 */
export interface HoroscopeTextOptions extends TextOptions {
  /**
   * 写到哪一层为止，更细的层级不写：decadal 只写大限，yearly 写大限、小限、流年，
   * monthly / daily / hourly 依次各多一层。省略写到查询精度（`Horoscope.precision()`）。
   * 只写到大限时大限段不列风云际会（它要读小限与流年才判得出）。
   */
  through?: Exclude<Scope, 'origin'> | undefined;
}

/** 星耀 */
export interface StarData {
  /** 语言无关星耀标识 */
  key: string;
  /** 星耀名称（按排盘语言翻译） */
  name: string;
  /** 星耀类型 */
  type: StarType;
  /** 作用范围 */
  scope: Scope;
  /** 亮度显示文本（如「庙」）：主星与辅星恒有该键，无亮度为空串；杂耀与流耀没有该键 */
  brightness?: string;
  /** 语言无关亮度标识，无亮度时没有该键 */
  brightnessKey?: Brightness;
  /** 四化显示文本（如「禄」）：可被四化的星恒有该键，无四化为空串；其余星没有该键 */
  mutagen?: string;
  /** 语言无关四化标识，无四化时没有该键 */
  mutagenKey?: Mutagen;
}

/** 大限 */
export interface DecadalData {
  /** 起止虚岁 [起, 止] */
  range: [number, number];
  /** 大限天干（翻译文本） */
  heavenlyStem: string;
  /** 大限天干标识 */
  heavenlyStemKey: HeavenlyStem;
  /** 大限地支（翻译文本） */
  earthlyBranch: string;
  /** 大限地支标识 */
  earthlyBranchKey: EarthlyBranch;
}

/** 宫位 */
export interface PalaceData {
  /** 宫位索引 0-11，0 为寅宫 */
  index: number;
  /** 宫名（按排盘语言翻译） */
  name: string;
  /** 宫名标识 */
  nameKey: PalaceName;
  /** 是否身宫 */
  isBodyPalace: boolean;
  /** 是否来因宫 */
  isOriginalPalace: boolean;
  /** 宫干（翻译文本） */
  heavenlyStem: string;
  /** 宫干标识 */
  heavenlyStemKey: HeavenlyStem;
  /** 宫支（翻译文本） */
  earthlyBranch: string;
  /** 宫支标识 */
  earthlyBranchKey: EarthlyBranch;
  /** 主星 */
  majorStars: StarData[];
  /** 辅星 */
  minorStars: StarData[];
  /** 杂耀 */
  adjectiveStars: StarData[];
  /** 长生十二神（翻译文本） */
  changsheng12: string;
  /** 长生十二神标识 */
  changsheng12Key: string;
  /** 博士十二神（翻译文本） */
  boshi12: string;
  /** 博士十二神标识 */
  boshi12Key: string;
  /** 将前十二神（翻译文本） */
  jiangqian12: string;
  /** 将前十二神标识 */
  jiangqian12Key: string;
  /** 岁前十二神（翻译文本） */
  suiqian12: string;
  /** 岁前十二神标识 */
  suiqian12Key: string;
  /** 大限 */
  decadal: DecadalData;
  /** 小限经过的虚岁 */
  ages: number[];
  /** 本宫天干化出的四颗星标识，顺序为禄、权、科、忌；随自定义四化表变化 */
  mutagenStarKeys: [string, string, string, string];
}

/** 数字化农历生日 */
export interface RawLunarDate {
  /** 农历年 */
  lunarYear: number;
  /** 农历月 1-12，闰月与否见 isLeap */
  lunarMonth: number;
  /** 农历日 1-30 */
  lunarDay: number;
  /** 是否闰月 */
  isLeap: boolean;
}

/** 数字化公历日期；1582-10-15 以前为儒略历日期 */
export interface RawSolarDate {
  /** 公历年；农历 9999 年腊月初三起的 27 天落在公历 10000 年，`lunarToSolar` 对它们返回 10000；各月初一都在 9999 年内 */
  solarYear: number;
  /** 公历月 1-12 */
  solarMonth: number;
  /** 公历日 1-31 */
  solarDay: number;
}

/** 农历年里的一个月 */
export interface LunarMonthInfo {
  /** 月份 1-12；闰月取所闰的月份，如闰四月为 4 */
  month: number;
  /** 是否闰月 */
  isLeap: boolean;
  /** 天数：29（小月）或 30（大月） */
  dayCount: number;
  /** 初一对应的公历日期 */
  firstDay: RawSolarDate;
}

/** 四柱干支的 DTO：每柱 [天干, 地支]，译文与标识两组。星盘上对应的对象是 `RawChineseDate` */
export interface RawChineseDateData {
  /** 年柱（翻译文本） */
  yearly: [string, string];
  /** 月柱（翻译文本） */
  monthly: [string, string];
  /** 日柱（翻译文本） */
  daily: [string, string];
  /** 时柱（翻译文本） */
  hourly: [string, string];
  /** 年柱标识 */
  yearlyKeys: [HeavenlyStem, EarthlyBranch];
  /** 月柱标识 */
  monthlyKeys: [HeavenlyStem, EarthlyBranch];
  /** 日柱标识 */
  dailyKeys: [HeavenlyStem, EarthlyBranch];
  /** 时柱标识 */
  hourlyKeys: [HeavenlyStem, EarthlyBranch];
}

/** 结构化出生日期的 DTO */
export interface RawDatesData {
  /** 数字化农历生日 */
  lunarDate: RawLunarDate;
  /** 四柱干支 */
  chineseDate: RawChineseDateData;
}

/** 星盘 */
export interface AstrolabeData {
  /** 性别（按排盘语言翻译） */
  gender: string;
  /** 阳历日期 `Y-M-D` */
  solarDate: string;
  /** 农历日期 */
  lunarDate: string;
  /** 干支纪年日期 */
  chineseDate: string;
  /** 结构化的农历生日与四柱 */
  rawDates: RawDatesData;
  /** 时辰 */
  time: string;
  /** 时辰对应的时间段 */
  timeRange: string;
  /** 星座 */
  sign: string;
  /** 星座标识（aries … pisces） */
  signKey: string;
  /** 生肖 */
  zodiac: string;
  /** 生肖标识（rat … pig） */
  zodiacKey: string;
  /** 命宫地支（翻译文本） */
  earthlyBranchOfSoulPalace: string;
  /** 命宫地支标识 */
  earthlyBranchOfSoulPalaceKey: EarthlyBranch;
  /** 身宫地支（翻译文本） */
  earthlyBranchOfBodyPalace: string;
  /** 身宫地支标识 */
  earthlyBranchOfBodyPalaceKey: EarthlyBranch;
  /** 命主星（翻译文本） */
  soul: string;
  /** 命主星标识 */
  soulKey: string;
  /** 身主星（翻译文本） */
  body: string;
  /** 身主星标识 */
  bodyKey: string;
  /** 五行局（翻译文本） */
  fiveElementsClass: string;
  /** 五行局标识 */
  fiveElementsClassKey: FiveElementsClass;
  /** 十二宫，按宫位索引排列 */
  palaces: PalaceData[];
  /** 性别标识 */
  genderKey: Gender;
  /** 出生时辰索引 */
  timeIndex: TimeIndex;
  /** 是否修正闰月 */
  fixLeap: boolean;
  /** 排盘语言 */
  language: Language;
  /** 排盘配置的六个开关 */
  config: ChartConfigData;
}

/** 运限的一层（大限、小限、流年、流月、流日、流时） */
export interface HoroscopeItemData {
  /** 该层命宫所在的宫位索引 0-11 */
  index: number;
  /** 层级显示名（大限/童限/小限/流年/流月/流日/流时，按排盘语言翻译） */
  name: string;
  /** 层级标识；大限层未起运时为 childhood（童限），与 decadal 是不同的解盘语义 */
  nameKey: string;
  /** 该层天干（翻译文本） */
  heavenlyStem: string;
  /** 该层天干标识 */
  heavenlyStemKey: HeavenlyStem;
  /** 该层地支（翻译文本） */
  earthlyBranch: string;
  /** 该层地支标识 */
  earthlyBranchKey: EarthlyBranch;
  /** 该层十二宫名（翻译文本，按宫位索引排列） */
  palaceNames: string[];
  /** 该层十二宫名标识 */
  palaceNameKeys: PalaceName[];
  /** 四化星名 [禄, 权, 科, 忌]（翻译文本） */
  mutagen: string[];
  /** 被化的四颗星标识 [禄, 权, 科, 忌] */
  mutagenStarKeys: string[];
  /** 流耀：十二宫各一组；该层没有流耀时不出现 */
  stars?: StarData[][];
}

/** 小限 */
export interface AgeItemData extends HoroscopeItemData {
  /** 虚岁 */
  nominalAge: number;
}

/** 流年十二神，按目标年支排布，索引即宫位索引 */
export interface YearlyDecStar {
  /** 将前十二神（翻译文本） */
  jiangqian12: string[];
  /** 将前十二神标识 */
  jiangqian12Keys: string[];
  /** 岁前十二神（翻译文本） */
  suiqian12: string[];
  /** 岁前十二神标识 */
  suiqian12Keys: string[];
}

/** 流年 */
export interface HoroscopeYearlyData extends HoroscopeItemData {
  /** 流年十二神 */
  yearlyDecStar: YearlyDecStar;
}

/** 运限 */
export interface HoroscopeData {
  /** 目标农历日期 */
  lunarDate: string;
  /** 目标阳历日期 `Y-M-D` */
  solarDate: string;
  /** 大限（未起运时为童限） */
  decadal: HoroscopeItemData;
  /** 小限 */
  age: AgeItemData;
  /** 流年 */
  yearly: HoroscopeYearlyData;
  /** 流月 */
  monthly: HoroscopeItemData;
  /** 流日 */
  daily: HoroscopeItemData;
  /** 流时 */
  hourly: HoroscopeItemData;
}

/** 大限列表项：一整个大限 */
export interface DecadalListItemData extends HoroscopeItemData {
  /** 该大限所在的本命宫名（翻译文本） */
  palaceName: string;
  /** 本命宫名标识 */
  palaceNameKey: PalaceName;
  /** 起止虚岁 [起, 止]，含两端 */
  ageRange: [number, number];
  /** 起止农历年份 [起, 止]，含两端 */
  yearRange: [number, number];
}

/** 流年列表项：某个大限内的一个流年 */
export interface YearlyListItemData extends HoroscopeItemData {
  /** 该流年的虚岁 */
  age: number;
  /** 农历年份 */
  year: number;
}

/** 流月列表项：某个农历年的一个流月段 */
export interface MonthlyListItemData extends HoroscopeItemData {
  /** 该流月的虚岁 */
  age: number;
  /** 农历年份 */
  year: number;
  /** 农历月份（正月为 1）；闰月与同月号的常规月共用，以 isLeapMonth 区分 */
  month: number;
  /** 本段是否属于闰月 */
  isLeapMonth: boolean;
  /** 分段：整月、闰月前半、闰月后半 */
  part: MonthPart;
  /** 本段覆盖的农历日区间 [起, 止]，含两端 */
  dayRange: [number, number];
}

/** 参与成格的一颗星 */
export interface PatternStarData {
  /** 星耀标识 */
  key: string;
  /** 星耀名称（翻译文本） */
  name: string;
  /** 落宫索引 0-11 */
  palaceIndex: number;
  /** 亮度显示文本，无亮度时不出现 */
  brightness?: string;
  /** 亮度标识，无亮度时不出现 */
  brightnessKey?: Brightness;
  /** 判定视角下的四化显示文本（本命为生年四化，运限为该层四化），无四化时不出现 */
  mutagen?: string;
  /** 四化标识，无四化时不出现 */
  mutagenKey?: Mutagen;
}

/** 一次格局命中 */
export interface PatternHitData {
  /** 格局标识 */
  key: string;
  /** 格局名称（翻译文本） */
  name: string;
  /** 判定视角：本命为 origin，运限为该层 */
  scope: Scope;
  /** 成格所在宫位索引 0-11 */
  palaceIndex: number;
  /** 成格宫位在该视角下的宫名（翻译文本） */
  palaceName: string;
  /** 宫名标识 */
  palaceNameKey: PalaceName;
  /** 多口径格局命中的口径，单口径时不出现 */
  variant?: string;
  /** 「破格 / 加杀平常」条件是否触发：成格照报，仅作标记 */
  broken: boolean;
  /** 参与成格的星与落宫 */
  stars: PatternStarData[];
}

/** 知识包校验的一条问题（`KnowledgePack.check`） */
export interface KnowledgeIssue {
  /**
   * `error`：违反规范的「必须 / 不得」或内核用到该处时会失败，构造知识包时拒绝；
   * `warning`：「应 / 不应」、被丢弃的字段、启发式匹配与相对底包的提醒，只报告
   */
  level: 'error' | 'warning';
  /** 问题所在的 JSON 路径：对象键为字符串、数组下标为数字；空数组指包本身 */
  path: (string | number)[];
  /** 问题种类，出处与 `params` 的键见文档站「知识包 · 校验」 */
  code: KnowledgeIssueCode;
  /** 补充数据，如 `{ key: 'ziweiMjj', suggestion: 'ziweiMaj' }` */
  params: Record<string, unknown>;
  /** 带路径的英文说明，供日志；界面应按 `code` 与 `params` 组织文案 */
  message: string;
}

/**
 * 星盘。
 *
 * 星盘是排盘结果的入口：基本信息、十二宫，以及从本盘再发起的重排、运限、格局与语义化文本。
 * 再发起的计算都是无状态的：把本盘的排盘上下文原样送回内核重算，不在 JS 与内核之间搬整盘。
 */

import { PalaceName, type EarthlyBranch, type FiveElementsClass, type Gender, type HeavenlyStem, type Language, type TimeIndex } from './enums.js';
import { birthParams, chartContexts, chartQuery, palaceCharts, starCharts, starPalaces, type ChartContext, type HoroscopeTarget } from './internal.js';
import { DecadalListItem, Horoscope, MonthlyListItem, YearlyListItem, type PatternTextOptions } from './horoscope.js';
import { Palace } from './palace.js';
import { PatternHit } from './pattern.js';
import { RawChineseDate, type RawDates } from './raw-dates.js';
import { invoke } from './runtime.js';
import type { Star } from './star-object.js';
import { FlankingPalaces, SurroundedPalaces } from './surpalaces.js';
import type {
  AstrolabeData,
  ChartConfig,
  ChartConfigData,
  DecadalListItemData,
  HoroscopeData,
  KnowledgeOptions,
  MonthlyListItemData,
  PalaceData,
  PatternConfig,
  PatternHitData,
  TextOptions,
  TopicPalacesOptions,
  TopicSelection,
  YearlyListItemData,
} from './types.js';

/** 星盘 */
export class Astrolabe {
  /** 性别（按排盘语言翻译） */
  readonly gender: string;
  /** 性别标识 */
  readonly genderKey: Gender;
  /** 阳历日期 `Y-M-D` */
  readonly solarDate: string;
  /** 农历日期 */
  readonly lunarDate: string;
  /** 干支纪年日期 */
  readonly chineseDate: string;
  /** 结构化的农历生日与四柱干支 */
  readonly rawDates: RawDates;
  /** 时辰 */
  readonly time: string;
  /** 时辰对应的时间段 */
  readonly timeRange: string;
  /** 星座 */
  readonly sign: string;
  /** 星座标识（aries … pisces） */
  readonly signKey: string;
  /** 生肖 */
  readonly zodiac: string;
  /** 生肖标识（rat … pig） */
  readonly zodiacKey: string;
  /** 命宫地支（翻译文本） */
  readonly earthlyBranchOfSoulPalace: string;
  /** 命宫地支标识 */
  readonly earthlyBranchOfSoulPalaceKey: EarthlyBranch;
  /** 身宫地支（翻译文本） */
  readonly earthlyBranchOfBodyPalace: string;
  /** 身宫地支标识 */
  readonly earthlyBranchOfBodyPalaceKey: EarthlyBranch;
  /** 命主星（翻译文本） */
  readonly soul: string;
  /** 命主星标识 */
  readonly soulKey: string;
  /** 身主星（翻译文本） */
  readonly body: string;
  /** 身主星标识 */
  readonly bodyKey: string;
  /** 五行局（翻译文本） */
  readonly fiveElementsClass: string;
  /** 五行局标识 */
  readonly fiveElementsClassKey: FiveElementsClass;
  /** 出生时辰索引 */
  readonly timeIndex: TimeIndex;
  /** 是否修正闰月 */
  readonly fixLeap: boolean;
  /** 排盘语言 */
  readonly language: Language;
  /** 排盘配置的六个开关（自定义四化与亮度表不回显，但再发起计算时仍沿用） */
  readonly config: Readonly<ChartConfigData>;
  /** 十二宫，按宫位索引排列 */
  readonly palaces: readonly Palace[];
  readonly #data: AstrolabeData;

  /**
   * 由内核 DTO 与排盘上下文构造。通常经 `astro.bySolar` / `astro.byLunar` 取得。
   *
   * @param data 内核返回的星盘 DTO
   * @param context 再发起计算所需的上下文：用户传入的配置原件与重排起点
   */
  constructor(data: AstrolabeData, context: ChartContext) {
    this.#data = data;
    this.gender = data.gender;
    this.genderKey = data.genderKey;
    this.solarDate = data.solarDate;
    this.lunarDate = data.lunarDate;
    this.chineseDate = data.chineseDate;
    this.rawDates = {
      lunarDate: { ...data.rawDates.lunarDate },
      chineseDate: new RawChineseDate(data.rawDates.chineseDate),
    };
    this.time = data.time;
    this.timeRange = data.timeRange;
    this.sign = data.sign;
    this.signKey = data.signKey;
    this.zodiac = data.zodiac;
    this.zodiacKey = data.zodiacKey;
    this.earthlyBranchOfSoulPalace = data.earthlyBranchOfSoulPalace;
    this.earthlyBranchOfSoulPalaceKey = data.earthlyBranchOfSoulPalaceKey;
    this.earthlyBranchOfBodyPalace = data.earthlyBranchOfBodyPalace;
    this.earthlyBranchOfBodyPalaceKey = data.earthlyBranchOfBodyPalaceKey;
    this.soul = data.soul;
    this.soulKey = data.soulKey;
    this.body = data.body;
    this.bodyKey = data.bodyKey;
    this.fiveElementsClass = data.fiveElementsClass;
    this.fiveElementsClassKey = data.fiveElementsClassKey;
    this.timeIndex = data.timeIndex;
    this.fixLeap = data.fixLeap;
    this.language = data.language;
    this.config = { ...data.config };
    chartContexts.set(this, context);
    this.palaces = data.palaces.map((p) => this.#adopt(p));
  }

  /** 星盘的 DTO（与 iztro 的 `JSON.stringify(astrolabe)` 逐键逐值对应）的深拷贝 */
  toJSON(): AstrolabeData {
    return structuredClone(this.#data);
  }

  /**
   * 星盘的语义化文本：面向语言模型与人的完整描述（Markdown 子集，按排盘语言输出）。
   * 与 `toJSON()`（机器结构）、翻译字段（展示文本）同源，是同一张盘的第三种投影；重排盘按重排后的布局描述。
   *
   * 给出 knowledge 时每宫事实之后紧跟该宫星耀释义、格局之后紧跟格局释义，文末另起四化释义一节。
   *
   * @throws IztroError `knowledge: true` 而排盘语言没有内嵌包（目前只有 zh-CN）；
   *   `knowledgeDepth: 'summary'` 而未给 knowledge，或所给的包缺摘要
   */
  toText(options: TextOptions = {}): string {
    return chartQuery(this, 'astrolabeToText', {
      knowledge: options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /** 不带释义的语义化文本，同 `toText()` */
  toString(): string {
    return this.toText();
  }

  /**
   * 按主题取材的语义化文本：按知识包里该主题的定义选宫（`surround` 的各宫连同三方四正各成一组，
   * `palaces` 的各宫只取本宫，同一宫只出一次），每宫一段，文末附落在选中宫内的格局与四化释义。
   * 主题定义与释义都取自 knowledge，省略时取排盘语言的内嵌包。
   *
   * @param topic 主题 slug（知识包 `topics` 段的键，默认包见 `KnowledgePack.topics()`）
   * @throws IztroError 包里没有该主题、主题里有不认识的宫位标识、knowledge 为 false 或排盘语言没有内嵌包，
   *   或 summary 而包缺摘要
   */
  topicToText(topic: string, options: TextOptions = {}): string {
    return chartQuery(this, 'topicToText', {
      topic,
      knowledge: options.knowledge === undefined ? true : options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /**
   * 主题在本命盘上选中的宫：各组位置、每宫的角色与宫干飞化，以及落在这些宫里的生年四化。
   * 与 `topicToText` 同一套选宫，`palaces` 的顺序即文本的成段顺序。
   *
   * @param topic 主题 slug
   * @throws IztroError 包里没有该主题、主题里有不认识的宫位标识、knowledge 为 false 或排盘语言没有内嵌包
   */
  topicPalaces(topic: string, options: TopicPalacesOptions = {}): TopicSelection {
    return chartQuery(this, 'topicPalaces', {
      topic,
      knowledge: options.knowledge === undefined ? true : options.knowledge,
    }) as TopicSelection;
  }

  // ------ 宫位查询 ------

  /**
   * 按索引、宫名标识或当前语言的宫名取宫位；`PalaceName.BODY` / `PalaceName.ORIGINAL` 取身宫 / 来因宫。
   * 定位不到为 undefined。
   */
  palace(target: number | PalaceName | string): Palace | undefined {
    if (typeof target === 'number') return this.palaces[target];
    if (target === PalaceName.BODY) return this.palaces.find((p) => p.isBodyPalace);
    if (target === PalaceName.ORIGINAL) return this.palaces.find((p) => p.isOriginalPalace);
    return this.palaces.find((p) => p.nameKey === target || p.name === target);
  }

  /** 指定宫位的三方四正：本宫、对宫、财帛位、官禄位；定位不到为 undefined */
  surroundedPalaces(target: number | PalaceName | string): SurroundedPalaces | undefined {
    const p = this.palace(target);
    if (p === undefined) return undefined;
    const at = (offset: number): Palace => this.palaces[(p.index + offset) % 12] as Palace;
    return new SurroundedPalaces(at(0), at(6), at(8), at(4));
  }

  /** 指定宫位的夹宫：前后相邻的两宫；定位不到为 undefined */
  flankingPalaces(target: number | PalaceName | string): FlankingPalaces | undefined {
    const p = this.palace(target);
    if (p === undefined) return undefined;
    const data = chartQuery(this, 'flankingPalaces', { palaceIndex: p.index }) as {
      previous: PalaceData;
      next: PalaceData;
    };
    return new FlankingPalaces(this.#adopt(data.previous), this.#adopt(data.next));
  }

  /** 指定宫位的三方四正是否包含 **全部** 指定星耀 */
  isSurrounded(target: number | PalaceName | string, stars: readonly string[]): boolean {
    return this.surroundedPalaces(target)?.have(stars) ?? false;
  }

  /** 指定宫位的三方四正是否包含指定星耀中的 **任意一颗** */
  isSurroundedOneOf(target: number | PalaceName | string, stars: readonly string[]): boolean {
    return this.surroundedPalaces(target)?.haveOneOf(stars) ?? false;
  }

  /** 指定宫位的三方四正是否 **一颗都不** 包含指定星耀 */
  notSurrounded(target: number | PalaceName | string, stars: readonly string[]): boolean {
    return this.surroundedPalaces(target)?.notHave(stars) ?? false;
  }

  // ------ 星耀查询 ------

  /** 按星耀标识（或当前语言星名）找本命盘上的星耀 */
  star(star: string): Star | undefined {
    return this.starInPalace(star)?.[0];
  }

  /** 按星耀标识（或当前语言星名）找星耀及其所在宫位 */
  starInPalace(star: string): [Star, Palace] | undefined {
    for (const p of this.palaces) {
      for (const s of [...p.majorStars, ...p.minorStars, ...p.adjectiveStars]) {
        if (s.key === star || s.name === star) return [s, p];
      }
    }
    return undefined;
  }

  /**
   * 以指定干支为命宫重排本盘，返回新盘；本盘不变。
   *
   * 传入的干支决定五行局，进而决定紫微天府落点、十二宫名、长生十二神与大限小限；
   * 辅星、杂耀（天伤天使天才除外）、博士十二神、岁前将前十二神沿用原盘。
   * 常规的天盘、地盘、人盘用 `ChartConfig.astroType` 指定即可，本方法用于从任意干支起盘。
   *
   * @throws IztroError 干支标识非法
   */
  rearranged(fromStem: HeavenlyStem, fromBranch: EarthlyBranch): Astrolabe {
    const context = chartContexts.get(this) as ChartContext;
    const data = invoke('by_solar', {
      ...birthParams(this, context),
      fromStem,
      fromBranch,
    }) as AstrolabeData;
    return new Astrolabe(data, { config: context.config, from: [fromStem, fromBranch] });
  }

  /**
   * 以本命盘为起点计算目标日期的运限，结果持有本盘。
   *
   * @param targetDate 目标阳历日期。字符串写作 "2024-1-1"（分隔符可为 `-` `/` `.`，可带 " 13:20" 或 " 13:20:05"），
   *   或 `Date`（取本地时区的年月日时分秒）；不传取此刻
   * @param targetTimeIndex 目标时辰索引 0-12；不传时按目标日期的时间换算，不带时间即早子时
   * @throws IztroError 目标日期或时辰索引非法
   */
  horoscope(targetDate?: string | Date, targetTimeIndex?: TimeIndex): Horoscope {
    const date = typeof targetDate === 'string' ? targetDate : dateParam(targetDate ?? new Date());
    return this.#horoscope(targetTimeIndex === undefined ? { date } : { date, timeIndex: targetTimeIndex });
  }

  /**
   * 指定农历年的运限，结果持有本盘：目标日期取该年六月初一，与 `yearlyList` 同一约定。
   *
   * 只问某一年时用它：公历年初到正月初一之前仍属上一个农历年，拿公历 1 月 1 日去算会得到上一年的流年。
   * 查询精度为流年：流月、流日、流时也在结果里，是六月初一那一天的，文本与按盘取材只取到流年，
   * 宫位查询、文本与格局的层级参数比流年细时报错。
   *
   * @param year 农历年份 1-9999
   * @throws IztroError 年份不在 1-9999，或该年六月初一落在支持范围外
   */
  horoscopeOfYear(year: number): Horoscope {
    return this.#horoscope({ year });
  }

  /**
   * 指定农历年某月的运限，结果持有本盘：目标日期取该月（非闰）初一。
   *
   * 闰月不单独成月号：默认分界下闰月前半月的流月随本月、后半月随下月。查询精度为流月。
   * 排盘配置的 `horoscopeDivide` 为 exact 时，目标日期改取第 month 个节令月（寅月为 1）的交节次日。
   *
   * @param year 农历年份 1-9999
   * @param month 农历月号 1-12
   * @throws IztroError 年份或月号越界、初一分界下该农历年没有这个月，或目标日期落在支持范围外
   */
  horoscopeOfMonth(year: number, month: number): Horoscope {
    return this.#horoscope({ year, month });
  }

  /**
   * 本盘的全部大限（12 项），按起运先后排列。每项带该限所在的本命宫名、起止虚岁与起止农历年份，
   * 以及以该宫为命宫推排的十二宫名、该限四化与大限流曜。
   */
  decadalList(): DecadalListItem[] {
    return (chartQuery(this, 'decadalList') as DecadalListItemData[]).map((d) => new DecadalListItem(d));
  }

  /**
   * 大限一览的语义化文本：十二个大限一张表；给出 knowledge 时表后附各限四化星的释义。
   *
   * @throws IztroError knowledge / knowledgeDepth 的报错同 `toText`
   */
  decadalListToText(options: KnowledgeOptions = {}): string {
    return chartQuery(this, 'decadalListToText', {
      knowledge: options.knowledge,
      knowledgeDepth: options.knowledgeDepth,
    }) as string;
  }

  /**
   * 指定大限内的全部流年（10 项），按虚岁先后排列；每个流年取该农历年六月初一为目标日期。
   *
   * @param decadal 大限序号（0 为第一个大限）或该限所在的本命宫名标识。定位是硬要求，缺省即报错
   * @throws IztroError 未给定位、序号越界、宫名定位不到，或某一年的目标日期落在支持范围外
   */
  yearlyList(decadal?: number | PalaceName): YearlyListItem[] {
    const params = typeof decadal === 'number' ? { decadalOrdinal: decadal } : { palaceKey: decadal };
    return (chartQuery(this, 'yearlyList', params) as YearlyListItemData[]).map((d) => new YearlyListItem(d));
  }

  /**
   * 流年一览的语义化文本：农历 fromYear 年到 toYear 年（含两端）每年一行，写年份、干支、正月初一的阳历日期、
   * 虚岁、大限与小限所在的本命宫、流年命宫所在的本命宫与流年四化落宫。各年与 `horoscopeOfYear` 同一口径。
   *
   * @throws IztroError fromYear 晚于 toYear、某一年不在 1-9999 或六月初一落在支持范围外；knowledge 的报错同 `toText`
   */
  yearlyOverviewToText(fromYear: number, toYear: number, options: KnowledgeOptions = {}): string {
    return chartQuery(this, 'yearlyOverviewToText', {
      fromYear,
      toYear,
      knowledge: options.knowledge,
      knowledgeDepth: options.knowledgeDepth,
    }) as string;
  }

  /**
   * 指定农历年的全部流月，按月份先后排列；每段目标日期取该段首日（后半段取十六）。
   *
   * @param fixLeap 闰月是否拆段，与排盘的同名开关无关：无闰月 12 项；有闰月时拆为前后半月两项（共 14 项），
   *   不拆则闰月整月一项（共 13 项）。默认 true
   * @throws IztroError 年份不在 1-9999，或某一段的目标日期落在支持范围外
   */
  monthlyList(year: number, fixLeap = true): MonthlyListItem[] {
    return (chartQuery(this, 'monthlyList', { year, monthFixLeap: fixLeap }) as MonthlyListItemData[]).map(
      (d) => new MonthlyListItem(d),
    );
  }

  /**
   * 流月一览的语义化文本：指定农历年每个流月一行，写农历月、阳历起止日、流月命宫所在的本命宫、干支与四化落宫；
   * 闰月按前后半月拆成两行。
   *
   * @throws IztroError 年份不在 1-9999 或某一段首日落在支持范围外；knowledge 的报错同 `toText`
   */
  monthlyListToText(year: number, options: KnowledgeOptions = {}): string {
    return chartQuery(this, 'monthlyListToText', {
      year,
      knowledge: options.knowledge,
      knowledgeDepth: options.knowledgeDepth,
    }) as string;
  }

  /**
   * 本命盘的全部格局命中，按《格局》页条目顺序；重排盘按重排后的布局判定。
   *
   * @param config 判定口径；省略取默认（亮度按 iztro 表、借宫、流曜参与）
   */
  patterns(config?: PatternConfig): PatternHit[] {
    return (chartQuery(this, 'patterns', { patternConfig: config }) as PatternHitData[]).map((d) => new PatternHit(d));
  }

  /**
   * 本命盘格局命中的语义化文本；给出 knowledge 时格局列表之后紧跟各格局的释义。
   *
   * @throws IztroError knowledge / knowledgeDepth 的报错同 `toText`
   */
  patternsToText(options: PatternTextOptions = {}): string {
    return chartQuery(this, 'patternsToText', {
      patternConfig: options.config,
      knowledge: options.knowledge,
      knowledgeDepth: options.knowledgeDepth,
    }) as string;
  }

  #horoscope(target: HoroscopeTarget): Horoscope {
    const context = chartContexts.get(this) as ChartContext;
    const data = invoke('horoscope', {
      ...birthParams(this, context),
      targetDate: target.date,
      targetTimeIndex: target.timeIndex,
      targetYear: target.year,
      targetMonth: target.month,
    }) as HoroscopeData;
    return new Horoscope(data, this, target);
  }

  /** 由宫位 DTO 构造宫位，并回填宫位与星耀对本盘的反向引用 */
  #adopt(data: PalaceData): Palace {
    const palace = new Palace(data);
    palaceCharts.set(palace, this);
    for (const s of [...palace.majorStars, ...palace.minorStars, ...palace.adjectiveStars]) {
      starPalaces.set(s, palace);
      starCharts.set(s, this);
    }
    return palace;
  }
}

/** `Date` 落成内核的日期串：本地时区的年月日时分秒 */
function dateParam(value: Date): string {
  return (
    `${value.getFullYear()}-${value.getMonth() + 1}-${value.getDate()} ` +
    `${value.getHours()}:${value.getMinutes()}:${value.getSeconds()}`
  );
}

/**
 * 由排盘入口构造星盘，记下用户传入的配置。记的是排盘当时的副本：调用方事后改动自己的配置对象，
 * 不会改变这张盘再发起的计算。
 */
export function newChart(data: AstrolabeData, config: ChartConfig | undefined): Astrolabe {
  return new Astrolabe(data, { config: config === undefined ? undefined : structuredClone(config), from: undefined });
}

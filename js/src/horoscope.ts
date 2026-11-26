/**
 * 运限。
 *
 * 大限、小限、流年、流月、流日、流时六个层级各有自己的宫名排布与四化，流耀按层级挂在十二宫上。
 * 运限由星盘发起，持有发起它的那张盘；查询方法可免传星盘。
 */

import { MUTAGEN_ORDER, Scope, type EarthlyBranch, type HeavenlyStem, type MonthPart, type Mutagen, type PalaceName } from './enums.js';
import { invalidArgument } from './errors.js';
import { horoscopeContext, horoscopeContexts, horoscopeQuery, type HoroscopeTarget } from './internal.js';
import type { Astrolabe } from './astrolabe.js';
import { copyExtraMutagens, type Palace } from './palace.js';
import { PatternHit } from './pattern.js';
import { Star, starIdentifiers } from './star-object.js';
import type { SurroundedPalaces } from './surpalaces.js';
import type {
  AgeItemData,
  DecadalListItemData,
  HoroscopeData,
  HoroscopeItemData,
  HoroscopeTextOptions,
  HoroscopeYearlyData,
  KnowledgeOptions,
  MonthlyListItemData,
  PatternConfig,
  PatternHitData,
  TextOptions,
  TopicPalacesOptions,
  TopicSelection,
  YearlyDecStar,
  YearlyListItemData,
} from './types.js';

/** 层级深浅：本命 0、大限 1 … 流时 5，越大越细；与内核 `Scope::depth` 一致 */
const SCOPE_DEPTH: readonly Scope[] = [
  Scope.ORIGIN,
  Scope.DECADAL,
  Scope.YEARLY,
  Scope.MONTHLY,
  Scope.DAILY,
  Scope.HOURLY,
];

/** 格局文本的选项 */
export interface PatternTextOptions extends KnowledgeOptions {
  /** 判定口径；省略取默认 */
  config?: PatternConfig | undefined;
}

/** 运限的一层（大限、小限、流年、流月、流日、流时）与各运限列表项的共同部分 */
export class HoroscopeItem {
  /** 该层命宫所在的宫位索引 0-11 */
  readonly index: number;
  /** 层级显示名（大限/童限/小限/流年/流月/流日/流时，按排盘语言翻译） */
  readonly name: string;
  /**
   * 层级标识。大限层未起运时为 childhood（童限）而非 decadal：两者是不同的解盘语义，
   * 程序判断一律用本字段而非译文。
   */
  readonly nameKey: string;
  /** 该层天干（翻译文本） */
  readonly heavenlyStem: string;
  /** 该层天干标识 */
  readonly heavenlyStemKey: HeavenlyStem;
  /** 该层地支（翻译文本） */
  readonly earthlyBranch: string;
  /** 该层地支标识 */
  readonly earthlyBranchKey: EarthlyBranch;
  /** 该层十二宫名（翻译文本，按宫位索引排列） */
  readonly palaceNames: readonly string[];
  /** 该层十二宫名标识 */
  readonly palaceNameKeys: readonly PalaceName[];
  /** 四化星名 [禄, 权, 科, 忌]（翻译文本） */
  readonly mutagen: readonly string[];
  /**
   * 被化的四颗星标识 [禄, 权, 科, 忌]，与 `Palace.mutagenStarKeys` 同义；
   * 单数的 `mutagenKey` 才是四化类型 `Mutagen` 的标识。
   */
  readonly mutagenStarKeys: readonly string[];
  /** 该层天干在四槽之外化出的附加星：四化标识 → 星耀标识列表（`ChartConfig.extraMutagens`）；没有附加星时为 undefined */
  readonly extraMutagenStarKeys: Readonly<Partial<Record<Mutagen, readonly string[]>>> | undefined;
  /** 流耀：十二宫各一组；该层没有流耀时为 undefined */
  readonly stars: readonly (readonly Star[])[] | undefined;

  constructor(data: HoroscopeItemData) {
    this.index = data.index;
    this.name = data.name;
    this.nameKey = data.nameKey;
    this.heavenlyStem = data.heavenlyStem;
    this.heavenlyStemKey = data.heavenlyStemKey;
    this.earthlyBranch = data.earthlyBranch;
    this.earthlyBranchKey = data.earthlyBranchKey;
    this.palaceNames = [...data.palaceNames];
    this.palaceNameKeys = [...data.palaceNameKeys];
    this.mutagen = [...data.mutagen];
    this.mutagenStarKeys = [...data.mutagenStarKeys];
    this.extraMutagenStarKeys = copyExtraMutagens(data.extraMutagenStarKeys);
    this.stars = data.stars?.map((group) => group.map((s) => new Star(s)));
  }

  /** 宫名标识（或当前语言宫名）在该层所在的宫位索引；找不到为 undefined */
  palaceIndexByName(name: PalaceName | string): number | undefined {
    for (let i = 0; i < this.palaceNameKeys.length; i++) {
      if (name === this.palaceNameKeys[i] || name === this.palaceNames[i]) return i;
    }
    return undefined;
  }
}

/** 小限 */
export class AgeItem extends HoroscopeItem {
  /** 虚岁 */
  readonly nominalAge: number;

  constructor(data: AgeItemData) {
    super(data);
    this.nominalAge = data.nominalAge;
  }
}

/** 流年（含流年十二神） */
export class HoroscopeYearly extends HoroscopeItem {
  /** 流年十二神：岁前、将前十二神按目标年支排布 */
  readonly yearlyDecStar: Readonly<YearlyDecStar>;

  constructor(data: HoroscopeYearlyData) {
    super(data);
    this.yearlyDecStar = structuredClone(data.yearlyDecStar);
  }
}

/** 大限列表项：一整个大限，含起止虚岁与起止农历年份 */
export class DecadalListItem extends HoroscopeItem {
  /** 该大限所在的本命宫名（按排盘语言翻译） */
  readonly palaceName: string;
  /** 本命宫名标识 */
  readonly palaceNameKey: PalaceName;
  /** 起止虚岁 [起, 止]，含两端 */
  readonly ageRange: readonly [number, number];
  /** 起止农历年份 [起, 止]，含两端 */
  readonly yearRange: readonly [number, number];

  constructor(data: DecadalListItemData) {
    super(data);
    this.palaceName = data.palaceName;
    this.palaceNameKey = data.palaceNameKey;
    this.ageRange = [data.ageRange[0], data.ageRange[1]];
    this.yearRange = [data.yearRange[0], data.yearRange[1]];
  }
}

/** 流年列表项：某个大限内的一个流年 */
export class YearlyListItem extends HoroscopeItem {
  /** 该流年的虚岁 */
  readonly age: number;
  /** 农历年份 */
  readonly year: number;

  constructor(data: YearlyListItemData) {
    super(data);
    this.age = data.age;
    this.year = data.year;
  }
}

/** 流月列表项：某个农历年的一个流月段 */
export class MonthlyListItem extends HoroscopeItem {
  /** 该流月的虚岁 */
  readonly age: number;
  /** 农历年份 */
  readonly year: number;
  /** 农历月份（正月为 1）；闰月与同月号的常规月共用本字段，以 isLeapMonth 区分 */
  readonly month: number;
  /** 本段是否属于闰月 */
  readonly isLeapMonth: boolean;
  /** 分段：整月、闰月前半、闰月后半 */
  readonly part: MonthPart;
  /** 本段覆盖的农历日区间 [起, 止]，含两端 */
  readonly dayRange: readonly [number, number];

  constructor(data: MonthlyListItemData) {
    super(data);
    this.age = data.age;
    this.year = data.year;
    this.month = data.month;
    this.isLeapMonth = data.isLeapMonth;
    this.part = data.part;
    this.dayRange = [data.dayRange[0], data.dayRange[1]];
  }
}

/** 运限 */
export class Horoscope {
  /** 目标农历日期 */
  readonly lunarDate: string;
  /** 目标阳历日期 `Y-M-D` */
  readonly solarDate: string;
  /** 大限（未起运时为童限） */
  readonly decadal: HoroscopeItem;
  /** 小限 */
  readonly age: AgeItem;
  /** 流年 */
  readonly yearly: HoroscopeYearly;
  /** 流月 */
  readonly monthly: HoroscopeItem;
  /** 流日 */
  readonly daily: HoroscopeItem;
  /** 流时 */
  readonly hourly: HoroscopeItem;
  readonly #data: HoroscopeData;

  /** 由内核 DTO、发起它的星盘与发起目标构造；通常经 `Astrolabe.horoscope*` 取得 */
  constructor(data: HoroscopeData, chart: Astrolabe, target: HoroscopeTarget) {
    this.#data = data;
    horoscopeContexts.set(this, { chart, target });
    this.lunarDate = data.lunarDate;
    this.solarDate = data.solarDate;
    this.decadal = new HoroscopeItem(data.decadal);
    this.age = new AgeItem(data.age);
    this.yearly = new HoroscopeYearly(data.yearly);
    this.monthly = new HoroscopeItem(data.monthly);
    this.daily = new HoroscopeItem(data.daily);
    this.hourly = new HoroscopeItem(data.hourly);
  }

  /** 运限的 DTO（与 iztro 的 JSON 同形）的深拷贝 */
  toJSON(): HoroscopeData {
    return structuredClone(this.#data);
  }

  /**
   * 运限的语义化文本：面向语言模型与人的完整描述（Markdown 子集，按排盘语言输出）。
   *
   * @throws IztroError `knowledge: true` 而排盘语言没有内嵌包；`knowledgeDepth: 'summary'` 而未给 knowledge
   *   或包缺摘要；`through` 为 origin、不是运限层级，或比查询精度细
   */
  toText(options: HoroscopeTextOptions = {}): string {
    return horoscopeQuery(this, 'horoscopeToText', {
      knowledge: options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      through: options.through,
      patternConfig: options.config,
    }) as string;
  }

  /** 不带释义的语义化文本，同 `toText()` */
  toString(): string {
    return this.toText();
  }

  /** 发起这次运限查询的星盘 */
  astrolabe(): Astrolabe {
    return horoscopeContext(this).chart;
  }

  /**
   * 查询精度：目标实际指定到的最细层级。`horoscopeOfYear` 为 yearly，`horoscopeOfMonth` 为 monthly，
   * 按日期发起的为 hourly。
   *
   * 比它细的层级是内核替调用方挑的代表日的，不是查询的答案：`toText` 默认只写到这一层，
   * 宫位查询、文本与格局的层级参数比它细时报 `IztroError`。
   */
  precision(): Scope {
    const { target } = horoscopeContext(this);
    if (target.year === undefined) return Scope.HOURLY;
    if (target.month === undefined) return Scope.YEARLY;
    return Scope.MONTHLY;
  }

  /** 指定层级的运限项；origin 或未知层级为 undefined（小限不按层级取，见 `age`） */
  scopeItem(scope: Scope): HoroscopeItem | undefined {
    switch (scope) {
      case Scope.DECADAL:
        return this.decadal;
      case Scope.YEARLY:
        return this.yearly;
      case Scope.MONTHLY:
        return this.monthly;
      case Scope.DAILY:
        return this.daily;
      case Scope.HOURLY:
        return this.hourly;
      default:
        return undefined;
    }
  }

  /** 小限所在的宫位；不传星盘取发起本次查询的那张盘 */
  agePalace(astrolabe?: Astrolabe): Palace | undefined {
    return (astrolabe ?? this.astrolabe()).palaces[this.age.index];
  }

  /**
   * 指定运限层级下的宫位。
   *
   * @param name 宫名标识或当前语言的宫名
   * @param scope 运限层级；origin 按本命宫名取
   * @param astrolabe 星盘；不传取发起本次查询的那张盘
   * @throws IztroError scope 不是层级标识，或比查询精度细
   */
  palace(name: PalaceName | string, scope: Scope, astrolabe?: Astrolabe): Palace | undefined {
    this.#checkScope(scope);
    const chart = astrolabe ?? this.astrolabe();
    if (scope === Scope.ORIGIN) return chart.palace(name);
    const idx = this.scopeItem(scope)?.palaceIndexByName(name);
    return idx === undefined ? undefined : chart.palaces[idx];
  }

  /**
   * 指定运限层级下某宫的三方四正。
   *
   * @throws IztroError 同 `palace`
   */
  surroundPalaces(name: PalaceName | string, scope: Scope, astrolabe?: Astrolabe): SurroundedPalaces | undefined {
    const chart = astrolabe ?? this.astrolabe();
    const p = this.palace(name, scope, chart);
    return p === undefined ? undefined : chart.surroundedPalaces(p.index);
  }

  /**
   * 指定运限层级下某宫是否有该层这一类四化的星（四槽之星或附加星，看主星与辅星）；origin 恒为 false。
   *
   * @throws IztroError 同 `palace`
   */
  hasHoroscopeMutagen(name: PalaceName | string, scope: Scope, mutagen: Mutagen, astrolabe?: Astrolabe): boolean {
    this.#checkScope(scope);
    if (scope === Scope.ORIGIN) return false;
    const item = this.scopeItem(scope);
    const p = this.palace(name, scope, astrolabe);
    if (item === undefined || p === undefined) return false;
    const starKey = item.mutagenStarKeys[MUTAGEN_ORDER.indexOf(mutagen)];
    if (starKey === undefined) return false;
    const starKeys = [starKey, ...(item.extraMutagenStarKeys?.[mutagen] ?? [])];
    return [...p.majorStars, ...p.minorStars].some((s) => starKeys.includes(s.key));
  }

  /**
   * 指定运限宫位是否包含指定的 **全部** 流耀（星耀标识或翻译名）；流耀取大限与流年两层。
   *
   * @throws IztroError 同 `palace`
   */
  hasHoroscopeStars(name: PalaceName | string, scope: Scope, stars: readonly string[], astrolabe?: Astrolabe): boolean {
    const p = this.palace(name, scope, astrolabe);
    if (p === undefined) return false;
    const ids = this.#flowStarIdentifiers(p.index);
    return stars.every((s) => ids.has(s));
  }

  /**
   * 指定运限宫位是否包含指定流耀中的 **任意一颗**。
   *
   * @throws IztroError 同 `palace`
   */
  hasOneOfHoroscopeStars(name: PalaceName | string, scope: Scope, stars: readonly string[], astrolabe?: Astrolabe): boolean {
    const p = this.palace(name, scope, astrolabe);
    if (p === undefined) return false;
    const ids = this.#flowStarIdentifiers(p.index);
    return stars.some((s) => ids.has(s));
  }

  /**
   * 指定运限宫位是否 **一颗都不** 包含指定的流耀。
   *
   * @throws IztroError 同 `palace`
   */
  notHaveHoroscopeStars(name: PalaceName | string, scope: Scope, stars: readonly string[], astrolabe?: Astrolabe): boolean {
    const p = this.palace(name, scope, astrolabe);
    if (p === undefined) return false;
    const ids = this.#flowStarIdentifiers(p.index);
    return stars.every((s) => !ids.has(s));
  }

  /**
   * 某运限层视角的格局命中：以该层命宫为命宫、合并该层流曜与四化后重跑全部规则，
   * 另加两条行运格（禄衰马困、风云际会）；origin 等同本命盘。
   *
   * @throws IztroError scope 不是层级标识，或比查询精度细
   */
  patterns(scope: Scope, config?: PatternConfig, astrolabe?: Astrolabe): PatternHit[] {
    const data = horoscopeQuery(this, 'horoscopePatterns', { scope, patternConfig: config }, astrolabe) as PatternHitData[];
    return data.map((d) => new PatternHit(d));
  }

  /**
   * 某运限层视角格局命中的语义化文本；宫名按该层重排后的宫名书写。
   *
   * @throws IztroError 同 `patterns`，以及 knowledge / knowledgeDepth 的报错（见 `toText`）
   */
  patternsToText(scope: Scope, options: PatternTextOptions = {}, astrolabe?: Astrolabe): string {
    return horoscopeQuery(
      this,
      'horoscopePatternsToText',
      {
        scope,
        patternConfig: options.config,
        knowledge: options.knowledge,
        knowledgeDepth: options.knowledgeDepth,
      },
      astrolabe,
    ) as string;
  }

  /**
   * 主题在 `scope` 层选中的宫：宫名按该层定位、三方四正以该层的本宫起算（`bodyPalace` / `originalPalace`
   * 落在本命那一宫），每宫的角色与宫干飞化，落在这些宫里的生年四化，以及大限至该层各层落在这些宫里的四化。
   * origin 等同 `Astrolabe.topicPalaces`。
   *
   * @param topic 主题 slug
   * @throws IztroError scope 不是层级标识或比查询精度细，或同 `Astrolabe.topicPalaces`
   */
  topicPalaces(topic: string, scope: Scope, options: TopicPalacesOptions = {}): TopicSelection {
    return horoscopeQuery(this, 'horoscopeTopicPalaces', {
      topic,
      scope,
      knowledge: options.knowledge === undefined ? true : options.knowledge,
    }) as TopicSelection;
  }

  /**
   * 主题在 `scope` 层的语义化文本：选宫同 `topicPalaces`。标题写运限时间与命盘，标题下逐层写大限至该层的
   * 命宫位置与四化；每宫段落写该层宫名并补本命宫名，宫内星曜照本命，另写各层落在本宫的四化与流耀；
   * 格局取该层视角在选中宫内的命中。origin 与 `Astrolabe.topicToText` 逐字相同。
   * 主题定义与释义都取自 knowledge，省略时取排盘语言的内嵌包。
   *
   * @throws IztroError scope 不是层级标识或比查询精度细、包里没有该主题、主题里有不认识的宫位标识、
   *   knowledge 为 false 或排盘语言没有内嵌包，或 summary 而包缺摘要
   */
  topicToText(topic: string, scope: Scope, options: TextOptions = {}): string {
    return horoscopeQuery(this, 'horoscopeTopicToText', {
      topic,
      scope,
      knowledge: options.knowledge === undefined ? true : options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /** 宫位查询的 scope 须是层级标识，且不比查询精度细；报错正文与内核同类校验相同 */
  #checkScope(scope: Scope): void {
    const depth = SCOPE_DEPTH.indexOf(scope);
    if (depth < 0) {
      const keys = SCOPE_DEPTH.map((s) => `'${s}'`);
      throw invalidArgument(`invalid scope '${scope}': expected ${keys.slice(0, -1).join(', ')} or ${keys.at(-1)}`);
    }
    const precision = this.precision();
    if (depth > SCOPE_DEPTH.indexOf(precision)) {
      throw invalidArgument(
        `invalid scope '${scope}': the horoscope target only specifies layers down to '${precision}'`,
      );
    }
  }

  /** 大限与流年两层在指定宫位的流耀标识 */
  #flowStarIdentifiers(palaceIndex: number): Set<string> {
    return starIdentifiers([
      ...(this.decadal.stars?.[palaceIndex] ?? []),
      ...(this.yearly.stars?.[palaceIndex] ?? []),
    ]);
  }
}

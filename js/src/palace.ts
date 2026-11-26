/**
 * 宫位。
 *
 * 宫位是星盘的基本单元：星耀落在宫位上，四化飞星在宫位之间发生，大限与小限也按宫位排布。
 * 判断方法一律基于语言无关标识，同时兼容当前语言的星名与宫名。
 */

import { MUTAGEN_ORDER, type EarthlyBranch, type HeavenlyStem, type Mutagen, type PalaceName } from './enums.js';
import { chartQuery, palaceCharts } from './internal.js';
import type { Astrolabe } from './astrolabe.js';
import { Star, starIdentifiers } from './star-object.js';
import type { SurroundedPalaces } from './surpalaces.js';
import type { DecadalData, PalaceData, TextOptions } from './types.js';

/** 宫位的大限 */
export type Decadal = Readonly<DecadalData>;

/** 宫位定位：宫位对象、宫位索引 0-11，或宫名标识 / 当前语言的宫名 */
export type PalaceTarget = Palace | number | PalaceName | string;

/** 宫位 */
export class Palace {
  /** 宫位索引 0-11，0 为寅宫 */
  readonly index: number;
  /** 宫名（按排盘语言翻译） */
  readonly name: string;
  /** 宫名标识 */
  readonly nameKey: PalaceName;
  /** 是否身宫 */
  readonly isBodyPalace: boolean;
  /** 是否来因宫 */
  readonly isOriginalPalace: boolean;
  /** 辅来因宫（辛年丑宫、壬年子宫）为 true，其余宫为 undefined；只在 `ChartConfig.originalPalace` 为 withSecondary 时出现 */
  readonly isSecondaryOriginalPalace: true | undefined;
  /** 宫干（翻译文本） */
  readonly heavenlyStem: string;
  /** 宫干标识 */
  readonly heavenlyStemKey: HeavenlyStem;
  /** 宫支（翻译文本） */
  readonly earthlyBranch: string;
  /** 宫支标识 */
  readonly earthlyBranchKey: EarthlyBranch;
  /** 主星 */
  readonly majorStars: readonly Star[];
  /** 辅星 */
  readonly minorStars: readonly Star[];
  /** 杂耀 */
  readonly adjectiveStars: readonly Star[];
  /** 长生十二神（翻译文本） */
  readonly changsheng12: string;
  /** 长生十二神标识（`Changsheng12` 取值） */
  readonly changsheng12Key: string;
  /** 博士十二神（翻译文本） */
  readonly boshi12: string;
  /** 博士十二神标识（`Boshi12` 取值） */
  readonly boshi12Key: string;
  /** 将前十二神（翻译文本）；本命不安（`ChartConfig.yearly12Placement` 为 yearlyOnly）时为空串 */
  readonly jiangqian12: string;
  /** 将前十二神标识（`Jiangqian12` 取值）；本命不安时为空串 */
  readonly jiangqian12Key: string;
  /** 岁前十二神（翻译文本）；本命不安时为空串 */
  readonly suiqian12: string;
  /** 岁前十二神标识（`Suiqian12` 取值）；本命不安时为空串 */
  readonly suiqian12Key: string;
  /** 大限 */
  readonly decadal: Decadal;
  /** 小限经过的虚岁 */
  readonly ages: readonly number[];
  /**
   * 本宫天干化出的四颗星标识，顺序为禄、权、科、忌。
   * 取自排盘时生效的四化表，自定义四化表（`ChartConfig.mutagens`）会反映在这里，飞星判断随之改变。
   */
  readonly mutagenStarKeys: readonly [string, string, string, string];
  /**
   * 本宫天干在四槽之外化出的附加星：四化标识 → 星耀标识列表（`ChartConfig.extraMutagens`）；
   * 没有附加星时为 undefined。飞星判断把附加星与同类的四槽之星一并计入。
   */
  readonly extraMutagenStarKeys: Readonly<Partial<Record<Mutagen, readonly string[]>>> | undefined;

  /** 由内核 DTO 构造；挂到星盘上的宫位由星盘构造时回填所属星盘 */
  constructor(data: PalaceData) {
    this.index = data.index;
    this.name = data.name;
    this.nameKey = data.nameKey;
    this.isBodyPalace = data.isBodyPalace;
    this.isOriginalPalace = data.isOriginalPalace;
    this.isSecondaryOriginalPalace = data.isSecondaryOriginalPalace;
    this.heavenlyStem = data.heavenlyStem;
    this.heavenlyStemKey = data.heavenlyStemKey;
    this.earthlyBranch = data.earthlyBranch;
    this.earthlyBranchKey = data.earthlyBranchKey;
    this.majorStars = data.majorStars.map((s) => new Star(s));
    this.minorStars = data.minorStars.map((s) => new Star(s));
    this.adjectiveStars = data.adjectiveStars.map((s) => new Star(s));
    this.changsheng12 = data.changsheng12;
    this.changsheng12Key = data.changsheng12Key;
    this.boshi12 = data.boshi12;
    this.boshi12Key = data.boshi12Key;
    this.jiangqian12 = data.jiangqian12;
    this.jiangqian12Key = data.jiangqian12Key;
    this.suiqian12 = data.suiqian12;
    this.suiqian12Key = data.suiqian12Key;
    this.decadal = { ...data.decadal, range: [data.decadal.range[0], data.decadal.range[1]] };
    this.ages = [...data.ages];
    this.mutagenStarKeys = [...data.mutagenStarKeys];
    this.extraMutagenStarKeys = copyExtraMutagens(data.extraMutagenStarKeys);
  }

  // ------ 星耀判断 ------

  /** 是否包含指定的 **全部** 星耀（星耀标识或当前语言的星名） */
  has(stars: readonly string[]): boolean {
    const ids = this.#identifiers();
    return stars.every((s) => ids.has(s));
  }

  /** 是否 **一颗都不** 包含指定的星耀 */
  notHave(stars: readonly string[]): boolean {
    const ids = this.#identifiers();
    return stars.every((s) => !ids.has(s));
  }

  /** 是否包含指定星耀中的 **至少一颗** */
  hasOneOf(stars: readonly string[]): boolean {
    const ids = this.#identifiers();
    return stars.some((s) => ids.has(s));
  }

  /** 是否有指定四化（只看主星与辅星） */
  hasMutagen(mutagen: Mutagen): boolean {
    return [...this.majorStars, ...this.minorStars].some((s) => s.mutagenKey === mutagen);
  }

  /** 是否没有指定四化 */
  notHaveMutagen(mutagen: Mutagen): boolean {
    return !this.hasMutagen(mutagen);
  }

  /**
   * 是否空宫（无十四主星）。
   *
   * @param excludeStars 视为「不空」的星耀；宫内出现其中任一颗即不作空宫论
   */
  isEmpty(excludeStars?: readonly string[]): boolean {
    if (this.majorStars.length > 0) return false;
    return !(excludeStars !== undefined && this.hasOneOf(excludeStars));
  }

  /** 宫位所属星盘；脱离星盘单独构造的宫位为 undefined */
  astrolabe(): Astrolabe | undefined {
    return palaceCharts.get(this);
  }

  /**
   * 本宫的语义化文本：面向语言模型与人的单宫完整描述（Markdown 子集，按排盘语言输出）。
   *
   * @throws Error 宫位脱离星盘单独构造，没有排盘上下文可转发
   * @throws IztroError `knowledge: true` 而排盘语言没有内嵌包（目前只有 zh-CN）；
   *   `knowledgeDepth: 'summary'` 而未给 knowledge，或所给的包缺摘要
   */
  toText(options: TextOptions = {}): string {
    return chartQuery(this.#chart('toText'), 'palaceToText', {
      palaceIndex: this.index,
      knowledge: options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /** 本宫的对宫（索引 +6）；命宫对迁移、财帛对福德。脱离星盘时为 undefined */
  oppositePalace(): Palace | undefined {
    return palaceCharts.get(this)?.palaces[(this.index + 6) % 12];
  }

  /** 本宫的三方四正；脱离星盘时为 undefined */
  surroundedPalaces(): SurroundedPalaces | undefined {
    return palaceCharts.get(this)?.surroundedPalaces(this.index);
  }

  // ------ 四化飞星 ------

  /** 本宫天干在指定四化位上的星耀标识，顺序同传入顺序，每一类先四槽之星、后附加星 */
  mutagenStars(mutagens: Mutagen | readonly Mutagen[]): string[] {
    return this.#mutagenGroups(mutagens).flat();
  }

  /**
   * 本宫天干的指定四化是否 **全部** 飞入目标宫位；一类四化化出不止一颗星时，其中任一颗在目标宫即算这一类飞入。
   * 四化列表为空时为 false
   */
  fliesTo(target: PalaceTarget, mutagens: Mutagen | readonly Mutagen[]): boolean {
    const to = this.#resolve(target);
    if (to === undefined) return false;
    const groups = this.#mutagenGroups(mutagens);
    return groups.length > 0 && groups.every((g) => to.hasOneOf(g));
  }

  /** 本宫天干的指定四化星是否有 **任意一颗** 落在目标宫位；四化列表为空时为 true */
  fliesOneOfTo(target: PalaceTarget, mutagens: Mutagen | readonly Mutagen[]): boolean {
    const to = this.#resolve(target);
    if (to === undefined) return false;
    const stars = this.mutagenStars(mutagens);
    return stars.length === 0 || to.hasOneOf(stars);
  }

  /** 本宫天干的指定四化星是否 **一颗都不** 落在目标宫位；四化列表为空时为 true */
  notFlyTo(target: PalaceTarget, mutagens: Mutagen | readonly Mutagen[]): boolean {
    const to = this.#resolve(target);
    if (to === undefined) return false;
    const stars = this.mutagenStars(mutagens);
    return stars.length === 0 || to.notHave(stars);
  }

  /** 本宫天干的指定四化是否 **全部** 落回本宫（自化）；一类四化有任一颗星在本宫即算 */
  selfMutaged(mutagens: Mutagen | readonly Mutagen[]): boolean {
    return this.#mutagenGroups(mutagens).every((g) => this.hasOneOf(g));
  }

  /** 本宫是否有指定四化中任一种自化；不传或传空表示检查全部四化 */
  selfMutagedOneOf(mutagens?: Mutagen | readonly Mutagen[]): boolean {
    return this.hasOneOf(this.mutagenStars(orAllMutagens(mutagens)));
  }

  /** 本宫是否没有指定四化中的任何一种自化；不传或传空表示检查全部四化 */
  notSelfMutaged(mutagens?: Mutagen | readonly Mutagen[]): boolean {
    return this.notHave(this.mutagenStars(orAllMutagens(mutagens)));
  }

  /**
   * 本宫天干四化四槽之星（不含附加星）分别所在的宫位：[禄, 权, 科, 忌]，找不到的为 undefined。
   *
   * @param allPalaces 检索范围；不传取所属星盘的十二宫，脱离星盘且不传时返回空数组
   */
  mutagedPlaces(allPalaces?: readonly Palace[]): (Palace | undefined)[] {
    const palaces = allPalaces ?? palaceCharts.get(this)?.palaces;
    if (palaces === undefined) return [];
    return this.mutagenStarKeys.map((key) => palaces.find((p) => p.has([key])));
  }

  /** 按传入顺序，每一类四化化出的星（四槽之星与附加星）各成一组；不认识的四化跳过 */
  #mutagenGroups(mutagens: Mutagen | readonly Mutagen[]): string[][] {
    const list = typeof mutagens === 'string' ? [mutagens] : mutagens;
    const out: string[][] = [];
    for (const m of list) {
      const idx = MUTAGEN_ORDER.indexOf(m);
      const key = idx < 0 ? undefined : this.mutagenStarKeys[idx];
      if (key !== undefined) out.push([key, ...(this.extraMutagenStarKeys?.[m] ?? [])]);
    }
    return out;
  }

  #resolve(target: PalaceTarget): Palace | undefined {
    if (target instanceof Palace) return target;
    return palaceCharts.get(this)?.palace(target);
  }

  #identifiers(): Set<string> {
    return starIdentifiers([...this.majorStars, ...this.minorStars, ...this.adjectiveStars]);
  }

  #chart(method: string): Astrolabe {
    const chart = palaceCharts.get(this);
    if (chart === undefined) {
      throw new Error(`${method} needs the owning chart: get the palace from an Astrolabe`);
    }
    return chart;
  }
}

/** 复制 DTO 的附加四化星；DTO 没有该键时为 undefined */
export function copyExtraMutagens(
  extra: Partial<Record<Mutagen, readonly string[]>> | undefined,
): Partial<Record<Mutagen, readonly string[]>> | undefined {
  if (extra === undefined) return undefined;
  return Object.fromEntries(Object.entries(extra).map(([m, stars]) => [m, [...stars]]));
}

/** 空值回退为全部四化，顺序为禄、权、科、忌 */
function orAllMutagens(mutagens: Mutagen | readonly Mutagen[] | undefined): readonly Mutagen[] {
  if (mutagens === undefined) return MUTAGEN_ORDER;
  const list = typeof mutagens === 'string' ? [mutagens] : mutagens;
  return list.length > 0 ? list : MUTAGEN_ORDER;
}

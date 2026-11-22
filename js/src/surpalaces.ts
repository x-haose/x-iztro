/**
 * 三方四正与夹宫。
 *
 * 紫微斗数不单看一宫：本宫、对宫、财帛位、官禄位四宫合看，`SurroundedPalaces` 把这四宫打包；
 * `FlankingPalaces` 打包目标宫前后相邻的两宫。两者的判断方法都对打包的诸宫取并集。
 */

import type { Mutagen } from './enums.js';
import { chartQuery } from './internal.js';
import type { Astrolabe } from './astrolabe.js';
import type { Palace } from './palace.js';
import { starIdentifiers } from './star-object.js';
import type { TextOptions } from './types.js';

/** 诸宫星耀的可匹配标识（标识与翻译名） */
function identifiersOf(palaces: readonly Palace[]): Set<string> {
  return starIdentifiers(palaces.flatMap((p) => [...p.majorStars, ...p.minorStars, ...p.adjectiveStars]));
}

/** 取宫位所属星盘，脱离星盘即报错 */
function chartOf(palace: Palace, what: string): Astrolabe {
  const chart = palace.astrolabe();
  if (chart === undefined) {
    throw new Error(`toText needs the owning chart: get the ${what} from an Astrolabe`);
  }
  return chart;
}

/** 三方四正 */
export class SurroundedPalaces {
  /** 本宫 */
  readonly target: Palace;
  /** 对宫 */
  readonly opposite: Palace;
  /** 财帛位（三方） */
  readonly wealth: Palace;
  /** 官禄位（三方） */
  readonly career: Palace;

  constructor(target: Palace, opposite: Palace, wealth: Palace, career: Palace) {
    this.target = target;
    this.opposite = opposite;
    this.wealth = wealth;
    this.career = career;
  }

  /**
   * 三方四正的语义化文本：四宫合看的完整描述。
   *
   * @throws Error 本宫脱离星盘单独构造
   * @throws IztroError 同 `Palace.toText`
   */
  toText(options: TextOptions = {}): string {
    return chartQuery(chartOf(this.target, 'surrounded palaces'), 'surroundedPalacesToText', {
      palaceIndex: this.target.index,
      knowledge: options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /** 是否包含指定的 **全部** 星耀（星耀标识或当前语言的星名） */
  have(stars: readonly string[]): boolean {
    const ids = identifiersOf(this.#palaces());
    return stars.every((s) => ids.has(s));
  }

  /** 是否 **一颗都不** 包含指定的星耀 */
  notHave(stars: readonly string[]): boolean {
    const ids = identifiersOf(this.#palaces());
    return stars.every((s) => !ids.has(s));
  }

  /** 是否包含指定星耀中的 **至少一颗** */
  haveOneOf(stars: readonly string[]): boolean {
    const ids = identifiersOf(this.#palaces());
    return stars.some((s) => ids.has(s));
  }

  /** 四宫中是否有指定四化 */
  haveMutagen(mutagen: Mutagen): boolean {
    return this.#palaces().some((p) => p.hasMutagen(mutagen));
  }

  /** 四宫中是否没有指定四化 */
  notHaveMutagen(mutagen: Mutagen): boolean {
    return !this.haveMutagen(mutagen);
  }

  #palaces(): Palace[] {
    return [this.target, this.opposite, this.wealth, this.career];
  }
}

/** 夹宫：目标宫前后相邻的两宫。十二宫首尾相连，首宫的前一宫是末宫 */
export class FlankingPalaces {
  /** 前一宫（索引 -1） */
  readonly previous: Palace;
  /** 后一宫（索引 +1） */
  readonly next: Palace;

  constructor(previous: Palace, next: Palace) {
    this.previous = previous;
    this.next = next;
  }

  /** 两宫所属的星盘 */
  astrolabe(): Astrolabe | undefined {
    return this.previous.astrolabe();
  }

  /**
   * 夹宫的语义化文本：前后两宫各一节的完整描述。
   *
   * @throws Error 两宫脱离星盘单独构造
   * @throws IztroError 同 `Palace.toText`
   */
  toText(options: TextOptions = {}): string {
    return chartQuery(chartOf(this.previous, 'flanking palaces'), 'flankingPalacesToText', {
      // 被夹的宫是前一宫的下一宫；十二宫首尾相连，索引对 12 回绕
      palaceIndex: (this.previous.index + 1) % 12,
      knowledge: options.knowledge,
      detail: options.detail,
      knowledgeDepth: options.knowledgeDepth,
      patternConfig: options.config,
    }) as string;
  }

  /** 两宫合计是否包含指定的 **全部** 星耀 */
  have(stars: readonly string[]): boolean {
    const ids = identifiersOf([this.previous, this.next]);
    return stars.every((s) => ids.has(s));
  }

  /** 两宫是否都 **不** 包含指定的星耀 */
  notHave(stars: readonly string[]): boolean {
    const ids = identifiersOf([this.previous, this.next]);
    return stars.every((s) => !ids.has(s));
  }

  /** 两宫合计是否包含指定星耀中的 **至少一颗** */
  haveOneOf(stars: readonly string[]): boolean {
    const ids = identifiersOf([this.previous, this.next]);
    return stars.some((s) => ids.has(s));
  }

  /** 两宫中是否有任一宫带指定四化 */
  haveMutagen(mutagen: Mutagen): boolean {
    return this.previous.hasMutagen(mutagen) || this.next.hasMutagen(mutagen);
  }

  /** 两宫是否都没有指定四化 */
  notHaveMutagen(mutagen: Mutagen): boolean {
    return !this.haveMutagen(mutagen);
  }
}

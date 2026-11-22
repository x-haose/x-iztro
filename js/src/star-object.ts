/**
 * 星耀对象。
 *
 * 星耀知道自己落在哪一宫、属于哪张盘，因此除了自身的亮度与四化，
 * 还能直接问出所在宫位的对宫与三方四正。
 */

import type { Brightness, Mutagen, Scope, StarType } from './enums.js';
import { starCharts, starPalaces } from './internal.js';
import type { Palace } from './palace.js';
import type { SurroundedPalaces } from './surpalaces.js';
import type { StarData } from './types.js';

/** 星耀 */
export class Star {
  /** 语言无关星耀标识（`MajorStar` / `MinorStar` / `AdjectiveStar` / `HoroscopeStar` 取值） */
  readonly key: string;
  /** 星耀名称（按排盘语言翻译） */
  readonly name: string;
  /** 星耀类型 */
  readonly type: StarType;
  /** 作用范围 */
  readonly scope: Scope;
  /** 亮度显示文本（如「庙」），无亮度为 undefined */
  readonly brightness: string | undefined;
  /** 语言无关亮度标识，无亮度为 undefined */
  readonly brightnessKey: Brightness | undefined;
  /** 四化显示文本（如「禄」），无四化为 undefined */
  readonly mutagen: string | undefined;
  /** 语言无关四化标识，无四化为 undefined */
  readonly mutagenKey: Mutagen | undefined;

  /** 由内核 DTO 构造；挂到星盘上的星耀由星盘构造时回填所在宫位与星盘 */
  constructor(data: StarData) {
    this.key = data.key;
    this.name = data.name;
    this.type = data.type;
    this.scope = data.scope;
    this.brightness = data.brightness || undefined;
    this.brightnessKey = data.brightnessKey;
    this.mutagen = data.mutagen || undefined;
    this.mutagenKey = data.mutagenKey;
  }

  /** 是否具有指定亮度（给数组时为其中任一） */
  withBrightness(brightness: Brightness | readonly Brightness[]): boolean {
    if (typeof brightness === 'string') return this.brightnessKey === brightness;
    return this.brightnessKey !== undefined && brightness.includes(this.brightnessKey);
  }

  /** 是否具有指定四化（给数组时为其中任一） */
  withMutagen(mutagen: Mutagen | readonly Mutagen[]): boolean {
    if (typeof mutagen === 'string') return this.mutagenKey === mutagen;
    return this.mutagenKey !== undefined && mutagen.includes(this.mutagenKey);
  }

  /** 星耀所在的宫位；脱离星盘单独构造的星耀（如运限流耀、安星函数的结果）为 undefined */
  palace(): Palace | undefined {
    return starPalaces.get(this);
  }

  /** 星耀所在宫位的对宫 */
  oppositePalace(): Palace | undefined {
    const palace = starPalaces.get(this);
    const chart = starCharts.get(this);
    if (palace === undefined || chart === undefined) return undefined;
    return chart.palaces[(palace.index + 6) % 12];
  }

  /** 星耀所在宫位的三方四正 */
  surroundedPalaces(): SurroundedPalaces | undefined {
    const palace = starPalaces.get(this);
    const chart = starCharts.get(this);
    if (palace === undefined || chart === undefined) return undefined;
    return chart.surroundedPalaces(palace.index);
  }
}

/** 星耀集合的可匹配标识：标识与翻译名都可作查询词 */
export function starIdentifiers(stars: Iterable<Star>): Set<string> {
  const out = new Set<string>();
  for (const s of stars) {
    out.add(s.key);
    out.add(s.name);
  }
  return out;
}

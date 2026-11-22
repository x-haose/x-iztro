/**
 * 格局命中。
 *
 * 判定在内核完成（规则见文档站「格局」一页），这里只把命中结果包成对象。
 * 本命盘调用 `Astrolabe.patterns()`，运限某层视角调用 `Horoscope.patterns(scope)`。
 */

import type { Brightness, Mutagen, PalaceName, PatternKey, Scope } from './enums.js';
import type { PatternHitData, PatternStarData } from './types.js';

/** 参与成格的一颗星及其落宫 */
export class PatternStar {
  /** 星耀标识 */
  readonly key: string;
  /** 星耀名称（按排盘语言翻译） */
  readonly name: string;
  /** 落宫索引 0-11，0 为寅宫 */
  readonly palaceIndex: number;
  /** 亮度显示文本，无亮度为 undefined */
  readonly brightness: string | undefined;
  /** 亮度标识，无亮度为 undefined */
  readonly brightnessKey: Brightness | undefined;
  /** 判定视角下的四化显示文本（本命为生年四化，运限为该层四化），无四化为 undefined */
  readonly mutagen: string | undefined;
  /** 四化标识，无四化为 undefined */
  readonly mutagenKey: Mutagen | undefined;

  constructor(data: PatternStarData) {
    this.key = data.key;
    this.name = data.name;
    this.palaceIndex = data.palaceIndex;
    this.brightness = data.brightness;
    this.brightnessKey = data.brightnessKey;
    this.mutagen = data.mutagen;
    this.mutagenKey = data.mutagenKey;
  }

  /** 亮度是否为指定值 */
  hasBrightness(brightness: Brightness): boolean {
    return this.brightnessKey !== undefined && this.brightnessKey === brightness;
  }

  /** 四化是否为指定值 */
  hasMutagen(mutagen: Mutagen): boolean {
    return this.mutagenKey !== undefined && this.mutagenKey === mutagen;
  }
}

/** 一次格局命中 */
export class PatternHit {
  /** 格局标识（`PatternKey` 取值） */
  readonly key: PatternKey;
  /** 格局名称（按排盘语言翻译） */
  readonly name: string;
  /** 判定视角：本命为 origin，运限为该层 */
  readonly scope: Scope;
  /** 成格所在宫位索引 0-11：多数为命宫，「身命」类可为身宫，任一宫成格的为实际落宫 */
  readonly palaceIndex: number;
  /** 成格宫位在该视角下的宫名（按排盘语言翻译） */
  readonly palaceName: string;
  /** 宫名标识 */
  readonly palaceNameKey: PalaceName;
  /** 「破格 / 加杀平常」条件是否触发：成格照报，仅作标记 */
  readonly broken: boolean;
  /** 参与成格的星与落宫 */
  readonly stars: readonly PatternStar[];
  /** 多口径格局命中的口径（如君臣庆会四形式），单口径为 undefined */
  readonly variant: string | undefined;
  readonly #data: PatternHitData;

  constructor(data: PatternHitData) {
    this.#data = data;
    this.key = data.key as PatternKey;
    this.name = data.name;
    this.scope = data.scope;
    this.palaceIndex = data.palaceIndex;
    this.palaceName = data.palaceName;
    this.palaceNameKey = data.palaceNameKey;
    this.broken = data.broken;
    this.stars = data.stars.map((s) => new PatternStar(s));
    this.variant = data.variant;
  }

  /** 是否为指定格局 */
  is(key: PatternKey): boolean {
    return this.key === key;
  }

  /** 成格宫位在该视角下是否为指定宫名（宫名标识或当前语言的宫名） */
  inPalace(name: PalaceName | string): boolean {
    return name === this.palaceNameKey || name === this.palaceName;
  }

  /** 命中的 DTO（camelCase 键，值按排盘语言翻译）的深拷贝 */
  toJSON(): PatternHitData {
    return structuredClone(this.#data);
  }
}

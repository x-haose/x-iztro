/**
 * 排盘算法用到的查表与顺序常量（iztro 的 `data` 模块）。
 *
 * 取值一律为语言无关标识，可直接同星盘对象的 `*Key` 字段比对；表是排盘的输入而非输出，与输出语言无关。
 */

import type { FiveElementsClass, Gender, HeavenlyStem } from './enums.js';
import { query } from './runtime.js';

/** 星耀基础信息 */
export interface StarInfo {
  /** 十二宫亮度标识，索引 0 为寅宫；该宫无亮度时为 null */
  brightness: (string | null)[];
  /** 五行；原表中太阳、七杀与六颗辅星未填，为 null */
  fiveElements: string | null;
  /** 阴阳；原表中部分星耀未填，为 null */
  yinYang: string | null;
}

/** 天干信息 */
export interface HeavenlyStemInfo {
  /** 阴阳 */
  yinYang: string;
  /** 五行 */
  fiveElements: string;
  /** 对冲天干标识；戊己无对冲，为 null */
  crash: string | null;
  /** 四化四星标识，顺序为禄、权、科、忌 */
  mutagen: string[];
}

/** 地支信息 */
export interface EarthlyBranchInfo {
  /** 阴阳 */
  yinYang: string;
  /** 五行 */
  fiveElements: string;
  /** 对冲地支标识 */
  crash: string;
  /** 命主星标识 */
  soul: string;
  /** 身主星标识 */
  body: string;
  /** 对应脏腑；只有中文一种写法 */
  inside: string;
  /** 对应身体部位；只有中文一种写法 */
  outside: string;
  /** 健康提示；只有中文一种写法 */
  healthTip: string;
}

/** 顺序常量与推算规则表 */
export interface Constants {
  /** 支持的语言代码 */
  languages: string[];
  /** 天干顺序 */
  heavenlyStems: string[];
  /** 地支顺序 */
  earthlyBranches: string[];
  /** 生肖标识，按地支顺序 */
  zodiac: string[];
  /** 星座标识，按黄道顺序 */
  signs: string[];
  /** 十二宫名，按命宫起的顺序 */
  palaces: string[];
  /** 男女各自的阴阳 */
  gender: Record<Gender, string>;
  /** 时辰标识，索引 0-12 */
  chineseTime: string[];
  /** 时辰对应的时间区间 */
  timeRange: string[];
  /** 五虎遁：年干推正月天干 */
  tigerRule: Record<HeavenlyStem, HeavenlyStem>;
  /** 五鼠遁：日干推子时天干 */
  ratRule: Record<HeavenlyStem, HeavenlyStem>;
  /** 四化顺序：禄、权、科、忌 */
  mutagen: string[];
  /** 五行局标识对应的局数 2-6，与 `FIVE_ELEMENTS_CLASS_NUMBER` 同源 */
  fiveElementsClass: Record<FiveElementsClass, number>;
}

/**
 * 流耀 → 对应本命辅星的全量对照表（50 条）。流耀（运昌、流魁等）没有独立的知识包条目，释义按对应的
 * 本命辅星查，这张表就是官方对照。键为流耀标识（`HoroscopeStar` 取值），值为本命辅星标识（`MinorStar` 取值）。
 */
export function flowStarCounterparts(): Record<string, string> {
  return query('flowStarCounterparts') as Record<string, string>;
}

/** 星耀基础信息：十四主星与文昌、文曲、火星、铃星、擎羊、陀罗共二十颗，键为星耀标识 */
export function starsInfo(): Record<string, StarInfo> {
  return query('starsInfo') as Record<string, StarInfo>;
}

/** 天干信息表，键为天干标识 */
export function heavenlyStems(): Record<HeavenlyStem, HeavenlyStemInfo> {
  return query('heavenlyStems') as Record<HeavenlyStem, HeavenlyStemInfo>;
}

/** 地支信息表，键为地支标识 */
export function earthlyBranches(): Record<string, EarthlyBranchInfo> {
  return query('earthlyBranches') as Record<string, EarthlyBranchInfo>;
}

/** 顺序常量与推算规则表 */
export function constants(): Constants {
  const raw = query('constants') as Record<string, unknown>;
  return {
    languages: raw.LANGUAGES as string[],
    heavenlyStems: raw.HEAVENLY_STEMS as string[],
    earthlyBranches: raw.EARTHLY_BRANCHES as string[],
    zodiac: raw.ZODIAC as string[],
    signs: raw.SIGNS as string[],
    palaces: raw.PALACES as string[],
    gender: raw.GENDER as Constants['gender'],
    chineseTime: raw.CHINESE_TIME as string[],
    timeRange: raw.TIME_RANGE as string[],
    tigerRule: raw.TIGER_RULE as Constants['tigerRule'],
    ratRule: raw.RAT_RULE as Constants['ratRule'],
    mutagen: raw.MUTAGEN as string[],
    fiveElementsClass: raw.FIVE_ELEMENTS_CLASS as Constants['fiveElementsClass'],
  };
}

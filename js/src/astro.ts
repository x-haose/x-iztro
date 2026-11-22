/**
 * 排盘入口、不需要完整星盘的轻量查询、宫位推算与插件。
 *
 * 与 iztro 的 `astro` 模块对应。轻量查询与排盘共用同一套内核逻辑，结果与完整排盘的对应字段永远一致。
 */

import { Astrolabe, newChart } from './astrolabe.js';
import type { EarthlyBranch, FiveElementsClass, Gender, HeavenlyStem, Language, PalaceName, TimeIndex } from './enums.js';
import { invoke, query } from './runtime.js';
import type { Decadal } from './palace.js';
import type { AstrolabeData, ChartConfig, DecadalData } from './types.js';

/** 阳历排盘的可选项 */
export interface SolarOptions {
  /** 是否修正闰月：闰月前半月算本月、后半月算下月。默认 true */
  fixLeap?: boolean | undefined;
  /** 输出语言，默认 zh-CN */
  language?: Language | undefined;
  /** 排盘配置；省略取与 iztro 一致的默认值 */
  config?: ChartConfig | undefined;
}

/** 农历排盘的可选项 */
export interface LunarOptions extends SolarOptions {
  /** 农历日期是否为闰月（该月无闰月时不生效），默认 false */
  isLeapMonth?: boolean | undefined;
}

/**
 * 阳历排盘。
 *
 * @param solarDate 阳历日期，如 "2000-8-16"（分隔符可为 `-` `/` `.`，可带时间，时间不参与排盘）
 * @param timeIndex 时辰索引 0-12
 * @param gender "male" 或 "female"
 * @throws IztroError 入参非法（日期格式或范围、时辰索引、性别、语言或配置）
 */
export function bySolar(solarDate: string, timeIndex: TimeIndex, gender: Gender, options: SolarOptions = {}): Astrolabe {
  const data = invoke('by_solar', {
    solarDate,
    timeIndex,
    gender,
    fixLeap: options.fixLeap ?? true,
    language: options.language ?? 'zh-CN',
    config: options.config,
  }) as AstrolabeData;
  return newChart(data, options.config);
}

/**
 * 农历排盘。
 *
 * @param lunarDate 农历日期，如 "2000-7-17"
 * @param timeIndex 时辰索引 0-12
 * @param gender "male" 或 "female"
 * @throws IztroError 入参非法（日期格式或范围、时辰索引、性别、语言或配置）
 */
export function byLunar(lunarDate: string, timeIndex: TimeIndex, gender: Gender, options: LunarOptions = {}): Astrolabe {
  const data = invoke('by_lunar', {
    lunarDate,
    timeIndex,
    gender,
    isLeapMonth: options.isLeapMonth ?? false,
    fixLeap: options.fixLeap ?? true,
    language: options.language ?? 'zh-CN',
    config: options.config,
  }) as AstrolabeData;
  return newChart(data, options.config);
}

// ------ 轻量查询 ------

/** 轻量查询的 `{text, keys}` 双轨结果 */
interface TextAndKeys {
  text: string;
  keys: string[];
}

/**
 * 阳历日期的生肖。生肖由年支决定，换年时点受 `config.yearDivide` 影响。
 *
 * @throws IztroError 日期非法
 */
export function getZodiacBySolarDate(solarDate: string, language: Language = 'zh-CN', config?: ChartConfig): string {
  return (query('zodiacBySolar', { solarDate, language, config }) as TextAndKeys).text;
}

/**
 * 阳历日期的星座；只由公历日期决定。
 *
 * @throws IztroError 日期非法
 */
export function getSignBySolarDate(solarDate: string, language: Language = 'zh-CN'): string {
  return (query('signBySolar', { solarDate, language }) as TextAndKeys).text;
}

/**
 * 农历日期的星座。
 *
 * @param isLeapMonth 是否闰月（该月无闰月时不生效）
 * @throws IztroError 日期非法
 */
export function getSignByLunarDate(lunarDate: string, isLeapMonth = false, language: Language = 'zh-CN'): string {
  return (query('signByLunar', { lunarDate, isLeapMonth, language }) as TextAndKeys).text;
}

/**
 * 阳历日期的命宫主星，多颗以逗号分隔；命宫为空宫时借对宫主星，与 iztro 一致。
 *
 * @throws IztroError 日期或时辰索引非法
 */
export function getMajorStarBySolarDate(solarDate: string, timeIndex: TimeIndex, options: SolarOptions = {}): string {
  return (
    query('majorStarBySolar', {
      solarDate,
      timeIndex,
      fixLeap: options.fixLeap ?? true,
      language: options.language ?? 'zh-CN',
      config: options.config,
    }) as TextAndKeys
  ).text;
}

/**
 * 农历日期的命宫主星，多颗以逗号分隔；命宫为空宫时借对宫主星。
 *
 * @throws IztroError 日期或时辰索引非法
 */
export function getMajorStarByLunarDate(lunarDate: string, timeIndex: TimeIndex, options: LunarOptions = {}): string {
  return (
    query('majorStarByLunar', {
      lunarDate,
      timeIndex,
      isLeapMonth: options.isLeapMonth ?? false,
      fixLeap: options.fixLeap ?? true,
      language: options.language ?? 'zh-CN',
      config: options.config,
    }) as TextAndKeys
  ).text;
}

/**
 * 阳历日期的命宫主星标识（`MajorStar` 取值）：与 `getMajorStarBySolarDate` 同一结果的标识投影，与输出语言无关。
 *
 * @throws IztroError 日期或时辰索引非法
 */
export function getMajorStarKeysBySolarDate(
  solarDate: string,
  timeIndex: TimeIndex,
  options: Omit<SolarOptions, 'language'> = {},
): string[] {
  return (
    query('majorStarBySolar', {
      solarDate,
      timeIndex,
      fixLeap: options.fixLeap ?? true,
      language: 'zh-CN',
      config: options.config,
    }) as TextAndKeys
  ).keys;
}

/**
 * 农历日期的命宫主星标识（`MajorStar` 取值），与输出语言无关。
 *
 * @throws IztroError 日期或时辰索引非法
 */
export function getMajorStarKeysByLunarDate(
  lunarDate: string,
  timeIndex: TimeIndex,
  options: Omit<LunarOptions, 'language'> = {},
): string[] {
  return (
    query('majorStarByLunar', {
      lunarDate,
      timeIndex,
      isLeapMonth: options.isLeapMonth ?? false,
      fixLeap: options.fixLeap ?? true,
      language: 'zh-CN',
      config: options.config,
    }) as TextAndKeys
  ).keys;
}

// ------ 宫位推算 ------

/** 命宫与身宫的落点 */
export interface SoulAndBody {
  /** 命宫的宫位索引 */
  soulIndex: number;
  /** 身宫的宫位索引 */
  bodyIndex: number;
  /** 命宫天干标识 */
  heavenlyStemOfSoul: HeavenlyStem;
  /** 命宫地支标识 */
  earthlyBranchOfSoul: EarthlyBranch;
}

/**
 * 由农历月索引、时辰索引与年干推命宫与身宫。
 *
 * @param monthIndex 农历月索引（正月为 0），可用 `util.fixLunarMonthIndex` 求得
 * @param timeIndex 时辰索引 0-12
 * @param yearlyStem 年干标识
 */
export function getSoulAndBody(monthIndex: number, timeIndex: TimeIndex, yearlyStem: HeavenlyStem): SoulAndBody {
  return query('getSoulAndBody', { monthIndex, timeIndex, stemKey: yearlyStem }) as SoulAndBody;
}

/** 由命宫干支推五行局标识 */
export function getFiveElementsClass(stem: HeavenlyStem, branch: EarthlyBranch): FiveElementsClass {
  return query('getFiveElementsClass', { stemKey: stem, branchKey: branch }) as FiveElementsClass;
}

/** 由命宫索引推十二宫名标识，按宫位索引排列：第 i 项即 `astrolabe.palaces[i]` 的宫名 */
export function getPalaceNames(soulIndex: number): PalaceName[] {
  return query('getPalaceNames', { soulIndex }) as PalaceName[];
}

/** 十二宫的大限与小限，两组都按宫位索引排列 */
export interface DecadalsAndAges {
  /** 每宫的大限：岁数区间与该宫干支 */
  decadals: Decadal[];
  /** 每宫的小限岁数 */
  ages: number[][];
}

/**
 * 由命宫索引与五行局推十二宫的大限与小限（iztro 同名函数）。
 * 大限起运岁数由五行局决定，顺逆由性别阴阳与年支阴阳决定；小限起宫由年支决定。
 */
export function getHoroscope(
  soulIndex: number,
  fiveElementsClass: FiveElementsClass,
  gender: Gender,
  yearlyStem: HeavenlyStem,
  yearlyBranch: EarthlyBranch,
): DecadalsAndAges {
  return query('getHoroscope', {
    soulIndex,
    fiveElementsClass,
    gender,
    stemKey: yearlyStem,
    branchKey: yearlyBranch,
  }) as { decadals: DecadalData[]; ages: number[][] };
}

// ------ 插件 ------

/** 插件：接受 `Astrolabe` 类，往它的原型上挂方法；加载后所有星盘都有这些方法，包括加载之前已排好的盘 */
export type Plugin = (astrolabeClass: typeof Astrolabe) => void;

/**
 * 加载单个插件。TypeScript 用户用声明合并给 `Astrolabe` 补上方法的类型：
 * `declare module 'x-iztro' { interface Astrolabe { majorStar(): string } }`。
 *
 * @throws TypeError plugin 不是函数
 */
export function loadPlugin(plugin: Plugin): void {
  if (typeof plugin !== 'function') {
    throw new TypeError(`plugin must be a function, got ${typeof plugin}`);
  }
  plugin(Astrolabe);
}

/** 按顺序加载多个插件；任一插件不是函数即报错，且不加载其后的插件 */
export function loadPlugins(plugins: Iterable<Plugin>): void {
  for (const plugin of plugins) {
    loadPlugin(plugin);
  }
}

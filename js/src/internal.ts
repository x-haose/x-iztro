/**
 * 对象之间的内部关联与「从星盘再发起计算」的统一入口，不对外导出。
 *
 * 绑定层调用无状态：宫位的文本、运限、格局、按盘取材等都要把星盘的排盘上下文
 * （生辰、语言、用户传入的配置原件、重排起点）原样送回内核再算一次。
 * 上下文只在这里拼一处；用户配置原件含自定义四化与亮度表，DTO 回显的配置不含，
 * 拿 DTO 的配置再算会让覆盖表在第二跳静默丢失。
 */

import type { Astrolabe } from './astrolabe.js';
import type { Horoscope } from './horoscope.js';
import type { KnowledgePack } from './knowledge.js';
import type { Palace } from './palace.js';
import { query } from './runtime.js';
import type { Star } from './star-object.js';
import type { ChartConfig, Knowledge, PatternConfig } from './types.js';

/** 星盘再发起计算所需、DTO 里没有的上下文 */
export interface ChartContext {
  /** 排盘时用户传入的配置原件；未传时为 undefined，内核取默认 */
  readonly config: ChartConfig | undefined;
  /** 重排起点 [天干标识, 地支标识]；非重排盘为 undefined */
  readonly from: readonly [string, string] | undefined;
}

/** 星盘 → 再发起计算的上下文 */
export const chartContexts = new WeakMap<Astrolabe, ChartContext>();
/** 宫位 → 所属星盘 */
export const palaceCharts = new WeakMap<Palace, Astrolabe>();
/** 星耀 → 所在宫位 */
export const starPalaces = new WeakMap<Star, Palace>();
/** 星耀 → 所属星盘 */
export const starCharts = new WeakMap<Star, Astrolabe>();

/** 知识包的内部状态 */
export interface PackState {
  /** 包对象本身（不复制） */
  readonly raw: Record<string, unknown>;
  /** 本实例就是该语言的内嵌包时记语言，发给内核时只发 "builtin" 而不搬整包；其余为 undefined */
  readonly builtinLanguage: string | undefined;
}

/** 知识包实例 → 内部状态 */
export const packStates = new WeakMap<KnowledgePack, PackState>();

/** 取知识包的内部状态；不是本库构造的实例即报 TypeError */
export function packState(pack: KnowledgePack): PackState {
  const state = packStates.get(pack);
  if (state === undefined) {
    throw new TypeError('knowledge must be true or a KnowledgePack built by x-iztro');
  }
  return state;
}

/**
 * 把 `knowledge` 选项落成内核入参：`true` 与「就是本盘语言的内嵌包」的实例都发 "builtin"
 * （内核自取，免得每次搬整包），其余实例发包对象，`false` / `null` / 省略不发。
 *
 * @throws TypeError 既不是布尔也不是 KnowledgePack
 */
export function wireKnowledge(knowledge: Knowledge | undefined, language: string): unknown {
  if (knowledge === undefined || knowledge === null || knowledge === false) return undefined;
  if (knowledge === true) return 'builtin';
  if (typeof knowledge !== 'object') {
    throw new TypeError(`knowledge must be true or a KnowledgePack, got ${typeof knowledge}`);
  }
  const state = packState(knowledge);
  return state.builtinLanguage === language ? 'builtin' : state.raw;
}

/** 再发起计算时的额外入参；`knowledge` 在这里换成内核入参，其余原样转发 */
export interface ContextParams {
  [key: string]: unknown;
  knowledge?: Knowledge | undefined;
  patternConfig?: PatternConfig | undefined;
}

/** 以星盘的排盘上下文（含重排起点）调用统一查询入口 */
export function chartQuery(chart: Astrolabe, kind: string, extra: ContextParams = {}): unknown {
  const context = chartContexts.get(chart);
  if (context === undefined) {
    throw new Error('this Astrolabe was not created by x-iztro');
  }
  const params: Record<string, unknown> = {
    ...birthParams(chart, context),
    ...extra,
  };
  if ('knowledge' in extra) {
    params.knowledge = wireKnowledge(extra.knowledge, chart.language);
  }
  return query(kind, params);
}

/** 星盘的生辰与排盘上下文，作为内核入参 */
export function birthParams(chart: Astrolabe, context: ChartContext): Record<string, unknown> {
  return {
    solarDate: chart.solarDate,
    timeIndex: chart.timeIndex,
    gender: chart.genderKey,
    fixLeap: chart.fixLeap,
    language: chart.language,
    config: context.config ?? chart.config,
    fromStem: context.from?.[0],
    fromBranch: context.from?.[1],
  };
}

/**
 * 发起运限的目标，原样记下供再发起计算时转发：日期（可带时间）与时辰，或农历年份（可再配农历月号）。
 * DTO 里的 `solarDate` 是规范化后的、省略时辰时已丢掉时间部分，不能拿来代替。
 */
export interface HoroscopeTarget {
  /** 目标阳历日期串；按农历年发起的为 undefined */
  readonly date?: string;
  /** 目标时辰索引；undefined 表示由目标日期串换算 */
  readonly timeIndex?: number;
  /** 目标农历年份；按日期发起的为 undefined */
  readonly year?: number;
  /** 目标农历月号，与 year 成对；其余发起方式为 undefined */
  readonly month?: number;
}

/** 运限再发起计算所需、DTO 里没有的上下文 */
export interface HoroscopeContext {
  /** 发起这次运限的星盘 */
  readonly chart: Astrolabe;
  /** 发起时的运限目标 */
  readonly target: HoroscopeTarget;
}

/** 运限 → 再发起计算的上下文 */
export const horoscopeContexts = new WeakMap<Horoscope, HoroscopeContext>();

/** 取运限的上下文；不是本库构造的实例即报错 */
export function horoscopeContext(horoscope: Horoscope): HoroscopeContext {
  const context = horoscopeContexts.get(horoscope);
  if (context === undefined) {
    throw new Error('this Horoscope was not created by x-iztro');
  }
  return context;
}

/**
 * 以发起运限的排盘上下文再发起计算：星盘上下文加上发起时的运限目标。
 *
 * @param astrolabe 显式指定的星盘；省略取发起这次运限的那张盘
 */
export function horoscopeQuery(
  horoscope: Horoscope,
  kind: string,
  extra: ContextParams,
  astrolabe?: Astrolabe,
): unknown {
  const { chart, target } = horoscopeContext(horoscope);
  return chartQuery(astrolabe ?? chart, kind, {
    targetDate: target.date,
    targetTimeIndex: target.timeIndex,
    targetYear: target.year,
    targetMonth: target.month,
    ...extra,
  });
}

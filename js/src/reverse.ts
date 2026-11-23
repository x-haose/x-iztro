/**
 * 反推：由八字四柱或星盘特征反查候选生辰。
 *
 * 计算全部在内核完成（剪枝枚举 + 正排终验，与正向排盘零分歧）。星盘布局与性别无关，反推的目标是生辰，故不收性别。
 */

import type { EarthlyBranch, FiveElementsClass, HeavenlyStem, TimeIndex } from './enums.js';
import { query } from './runtime.js';
import type { ChartConfig } from './types.js';

/** 一柱干支：[天干标识, 地支标识] */
export type Pillar = readonly [HeavenlyStem, EarthlyBranch];

/** 一个候选生辰，可直接交给 `astro.bySolar` 排盘 */
export interface BirthCandidate {
  /** 公历日期 `Y-M-D` */
  solarDate: string;
  /** 时辰索引 0-12（0 为早子时，12 为晚子时） */
  timeIndex: TimeIndex;
}

/** 一颗星与其落宫地支：星盘特征反推的原子条件 */
export interface StarPosition {
  /** 星耀标识（须为本命盘星耀，流耀不接受） */
  star: string;
  /** 落宫地支标识 */
  branch: EarthlyBranch;
}

/** 星盘特征反推的条件集。全部可选，但至少要给一个条件；条件越具体（命宫地支、生年四化、主星落宫）反推越快 */
export interface ReverseCriteria {
  /** 命宫地支 */
  soulBranch?: EarthlyBranch | undefined;
  /** 身宫地支 */
  bodyBranch?: EarthlyBranch | undefined;
  /** 五行局 */
  fiveElementsClass?: FiveElementsClass | undefined;
  /** 星耀落宫条件，全部须同时满足 */
  stars?: readonly StarPosition[] | undefined;
  /** 生年四化 [禄, 权, 科, 忌] 各自是哪颗星（星耀标识），不限定的位次写 null */
  mutagens?: readonly [string | null, string | null, string | null, string | null] | undefined;
  /** 公历年闭区间 [起, 止]，须落在 1-9999 内；默认 [1900, 2100] */
  yearRange?: readonly [number, number] | undefined;
  /** 是否修正闰月（与排盘入参同义），默认 true */
  fixLeap?: boolean | undefined;
  /** 候选数上限：达到即停止搜索并置 truncated；省略或 0 取内核默认（512） */
  limit?: number | undefined;
}

/** 星盘特征反推的结果 */
export interface ReverseResult {
  /** 满足全部条件的候选生辰，按枚举序：农历年升序，年内依 月→时辰→日；同一年内不保证公历日期升序 */
  candidates: BirthCandidate[];
  /** 是否因达到候选数上限而提前截断；截断时枚举序更靠后的解未被搜索，其中可能有公历日期更早的解 */
  truncated: boolean;
}

/**
 * 由八字四柱反查公历生辰。
 *
 * 四柱按 config 的分界口径解释（年柱 yearDivide、月柱 horoscopeDivide、晚子归属 dayDivide），
 * 与星盘 `rawDates.chineseDate` 同一套语义。一组四柱在范围内通常每约 60 年出现一次；子时可能给出早晚子两个候选。
 *
 * @param options.yearRange 公历年闭区间，默认 [1900, 2100]，须落在 1-9999 内
 * @throws IztroError 干支阴阳不配、年份范围非法
 */
export function solarDatesByBazi(
  yearly: Pillar,
  monthly: Pillar,
  daily: Pillar,
  hourly: Pillar,
  options: { yearRange?: readonly [number, number]; config?: ChartConfig } = {},
): BirthCandidate[] {
  const [startYear, endYear] = options.yearRange ?? [1900, 2100];
  return query('solarDatesByBazi', {
    pillars: [yearly, monthly, daily, hourly],
    startYear,
    endYear,
    config: options.config,
  }) as BirthCandidate[];
}

/**
 * 由星盘特征反查候选生辰。排盘配置贯穿判定（四化表、派别、分界都按它算），候选用同一配置排盘必满足全部条件。
 *
 * @throws IztroError 条件为空、包含流耀、年份范围非法
 */
export function reverseChart(criteria: ReverseCriteria, config?: ChartConfig): ReverseResult {
  return query('reverseChart', { reverseCriteria: criteria, config }) as ReverseResult;
}

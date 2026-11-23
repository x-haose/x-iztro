/**
 * 历法查询：农历年的月份表、公历与农历互换、农历年的年干支。
 *
 * 与排盘同一份历法内核，结果与星盘的 `rawDates.lunarDate`、年柱逐日一致。支持公历与农历 1-9999 年；
 * 1582-10-15 以前按儒略历，1582-10-05 至 10-14 不存在。
 *
 * 报错与排盘入口一致：给出完整日期的换算在日期不存在时抛 `IztroError`（code 为 `invalid_date`），
 * 只给年份的查询在年份越界时抛 `IztroError`（code 为 `invalid_argument`）。
 */

import type { Pillar } from './reverse.js';
import { query } from './runtime.js';
import type { LunarMonthInfo, RawLunarDate, RawSolarDate } from './types.js';

/** `lunarToSolar` 的选项 */
export interface LunarToSolarOptions {
  /**
   * 这一天在闰月里，默认 false。该年没有这个闰月时报错，不退回同号的常规月。
   * 排盘的 `fixLeap`（闰月十五之后视作次月）管的是星曜按哪个月安，不改变日期本身，这里没有对应选项。
   */
  isLeap?: boolean | undefined;
}

/**
 * 农历 `year` 年的全部月份，按时间先后排列；闰月紧跟在所闰的月之后。
 *
 * 一般为 12 个月，有闰月的年为 13 个月。个别古代年份例外，与 lunar-typescript 相同：
 * 如农历 237 年只有 11 个月、没有三月。
 *
 * @throws IztroError `year` 不在 1-9999（invalid_argument）
 */
export function lunarMonths(year: number): LunarMonthInfo[] {
  return query('lunarMonths', { year }) as LunarMonthInfo[];
}

/**
 * 公历日期对应的农历日期。公历 1 年 1 月初的几十天属农历 0 年（冬月、腊月），此时 `lunarYear` 为 0。
 *
 * @throws IztroError 年不在 1-9999、月日不存在，或落在改历空缺 1582-10-05 至 10-14（invalid_date）
 */
export function solarToLunar(year: number, month: number, day: number): RawLunarDate {
  return query('solarToLunar', { year, month, day }) as RawLunarDate;
}

/**
 * 农历日期对应的公历日期。
 *
 * @throws IztroError 年不在 1-9999、该年没有这个月（含不闰此月）、日超出当月天数（invalid_date）
 */
export function lunarToSolar(
  year: number,
  month: number,
  day: number,
  options: LunarToSolarOptions = {},
): RawSolarDate {
  return query('lunarToSolar', {
    year,
    lunarMonth: month,
    lunarDay: day,
    isLeap: options.isLeap ?? false,
  }) as RawSolarDate;
}

/**
 * 农历 `year` 年的年干支标识（公历 4 年为甲子）。
 *
 * 年干支按年号循环，与分界口径无关；口径决定这一柱从哪天起算：正月初一分界（默认）为农历
 * `year` 年正月初一至除夕，立春分界为公历 `year` 年立春当日至次年立春前一日。
 *
 * @throws IztroError `year` 不在 1-9999（invalid_argument）
 */
export function lunarYearPillar(year: number): Pillar {
  return query('lunarYearPillar', { year }) as Pillar;
}

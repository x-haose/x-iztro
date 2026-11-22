/**
 * 排盘算法的公开工具函数（iztro 的 `util` 模块）。
 *
 * 参数与返回值中的枚举一律是语言无关标识，因此这些函数与输出语言无关，
 * 可直接与星盘对象的 `*Key` 字段互操作。
 */

import type { Brightness, EarthlyBranch, HeavenlyStem, Language, Mutagen, TimeIndex } from './enums.js';
import { query } from './runtime.js';
import type { Star } from './star-object.js';
import type { ChartConfig } from './types.js';

/** 把索引约束到 0..max 的循环区间；负数也能正确回绕 */
export function fixIndex(index: number, max = 12): number {
  return query('fixIndex', { index, max }) as number;
}

/** 地支标识转宫位索引（寅宫为 0） */
export function earthlyBranchIndexToPalaceIndex(branch: EarthlyBranch): number {
  return query('earthlyBranchToPalaceIndex', { branchKey: branch }) as number;
}

/** 小时 0-23 转时辰索引：0 为早子时，12 为晚子时 */
export function timeToIndex(hour: number): TimeIndex {
  return query('timeToIndex', { hour }) as TimeIndex;
}

/** 由生年地支取小限起始宫位索引 */
export function getAgeIndex(branch: EarthlyBranch): number {
  return query('getAgeIndex', { branchKey: branch }) as number;
}

/**
 * 星耀落在指定宫位时的亮度标识；该星没有亮度表时为 undefined。
 *
 * @param palaceIndex 宫位索引，越界对 12 取模
 */
export function getBrightness(star: string, palaceIndex: number, config?: ChartConfig): Brightness | undefined {
  return (query('getBrightness', { starKey: star, index: palaceIndex, config }) as Brightness | null) ?? undefined;
}

/** 指定天干下某颗星化什么；不在该天干四化表内时为 undefined */
export function getMutagen(star: string, stem: HeavenlyStem, config?: ChartConfig): Mutagen | undefined {
  return (query('getMutagen', { starKey: star, stemKey: stem, config }) as Mutagen | null) ?? undefined;
}

/** 指定天干化出的四颗星标识，顺序为禄、权、科、忌 */
export function getMutagensByHeavenlyStem(stem: HeavenlyStem, config?: ChartConfig): string[] {
  return query('getMutagensByHeavenlyStem', { stemKey: stem, config }) as string[];
}

/** 修正后的农历月索引（正月为 0）：修正闰月时闰月十五日之后按下月算；晚子时（索引 12）不参与修正 */
export function fixLunarMonthIndex(
  lunarMonth: number,
  lunarDay: number,
  isLeap: boolean,
  timeIndex: TimeIndex,
  fixLeap: boolean,
): number {
  return query('fixLunarMonthIndex', { lunarMonth, lunarDay, isLeap, timeIndex, fixLeap }) as number;
}

/** 修正后的农历日索引；晚子时属次日，因此不减一 */
export function fixLunarDayIndex(lunarDay: number, timeIndex: TimeIndex): number {
  return query('fixLunarDayIndex', { lunarDay, timeIndex }) as number;
}

/**
 * 按语言拼接四柱干支展示串（星盘 `chineseDate` 字段即由此生成）：词条均为单字符时柱内紧凑相连、柱间空格
 * （如「庚辰 甲申 丁未 庚子」），任一词条为多字符时柱内空格、柱间「 - 」。
 *
 * @param pillars 四柱标识 [年, 月, 日, 时]，每柱 [天干, 地支]，如 `rawDates.chineseDate.yearlyKeys`
 * @throws IztroError 柱数不为四、某柱不是两项，或干支标识非法
 */
export function translateChineseDate(
  pillars: readonly (readonly [HeavenlyStem, EarthlyBranch])[],
  language: Language = 'zh-CN',
): string {
  return query('translateChineseDate', { pillars, language }) as string;
}

/**
 * 把多组「十二宫星耀」按宫位合并成一组，顺序为传入顺序。安星分批进行（主星、辅星、杂耀各出一组），
 * 本函数按宫位索引把它们首尾相接。
 *
 * @throws RangeError 某一组的长度不是 12
 */
export function mergeStars(...groups: readonly (readonly (readonly Star[])[])[]): Star[][] {
  const merged: Star[][] = Array.from({ length: 12 }, () => []);
  for (const group of groups) {
    if (group.length !== 12) {
      throw new RangeError(`each star group must cover 12 palaces, got ${group.length}`);
    }
    group.forEach((stars, i) => merged[i]?.push(...stars));
  }
  return merged;
}

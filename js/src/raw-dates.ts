/**
 * 星盘上的结构化出生日期：数字化农历生日与四柱干支。
 *
 * 是 `lunarDate` / `chineseDate` 两个展示串的数据形式，做日期运算或按干支查表时用它，不必解析展示串。
 */

import type { EarthlyBranch, HeavenlyStem } from './enums.js';
import type { Pillar } from './reverse.js';
import type { RawChineseDateData, RawLunarDate } from './types.js';

/** 四柱干支：每柱 [天干, 地支]，译文与标识两组 */
export class RawChineseDate {
  /** 年柱（翻译文本） */
  readonly yearly: readonly [string, string];
  /** 月柱（翻译文本） */
  readonly monthly: readonly [string, string];
  /** 日柱（翻译文本） */
  readonly daily: readonly [string, string];
  /** 时柱（翻译文本） */
  readonly hourly: readonly [string, string];
  /** 年柱标识 */
  readonly yearlyKeys: readonly [HeavenlyStem, EarthlyBranch];
  /** 月柱标识 */
  readonly monthlyKeys: readonly [HeavenlyStem, EarthlyBranch];
  /** 日柱标识 */
  readonly dailyKeys: readonly [HeavenlyStem, EarthlyBranch];
  /** 时柱标识 */
  readonly hourlyKeys: readonly [HeavenlyStem, EarthlyBranch];

  constructor(data: RawChineseDateData) {
    this.yearly = [...data.yearly];
    this.monthly = [...data.monthly];
    this.daily = [...data.daily];
    this.hourly = [...data.hourly];
    this.yearlyKeys = [...data.yearlyKeys];
    this.monthlyKeys = [...data.monthlyKeys];
    this.dailyKeys = [...data.dailyKeys];
    this.hourlyKeys = [...data.hourlyKeys];
  }

  /**
   * 四柱标识 [年, 月, 日, 时]，可直接交给 `util.translateChineseDate`，
   * 或展开交给 `solarDatesByBazi(...pillarKeys)`。
   */
  pillarKeys(): [Pillar, Pillar, Pillar, Pillar] {
    return [this.yearlyKeys, this.monthlyKeys, this.dailyKeys, this.hourlyKeys];
  }
}

/** 结构化的出生日期 */
export interface RawDates {
  /** 数字化农历生日 */
  readonly lunarDate: Readonly<RawLunarDate>;
  /** 四柱干支 */
  readonly chineseDate: RawChineseDate;
}

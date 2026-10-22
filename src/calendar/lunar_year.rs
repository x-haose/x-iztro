//! 农历年：月表（每月首日与天数、闰月）与节气表。移植自 lunar-typescript 的 `LunarYear.compute`。

use std::sync::OnceLock;

use super::shouxing::{J2000, calc_qi, calc_shuo, qi_accurate2};
use super::solar::SolarTime;
use super::tables::{LEAP_11, LEAP_12};

/// 农历年计算所覆盖的节气个数：上年大雪起，至次年惊蛰止。
pub(super) const JIE_QI_COUNT: usize = 31;
/// 节气表中「立春」的位置（当年）。
pub(super) const LI_CHUN: usize = 4;
/// 节气表中「立春」的位置（次年）。
pub(super) const NEXT_LI_CHUN: usize = 28;

/// 农历月。
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LunarMonth {
    /// 所属农历年
    pub year: i64,
    /// 带符号月：1-12 为常规月，负数为对应闰月
    pub month: i64,
    /// 天数（29 或 30）
    pub day_count: i64,
    /// 首日正午的儒略日（整数值）
    pub first_julian_day: f64,
}

/// 一个农历年的节气表：[`JIE_QI_COUNT`] 个节气的精确时刻（北京时间），自上年大雪起依次为
/// 大雪、冬至、小寒、大寒、立春 … 大雪、冬至、小寒、大寒、立春、雨水、惊蛰。
#[derive(Debug, Clone)]
pub(crate) struct JieQi {
    /// 节气时刻
    pub times: [SolarTime; JIE_QI_COUNT],
    /// 各时刻的比较键（[`SolarTime::ymdhms_key`]），与 `times` 一一对应
    pub keys: [String; JIE_QI_COUNT],
}

/// 一个农历年的推算结果。
///
/// 月表从上一年冬至所在月起共 15 个月，覆盖公历 `year` 年全年；其中归属农历 `year` 年的是
/// `year` 字段等于本年的那些月。节气表只在按节气分界时用到，且推算代价是月表的两倍多，
/// 首次取用时才算（[`LunarYear::jie_qi`]）。
#[derive(Debug)]
pub(crate) struct LunarYear {
    /// 农历年
    pub year: i64,
    /// 15 个农历月，按时间先后
    pub months: [LunarMonth; 15],
    /// 冬至起 26 个节气的整日（J2000 起算），定闰之用，也是精确节气时刻的迭代起点
    jq: [f64; 26],
    /// 节气表，首次取用时推算
    jie_qi: OnceLock<JieQi>,
}

impl LunarYear {
    /// 推算农历 `year` 年。
    pub(crate) fn compute(year: i64) -> Self {
        // 冬至起算的 26 个节气（整日），用于定闰
        let jd = ((year - 2000) as f64 * 365.2422 + 180.0).floor();
        let mut w = ((jd - 355.0 + 183.0) / 365.2422).floor() * 365.2422 + 355.0;
        if calc_qi(w) > jd {
            w -= 365.2422;
        }
        let jq: [f64; 26] = std::array::from_fn(|i| calc_qi(w + 15.2184 * i as f64));

        // 冬至前的合朔起 16 个朔日
        w = calc_shuo(jq[0]);
        if w > jq[0] {
            w -= 29.53;
        }
        let hs: [f64; 16] = std::array::from_fn(|i| calc_shuo(w + 29.5306 * i as f64));
        let day_counts: [i64; 15] = std::array::from_fn(|i| (hs[i + 1] - hs[i]).floor() as i64);

        // 闰月位置：历史记载优先，否则无中气之月为闰
        let leap_index = if LEAP_11.contains(&year) {
            13
        } else if LEAP_12.contains(&year) {
            14
        } else if hs[13] <= jq[24] {
            let mut i = 1;
            while i < 13 && hs[i + 1] > jq[2 * i] {
                i += 1;
            }
            i
        } else {
            16
        };
        let ordinals: [i64; 15] = std::array::from_fn(|i| {
            if i >= leap_index {
                i as i64 - 1
            } else {
                i as i64
            }
        });

        // 月序转月名：冬月起；武周与南朝陈两段特殊历法改月名
        const MONTH_NAMES: [i64; 12] = [11, 12, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let mut first_name = -1;
        let mut y = year - 1;
        let months = std::array::from_fn(|i| {
            let dm = hs[i] + J2000;
            let v2 = ordinals[i];
            let mut mc = MONTH_NAMES[(v2 % 12) as usize];
            if (1724360.0..1729794.0).contains(&dm) || (1807724.0..1808699.0).contains(&dm) {
                mc = MONTH_NAMES[((v2 + 1) % 12) as usize];
            } else if dm == 1729794.0 || dm == 1808699.0 {
                mc = 12;
            }
            if first_name == -1 {
                first_name = mc;
            }
            if mc < first_name {
                y += 1;
            }
            first_name = mc;
            if i == leap_index {
                mc = -mc;
            } else if dm == 1729794.0 || dm == 1808699.0 {
                mc = -11;
            }
            LunarMonth {
                year: y,
                month: mc,
                day_count: day_counts[i],
                first_julian_day: dm,
            }
        });

        LunarYear {
            year,
            months,
            jq,
            jie_qi: OnceLock::new(),
        }
    }

    /// 本年的节气表（首次调用时推算）。
    pub(crate) fn jie_qi(&self) -> &JieQi {
        self.jie_qi.get_or_init(|| {
            let jq = &self.jq;
            let times: [SolarTime; JIE_QI_COUNT] = std::array::from_fn(|i| {
                let t = if i == 0 {
                    qi_accurate2(jq[0] - 15.2184)
                } else if i <= 26 {
                    qi_accurate2(jq[i - 1])
                } else {
                    qi_accurate2(jq[25] + 15.2184 * (i - 26) as f64)
                };
                SolarTime::from_julian_day(t + J2000)
            });
            JieQi {
                keys: times.map(|t| t.ymdhms_key()),
                times,
            }
        })
    }

    /// 本年的某个月（带符号月号，负数为闰月）；本年无此月返回 `None`。
    pub(crate) fn month(&self, signed_month: i64) -> Option<&LunarMonth> {
        self.months
            .iter()
            .find(|m| m.year == self.year && m.month == signed_month)
    }

    /// 本年的闰月月份（1-12）；无闰月返回 `None`。
    pub(crate) fn leap_month(&self) -> Option<i64> {
        self.months
            .iter()
            .find(|m| m.year == self.year && m.month < 0)
            .map(|m| -m.month)
    }
}

//! 公历 ↔ 农历与八字各柱。移植自 lunar-typescript 的 `Lunar`（`fromSolar`、`fromYmdHms` 与
//! `_computeYear` / `_computeMonth` / `_computeDay` / `_computeTime`）。

use super::lunar_year::{LI_CHUN, LunarYear, NEXT_LI_CHUN};
use super::solar::SolarTime;
use crate::data::types::{EarthlyBranch, HeavenlyStem};

/// 一柱干支。
pub(crate) type Pillar = (HeavenlyStem, EarthlyBranch);

/// 农历日期。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LunarDate {
    /// 农历年
    pub year: i64,
    /// 带符号月：1-12 为常规月，负数为对应闰月
    pub month: i64,
    /// 日（1-30）
    pub day: i64,
}

/// 公历时刻在农历中的日期与不依赖节气的各柱干支；按节气分界的年柱与月柱见
/// [`year_by_li_chun`] 与 [`month_exact`]。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Lunar {
    /// 农历日期
    pub date: LunarDate,
    /// 年柱，正月初一换年
    pub year: Pillar,
    /// 日柱，子正（0 点）换日
    pub day: Pillar,
    /// 日柱，23 点起即算次日
    pub day_exact: Pillar,
    /// 时柱：时支按钟点，时干由 `day_exact` 的日干推出
    pub time: Pillar,
}

/// 时刻比较键（`… HH:MM:SS`）的日期部分。
fn date_part(ymdhms: &str) -> &str {
    &ymdhms[..ymdhms.len() - " HH:MM:SS".len()]
}

/// 天干序与地支序（可为负或越界）各自取模成一柱。
fn pillar(stem: i64, branch: i64) -> Pillar {
    (
        HeavenlyStem::from_index(stem.rem_euclid(10) as usize),
        EarthlyBranch::from_index(branch.rem_euclid(12) as usize),
    )
}

/// 公历日期所在的农历日期：在该公历年的农历年月表中找到所属月份。
pub(crate) fn lunar_date(ly: &LunarYear, solar: &SolarTime) -> LunarDate {
    for m in &ly.months {
        let days = solar.days_since(&SolarTime::from_julian_day(m.first_julian_day));
        if days < m.day_count {
            return LunarDate {
                year: m.year,
                month: m.month,
                day: days + 1,
            };
        }
    }
    unreachable!("农历年月表的 15 个月覆盖公历 {} 年全年", ly.year)
}

/// 农历日期对应的公历日期 (年, 月, 日)；该年无此月或日超出当月天数时返回 `None`。
pub(crate) fn solar_date(ly: &LunarYear, signed_month: i64, day: i64) -> Option<(i64, i64, i64)> {
    let m = ly.month(signed_month)?;
    if day < 1 || day > m.day_count {
        return None;
    }
    let noon = SolarTime::from_julian_day(m.first_julian_day + (day - 1) as f64);
    Some((noon.year, noon.month, noon.day))
}

/// 日柱的六十甲子序（甲子 = 0），子正换日。
pub(crate) fn day_cycle(year: i64, month: i64, day: i64) -> i64 {
    let noon = SolarTime::new(year, month, day, 12, 0);
    (noon.julian_day().floor() as i64 - 11).rem_euclid(60)
}

/// 公历时刻的农历日期与不依赖节气的各柱。
///
/// `ly` 必须是公历 `solar.year` 年对应的农历年（[`LunarYear::compute`]`(solar.year)`），
/// 下面两个按节气分界的函数同此要求。
pub(crate) fn lunar(ly: &LunarYear, solar: &SolarTime) -> Lunar {
    let date = lunar_date(ly, solar);
    let base = date.year - 4;

    // 日：正午儒略日定甲子序；23 点起 exact 日柱进一日
    let day = day_cycle(solar.year, solar.month, solar.day);
    let day_exact = if solar.hour == 23 { day + 1 } else { day };

    // 时：子时跨 23-1 点，此后每两小时一支
    let time_branch = (solar.hour + 1) / 2 % 12;
    let time_stem = day_exact.rem_euclid(10) % 5 * 2 + time_branch;

    Lunar {
        date,
        year: pillar(base, base),
        day: pillar(day, day),
        day_exact: pillar(day_exact, day_exact),
        time: pillar(time_stem, time_branch),
    }
}

/// 立春分界的年干支序（甲子 = 0 的同余数），返回 (立春当日换年, 立春时刻换年)。
/// `lunar_year` 为该时刻的农历年。
fn li_chun_years(ly: &LunarYear, solar: &SolarTime, lunar_year: i64) -> (i64, i64) {
    let jie_qi = ly.jie_qi();
    let ymdhms = solar.ymdhms_key();
    let ymd = date_part(&ymdhms);
    let base = lunar_year - 4;
    let (mut by_day, mut exact) = (base, base);
    let li_chun = if jie_qi.times[LI_CHUN].year == solar.year {
        LI_CHUN
    } else {
        NEXT_LI_CHUN
    };
    let key = jie_qi.keys[li_chun].as_str();
    let (li_chun_ymd, li_chun_ymdhms) = (date_part(key), key);
    if lunar_year == solar.year {
        if ymd < li_chun_ymd {
            by_day -= 1;
        }
        if ymdhms.as_str() < li_chun_ymdhms {
            exact -= 1;
        }
    } else if lunar_year < solar.year {
        if ymd >= li_chun_ymd {
            by_day += 1;
        }
        if ymdhms.as_str() >= li_chun_ymdhms {
            exact += 1;
        }
    }
    (by_day, exact)
}

/// 年柱，立春当日换年（只比日期）。`lunar_year` 为该时刻的农历年。
pub(crate) fn year_by_li_chun(ly: &LunarYear, solar: &SolarTime, lunar_year: i64) -> Pillar {
    let (by_day, _) = li_chun_years(ly, solar, lunar_year);
    pillar(by_day, by_day)
}

/// 月柱，交节时刻换月：从上年大雪起逐个「节」比较交节时刻，落在某两节之间即该月；
/// 月干由立春时刻换年的年干起。`lunar_year` 为该时刻的农历年。
pub(crate) fn month_exact(ly: &LunarYear, solar: &SolarTime, lunar_year: i64) -> Pillar {
    let (_, exact) = li_chun_years(ly, solar, lunar_year);
    let keys = &ly.jie_qi().keys;
    let ymdhms = solar.ymdhms_key();
    let mut index = -3;
    let mut start: Option<&str> = None;
    for key in keys.iter().step_by(2) {
        let end = key.as_str();
        let s = start.unwrap_or(ymdhms.as_str());
        if ymdhms.as_str() >= s && ymdhms.as_str() < end {
            break;
        }
        start = Some(end);
        index += 1;
    }
    let year_stem_exact = exact.rem_euclid(10);
    let offset = ((year_stem_exact + i64::from(index < 0)) % 5 + 1) * 2 % 10;
    pillar(
        (if index < 0 { index + 10 } else { index }) + offset,
        (if index < 0 { index + 12 } else { index }) + 2,
    )
}

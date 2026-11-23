//! 历法查询：农历年的月份表、公历与农历互换、农历年的年干支。
//!
//! 与排盘同一份历法内核（移植自 lunar-typescript，见 `crate::calendar`），结果与排盘里的
//! 农历日期、年柱逐日一致。支持公历与农历 1-9999 年；1582-10-15 以前按儒略历，
//! 1582-10-05 至 10-14 不存在。
//!
//! 入参是整数年月日而不是日期串：这组函数供日期选择器、日历视图这类按数值取用的场合，
//! 不需要先拼串再解析。
//!
//! 报错分两类，与排盘入口一致：给出完整日期的换算（[`solar_to_lunar`]、[`lunar_to_solar`]）
//! 在日期不存在时返回 [`IztroError::InvalidDate`]，同 [`crate::by_solar`] / [`crate::by_lunar`]；
//! 只给年份的查询（[`lunar_months`]、[`lunar_year_pillar`]）在年份越界时返回
//! [`IztroError::InvalidArgument`]，同 [`crate::models::astrolabe::Astrolabe::horoscope_of_year`]。

use crate::astro::builder::SUPPORTED_YEARS;
use crate::calendar;
use crate::data::types::{EarthlyBranch, HeavenlyStem};
use crate::error::IztroError;
use crate::models::astrolabe::RawLunarDate;

/// 数字化公历日期。1582-10-15 以前为儒略历日期。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawSolarDate {
    /// 公历年。农历 9999 年腊月初三起的 27 天落在公历 10000 年，[`lunar_to_solar`] 对它们返回 10000；
    /// 各月初一都在 9999 年内
    pub solar_year: i64,
    /// 公历月（1-12）
    pub solar_month: u32,
    /// 公历日（1-31）
    pub solar_day: u32,
}

/// 农历年里的一个月。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LunarMonthInfo {
    /// 月份（1-12）；闰月取所闰的月份，如闰四月为 4
    pub month: u32,
    /// 是否闰月
    pub is_leap: bool,
    /// 天数：29（小月）或 30（大月）
    pub day_count: u32,
    /// 初一对应的公历日期
    pub first_day: RawSolarDate,
}

/// 农历 `year` 年的全部月份，按时间先后排列；闰月紧跟在所闰的月之后。
///
/// 一般为 12 个月，有闰月的年为 13 个月。个别古代年份例外，与 lunar-typescript 相同：
/// 如农历 237 年只有 11 个月、没有三月。
///
/// # Errors
/// `year` 不在 1-9999 时返回 [`IztroError::InvalidArgument`]。
pub fn lunar_months(year: i64) -> Result<Vec<LunarMonthInfo>, IztroError> {
    check_year(year)?;
    let ly = calendar::lunar_year(year);
    Ok(ly
        .months
        .iter()
        .filter(|m| m.year == year)
        .map(|m| {
            let (y, mo, d) = calendar::solar_date_of(year, m.month, 1).expect("本年的月必有初一");
            LunarMonthInfo {
                month: m.month.unsigned_abs() as u32,
                is_leap: m.month < 0,
                day_count: m.day_count as u32,
                first_day: solar(y, mo, d),
            }
        })
        .collect())
}

/// 公历日期对应的农历日期。
///
/// 公历 1 年 1 月初的几十天属农历 0 年（冬月、腊月），此时 `lunar_year` 为 0。
///
/// # Errors
/// 年不在 1-9999、月不在 1-12、日超出当月天数，或落在改历空缺 1582-10-05 至 10-14 时
/// 返回 [`IztroError::InvalidDate`]。
pub fn solar_to_lunar(year: i64, month: u32, day: u32) -> Result<RawLunarDate, IztroError> {
    let err = |detail: &str| {
        IztroError::InvalidDate(format!(
            "invalid solar date '{year}-{month}-{day}': {detail}"
        ))
    };
    if !SUPPORTED_YEARS.contains(&year) {
        return Err(err(&year_range()));
    }
    if !(1..=12).contains(&month) {
        return Err(err("month must be within 1-12"));
    }
    let (m, d) = (i64::from(month), i64::from(day));
    if d < 1 || d > calendar::days_in_month(year, m) {
        return Err(err("day is out of range for that month"));
    }
    if calendar::is_reform_gap(year, m, d) {
        return Err(err(
            "1582-10-05 to 1582-10-14 do not exist (Julian to Gregorian reform)",
        ));
    }
    let l = calendar::lunar_date_of(year, m, d);
    Ok(RawLunarDate {
        lunar_year: l.year,
        lunar_month: l.month.unsigned_abs() as u32,
        lunar_day: l.day as u32,
        is_leap: l.month < 0,
    })
}

/// 农历日期对应的公历日期。
///
/// `is_leap` 只表示「这一天在闰月里」。排盘入口的 `fix_leap`（闰月十五之后视作次月）
/// 管的是星曜按哪个月安，不改变日期本身，这里没有对应参数。
///
/// 与 iztro 排盘入口（及 lunar-lite 的 `lunar2solar`）不同，该年没有这个闰月时不会退回同号的
/// 常规月，而是报错：退回会得到相差一个月、却看不出错的日期。
///
/// # Errors
/// 年不在 1-9999、月不在 1-12、该年没有这个月（`is_leap` 为真而该年不闰此月，或改历年缺此月），
/// 或日超出当月天数时返回 [`IztroError::InvalidDate`]。
pub fn lunar_to_solar(
    year: i64,
    month: u32,
    day: u32,
    is_leap: bool,
) -> Result<RawSolarDate, IztroError> {
    let leap = if is_leap { " (leap month)" } else { "" };
    let err = |detail: &str| {
        IztroError::InvalidDate(format!(
            "invalid lunar date '{year}-{month}-{day}'{leap}: {detail}"
        ))
    };
    if !SUPPORTED_YEARS.contains(&year) {
        return Err(err(&year_range()));
    }
    if !(1..=12).contains(&month) {
        return Err(err("month must be within 1-12"));
    }
    let signed_month = if is_leap {
        -i64::from(month)
    } else {
        i64::from(month)
    };
    let day_count = calendar::month_day_count(year, signed_month).ok_or_else(|| {
        err(if is_leap {
            "that lunar year has no such leap month"
        } else {
            "that lunar year has no such month"
        })
    })?;
    if day < 1 || i64::from(day) > day_count {
        return Err(err("day is out of range for that lunar month"));
    }
    let (y, m, d) = calendar::solar_date_of(year, signed_month, i64::from(day))
        .expect("月与日已按该农历年的月表校验");
    Ok(solar(y, m, d))
}

/// 农历 `year` 年的年干支（公历 4 年为甲子）。
///
/// 年干支按年号循环，与分界口径无关；分界口径决定的是这一柱从哪天起算：
/// - 正月初一分界（[`crate::YearDivide::Normal`]，默认）：农历 `year` 年正月初一至除夕；
/// - 立春分界（[`crate::YearDivide::Exact`]）：公历 `year` 年立春当日至次年立春前一日。
///
/// 两种口径下排盘的年柱都按上述区间取这一柱。
///
/// # Errors
/// `year` 不在 1-9999 时返回 [`IztroError::InvalidArgument`]。
pub fn lunar_year_pillar(year: i64) -> Result<(HeavenlyStem, EarthlyBranch), IztroError> {
    check_year(year)?;
    Ok(calendar::year_pillar(year))
}

fn check_year(year: i64) -> Result<(), IztroError> {
    if SUPPORTED_YEARS.contains(&year) {
        Ok(())
    } else {
        Err(IztroError::InvalidArgument(format!(
            "invalid lunar year {year}: expected {}-{}",
            SUPPORTED_YEARS.start(),
            SUPPORTED_YEARS.end()
        )))
    }
}

fn year_range() -> String {
    format!(
        "year must be within {}-{}",
        SUPPORTED_YEARS.start(),
        SUPPORTED_YEARS.end()
    )
}

fn solar(year: i64, month: i64, day: i64) -> RawSolarDate {
    RawSolarDate {
        solar_year: year,
        solar_month: month as u32,
        solar_day: day as u32,
    }
}

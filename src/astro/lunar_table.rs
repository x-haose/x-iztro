//! 农历月表读取口：对 lunar_rust 月界数据缺陷的修正视图。
//!
//! lunar_rust 1.0.1 推算农历 1602 年（明万历三十年）闰二月的合朔晚了一天：
//! 其月表把闰二月首日放在儒略日 2306262（公历 1602-3-25），真值为 2306261
//! （公历 1602-3-24）。由此该表内二月 31 天、闰二月 28 天——农历月只可能
//! 29 或 30 天，数据自相矛盾；直接后果是公历 1602-3-24 被换算成「二月 31 日」
//! （越界，取中文日名即 panic），1602-3-25 至 1602-4-21 的农历日序整体偏早一天。
//!
//! 真值口径经多个独立历表源交叉验证一致：lunar-typescript（lunar_rust 的移植
//! 上游，亦是 iztro 所用 lunar-lite 的底层）、寿星天文历移植 sxtwl、韩国天文
//! 研究院（KARI）历表均以 1602-3-24 为闰二月初一，二月 30 天、闰二月 29 天。
//! 逐月扫描 1582-9999 全部农历年，lunar_rust 月表在 29/30 天之外的条目仅此
//! 一处（见 tests/lunar_table.rs 的全域扫描）。
//!
//! 本模块是仓库内读取「农历月结构」（公历↔农历日期、月天数、中文月/日名）的
//! 唯一入口；日柱、时柱与节气类取值由儒略日与节气直接推出，不经月表，
//! Normal 分界的年柱虽经月表的农历年归属判定，但本缺陷位于年中（二月/闰二月），
//! 年归属不受影响——这些取值各处仍直接调用 lunar_rust。

use lunar_rust::lunar::{self, LunarRefHelper};
use lunar_rust::lunar_month::LunarMonthRefHelper;
use lunar_rust::lunar_year::{self, LunarYearRefHelper};
use lunar_rust::solar::{self, SolarRefHelper};

use crate::error::IztroError;

/// 缺陷所在的农历年。
const DEFECT_YEAR: i64 = 1602;
/// 缺陷月：1602 年闰二月（lunar_rust 以负数月号表示闰月）。
const DEFECT_MONTH: i64 = -2;
/// 1602 年闰二月初一的真实儒略日（公历 1602-3-24 正午）。
const DEFECT_MONTH_TRUE_FIRST_JD: f64 = 2306261.0;
/// lunar_rust 表内的 1602 年闰二月首日儒略日（晚一天的错值）。
const DEFECT_MONTH_TABLE_FIRST_JD: f64 = 2306262.0;
/// 修正后 1602 年二月的天数（lunar_rust 表内为 31）。
const DEFECT_PREV_MONTH_DAY_COUNT: i64 = 30;
/// 修正后 1602 年闰二月的天数（lunar_rust 表内为 28）。
const DEFECT_MONTH_DAY_COUNT: i64 = 29;

/// lunar_rust 月表当前是否带着该缺陷（闰二月首日为错值 2306262）。
///
/// [`ymd_of`] 对闰二月日序的 +1 重映射解释的是 lunar_rust 的输出标签，
/// 只在错误月界在场时才成立；依赖数据修好后该重映射自动停用。
/// 天数与真值儒略日（[`month_day_count`]、[`solar_date_of`]）本身就是
/// 真值，与表状态无关，无须探测。探测结果只算一次。
fn defect_present() -> bool {
    static PRESENT: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
    *PRESENT.get_or_init(|| {
        lunar_year::LunarYear::from_lunar_year(DEFECT_YEAR)
            .get_month(DEFECT_MONTH)
            .is_some_and(|m| m.get_first_julian_day() == DEFECT_MONTH_TABLE_FIRST_JD)
    })
}

/// 修正后的农历日期三元组（月为带符号月号，负数表示闰月）。
pub(crate) struct LunarYmd {
    /// 农历年
    pub year: i64,
    /// 带符号农历月：1..=12 为正常月，-1..=-12 为对应闰月
    pub month: i64,
    /// 农历日（1..=30）
    pub day: i64,
}

/// 从公历换算来的 [`lunar::LunarRef`] 中取出修正后的农历年月日。
///
/// 落在缺陷窗口（公历 1602-3-24 至 1602-4-21）的日期由 lunar_rust 的错误
/// 月界重映射回真值：「二月 31 日」即闰二月初一，「闰二月第 d 日」实为第
/// d+1 日；其余日期原样返回。
///
/// # Errors
/// 修正后月日仍越界（月号超出 ±1..=12 或日超出 1..=30）说明 lunar_rust
/// 月表存在未知的新缺陷，返回 [`IztroError::Internal`]；在此一并校验，
/// 下游对月/日名表的索引因而总是安全的。
pub(crate) fn ymd_of(lunar_ref: &lunar::LunarRef) -> Result<LunarYmd, IztroError> {
    let year = lunar_ref.get_year();
    let month = lunar_ref.get_month();
    let day = lunar_ref.get_day();
    let (month, day) = match (year, month) {
        // 二月不可能有第 31 天，出现即是被错误月界吞掉的闰二月初一，无须另探测
        (DEFECT_YEAR, 2) if day > DEFECT_PREV_MONTH_DAY_COUNT => {
            (DEFECT_MONTH, day - DEFECT_PREV_MONTH_DAY_COUNT)
        }
        (DEFECT_YEAR, DEFECT_MONTH) if defect_present() => (DEFECT_MONTH, day + 1),
        _ => (month, day),
    };
    if !(1..=12).contains(&month.abs()) || !(1..=30).contains(&day) {
        return Err(IztroError::Internal(format!(
            "lunar_rust returned an out-of-range lunar date: year {year} month {month} day {day}"
        )));
    }
    Ok(LunarYmd { year, month, day })
}

/// 农历某月的天数（大月 30 / 小月 29）；该年无此月时返回 `None`。
pub(crate) fn month_day_count(year: i64, signed_month: i64) -> Option<i64> {
    match (year, signed_month) {
        (DEFECT_YEAR, 2) => Some(DEFECT_PREV_MONTH_DAY_COUNT),
        (DEFECT_YEAR, DEFECT_MONTH) => Some(DEFECT_MONTH_DAY_COUNT),
        _ => lunar_year::LunarYear::from_lunar_year(year)
            .get_month(signed_month)
            .map(|m| m.get_day_count()),
    }
}

/// 农历日期转公历日期串（"YYYY-M-D"，不带前导零）。
///
/// 调用方须先经 [`month_day_count`] 校验日在当月范围内。缺陷月按真实首日
/// 儒略日作纯算术推算，其余月份走 lunar_rust 的常规换算。
pub(crate) fn solar_date_of(year: i64, signed_month: i64, day: i64) -> String {
    let solar_ref = if (year, signed_month) == (DEFECT_YEAR, DEFECT_MONTH) {
        solar::from_julian_day(DEFECT_MONTH_TRUE_FIRST_JD + (day - 1) as f64)
    } else {
        lunar::from_ymd(year, signed_month, day).get_solar()
    };
    format!(
        "{}-{}-{}",
        solar_ref.get_year(),
        solar_ref.get_month(),
        solar_ref.get_day(),
    )
}

/// 农历月的中文名（闰月带「闰」前缀，不含「月」字），与 lunar_rust 的
/// `get_month_in_chinese` 同表同格式。
pub(crate) fn month_in_chinese(month: u32, is_leap: bool) -> String {
    const MONTH: [&str; 13] = [
        "", "正", "二", "三", "四", "五", "六", "七", "八", "九", "十", "冬", "腊",
    ];
    format!(
        "{}{}",
        if is_leap { "闰" } else { "" },
        MONTH[month as usize]
    )
}

/// 农历日的中文名，与 lunar_rust 的 `get_day_in_chinese` 同表。
pub(crate) fn day_in_chinese(day: u32) -> &'static str {
    const DAY: [&str; 31] = [
        "", "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一",
        "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二",
        "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
    ];
    DAY[day as usize]
}

#[cfg(test)]
mod tests {
    use super::*;
    use lunar_rust::solar;

    /// 公历某日（正午）换算的修正后农历三元组。
    fn ymd(y: i64, m: i64, d: i64) -> (i64, i64, i64) {
        let l = lunar::from_solar(&solar::from_ymdhms(y, m, d, 12, 0, 0));
        let v = ymd_of(&l).unwrap();
        (v.year, v.month, v.day)
    }

    #[test]
    fn defect_window_remaps_to_true_lunar_dates() {
        assert_eq!(ymd(1602, 3, 23), (1602, 2, 30)); // 二月三十（窗口前一日不变）
        assert_eq!(ymd(1602, 3, 24), (1602, -2, 1)); // 闰二月初一（表内为二月 31 日）
        assert_eq!(ymd(1602, 3, 25), (1602, -2, 2)); // 表内日序偏早一天，+1 归位
        assert_eq!(ymd(1602, 4, 21), (1602, -2, 29)); // 闰二月廿九（窗口末日）
        assert_eq!(ymd(1602, 4, 22), (1602, 3, 1)); // 三月初一（窗口后一日不变）
    }

    #[test]
    fn normal_dates_pass_through() {
        assert_eq!(ymd(2000, 8, 16), (2000, 7, 17));
        assert_eq!(ymd(1602, 2, 22), (1602, 2, 1)); // 二月初一：缺陷月之前，月界正确
    }

    #[test]
    fn corrected_day_counts() {
        assert_eq!(month_day_count(1602, 2), Some(30));
        assert_eq!(month_day_count(1602, -2), Some(29));
        assert_eq!(month_day_count(1602, 3), Some(29));
        assert_eq!(month_day_count(2023, -2), Some(29)); // 2023 闰二月走 lunar_rust 常规路径
        assert_eq!(month_day_count(2000, -5), None); // 2000 年无闰五月
    }

    #[test]
    fn defect_month_maps_back_to_true_solar_dates() {
        assert_eq!(solar_date_of(1602, -2, 1), "1602-3-24");
        assert_eq!(solar_date_of(1602, -2, 29), "1602-4-21");
        assert_eq!(solar_date_of(1602, 2, 30), "1602-3-23"); // 二月首日正确，日序无需修正
        assert_eq!(solar_date_of(1602, 3, 1), "1602-4-22");
        assert_eq!(solar_date_of(2000, 7, 17), "2000-8-16");
    }

    #[test]
    fn chinese_names_match_lunar_rust_tables() {
        assert_eq!(month_in_chinese(1, false), "正");
        assert_eq!(month_in_chinese(2, true), "闰二");
        assert_eq!(month_in_chinese(12, false), "腊");
        assert_eq!(day_in_chinese(1), "初一");
        assert_eq!(day_in_chinese(30), "三十");
    }

    /// 全域月表自洽性：1582-9999 每个农历月经修正后的天数必须是 29 或 30。
    ///
    /// lunar_rust 表内在 29/30 之外的条目仅 1602 年二月（31）与闰二月（28）
    /// 一处，均由本模块归位；此扫描守住「不再出现同类月界错位」。
    /// 运行：`cargo test --release -- --ignored month_table`
    #[test]
    #[ignore = "全域逐月约 90 秒（release）"]
    fn month_table_day_counts_all_valid() {
        use lunar_rust::lunar_year::LunarYearRefHelper;

        for year in 1582..=9999i64 {
            for month_ref in lunar_year::LunarYear::from_lunar_year(year).get_months() {
                let corrected = month_day_count(month_ref.get_year(), month_ref.get_month())
                    .unwrap_or_else(|| {
                        panic!(
                            "month {} of lunar year {} vanished through the correction layer",
                            month_ref.get_month(),
                            month_ref.get_year()
                        )
                    });
                assert!(
                    corrected == 29 || corrected == 30,
                    "lunar year {} month {}: corrected day count {corrected}",
                    month_ref.get_year(),
                    month_ref.get_month(),
                );
            }
        }
    }
}

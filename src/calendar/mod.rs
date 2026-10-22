//! 历法：公历（儒略历 / 格里历）↔ 农历、节气、八字各柱。
//!
//! 仓库内一切农历与干支取值的唯一来源。逐函数移植自 lunar-typescript（iztro 经 lunar-lite 所用的
//! 历法库），取值与之逐日一致——由全量金标守护（公历 1-9999 年逐日，农历年月表与节气时刻逐年）。
//! 移植部分的版权与许可见同目录 `LICENSE-lunar-typescript`。

use std::sync::{Arc, OnceLock};

#[cfg(test)]
mod golden;
mod lunar;
mod lunar_year;
mod shouxing;
mod solar;
mod tables;

pub(crate) use lunar::{Lunar, LunarDate, Pillar, day_cycle};
pub(crate) use lunar_year::LunarYear;
pub(crate) use solar::{SolarTime, days_in_month, is_reform_gap, sign_index};

/// 缓存的农历年范围 `0..CACHED_YEARS`：排盘取值所及的全部农历年（公历 1 年年初属农历 0 年）。
const CACHED_YEARS: usize = 10000;

/// 农历年 → 月表与节气表的记忆化表，每年至多推算一次，各线程共享。
static LUNAR_YEARS: [OnceLock<Arc<LunarYear>>; CACHED_YEARS] =
    [const { OnceLock::new() }; CACHED_YEARS];

/// 农历 `year` 年的月表与节气表。
///
/// 推算月表约需百余微秒（节气表另算，见 [`LunarYear::jie_qi`]），远超排一张盘的其余开销，
/// 而它是年份的纯函数、定义域有限，故 0-9999 年按年记忆化；域外年份每次现算。
pub(crate) fn lunar_year(year: i64) -> Arc<LunarYear> {
    match usize::try_from(year).ok().and_then(|y| LUNAR_YEARS.get(y)) {
        Some(cell) => Arc::clone(cell.get_or_init(|| Arc::new(LunarYear::compute(year)))),
        None => Arc::new(LunarYear::compute(year)),
    }
}

/// 公历时刻的农历日期与不依赖节气的各柱。
pub(crate) fn lunar_of(time: &SolarTime) -> Lunar {
    lunar::lunar(&lunar_year(time.year), time)
}

/// 公历时刻按立春当日换年的年柱；`lunar_year` 为该时刻的农历年（[`lunar_of`] 的 `date.year`）。
pub(crate) fn year_by_li_chun(time: &SolarTime, lunar_year: i64) -> Pillar {
    lunar::year_by_li_chun(&self::lunar_year(time.year), time, lunar_year)
}

/// 公历时刻按交节时刻换月的月柱；`lunar_year` 同 [`year_by_li_chun`]。
pub(crate) fn month_exact(time: &SolarTime, lunar_year: i64) -> Pillar {
    lunar::month_exact(&self::lunar_year(time.year), time, lunar_year)
}

/// 公历日期所在的农历日期。
pub(crate) fn lunar_date_of(year: i64, month: i64, day: i64) -> LunarDate {
    lunar::lunar_date(&lunar_year(year), &SolarTime::new(year, month, day, 0, 0))
}

/// 农历日期对应的公历日期 (年, 月, 日)；该年无此月或日超出当月天数时返回 `None`。
pub(crate) fn solar_date_of(year: i64, signed_month: i64, day: i64) -> Option<(i64, i64, i64)> {
    lunar::solar_date(&lunar_year(year), signed_month, day)
}

/// 农历某月的天数（29 或 30）；该年无此月时返回 `None`。
pub(crate) fn month_day_count(year: i64, signed_month: i64) -> Option<i64> {
    lunar_year(year).month(signed_month).map(|m| m.day_count)
}

/// 农历某年的闰月月份（1-12）；无闰月返回 `None`。
pub(crate) fn leap_month(year: i64) -> Option<i64> {
    lunar_year(year).leap_month()
}

/// 农历日期的中文写法「二〇〇〇年七月十七」，闰月作「闰四月」；与 lunar-typescript 的
/// `Lunar.toString()` 同表同格式。
pub(crate) fn chinese_lunar_date(year: i64, month: u32, day: u32, is_leap: bool) -> String {
    const DIGIT: [char; 10] = ['〇', '一', '二', '三', '四', '五', '六', '七', '八', '九'];
    const MONTH: [&str; 13] = [
        "", "正", "二", "三", "四", "五", "六", "七", "八", "九", "十", "冬", "腊",
    ];
    const DAY: [&str; 31] = [
        "", "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十", "十一",
        "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一", "廿二",
        "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
    ];
    let year: String = year
        .to_string()
        .bytes()
        .map(|b| DIGIT[(b - b'0') as usize])
        .collect();
    format!(
        "{year}年{}{}月{}",
        if is_leap { "闰" } else { "" },
        MONTH[month as usize],
        DAY[day as usize]
    )
}

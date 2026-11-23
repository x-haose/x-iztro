//! 公历：儒略历与格里历的日期算术、儒略日互转、星座。
//!
//! 1582-10-04（儒略历）的次日是 1582-10-15（格里历），其间十天不存在。闰年规则按
//! lunar-typescript 的 `SolarUtil`：1600 年以前四年一闰，此后按格里历——两者在 1583-1599
//! 年间结论相同。移植自 lunar-typescript 的 `Solar` / `SolarUtil`。

/// 公历时刻。
///
/// 字段不做归一：[`SolarTime::from_julian_day`] 的秒数进位可让日数超出当月天数
/// （如「4 月 31 日」），与 lunar-typescript 相同；这样的值只用于先后比较。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SolarTime {
    /// 年（1-9999；节气表末尾可到 10000）
    pub year: i64,
    /// 月（1-12）
    pub month: i64,
    /// 日
    pub day: i64,
    /// 时（0-23）
    pub hour: i64,
    /// 分（0-59）
    pub minute: i64,
    /// 秒（0-59）
    pub second: i64,
}

/// 各月天数（平年）。
const DAYS_OF_MONTH: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

/// 闰年：1600 年以前四年一闰（儒略历），此后按格里历。
fn is_leap_year(year: i64) -> bool {
    if year < 1600 {
        return year % 4 == 0;
    }
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// 公历某月的名义天数（1582 年 10 月按 31 天计，其中 5-14 日不存在，见 [`is_reform_gap`]）。
pub(crate) fn days_in_month(year: i64, month: i64) -> i64 {
    if month == 2 && is_leap_year(year) {
        29
    } else {
        DAYS_OF_MONTH[(month - 1) as usize]
    }
}

/// 是否落在改历空缺 1582-10-05 至 1582-10-14。
pub(crate) fn is_reform_gap(year: i64, month: i64, day: i64) -> bool {
    year == 1582 && month == 10 && (5..=14).contains(&day)
}

/// 某月实有天数：1582 年 10 月为 21 天，其余同 [`days_in_month`]。
fn days_of_month(year: i64, month: i64) -> i64 {
    if year == 1582 && month == 10 {
        21
    } else {
        days_in_month(year, month)
    }
}

/// 某年实有天数：1582 年为 355 天。
fn days_of_year(year: i64) -> i64 {
    if year == 1582 {
        355
    } else if is_leap_year(year) {
        366
    } else {
        365
    }
}

/// 年内第几天（元旦为 1），1582-10-15 起扣除改历空缺的十天。
fn day_of_year(year: i64, month: i64, day: i64) -> i64 {
    let before: i64 = (1..month).map(|m| days_of_month(year, m)).sum();
    let day = if year == 1582 && month == 10 && day >= 15 {
        day - 10
    } else {
        day
    };
    before + day
}

/// 从 a 日到 b 日的天数（b 晚于 a 为正）。
fn days_between(a: (i64, i64, i64), b: (i64, i64, i64)) -> i64 {
    let ((ay, am, ad), (by, bm, bd)) = (a, b);
    if ay == by {
        day_of_year(by, bm, bd) - day_of_year(ay, am, ad)
    } else if ay > by {
        let mut days = days_of_year(by) - day_of_year(by, bm, bd);
        for y in by + 1..ay {
            days += days_of_year(y);
        }
        days += day_of_year(ay, am, ad);
        -days
    } else {
        let mut days = days_of_year(ay) - day_of_year(ay, am, ad);
        for y in ay + 1..by {
            days += days_of_year(y);
        }
        days += day_of_year(by, bm, bd);
        days
    }
}

/// 星座序号（白羊 = 0 … 双鱼 = 11），按月日的固定分界。
pub(crate) fn sign_index(month: i64, day: i64) -> usize {
    let y = month * 100 + day;
    match y {
        321..=419 => 0,
        420..=520 => 1,
        521..=621 => 2,
        622..=722 => 3,
        723..=822 => 4,
        823..=922 => 5,
        923..=1023 => 6,
        1024..=1122 => 7,
        1123..=1221 => 8,
        _ if y >= 1222 || y <= 119 => 9,
        _ if y <= 218 => 10,
        _ => 11,
    }
}

impl SolarTime {
    /// 某日某时某分（秒为 0）。
    pub(crate) fn new(year: i64, month: i64, day: i64, hour: i64, minute: i64) -> Self {
        SolarTime {
            year,
            month,
            day,
            hour,
            minute,
            second: 0,
        }
    }

    /// 儒略日（含时分秒的小数部分）；1582-10-15 起按格里历换算。
    pub(crate) fn julian_day(&self) -> f64 {
        let mut y = self.year as f64;
        let mut m = self.month as f64;
        let d = self.day as f64
            + ((self.second as f64 / 60.0 + self.minute as f64) / 60.0 + self.hour as f64) / 24.0;
        let gregorian = self.year * 372 + self.month * 31 + d.floor() as i64 >= 588829;
        if m <= 2.0 {
            m += 12.0;
            y -= 1.0;
        }
        let mut n = 0.0;
        if gregorian {
            n = (y / 100.0).floor();
            n = 2.0 - n + (n / 4.0).floor();
        }
        (365.25 * (y + 4716.0)).floor() + (30.6001 * (m + 1.0)).floor() + d + n - 1524.5
    }

    /// 儒略日对应的公历时刻，秒四舍五入；早于 2299161（1582-10-15）按儒略历。
    pub(crate) fn from_julian_day(julian_day: f64) -> Self {
        let mut d = (julian_day + 0.5).floor();
        let mut f = julian_day + 0.5 - d;
        if d >= 2299161.0 {
            let c = ((d - 1867216.25) / 36524.25).floor();
            d += 1.0 + c - (c / 4.0).floor();
        }
        d += 1524.0;
        let mut year = ((d - 122.1) / 365.25).floor();
        d -= (365.25 * year).floor();
        let mut month = (d / 30.601).floor();
        d -= (30.601 * month).floor();
        let mut day = d;
        if month > 13.0 {
            month -= 13.0;
            year -= 4715.0;
        } else {
            month -= 1.0;
            year -= 4716.0;
        }
        f *= 24.0;
        let mut hour = f.floor();
        f -= hour;
        f *= 60.0;
        let mut minute = f.floor();
        f -= minute;
        f *= 60.0;
        let mut second = f.round();
        if second > 59.0 {
            second -= 60.0;
            minute += 1.0;
        }
        if minute > 59.0 {
            minute -= 60.0;
            hour += 1.0;
        }
        if hour > 23.0 {
            hour -= 24.0;
            day += 1.0;
        }
        SolarTime {
            year: year as i64,
            month: month as i64,
            day: day as i64,
            hour: hour as i64,
            minute: minute as i64,
            second: second as i64,
        }
    }

    /// 从 `earlier` 那天到本日的天数（只看日期）。
    pub(crate) fn days_since(&self, earlier: &SolarTime) -> i64 {
        days_between(
            (earlier.year, earlier.month, earlier.day),
            (self.year, self.month, self.day),
        )
    }

    /// 日期的比较键 `YYYY-MM-DD`（年至少 4 位）。先后比较按此串的字典序，与 lunar-typescript 一致：
    /// 年份 10000 是 5 位，排在 `9999-…` 之前。
    pub(crate) fn ymd_key(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    /// 时刻的比较键 `YYYY-MM-DD HH:MM:SS`，规则同 [`SolarTime::ymd_key`]。
    pub(crate) fn ymdhms_key(&self) -> String {
        format!(
            "{} {:02}:{:02}:{:02}",
            self.ymd_key(),
            self.hour,
            self.minute,
            self.second
        )
    }
}

//! 日期入参的写法：公历、农历与运限目标日期共用一套文法（与 iztro 的 `validateDate` 一致）。
//!
//! 文法为 `Y{sep}M{sep}D`，可再跟一个空格与 `H:M` 或 `H:M:S`：`sep` 取 `-` `/` `.` 之一且
//! 前后一致，年 1–4 位数字、月日与时分秒各 1–2 位数字；时不超过 23、分秒不超过 59。
//! 本模块只管文法，年份范围与日期是否真实存在由公历 / 农历各自校验。

/// 按文法拆出的日期入参。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct DateInput {
    /// 年（未校验范围）
    pub year: i64,
    /// 月（未校验范围）
    pub month: i64,
    /// 日（未校验范围）
    pub day: i64,
    /// 时间部分的小时（0–23）；入参不带时间时为 `None`
    pub hour: Option<u32>,
}

/// 由 1 到 `max_len` 位 ASCII 数字组成的片段转为数值；不符时返回 `None`。
fn digits(part: &str, max_len: usize) -> Option<u32> {
    if part.is_empty() || part.len() > max_len || !part.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    part.parse().ok()
}

/// 解析日期串；文法不符时返回给人读的原因（英文，写进报错正文）。
pub(crate) fn parse(input: &str) -> Result<DateInput, &'static str> {
    const GRAMMAR: &str = "expected 'YYYY-M-D' ('-', '/' or '.' as the separator), optionally followed by ' HH:mm' or ' HH:mm:ss'";
    let (date, time) = match input.split_once(' ') {
        Some((date, time)) => (date, Some(time)),
        None => (input, None),
    };
    let sep = date
        .chars()
        .find(|c| !c.is_ascii_digit())
        .filter(|c| matches!(c, '-' | '/' | '.'))
        .ok_or(GRAMMAR)?;
    let parts: Vec<&str> = date.split(sep).collect();
    let [year, month, day] = parts.as_slice() else {
        return Err(GRAMMAR);
    };
    let year = digits(year, 4).ok_or(GRAMMAR)?;
    let month = digits(month, 2).ok_or(GRAMMAR)?;
    let day = digits(day, 2).ok_or(GRAMMAR)?;

    let hour = match time {
        None => None,
        Some(time) => {
            let fields: Vec<&str> = time.split(':').collect();
            let (hour, minute, second) = match fields.as_slice() {
                [h, m] => (digits(h, 2), digits(m, 2), Some(0)),
                [h, m, s] => (digits(h, 2), digits(m, 2), digits(s, 2)),
                _ => return Err(GRAMMAR),
            };
            let (hour, minute, second) = (
                hour.ok_or(GRAMMAR)?,
                minute.ok_or(GRAMMAR)?,
                second.ok_or(GRAMMAR)?,
            );
            if hour > 23 || minute > 59 || second > 59 {
                return Err("time must be within 00:00:00-23:59:59");
            }
            Some(hour)
        }
    };
    Ok(DateInput {
        year: i64::from(year),
        month: i64::from(month),
        day: i64::from(day),
        hour,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: &str) -> (i64, i64, i64, Option<u32>) {
        let d = parse(s).unwrap_or_else(|e| panic!("{s}: {e}"));
        (d.year, d.month, d.day, d.hour)
    }

    #[test]
    fn accepts_the_three_separators_padding_and_time() {
        assert_eq!(ok("2000-8-16"), (2000, 8, 16, None));
        assert_eq!(ok("2000/08/16"), (2000, 8, 16, None));
        assert_eq!(ok("2000.8.16 13:20"), (2000, 8, 16, Some(13)));
        assert_eq!(ok("999-3-1 23:59:59"), (999, 3, 1, Some(23)));
        assert_eq!(ok("2000-8-16 0:0"), (2000, 8, 16, Some(0)));
    }

    #[test]
    fn rejects_what_iztro_rejects() {
        for s in [
            "2000-8/16",
            "2000-8-16T10:30",
            "2000-8-16 24:00",
            "2000-8-16 10:60",
            "2000-8-16  10:30",
            "2000-8-16 10",
            "10000-1-1",
            "2000-123-1",
            "+2000-1-1",
            "2000-8",
            "",
        ] {
            assert!(parse(s).is_err(), "{s:?} 应被拒绝");
        }
    }
}

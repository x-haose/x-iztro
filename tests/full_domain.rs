//! 全域扫描：公历 1-9999 年每一天都能排盘。
//!
//! 历法取值由 calendar.csv 逐日对 JS 全量核对；本扫描守的是排盘流程在任何合法日期上
//! 不报内部错误、不 panic——尤其是「农历日期所在月在该农历年的月表里查得到天数」这条
//! 跨表不变量（农历日期取自公历年的月表，月天数取自农历年的月表，与 iztro 同源）。
//! 运行：`cargo test --release --test full_domain -- --ignored`

use x_iztro::{Config, Gender, Language, by_solar};

/// 公历某月天数：1600 年以前四年一闰（儒略历），此后按格里历，与 iztro 的日期校验同一规则。
fn days_in_month(year: i64, month: i64) -> i64 {
    let leap = if year < 1600 {
        year % 4 == 0
    } else {
        (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
    };
    match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    }
}

/// 每一天 × 时辰 {0, 12}（晚子时取次日日柱），任何 Err 或 panic 都使测试失败。
#[test]
#[ignore = "全域约 730 万例，release 下数分钟"]
fn every_day_charts_ok() {
    let years: Vec<i64> = (1..=9999).collect();
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get());
    let chunk = years.len().div_ceil(workers);
    std::thread::scope(|s| {
        for part in years.chunks(chunk) {
            s.spawn(move || {
                for &year in part {
                    for month in 1..=12 {
                        for day in 1..=days_in_month(year, month) {
                            if year == 1582 && month == 10 && (5..=14).contains(&day) {
                                continue;
                            }
                            let date = format!("{year}-{month}-{day}");
                            for ti in [0u8, 12] {
                                by_solar(
                                    &date,
                                    ti,
                                    Gender::Male,
                                    true,
                                    Language::ZhCN,
                                    Config::default(),
                                )
                                .unwrap_or_else(|e| panic!("by_solar({date}, {ti}): {e:?}"));
                            }
                        }
                    }
                }
            });
        }
    });
}

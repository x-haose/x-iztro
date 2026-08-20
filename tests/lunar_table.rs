//! 农历换算修正层（src/astro/lunar_table.rs）的入口级测试。
//!
//! lunar_rust 1.0.1 把农历 1602 年（明万历三十年）闰二月的合朔算晚一天，
//! 未修正时公历 1602-3-24 被换算成「二月 31 日」（取中文日名即 panic，全天
//! 13 个时辰排盘皆挂），1602-3-25 至 1602-4-21 的农历日序整体偏早一天。
//! 此处按修正后的真值（1602-3-24 = 闰二月初一，与 lunar-typescript、sxtwl、
//! KARI 历表交叉一致）断言排盘入口的行为；星耀层面的对照见 golden_1602。
//!
//! 另含 `#[ignore]` 的全域逐日扫描（by_solar 全域不 panic 不报错）；
//! 月表天数的全域自洽扫描在 src/astro/lunar_table.rs 的单元测试里。

use x_iztro::{Config, Gender, Language, LeapMonth, by_lunar, by_solar, get_horoscope};

/// 默认参数排盘（男，fix_leap=true，zh-CN）。
fn chart(solar_date: &str, time_index: u8) -> x_iztro::models::astrolabe::Astrolabe {
    by_solar(
        solar_date,
        time_index,
        Gender::Male,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap_or_else(|e| panic!("by_solar({solar_date}, {time_index}) failed: {e:?}"))
}

/// 公历某月天数（格里历闰年规则）。
fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
    }
}

/// 1602-3-24（闰二月初一）全部 13 个时辰均可排盘，农历字段为真值。
#[test]
fn solar_1602_3_24_all_time_indices() {
    for ti in 0..=12u8 {
        for gender in [Gender::Male, Gender::Female] {
            for fix_leap in [true, false] {
                let a = by_solar(
                    "1602-3-24",
                    ti,
                    gender,
                    fix_leap,
                    Language::ZhCN,
                    Config::default(),
                )
                .unwrap_or_else(|e| panic!("ti={ti} gender={gender:?} fix_leap={fix_leap}: {e:?}"));
                assert_eq!(a.lunar_date, "一六〇二年闰二月初一", "ti={ti}");
                let l = &a.raw_dates.lunar_date;
                assert_eq!(
                    (l.lunar_year, l.lunar_month, l.lunar_day, l.is_leap),
                    (1602, 2, 1, true),
                    "ti={ti}"
                );
            }
        }
    }
}

/// 窗口内外逐类抽点：农历日期串与原始农历字段按真值换算。
#[test]
fn window_boundaries_convert_to_true_lunar_dates() {
    // (公历日期, 农历串, (月, 日, 闰否))
    let cases = [
        ("1602-3-23", "一六〇二年二月三十", (2, 30, false)), // 窗口前一日：二月最后一天
        ("1602-3-25", "一六〇二年闰二月初二", (2, 2, true)), // 未修正时错报闰二月初一
        ("1602-4-8", "一六〇二年闰二月十六", (2, 16, true)), // 未修正时错报十五，下半月修正随之错
        ("1602-4-21", "一六〇二年闰二月廿九", (2, 29, true)), // 窗口末日：未修正时该日不存在于闰二月
        ("1602-4-22", "一六〇二年三月初一", (3, 1, false)),   // 窗口后一日：月界恢复一致
    ];
    for (solar, lunar_str, (month, day, is_leap)) in cases {
        let a = chart(solar, 0);
        assert_eq!(a.lunar_date, lunar_str, "{solar}");
        let l = &a.raw_dates.lunar_date;
        assert_eq!(
            (l.lunar_year, l.lunar_month, l.lunar_day, l.is_leap),
            (1602, month, day, is_leap),
            "{solar}"
        );
    }
}

/// 农历入口：闰二月全月按真值映射公历，边界日数按 30/29 校验。
#[test]
fn lunar_1602_leap_month_roundtrips() {
    let by = |date: &str, leap: LeapMonth| {
        by_lunar(
            date,
            0,
            Gender::Male,
            leap,
            Language::ZhCN,
            Config::default(),
        )
    };

    // 闰二月初一/廿九 → 公历窗口两端；非闰二月三十 → 窗口前一日
    assert_eq!(
        by("1602-2-1", LeapMonth::Leap).unwrap().solar_date,
        "1602-3-24"
    );
    assert_eq!(
        by("1602-2-29", LeapMonth::Leap).unwrap().solar_date,
        "1602-4-21"
    );
    assert_eq!(
        by("1602-2-30", LeapMonth::NotLeap).unwrap().solar_date,
        "1602-3-23"
    );

    // 修正后的日数上限：二月 30 天（lunar_rust 表内错为 31）、闰二月 29 天（表内错为 28）
    assert!(by("1602-2-31", LeapMonth::NotLeap).is_err());
    assert!(by("1602-2-30", LeapMonth::Leap).is_err());

    // 农历入口与公历入口同盘
    let via_lunar = by("1602-2-15", LeapMonth::Leap).unwrap();
    let via_solar = chart(&via_lunar.solar_date, 0);
    assert_eq!(via_lunar.lunar_date, via_solar.lunar_date);
    assert_eq!(via_lunar.solar_date, "1602-4-7");
}

/// 运限目标日期落在窗口内时正常计算且农历串为真值。
#[test]
fn horoscope_target_inside_window() {
    let birth = chart("1583-1-15", 2);
    let h = get_horoscope(&birth, "1602-3-24", 0, Language::ZhCN).unwrap();
    assert_eq!(h.lunar_date, "一六〇二年闰二月初一");
    let h2 = get_horoscope(&birth, "1602-4-21", 12, Language::ZhCN).unwrap();
    assert_eq!(h2.lunar_date, "一六〇二年闰二月廿九");

    // 出生日期本身落在窗口内
    let birth_in_window = chart("1602-4-1", 6);
    assert!(get_horoscope(&birth_in_window, "1650-6-1", 0, Language::ZhCN).is_ok());
}

/// 中州派地盘/人盘重排经过修正后的月天数（闰二月 29 天）。
#[test]
fn rearranged_charts_inside_window() {
    use x_iztro::AstroType;
    for astro_type in [AstroType::Earth, AstroType::Human] {
        let a = by_solar(
            "1602-4-21",
            12,
            Gender::Female,
            true,
            Language::ZhCN,
            Config {
                astro_type,
                ..Config::default()
            },
        )
        .unwrap();
        assert_eq!(a.palaces.len(), 12);
    }
}

/// C FFI 路线（Go/wasm 绑定的调用链）同样返回修正后的成功 JSON 而非错误或 trap。
#[test]
fn ffi_route_returns_chart_for_1602_3_24() {
    use std::ffi::{CStr, CString};
    use x_iztro::ffi::{iztro_by_solar, iztro_free_string};

    let date = CString::new("1602-3-24").unwrap();
    let gender = CString::new("male").unwrap();
    let language = CString::new("zh-CN").unwrap();
    let ptr = unsafe {
        iztro_by_solar(
            date.as_ptr(),
            0,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            std::ptr::null(),
        )
    };
    assert!(!ptr.is_null());
    let json = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
    unsafe { iztro_free_string(ptr) };

    let v: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert!(v.get("error").is_none(), "unexpected error: {json}");
    assert_eq!(v["lunarDate"], "一六〇二年闰二月初一");
    assert_eq!(v["rawDates"]["lunarDate"]["isLeap"], true);
}

/// 全域扫描：1583-9999 每一天 × 时辰 {0, 12}，by_solar 必须返回 Ok。
///
/// 按年切块多线程跑满 CPU；任何 panic 或 Err 都会使测试失败。
/// 运行：`cargo test --release --test lunar_table -- --ignored full_domain`
#[test]
#[ignore = "全域约 614 万例，release 下数分钟"]
fn full_domain_every_day_charts_ok() {
    let years: Vec<i64> = (1583..=9999).collect();
    let workers = std::thread::available_parallelism().map_or(4, |n| n.get());
    let chunk = years.len().div_ceil(workers);
    std::thread::scope(|s| {
        for part in years.chunks(chunk) {
            s.spawn(move || {
                for &year in part {
                    for month in 1..=12i64 {
                        for day in 1..=days_in_month(year, month) {
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

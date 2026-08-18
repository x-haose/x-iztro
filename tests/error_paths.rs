//! Rust 原生入口的错误路径：非法输入必须返回 `Err(IztroError::…)` 而非 panic。
//!
//! 这条防线在 wasm 目标上是硬要求——wasm 下 panic 即 trap，且每次 trap 都会
//! 永久损耗模块实例的栈空间，绑定层的 catch_unwind 在 wasm 上不生效。
//! 因此校验必须落在核心入口里，本文件直接对着核心入口测。

use x_iztro::data::types::*;
use x_iztro::error::IztroError;
use x_iztro::{by_lunar, by_solar, get_horoscope};

const LANG: Language = Language::ZhCN;

fn solar(date: &str, time_index: u8) -> Result<x_iztro::Astrolabe, IztroError> {
    by_solar(
        date,
        time_index,
        Gender::Female,
        true,
        LANG,
        Config::default(),
    )
}

fn lunar(date: &str, time_index: u8, is_leap: bool) -> Result<x_iztro::Astrolabe, IztroError> {
    by_lunar(
        date,
        time_index,
        Gender::Female,
        is_leap,
        true,
        LANG,
        Config::default(),
    )
}

#[test]
fn by_solar_rejects_malformed_dates() {
    for date in [
        "",
        "not-a-date",
        "2000/8/16",
        "2000-8",
        "2000-8-16-1",
        "两千-8-16",
        "2000-8-x",
    ] {
        assert!(
            matches!(solar(date, 2), Err(IztroError::InvalidDate(_))),
            "by_solar({date:?}) should reject the date format"
        );
    }
}

#[test]
fn by_solar_rejects_nonexistent_dates() {
    for date in [
        "2000-13-1",
        "2000-0-1",
        "2000-8-0",
        "2000-8-32",
        "2001-2-29",
    ] {
        assert!(
            matches!(solar(date, 2), Err(IztroError::InvalidDate(_))),
            "by_solar({date:?}) should reject the nonexistent date"
        );
    }
    // 2000 是闰年，2-29 存在
    assert!(solar("2000-2-29", 2).is_ok());
}

#[test]
fn by_solar_enforces_supported_year_range() {
    for date in ["1582-12-31", "1000-1-1", "10000-1-1"] {
        assert!(
            matches!(solar(date, 2), Err(IztroError::InvalidDate(_))),
            "by_solar({date:?}) should be out of the supported range"
        );
    }
    assert!(solar("1583-1-1", 0).is_ok(), "1583-1-1 is the lower bound");
    assert!(
        solar("9999-12-31", 12).is_ok(),
        "9999-12-31 is the upper bound"
    );
}

#[test]
fn by_solar_rejects_out_of_range_time_index() {
    for ti in [13u8, 24, 255] {
        assert!(
            matches!(solar("2000-8-16", ti).err(), Some(IztroError::InvalidTimeIndex(t)) if t == ti),
            "by_solar time_index={ti} should be rejected"
        );
    }
    for ti in 0..=12u8 {
        assert!(solar("2000-8-16", ti).is_ok(), "time_index={ti} is valid");
    }
}

#[test]
fn by_lunar_rejects_invalid_dates() {
    for date in [
        "",
        "garbage",
        "2000-13-1",
        "2000-7-31",
        "2000-7-0",
        "2000-0-1",
    ] {
        assert!(
            matches!(lunar(date, 2, false), Err(IztroError::InvalidDate(_))),
            "by_lunar({date:?}) should be rejected"
        );
    }
    assert!(lunar("2000-7-17", 2, false).is_ok());
}

#[test]
fn by_lunar_rejects_out_of_range_year_and_time_index() {
    assert!(matches!(
        lunar("1582-1-1", 2, false),
        Err(IztroError::InvalidDate(_))
    ));
    assert_eq!(
        lunar("2000-7-17", 13, false).err(),
        Some(IztroError::InvalidTimeIndex(13))
    );
}

#[test]
fn by_lunar_handles_nonexistent_leap_month() {
    // 2000 年闰四月；对非闰月标 is_leap_month 不得 panic，
    // 结果或为 Err，或按 iztro 语义退回该月的常规排盘。
    let result = lunar("2000-7-17", 2, true);
    assert!(
        matches!(result, Ok(_) | Err(IztroError::InvalidDate(_))),
        "nonexistent leap month must not panic: {:?}",
        result.err()
    );
    // 真实存在的闰月正常排盘
    assert!(lunar("2000-4-15", 2, true).is_ok());
}

#[test]
fn get_horoscope_rejects_invalid_targets() {
    let astrolabe = solar("2000-8-16", 2).unwrap();

    for date in ["", "garbage", "2025/1/1", "2025-2-30", "2025-13-1"] {
        assert!(
            matches!(
                get_horoscope(&astrolabe, date, 0, LANG),
                Err(IztroError::InvalidDate(_))
            ),
            "get_horoscope({date:?}) should be rejected"
        );
    }

    for date in ["1582-12-31", "10000-1-1"] {
        assert!(
            matches!(
                get_horoscope(&astrolabe, date, 0, LANG),
                Err(IztroError::InvalidDate(_))
            ),
            "get_horoscope({date:?}) should be out of the supported range"
        );
    }

    assert_eq!(
        get_horoscope(&astrolabe, "2025-1-1", 13, LANG).err(),
        Some(IztroError::InvalidTimeIndex(13))
    );
    assert!(get_horoscope(&astrolabe, "2025-1-1", 12, LANG).is_ok());
}

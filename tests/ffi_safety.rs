//! C FFI 边界安全测试：任何非法输入都必须返回 `{"error":...}` JSON，
//! 绝不允许 panic 越过 C ABI（跨 ABI unwind 是未定义行为）。

use std::ffi::{CStr, CString, c_char};
use std::ptr;

use x_iztro::ffi::{iztro_by_lunar, iztro_by_solar, iztro_free_string, iztro_get_horoscope};

/// 调用返回后取出内容并释放缓冲，同时断言内容是合法 JSON
/// （错误消息含引号、反斜杠、控制字符时转义必须完备）。
fn take(ptr: *mut c_char) -> String {
    assert!(!ptr.is_null());
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
    unsafe { iztro_free_string(ptr) };
    serde_json::from_str::<serde_json::Value>(&s)
        .unwrap_or_else(|e| panic!("response must be valid JSON ({e}): {s}"));
    s
}

fn c(s: &str) -> CString {
    CString::new(s).unwrap()
}

/// 阳历排盘的便捷调用：date/gender/language 为字符串，config 传 NULL。
fn by_solar(date: &str, time_index: u8, gender: &str, language: &str) -> String {
    let date = c(date);
    let gender = c(gender);
    let language = c(language);
    take(unsafe {
        iztro_by_solar(
            date.as_ptr(),
            time_index,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
        )
    })
}

fn is_error(json: &str) -> bool {
    json.starts_with(r#"{"error":"#)
}

#[test]
fn valid_input_returns_chart_json() {
    let json = by_solar("2000-8-16", 2, "female", "zh-CN");
    assert!(!is_error(&json), "unexpected error: {json}");
    assert!(json.contains(r#""palaces":"#));
}

#[test]
fn null_required_pointer_returns_error() {
    let gender = c("male");
    let language = c("zh-CN");
    let json = take(unsafe {
        iztro_by_solar(
            ptr::null(),
            0,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn invalid_gender_returns_error() {
    let json = by_solar("2000-8-16", 2, "unknown", "zh-CN");
    assert!(is_error(&json), "expected error, got: {json}");
    assert!(json.contains("gender"));
}

#[test]
fn invalid_language_returns_error() {
    let json = by_solar("2000-8-16", 2, "male", "xx_yy");
    assert!(is_error(&json), "expected error, got: {json}");
    assert!(json.contains("language"));
}

#[test]
fn garbage_solar_date_returns_error_not_panic() {
    let json = by_solar("not-a-date", 2, "male", "zh-CN");
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn out_of_range_time_index_returns_error_not_panic() {
    let json = by_solar("2000-8-16", 13, "male", "zh-CN");
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn invalid_config_json_returns_error() {
    let date = c("2000-8-16");
    let gender = c("male");
    let language = c("zh-CN");
    let config = c(r#"{"algorithm":"nonexistent"}"#);
    let json = take(unsafe {
        iztro_by_solar(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            config.as_ptr(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn garbage_lunar_date_returns_error_not_panic() {
    let date = c("9999-99-99");
    let gender = c("male");
    let language = c("zh-CN");
    let json = take(unsafe {
        iztro_by_lunar(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            false,
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn garbage_horoscope_target_returns_error_not_panic() {
    let date = c("2000-8-16");
    let gender = c("male");
    let language = c("zh-CN");
    let target = c("garbage");
    let json = take(unsafe {
        iztro_get_horoscope(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
            target.as_ptr(),
            0,
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn valid_horoscope_returns_json() {
    let date = c("2000-8-16");
    let gender = c("female");
    let language = c("zh-CN");
    let target = c("2024-6-1");
    let json = take(unsafe {
        iztro_get_horoscope(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
            target.as_ptr(),
            0,
        )
    });
    assert!(!is_error(&json), "unexpected error: {json}");
    assert!(json.contains(r#""decadal":"#));
}

#[test]
fn error_message_escapes_backslash_and_control_chars() {
    for bad in ["2000\\8", "2000\n8", "2000-\"8\"-16", "2000-8-16\t"] {
        let json = by_solar(bad, 2, "male", "zh-CN");
        assert!(is_error(&json), "expected error for {bad:?}, got: {json}");
    }
    let json = by_solar("2000-8-16", 2, "a\"x", "zh-CN");
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn lunar_out_of_range_time_index_returns_error() {
    let date = c("2000-7-16");
    let gender = c("male");
    let language = c("zh-CN");
    let json = take(unsafe {
        iztro_by_lunar(
            date.as_ptr(),
            13,
            gender.as_ptr(),
            false,
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn horoscope_out_of_range_target_time_index_returns_error() {
    let date = c("2000-8-16");
    let gender = c("male");
    let language = c("zh-CN");
    let target = c("2024-6-1");
    let json = take(unsafe {
        iztro_get_horoscope(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
            target.as_ptr(),
            13,
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn invalid_utf8_input_returns_error() {
    let bad = CString::new([0xE4u8, 0xB8, 0xAD, 0xFF, 0xFE].to_vec()).unwrap();
    let gender = c("male");
    let language = c("zh-CN");
    let json = take(unsafe {
        iztro_by_solar(
            bad.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
    assert!(json.contains("UTF-8"), "expected UTF-8 error, got: {json}");
}

#[test]
fn lunar_leap_month_and_day_bounds() {
    let gender = c("male");
    let language = c("zh-CN");
    // 2000 年无闰七月：is_leap_month 不生效，正常排盘
    let date = c("2000-7-16");
    let json = take(unsafe {
        iztro_by_lunar(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            true,
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(!is_error(&json), "unexpected error: {json}");
    // 农历月最多 30 天：31 必然越界
    let date = c("2000-7-31");
    let json = take(unsafe {
        iztro_by_lunar(
            date.as_ptr(),
            2,
            gender.as_ptr(),
            false,
            true,
            language.as_ptr(),
            ptr::null(),
        )
    });
    assert!(is_error(&json), "expected error, got: {json}");
}

#[test]
fn date_out_of_supported_years_returns_error() {
    for bad in ["1582-10-10", "10000-1-1", "0-1-1", "-100-1-1"] {
        let json = by_solar(bad, 2, "male", "zh-CN");
        assert!(is_error(&json), "expected error for {bad}, got: {json}");
    }
    let json = by_solar("1583-1-1", 2, "male", "zh-CN");
    assert!(!is_error(&json), "1583 should be supported: {json}");
    let json = by_solar("9999-12-31", 2, "male", "zh-CN");
    assert!(!is_error(&json), "9999 should be supported: {json}");
}

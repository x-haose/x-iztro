//! C FFI bindings for rs-iztro.
//!
//! These functions expose the core astrolabe functionality via a C-compatible ABI,
//! allowing the library to be called from Go, C, and any other C-compatible language.

use std::ffi::{c_char, CStr, CString};

use crate::data::types::{Gender, Language};
use crate::dto::parse_config_json;

/// Parse a gender string, returning Ok or an error message.
fn parse_gender(s: &str) -> Result<Gender, String> {
    match s.to_lowercase().as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(format!(
            "Invalid gender '{}'. Expected 'male' or 'female'.",
            s
        )),
    }
}

/// Parse a language string, returning Ok or an error message.
fn parse_language(s: &str) -> Result<Language, String> {
    match s.to_lowercase().as_str() {
        "zh_cn" => Ok(Language::ZhCN),
        "zh_tw" => Ok(Language::ZhTW),
        "en_us" => Ok(Language::EnUS),
        "ja_jp" => Ok(Language::JaJP),
        "ko_kr" => Ok(Language::KoKR),
        "vi_vn" => Ok(Language::ViVN),
        _ => Err(format!(
            "Invalid language '{}'. Expected one of: zh_cn, zh_tw, en_us, ja_jp, ko_kr, vi_vn.",
            s
        )),
    }
}

/// Helper: convert a C string pointer to a Rust &str.
/// Returns Err with a message if the pointer is null or not valid UTF-8.
unsafe fn cstr_to_str<'a>(ptr: *const c_char, param_name: &str) -> Result<&'a str, String> {
    if ptr.is_null() {
        return Err(format!("'{}' is null", param_name));
    }
    unsafe { CStr::from_ptr(ptr) }
        .to_str()
        .map_err(|e| format!("'{}' is not valid UTF-8: {}", param_name, e))
}

/// Helper: convert an optional C string pointer (NULL allowed) to Option<&str>.
unsafe fn cstr_to_opt_str<'a>(ptr: *const c_char, param_name: &str) -> Result<Option<&'a str>, String> {
    if ptr.is_null() {
        return Ok(None);
    }
    unsafe { cstr_to_str(ptr, param_name) }.map(Some)
}

/// Helper: return a JSON error string as a C string.
fn error_json(msg: &str) -> *mut c_char {
    let json = format!(r#"{{"error":"{}"}}"#, msg.replace('\"', "\\\""));
    CString::new(json).unwrap().into_raw()
}

/// Helper: return a JSON result string as a C string.
fn ok_json(json: String) -> *mut c_char {
    CString::new(json).unwrap().into_raw()
}

/// Generate an astrolabe from a solar (Gregorian) date and return it as a JSON string.
///
/// # Parameters
/// - `solar_date`: Date string, e.g. "2000-8-16"
/// - `time_index`: Time index (0-12)
/// - `gender`: "male" or "female"
/// - `fix_leap`: Whether to fix leap month
/// - `language`: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
/// - `config_json`: NULL/empty for defaults, or a partial-config JSON such as
///   `{"algorithm":"zhongzhou","yearDivide":"exact"}` (keys: yearDivide,
///   horoscopeDivide, ageDivide, dayDivide, algorithm)
///
/// # Returns
/// A heap-allocated JSON C string. The caller must free it with `iztro_free_string`.
/// On error, returns a JSON string like `{"error": "message"}`.
///
/// # Safety
/// All pointer parameters except `config_json` must be valid NUL-terminated
/// C strings (or null, which yields an error JSON); `config_json` may be NULL.
/// The returned pointer must be released with `iztro_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_by_solar(
    solar_date: *const c_char,
    time_index: u8,
    gender: *const c_char,
    fix_leap: bool,
    language: *const c_char,
    config_json: *const c_char,
) -> *mut c_char {
    let result = (|| -> Result<String, String> {
        let solar_date = unsafe { cstr_to_str(solar_date, "solar_date")? };
        let gender_str = unsafe { cstr_to_str(gender, "gender")? };
        let language_str = unsafe { cstr_to_str(language, "language")? };
        let config_str = unsafe { cstr_to_opt_str(config_json, "config_json")? };

        let gender = parse_gender(gender_str)?;
        let language = parse_language(language_str)?;
        let config = parse_config_json(config_str)?;

        Ok(crate::by_solar_json(
            solar_date, time_index, gender, fix_leap, language, config,
        ))
    })();

    match result {
        Ok(json) => ok_json(json),
        Err(msg) => error_json(&msg),
    }
}

/// Generate an astrolabe from a lunar (Chinese calendar) date and return it as a JSON string.
///
/// # Parameters
/// - `lunar_date`: Lunar date string, e.g. "2000-7-16"
/// - `time_index`: Time index (0-12)
/// - `gender`: "male" or "female"
/// - `is_leap_month`: Whether the lunar month is a leap month
/// - `fix_leap`: Whether to fix leap month
/// - `language`: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
/// - `config_json`: NULL/empty for defaults, or a partial-config JSON
///
/// # Returns
/// A heap-allocated JSON C string. The caller must free it with `iztro_free_string`.
/// On error, returns a JSON string like `{"error": "message"}`.
///
/// # Safety
/// All pointer parameters except `config_json` must be valid NUL-terminated
/// C strings (or null, which yields an error JSON); `config_json` may be NULL.
/// The returned pointer must be released with `iztro_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_by_lunar(
    lunar_date: *const c_char,
    time_index: u8,
    gender: *const c_char,
    is_leap_month: bool,
    fix_leap: bool,
    language: *const c_char,
    config_json: *const c_char,
) -> *mut c_char {
    let result = (|| -> Result<String, String> {
        let lunar_date = unsafe { cstr_to_str(lunar_date, "lunar_date")? };
        let gender_str = unsafe { cstr_to_str(gender, "gender")? };
        let language_str = unsafe { cstr_to_str(language, "language")? };
        let config_str = unsafe { cstr_to_opt_str(config_json, "config_json")? };

        let gender = parse_gender(gender_str)?;
        let language = parse_language(language_str)?;
        let config = parse_config_json(config_str)?;

        Ok(crate::by_lunar_json(
            lunar_date, time_index, gender, is_leap_month, fix_leap, language, config,
        ))
    })();

    match result {
        Ok(json) => ok_json(json),
        Err(msg) => error_json(&msg),
    }
}

/// Calculate horoscope data for a birth chart and a target date, returned as
/// a JS-iztro-compatible JSON string.
///
/// The birth chart is recomputed from its parameters (stateless interface —
/// no chart JSON round-trip is needed).
///
/// # Parameters
/// - `solar_date`: Birth date string, e.g. "2000-8-16"
/// - `time_index`: Birth time index (0-12)
/// - `gender`: "male" or "female"
/// - `fix_leap`: Whether to fix leap month
/// - `language`: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
/// - `config_json`: NULL/empty for defaults, or a partial-config JSON
/// - `target_date`: Target date string, e.g. "2024-1-1"
/// - `target_time_index`: Target time index (0-12)
///
/// # Returns
/// A heap-allocated JSON C string. The caller must free it with `iztro_free_string`.
/// On error, returns a JSON string like `{"error": "message"}`.
///
/// # Safety
/// All pointer parameters except `config_json` must be valid NUL-terminated
/// C strings (or null, which yields an error JSON); `config_json` may be NULL.
/// The returned pointer must be released with `iztro_free_string`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_get_horoscope(
    solar_date: *const c_char,
    time_index: u8,
    gender: *const c_char,
    fix_leap: bool,
    language: *const c_char,
    config_json: *const c_char,
    target_date: *const c_char,
    target_time_index: u8,
) -> *mut c_char {
    let result = (|| -> Result<String, String> {
        let solar_date = unsafe { cstr_to_str(solar_date, "solar_date")? };
        let gender_str = unsafe { cstr_to_str(gender, "gender")? };
        let language_str = unsafe { cstr_to_str(language, "language")? };
        let config_str = unsafe { cstr_to_opt_str(config_json, "config_json")? };
        let target_date = unsafe { cstr_to_str(target_date, "target_date")? };

        let gender = parse_gender(gender_str)?;
        let language = parse_language(language_str)?;
        let config = parse_config_json(config_str)?;

        let astrolabe =
            crate::by_solar(solar_date, time_index, gender, fix_leap, language, config);
        let horoscope =
            crate::get_horoscope(&astrolabe, target_date, target_time_index, language);

        serde_json::to_string(&horoscope.to_dto(language))
            .map_err(|e| format!("Failed to serialize horoscope: {}", e))
    })();

    match result {
        Ok(json) => ok_json(json),
        Err(msg) => error_json(&msg),
    }
}

/// Free a string that was returned by any of the `iztro_*` functions.
///
/// # Safety
/// The pointer must have been returned by one of the FFI functions in this module.
/// Passing any other pointer is undefined behavior. Passing null is a no-op.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_free_string(s: *mut c_char) {
    if !s.is_null() {
        unsafe {
            drop(CString::from_raw(s));
        }
    }
}

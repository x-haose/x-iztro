//! wasm32 导出：供 wazero 等 WebAssembly 运行时（Go 绑定）调用。
//!
//! 内存协定：
//! - 调用方用 `iztro_wasm_alloc` 申请入参缓冲并写入 UTF-8 JSON；
//! - 功能函数接收 (ptr, len)，返回 `(ptr << 32) | len` 打包的结果缓冲，
//!   内容为 DTO JSON 或 `{"error":"..."}`；
//! - 双方的缓冲都用 `iztro_wasm_free` 释放。
//!
//! 入参 JSON 与绑定契约同构（camelCase）：
//! - by_solar:  {solarDate, timeIndex, gender, fixLeap, language, config?}
//! - by_lunar:  {lunarDate, timeIndex, gender, isLeapMonth, fixLeap, language, config?}
//! - horoscope: by_solar 字段 + {targetDate, targetTimeIndex}
//! 其中 config 为部分键对象（如 {"algorithm":"zhongzhou"}），省略取默认。

use serde::Deserialize;

use crate::data::types::{Config, Gender, Language};
use crate::dto::parse_config_json;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BySolarInput {
    solar_date: String,
    time_index: u8,
    gender: String,
    fix_leap: bool,
    language: String,
    config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ByLunarInput {
    lunar_date: String,
    time_index: u8,
    gender: String,
    is_leap_month: bool,
    fix_leap: bool,
    language: String,
    config: Option<serde_json::Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct HoroscopeInput {
    solar_date: String,
    time_index: u8,
    gender: String,
    fix_leap: bool,
    language: String,
    config: Option<serde_json::Value>,
    target_date: String,
    target_time_index: u8,
}

fn parse_gender(s: &str) -> Result<Gender, String> {
    match s.to_lowercase().as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(format!("Invalid gender '{s}'. Expected 'male' or 'female'.")),
    }
}

fn parse_language(s: &str) -> Result<Language, String> {
    match s.to_lowercase().as_str() {
        "zh_cn" => Ok(Language::ZhCN),
        "zh_tw" => Ok(Language::ZhTW),
        "en_us" => Ok(Language::EnUS),
        "ja_jp" => Ok(Language::JaJP),
        "ko_kr" => Ok(Language::KoKR),
        "vi_vn" => Ok(Language::ViVN),
        _ => Err(format!(
            "Invalid language '{s}'. Expected one of: zh_cn, zh_tw, en_us, ja_jp, ko_kr, vi_vn."
        )),
    }
}

fn parse_config_value(v: &Option<serde_json::Value>) -> Result<Config, String> {
    match v {
        None => Ok(Config::default()),
        Some(value) => parse_config_json(Some(&value.to_string())),
    }
}

/// 将结果字符串移交给调用方：泄漏缓冲并打包 (ptr << 32) | len。
fn hand_over(s: String) -> u64 {
    let bytes = s.into_bytes();
    let len = bytes.len() as u64;
    let ptr = Box::leak(bytes.into_boxed_slice()).as_mut_ptr() as u64;
    (ptr << 32) | len
}

fn error_result(msg: &str) -> u64 {
    hand_over(format!(r#"{{"error":"{}"}}"#, msg.replace('"', "\\\"")))
}

/// 读取调用方写入的入参缓冲。
///
/// # Safety
/// (ptr, len) 必须指向本模块 `iztro_wasm_alloc` 分配且已写入 len 字节的缓冲。
unsafe fn read_input(ptr: *const u8, len: u32) -> Result<String, String> {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    String::from_utf8(slice.to_vec()).map_err(|e| format!("Input is not valid UTF-8: {e}"))
}

/// 分配 len 字节的缓冲，供调用方写入入参。
#[unsafe(no_mangle)]
pub extern "C" fn iztro_wasm_alloc(len: u32) -> *mut u8 {
    let buf = vec![0u8; len as usize];
    Box::leak(buf.into_boxed_slice()).as_mut_ptr()
}

/// 释放由 `iztro_wasm_alloc` 分配或功能函数返回的缓冲。
///
/// # Safety
/// (ptr, len) 必须与分配时完全一致，且只释放一次。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_free(ptr: *mut u8, len: u32) {
    if !ptr.is_null() {
        unsafe {
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(ptr, len as usize)));
        }
    }
}

/// 阳历排盘：入参 by_solar JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_by_solar(ptr: *const u8, len: u32) -> u64 {
    let result = (|| -> Result<String, String> {
        let input = unsafe { read_input(ptr, len)? };
        let input: BySolarInput =
            serde_json::from_str(&input).map_err(|e| format!("Invalid input JSON: {e}"))?;
        let gender = parse_gender(&input.gender)?;
        let language = parse_language(&input.language)?;
        let config = parse_config_value(&input.config)?;
        let astrolabe = crate::by_solar(
            &input.solar_date,
            input.time_index,
            gender,
            input.fix_leap,
            language,
            config,
        );
        serde_json::to_string(&astrolabe.to_dto()).map_err(|e| e.to_string())
    })();

    match result {
        Ok(json) => hand_over(json),
        Err(msg) => error_result(&msg),
    }
}

/// 农历排盘：入参 by_lunar JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_by_lunar(ptr: *const u8, len: u32) -> u64 {
    let result = (|| -> Result<String, String> {
        let input = unsafe { read_input(ptr, len)? };
        let input: ByLunarInput =
            serde_json::from_str(&input).map_err(|e| format!("Invalid input JSON: {e}"))?;
        let gender = parse_gender(&input.gender)?;
        let language = parse_language(&input.language)?;
        let config = parse_config_value(&input.config)?;
        let astrolabe = crate::by_lunar(
            &input.lunar_date,
            input.time_index,
            gender,
            input.is_leap_month,
            input.fix_leap,
            language,
            config,
        );
        serde_json::to_string(&astrolabe.to_dto()).map_err(|e| e.to_string())
    })();

    match result {
        Ok(json) => hand_over(json),
        Err(msg) => error_result(&msg),
    }
}

/// 运限（无状态）：入参 horoscope JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_horoscope(ptr: *const u8, len: u32) -> u64 {
    let result = (|| -> Result<String, String> {
        let input = unsafe { read_input(ptr, len)? };
        let input: HoroscopeInput =
            serde_json::from_str(&input).map_err(|e| format!("Invalid input JSON: {e}"))?;
        let gender = parse_gender(&input.gender)?;
        let language = parse_language(&input.language)?;
        let config = parse_config_value(&input.config)?;
        let astrolabe = crate::by_solar(
            &input.solar_date,
            input.time_index,
            gender,
            input.fix_leap,
            language,
            config,
        );
        let horoscope = crate::get_horoscope(
            &astrolabe,
            &input.target_date,
            input.target_time_index,
            language,
        );
        serde_json::to_string(&horoscope.to_dto(language)).map_err(|e| e.to_string())
    })();

    match result {
        Ok(json) => hand_over(json),
        Err(msg) => error_result(&msg),
    }
}

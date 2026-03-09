//! PyO3 Python bindings for rs-iztro.
//!
//! This module is only compiled when the `python` feature is enabled.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use crate::data::types::{Algorithm, Gender, Language};
use crate::models::astrolabe::Astrolabe;

/// Parse a gender string into the Rust enum.
fn parse_gender(s: &str) -> PyResult<Gender> {
    match s.to_lowercase().as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(PyValueError::new_err(format!(
            "Invalid gender '{}'. Expected 'male' or 'female'.",
            s
        ))),
    }
}

/// Parse a language string into the Rust enum.
fn parse_language(s: &str) -> PyResult<Language> {
    match s.to_lowercase().as_str() {
        "zh_cn" => Ok(Language::ZhCN),
        "zh_tw" => Ok(Language::ZhTW),
        "en_us" => Ok(Language::EnUS),
        "ja_jp" => Ok(Language::JaJP),
        "ko_kr" => Ok(Language::KoKR),
        "vi_vn" => Ok(Language::ViVN),
        _ => Err(PyValueError::new_err(format!(
            "Invalid language '{}'. Expected one of: zh_cn, zh_tw, en_us, ja_jp, ko_kr, vi_vn.",
            s
        ))),
    }
}

/// Parse an algorithm string into the Rust enum.
fn parse_algorithm(s: &str) -> PyResult<Algorithm> {
    match s.to_lowercase().as_str() {
        "default" => Ok(Algorithm::Default),
        "zhongzhou" => Ok(Algorithm::Zhongzhou),
        _ => Err(PyValueError::new_err(format!(
            "Invalid algorithm '{}'. Expected 'default' or 'zhongzhou'.",
            s
        ))),
    }
}

/// Generate an astrolabe from a solar (Gregorian) date and return it as a JSON string.
///
/// Args:
///     solar_date: Date string, e.g. "2000-8-16"
///     time_index: Time index (0-12)
///     gender: "male" or "female"
///     fix_leap: Whether to fix leap month
///     language: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
///     algorithm: "default" or "zhongzhou"
///
/// Returns:
///     JSON string of the astrolabe
#[pyfunction]
fn by_solar_json(
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    algorithm: &str,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let algorithm = parse_algorithm(algorithm)?;

    let result = crate::by_solar_json(solar_date, time_index, gender, fix_leap, language, algorithm);
    Ok(result)
}

/// Generate an astrolabe from a lunar (Chinese calendar) date and return it as a JSON string.
///
/// Args:
///     lunar_date: Lunar date string, e.g. "2000-7-16"
///     time_index: Time index (0-12)
///     gender: "male" or "female"
///     is_leap_month: Whether the lunar month is a leap month
///     fix_leap: Whether to fix leap month
///     language: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
///     algorithm: "default" or "zhongzhou"
///
/// Returns:
///     JSON string of the astrolabe
#[pyfunction]
fn by_lunar_json(
    lunar_date: &str,
    time_index: u8,
    gender: &str,
    is_leap_month: bool,
    fix_leap: bool,
    language: &str,
    algorithm: &str,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let algorithm = parse_algorithm(algorithm)?;

    let result = crate::by_lunar_json(
        lunar_date, time_index, gender, is_leap_month, fix_leap, language, algorithm,
    );
    Ok(result)
}

/// Calculate horoscope data from an astrolabe JSON string and return it as a JSON string.
///
/// Args:
///     astrolabe_json: JSON string of an Astrolabe (as returned by by_solar_json or by_lunar_json)
///     target_date: Target date string, e.g. "2024-1-1"
///     time_index: Time index (0-12)
///     language: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
///
/// Returns:
///     JSON string of the horoscope data
#[pyfunction]
fn get_horoscope_json(
    astrolabe_json: &str,
    target_date: &str,
    time_index: u8,
    language: &str,
) -> PyResult<String> {
    let language = parse_language(language)?;
    let astrolabe: Astrolabe = serde_json::from_str(astrolabe_json).map_err(|e| {
        PyValueError::new_err(format!("Failed to parse astrolabe JSON: {}", e))
    })?;

    let horoscope = crate::get_horoscope(&astrolabe, target_date, time_index, language);

    let json = serde_json::to_string(&horoscope).map_err(|e| {
        PyValueError::new_err(format!("Failed to serialize horoscope: {}", e))
    })?;

    Ok(json)
}

/// The rs_iztro Python module.
#[pymodule]
fn rs_iztro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(by_solar_json, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar_json, m)?)?;
    m.add_function(wrap_pyfunction!(get_horoscope_json, m)?)?;
    Ok(())
}

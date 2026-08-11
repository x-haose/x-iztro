//! PyO3 Python bindings for rs-iztro.
//!
//! This module is only compiled when the `python` feature is enabled.
//! Functions return native Python dicts/lists instead of JSON strings.

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

/// Convert a serde-serializable value to a Python object.
fn to_python<T: serde::Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    pythonize::pythonize(py, value)
        .map(|bound| bound.unbind())
        .map_err(|e| PyValueError::new_err(format!("Failed to convert to Python object: {}", e)))
}

/// Generate an astrolabe from a solar (Gregorian) date.
///
/// Returns a Python dict with all astrolabe fields.
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
///     dict with astrolabe data
#[pyfunction]
fn by_solar(
    py: Python<'_>,
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    algorithm: &str,
) -> PyResult<PyObject> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let algorithm = parse_algorithm(algorithm)?;

    let astrolabe = crate::by_solar(solar_date, time_index, gender, fix_leap, language, algorithm);
    to_python(py, &astrolabe)
}

/// Generate an astrolabe from a lunar (Chinese calendar) date.
///
/// Returns a Python dict with all astrolabe fields.
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
///     dict with astrolabe data
#[pyfunction]
fn by_lunar(
    py: Python<'_>,
    lunar_date: &str,
    time_index: u8,
    gender: &str,
    is_leap_month: bool,
    fix_leap: bool,
    language: &str,
    algorithm: &str,
) -> PyResult<PyObject> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let algorithm = parse_algorithm(algorithm)?;

    let astrolabe = crate::by_lunar(
        lunar_date, time_index, gender, is_leap_month, fix_leap, language, algorithm,
    );
    to_python(py, &astrolabe)
}

/// Calculate horoscope data from an astrolabe dict.
///
/// Args:
///     astrolabe: dict (as returned by by_solar or by_lunar)
///     target_date: Target date string, e.g. "2024-1-1"
///     time_index: Time index (0-12)
///     language: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
///
/// Returns:
///     dict with horoscope data
#[pyfunction]
fn get_horoscope(
    py: Python<'_>,
    astrolabe: &Bound<'_, pyo3::types::PyAny>,
    target_date: &str,
    time_index: u8,
    language: &str,
) -> PyResult<PyObject> {
    let language = parse_language(language)?;
    let astrolabe: Astrolabe = pythonize::depythonize(astrolabe)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse astrolabe: {}", e)))?;

    let horoscope = crate::get_horoscope(&astrolabe, target_date, time_index, language);
    to_python(py, &horoscope)
}

/// Generate an AI prompt from an astrolabe dict.
///
/// Args:
///     astrolabe: dict (as returned by by_solar or by_lunar)
///     language: "zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", or "vi_vn"
///
/// Returns:
///     str with the AI prompt text
#[pyfunction]
fn astrolabe_to_prompt(
    astrolabe: &Bound<'_, pyo3::types::PyAny>,
    language: &str,
) -> PyResult<String> {
    let language = parse_language(language)?;
    let astrolabe: Astrolabe = pythonize::depythonize(astrolabe)
        .map_err(|e| PyValueError::new_err(format!("Failed to parse astrolabe: {}", e)))?;

    Ok(crate::astrolabe_to_prompt(&astrolabe, language))
}

/// Generate an astrolabe from a solar date and return as JSON string.
///
/// Use by_solar() instead for a native Python dict.
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

    Ok(crate::by_solar_json(solar_date, time_index, gender, fix_leap, language, algorithm))
}

/// Generate an astrolabe from a lunar date and return as JSON string.
///
/// Use by_lunar() instead for a native Python dict.
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

    Ok(crate::by_lunar_json(
        lunar_date, time_index, gender, is_leap_month, fix_leap, language, algorithm,
    ))
}

/// The native rs_iztro Python module (internal, used by the Python wrapper).
#[pymodule]
fn _rs_iztro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Native dict API (recommended)
    m.add_function(wrap_pyfunction!(by_solar, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar, m)?)?;
    m.add_function(wrap_pyfunction!(get_horoscope, m)?)?;
    m.add_function(wrap_pyfunction!(astrolabe_to_prompt, m)?)?;

    // JSON string API (backward compatible)
    m.add_function(wrap_pyfunction!(by_solar_json, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar_json, m)?)?;
    Ok(())
}

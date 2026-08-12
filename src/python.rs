//! PyO3 Python bindings for rs-iztro.
//!
//! 仅在 `python` feature 下编译。所有函数为无状态接口：以排盘参数直接调用，
//! 返回 JS iztro 兼容 DTO（camelCase 键 + 按语言翻译的值）的原生 Python dict。

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use crate::data::types::{Gender, Language};
use crate::dto::parse_config_json;

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

/// Convert a serde-serializable value to a Python object.
fn to_python<T: serde::Serialize>(py: Python<'_>, value: &T) -> PyResult<PyObject> {
    pythonize::pythonize(py, value)
        .map(|bound| bound.unbind())
        .map_err(|e| PyValueError::new_err(format!("Failed to convert to Python object: {}", e)))
}

/// 阳历排盘，返回 JS iztro 兼容的 dict。
#[pyfunction]
#[pyo3(signature = (solar_date, time_index, gender, fix_leap, language, config_json=None))]
fn by_solar(
    py: Python<'_>,
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
) -> PyResult<PyObject> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    let astrolabe = crate::by_solar(solar_date, time_index, gender, fix_leap, language, config);
    to_python(py, &astrolabe.to_dto())
}

/// 农历排盘，返回 JS iztro 兼容的 dict。
#[pyfunction]
#[pyo3(signature = (lunar_date, time_index, gender, is_leap_month, fix_leap, language, config_json=None))]
fn by_lunar(
    py: Python<'_>,
    lunar_date: &str,
    time_index: u8,
    gender: &str,
    is_leap_month: bool,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
) -> PyResult<PyObject> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    let astrolabe = crate::by_lunar(
        lunar_date, time_index, gender, is_leap_month, fix_leap, language, config,
    );
    to_python(py, &astrolabe.to_dto())
}

/// 计算运限（无状态：以出生排盘参数与目标日期直接调用），返回 dict。
#[pyfunction]
#[allow(clippy::too_many_arguments)]
#[pyo3(signature = (solar_date, time_index, gender, fix_leap, language, config_json, target_date, target_time_index))]
fn get_horoscope(
    py: Python<'_>,
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
    target_date: &str,
    target_time_index: u8,
) -> PyResult<PyObject> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    let astrolabe = crate::by_solar(solar_date, time_index, gender, fix_leap, language, config);
    let horoscope = crate::get_horoscope(&astrolabe, target_date, target_time_index, language);
    to_python(py, &horoscope.to_dto(language))
}

/// 生成本命盘 AI Prompt（无状态）。
#[pyfunction]
#[pyo3(signature = (solar_date, time_index, gender, fix_leap, language, config_json=None))]
fn astrolabe_to_prompt(
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    let astrolabe = crate::by_solar(solar_date, time_index, gender, fix_leap, language, config);
    Ok(crate::astrolabe_to_prompt(&astrolabe, language))
}

/// 生成运限 AI Prompt（无状态）。
#[pyfunction]
#[allow(clippy::too_many_arguments)]
#[pyo3(signature = (solar_date, time_index, gender, fix_leap, language, config_json, target_date, target_time_index))]
fn horoscope_to_prompt(
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
    target_date: &str,
    target_time_index: u8,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    let astrolabe = crate::by_solar(solar_date, time_index, gender, fix_leap, language, config);
    let horoscope = crate::get_horoscope(&astrolabe, target_date, target_time_index, language);
    Ok(crate::horoscope_to_prompt(&astrolabe, &horoscope, language))
}

/// 阳历排盘并返回 JSON 字符串（内容与 by_solar 的 dict 一致）。
#[pyfunction]
#[pyo3(signature = (solar_date, time_index, gender, fix_leap, language, config_json=None))]
fn by_solar_json(
    solar_date: &str,
    time_index: u8,
    gender: &str,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    Ok(crate::by_solar_json(solar_date, time_index, gender, fix_leap, language, config))
}

/// 农历排盘并返回 JSON 字符串（内容与 by_lunar 的 dict 一致）。
#[pyfunction]
#[pyo3(signature = (lunar_date, time_index, gender, is_leap_month, fix_leap, language, config_json=None))]
fn by_lunar_json(
    lunar_date: &str,
    time_index: u8,
    gender: &str,
    is_leap_month: bool,
    fix_leap: bool,
    language: &str,
    config_json: Option<&str>,
) -> PyResult<String> {
    let gender = parse_gender(gender)?;
    let language = parse_language(language)?;
    let config = parse_config_json(config_json).map_err(PyValueError::new_err)?;

    Ok(crate::by_lunar_json(
        lunar_date, time_index, gender, is_leap_month, fix_leap, language, config,
    ))
}

/// The native rs_iztro Python module (internal, used by the Python wrapper).
#[pymodule]
fn _rs_iztro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(by_solar, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar, m)?)?;
    m.add_function(wrap_pyfunction!(get_horoscope, m)?)?;
    m.add_function(wrap_pyfunction!(astrolabe_to_prompt, m)?)?;
    m.add_function(wrap_pyfunction!(horoscope_to_prompt, m)?)?;
    m.add_function(wrap_pyfunction!(by_solar_json, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar_json, m)?)?;
    Ok(())
}

//! PyO3 Python 绑定。
//!
//! 仅在 `python` feature 下编译。只负责 Python 对象与 Rust 结构体的互转，
//! 计算与编组一律走 [`crate::bridge`]——与 Go 绑定同一条代码路径，
//! 两侧的行为没有分叉的余地。
//!
//! 入参为 camelCase 键的 dict，字段见 [`crate::bridge`] 的入参结构体；
//! 出参为 camelCase 键、值按语言翻译的原生 Python 对象。

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::de::DeserializeOwned;

use crate::bridge::{self, HoroscopeInput, LunarChartInput, QueryInput, SolarChartInput};

/// 把入参 dict 转成 bridge 的入参结构体。
fn from_python<T: DeserializeOwned>(input: &Bound<'_, PyAny>) -> PyResult<T> {
    pythonize::depythonize(input).map_err(|e| PyValueError::new_err(format!("Invalid input: {e}")))
}

/// 把结果转成原生 Python 对象。
fn to_python(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    pythonize::pythonize(py, value)
        .map(|bound| bound.unbind())
        .map_err(|e| PyValueError::new_err(format!("Failed to convert to Python object: {e}")))
}

/// 调用 bridge：入参错误转为 ValueError；库内部缺陷导致的 panic 同样兜底转为
/// ValueError，避免抛出 `except Exception` 捕获不到的 PanicException。
fn run<T: DeserializeOwned>(
    input: &Bound<'_, PyAny>,
    compute: impl FnOnce(&T) -> Result<serde_json::Value, String>,
) -> PyResult<serde_json::Value> {
    let parsed: T = from_python(input)?;

    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compute(&parsed))) {
        Ok(result) => result.map_err(PyValueError::new_err),
        Err(panic) => Err(PyValueError::new_err(format!(
            "Invalid input: {}",
            crate::dto::panic_message(panic.as_ref())
        ))),
    }
}

/// 阳历排盘，返回 JS iztro 兼容的 dict。
#[pyfunction]
fn by_solar(py: Python<'_>, input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    to_python(py, &run(input, |i: &SolarChartInput| bridge::by_solar(i))?)
}

/// 农历排盘，返回 JS iztro 兼容的 dict。
#[pyfunction]
fn by_lunar(py: Python<'_>, input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    to_python(py, &run(input, |i: &LunarChartInput| bridge::by_lunar(i))?)
}

/// 运限，返回 JS iztro 兼容的 dict。
#[pyfunction]
fn get_horoscope(py: Python<'_>, input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    to_python(py, &run(input, |i: &HoroscopeInput| bridge::horoscope(i))?)
}

/// 统一查询：由入参的 `kind` 分派到轻量查询、工具函数、安星、数据表或翻译。
#[pyfunction]
fn query(py: Python<'_>, input: &Bound<'_, PyAny>) -> PyResult<PyObject> {
    to_python(py, &run(input, |i: &QueryInput| bridge::query(i))?)
}

/// 阳历排盘并返回 JSON 字符串（内容与 by_solar 的 dict 一致）。
#[pyfunction]
fn by_solar_json(input: &Bound<'_, PyAny>) -> PyResult<String> {
    let value = run(input, |i: &SolarChartInput| bridge::by_solar(i))?;
    serde_json::to_string(&value).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// 农历排盘并返回 JSON 字符串（内容与 by_lunar 的 dict 一致）。
#[pyfunction]
fn by_lunar_json(input: &Bound<'_, PyAny>) -> PyResult<String> {
    let value = run(input, |i: &LunarChartInput| bridge::by_lunar(i))?;
    serde_json::to_string(&value).map_err(|e| PyValueError::new_err(e.to_string()))
}

/// x-iztro 的 PyO3 原生模块。
#[pymodule]
fn _x_iztro(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(by_solar, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar, m)?)?;
    m.add_function(wrap_pyfunction!(get_horoscope, m)?)?;
    m.add_function(wrap_pyfunction!(query, m)?)?;
    m.add_function(wrap_pyfunction!(by_solar_json, m)?)?;
    m.add_function(wrap_pyfunction!(by_lunar_json, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

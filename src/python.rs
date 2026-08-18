//! PyO3 Python 绑定。
//!
//! 仅在 `python` feature 下编译。只负责 Python 对象与 Rust 结构体的互转，
//! 计算与编组一律走 [`crate::bridge`]——与 Go 绑定同一条代码路径，
//! 两侧的行为没有分叉的余地。
//!
//! 入参为 camelCase 键的 dict，字段见 [`crate::bridge`] 的入参结构体；
//! 出参为 camelCase 键、值按语言翻译的原生 Python 对象。

use pyo3::create_exception;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use serde::de::DeserializeOwned;

use crate::bridge::{self, HoroscopeInput, LunarChartInput, QueryInput, SolarChartInput};
use crate::error::BridgeError;

create_exception!(
    _x_iztro,
    IztroError,
    PyValueError,
    "排盘出错。`args[0]` 为面向人的描述，`code` 属性为机器可读的分类标识\n     （invalid_date / invalid_time_index / invalid_argument / internal）。\n     继承 ValueError，既有的 `except ValueError` 依然能捕获。"
);

/// 把 [`BridgeError`] 落成带 `code` 属性的 Python 异常。
fn to_py_err(err: &BridgeError) -> PyErr {
    let py_err = IztroError::new_err(err.message.clone());
    Python::with_gil(|py| {
        // 异常实例带 __dict__，附加属性不会失败；万一失败也只是少了 code，
        // 不该把原始错误换成属性设置错误。
        let _ = py_err.value(py).setattr("code", err.code);
    });
    py_err
}

/// 把入参 dict 转成 bridge 的入参结构体。
fn from_python<T: DeserializeOwned>(input: &Bound<'_, PyAny>) -> PyResult<T> {
    pythonize::depythonize(input).map_err(|e| {
        to_py_err(&BridgeError::invalid_argument(format!(
            "invalid input: {e}"
        )))
    })
}

/// 把结果转成原生 Python 对象。
fn to_python(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    pythonize::pythonize(py, value)
        .map(|bound| bound.unbind())
        .map_err(|e| {
            to_py_err(&BridgeError::internal(format!(
                "failed to convert to Python object: {e}"
            )))
        })
}

/// 调用 bridge：入参错误与库内部缺陷导致的 panic 一律转为 `IztroError`，
/// 避免抛出 `except Exception` 捕获不到的 PanicException。
fn run<T: DeserializeOwned>(
    input: &Bound<'_, PyAny>,
    compute: impl FnOnce(&T) -> Result<serde_json::Value, BridgeError>,
) -> PyResult<serde_json::Value> {
    let parsed: T = from_python(input)?;

    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| compute(&parsed))) {
        Ok(result) => result.map_err(|e| to_py_err(&e)),
        Err(panic) => Err(to_py_err(&BridgeError::internal(
            crate::dto::panic_message(panic.as_ref()),
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
    serde_json::to_string(&value).map_err(|e| to_py_err(&BridgeError::internal(e.to_string())))
}

/// 农历排盘并返回 JSON 字符串（内容与 by_lunar 的 dict 一致）。
#[pyfunction]
fn by_lunar_json(input: &Bound<'_, PyAny>) -> PyResult<String> {
    let value = run(input, |i: &LunarChartInput| bridge::by_lunar(i))?;
    serde_json::to_string(&value).map_err(|e| to_py_err(&BridgeError::internal(e.to_string())))
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
    m.add("IztroError", m.py().get_type::<IztroError>())?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}

//! wasm32 导出：供 wazero 等 WebAssembly 运行时（Go 绑定）调用。
//!
//! 只负责内存协定与 JSON 收发，计算与编组一律走 [`crate::bridge`]——
//! 与 Python 绑定同一条代码路径，两侧的行为没有分叉的余地。
//!
//! 全部外部输入（含日期与时辰）在核心层校验并以错误 JSON 返回，正常使用
//! 不会触发 panic。wasm 下 panic 即 abort（trap），且每次 trap 都永久损耗
//! 模块实例的栈空间——trap 仅在库内部缺陷时可能发生，宿主一旦观察到
//! trap 应重建模块实例。
//!
//! 内存协定：
//! - 调用方用 `iztro_wasm_alloc` 申请入参缓冲并写入 UTF-8 JSON；
//! - 功能函数接收 (ptr, len)，返回 `(ptr << 32) | len` 打包的结果缓冲，
//!   内容为 DTO JSON 或 `{"error":"...","code":"..."}`；
//! - 双方的缓冲都用 `iztro_wasm_free` 释放。
//!
//! 入参 JSON 键为 camelCase，字段见 [`crate::bridge`] 的入参结构体：
//! - by_solar / by_lunar / horoscope：各自的排盘参数
//! - query：`{kind, ...}` 统一入口，返回 `{"value": <JSON>}`

use serde::de::DeserializeOwned;

use crate::bridge::{self, HoroscopeInput, LunarChartInput, QueryInput, SolarChartInput};
use crate::error::BridgeError;

/// 将结果字符串移交给调用方：泄漏缓冲并打包 (ptr << 32) | len。
fn hand_over(s: String) -> u64 {
    let bytes = s.into_bytes();
    let len = bytes.len() as u64;
    let ptr = Box::leak(bytes.into_boxed_slice()).as_mut_ptr() as u64;
    (ptr << 32) | len
}

fn error_result(err: &BridgeError) -> u64 {
    hand_over(crate::dto::error_json(err))
}

/// 读取调用方写入的入参缓冲。
///
/// # Safety
/// (ptr, len) 必须指向本模块 `iztro_wasm_alloc` 分配且已写入 len 字节的缓冲。
unsafe fn read_input(ptr: *const u8, len: u32) -> Result<String, BridgeError> {
    let slice = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    String::from_utf8(slice.to_vec())
        .map_err(|e| BridgeError::invalid_argument(format!("invalid input: not valid UTF-8: {e}")))
}

/// 读入参、反序列化、交给 bridge、序列化结果，任一步出错都落成错误 JSON。
///
/// # Safety
/// (ptr, len) 须满足 [`read_input`] 的要求。
unsafe fn run<T, F>(ptr: *const u8, len: u32, compute: F) -> u64
where
    T: DeserializeOwned,
    F: FnOnce(&T) -> Result<serde_json::Value, BridgeError>,
{
    let result = unsafe { read_input(ptr, len) }.and_then(|raw| {
        let input: T = serde_json::from_str(&raw)
            .map_err(|e| BridgeError::invalid_argument(format!("invalid input JSON: {e}")))?;
        let value = compute(&input)?;
        serde_json::to_string(&value)
            .map_err(|e| BridgeError::internal(format!("failed to serialize result: {e}")))
    });

    match result {
        Ok(json) => hand_over(json),
        Err(err) => error_result(&err),
    }
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
            drop(Box::from_raw(std::ptr::slice_from_raw_parts_mut(
                ptr,
                len as usize,
            )));
        }
    }
}

/// 阳历排盘：入参 by_solar JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_by_solar(ptr: *const u8, len: u32) -> u64 {
    unsafe { run(ptr, len, |input: &SolarChartInput| bridge::by_solar(input)) }
}

/// 农历排盘：入参 by_lunar JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_by_lunar(ptr: *const u8, len: u32) -> u64 {
    unsafe { run(ptr, len, |input: &LunarChartInput| bridge::by_lunar(input)) }
}

/// 运限（无状态）：入参 horoscope JSON，返回 DTO JSON 缓冲。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_horoscope(ptr: *const u8, len: u32) -> u64 {
    unsafe { run(ptr, len, |input: &HoroscopeInput| bridge::horoscope(input)) }
}

/// 统一查询：入参 QueryInput JSON，返回 `{"value": <JSON>}` 缓冲。
///
/// 全部轻量查询、工具函数、安星、数据表与翻译共用此入口，由 `kind` 分派——
/// 免去为每个函数各开一个 wasm 符号与一套内存往返。
///
/// # Safety
/// (ptr, len) 须满足 `read_input` 的要求。
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iztro_wasm_query(ptr: *const u8, len: u32) -> u64 {
    unsafe {
        run(ptr, len, |input: &QueryInput| {
            bridge::query(input).map(|value| serde_json::json!({ "value": value }))
        })
    }
}

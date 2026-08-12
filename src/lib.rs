//! 紫微斗数排盘核心库，移植自 JS iztro v2.5.8。
//!
//! 入口：[`by_solar`] / [`by_lunar`] 排盘，[`get_horoscope`] 计算运限，
//! [`astrolabe_to_prompt`] / [`horoscope_to_prompt`] 生成 AI 分析文本。
//! 排盘行为由 [`Config`] 的五个开关控制，默认与 JS iztro 一致。

#![warn(missing_docs)]

/// 排盘与运限主流程
pub mod astro;
/// 枚举、常量与星耀数据表
pub mod data;
/// 跨语言绑定的序列化 DTO（camelCase + 翻译值 + 语言无关标识）
pub mod dto;
/// 排盘入口的错误类型
pub mod error;
/// 六语言词表与翻译入口
pub mod i18n;
/// 星盘、宫位、星耀与运限的数据结构
pub mod models;
/// AI 分析 Prompt 生成
pub mod prompt;
/// 安星算法
pub mod star;
/// 通用索引工具
pub mod utils;

#[cfg(feature = "python")]
mod python;

/// C ABI 导出（供 Go/C 等语言调用）
pub mod ffi;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

// Re-export main public API
pub use astro::builder::{by_lunar, by_solar};
pub use astro::horoscope::get_horoscope;
pub use data::types::*;
pub use error::IztroError;
pub use models::astrolabe::Astrolabe;
pub use models::horoscope::HoroscopeData;
pub use prompt::{astrolabe_to_prompt, horoscope_to_prompt};

/// 便捷函数：排盘并返回 JSON
///
/// # Errors
/// 入参非法时返回 [`IztroError`]（同 [`by_solar`]）。
pub fn by_solar_json(
    solar_date: &str,
    time_index: u8,
    gender: Gender,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = by_solar(solar_date, time_index, gender, fix_leap, language, config)?;
    Ok(serde_json::to_string(&astrolabe.to_dto()).unwrap())
}

/// 便捷函数：农历排盘并返回 JSON
///
/// # Errors
/// 入参非法时返回 [`IztroError`]（同 [`by_lunar`]）。
pub fn by_lunar_json(
    lunar_date: &str,
    time_index: u8,
    gender: Gender,
    is_leap_month: bool,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = by_lunar(
        lunar_date,
        time_index,
        gender,
        is_leap_month,
        fix_leap,
        language,
        config,
    )?;
    Ok(serde_json::to_string(&astrolabe.to_dto()).unwrap())
}

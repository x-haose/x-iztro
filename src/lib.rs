//! 紫微斗数排盘核心库，移植自 JS iztro v2.5.8。
//!
//! 入口：[`by_solar`] / [`by_lunar`] 排盘，[`get_horoscope`] 计算运限，
//! [`astrolabe_to_text`] / [`horoscope_to_text`] 生成语义化文本（星盘对象也有
//! 同名 `to_text` 方法，与 `to_json` 相对：机器格式与自然语言两种投影）。
//! 排盘行为由 [`Config`] 的六个开关控制，默认与 JS iztro 一致。

#![warn(missing_docs)]
// 核心库不写 unsafe；FFI 与 wasm 边界模块各自以 #![allow(unsafe_code)] 豁免，
// 「unsafe 只出现在边界」由编译期保证而非约定。
#![deny(unsafe_code)]

/// 绑定层共用的编组与分派。
///
/// 排盘入口只被 `python` feature 与 wasm32 目标的绑定文件消费——两者都不在
/// 默认的本机构建里，故豁免 dead_code；C FFI 只经它走统一查询入口。
#[allow(dead_code)]
pub(crate) mod bridge;

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
/// 知识包：解读文本与门派属性
pub mod knowledge;
/// 星盘、宫位、星耀与运限的数据结构
pub mod models;
/// 格局判定引擎
pub mod pattern;
/// 安星算法
pub mod star;
/// 语义化文本投影（to_text）
pub mod text;
/// 通用索引工具
pub mod utils;

#[cfg(feature = "python")]
mod python;

/// C ABI 导出（供 Go/C 等语言调用）
#[doc(hidden)]
pub mod ffi;

#[cfg(target_arch = "wasm32")]
#[doc(hidden)]
pub mod wasm;

// Re-export main public API
pub use astro::builder::{by_lunar, by_solar};
pub use astro::horoscope::get_horoscope;
pub use astro::horoscope::{flow_star_counterparts, natal_counterpart_of_flow_star};
pub use astro::palace::{get_five_elements_class, get_palace_names, get_soul_and_body};
pub use astro::query::{
    get_major_star_by_lunar_date, get_major_star_by_solar_date, get_sign_by_lunar_date,
    get_sign_by_solar_date, get_zodiac_by_solar_date, major_star_keys_of_soul_palace,
};
pub use astro::reverse::{
    BirthCandidate, DEFAULT_REVERSE_LIMIT, ReverseCriteria, ReverseResult, StarPosition,
    reverse_chart, solar_dates_by_bazi,
};
pub use data::stars::StarKey;
pub use data::types::*;
pub use error::{BridgeError, IztroError};
pub use i18n::lookup::{key_of, key_of_in, translate_key};
pub use i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_horoscope_name, translate_mutagen,
    translate_palace, translate_pattern, translate_sign, translate_star, translate_time,
    translate_zodiac,
};
pub use knowledge::KnowledgePack;
pub use models::astrolabe::{
    Astrolabe, PalaceRef, PalaceTarget, RawChineseDate, RawDates, RawLunarDate, StarRef,
};
pub use models::horoscope::{
    AgeItem, HoroscopeData, HoroscopeItem, HoroscopeRef, YearlyDecStar, YearlyItem,
};
pub use models::palace::{Decadal, PalaceData};
pub use models::star::Star;
pub use models::surpalaces::SurroundedPalaces;
pub use pattern::{
    ALL_PATTERNS, BrightnessSource, PatternConfig, PatternHit, PatternKey, StarAt, patterns_at,
};
pub use text::{
    TextOptions, astrolabe_to_text, astrolabe_to_text_with, horoscope_to_text,
    horoscope_to_text_with, palace_to_text, palace_to_text_with, patterns_to_text,
    patterns_to_text_with, surrounded_palaces_to_text, surrounded_palaces_to_text_with,
};

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
    Ok(serde_json::to_string(&astrolabe.to_dto())
        .expect("DTO 只含字符串、数值与容器，序列化不会失败"))
}

/// 便捷函数：农历排盘并返回 JSON
///
/// # Errors
/// 入参非法时返回 [`IztroError`]（同 [`by_lunar`]）。
pub fn by_lunar_json(
    lunar_date: &str,
    time_index: u8,
    gender: Gender,
    leap: LeapMonth,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = by_lunar(lunar_date, time_index, gender, leap, language, config)?;
    Ok(serde_json::to_string(&astrolabe.to_dto())
        .expect("DTO 只含字符串、数值与容器，序列化不会失败"))
}

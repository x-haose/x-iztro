pub mod data;
pub mod models;
pub mod star;
pub mod astro;
pub mod i18n;
pub mod utils;
pub mod prompt;

#[cfg(feature = "python")]
mod python;

pub mod ffi;

// Re-export main public API
pub use astro::builder::{by_solar, by_lunar};
pub use astro::horoscope::get_horoscope;
pub use models::astrolabe::Astrolabe;
pub use models::horoscope::HoroscopeData;
pub use data::types::*;
pub use prompt::{astrolabe_to_prompt, horoscope_to_prompt};

/// 便捷函数：排盘并返回 JSON
pub fn by_solar_json(
    solar_date: &str,
    time_index: u8,
    gender: Gender,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> String {
    let astrolabe = by_solar(solar_date, time_index, gender, fix_leap, language, config);
    serde_json::to_string(&astrolabe).unwrap()
}

/// 便捷函数：农历排盘并返回 JSON
pub fn by_lunar_json(
    lunar_date: &str,
    time_index: u8,
    gender: Gender,
    is_leap_month: bool,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> String {
    let astrolabe = by_lunar(lunar_date, time_index, gender, is_leap_month, fix_leap, language, config);
    serde_json::to_string(&astrolabe).unwrap()
}

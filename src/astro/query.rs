//! 不需要完整星盘的轻量查询。
//!
//! 对齐 iztro `astro` 模块中那几个「只取一个结果」的函数：生肖、星座、命宫主星。
//! 实现上复用排盘主流程再取字段，以保证与 [`by_solar`] / [`by_lunar`] 的结果永远一致。

use crate::astro::builder::{by_lunar, by_solar};
use crate::data::types::*;
use crate::error::IztroError;
use crate::models::astrolabe::Astrolabe;
use crate::utils::fix_index;

/// 取生肖时的固定时辰。
///
/// 生肖由年支决定，与出生时辰无关；iztro 同样固定用早子时查年支。
const ZODIAC_TIME_INDEX: u8 = 0;

/// 命宫主星之间的分隔符，与 iztro 的输出一致。
const MAJOR_STAR_SEPARATOR: &str = ",";

/// 通过阳历日期取生肖。
///
/// 年支的换年时点受 [`Config::year_divide`] 影响，因此正月初一与立春之间的
/// 生日会随配置得到不同结果。
///
/// # Errors
/// 日期非法时返回 [`IztroError`]。
pub fn get_zodiac_by_solar_date(
    solar_date: &str,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    Ok(by_solar(
        solar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        true,
        language,
        config,
    )?
    .zodiac)
}

/// 通过阳历日期取星座。
///
/// 星座只由公历日期决定，与配置和时辰无关。
///
/// # Errors
/// 日期非法时返回 [`IztroError`]。
pub fn get_sign_by_solar_date(solar_date: &str, language: Language) -> Result<String, IztroError> {
    Ok(by_solar(
        solar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        true,
        language,
        Config::default(),
    )?
    .sign)
}

/// 通过农历日期取星座。
///
/// 内部先把农历日期换算为公历再取星座；`is_leap_month` 在该月没有闰月时不生效。
///
/// # Errors
/// 日期非法时返回 [`IztroError`]。
pub fn get_sign_by_lunar_date(
    lunar_date: &str,
    is_leap_month: bool,
    language: Language,
) -> Result<String, IztroError> {
    Ok(by_lunar(
        lunar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        is_leap_month,
        true,
        language,
        Config::default(),
    )?
    .sign)
}

/// 取命宫主星名，多颗时以逗号分隔。
///
/// 命宫为空宫时借对宫主星，与 iztro 行为一致；对宫也无主星时返回空串。
fn major_stars_of_soul_palace(astrolabe: &Astrolabe) -> String {
    let soul_index = astrolabe
        .palaces
        .iter()
        .find(|p| p.name == Palace::Soul)
        .map_or(0, |p| p.index);

    let names_at = |index: usize| -> Vec<String> {
        astrolabe.palaces[index]
            .major_stars
            .iter()
            .filter(|s| s.star_type == StarType::Major)
            .map(|s| s.name.clone())
            .collect()
    };

    let names = names_at(soul_index);
    if !names.is_empty() {
        return names.join(MAJOR_STAR_SEPARATOR);
    }

    names_at(fix_index(soul_index as i32 + 6, 12)).join(MAJOR_STAR_SEPARATOR)
}

/// 通过阳历日期取命宫主星。
///
/// 命宫为空宫时借对宫主星。返回值按排盘语言翻译，多颗主星以逗号分隔。
///
/// # Errors
/// 日期或时辰索引非法时返回 [`IztroError`]。
pub fn get_major_star_by_solar_date(
    solar_date: &str,
    time_index: u8,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = by_solar(
        solar_date,
        time_index,
        Gender::Male,
        fix_leap,
        language,
        config,
    )?;
    Ok(major_stars_of_soul_palace(&astrolabe))
}

/// 通过农历日期取命宫主星。
///
/// 命宫为空宫时借对宫主星；`is_leap_month` 在该月没有闰月时不生效。
///
/// # Errors
/// 日期或时辰索引非法时返回 [`IztroError`]。
pub fn get_major_star_by_lunar_date(
    lunar_date: &str,
    time_index: u8,
    is_leap_month: bool,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = by_lunar(
        lunar_date,
        time_index,
        Gender::Male,
        is_leap_month,
        fix_leap,
        language,
        config,
    )?;
    Ok(major_stars_of_soul_palace(&astrolabe))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 轻量查询的结果必须与完整排盘的对应字段一致。
    #[test]
    fn test_matches_full_astrolabe() {
        let chart = by_solar(
            "2000-8-16",
            0,
            Gender::Male,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();

        assert_eq!(
            get_zodiac_by_solar_date("2000-8-16", Language::ZhCN, Config::default()).unwrap(),
            chart.zodiac
        );
        assert_eq!(
            get_sign_by_solar_date("2000-8-16", Language::ZhCN).unwrap(),
            chart.sign
        );
    }

    #[test]
    fn test_sign_by_lunar_date_equals_solar() {
        // 2000-7-17（农历）即 2000-8-16（阳历）
        assert_eq!(
            get_sign_by_lunar_date("2000-7-17", false, Language::ZhCN).unwrap(),
            get_sign_by_solar_date("2000-8-16", Language::ZhCN).unwrap()
        );
    }

    #[test]
    fn test_major_star_matches_soul_palace() {
        let chart = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let soul = chart.palace(Palace::Soul).unwrap();

        let expected: Vec<String> = soul
            .major_stars
            .iter()
            .filter(|s| s.star_type == StarType::Major)
            .map(|s| s.name.clone())
            .collect();

        assert!(!expected.is_empty(), "该盘命宫应有主星");
        assert_eq!(
            get_major_star_by_solar_date("2000-8-16", 2, true, Language::ZhCN, Config::default())
                .unwrap(),
            expected.join(",")
        );
    }

    #[test]
    fn test_major_star_borrows_from_opposite_when_empty() {
        // 逐日扫描找一个命宫空宫的盘，验证借对宫主星
        for day in 1..=28 {
            let date = format!("1990-3-{day}");
            let chart = by_solar(
                &date,
                0,
                Gender::Male,
                true,
                Language::ZhCN,
                Config::default(),
            )
            .unwrap();
            let soul = chart.palace(Palace::Soul).unwrap();
            if !soul.is_empty() {
                continue;
            }

            let opposite = &chart.palaces[fix_index(soul.index as i32 + 6, 12)];
            let expected: Vec<String> = opposite
                .major_stars
                .iter()
                .filter(|s| s.star_type == StarType::Major)
                .map(|s| s.name.clone())
                .collect();

            assert_eq!(
                get_major_star_by_solar_date(&date, 0, true, Language::ZhCN, Config::default())
                    .unwrap(),
                expected.join(","),
                "{date} 命宫空宫时应借对宫主星"
            );
            return;
        }
        panic!("测试区间内没有命宫空宫的盘，用例失去意义");
    }

    #[test]
    fn test_invalid_input_is_rejected() {
        assert!(get_zodiac_by_solar_date("2000-2-30", Language::ZhCN, Config::default()).is_err());
        assert!(
            get_major_star_by_solar_date("2000-8-16", 13, true, Language::ZhCN, Config::default())
                .is_err()
        );
    }
}

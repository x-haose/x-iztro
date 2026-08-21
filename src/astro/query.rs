//! 不需要完整星盘的轻量查询。
//!
//! 对齐 iztro `astro` 模块中那几个「只取一个结果」的函数：生肖、星座、命宫主星。
//! 实现上复用排盘主流程再取字段，以保证与 [`crate::by_solar`] / [`crate::by_lunar`] 的结果永远一致。

use crate::astro::builder::{by_lunar, by_solar};
use crate::data::types::*;
use crate::error::IztroError;
use crate::models::astrolabe::Astrolabe;
use crate::utils::fix_index;

/// 取生肖时的固定时辰。
///
/// 生肖由年支决定，与出生时辰无关；iztro 同样固定用早子时查年支。
const ZODIAC_TIME_INDEX: u8 = 0;

/// 生肖查询的排盘配方：固定早子时与 fix_leap，性别不影响星盘布局。
///
/// 绑定层的 `zodiacBySolar` 也走本函数取盘——配方只此一份，改动不会分叉。
pub(crate) fn zodiac_chart(
    solar_date: &str,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    by_solar(
        solar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        true,
        language,
        config,
    )
}

/// 星座查询的排盘配方（阳历）：星座只由公历日期决定，配置固定默认。
///
/// 绑定层的 `signBySolar` 也走本函数取盘。
pub(crate) fn sign_chart_by_solar(
    solar_date: &str,
    language: Language,
) -> Result<Astrolabe, IztroError> {
    by_solar(
        solar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        true,
        language,
        Config::default(),
    )
}

/// 星座查询的排盘配方（农历）；绑定层的 `signByLunar` 也走本函数取盘。
pub(crate) fn sign_chart_by_lunar(
    lunar_date: &str,
    is_leap_month: bool,
    language: Language,
) -> Result<Astrolabe, IztroError> {
    by_lunar(
        lunar_date,
        ZODIAC_TIME_INDEX,
        Gender::Male,
        LeapMonth::from_flags(is_leap_month, true),
        language,
        Config::default(),
    )
}

/// 命宫主星查询的排盘配方（阳历）：性别不影响星盘布局，按 iztro 同名函数固定男。
///
/// 绑定层的 `majorStarBySolar` 也走本函数取盘。
pub(crate) fn major_star_chart_by_solar(
    solar_date: &str,
    time_index: u8,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    by_solar(
        solar_date,
        time_index,
        Gender::Male,
        fix_leap,
        language,
        config,
    )
}

/// 命宫主星查询的排盘配方（农历）；绑定层的 `majorStarByLunar` 也走本函数取盘。
pub(crate) fn major_star_chart_by_lunar(
    lunar_date: &str,
    time_index: u8,
    leap: LeapMonth,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    by_lunar(lunar_date, time_index, Gender::Male, leap, language, config)
}

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
    Ok(zodiac_chart(solar_date, language, config)?.zodiac)
}

/// 通过阳历日期取星座。
///
/// 星座只由公历日期决定，与配置和时辰无关。
///
/// # Errors
/// 日期非法时返回 [`IztroError`]。
pub fn get_sign_by_solar_date(solar_date: &str, language: Language) -> Result<String, IztroError> {
    Ok(sign_chart_by_solar(solar_date, language)?.sign)
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
    Ok(sign_chart_by_lunar(lunar_date, is_leap_month, language)?.sign)
}

/// 命宫主星的取值宫索引：命宫，命宫无主星时借对宫（iztro 同款借宫规则）。
///
/// 译文轨与 key 轨共用这一处，借宫口径改动不会分叉。
fn major_star_source_index(astrolabe: &Astrolabe) -> usize {
    let soul_index = astrolabe
        .palaces
        .iter()
        .find(|p| p.name == Palace::Soul)
        .map_or(0, |p| p.index);
    let has_major = |index: usize| {
        astrolabe.palaces[index]
            .major_stars
            .iter()
            .any(|s| s.star_type == StarType::Major)
    };
    if has_major(soul_index) {
        soul_index
    } else {
        fix_index(soul_index as i32 + 6, 12)
    }
}

/// 命宫主星的语言无关标识列表。
///
/// 借宫规则与 [`major_stars_of_soul_palace`] 完全一致：命宫为空宫时借对宫主星，
/// 对宫也无主星时返回空列表。译文形态接不上知识包，按 key 消费用本函数。
pub fn major_star_keys_of_soul_palace(astrolabe: &Astrolabe) -> Vec<String> {
    astrolabe.palaces[major_star_source_index(astrolabe)]
        .major_stars
        .iter()
        .filter(|s| s.star_type == StarType::Major)
        .map(|s| s.key.as_key().to_string())
        .collect()
}

/// 取命宫主星名，多颗时以逗号分隔。
///
/// 命宫为空宫时借对宫主星，与 iztro 行为一致；对宫也无主星时返回空串。
pub(crate) fn major_stars_of_soul_palace(astrolabe: &Astrolabe) -> String {
    astrolabe.palaces[major_star_source_index(astrolabe)]
        .major_stars
        .iter()
        .filter(|s| s.star_type == StarType::Major)
        .map(|s| s.name.as_str())
        .collect::<Vec<_>>()
        .join(MAJOR_STAR_SEPARATOR)
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
    let astrolabe = major_star_chart_by_solar(solar_date, time_index, fix_leap, language, config)?;
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
    leap: LeapMonth,
    language: Language,
    config: Config,
) -> Result<String, IztroError> {
    let astrolabe = major_star_chart_by_lunar(lunar_date, time_index, leap, language, config)?;
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

//! 自定义四化与亮度表的行为测试。
//!
//! 覆盖三件事：覆盖确实生效、未覆盖的项保持默认、覆盖沿着所有取表路径
//! （安星、飞星、运限、工具函数）一致传播。

use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::utils::{get_brightness, get_mutagen, get_mutagens_by_heavenly_stem};
use x_iztro::{Astrolabe, by_solar, get_horoscope};

/// 2000 年为庚辰年，默认庚干四化：太阳禄、武曲权、太阴科、天同忌。
const BIRTH: &str = "2000-8-16";

/// 取 iztro 文档给出的另一派庚干四化：太阳、武曲、天同、天相。
fn custom_geng() -> Config {
    Config::default().with_mutagens(
        HeavenlyStem::Geng,
        [
            StarKey::TaiyangMaj,
            StarKey::WuquMaj,
            StarKey::TiantongMaj,
            StarKey::TianxiangMaj,
        ],
    )
}

/// 找出盘上带指定四化的星。
fn stars_with_mutagen(chart: &Astrolabe, mutagen: Mutagen) -> Vec<StarKey> {
    chart
        .palaces
        .iter()
        .flat_map(|p| p.major_stars.iter().chain(p.minor_stars.iter()))
        .filter(|s| s.mutagen == Some(mutagen))
        .map(|s| s.key)
        .collect()
}

#[test]
fn test_default_config_unchanged() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();

    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Ke),
        [StarKey::TaiyinMaj]
    );
    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Ji),
        [StarKey::TiantongMaj]
    );
}

#[test]
fn test_custom_mutagens_apply_to_natal_stars() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        custom_geng(),
    )
    .unwrap();

    // 天同由默认的化忌变为化科
    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Ke),
        [StarKey::TiantongMaj]
    );
    // 太阴不再化科
    assert!(!stars_with_mutagen(&chart, Mutagen::Ke).contains(&StarKey::TaiyinMaj));
    // 天相接替化忌
    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Ji),
        [StarKey::TianxiangMaj]
    );
    // 未改动的禄、权保持原状
    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Lu),
        [StarKey::TaiyangMaj]
    );
    assert_eq!(
        stars_with_mutagen(&chart, Mutagen::Quan),
        [StarKey::WuquMaj]
    );
}

#[test]
fn test_custom_mutagens_do_not_leak_to_other_stems() {
    let config = custom_geng();

    // 被覆盖的庚干走自定义表
    assert_eq!(
        get_mutagens_by_heavenly_stem(HeavenlyStem::Geng, &config)[3],
        StarKey::TianxiangMaj
    );
    // 其余天干仍是默认表：壬干化忌为武曲
    assert_eq!(
        get_mutagens_by_heavenly_stem(HeavenlyStem::Ren, &config),
        get_mutagens_by_heavenly_stem(HeavenlyStem::Ren, &Config::default())
    );
    assert_eq!(
        get_mutagen(StarKey::TiantongMaj, HeavenlyStem::Geng, &config),
        Some(Mutagen::Ke)
    );
    assert_eq!(
        get_mutagen(StarKey::TiantongMaj, HeavenlyStem::Geng, &Config::default()),
        Some(Mutagen::Ji)
    );
}

#[test]
fn test_custom_mutagens_apply_to_flying_stars() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        custom_geng(),
    )
    .unwrap();

    // 找一个宫干为庚的宫位，它的四化星应取自自定义表
    let palace = chart
        .palaces
        .iter()
        .find(|p| p.heavenly_stem == HeavenlyStem::Geng)
        .expect("该盘应有宫干为庚的宫位");

    assert_eq!(
        palace.mutagen_stars(&[Mutagen::Ji]),
        vec![StarKey::TianxiangMaj]
    );

    // 同一宫在默认配置下化忌为天同
    let default_chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();
    let default_palace = &default_chart.palaces[palace.index];
    assert_eq!(
        default_palace.mutagen_stars(&[Mutagen::Ji]),
        vec![StarKey::TiantongMaj]
    );
}

#[test]
fn test_custom_mutagens_apply_to_horoscope() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        custom_geng(),
    )
    .unwrap();
    // 2030 年为庚戌年，流年干为庚
    let horoscope = get_horoscope(&chart, "2030-6-1", 0, Language::ZhCN).unwrap();

    assert_eq!(
        horoscope.yearly.base.mutagen,
        vec![
            StarKey::TaiyangMaj,
            StarKey::WuquMaj,
            StarKey::TiantongMaj,
            StarKey::TianxiangMaj
        ],
        "流年干为庚时应取自定义四化表"
    );
}

#[test]
fn test_custom_brightness_applies_and_is_scoped() {
    // 把贪狼十二宫亮度全设为「旺」
    let config =
        Config::default().with_brightness(StarKey::TanlangMaj, [Some(Brightness::Wang); 12]);
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        config.clone(),
    )
    .unwrap();

    let tanlang = chart.star(StarKey::TanlangMaj).expect("贪狼应在盘上");
    assert_eq!(tanlang.brightness, Some(Brightness::Wang));

    // 工具函数在任意宫位都返回「旺」
    for index in 0..12 {
        assert_eq!(
            get_brightness(StarKey::TanlangMaj, index, &config),
            Some(Brightness::Wang)
        );
    }

    // 未覆盖的星保持默认：紫微的亮度与默认配置一致
    let default_chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();
    let ziwei = chart.star(StarKey::ZiweiMaj).unwrap();
    let default_ziwei = default_chart.star(StarKey::ZiweiMaj).unwrap();
    assert_eq!(ziwei.brightness, default_ziwei.brightness);
}

#[test]
fn test_overrides_absent_from_serialized_dto() {
    let config = custom_geng();
    let json = x_iztro::by_solar_json(BIRTH, 2, Gender::Female, true, Language::ZhCN, config)
        .expect("排盘应成功");

    // DTO 的 config 只含五个开关，自定义表不进序列化契约
    assert!(json.contains("\"algorithm\":\"default\""));
    assert!(!json.contains("overrides"));
    assert!(!json.contains("mutagens"));
}

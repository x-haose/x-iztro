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
        horoscope.yearly.mutagen,
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

/// 庚干四槽之外加天相化忌：天同、天相同化忌。
fn geng_double_ji() -> Config {
    Config::default().with_extra_mutagens(HeavenlyStem::Geng, Mutagen::Ji, [StarKey::TianxiangMaj])
}

#[test]
fn test_extra_mutagens_mark_natal_stars() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        geng_double_ji(),
    )
    .unwrap();

    let mut ji = stars_with_mutagen(&chart, Mutagen::Ji);
    ji.sort_by_key(|k| k.as_key());
    assert_eq!(ji, [StarKey::TiantongMaj, StarKey::TianxiangMaj]);
    // 四槽不变：DTO 的四化数组与工具函数仍是四颗
    assert_eq!(
        get_mutagens_by_heavenly_stem(HeavenlyStem::Geng, &chart.config)[3],
        StarKey::TiantongMaj
    );
    assert_eq!(
        get_mutagen(StarKey::TianxiangMaj, HeavenlyStem::Geng, &chart.config),
        Some(Mutagen::Ji)
    );
}

#[test]
fn test_extra_mutagens_apply_to_flying_horoscope_and_text() {
    let chart = by_solar(
        BIRTH,
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        geng_double_ji(),
    )
    .unwrap();
    let tianxiang = chart.star(StarKey::TianxiangMaj).unwrap().palace().index;
    let tiantong = chart.star(StarKey::TiantongMaj).unwrap().palace().index;
    assert_ne!(tianxiang, tiantong, "取样盘的天同、天相不同宫");

    // 宫干飞化：庚干宫的化忌同时飞入天同、天相两宫，四槽之外的星排在同类四槽之星之后
    let geng = chart
        .palaces
        .iter()
        .find(|p| p.heavenly_stem == HeavenlyStem::Geng)
        .unwrap();
    assert_eq!(
        geng.mutagen_stars(&[Mutagen::Ji]),
        [StarKey::TiantongMaj, StarKey::TianxiangMaj]
    );
    assert!(geng.flies_to(&chart.palaces[tianxiang], &[Mutagen::Ji]));
    assert!(geng.flies_to(&chart.palaces[tiantong], &[Mutagen::Ji]));
    let places = geng.mutaged_places(&chart.palaces);
    assert_eq!(places.len(), 4, "四化落宫只按四槽");
    assert_eq!(places[3], Some(tiantong));

    // 运限：2030 年为庚戌年，流年四槽不变，附加星另列
    let h = chart.horoscope("2030-6-1", 0).unwrap();
    assert_eq!(h.yearly.mutagen[3], StarKey::TiantongMaj);
    assert_eq!(h.yearly.extra_mutagen[3], [StarKey::TianxiangMaj]);
    let tianxiang_name = chart.palaces[tianxiang].name;
    let yearly_name = h.yearly.palace_names[tianxiang];
    assert!(
        h.has_horoscope_mutagen(yearly_name, Scope::Yearly, Mutagen::Ji)
            .unwrap()
    );
    assert!(
        h.patterns(Scope::Yearly).is_ok(),
        "附加星参与运限视角的格局判定"
    );
    let view = x_iztro::pattern::view::ChartView::at(
        &chart,
        h.data(),
        Scope::Yearly,
        &x_iztro::PatternConfig::default(),
    );
    let star = view.find(tianxiang, StarKey::TianxiangMaj).unwrap();
    assert_eq!(view.mutagen_of(star), Some(Mutagen::Ji));

    // DTO：四槽数组原样，附加星只在新增的键里
    let dto = serde_json::to_value(h.data().to_dto(Language::ZhCN)).unwrap();
    assert_eq!(
        dto["yearly"]["mutagenStarKeys"].as_array().unwrap().len(),
        4
    );
    assert_eq!(
        dto["yearly"]["extraMutagenStarKeys"],
        serde_json::json!({"sihuaJi": ["tianxiangMaj"]})
    );
    if h.decadal.heavenly_stem != HeavenlyStem::Geng {
        assert!(dto["decadal"].get("extraMutagenStarKeys").is_none());
    }
    let chart_dto = serde_json::to_value(chart.to_dto()).unwrap();
    assert_eq!(
        chart_dto["palaces"][geng.index]["extraMutagenStarKeys"],
        serde_json::json!({"sihuaJi": ["tianxiangMaj"]})
    );

    // 文本：生年四化行与流年四化行都写出天相化忌
    let natal_text = chart.to_text();
    let birth_line = natal_text
        .lines()
        .find(|l| l.starts_with("- 生年四化: "))
        .unwrap();
    assert!(
        birth_line.contains("天同化忌") && birth_line.contains("天相化忌"),
        "{birth_line}"
    );
    let palace_text = chart.palace(tianxiang_name).unwrap().to_text();
    assert!(palace_text.contains("天相(庙)化忌"), "{palace_text}");
    let h_text = h.to_text();
    assert!(h_text.contains("天相化忌"), "{h_text}");
}

#[test]
fn test_extra_mutagens_rejected_when_star_repeats_in_stem() {
    // 天同已在庚干四槽化忌，再列为附加的化禄星即重复
    let config = Config::default().with_extra_mutagens(
        HeavenlyStem::Geng,
        Mutagen::Lu,
        [StarKey::TiantongMaj],
    );
    let err = by_solar(BIRTH, 2, Gender::Female, true, Language::ZhCN, config).unwrap_err();
    assert!(
        matches!(err, x_iztro::IztroError::InvalidArgument(_)),
        "{err}"
    );
    assert!(err.to_string().contains("tiantongMaj"), "{err}");
}

#[test]
fn test_custom_brightness_applies_to_adjective_stars() {
    // 红鸾子宫（索引 10）为庙，其余各宫为闲
    let mut table = [Some(Brightness::Xian2); 12];
    table[10] = Some(Brightness::Miao);
    let config = Config::default().with_brightness(StarKey::Hongluan, table);
    let chart = by_solar(BIRTH, 2, Gender::Female, true, Language::ZhCN, config).unwrap();
    let hongluan = chart.star(StarKey::Hongluan).unwrap();
    let index = hongluan.palace().index;
    let want = table[index].unwrap();
    assert_eq!(hongluan.brightness, Some(want));

    let dto = serde_json::to_value(chart.to_dto()).unwrap();
    let star = dto["palaces"][index]["adjectiveStars"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["key"] == "hongluan")
        .unwrap();
    assert_eq!(star["brightnessKey"], want.as_key());
    assert_eq!(
        star["brightness"],
        x_iztro::translate_brightness(want, Language::ZhCN)
    );

    // 默认配置下杂耀不带亮度键
    let default_dto = serde_json::to_value(
        by_solar(
            BIRTH,
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap()
        .to_dto(),
    )
    .unwrap();
    for p in default_dto["palaces"].as_array().unwrap() {
        for s in p["adjectiveStars"].as_array().unwrap() {
            assert!(s.get("brightness").is_none() && s.get("brightnessKey").is_none());
        }
    }
}

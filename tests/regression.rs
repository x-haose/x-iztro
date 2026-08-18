//! 单张固定命盘的全接口面回归测试：对照 JS iztro 的同参数输出。
//!
//! 数据由 tests/golden/generate_regression.mjs 生成（命盘 2000-8-16 时辰 2
//! 女命，运限目标 2025-1-1 时辰 0），覆盖工具函数、命盘顶层字段与十二宫、
//! 宫位与三方四正查询方法、运限六层级与运限查询方法。

use serde_json::Value;
use x_iztro::data::heavenly_stems::get_heavenly_stem_info;
use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::i18n::{translate_five_elements_class, translate_star};
use x_iztro::utils::{fix_index, get_age_index, time_to_index};
use x_iztro::{by_solar, get_horoscope};

static EXPECTED: &str = include_str!("golden/regression_data.json");

fn load_json() -> Value {
    serde_json::from_str(EXPECTED).expect("Failed to parse regression_data.json")
}

// ============================================================
// Helper functions: Chinese string -> Rust enum
// ============================================================

fn parse_palace(s: &str) -> Palace {
    match s {
        "命宫" => Palace::Soul,
        "父母" => Palace::Parents,
        "福德" => Palace::Spirit,
        "田宅" => Palace::Property,
        "官禄" => Palace::Career,
        "仆役" | "交友" => Palace::Friends,
        "迁移" => Palace::Surface,
        "疾厄" => Palace::Health,
        "财帛" => Palace::Wealth,
        "子女" => Palace::Children,
        "夫妻" => Palace::Spouse,
        "兄弟" => Palace::Siblings,
        _ => panic!("Unknown palace: {}", s),
    }
}

fn parse_heavenly_stem(s: &str) -> HeavenlyStem {
    match s {
        "甲" => HeavenlyStem::Jia,
        "乙" => HeavenlyStem::Yi,
        "丙" => HeavenlyStem::Bing,
        "丁" => HeavenlyStem::Ding,
        "戊" => HeavenlyStem::Wu,
        "己" => HeavenlyStem::Ji,
        "庚" => HeavenlyStem::Geng,
        "辛" => HeavenlyStem::Xin,
        "壬" => HeavenlyStem::Ren,
        "癸" => HeavenlyStem::Gui,
        _ => panic!("Unknown heavenly stem: {}", s),
    }
}

fn parse_earthly_branch(s: &str) -> EarthlyBranch {
    match s {
        "子" => EarthlyBranch::Zi,
        "丑" => EarthlyBranch::Chou,
        "寅" => EarthlyBranch::Yin,
        "卯" => EarthlyBranch::Mao,
        "辰" => EarthlyBranch::Chen,
        "巳" => EarthlyBranch::Si,
        "午" => EarthlyBranch::Wu,
        "未" => EarthlyBranch::Wei,
        "申" => EarthlyBranch::Shen,
        "酉" => EarthlyBranch::You,
        "戌" => EarthlyBranch::Xu,
        "亥" => EarthlyBranch::Hai,
        _ => panic!("Unknown earthly branch: {}", s),
    }
}

// ============================================================
// Helper: create the standard astrolabe from JSON params
// ============================================================

fn make_astrolabe() -> x_iztro::Astrolabe {
    by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap()
}

// ============================================================
// Test 1: Utility functions
// ============================================================

#[test]
fn test_utils() {
    let json = load_json();
    let util = &json["util"];

    // time_to_index
    let tti = &util["time_to_index"];
    for (hour_str, expected) in tti.as_object().unwrap() {
        let hour: u8 = hour_str.parse().unwrap();
        let exp: u8 = expected.as_u64().unwrap() as u8;
        assert_eq!(
            time_to_index(hour),
            exp,
            "time_to_index({}) should be {}",
            hour,
            exp
        );
    }

    // fix_index
    let fi = &util["fix_index"];
    for (key, expected) in fi.as_object().unwrap() {
        let parts: Vec<&str> = key.split('_').collect();
        let index: i32 = parts[0].parse().unwrap();
        let max: usize = parts[1].parse().unwrap();
        let exp = expected.as_u64().unwrap() as usize;
        assert_eq!(
            fix_index(index, max as i32),
            exp,
            "fix_index({}, {}) should be {}",
            index,
            max,
            exp
        );
    }

    // get_age_index
    let gai = &util["get_age_index"];
    for (branch_str, expected) in gai.as_object().unwrap() {
        let branch = parse_earthly_branch(branch_str);
        let exp = expected.as_u64().unwrap() as usize;
        assert_eq!(
            get_age_index(branch),
            exp,
            "get_age_index({}) should be {}",
            branch_str,
            exp
        );
    }
}

// ============================================================
// Test 2: Mutagens by heavenly stem
// ============================================================

#[test]
fn test_mutagens_by_heavenly_stem() {
    let json = load_json();
    let data = &json["util"]["get_mutagens_by_heavenly_stem"];

    for (stem_str, expected_arr) in data.as_object().unwrap() {
        let stem = parse_heavenly_stem(stem_str);
        let info = get_heavenly_stem_info(stem);
        let expected: Vec<&str> = expected_arr
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();

        for (i, exp_name) in expected.iter().enumerate() {
            let actual_name = translate_star(info.mutagen[i], Language::ZhCN);
            assert_eq!(
                actual_name, *exp_name,
                "Stem {} mutagen[{}]: expected {}, got {}",
                stem_str, i, exp_name, actual_name
            );
        }
    }
}

// ============================================================
// Test 3: Astrolabe basic fields
// ============================================================

#[test]
fn test_astrolabe_basic() {
    let json = load_json();
    let expected = &json["astrolabe"];
    let astrolabe = make_astrolabe();

    assert_eq!(astrolabe.gender, Gender::Female);
    assert_eq!(
        astrolabe.solar_date,
        expected["solar_date"].as_str().unwrap()
    );
    assert_eq!(
        astrolabe.lunar_date,
        expected["lunar_date"].as_str().unwrap()
    );
    assert_eq!(
        astrolabe.chinese_date,
        expected["chinese_date"].as_str().unwrap()
    );
    assert_eq!(astrolabe.time, expected["time"].as_str().unwrap());
    assert_eq!(
        astrolabe.time_range,
        expected["time_range"].as_str().unwrap()
    );
    assert_eq!(astrolabe.sign, expected["sign"].as_str().unwrap());
    assert_eq!(astrolabe.zodiac, expected["zodiac"].as_str().unwrap());

    // Soul and body star names
    let expected_soul = expected["soul"].as_str().unwrap();
    let actual_soul = translate_star(astrolabe.soul, Language::ZhCN);
    assert_eq!(actual_soul, expected_soul, "Soul star mismatch");

    let expected_body = expected["body"].as_str().unwrap();
    let actual_body = translate_star(astrolabe.body, Language::ZhCN);
    assert_eq!(actual_body, expected_body, "Body star mismatch");

    // Five elements class
    let expected_fec = expected["five_elements_class"].as_str().unwrap();
    let actual_fec = translate_five_elements_class(astrolabe.five_elements_class, Language::ZhCN);
    assert_eq!(actual_fec, expected_fec, "Five elements class mismatch");

    // Earthly branches of soul/body palace
    let expected_soul_branch =
        parse_earthly_branch(expected["earthly_branch_of_soul_palace"].as_str().unwrap());
    assert_eq!(
        astrolabe.earthly_branch_of_soul_palace, expected_soul_branch,
        "Soul palace earthly branch mismatch"
    );

    let expected_body_branch =
        parse_earthly_branch(expected["earthly_branch_of_body_palace"].as_str().unwrap());
    assert_eq!(
        astrolabe.earthly_branch_of_body_palace, expected_body_branch,
        "Body palace earthly branch mismatch"
    );

    // Palace count
    let expected_count = expected["palaces_count"].as_u64().unwrap() as usize;
    assert_eq!(
        astrolabe.palaces.len(),
        expected_count,
        "Palace count mismatch"
    );
}

// ============================================================
// Test 4: Astrolabe palaces
// ============================================================

#[test]
fn test_astrolabe_palaces() {
    let json = load_json();
    let expected_palaces = json["astrolabe"]["palaces"].as_array().unwrap();
    let astrolabe = make_astrolabe();

    for exp in expected_palaces {
        let idx = exp["index"].as_u64().unwrap() as usize;
        let palace = astrolabe.palace(idx).unwrap();

        // Name
        let expected_name = parse_palace(exp["name"].as_str().unwrap());
        assert_eq!(palace.name, expected_name, "Palace {} name mismatch", idx);

        // Index
        assert_eq!(palace.index, idx, "Palace {} index mismatch", idx);

        // Heavenly stem
        let expected_stem = parse_heavenly_stem(exp["heavenly_stem"].as_str().unwrap());
        assert_eq!(
            palace.heavenly_stem, expected_stem,
            "Palace {} heavenly_stem mismatch",
            idx
        );

        // Earthly branch
        let expected_branch = parse_earthly_branch(exp["earthly_branch"].as_str().unwrap());
        assert_eq!(
            palace.earthly_branch, expected_branch,
            "Palace {} earthly_branch mismatch",
            idx
        );

        // Major stars count
        let expected_major = exp["major_stars_count"].as_u64().unwrap() as usize;
        let actual_major = palace
            .major_stars
            .iter()
            .filter(|s| s.star_type == StarType::Major)
            .count();
        assert_eq!(
            actual_major, expected_major,
            "Palace {} major_stars_count: expected {}, got {}",
            idx, expected_major, actual_major
        );

        // Minor stars count
        let expected_minor = exp["minor_stars_count"].as_u64().unwrap() as usize;
        assert_eq!(
            palace.minor_stars.len(),
            expected_minor,
            "Palace {} minor_stars_count mismatch",
            idx
        );

        // Adjective stars count
        let expected_adj = exp["adjective_stars_count"].as_u64().unwrap() as usize;
        assert_eq!(
            palace.adjective_stars.len(),
            expected_adj,
            "Palace {} adjective_stars_count mismatch",
            idx
        );

        // Body / original palace flags
        assert_eq!(
            palace.is_body_palace,
            exp["is_body_palace"].as_bool().unwrap(),
            "Palace {} is_body_palace mismatch",
            idx
        );
        assert_eq!(
            palace.is_original_palace,
            exp["is_original_palace"].as_bool().unwrap(),
            "Palace {} is_original_palace mismatch",
            idx
        );

        // Decadal range
        let expected_range = exp["decadal_range"].as_array().unwrap();
        let exp_start = expected_range[0].as_u64().unwrap() as u32;
        let exp_end = expected_range[1].as_u64().unwrap() as u32;
        assert_eq!(
            palace.decadal.range,
            (exp_start, exp_end),
            "Palace {} decadal range mismatch",
            idx
        );

        // Decadal heavenly stem
        let exp_dec_stem = parse_heavenly_stem(exp["decadal_heavenly_stem"].as_str().unwrap());
        assert_eq!(
            palace.decadal.heavenly_stem, exp_dec_stem,
            "Palace {} decadal heavenly_stem mismatch",
            idx
        );

        // Decadal earthly branch
        let exp_dec_branch = parse_earthly_branch(exp["decadal_earthly_branch"].as_str().unwrap());
        assert_eq!(
            palace.decadal.earthly_branch, exp_dec_branch,
            "Palace {} decadal earthly_branch mismatch",
            idx
        );
    }
}

// ============================================================
// Test 5: Palace queries (has/not_have/has_one_of/has_mutagen/is_empty/flies_to/self_mutaged)
// ============================================================

#[test]
fn test_palace_queries() {
    let json = load_json();
    let palace_data = &json["palace"];
    let astrolabe = make_astrolabe();

    for i in 0..12 {
        let key = i.to_string();
        let exp = &palace_data[&key];
        let palace = astrolabe.palace(i).unwrap();

        // Verify name and index
        let expected_name = parse_palace(exp["name"].as_str().unwrap());
        assert_eq!(palace.name, expected_name, "Palace {} name mismatch", i);
        assert_eq!(palace.index, exp["index"].as_u64().unwrap() as usize);

        // has(武曲)
        let exp_has_wuqu = exp["has_武曲"].as_bool().unwrap();
        assert_eq!(
            palace.has(&[StarKey::WuquMaj]),
            exp_has_wuqu,
            "Palace {} has(武曲) mismatch",
            i
        );

        // has(紫微)
        let exp_has_ziwei = exp["has_紫微"].as_bool().unwrap();
        assert_eq!(
            palace.has(&[StarKey::ZiweiMaj]),
            exp_has_ziwei,
            "Palace {} has(紫微) mismatch",
            i
        );

        // not_have(紫微)
        let exp_not_have = exp["not_have_紫微"].as_bool().unwrap();
        assert_eq!(
            palace.not_have(&[StarKey::ZiweiMaj]),
            exp_not_have,
            "Palace {} not_have(紫微) mismatch",
            i
        );

        // has_one_of(武曲, 紫微)
        let exp_has_one_of = exp["has_one_of_武曲_紫微"].as_bool().unwrap();
        assert_eq!(
            palace.has_one_of(&[StarKey::WuquMaj, StarKey::ZiweiMaj]),
            exp_has_one_of,
            "Palace {} has_one_of(武曲, 紫微) mismatch",
            i
        );

        // has_mutagen tests
        let exp_lu = exp["has_mutagen_禄"].as_bool().unwrap();
        assert_eq!(
            palace.has_mutagen(Mutagen::Lu),
            exp_lu,
            "Palace {} has_mutagen(禄) mismatch",
            i
        );

        let exp_quan = exp["has_mutagen_权"].as_bool().unwrap();
        assert_eq!(
            palace.has_mutagen(Mutagen::Quan),
            exp_quan,
            "Palace {} has_mutagen(权) mismatch",
            i
        );

        let exp_ke = exp["has_mutagen_科"].as_bool().unwrap();
        assert_eq!(
            palace.has_mutagen(Mutagen::Ke),
            exp_ke,
            "Palace {} has_mutagen(科) mismatch",
            i
        );

        let exp_ji = exp["has_mutagen_忌"].as_bool().unwrap();
        assert_eq!(
            palace.has_mutagen(Mutagen::Ji),
            exp_ji,
            "Palace {} has_mutagen(忌) mismatch",
            i
        );

        // not_have_mutagen(禄)
        let exp_not_lu = exp["not_have_mutagen_禄"].as_bool().unwrap();
        assert_eq!(
            palace.not_have_mutagen(Mutagen::Lu),
            exp_not_lu,
            "Palace {} not_have_mutagen(禄) mismatch",
            i
        );

        // is_empty
        let exp_empty = exp["is_empty"].as_bool().unwrap();
        assert_eq!(
            palace.is_empty(),
            exp_empty,
            "Palace {} is_empty mismatch",
            i
        );

        // flies_to(6, 禄)
        let exp_flies_6_lu = exp["flies_to_6_禄"].as_bool().unwrap();
        assert_eq!(
            palace.flies_to(6, &[Mutagen::Lu]),
            exp_flies_6_lu,
            "Palace {} flies_to(6, 禄) mismatch",
            i
        );

        // flies_to(0, 权)
        let exp_flies_0_quan = exp["flies_to_0_权"].as_bool().unwrap();
        assert_eq!(
            palace.flies_to(0, &[Mutagen::Quan]),
            exp_flies_0_quan,
            "Palace {} flies_to(0, 权) mismatch",
            i
        );

        // self_mutaged(禄)
        let exp_self_lu = exp["self_mutaged_禄"].as_bool().unwrap();
        assert_eq!(
            palace.self_mutaged(&[Mutagen::Lu]),
            exp_self_lu,
            "Palace {} self_mutaged(禄) mismatch",
            i
        );

        // self_mutaged_one_of：空列表与实列表两种语义
        let exp_self_one_empty = exp["self_mutaged_one_of_empty"].as_bool().unwrap();
        assert_eq!(
            palace.self_mutaged_one_of(&[]),
            exp_self_one_empty,
            "Palace {} self_mutaged_one_of([]) mismatch",
            i
        );
        let exp_self_one = exp["self_mutaged_one_of_禄权"].as_bool().unwrap();
        assert_eq!(
            palace.self_mutaged_one_of(&[Mutagen::Lu, Mutagen::Quan]),
            exp_self_one,
            "Palace {} self_mutaged_one_of(禄, 权) mismatch",
            i
        );

        // not_self_mutaged：空列表与实列表两种语义
        let exp_not_self_empty = exp["not_self_mutaged_empty"].as_bool().unwrap();
        assert_eq!(
            palace.not_self_mutaged(&[]),
            exp_not_self_empty,
            "Palace {} not_self_mutaged([]) mismatch",
            i
        );
        let exp_not_self = exp["not_self_mutaged_禄权"].as_bool().unwrap();
        assert_eq!(
            palace.not_self_mutaged(&[Mutagen::Lu, Mutagen::Quan]),
            exp_not_self,
            "Palace {} not_self_mutaged(禄, 权) mismatch",
            i
        );

        // mutaged_places length
        let exp_mp_len = exp["mutaged_places_length"].as_u64().unwrap() as usize;
        let mp = palace.mutaged_places();
        assert_eq!(
            mp.len(),
            exp_mp_len,
            "Palace {} mutaged_places length mismatch",
            i
        );
    }

    // Test palace_by_name
    let by_name = &palace_data["by_name"];
    for (name_str, exp) in by_name.as_object().unwrap() {
        let palace_enum = parse_palace(name_str);
        let palace = astrolabe.palace(palace_enum);
        assert!(
            palace.is_some(),
            "palace_by_name({}) should return Some",
            name_str
        );
        let p = palace.unwrap();
        let expected_name = parse_palace(exp["name"].as_str().unwrap());
        assert_eq!(p.name, expected_name);
        assert_eq!(p.index, exp["index"].as_u64().unwrap() as usize);
    }
}

// ============================================================
// Test 6: Surrounded palaces
// ============================================================

#[test]
fn test_surrounded_palaces() {
    let json = load_json();
    let surround_data = &json["surround"];
    let astrolabe = make_astrolabe();

    for i in 0..12 {
        let key = i.to_string();
        let exp = &surround_data[&key];
        let sp = astrolabe.surrounded_palaces(i).unwrap();

        // Target
        let exp_target_name = parse_palace(exp["target_name"].as_str().unwrap());
        assert_eq!(
            sp.target.name, exp_target_name,
            "Surround {} target name mismatch",
            i
        );
        assert_eq!(
            sp.target.index,
            exp["target_index"].as_u64().unwrap() as usize,
            "Surround {} target index mismatch",
            i
        );

        // Opposite
        let exp_opposite_name = parse_palace(exp["opposite_name"].as_str().unwrap());
        assert_eq!(
            sp.opposite.name, exp_opposite_name,
            "Surround {} opposite name mismatch",
            i
        );
        assert_eq!(
            sp.opposite.index,
            exp["opposite_index"].as_u64().unwrap() as usize,
            "Surround {} opposite index mismatch",
            i
        );

        // Wealth
        let exp_wealth_name = parse_palace(exp["wealth_name"].as_str().unwrap());
        assert_eq!(
            sp.wealth.name, exp_wealth_name,
            "Surround {} wealth name mismatch",
            i
        );
        assert_eq!(
            sp.wealth.index,
            exp["wealth_index"].as_u64().unwrap() as usize,
            "Surround {} wealth index mismatch",
            i
        );

        // Career
        let exp_career_name = parse_palace(exp["career_name"].as_str().unwrap());
        assert_eq!(
            sp.career.name, exp_career_name,
            "Surround {} career name mismatch",
            i
        );
        assert_eq!(
            sp.career.index,
            exp["career_index"].as_u64().unwrap() as usize,
            "Surround {} career index mismatch",
            i
        );

        // have(紫微)
        let exp_have = exp["have_紫微"].as_bool().unwrap();
        assert_eq!(
            sp.have(&[StarKey::ZiweiMaj]),
            exp_have,
            "Surround {} have(紫微) mismatch",
            i
        );

        // not_have(紫微)
        let exp_not_have = exp["not_have_紫微"].as_bool().unwrap();
        assert_eq!(
            sp.not_have(&[StarKey::ZiweiMaj]),
            exp_not_have,
            "Surround {} not_have(紫微) mismatch",
            i
        );

        // have_one_of(武曲, 紫微)
        let exp_have_one_of = exp["have_one_of_武曲_紫微"].as_bool().unwrap();
        assert_eq!(
            sp.have_one_of(&[StarKey::WuquMaj, StarKey::ZiweiMaj]),
            exp_have_one_of,
            "Surround {} have_one_of(武曲, 紫微) mismatch",
            i
        );

        // have_mutagen
        let exp_have_lu = exp["have_mutagen_禄"].as_bool().unwrap();
        assert_eq!(
            sp.have_mutagen(Mutagen::Lu),
            exp_have_lu,
            "Surround {} have_mutagen(禄) mismatch",
            i
        );

        let exp_have_quan = exp["have_mutagen_权"].as_bool().unwrap();
        assert_eq!(
            sp.have_mutagen(Mutagen::Quan),
            exp_have_quan,
            "Surround {} have_mutagen(权) mismatch",
            i
        );

        // not_have_mutagen
        let exp_not_lu = exp["not_have_mutagen_禄"].as_bool().unwrap();
        assert_eq!(
            sp.not_have_mutagen(Mutagen::Lu),
            exp_not_lu,
            "Surround {} not_have_mutagen(禄) mismatch",
            i
        );
    }
}

// ============================================================
// Test 7: Horoscope
// ============================================================

#[test]
fn test_horoscope() {
    let json = load_json();
    let exp = &json["horoscope"];
    let astrolabe = make_astrolabe();
    let horoscope = get_horoscope(&astrolabe, "2025-1-1", 0, Language::ZhCN).unwrap();

    // Basic dates
    assert_eq!(horoscope.lunar_date, exp["lunar_date"].as_str().unwrap());
    assert_eq!(horoscope.solar_date, exp["solar_date"].as_str().unwrap());

    // Decadal
    assert_eq!(
        horoscope.decadal.index,
        exp["decadal_index"].as_u64().unwrap() as usize,
        "Decadal index mismatch"
    );
    let exp_dec_stem = parse_heavenly_stem(exp["decadal_heavenly_stem"].as_str().unwrap());
    assert_eq!(
        horoscope.decadal.heavenly_stem, exp_dec_stem,
        "Decadal stem mismatch"
    );
    let exp_dec_branch = parse_earthly_branch(exp["decadal_earthly_branch"].as_str().unwrap());
    assert_eq!(
        horoscope.decadal.earthly_branch, exp_dec_branch,
        "Decadal branch mismatch"
    );

    // Decadal mutagen
    let exp_dec_mutagen = exp["decadal_mutagen"].as_array().unwrap();
    for (i, exp_star) in exp_dec_mutagen.iter().enumerate() {
        let actual = translate_star(horoscope.decadal.mutagen[i], Language::ZhCN);
        assert_eq!(
            actual,
            exp_star.as_str().unwrap(),
            "Decadal mutagen[{}] mismatch",
            i
        );
    }

    // Age
    assert_eq!(
        horoscope.age.index,
        exp["age_index"].as_u64().unwrap() as usize,
        "Age index mismatch"
    );
    assert_eq!(
        horoscope.age.nominal_age,
        exp["age_nominal_age"].as_u64().unwrap() as u32,
        "Age nominal_age mismatch"
    );

    // Yearly
    assert_eq!(
        horoscope.yearly.index,
        exp["yearly_index"].as_u64().unwrap() as usize,
        "Yearly index mismatch"
    );
    let exp_yearly_mutagen = exp["yearly_mutagen"].as_array().unwrap();
    for (i, exp_star) in exp_yearly_mutagen.iter().enumerate() {
        let actual = translate_star(horoscope.yearly.mutagen[i], Language::ZhCN);
        assert_eq!(
            actual,
            exp_star.as_str().unwrap(),
            "Yearly mutagen[{}] mismatch",
            i
        );
    }

    // Monthly
    assert_eq!(
        horoscope.monthly.index,
        exp["monthly_index"].as_u64().unwrap() as usize,
        "Monthly index mismatch"
    );
    let exp_monthly_mutagen = exp["monthly_mutagen"].as_array().unwrap();
    for (i, exp_star) in exp_monthly_mutagen.iter().enumerate() {
        let actual = translate_star(horoscope.monthly.mutagen[i], Language::ZhCN);
        assert_eq!(
            actual,
            exp_star.as_str().unwrap(),
            "Monthly mutagen[{}] mismatch",
            i
        );
    }

    // Daily
    assert_eq!(
        horoscope.daily.index,
        exp["daily_index"].as_u64().unwrap() as usize,
        "Daily index mismatch"
    );
    let exp_daily_mutagen = exp["daily_mutagen"].as_array().unwrap();
    for (i, exp_star) in exp_daily_mutagen.iter().enumerate() {
        let actual = translate_star(horoscope.daily.mutagen[i], Language::ZhCN);
        assert_eq!(
            actual,
            exp_star.as_str().unwrap(),
            "Daily mutagen[{}] mismatch",
            i
        );
    }

    // Hourly
    assert_eq!(
        horoscope.hourly.index,
        exp["hourly_index"].as_u64().unwrap() as usize,
        "Hourly index mismatch"
    );
    let exp_hourly_mutagen = exp["hourly_mutagen"].as_array().unwrap();
    for (i, exp_star) in exp_hourly_mutagen.iter().enumerate() {
        let actual = translate_star(horoscope.hourly.mutagen[i], Language::ZhCN);
        assert_eq!(
            actual,
            exp_star.as_str().unwrap(),
            "Hourly mutagen[{}] mismatch",
            i
        );
    }

    // age_palace name
    let age_palace = horoscope.age_palace(&astrolabe);
    let exp_age_palace_name = parse_palace(exp["age_palace_name"].as_str().unwrap());
    assert_eq!(
        age_palace.name, exp_age_palace_name,
        "Age palace name mismatch"
    );

    // palace lookups via horoscope.palace()
    // palace_命宫_origin
    let origin_soul = horoscope.palace(Palace::Soul, Scope::Origin, &astrolabe);
    assert!(origin_soul.is_some());
    let exp_origin = parse_palace(exp["palace_命宫_origin"].as_str().unwrap());
    assert_eq!(
        origin_soul.unwrap().name,
        exp_origin,
        "palace(命宫, origin) mismatch"
    );

    // palace_命宫_decadal
    let decadal_soul = horoscope.palace(Palace::Soul, Scope::Decadal, &astrolabe);
    assert!(decadal_soul.is_some());
    let exp_decadal = parse_palace(exp["palace_命宫_decadal"].as_str().unwrap());
    assert_eq!(
        decadal_soul.unwrap().name,
        exp_decadal,
        "palace(命宫, decadal) mismatch"
    );

    // palace_命宫_yearly
    let yearly_soul = horoscope.palace(Palace::Soul, Scope::Yearly, &astrolabe);
    assert!(yearly_soul.is_some());
    let exp_yearly = parse_palace(exp["palace_命宫_yearly"].as_str().unwrap());
    assert_eq!(
        yearly_soul.unwrap().name,
        exp_yearly,
        "palace(命宫, yearly) mismatch"
    );

    // has_horoscope_mutagen tests
    let exp_dec_lu = exp["has_horoscope_mutagen_命宫_decadal_禄"]
        .as_bool()
        .unwrap();
    assert_eq!(
        horoscope.has_horoscope_mutagen(Palace::Soul, Scope::Decadal, Mutagen::Lu, &astrolabe),
        exp_dec_lu,
        "has_horoscope_mutagen(命宫, decadal, 禄) mismatch"
    );

    let exp_yr_lu = exp["has_horoscope_mutagen_命宫_yearly_禄"]
        .as_bool()
        .unwrap();
    assert_eq!(
        horoscope.has_horoscope_mutagen(Palace::Soul, Scope::Yearly, Mutagen::Lu, &astrolabe),
        exp_yr_lu,
        "has_horoscope_mutagen(命宫, yearly, 禄) mismatch"
    );

    // has_horoscope_stars and not_have_horoscope_stars
    // JSON tests with 武曲 for 命宫 decadal scope
    let exp_has_stars = exp["has_horoscope_stars_命宫_decadal"].as_bool().unwrap();
    assert_eq!(
        horoscope.has_horoscope_stars(
            Palace::Soul,
            Scope::Decadal,
            &[StarKey::WuquMaj],
            &astrolabe
        ),
        exp_has_stars,
        "has_horoscope_stars(命宫, decadal, 武曲) mismatch"
    );

    let exp_not_stars = exp["not_have_horoscope_stars_命宫_decadal"]
        .as_bool()
        .unwrap();
    assert_eq!(
        horoscope.not_have_horoscope_stars(
            Palace::Soul,
            Scope::Decadal,
            &[StarKey::WuquMaj],
            &astrolabe
        ),
        exp_not_stars,
        "not_have_horoscope_stars(命宫, decadal, 武曲) mismatch"
    );

    // surround_palaces via horoscope
    let sp = horoscope.surround_palaces(Palace::Soul, Scope::Origin, &astrolabe);
    assert!(
        sp.is_some(),
        "surround_palaces(命宫, origin) should return Some"
    );
    let sp = sp.unwrap();
    let exp_sp_target = parse_palace(exp["surround_palaces_命宫_origin_target"].as_str().unwrap());
    assert_eq!(
        sp.target.name, exp_sp_target,
        "surround_palaces target mismatch"
    );
}

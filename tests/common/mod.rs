//! 金标测试共享工具。
//!
//! 各测试二进制只使用其中一部分函数，未使用部分的 dead_code 告警予以豁免。
#![allow(dead_code)]

use x_iztro::data::types::*;
use x_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use x_iztro::models::astrolabe::Astrolabe;
use x_iztro::models::horoscope::HoroscopeItem;
use x_iztro::models::star::Star;
use x_iztro::{by_solar, get_horoscope};
use serde_json::Value;

const LANG: Language = Language::ZhCN;

/// 星耀条目：`名:亮度:四化`，亮度/四化无则为空串。
fn star_entry(s: &Star) -> String {
    format!(
        "{}:{}:{}",
        s.name,
        s.brightness
            .map(|b| translate_brightness(b, LANG))
            .unwrap_or(""),
        s.mutagen.map(|m| translate_mutagen(m, LANG)).unwrap_or(""),
    )
}

/// 排盘结果的规范化字符串。
///
/// 与 JS 侧 tests/golden/canonical.mjs 的 canonicalAstrolabe() 严格同构：
/// 逐字节一致的字符串经 SHA-256 后即为 tier3 金标哈希。
///
/// 格式：顶层字段 '|' 连接，之后每宫一段，段间 '#' 连接；
/// 辅星与杂耀条目按字符串排序，主星保持安放顺序。
pub fn canonical_astrolabe(a: &Astrolabe) -> String {
    let top = [
        translate_gender(a.gender, LANG).to_string(),
        a.solar_date.clone(),
        a.lunar_date.clone(),
        a.chinese_date.clone(),
        a.time.clone(),
        a.time_range.clone(),
        a.sign.clone(),
        a.zodiac.clone(),
        translate_earthly_branch(a.earthly_branch_of_soul_palace, LANG).to_string(),
        translate_earthly_branch(a.earthly_branch_of_body_palace, LANG).to_string(),
        translate_star(a.soul, LANG).to_string(),
        translate_star(a.body, LANG).to_string(),
        translate_five_elements_class(a.five_elements_class, LANG).to_string(),
    ]
    .join("|");

    let palaces: Vec<String> = a
        .palaces
        .iter()
        .map(|p| {
            let majors: Vec<String> = p.major_stars.iter().map(star_entry).collect();
            let mut minors: Vec<String> = p.minor_stars.iter().map(star_entry).collect();
            minors.sort();
            let mut adjs: Vec<String> = p.adjective_stars.iter().map(|s| s.name.clone()).collect();
            adjs.sort();
            let ages: Vec<String> = p.ages.iter().map(|x| x.to_string()).collect();
            [
                translate_palace(p.name, LANG).to_string(),
                translate_heavenly_stem(p.heavenly_stem, LANG).to_string(),
                translate_earthly_branch(p.earthly_branch, LANG).to_string(),
                if p.is_body_palace { "1" } else { "0" }.to_string(),
                if p.is_original_palace { "1" } else { "0" }.to_string(),
                majors.join(";"),
                minors.join(";"),
                adjs.join(";"),
                translate_star(p.changsheng12, LANG).to_string(),
                translate_star(p.boshi12, LANG).to_string(),
                translate_star(p.jiangqian12, LANG).to_string(),
                translate_star(p.suiqian12, LANG).to_string(),
                format!("{}-{}", p.decadal.range.0, p.decadal.range.1),
                ages.join(","),
            ]
            .join("|")
        })
        .collect();

    format!("{}#{}", top, palaces.join("#"))
}

/// 对照单个运限层级的通用字段（index/name/干支/四化/流耀分布）。
fn check_scope(
    label: &str,
    exp: &Value,
    item: &HoroscopeItem,
    failures: &mut Vec<String>,
    case_label: &str,
) {
    let l = format!("{case_label} {label}");

    let exp_index = exp["i"].as_u64().unwrap() as usize;
    if item.index != exp_index {
        failures.push(format!(
            "{l}: index expected={} actual={}",
            exp_index, item.index
        ));
    }

    let exp_name = exp["n"].as_str().unwrap();
    if item.name != exp_name {
        failures.push(format!(
            "{l}: name expected={} actual={}",
            exp_name, item.name
        ));
    }

    let exp_hs = exp["hs"].as_str().unwrap();
    let act_hs = translate_heavenly_stem(item.heavenly_stem, LANG);
    if act_hs != exp_hs {
        failures.push(format!("{l}: stem expected={exp_hs} actual={act_hs}"));
    }

    let exp_eb = exp["eb"].as_str().unwrap();
    let act_eb = translate_earthly_branch(item.earthly_branch, LANG);
    if act_eb != exp_eb {
        failures.push(format!("{l}: branch expected={exp_eb} actual={act_eb}"));
    }

    let exp_mutagen: Vec<&str> = exp["m"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let act_mutagen: Vec<&str> = item
        .mutagen
        .iter()
        .map(|k| translate_star(*k, LANG))
        .collect();
    if act_mutagen != exp_mutagen {
        failures.push(format!(
            "{l}: mutagen expected={exp_mutagen:?} actual={act_mutagen:?}"
        ));
    }

    if let Some(exp_stars) = exp.get("s").and_then(|v| v.as_array()) {
        let act_stars = item.stars.as_ref().expect("scope stars missing");
        for (pi, exp_group) in exp_stars.iter().enumerate() {
            let exp_names: Vec<&str> = exp_group
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let mut act_names: Vec<&str> = act_stars[pi].iter().map(|s| s.name.as_str()).collect();
            act_names.sort();
            if act_names != exp_names {
                failures.push(format!(
                    "{l} p[{pi}]: stars expected={exp_names:?} actual={act_names:?}"
                ));
            }
        }
    }
}

/// 对照单个运限用例（generate_horoscope.mjs / generate_config.mjs 的紧凑格式）：
/// 按用例参数与给定配置排盘、算运限，逐字段断言六个层级与流年十二神。
pub fn check_horoscope_case(case: &Value, config: Config, failures: &mut Vec<String>) {
    let p = &case["p"];
    let birth_date = p["d"].as_str().unwrap();
    let birth_ti = p["t"].as_u64().unwrap() as u8;
    let gender = if p["g"].as_u64().unwrap() == 0 {
        Gender::Male
    } else {
        Gender::Female
    };
    let target_date = case["td"].as_str().unwrap();
    let target_ti = case["tt"].as_u64().unwrap() as u8;
    let case_label = format!(
        "[{birth_date} t{birth_ti} g{}] -> {target_date} t{target_ti}",
        p["g"]
    );

    let astrolabe = by_solar(birth_date, birth_ti, gender, true, LANG, config).unwrap();
    let h = get_horoscope(&astrolabe, target_date, target_ti, LANG).unwrap();

    let exp_ld = case["ld"].as_str().unwrap();
    if h.lunar_date != exp_ld {
        failures.push(format!(
            "{case_label}: lunar_date expected={exp_ld} actual={}",
            h.lunar_date
        ));
    }

    check_scope("dec", &case["dec"], &h.decadal, failures, &case_label);
    check_scope("age", &case["age"], &h.age.base, failures, &case_label);
    check_scope("yr", &case["yr"], &h.yearly.base, failures, &case_label);
    check_scope("mo", &case["mo"], &h.monthly, failures, &case_label);
    check_scope("da", &case["da"], &h.daily, failures, &case_label);
    check_scope("hr", &case["hr"], &h.hourly, failures, &case_label);

    let exp_na = case["age"]["na"].as_u64().unwrap() as u32;
    if h.age.nominal_age != exp_na {
        failures.push(format!(
            "{case_label}: nominal_age expected={exp_na} actual={}",
            h.age.nominal_age
        ));
    }

    let exp_sq: Vec<&str> = case["yr"]["sq"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let act_sq: Vec<&str> = h
        .yearly
        .yearly_dec_star
        .suiqian12
        .iter()
        .map(|k| translate_star(*k, LANG))
        .collect();
    if act_sq != exp_sq {
        failures.push(format!(
            "{case_label}: suiqian12 expected={exp_sq:?} actual={act_sq:?}"
        ));
    }

    let exp_jq: Vec<&str> = case["yr"]["jq"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    let act_jq: Vec<&str> = h
        .yearly
        .yearly_dec_star
        .jiangqian12
        .iter()
        .map(|k| translate_star(*k, LANG))
        .collect();
    if act_jq != exp_jq {
        failures.push(format!(
            "{case_label}: jiangqian12 expected={exp_jq:?} actual={act_jq:?}"
        ));
    }
}

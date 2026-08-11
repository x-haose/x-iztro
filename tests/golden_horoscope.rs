//! Golden horoscope 测试：运限全字段对照 JS 输出。
//!
//! 数据由 tests/golden/generate_horoscope.mjs 生成：360 个命盘 × 16 个目标
//! 日期（12 流年支、童限、高龄大限、闰月下半月、晚子时目标），共 5760 例。
//! 对照六个运限层级的宫位索引、层级名、干支、四化星名、流耀分布，
//! 以及小限虚岁、流年岁前/将前十二神与目标农历日期。

use rs_iztro::data::types::*;
use rs_iztro::i18n::{translate_earthly_branch, translate_heavenly_stem, translate_star};
use rs_iztro::models::horoscope::HoroscopeItem;
use rs_iztro::{by_solar, get_horoscope};
use serde_json::Value;
use std::fs;

const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/horoscope_data.json");
const LANG: Language = Language::ZhCN;
const MAX_FAILURES: usize = 50;

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
        failures.push(format!("{l}: index expected={} actual={}", exp_index, item.index));
    }

    let exp_name = exp["n"].as_str().unwrap();
    if item.name != exp_name {
        failures.push(format!("{l}: name expected={} actual={}", exp_name, item.name));
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

    let exp_mutagen: Vec<&str> = exp["m"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
    let act_mutagen: Vec<&str> = item.mutagen.iter().map(|k| translate_star(*k, LANG)).collect();
    if act_mutagen != exp_mutagen {
        failures.push(format!("{l}: mutagen expected={exp_mutagen:?} actual={act_mutagen:?}"));
    }

    if let Some(exp_stars) = exp.get("s").and_then(|v| v.as_array()) {
        let act_stars = item.stars.as_ref().expect("scope stars missing");
        for (pi, exp_group) in exp_stars.iter().enumerate() {
            let exp_names: Vec<&str> = exp_group.as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
            let mut act_names: Vec<&str> = act_stars[pi].iter().map(|s| s.name.as_str()).collect();
            act_names.sort();
            if act_names != exp_names {
                failures.push(format!("{l} p[{pi}]: stars expected={exp_names:?} actual={act_names:?}"));
            }
        }
    }
}

#[test]
fn golden_horoscope_full() {
    let data = fs::read_to_string(DATA_PATH)
        .expect("horoscope_data.json missing — run `node tests/golden/generate_horoscope.mjs` first");
    let cases: Vec<Value> = serde_json::from_str(&data).expect("Failed to parse horoscope_data.json");

    let mut failures: Vec<String> = Vec::new();

    for case in &cases {
        let p = &case["p"];
        let birth_date = p["d"].as_str().unwrap();
        let birth_ti = p["t"].as_u64().unwrap() as u8;
        let gender = if p["g"].as_u64().unwrap() == 0 { Gender::Male } else { Gender::Female };
        let target_date = case["td"].as_str().unwrap();
        let target_ti = case["tt"].as_u64().unwrap() as u8;
        let case_label = format!("[{birth_date} t{birth_ti} g{}] -> {target_date} t{target_ti}", p["g"]);

        let astrolabe = by_solar(birth_date, birth_ti, gender, true, LANG, Algorithm::Default);
        let h = get_horoscope(&astrolabe, target_date, target_ti, LANG);

        let exp_ld = case["ld"].as_str().unwrap();
        if h.lunar_date != exp_ld {
            failures.push(format!("{case_label}: lunar_date expected={exp_ld} actual={}", h.lunar_date));
        }

        check_scope("dec", &case["dec"], &h.decadal, &mut failures, &case_label);
        check_scope("age", &case["age"], &h.age.base, &mut failures, &case_label);
        check_scope("yr", &case["yr"], &h.yearly.base, &mut failures, &case_label);
        check_scope("mo", &case["mo"], &h.monthly, &mut failures, &case_label);
        check_scope("da", &case["da"], &h.daily, &mut failures, &case_label);
        check_scope("hr", &case["hr"], &h.hourly, &mut failures, &case_label);

        let exp_na = case["age"]["na"].as_u64().unwrap() as u32;
        if h.age.nominal_age != exp_na {
            failures.push(format!("{case_label}: nominal_age expected={exp_na} actual={}", h.age.nominal_age));
        }

        let exp_sq: Vec<&str> = case["yr"]["sq"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let act_sq: Vec<&str> = h.yearly.yearly_dec_star.suiqian12.iter().map(|k| translate_star(*k, LANG)).collect();
        if act_sq != exp_sq {
            failures.push(format!("{case_label}: suiqian12 expected={exp_sq:?} actual={act_sq:?}"));
        }

        let exp_jq: Vec<&str> = case["yr"]["jq"].as_array().unwrap().iter().map(|v| v.as_str().unwrap()).collect();
        let act_jq: Vec<&str> = h.yearly.yearly_dec_star.jiangqian12.iter().map(|k| translate_star(*k, LANG)).collect();
        if act_jq != exp_jq {
            failures.push(format!("{case_label}: jiangqian12 expected={exp_jq:?} actual={act_jq:?}"));
        }

        if failures.len() >= MAX_FAILURES {
            break;
        }
    }

    if !failures.is_empty() {
        let shown = failures.len().min(MAX_FAILURES);
        let mut msg = format!(
            "\n\nGolden horoscope FAILED: {} failure(s) (showing first {}):\n\n",
            failures.len(),
            shown,
        );
        for (i, f) in failures.iter().take(shown).enumerate() {
            msg.push_str(&format!("  {}. {}\n", i + 1, f));
        }
        panic!("{}", msg);
    }

    eprintln!("Golden horoscope: all {} cases passed!", cases.len());
}

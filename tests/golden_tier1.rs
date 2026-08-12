//! Golden tier-1 tests: compare rs-iztro output against 780 JS-generated cases.
//!
//! Each case covers full astrolabe fields including all 12 palaces with stars,
//! brightness, mutagen, decorative stars, and decadals.
//! All 780 cases (including time_index=12) must match the JS output exactly.

use rs_iztro::by_solar;
use rs_iztro::data::types::*;
use rs_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use serde_json::Value;
use std::collections::BTreeSet;

static GOLDEN_DATA: &str = include_str!("golden/tier1_data.json");
const LANG: Language = Language::ZhCN;
const MAX_FAILURES: usize = 50;

fn load_cases() -> Vec<Value> {
    serde_json::from_str(GOLDEN_DATA).expect("Failed to parse tier1_data.json")
}

fn fmt_brightness(b: Option<Brightness>) -> String {
    match b {
        None => String::new(),
        Some(br) => translate_brightness(br, LANG).to_string(),
    }
}

fn fmt_mutagen(m: Option<Mutagen>) -> String {
    match m {
        None => String::new(),
        Some(mu) => translate_mutagen(mu, LANG).to_string(),
    }
}

fn parse_gender(s: &str) -> Gender {
    match s {
        "男" => Gender::Male,
        "女" => Gender::Female,
        _ => panic!("Unknown gender: {}", s),
    }
}

fn json_str(v: &Value) -> String {
    match v {
        Value::Null => String::new(),
        Value::String(s) => s.clone(),
        _ => panic!("Unexpected value: {:?}", v),
    }
}

/// Run comparison for a single case, appending failures to the provided vec.
fn check_case(case: &Value, case_idx: usize, failures: &mut Vec<String>) {
    let params = &case["params"];
    let solar_date = params["solar_date"].as_str().unwrap();
    let time_index = params["time_index"].as_u64().unwrap() as u8;
    let gender = parse_gender(params["gender"].as_str().unwrap());
    let case_label = format!(
        "case[{}] date={} ti={} g={}",
        case_idx,
        solar_date,
        time_index,
        params["gender"].as_str().unwrap()
    );

    let astrolabe = by_solar(
        solar_date,
        time_index,
        gender,
        true,
        LANG,
        Config::default(),
    )
    .unwrap();

    // --- Top-level fields ---
    let checks: Vec<(&str, String, &str)> = vec![
        (
            "gender",
            translate_gender(astrolabe.gender, LANG).to_string(),
            case["gender"].as_str().unwrap(),
        ),
        (
            "lunar_date",
            astrolabe.lunar_date.clone(),
            case["lunar_date"].as_str().unwrap(),
        ),
        (
            "chinese_date",
            astrolabe.chinese_date.clone(),
            case["chinese_date"].as_str().unwrap(),
        ),
        (
            "time",
            astrolabe.time.clone(),
            case["time"].as_str().unwrap(),
        ),
        (
            "time_range",
            astrolabe.time_range.clone(),
            case["time_range"].as_str().unwrap(),
        ),
        (
            "sign",
            astrolabe.sign.clone(),
            case["sign"].as_str().unwrap(),
        ),
        (
            "zodiac",
            astrolabe.zodiac.clone(),
            case["zodiac"].as_str().unwrap(),
        ),
        (
            "soul_branch",
            translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, LANG).to_string(),
            case["soul_palace_branch"].as_str().unwrap(),
        ),
        (
            "body_branch",
            translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, LANG).to_string(),
            case["body_palace_branch"].as_str().unwrap(),
        ),
        (
            "five_elements",
            translate_five_elements_class(astrolabe.five_elements_class, LANG).to_string(),
            case["five_elements_class"].as_str().unwrap(),
        ),
        (
            "soul_star",
            translate_star(astrolabe.soul, LANG).to_string(),
            case["soul_star"].as_str().unwrap(),
        ),
        (
            "body_star",
            translate_star(astrolabe.body, LANG).to_string(),
            case["body_star"].as_str().unwrap(),
        ),
    ];
    for (field, actual, expected) in &checks {
        if actual != expected {
            failures.push(format!(
                "{}: {} expected={} actual={}",
                case_label, field, expected, actual
            ));
        }
    }

    // --- rawDates ---
    let rl = &case["raw_lunar"];
    let act_rl = &astrolabe.raw_dates.lunar_date;
    if act_rl.lunar_year != rl["y"].as_i64().unwrap()
        || act_rl.lunar_month as u64 != rl["m"].as_u64().unwrap()
        || act_rl.lunar_day as u64 != rl["d"].as_u64().unwrap()
        || act_rl.is_leap != rl["leap"].as_bool().unwrap()
    {
        failures.push(format!(
            "{}: raw_lunar expected={:?} actual={:?}",
            case_label, rl, act_rl
        ));
    }

    let rc = &case["raw_chinese"];
    let act_rc = &astrolabe.raw_dates.chinese_date;
    let pillar = |p: (HeavenlyStem, EarthlyBranch)| {
        format!(
            "{}{}",
            translate_heavenly_stem(p.0, LANG),
            translate_earthly_branch(p.1, LANG)
        )
    };
    let pillar_checks = [
        ("yearly", pillar(act_rc.yearly), rc["y"].as_str().unwrap()),
        ("monthly", pillar(act_rc.monthly), rc["m"].as_str().unwrap()),
        ("daily", pillar(act_rc.daily), rc["d"].as_str().unwrap()),
        ("hourly", pillar(act_rc.hourly), rc["h"].as_str().unwrap()),
    ];
    for (field, actual, expected) in &pillar_checks {
        if actual != expected {
            failures.push(format!(
                "{}: raw_chinese.{} expected={} actual={}",
                case_label, field, expected, actual
            ));
        }
    }

    // --- Palaces ---
    let exp_palaces = case["palaces"].as_array().unwrap();
    if astrolabe.palaces.len() != exp_palaces.len() {
        failures.push(format!(
            "{}: palace count expected={} actual={}",
            case_label,
            exp_palaces.len(),
            astrolabe.palaces.len()
        ));
        return;
    }

    for (pi, exp_palace) in exp_palaces.iter().enumerate() {
        let act_palace = &astrolabe.palaces[pi];
        let pl = format!("{} p[{}]", case_label, pi);

        // Palace name
        let exp_name = exp_palace["name"].as_str().unwrap();
        let act_name = translate_palace(act_palace.name, LANG);
        if act_name != exp_name {
            failures.push(format!(
                "{}: name expected={} actual={}",
                pl, exp_name, act_name
            ));
        }

        // Body palace / original palace flags
        let exp_body = exp_palace["is_body_palace"].as_bool().unwrap();
        if act_palace.is_body_palace != exp_body {
            failures.push(format!(
                "{}: is_body_palace expected={} actual={}",
                pl, exp_body, act_palace.is_body_palace
            ));
        }
        let exp_orig = exp_palace["is_original_palace"].as_bool().unwrap();
        if act_palace.is_original_palace != exp_orig {
            failures.push(format!(
                "{}: is_original_palace expected={} actual={}",
                pl, exp_orig, act_palace.is_original_palace
            ));
        }

        // Heavenly stem & earthly branch
        let exp_stem = exp_palace["heavenly_stem"].as_str().unwrap();
        let act_stem = translate_heavenly_stem(act_palace.heavenly_stem, LANG);
        if act_stem != exp_stem {
            failures.push(format!(
                "{}: stem expected={} actual={}",
                pl, exp_stem, act_stem
            ));
        }

        let exp_branch = exp_palace["earthly_branch"].as_str().unwrap();
        let act_branch = translate_earthly_branch(act_palace.earthly_branch, LANG);
        if act_branch != exp_branch {
            failures.push(format!(
                "{}: branch expected={} actual={}",
                pl, exp_branch, act_branch
            ));
        }

        // Major stars
        let exp_majors = exp_palace["major_stars"].as_array().unwrap();
        if act_palace.major_stars.len() != exp_majors.len() {
            failures.push(format!(
                "{}: major count expected={} actual={}",
                pl,
                exp_majors.len(),
                act_palace.major_stars.len()
            ));
        } else {
            for (si, exp_star) in exp_majors.iter().enumerate() {
                let act_star = &act_palace.major_stars[si];
                let exp_sn = exp_star["name"].as_str().unwrap();
                let act_sn = translate_star(act_star.key, LANG);
                if act_sn != exp_sn {
                    failures.push(format!(
                        "{}: major[{}] name expected={} actual={}",
                        pl, si, exp_sn, act_sn
                    ));
                }
                let exp_br = json_str(&exp_star["brightness"]);
                let act_br = fmt_brightness(act_star.brightness);
                if act_br != exp_br {
                    failures.push(format!(
                        "{}: major[{}] {} bright expected={:?} actual={:?}",
                        pl, si, exp_sn, exp_br, act_br
                    ));
                }
                let exp_mu = json_str(&exp_star["mutagen"]);
                let act_mu = fmt_mutagen(act_star.mutagen);
                if act_mu != exp_mu {
                    failures.push(format!(
                        "{}: major[{}] {} mutagen expected={:?} actual={:?}",
                        pl, si, exp_sn, exp_mu, act_mu
                    ));
                }
            }
        }

        // Minor stars (set comparison, then brightness/mutagen by name)
        let exp_minors = exp_palace["minor_stars"].as_array().unwrap();
        let act_minor_set: BTreeSet<String> = act_palace
            .minor_stars
            .iter()
            .map(|s| translate_star(s.key, LANG).to_string())
            .collect();
        let exp_minor_set: BTreeSet<String> = exp_minors
            .iter()
            .map(|s| s["name"].as_str().unwrap().to_string())
            .collect();
        if act_minor_set != exp_minor_set {
            failures.push(format!(
                "{}: minor set expected={:?} actual={:?}",
                pl, exp_minor_set, act_minor_set
            ));
        } else {
            for exp_star_val in exp_minors.iter() {
                let exp_sn = exp_star_val["name"].as_str().unwrap();
                if let Some(act_star) = act_palace
                    .minor_stars
                    .iter()
                    .find(|s| translate_star(s.key, LANG) == exp_sn)
                {
                    let exp_br = json_str(&exp_star_val["brightness"]);
                    let act_br = fmt_brightness(act_star.brightness);
                    if act_br != exp_br {
                        failures.push(format!(
                            "{}: minor {} bright expected={:?} actual={:?}",
                            pl, exp_sn, exp_br, act_br
                        ));
                    }
                    let exp_mu = json_str(&exp_star_val["mutagen"]);
                    let act_mu = fmt_mutagen(act_star.mutagen);
                    if act_mu != exp_mu {
                        failures.push(format!(
                            "{}: minor {} mutagen expected={:?} actual={:?}",
                            pl, exp_sn, exp_mu, act_mu
                        ));
                    }
                }
            }
        }

        // Adjective stars (set comparison)
        let exp_adjs = exp_palace["adjective_stars"].as_array().unwrap();
        let act_adj_set: BTreeSet<String> = act_palace
            .adjective_stars
            .iter()
            .map(|s| translate_star(s.key, LANG).to_string())
            .collect();
        let exp_adj_set: BTreeSet<String> = exp_adjs
            .iter()
            .map(|s| s["name"].as_str().unwrap().to_string())
            .collect();
        if act_adj_set != exp_adj_set {
            failures.push(format!(
                "{}: adj set expected={:?} actual={:?}",
                pl, exp_adj_set, act_adj_set
            ));
        }

        // 12-series stars
        let twelve_checks: Vec<(&str, &str, &str)> = vec![
            (
                "changsheng12",
                exp_palace["changsheng12"].as_str().unwrap(),
                translate_star(act_palace.changsheng12, LANG),
            ),
            (
                "boshi12",
                exp_palace["boshi12"].as_str().unwrap(),
                translate_star(act_palace.boshi12, LANG),
            ),
            (
                "jiangqian12",
                exp_palace["jiangqian12"].as_str().unwrap(),
                translate_star(act_palace.jiangqian12, LANG),
            ),
            (
                "suiqian12",
                exp_palace["suiqian12"].as_str().unwrap(),
                translate_star(act_palace.suiqian12, LANG),
            ),
        ];
        for (field, expected, actual) in &twelve_checks {
            if actual != expected {
                failures.push(format!(
                    "{}: {} expected={} actual={}",
                    pl, field, expected, actual
                ));
            }
        }

        // Decadal
        let exp_range = exp_palace["decadal_range"].as_array().unwrap();
        let exp_start = exp_range[0].as_u64().unwrap() as u32;
        let exp_end = exp_range[1].as_u64().unwrap() as u32;
        if act_palace.decadal.range != (exp_start, exp_end) {
            failures.push(format!(
                "{}: decadal_range expected=({},{}) actual=({},{})",
                pl, exp_start, exp_end, act_palace.decadal.range.0, act_palace.decadal.range.1
            ));
        }

        let exp_dstem = exp_palace["decadal_heavenly_stem"].as_str().unwrap();
        let act_dstem = translate_heavenly_stem(act_palace.decadal.heavenly_stem, LANG);
        if act_dstem != exp_dstem {
            failures.push(format!(
                "{}: dec_stem expected={} actual={}",
                pl, exp_dstem, act_dstem
            ));
        }

        let exp_dbranch = exp_palace["decadal_earthly_branch"].as_str().unwrap();
        let act_dbranch = translate_earthly_branch(act_palace.decadal.earthly_branch, LANG);
        if act_dbranch != exp_dbranch {
            failures.push(format!(
                "{}: dec_branch expected={} actual={}",
                pl, exp_dbranch, act_dbranch
            ));
        }

        // Ages
        let exp_ages: Vec<u32> = exp_palace["ages"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap() as u32)
            .collect();
        if act_palace.ages != exp_ages {
            failures.push(format!(
                "{}: ages expected={:?} actual={:?}",
                pl, exp_ages, act_palace.ages
            ));
        }

        if failures.len() >= MAX_FAILURES {
            break;
        }
    }
}

#[test]
fn golden_tier1_full() {
    let cases = load_cases();
    let mut failures: Vec<String> = Vec::new();

    for (case_idx, case) in cases.iter().enumerate() {
        check_case(case, case_idx, &mut failures);
        if failures.len() >= MAX_FAILURES {
            break;
        }
    }

    if !failures.is_empty() {
        let shown = failures.len().min(MAX_FAILURES);
        let mut msg = format!(
            "\n\nGolden tier-1 FAILED: {} failure(s) (showing first {}):\n\n",
            failures.len(),
            shown,
        );
        for (i, f) in failures.iter().take(shown).enumerate() {
            msg.push_str(&format!("  {}. {}\n", i + 1, f));
        }
        panic!("{}", msg);
    }

    eprintln!("Golden tier-1: all {} cases passed!", cases.len());
}

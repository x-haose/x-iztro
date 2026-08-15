//! Golden data 测试：`data` 模块四张查表逐键逐值对照 JS 输出。
//!
//! 数据由 tests/golden/generate_data.mjs 从 iztro 的 `data` 模块直接落盘：
//! - starsInfo：20 颗有记录星耀的十二宫亮度、五行、阴阳
//! - heavenlyStems：10 天干的阴阳、五行、对冲、四化四星
//! - earthlyBranches：12 地支的阴阳、五行、对冲、命主、身主、脏腑、部位、健康提示
//! - constants：干支宫名顺序表与时辰区间、五虎遁、五鼠遁
//!
//! 这些表是排盘算法的输入，任一格错位都会让整盘失准，因此零容忍差异。

use serde_json::Value;
use std::fs;
use x_iztro::data::constants::*;
use x_iztro::data::earthly_branches::get_earthly_branch_info;
use x_iztro::data::heavenly_stems::get_heavenly_stem_info;
use x_iztro::data::stars::{StarKey, get_star_info};
use x_iztro::data::types::*;

const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/data_tables.json");

fn load() -> Value {
    let raw =
        fs::read_to_string(GOLDEN).expect("缺少 data_tables.json，先跑 node generate_data.mjs");
    serde_json::from_str(&raw).expect("data_tables.json 不是合法 JSON")
}

/// JS 侧缺字段或空串统一视为「未填」，与 Rust 的 `None` 对齐。
fn opt_str(v: &Value, field: &str) -> Option<String> {
    match v.get(field).and_then(Value::as_str) {
        None | Some("") => None,
        Some(s) => Some(s.to_string()),
    }
}

fn str_at(v: &Value, field: &str) -> String {
    v[field]
        .as_str()
        .unwrap_or_else(|| panic!("缺少字段 {field}"))
        .to_string()
}

#[test]
fn stars_info_matches_js() {
    let golden = load();
    let stars = golden["starsInfo"].as_object().expect("starsInfo 不是对象");
    let mut failures = Vec::new();

    for (key, expected) in stars {
        let Some(star) = StarKey::from_key(key) else {
            failures.push(format!("{key}: Rust 侧无此星耀标识"));
            continue;
        };
        let Some(info) = get_star_info(star) else {
            failures.push(format!("{key}: Rust 侧无 StarInfo 记录"));
            continue;
        };

        let expected_brightness = expected["brightness"]
            .as_array()
            .expect("brightness 不是数组");
        for (i, want) in expected_brightness.iter().enumerate() {
            let want = want.as_str().unwrap_or("");
            let got = info.brightness[i].map(|b| b.as_key()).unwrap_or("");
            if want != got {
                failures.push(format!("{key}.brightness[{i}]: JS={want} Rust={got}"));
            }
        }
        if expected_brightness.len() != info.brightness.len() {
            failures.push(format!(
                "{key}.brightness 长度: JS={} Rust={}",
                expected_brightness.len(),
                info.brightness.len()
            ));
        }

        let want_fe = opt_str(expected, "fiveElements");
        let got_fe = info.five_elements.map(|f| f.as_str().to_string());
        if want_fe != got_fe {
            failures.push(format!(
                "{key}.fiveElements: JS={want_fe:?} Rust={got_fe:?}"
            ));
        }

        let want_yy = opt_str(expected, "yinYang");
        let got_yy = info.yin_yang.map(|y| y.as_str().to_string());
        if want_yy != got_yy {
            failures.push(format!("{key}.yinYang: JS={want_yy:?} Rust={got_yy:?}"));
        }
    }

    // 表外星耀不得有记录：iztro 只给 14 主星与六颗有亮度的辅星建表
    for key in [
        "lucunMin",
        "tianmaMin",
        "zuofuMin",
        "youbiMin",
        "tiankuiMin",
        "tianyueMin",
    ] {
        let star = StarKey::from_key(key).expect("标识应当有效");
        if get_star_info(star).is_some() {
            failures.push(format!("{key}: JS 表中无此星耀，Rust 却有 StarInfo 记录"));
        }
    }

    assert_eq!(stars.len(), 20, "STARS_INFO 条数应为 20");
    assert!(
        failures.is_empty(),
        "STARS_INFO 差异:\n{}",
        failures.join("\n")
    );
}

#[test]
fn heavenly_stems_match_js() {
    let golden = load();
    let stems = golden["heavenlyStems"]
        .as_object()
        .expect("heavenlyStems 不是对象");
    let mut failures = Vec::new();

    assert_eq!(stems.len(), HEAVENLY_STEMS.len(), "天干条数不一致");

    for (key, expected) in stems {
        let stem = HeavenlyStem::from_key(key).unwrap_or_else(|| panic!("{key}: Rust 侧无此天干"));
        let info = get_heavenly_stem_info(stem);

        if str_at(expected, "yinYang") != info.yin_yang.as_str() {
            failures.push(format!("{key}.yinYang 不一致"));
        }
        if str_at(expected, "fiveElements") != info.five_elements.as_str() {
            failures.push(format!("{key}.fiveElements 不一致"));
        }

        let want_crash = opt_str(expected, "crash");
        let got_crash = info.crash.map(|c| c.as_key().to_string());
        if want_crash != got_crash {
            failures.push(format!("{key}.crash: JS={want_crash:?} Rust={got_crash:?}"));
        }

        let want_mutagen = expected["mutagen"].as_array().expect("mutagen 不是数组");
        for (i, want) in want_mutagen.iter().enumerate() {
            let want = want.as_str().unwrap();
            let got = info.mutagen[i].as_key();
            if want != got {
                failures.push(format!("{key}.mutagen[{i}]: JS={want} Rust={got}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "heavenlyStems 差异:\n{}",
        failures.join("\n")
    );
}

#[test]
fn earthly_branches_match_js() {
    let golden = load();
    let branches = golden["earthlyBranches"]
        .as_object()
        .expect("earthlyBranches 不是对象");
    let mut failures = Vec::new();

    assert_eq!(branches.len(), EARTHLY_BRANCHES.len(), "地支条数不一致");

    for (key, expected) in branches {
        let branch =
            EarthlyBranch::from_key(key).unwrap_or_else(|| panic!("{key}: Rust 侧无此地支"));
        let info = get_earthly_branch_info(branch);

        let checks: [(&str, String, String); 8] = [
            (
                "yinYang",
                str_at(expected, "yinYang"),
                info.yin_yang.as_str().into(),
            ),
            (
                "fiveElements",
                str_at(expected, "fiveElements"),
                info.five_elements.as_str().into(),
            ),
            (
                "crash",
                str_at(expected, "crash"),
                info.crash.as_key().into(),
            ),
            ("soul", str_at(expected, "soul"), info.soul.as_key().into()),
            ("body", str_at(expected, "body"), info.body.as_key().into()),
            ("inside", str_at(expected, "inside"), info.inside.into()),
            ("outside", str_at(expected, "outside"), info.outside.into()),
            (
                "healthTip",
                str_at(expected, "healthTip"),
                info.health_tip.into(),
            ),
        ];

        for (field, want, got) in checks {
            if want != got {
                failures.push(format!("{key}.{field}: JS={want} Rust={got}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "earthlyBranches 差异:\n{}",
        failures.join("\n")
    );
}

#[test]
fn constants_match_js() {
    let golden = load();
    let c = &golden["constants"];
    let mut failures = Vec::new();

    let list = |field: &str| -> Vec<String> {
        c[field]
            .as_array()
            .unwrap_or_else(|| panic!("缺少常量 {field}"))
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect()
    };

    let cmp = |failures: &mut Vec<String>, name: &str, want: Vec<String>, got: Vec<String>| {
        if want != got {
            failures.push(format!("{name}: JS={want:?} Rust={got:?}"));
        }
    };

    cmp(
        &mut failures,
        "HEAVENLY_STEMS",
        list("HEAVENLY_STEMS"),
        HEAVENLY_STEMS
            .iter()
            .map(|s| s.as_key().to_string())
            .collect(),
    );
    cmp(
        &mut failures,
        "EARTHLY_BRANCHES",
        list("EARTHLY_BRANCHES"),
        EARTHLY_BRANCHES
            .iter()
            .map(|b| b.as_key().to_string())
            .collect(),
    );
    cmp(
        &mut failures,
        "PALACES",
        list("PALACES"),
        PALACES.iter().map(|p| p.as_key().to_string()).collect(),
    );
    cmp(
        &mut failures,
        "TIME_RANGE",
        list("TIME_RANGE"),
        TIME_RANGES.iter().map(|s| s.to_string()).collect(),
    );
    cmp(
        &mut failures,
        "ZODIAC",
        list("ZODIAC"),
        ZODIAC.iter().map(|s| s.to_string()).collect(),
    );
    cmp(
        &mut failures,
        "LANGUAGES",
        list("LANGUAGES"),
        LANGUAGES.iter().map(|s| s.to_string()).collect(),
    );
    cmp(
        &mut failures,
        "CHINESE_TIME",
        list("CHINESE_TIME"),
        CHINESE_TIME.iter().map(|s| s.to_string()).collect(),
    );

    // 五虎遁：年干（或月干）推正月天干；五鼠遁：日干推子时天干
    for (name, rule, table) in [
        ("TIGER_RULE", &c["TIGER_RULE"], &TIGER_RULE),
        ("RAT_RULE", &c["RAT_RULE"], &RAT_RULE),
    ] {
        for (i, stem) in HEAVENLY_STEMS.iter().enumerate() {
            let want = rule[stem.as_key()].as_str().unwrap_or_else(|| {
                panic!("{name} 缺少 {}", stem.as_key());
            });
            let got = table[i].as_key();
            if want != got {
                failures.push(format!("{name}[{}]: JS={want} Rust={got}", stem.as_key()));
            }
        }
    }

    // GENDER：男女各自的阴阳，决定大限顺逆
    for (key, gender) in [("male", Gender::Male), ("female", Gender::Female)] {
        let want = c["GENDER"][key].as_str().unwrap();
        let got = gender.yin_yang().as_str();
        if want != got {
            failures.push(format!("GENDER.{key}: JS={want} Rust={got}"));
        }
    }

    assert!(
        failures.is_empty(),
        "constants 差异:\n{}",
        failures.join("\n")
    );
}

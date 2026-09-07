//! 夹宫与三个运限列表对照 JS iztro v2.6.1 逐项比。
//!
//! 金标由 `tests/golden/generate_chart_lists.mjs` 生成，取样盘含晚子时与闰月出生：
//! 列表用的时辰取自时柱地支而非出生入参，晚子时两者差 12，只有这类盘能暴露取错。

mod common;

use serde_json::Value;
use std::fs;
use x_iztro::*;

const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/chart_lists.json");

fn load() -> Vec<Value> {
    let raw = fs::read_to_string(GOLDEN)
        .expect("缺少 chart_lists.json，先跑 node tests/golden/generate_chart_lists.mjs");
    serde_json::from_str(&raw).expect("chart_lists.json 不是合法 JSON")
}

fn chart_of(birth: &Value) -> Astrolabe {
    let gender = match birth["g"].as_str().expect("缺 g") {
        "男" => Gender::Male,
        _ => Gender::Female,
    };
    by_solar(
        birth["d"].as_str().expect("缺 d"),
        birth["t"].as_u64().expect("缺 t") as u8,
        gender,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .expect("排盘成功")
}

fn label(birth: &Value) -> String {
    format!(
        "{} t={} {}",
        birth["d"].as_str().unwrap_or(""),
        birth["t"].as_u64().unwrap_or(0),
        birth["g"].as_str().unwrap_or("")
    )
}

/// 译名列表与 JS 的字符串数组逐项比。
fn assert_names(got: &[Palace], want: &Value, ctx: &str, failures: &mut Vec<String>) {
    let want: Vec<&str> = want
        .as_array()
        .expect("宫名数组")
        .iter()
        .map(|v| v.as_str().unwrap_or(""))
        .collect();
    let got: Vec<String> = got
        .iter()
        .map(|p| translate_palace(*p, Language::ZhCN).to_string())
        .collect();
    if got != want {
        failures.push(format!("{ctx}: 宫名 {got:?} != {want:?}"));
    }
}

fn assert_stars(got: &[StarKey], want: &Value, ctx: &str, failures: &mut Vec<String>) {
    let want: Vec<&str> = want
        .as_array()
        .expect("星名数组")
        .iter()
        .map(|v| v.as_str().unwrap_or(""))
        .collect();
    let got: Vec<String> = got
        .iter()
        .map(|k| translate_star(*k, Language::ZhCN).to_string())
        .collect();
    if got != want {
        failures.push(format!("{ctx}: 四化 {got:?} != {want:?}"));
    }
}

fn gz(stem: HeavenlyStem, branch: EarthlyBranch) -> (String, String) {
    (
        translate_heavenly_stem(stem, Language::ZhCN).to_string(),
        translate_earthly_branch(branch, Language::ZhCN).to_string(),
    )
}

#[test]
fn flanking_palaces_match_js() {
    let mut failures = Vec::new();
    for case in load() {
        let a = chart_of(&case["birth"]);
        let ctx0 = label(&case["birth"]);
        for entry in case["flanking"].as_array().expect("夹宫数组") {
            let i = entry["index"].as_u64().expect("缺 index") as usize;
            let f = a.flanking_palaces(i).expect("宫位存在");
            let ctx = format!("{ctx0} 第 {i} 宫");
            for (side, got) in [("previous", f.previous), ("next", f.next)] {
                let want = &entry[side];
                let want_index = want["index"].as_u64().unwrap_or(99) as usize;
                let want_name = want["name"].as_str().unwrap_or("");
                let name = translate_palace(got.name, Language::ZhCN);
                if got.index != want_index || name != want_name {
                    failures.push(format!(
                        "{ctx} {side}: rust ({}, {}) != js ({want_index}, {want_name})",
                        got.index, name
                    ));
                }
            }
        }
    }
    assert!(
        failures.is_empty(),
        "夹宫与 JS 不一致:\n{}",
        failures.join("\n")
    );
}

#[test]
fn decadal_list_matches_js() {
    let mut failures = Vec::new();
    for case in load() {
        let a = chart_of(&case["birth"]);
        let ctx0 = label(&case["birth"]);
        let got = a.decadal_list();
        let want = case["decadals"].as_array().expect("大限数组");
        assert_eq!(got.len(), want.len(), "{ctx0}: 大限项数");

        for (n, (g, w)) in got.iter().zip(want).enumerate() {
            let ctx = format!("{ctx0} 大限#{n}");
            let (stem, branch) = gz(g.heavenly_stem, g.earthly_branch);
            let palace_name = translate_palace(g.palace_name, Language::ZhCN);
            let checks: [(&str, String, String); 7] = [
                (
                    "index",
                    g.index.to_string(),
                    w["index"].as_u64().unwrap_or(u64::MAX).to_string(),
                ),
                (
                    "name",
                    g.name.clone(),
                    w["name"].as_str().unwrap_or("").to_string(),
                ),
                (
                    "palaceName",
                    palace_name.to_string(),
                    w["palaceName"].as_str().unwrap_or("").to_string(),
                ),
                (
                    "ageRange",
                    format!("[{},{}]", g.age_range.0, g.age_range.1),
                    format!(
                        "[{},{}]",
                        w["ageRange"][0].as_u64().unwrap_or(u64::MAX),
                        w["ageRange"][1].as_u64().unwrap_or(u64::MAX)
                    ),
                ),
                (
                    "yearRange",
                    format!("[{},{}]", g.year_range.0, g.year_range.1),
                    format!(
                        "[{},{}]",
                        w["yearRange"][0].as_i64().unwrap_or(i64::MAX),
                        w["yearRange"][1].as_i64().unwrap_or(i64::MAX)
                    ),
                ),
                (
                    "heavenlyStem",
                    stem,
                    w["heavenlyStem"].as_str().unwrap_or("").to_string(),
                ),
                (
                    "earthlyBranch",
                    branch,
                    w["earthlyBranch"].as_str().unwrap_or("").to_string(),
                ),
            ];
            for (field, g_val, w_val) in checks {
                if g_val != w_val {
                    failures.push(format!("{ctx} {field}: rust {g_val} != js {w_val}"));
                }
            }
            assert_names(&g.palace_names, &w["palaceNames"], &ctx, &mut failures);
            assert_stars(&g.mutagen, &w["mutagen"], &ctx, &mut failures);
        }
    }
    assert!(
        failures.is_empty(),
        "大限列表与 JS 不一致（{} 处）:\n{}",
        failures.len(),
        failures
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn yearly_list_matches_js() {
    let mut failures = Vec::new();
    for case in load() {
        let a = chart_of(&case["birth"]);
        let ctx0 = label(&case["birth"]);
        for (key, want) in case["yearly"].as_object().expect("流年对象") {
            let ordinal: usize = key.parse().expect("大限序号");
            let got = a.yearly_list(ordinal).expect("流年列表");
            let want = want.as_array().expect("流年数组");
            assert_eq!(got.len(), want.len(), "{ctx0} 大限#{ordinal}: 流年项数");

            for (g, w) in got.iter().zip(want) {
                let ctx = format!("{ctx0} 大限#{ordinal} 虚岁{}", g.age);
                let (stem, branch) = gz(g.heavenly_stem, g.earthly_branch);
                if g.index as u64 != w["index"].as_u64().unwrap_or(u64::MAX)
                    || g.age as u64 != w["age"].as_u64().unwrap_or(u64::MAX)
                    || g.year != w["year"].as_i64().unwrap_or(i64::MAX)
                    || stem != w["heavenlyStem"].as_str().unwrap_or("")
                    || branch != w["earthlyBranch"].as_str().unwrap_or("")
                {
                    failures.push(format!(
                        "{ctx}: rust (idx {}, 虚岁 {}, 年 {}, {stem}{branch}) != js {w}",
                        g.index, g.age, g.year
                    ));
                }
                assert_names(&g.palace_names, &w["palaceNames"], &ctx, &mut failures);
                assert_stars(&g.mutagen, &w["mutagen"], &ctx, &mut failures);
            }
        }
    }
    assert!(
        failures.is_empty(),
        "流年列表与 JS 不一致（{} 处）:\n{}",
        failures.len(),
        failures
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn monthly_list_matches_js() {
    let mut failures = Vec::new();
    for case in load() {
        let a = chart_of(&case["birth"]);
        let ctx0 = label(&case["birth"]);
        for (key, want) in case["monthly"].as_object().expect("流月对象") {
            let (year, fix_leap) = key.split_once('_').expect("键形如 2020_true");
            let year: i64 = year.parse().expect("年份");
            let fix_leap: bool = fix_leap.parse().expect("fixLeap");
            let got = a.monthly_list(year, fix_leap).expect("流月列表");
            let want = want.as_array().expect("流月数组");
            assert_eq!(
                got.len(),
                want.len(),
                "{ctx0} {year} fixLeap={fix_leap}: 流月项数"
            );

            for (g, w) in got.iter().zip(want) {
                let ctx = format!("{ctx0} {year} fixLeap={fix_leap} {}月", g.month);
                let (stem, branch) = gz(g.heavenly_stem, g.earthly_branch);
                let num = |v: &Value| v.as_u64().unwrap_or(u64::MAX);
                let same = g.index as u64 == num(&w["index"])
                    && g.age as u64 == num(&w["age"])
                    && g.month as u64 == num(&w["month"])
                    && g.is_leap_month == w["isLeapMonth"].as_bool().unwrap_or(false)
                    && g.part.as_key() == w["part"].as_str().unwrap_or("")
                    && g.day_range.0 as u64 == num(&w["dayRange"][0])
                    && g.day_range.1 as u64 == num(&w["dayRange"][1])
                    && stem == w["heavenlyStem"].as_str().unwrap_or("")
                    && branch == w["earthlyBranch"].as_str().unwrap_or("");
                if !same {
                    failures.push(format!(
                        "{ctx}: rust (idx {}, 闰 {}, {}, 日[{},{}], {stem}{branch}) != js {w}",
                        g.index,
                        g.is_leap_month,
                        g.part.as_key(),
                        g.day_range.0,
                        g.day_range.1
                    ));
                }
                assert_stars(&g.mutagen, &w["mutagen"], &ctx, &mut failures);
            }
        }
    }
    assert!(
        failures.is_empty(),
        "流月列表与 JS 不一致（{} 处）:\n{}",
        failures.len(),
        failures
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

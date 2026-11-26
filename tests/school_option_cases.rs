//! 流派口径开关的四侧共用用例 `tests/data/school_option_cases.json`：每个非默认取值一张盘，核对农历生日、
//! 年柱、各宫星耀与十二神、来因宫标记与一个流月命宫；另有格局口径用例核对命中。Python、Go、JS 经各自的
//! 类型化配置排同一张盘逐项比对，配置字段漏接即失败。期望缺失或与当前输出不同都是失败；
//! 有意改动后用 `UPDATE_SCHOOL_OPTION_CASES=1` 重跑写入。规则本身由 `school_options.rs` 按出处核对。

use serde_json::{Value, json};
use x_iztro::{Config, Gender, Language, PatternConfig, by_solar};

const CASES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/school_option_cases.json"
);

/// 本批流派口径开关的线协议键（`dayDivide` 的 `forwardCarry` 另判）
const OPTION_KEYS: [&str; 8] = [
    "leapDivide",
    "tianmaPlacement",
    "changshengDirection",
    "tianshangTianshi",
    "tianwuPlacement",
    "yearly12Placement",
    "originalPalace",
    "douJunMonth",
];

/// 流月命宫所取的运限目标
const TARGET: (&str, u8) = ("2024-8-1", 3);

fn chart_of(case: &Value) -> x_iztro::Astrolabe {
    let c = &case["chart"];
    let config: Config = serde_json::from_value(case["config"].clone()).unwrap();
    let gender = match c["gender"].as_str().unwrap() {
        "male" => Gender::Male,
        _ => Gender::Female,
    };
    by_solar(
        c["solarDate"].as_str().unwrap(),
        c["timeIndex"].as_u64().unwrap() as u8,
        gender,
        true,
        Language::ZhCN,
        config,
    )
    .unwrap()
}

/// 一张盘的比对投影，各语言从类型化对象取同样的字段。
fn projection(chart: &x_iztro::Astrolabe) -> Value {
    let dto = serde_json::to_value(chart.to_dto()).unwrap();
    let lunar = &dto["rawDates"]["lunarDate"];
    let palaces: Vec<Value> = dto["palaces"]
        .as_array()
        .unwrap()
        .iter()
        .map(|p| {
            let stars: Vec<&Value> = ["majorStars", "minorStars", "adjectiveStars"]
                .iter()
                .flat_map(|k| p[*k].as_array().unwrap())
                .map(|s| &s["key"])
                .collect();
            json!({
                "stars": stars,
                "changsheng12Key": p["changsheng12Key"],
                "suiqian12Key": p["suiqian12Key"],
                "jiangqian12Key": p["jiangqian12Key"],
                "isOriginalPalace": p["isOriginalPalace"],
                "isSecondaryOriginalPalace": p.get("isSecondaryOriginalPalace").is_some(),
            })
        })
        .collect();
    let horoscope = chart.horoscope(TARGET.0, TARGET.1).unwrap();
    json!({
        "lunarDate": [lunar["lunarYear"], lunar["lunarMonth"], lunar["lunarDay"], lunar["isLeap"]],
        "yearlyKeys": dto["rawDates"]["chineseDate"]["yearlyKeys"],
        "palaces": palaces,
        "monthlyIndex": horoscope.monthly.index,
    })
}

#[test]
fn shared_school_option_cases() {
    let mut doc: Value = serde_json::from_str(&std::fs::read_to_string(CASES).unwrap()).unwrap();
    let update = std::env::var_os("UPDATE_SCHOOL_OPTION_CASES").is_some();
    assert_eq!(doc["target"], json!([TARGET.0, TARGET.1]));
    for case in doc["cases"].as_array_mut().unwrap() {
        let got = projection(&chart_of(case));
        // 每个用例都要让开关改变投影，否则漏接的配置字段在各侧也比对不出来
        let mut baseline = case.clone();
        for key in OPTION_KEYS {
            baseline["config"].as_object_mut().unwrap().remove(key);
        }
        if baseline["config"]["dayDivide"] == "forwardCarry" {
            baseline["config"]
                .as_object_mut()
                .unwrap()
                .remove("dayDivide");
        }
        assert_ne!(got, projection(&chart_of(&baseline)), "{}", case["name"]);
        if update {
            case["expect"] = got;
        } else {
            assert_eq!(got, case["expect"], "{}", case["name"]);
        }
    }
    for case in doc["patternCases"].as_array_mut().unwrap() {
        let pattern_config: PatternConfig =
            serde_json::from_value(case["patternConfig"].clone()).unwrap();
        let hits: Vec<Value> = chart_of(case)
            .patterns_with(&pattern_config)
            .iter()
            .map(|h| json!([h.key.as_key(), h.variant]))
            .collect();
        if update {
            case["expect"] = json!(hits);
        } else {
            assert_eq!(json!(hits), case["expect"], "{}", case["name"]);
        }
    }
    if update {
        std::fs::write(CASES, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    }
}

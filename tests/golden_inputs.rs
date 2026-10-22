//! 输入写法对照 JS iztro：日期串的文法（分隔符、补零、时间部分、非法写法）与运限省略时辰的换算。
//!
//! 数据由 tests/golden/generate_inputs.mjs 生成。

mod common;

use std::fs;

use serde_json::Value;
use x_iztro::{Config, Gender, Language, LeapMonth, by_lunar, by_solar, get_horoscope};

const DATA_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/inputs.json");

fn data() -> Value {
    let raw = fs::read_to_string(DATA_PATH).unwrap_or_else(|e| {
        panic!("读取 {DATA_PATH} 失败（cd tests/golden && npm run gen:inputs）: {e}")
    });
    serde_json::from_str(&raw).expect("inputs.json 不是合法 JSON")
}

/// 一例排盘结果与 JS 对照：JS 报错则 Rust 必须报错；否则 solarDate 回显与规范化哈希一致。
fn check_chart(
    case: &Value,
    label: &str,
    result: Result<x_iztro::Astrolabe, x_iztro::IztroError>,
    failures: &mut Vec<String>,
) {
    match (case.get("error"), result) {
        (Some(_), Ok(a)) => failures.push(format!(
            "{label}: JS 报错，Rust 却排出了盘 {}",
            a.solar_date
        )),
        (Some(_), Err(_)) => {}
        (None, Err(e)) => failures.push(format!("{label}: Rust 报错 {e}")),
        (None, Ok(a)) => {
            if a.solar_date != case["solarDate"].as_str().unwrap() {
                failures.push(format!(
                    "{label}: solarDate {} ≠ {}",
                    a.solar_date, case["solarDate"]
                ));
            }
            if common::hash_astrolabe(&a) != case["hash"].as_str().unwrap() {
                failures.push(format!("{label}: 盘面哈希不一致"));
            }
        }
    }
}

#[test]
fn date_grammar_matches_iztro() {
    let data = data();
    let mut failures = Vec::new();
    for case in data["solar"].as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let result = by_solar(
            input,
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        );
        check_chart(case, &format!("solar {input:?}"), result, &mut failures);
    }
    for case in data["lunar"].as_array().unwrap() {
        let input = case["input"].as_str().unwrap();
        let leap = LeapMonth::from_flags(case["leap"].as_bool().unwrap(), true);
        let result = by_lunar(
            input,
            2,
            Gender::Female,
            leap,
            Language::ZhCN,
            Config::default(),
        );
        check_chart(
            case,
            &format!("lunar {input:?} {leap:?}"),
            result,
            &mut failures,
        );
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn horoscope_target_time_matches_iztro() {
    let data = data();
    let mut failures = Vec::new();
    for case in data["horoscope"].as_array().unwrap() {
        if case.get("error").is_some() {
            let p = &case["p"];
            let birth = by_solar(
                p["d"].as_str().unwrap(),
                p["t"].as_u64().unwrap() as u8,
                Gender::Female,
                true,
                Language::ZhCN,
                Config::default(),
            )
            .unwrap();
            let td = case["td"].as_str().unwrap();
            let tt = case["tt"].as_u64().map(|t| t as u8);
            if get_horoscope(&birth, td, tt, Language::ZhCN).is_ok() {
                failures.push(format!("horoscope {td:?}: JS 报错，Rust 却算出了运限"));
            }
            continue;
        }
        common::check_horoscope_case(case, Config::default(), &mut failures);
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

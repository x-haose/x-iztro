//! Golden config 测试：四个非默认配置取值对照 JS 输出。
//!
//! 数据由 tests/golden/generate_config.mjs 生成：
//! - config_yeardivide.csv：yearDivide=exact，立春/初一分歧窗口（每年 1/20-2/20
//!   逐日 × 时辰 {0,6} × 男女）排盘哈希
//! - config_daydivide.csv：dayDivide=current，晚子时用例（含农历月末）排盘哈希
//! - config_agedivide.json：ageDivide=birthday，骑农历生日的运限用例
//! - config_horoscopedivide.json：horoscopeDivide=exact，立春窗口目标的运限用例
//!
//! 哈希不一致时用生成器 --inspect 模式重放 JS 单例与 Rust 规范化串 diff。

mod common;

use rs_iztro::by_solar;
use rs_iztro::data::types::*;
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
const HASH_LEN: usize = 32;
const MAX_FAILURES: usize = 20;

fn hash_astrolabe(astrolabe: &rs_iztro::Astrolabe) -> String {
    let canonical = common::canonical_astrolabe(astrolabe);
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    hex::encode(hasher.finalize())[..HASH_LEN].to_string()
}

/// 对照一份 `date,ti,g,hash` 格式的排盘哈希 CSV。
fn check_hash_csv(file: &str, config: Config) {
    let content = fs::read_to_string(format!("{GOLDEN_DIR}/{file}"))
        .unwrap_or_else(|_| panic!("{file} missing — run `node tests/golden/generate_config.mjs`"));

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for line in content.lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert!(f.len() == 4, "Bad line in {file}: {line}");
        let (date, ti, g, expected) = (f[0], f[1], f[2], f[3]);
        let gender = if g == "0" { Gender::Male } else { Gender::Female };

        let astrolabe = by_solar(date, ti.parse().unwrap(), gender, true, Language::ZhCN, config);
        total += 1;

        if hash_astrolabe(&astrolabe) != expected {
            failures.push(format!(
                "{date} ti={ti} g={g}: hash mismatch\n  rust canonical: {}",
                common::canonical_astrolabe(&astrolabe),
            ));
            if failures.len() >= MAX_FAILURES {
                break;
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n\n{file} FAILED: {} mismatch(es):\n\n{}\n",
        failures.len(),
        failures.join("\n\n"),
    );
    eprintln!("{file}: all {total} cases match!");
}

/// 对照一份运限紧凑 JSON 用例文件。
fn check_horoscope_json(file: &str, config: Config) {
    let data = fs::read_to_string(format!("{GOLDEN_DIR}/{file}"))
        .unwrap_or_else(|_| panic!("{file} missing — run `node tests/golden/generate_config.mjs`"));
    let cases: Vec<Value> = serde_json::from_str(&data).unwrap();

    let mut failures: Vec<String> = Vec::new();
    for case in &cases {
        common::check_horoscope_case(case, config, &mut failures);
        if failures.len() >= MAX_FAILURES {
            break;
        }
    }

    assert!(
        failures.is_empty(),
        "\n\n{file} FAILED: {} failure(s):\n{}\n",
        failures.len(),
        failures.join("\n"),
    );
    eprintln!("{file}: all {} cases passed!", cases.len());
}

#[test]
fn golden_config_year_divide_exact() {
    check_hash_csv(
        "config_yeardivide.csv",
        Config { year_divide: YearDivide::Exact, ..Config::default() },
    );
}

#[test]
fn golden_config_day_divide_current() {
    check_hash_csv(
        "config_daydivide.csv",
        Config { day_divide: DayDivide::Current, ..Config::default() },
    );
}

#[test]
fn golden_config_age_divide_birthday() {
    check_horoscope_json(
        "config_agedivide.json",
        Config { age_divide: AgeDivide::Birthday, ..Config::default() },
    );
}

#[test]
fn golden_config_horoscope_divide_exact() {
    check_horoscope_json(
        "config_horoscopedivide.json",
        Config { horoscope_divide: HoroscopeDivide::Exact, ..Config::default() },
    );
}

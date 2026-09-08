//! Golden config 测试：四个非默认配置取值对照 JS 输出。
//!
//! 数据由 tests/golden/generate_config.mjs 生成：
//! - config_yeardivide.csv：yearDivide=exact，立春/初一分歧窗口（每年 1/20-2/20
//!   逐日 × 时辰 {0,6} × 男女）排盘哈希
//! - config_daydivide.csv：dayDivide=current，晚子时用例（含农历月末）排盘哈希
//! - config_agedivide.json：ageDivide=birthday，骑农历生日的运限用例
//! - config_horoscopedivide.json：horoscopeDivide=exact，立春窗口目标的运限用例
//! - config_jieqi.json：horoscopeDivide=exact，按每个「节」的实际时刻取样的本命四柱
//! - config_combos.csv：排盘层开关组合（yearDivide×dayDivide、中州派算法与
//!   两者的交叉）排盘哈希，行首多一列组合标识
//! - config_combos_horoscope.json：运限层开关组合（ageDivide×horoscopeDivide、
//!   中州派算法与两者的交叉）运限用例，每例带组合标识
//!
//! 哈希不一致时用生成器 --inspect 模式重放 JS 单例与 Rust 规范化串 diff。

mod common;

use serde_json::Value;
use std::fs;
use x_iztro::data::types::*;
use x_iztro::{by_solar, translate_earthly_branch, translate_heavenly_stem};

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
const MAX_FAILURES: usize = 20;

/// 组合标识 -> Config：与 generate_config.mjs 的 CHART_COMBOS/HOROSCOPE_COMBOS 一一对应。
fn combo_config(cfg: &str) -> Config {
    let mut c = Config::default();
    for part in cfg.split('_') {
        match part {
            "yd" => c.year_divide = YearDivide::Exact,
            "dd" => c.day_divide = DayDivide::Current,
            "zz" => c.algorithm = Algorithm::Zhongzhou,
            "age" => c.age_divide = AgeDivide::Birthday,
            "hd" => c.horoscope_divide = HoroscopeDivide::Exact,
            other => panic!("Unknown config combo part: {other}"),
        }
    }
    c
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
        let gender = if g == "0" {
            Gender::Male
        } else {
            Gender::Female
        };

        let astrolabe = by_solar(
            date,
            ti.parse().unwrap(),
            gender,
            true,
            Language::ZhCN,
            config.clone(),
        )
        .unwrap();
        total += 1;

        if common::hash_astrolabe(&astrolabe) != expected {
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
        common::check_horoscope_case(case, config.clone(), &mut failures);
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
        Config {
            year_divide: YearDivide::Exact,
            ..Config::default()
        },
    );
}

#[test]
fn golden_config_day_divide_current() {
    check_hash_csv(
        "config_daydivide.csv",
        Config {
            day_divide: DayDivide::Current,
            ..Config::default()
        },
    );
}

#[test]
fn golden_config_age_divide_birthday() {
    check_horoscope_json(
        "config_agedivide.json",
        Config {
            age_divide: AgeDivide::Birthday,
            ..Config::default()
        },
    );
}

#[test]
fn golden_config_horoscope_divide_exact() {
    check_horoscope_json(
        "config_horoscopedivide.json",
        Config {
            horoscope_divide: HoroscopeDivide::Exact,
            ..Config::default()
        },
    );
}

/// `horoscopeDivide=exact` 下按节气时刻取样的本命四柱。
///
/// 按节气取月柱是时刻级比较，而构造时刻用的是时辰整点后 30 分。节气落在
/// `HH:00`~`HH:30` 的那些日子，分钟数取 0 会判到节气另一侧、月柱差一个月。
/// 这批取样直接骑在每个「节」的实际时刻上，固定日期抽样抓不到它们。
#[test]
fn golden_config_jieqi_pillars() {
    let file = "config_jieqi.json";
    let raw = fs::read_to_string(format!("{GOLDEN_DIR}/{file}"))
        .unwrap_or_else(|_| panic!("{file} missing — run `node tests/golden/generate_config.mjs`"));
    let cases: Vec<Value> = serde_json::from_str(&raw).expect("config_jieqi.json 不是合法 JSON");

    let config = Config {
        horoscope_divide: HoroscopeDivide::Exact,
        ..Config::default()
    };
    let mut failures: Vec<String> = Vec::new();
    for case in &cases {
        let date = case["d"].as_str().expect("缺 d");
        let ti = case["t"].as_u64().expect("缺 t") as u8;
        let gender = match case["g"].as_u64().expect("缺 g") {
            0 => Gender::Male,
            _ => Gender::Female,
        };
        let chart = by_solar(date, ti, gender, true, Language::ZhCN, config.clone())
            .unwrap_or_else(|e| panic!("{date} ti={ti}: {e}"));
        let cd = &chart.raw_dates.chinese_date;
        let got = [cd.yearly, cd.monthly, cd.daily, cd.hourly].map(|(s, b)| {
            format!(
                "{}{}",
                translate_heavenly_stem(s, Language::ZhCN),
                translate_earthly_branch(b, Language::ZhCN)
            )
        });
        let want: Vec<String> = ["yearly", "monthly", "daily", "hourly"]
            .iter()
            .map(|k| {
                let p = &case["cd"][k];
                format!(
                    "{}{}",
                    p[0].as_str().unwrap_or(""),
                    p[1].as_str().unwrap_or("")
                )
            })
            .collect();
        if got.as_slice() != want.as_slice() {
            failures.push(format!("{date} ti={ti}: rust {got:?} != js {want:?}"));
        }
    }
    assert!(
        failures.is_empty(),
        "节气取样四柱与 JS 不一致（{} / {} 例）:\n{}",
        failures.len(),
        cases.len(),
        failures
            .iter()
            .take(10)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn golden_config_chart_combos() {
    let file = "config_combos.csv";
    let content = fs::read_to_string(format!("{GOLDEN_DIR}/{file}"))
        .unwrap_or_else(|_| panic!("{file} missing — run `node tests/golden/generate_config.mjs`"));

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for line in content.lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert!(f.len() == 5, "Bad line in {file}: {line}");
        let (cfg, date, ti, g, expected) = (f[0], f[1], f[2], f[3], f[4]);
        let gender = if g == "0" {
            Gender::Male
        } else {
            Gender::Female
        };

        let astrolabe = by_solar(
            date,
            ti.parse().unwrap(),
            gender,
            true,
            Language::ZhCN,
            combo_config(cfg),
        )
        .unwrap();
        total += 1;

        if common::hash_astrolabe(&astrolabe) != expected {
            failures.push(format!(
                "[{cfg}] {date} ti={ti} g={g}: hash mismatch\n  rust canonical: {}",
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

#[test]
fn golden_config_horoscope_combos() {
    let file = "config_combos_horoscope.json";
    let data = fs::read_to_string(format!("{GOLDEN_DIR}/{file}"))
        .unwrap_or_else(|_| panic!("{file} missing — run `node tests/golden/generate_config.mjs`"));
    let cases: Vec<Value> = serde_json::from_str(&data).unwrap();

    let mut failures: Vec<String> = Vec::new();
    for case in &cases {
        let cfg = case["cfg"].as_str().unwrap();
        let before = failures.len();
        common::check_horoscope_case(case, combo_config(cfg), &mut failures);
        for f in failures.iter_mut().skip(before) {
            *f = format!("[{cfg}] {f}");
        }
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

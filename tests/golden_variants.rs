//! Golden variants 测试：by_lunar 入口、六语言词表、中州派算法对照 JS 输出。
//!
//! 数据由 tests/golden/generate_variants.mjs 生成：
//! - variants_bylunar.csv：全部闰月年的闰月逐日（is_leap_month × fix_leap ×
//!   时辰组合）加每年普通农历日期采样，规范化串哈希对照
//! - variants_languages.json：3 个命盘 × 6 语言全字段对照
//! - variants_zhongzhou.csv：60 年 × 4 个跨季日期 × 13 时辰 × 男女，
//!   中州派算法规范化串哈希对照
//!
//! 哈希不一致时用生成器的 --inspect-* 模式重放 JS 单例，与失败输出中的
//! Rust 规范化串 diff 定位。

mod common;

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use x_iztro::data::types::*;
use x_iztro::i18n::{
    translate_earthly_branch, translate_five_elements_class, translate_gender,
    translate_heavenly_stem, translate_palace, translate_star,
};
use x_iztro::{by_lunar, by_solar};

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
const HASH_LEN: usize = 32;
const MAX_FAILURES: usize = 20;

fn hash_astrolabe(astrolabe: &x_iztro::Astrolabe) -> String {
    let canonical = common::canonical_astrolabe(astrolabe);
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    hex::encode(hasher.finalize())[..HASH_LEN].to_string()
}

fn parse_lang(s: &str) -> Language {
    match s {
        "zh-CN" => Language::ZhCN,
        "zh-TW" => Language::ZhTW,
        "en-US" => Language::EnUS,
        "ja-JP" => Language::JaJP,
        "ko-KR" => Language::KoKR,
        "vi-VN" => Language::ViVN,
        _ => panic!("Unknown language: {s}"),
    }
}

#[test]
fn golden_variants_by_lunar() {
    let content = fs::read_to_string(format!("{GOLDEN_DIR}/variants_bylunar.csv"))
        .expect("variants_bylunar.csv missing — run `node tests/golden/generate_variants.mjs`");

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for line in content.lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert!(f.len() == 6, "Bad bylunar line: {line}");
        let (ld, ti, g, il, fl, expected) = (f[0], f[1], f[2], f[3], f[4], f[5]);
        let gender = if g == "0" {
            Gender::Male
        } else {
            Gender::Female
        };

        let astrolabe = by_lunar(
            ld,
            ti.parse().unwrap(),
            gender,
            il == "1",
            fl == "1",
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        total += 1;

        if hash_astrolabe(&astrolabe) != expected {
            failures.push(format!(
                "{ld} ti={ti} g={g} il={il} fl={fl}: hash mismatch\n  rust canonical: {}",
                common::canonical_astrolabe(&astrolabe),
            ));
            if failures.len() >= MAX_FAILURES {
                break;
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n\nGolden by_lunar FAILED: {} mismatch(es):\n\n{}\n",
        failures.len(),
        failures.join("\n\n"),
    );
    eprintln!("Golden by_lunar: all {total} cases match!");
}

#[test]
fn golden_variants_zhongzhou() {
    let content = fs::read_to_string(format!("{GOLDEN_DIR}/variants_zhongzhou.csv"))
        .expect("variants_zhongzhou.csv missing — run `node tests/golden/generate_variants.mjs`");

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for line in content.lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert!(f.len() == 4, "Bad zhongzhou line: {line}");
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
            Config {
                algorithm: Algorithm::Zhongzhou,
                ..Config::default()
            },
        )
        .unwrap();
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
        "\n\nGolden zhongzhou FAILED: {} mismatch(es):\n\n{}\n",
        failures.len(),
        failures.join("\n\n"),
    );
    eprintln!("Golden zhongzhou: all {total} cases match!");
}

#[test]
fn golden_variants_languages() {
    let data = fs::read_to_string(format!("{GOLDEN_DIR}/variants_languages.json"))
        .expect("variants_languages.json missing — run `node tests/golden/generate_variants.mjs`");
    let cases: Vec<Value> = serde_json::from_str(&data).unwrap();

    let mut failures: Vec<String> = Vec::new();

    for case in &cases {
        let p = &case["p"];
        let date = p["d"].as_str().unwrap();
        let ti = p["t"].as_u64().unwrap() as u8;
        let gender = if p["g"].as_u64().unwrap() == 0 {
            Gender::Male
        } else {
            Gender::Female
        };
        let lang = parse_lang(p["lang"].as_str().unwrap());
        let label = format!("[{date} t{ti} {}]", p["lang"].as_str().unwrap());

        let a = by_solar(date, ti, gender, true, lang, Config::default()).unwrap();

        let star = |s: &x_iztro::models::star::Star| {
            format!(
                "{}:{}:{}",
                s.name,
                s.brightness
                    .map(|b| x_iztro::i18n::translate_brightness(b, lang))
                    .unwrap_or(""),
                s.mutagen
                    .map(|m| x_iztro::i18n::translate_mutagen(m, lang))
                    .unwrap_or(""),
            )
        };

        let top_checks: Vec<(&str, String, &str)> = vec![
            (
                "gender",
                translate_gender(a.gender, lang).to_string(),
                case["gender"].as_str().unwrap(),
            ),
            ("time", a.time.clone(), case["time"].as_str().unwrap()),
            ("sign", a.sign.clone(), case["sign"].as_str().unwrap()),
            ("zodiac", a.zodiac.clone(), case["zodiac"].as_str().unwrap()),
            (
                "chinese_date",
                a.chinese_date.clone(),
                case["chinese_date"].as_str().unwrap(),
            ),
            (
                "soul",
                translate_star(a.soul, lang).to_string(),
                case["soul"].as_str().unwrap(),
            ),
            (
                "body",
                translate_star(a.body, lang).to_string(),
                case["body"].as_str().unwrap(),
            ),
            (
                "five_elements_class",
                translate_five_elements_class(a.five_elements_class, lang).to_string(),
                case["five_elements_class"].as_str().unwrap(),
            ),
            (
                "soul_branch",
                translate_earthly_branch(a.earthly_branch_of_soul_palace, lang).to_string(),
                case["soul_branch"].as_str().unwrap(),
            ),
        ];
        for (field, actual, expected) in &top_checks {
            if actual != expected {
                failures.push(format!(
                    "{label} {field}: expected={expected} actual={actual}"
                ));
            }
        }

        for (pi, exp_p) in case["palaces"].as_array().unwrap().iter().enumerate() {
            let ap = &a.palaces[pi];
            let pl = format!("{label} p[{pi}]");

            let checks: Vec<(&str, String, &str)> = vec![
                (
                    "name",
                    translate_palace(ap.name, lang).to_string(),
                    exp_p["name"].as_str().unwrap(),
                ),
                (
                    "hs",
                    translate_heavenly_stem(ap.heavenly_stem, lang).to_string(),
                    exp_p["hs"].as_str().unwrap(),
                ),
                (
                    "eb",
                    translate_earthly_branch(ap.earthly_branch, lang).to_string(),
                    exp_p["eb"].as_str().unwrap(),
                ),
                (
                    "cs",
                    translate_star(ap.changsheng12, lang).to_string(),
                    exp_p["cs"].as_str().unwrap(),
                ),
                (
                    "bo",
                    translate_star(ap.boshi12, lang).to_string(),
                    exp_p["bo"].as_str().unwrap(),
                ),
                (
                    "jq",
                    translate_star(ap.jiangqian12, lang).to_string(),
                    exp_p["jq"].as_str().unwrap(),
                ),
                (
                    "sq",
                    translate_star(ap.suiqian12, lang).to_string(),
                    exp_p["sq"].as_str().unwrap(),
                ),
            ];
            for (field, actual, expected) in &checks {
                if actual != expected {
                    failures.push(format!("{pl} {field}: expected={expected} actual={actual}"));
                }
            }

            let exp_ms: Vec<&str> = exp_p["ms"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let act_ms: Vec<String> = ap.major_stars.iter().map(&star).collect();
            if act_ms != exp_ms {
                failures.push(format!("{pl} ms: expected={exp_ms:?} actual={act_ms:?}"));
            }

            let exp_ns: Vec<&str> = exp_p["ns"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let mut act_ns: Vec<String> = ap.minor_stars.iter().map(&star).collect();
            act_ns.sort();
            if act_ns != exp_ns {
                failures.push(format!("{pl} ns: expected={exp_ns:?} actual={act_ns:?}"));
            }

            let exp_adj: Vec<&str> = exp_p["adj"]
                .as_array()
                .unwrap()
                .iter()
                .map(|v| v.as_str().unwrap())
                .collect();
            let mut act_adj: Vec<String> =
                ap.adjective_stars.iter().map(|s| s.name.clone()).collect();
            act_adj.sort();
            if act_adj != exp_adj {
                failures.push(format!("{pl} adj: expected={exp_adj:?} actual={act_adj:?}"));
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n\nGolden languages FAILED: {} mismatch(es):\n{}\n",
        failures.len(),
        failures.join("\n"),
    );
    eprintln!("Golden languages: all {} cases match!", cases.len());
}

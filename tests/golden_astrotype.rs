//! 天盘/地盘/人盘（`astro_type`）金标对照
//!
//! 数据由 tests/golden/generate_astrotype.mjs 从 JS iztro 的 `astro.withOptions`
//! 生成：
//! - astrotype.csv：60 年 × 4 个跨季日期 × 13 时辰 × 男女 × {地盘,人盘}，
//!   规范化串哈希对照，覆盖广度
//! - astrotype_full.json：4 个命盘 × {地盘,人盘} 全字段对照，
//!   覆盖杂耀顺序等哈希（对杂耀排序）看不见的细节
//!
//! 哈希不一致时用 `node tests/golden/generate_astrotype.mjs --inspect <date> <ti>
//! <男|女> <earth|human>` 重放 JS 单例，与失败输出中的 Rust 规范化串 diff 定位。

mod common;

use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use x_iztro::by_solar;
use x_iztro::data::types::*;
use x_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};

const GOLDEN_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden");
const HASH_LEN: usize = 32;
const MAX_FAILURES: usize = 20;

fn hash_astrolabe(astrolabe: &x_iztro::Astrolabe) -> String {
    let canonical = common::canonical_astrolabe(astrolabe);
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    hex::encode(hasher.finalize())[..HASH_LEN].to_string()
}

fn parse_astro_type(s: &str) -> AstroType {
    AstroType::from_key(s).unwrap_or_else(|| panic!("Unknown astroType: {s}"))
}

fn chart(date: &str, time_index: u8, gender: Gender, astro_type: AstroType) -> x_iztro::Astrolabe {
    by_solar(
        date,
        time_index,
        gender,
        true,
        Language::ZhCN,
        Config::default().with_astro_type(astro_type),
    )
    .unwrap()
}

#[test]
fn golden_astrotype_hashes() {
    let content = fs::read_to_string(format!("{GOLDEN_DIR}/astrotype.csv"))
        .expect("astrotype.csv missing — run `node tests/golden/generate_astrotype.mjs`");

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    for line in content.lines() {
        let f: Vec<&str> = line.split(',').collect();
        assert!(f.len() == 5, "Bad astrotype line: {line}");
        let (date, ti, g, at, expected) = (f[0], f[1], f[2], f[3], f[4]);
        let gender = if g == "0" {
            Gender::Male
        } else {
            Gender::Female
        };

        let astrolabe = chart(date, ti.parse().unwrap(), gender, parse_astro_type(at));
        total += 1;

        if hash_astrolabe(&astrolabe) != expected {
            failures.push(format!(
                "{date} ti={ti} g={g} astroType={at}: hash mismatch\n  rust canonical: {}",
                common::canonical_astrolabe(&astrolabe),
            ));
            if failures.len() >= MAX_FAILURES {
                break;
            }
        }
    }

    assert!(
        failures.is_empty(),
        "\n\nGolden astroType FAILED: {} mismatch(es):\n\n{}\n",
        failures.len(),
        failures.join("\n\n"),
    );
    eprintln!("Golden astroType: all {total} cases match!");
}

#[test]
fn golden_astrotype_full_fields() {
    let data = fs::read_to_string(format!("{GOLDEN_DIR}/astrotype_full.json"))
        .expect("astrotype_full.json missing — run `node tests/golden/generate_astrotype.mjs`");
    let cases: Vec<Value> = serde_json::from_str(&data).unwrap();

    let lang = Language::ZhCN;
    let mut failures: Vec<String> = Vec::new();
    let mut check = |label: String, expected: &str, actual: String| {
        if expected != actual {
            failures.push(format!("{label}: expected {expected:?}, got {actual:?}"));
        }
    };

    for case in &cases {
        let p = &case["params"];
        let date = p["solar_date"].as_str().unwrap();
        let ti = p["time_index"].as_u64().unwrap() as u8;
        let gender = if p["gender"].as_str().unwrap() == "男" {
            Gender::Male
        } else {
            Gender::Female
        };
        let at = parse_astro_type(p["astro_type"].as_str().unwrap());
        let tag = format!("{date} ti={ti} {at:?}");

        let a = chart(date, ti, gender, at);

        check(
            format!("{tag} soul_palace_branch"),
            case["soul_palace_branch"].as_str().unwrap(),
            translate_earthly_branch(a.earthly_branch_of_soul_palace, lang).to_string(),
        );
        check(
            format!("{tag} body_palace_branch"),
            case["body_palace_branch"].as_str().unwrap(),
            translate_earthly_branch(a.earthly_branch_of_body_palace, lang).to_string(),
        );
        check(
            format!("{tag} five_elements_class"),
            case["five_elements_class"].as_str().unwrap(),
            translate_five_elements_class(a.five_elements_class, lang).to_string(),
        );
        check(
            format!("{tag} soul_star"),
            case["soul_star"].as_str().unwrap(),
            translate_star(a.soul, lang).to_string(),
        );
        check(
            format!("{tag} body_star"),
            case["body_star"].as_str().unwrap(),
            translate_star(a.body, lang).to_string(),
        );

        for (i, ep) in case["palaces"].as_array().unwrap().iter().enumerate() {
            let rp = &a.palaces[i];
            let ptag = format!("{tag} palace[{i}]");

            check(
                format!("{ptag} name"),
                ep["name"].as_str().unwrap(),
                translate_palace(rp.name, lang).to_string(),
            );
            check(
                format!("{ptag} heavenly_stem"),
                ep["heavenly_stem"].as_str().unwrap(),
                translate_heavenly_stem(rp.heavenly_stem, lang).to_string(),
            );
            check(
                format!("{ptag} earthly_branch"),
                ep["earthly_branch"].as_str().unwrap(),
                translate_earthly_branch(rp.earthly_branch, lang).to_string(),
            );
            check(
                format!("{ptag} is_body_palace"),
                &ep["is_body_palace"].as_bool().unwrap().to_string(),
                rp.is_body_palace.to_string(),
            );
            check(
                format!("{ptag} is_original_palace"),
                &ep["is_original_palace"].as_bool().unwrap().to_string(),
                rp.is_original_palace.to_string(),
            );
            check(
                format!("{ptag} changsheng12"),
                ep["changsheng12"].as_str().unwrap(),
                translate_star(rp.changsheng12, lang).to_string(),
            );
            check(
                format!("{ptag} boshi12"),
                ep["boshi12"].as_str().unwrap(),
                translate_star(rp.boshi12, lang).to_string(),
            );
            check(
                format!("{ptag} jiangqian12"),
                ep["jiangqian12"].as_str().unwrap(),
                translate_star(rp.jiangqian12, lang).to_string(),
            );
            check(
                format!("{ptag} suiqian12"),
                ep["suiqian12"].as_str().unwrap(),
                translate_star(rp.suiqian12, lang).to_string(),
            );
            check(
                format!("{ptag} decadal_range"),
                &format!(
                    "{}-{}",
                    ep["decadal_range"][0].as_u64().unwrap(),
                    ep["decadal_range"][1].as_u64().unwrap()
                ),
                format!("{}-{}", rp.decadal.range.0, rp.decadal.range.1),
            );
            check(
                format!("{ptag} decadal_heavenly_stem"),
                ep["decadal_heavenly_stem"].as_str().unwrap(),
                translate_heavenly_stem(rp.decadal.heavenly_stem, lang).to_string(),
            );
            check(
                format!("{ptag} decadal_earthly_branch"),
                ep["decadal_earthly_branch"].as_str().unwrap(),
                translate_earthly_branch(rp.decadal.earthly_branch, lang).to_string(),
            );
            check(
                format!("{ptag} ages"),
                &join_u64(&ep["ages"]),
                rp.ages
                    .iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );

            // 主星、辅星按安放顺序逐项比对，含亮度与四化
            for (kind, expected_stars, actual_stars) in [
                ("major", &ep["major_stars"], &rp.major_stars),
                ("minor", &ep["minor_stars"], &rp.minor_stars),
            ] {
                let expected_list = expected_stars.as_array().unwrap();
                check(
                    format!("{ptag} {kind}_stars count"),
                    &expected_list.len().to_string(),
                    actual_stars.len().to_string(),
                );
                for (j, es) in expected_list.iter().enumerate() {
                    let Some(rs) = actual_stars.get(j) else {
                        continue;
                    };
                    check(
                        format!("{ptag} {kind}_stars[{j}]"),
                        &format!(
                            "{}:{}:{}",
                            es["name"].as_str().unwrap(),
                            es["brightness"].as_str().unwrap_or(""),
                            es["mutagen"].as_str().unwrap_or(""),
                        ),
                        format!(
                            "{}:{}:{}",
                            translate_star(rs.key, lang),
                            rs.brightness
                                .map(|b| translate_brightness(b, lang))
                                .unwrap_or(""),
                            rs.mutagen.map(|m| translate_mutagen(m, lang)).unwrap_or(""),
                        ),
                    );
                }
            }

            // 杂耀按顺序比对：重排挪动的天伤天使天才会落到列表末尾
            check(
                format!("{ptag} adjective_stars"),
                &ep["adjective_stars"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|v| v.as_str().unwrap())
                    .collect::<Vec<_>>()
                    .join(";"),
                rp.adjective_stars
                    .iter()
                    .map(|s| translate_star(s.key, lang))
                    .collect::<Vec<_>>()
                    .join(";"),
            );
        }
    }

    assert!(
        failures.is_empty(),
        "\n\nGolden astroType full-field FAILED: {} mismatch(es):\n\n{}\n",
        failures.len(),
        failures
            .iter()
            .take(MAX_FAILURES)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n"),
    );
    eprintln!(
        "Golden astroType full-field: all {} cases match!",
        cases.len()
    );
}

fn join_u64(v: &Value) -> String {
    v.as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_u64().unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",")
}

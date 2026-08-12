//! Golden tier-3: JS 全参数空间哈希对照。
//!
//! 数据由 tests/golden/generate_tier3.mjs 生成：1984-2043 每一天 × 13 时辰
//! × 男女 × fix_leap（闰月日期含 fix_leap=false），约 57 万例。
//! 每例将排盘结果按规范化串（tests/common/mod.rs）取 SHA-256 前 32 个
//! hex 字符与 JS 侧对照，任何字段偏差都会改变哈希。
//!
//! 用例量大（约 40 秒），标记 `#[ignore]`，通过
//! `cargo test --test golden_tier3 -- --ignored` 运行。
//!
//! 哈希不一致时的排查方式：用失败行参数运行
//! `node tests/golden/generate_tier3.mjs --inspect <date> <ti> <男|女> <fl>`
//! 得到 JS 规范化串，与失败输出中打印的 Rust 规范化串 diff 定位字段。

mod common;

use sha2::{Digest, Sha256};
use std::fs;
use x_iztro::by_solar;
use x_iztro::data::types::*;

const TIER3_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier3");
const HASH_LEN: usize = 32;
const MAX_FAILURES: usize = 20;

fn hash_astrolabe(astrolabe: &x_iztro::Astrolabe) -> String {
    let canonical = common::canonical_astrolabe(astrolabe);
    let mut hasher = Sha256::new();
    hasher.update(canonical.as_bytes());
    hex::encode(hasher.finalize())[..HASH_LEN].to_string()
}

#[test]
#[ignore]
fn golden_tier3_full_parameter_space() {
    let mut entries: Vec<_> = fs::read_dir(TIER3_DIR)
        .expect("tier3 directory missing — run `node tests/golden/generate_tier3.mjs` first")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.starts_with("year_") && name.ends_with(".csv")
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    assert!(
        !entries.is_empty(),
        "No tier3 year files found in {TIER3_DIR}"
    );

    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    'outer: for entry in &entries {
        let path = entry.path();
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));

        for line in content.lines() {
            let fields: Vec<&str> = line.split(',').collect();
            assert!(fields.len() == 5, "Bad tier3 line: {line}");
            let (date, ti, g, fl, expected) =
                (fields[0], fields[1], fields[2], fields[3], fields[4]);
            let time_index: u8 = ti.parse().unwrap();
            let gender = if g == "0" {
                Gender::Male
            } else {
                Gender::Female
            };
            let fix_leap = fl == "1";

            let astrolabe = by_solar(
                date,
                time_index,
                gender,
                fix_leap,
                Language::ZhCN,
                Config::default(),
            )
            .unwrap();
            let actual = hash_astrolabe(&astrolabe);
            total += 1;

            if actual != expected {
                failures.push(format!(
                    "{date} ti={ti} g={g} fl={fl}: hash mismatch\n  rust canonical: {}",
                    common::canonical_astrolabe(&astrolabe),
                ));
                if failures.len() >= MAX_FAILURES {
                    break 'outer;
                }
            }
        }
        eprint!(
            "\r  {} checked ({} total)",
            path.file_name().unwrap().to_string_lossy(),
            total
        );
    }
    eprintln!();

    if !failures.is_empty() {
        panic!(
            "\n\nGolden tier-3 FAILED: {} mismatch(es) (showing up to {}):\n\n{}\n",
            failures.len(),
            MAX_FAILURES,
            failures.join("\n\n"),
        );
    }

    eprintln!("Golden tier-3: all {total} cases match JS hashes!");
}

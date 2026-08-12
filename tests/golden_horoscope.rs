//! Golden horoscope 测试：运限全字段对照 JS 输出。
//!
//! 数据由 tests/golden/generate_horoscope.mjs 生成：360 个命盘 × 16 个目标
//! 日期（12 流年支、童限、高龄大限、闰月下半月、晚子时目标），共 5760 例。
//! 对照六个运限层级的宫位索引、层级名、干支、四化星名、流耀分布，
//! 以及小限虚岁、流年岁前/将前十二神与目标农历日期。

mod common;

use rs_iztro::data::types::*;
use serde_json::Value;
use std::fs;

const DATA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/golden/horoscope_data.json"
);
const MAX_FAILURES: usize = 50;

#[test]
fn golden_horoscope_full() {
    let data = fs::read_to_string(DATA_PATH).expect(
        "horoscope_data.json missing — run `node tests/golden/generate_horoscope.mjs` first",
    );
    let cases: Vec<Value> =
        serde_json::from_str(&data).expect("Failed to parse horoscope_data.json");

    let mut failures: Vec<String> = Vec::new();

    for case in &cases {
        common::check_horoscope_case(case, Config::default(), &mut failures);
        if failures.len() >= MAX_FAILURES {
            break;
        }
    }

    if !failures.is_empty() {
        let shown = failures.len().min(MAX_FAILURES);
        let mut msg = format!(
            "\n\nGolden horoscope FAILED: {} failure(s) (showing first {}):\n\n",
            failures.len(),
            shown,
        );
        for (i, f) in failures.iter().take(shown).enumerate() {
            msg.push_str(&format!("  {}. {}\n", i + 1, f));
        }
        panic!("{}", msg);
    }

    eprintln!("Golden horoscope: all {} cases passed!", cases.len());
}

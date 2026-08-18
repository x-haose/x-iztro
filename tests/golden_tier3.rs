//! Golden tier-3: JS 全参数空间哈希对照。
//!
//! 数据由 tests/golden/generate_tier3.mjs 生成：1984-2043 每一天 × 13 时辰
//! × 男女 × fix_leap（闰月日期含 fix_leap=false），约 59 万例。
//! 每例将排盘结果按规范化串（tests/common/mod.rs）取 SHA-256 前 32 个
//! hex 字符与 JS 侧对照，任何字段偏差都会改变哈希。
//!
//! 用例量大（release 下约 2 分钟），标记 `#[ignore]`，通过
//! `cargo test --release --test golden_tier3 -- --ignored` 运行。
//!
//! 哈希不一致时的排查方式：用失败行参数运行
//! `node tests/golden/generate_tier3.mjs --inspect <date> <ti> <男|女> <fl>`
//! 得到 JS 规范化串，与失败输出中打印的 Rust 规范化串 diff 定位字段。

mod common;

const TIER3_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier3");
const MAX_FAILURES: usize = 20;

#[test]
#[ignore]
fn golden_tier3_full_parameter_space() {
    common::check_hash_year_csvs(
        TIER3_DIR,
        "node tests/golden/generate_tier3.mjs",
        MAX_FAILURES,
    );
}

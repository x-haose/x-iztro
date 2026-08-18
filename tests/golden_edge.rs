//! Golden 边界年代：其它金标（1984-2043）之外的日期区间哈希对照。
//!
//! 数据由 tests/golden/generate_tier_edge.mjs 生成：1583-1983 与 2044-2100
//! 每 10 年取 1 年（另单取 2100）× 12 个月 × 日 {1,15,28} × 13 时辰 × 男女，
//! 闰月日期额外含 fix_leap=false，约 4.5 万例。机制与 tier3 相同：
//! 规范化串（tests/common/mod.rs）SHA-256 前 32 个 hex 字符逐例对照。
//!
//! 1583 是 x-iztro 入口校验的公历下界；JS 侧 iztro/lunar-lite 对更早年份
//! 不报错而是外推，故区间由 x-iztro 的定义域决定，不是 JS 能力所限。
//!
//! 哈希不一致时的排查方式：用失败行参数运行
//! `node tests/golden/generate_tier_edge.mjs --inspect <date> <ti> <男|女> <fl>`
//! 得到 JS 规范化串，与失败输出中打印的 Rust 规范化串 diff 定位字段。

mod common;

const EDGE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier_edge");
const MAX_FAILURES: usize = 20;

#[test]
fn golden_edge_eras() {
    common::check_hash_year_csvs(
        EDGE_DIR,
        "node tests/golden/generate_tier_edge.mjs",
        MAX_FAILURES,
    );
}

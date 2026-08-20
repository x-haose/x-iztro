//! Golden 1602 闰二月窗口：lunar_rust 月界缺陷修正后的逐字节对照。
//!
//! lunar_rust 1.0.1 把农历 1602 年（明万历三十年）闰二月的合朔算晚一天
//! （src/astro/lunar_table.rs 修正），受影响的公历区间为 1602-3-24 至
//! 1602-4-21。数据由 tests/golden/generate_1602.mjs 生成：1602-2-20 至
//! 1602-4-25 逐日（覆盖窗口及二月/闰二月/三月两侧月界）× 13 时辰 × 男女，
//! 闰月日期额外含 fix_leap=false，共 2,444 例。机制与 tier3/tier_edge
//! 相同：规范化串（tests/common/mod.rs）SHA-256 前 32 个 hex 字符逐例对照。
//!
//! 哈希不一致时的排查方式：用失败行参数运行
//! `node tests/golden/generate_1602.mjs --inspect <date> <ti> <男|女> <fl>`
//! 得到 JS 规范化串，与失败输出中打印的 Rust 规范化串 diff 定位字段。

mod common;

const DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier_1602");
const MAX_FAILURES: usize = 20;

#[test]
fn golden_1602_leap_month_window() {
    common::check_hash_year_csvs(DIR, "node tests/golden/generate_1602.mjs", MAX_FAILURES);
}

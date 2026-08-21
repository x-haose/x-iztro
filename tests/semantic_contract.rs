//! 语义化 key 契约的守护测试。
//!
//! 契约：DTO 输出中任何随排盘语言变化的译文字段，都必须有配套的语言无关
//! 标识字段，命名规则（与 src/dto.rs 模块注释一致）：
//! - 译文属性 `x` → 同级 `xKey`（数组为 `xKeys`；复数属性 `xs` → `xKeys`）
//! - 实体自身的译名 `name` → 同级 `key`（星、格局）或 `nameKey`（宫、运限层级）
//! - 少数语义改名与结构化对应（`mutagen` → `mutagenStarKeys`、
//!   `time` → `timeIndex`、`chineseDate` → `rawDates.chineseDate.*Keys`）列入显式映射
//!
//! 判定方法：同一张盘分别按 zh-CN 与 en-US 排盘取 DTO JSON，逐字段并行遍历；
//! 值在两种语言下不同的字段即「译文字段」，检查其同级是否存在按上述规则命名、
//! 且两种语言下取值一致的 key 字段。新增译文字段而忘配 key 时本测试失败。

use serde_json::Value;
use x_iztro::data::types::*;
use x_iztro::{by_solar, get_horoscope};

/// 语义改名与结构化对应的显式映射：译文字段名 → 认可的配套字段名列表。
/// 空列表表示无需同级配套（`chineseDate` 由 `rawDates.chineseDate.*Keys` 覆盖，
/// `lunarDate` 由 `rawDates.lunarDate` 覆盖，`language` 本身即语言无关代码）。
/// `mutagen` 两形态各配一名：星上的字符串配 `mutagenKey`（四化类型），
/// 运限层级的数组配 `mutagenStarKeys`（被化的四颗星）。
const EXPLICIT_PAIRS: [(&str, &[&str]); 5] = [
    ("mutagen", &["mutagenKey", "mutagenStarKeys"]),
    ("time", &["timeIndex"]),
    ("chineseDate", &[]),
    ("lunarDate", &[]),
    ("language", &[]),
];

/// 译文字段 `field` 在对象 `obj` 里认可的配套 key 字段名候选。
fn key_candidates(field: &str) -> Vec<String> {
    if let Some((_, mapped)) = EXPLICIT_PAIRS.iter().find(|(f, _)| *f == field) {
        return mapped.iter().map(|m| (*m).to_string()).collect();
    }
    let mut cands = vec![format!("{field}Key"), format!("{field}Keys")];
    if field == "name" {
        cands.push("key".to_string());
    }
    if let Some(stem) = field.strip_suffix('s') {
        cands.push(format!("{stem}Keys"));
    }
    cands
}

/// 并行遍历两种语言的同构 JSON，收集「值随语言变化但同级无配套 key」的字段路径。
fn walk(zh: &Value, en: &Value, path: &str, violations: &mut Vec<String>) {
    match (zh, en) {
        (Value::Object(zh_map), Value::Object(en_map)) => {
            for (field, zh_val) in zh_map {
                let Some(en_val) = en_map.get(field) else {
                    violations.push(format!("{path}.{field}: 两种语言的结构不同"));
                    continue;
                };
                // 已是 key 字段或纯结构字段：只需递归
                if zh_val == en_val {
                    walk(zh_val, en_val, &format!("{path}.{field}"), violations);
                    continue;
                }
                match zh_val {
                    // 值随语言变化的标量/字符串数组：要求同级配套 key
                    Value::String(_) => {
                        check_pair(zh_map, en_map, field, path, violations);
                    }
                    Value::Array(items) if items.iter().all(|v| matches!(v, Value::String(_))) => {
                        check_pair(zh_map, en_map, field, path, violations);
                    }
                    // 嵌套结构：递归找到具体的叶子差异
                    _ => walk(zh_val, en_val, &format!("{path}.{field}"), violations),
                }
            }
        }
        (Value::Array(zh_items), Value::Array(en_items)) => {
            assert_eq!(
                zh_items.len(),
                en_items.len(),
                "{path}: 两种语言的数组长度不同"
            );
            for (i, (z, e)) in zh_items.iter().zip(en_items).enumerate() {
                walk(z, e, &format!("{path}[{i}]"), violations);
            }
        }
        _ => {}
    }
}

/// 译文字段的同级必须存在某个候选 key 字段，且其取值语言无关。
fn check_pair(
    zh_map: &serde_json::Map<String, Value>,
    en_map: &serde_json::Map<String, Value>,
    field: &str,
    path: &str,
    violations: &mut Vec<String>,
) {
    let cands = key_candidates(field);
    if cands.is_empty() {
        return; // 显式映射声明配套标识不在同级
    }
    let ok = cands
        .iter()
        .any(|c| matches!((zh_map.get(c), en_map.get(c)), (Some(z), Some(e)) if z == e));
    if !ok {
        violations.push(format!(
            "{path}.{field}: 译文字段缺少配套的语言无关 key（候选名 {cands:?}）"
        ));
    }
}

fn dto_pair(lang: Language) -> (Value, Value, Value) {
    let chart = by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        lang,
        Config::default(),
    )
    .unwrap();
    let horoscope = get_horoscope(&chart, "2025-1-1", 0, lang).unwrap();
    let patterns = chart.patterns_dto(&x_iztro::PatternConfig::default());
    (
        serde_json::to_value(chart.to_dto()).unwrap(),
        serde_json::to_value(horoscope.to_dto(lang)).unwrap(),
        serde_json::to_value(patterns).unwrap(),
    )
}

/// 星盘、运限、格局三类 DTO 的全部译文字段都有配套 key。
#[test]
fn every_translated_field_has_a_language_neutral_key() {
    let (zh_chart, zh_horoscope, zh_patterns) = dto_pair(Language::ZhCN);
    let (en_chart, en_horoscope, en_patterns) = dto_pair(Language::EnUS);

    let mut violations = Vec::new();
    walk(&zh_chart, &en_chart, "astrolabe", &mut violations);
    walk(&zh_horoscope, &en_horoscope, "horoscope", &mut violations);
    walk(&zh_patterns, &en_patterns, "patterns", &mut violations);

    assert!(
        violations.is_empty(),
        "语义化契约违约（译文字段无配套 key）：\n{}",
        violations.join("\n")
    );
}

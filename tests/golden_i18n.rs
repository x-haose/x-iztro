//! Golden i18n 测试：全部可翻译标识 × 六种语言逐格对照 JS 输出。
//!
//! 数据由 tests/golden/generate_i18n.mjs 从 iztro 的六个 locale 落盘。
//! 覆盖 x-iztro 的 `translate_key`（对应 iztro 的 `t`）与
//! `key_of`（对应 `kot`）：标识集合必须完全一致，每格译文必须相同，
//! 且每条译文都能反查回原标识。

use serde_json::Value;
use std::collections::BTreeSet;
use std::fs;
use x_iztro::data::types::Language;
use x_iztro::i18n::lookup::{all_keys, key_of, key_of_in, translate_key};

const GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/i18n_table.json");
const KOT_GOLDEN: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/i18n_kot.json");

/// iztro 的 en-US 表漏了 `childhood` 一条，其 `t()` 找不到译文时原样返回标识；
/// x-iztro 的英文译名同样是 `childhood`，两侧观感一致。此处显式断言这是唯一一处
/// iztro 表缺条目，iztro 哪天补上会立刻暴露。
const IZTRO_MISSING_EN: &str = "childhood";

fn load() -> Value {
    let raw =
        fs::read_to_string(GOLDEN).expect("缺少 i18n_table.json，先跑 node generate_i18n.mjs");
    serde_json::from_str(&raw).expect("i18n_table.json 不是合法 JSON")
}

fn lang_of(code: &str) -> Language {
    Language::from_code(code).unwrap_or_else(|| panic!("未知语言代码 {code}"))
}

#[test]
fn key_set_matches_js() {
    let golden = load();
    let want: BTreeSet<String> = golden
        .as_object()
        .expect("i18n_table 不是对象")
        .keys()
        .cloned()
        .collect();
    let got: BTreeSet<String> = all_keys().into_iter().map(String::from).collect();

    let missing: Vec<_> = want.difference(&got).collect();
    let extra: Vec<_> = got.difference(&want).collect();

    assert!(
        missing.is_empty() && extra.is_empty(),
        "标识集合不一致\n  Rust 缺: {missing:?}\n  Rust 多: {extra:?}"
    );
    assert_eq!(got.len(), 260, "标识总数应为 260");
}

#[test]
fn translations_match_js() {
    let golden = load();
    let mut failures = Vec::new();
    let mut missing_en_hits = 0usize;

    for (key, langs) in golden.as_object().expect("i18n_table 不是对象") {
        let langs = langs.as_object().expect("语言表不是对象");

        for code in ["zh-CN", "zh-TW", "en-US", "ja-JP", "ko-KR", "vi-VN"] {
            let lang = lang_of(code);
            let got = translate_key(key, lang);

            // iztro 表里没有的条目在落盘时整键丢失
            let Some(want) = langs.get(code).and_then(Value::as_str) else {
                assert_eq!(
                    key, IZTRO_MISSING_EN,
                    "{key}/{code}: iztro 缺译文，超出已知范围"
                );
                assert_eq!(code, "en-US", "{key}: iztro 缺译文的语言超出已知范围");
                assert!(got.is_some(), "{key}/{code}: x-iztro 应当补上译文");
                missing_en_hits += 1;
                continue;
            };

            match got {
                Some(got) if got == want => {}
                Some(got) => failures.push(format!("{key}/{code}: JS={want} Rust={got}")),
                None => failures.push(format!("{key}/{code}: Rust 无法翻译该标识")),
            }
        }
    }

    assert_eq!(missing_en_hits, 1, "iztro 缺译文的条目数与预期不符");
    assert!(failures.is_empty(), "译文差异:\n{}", failures.join("\n"));
}

#[test]
fn every_translation_resolves_back_to_a_key() {
    let golden = load();
    let mut failures = Vec::new();

    for (key, langs) in golden.as_object().expect("i18n_table 不是对象") {
        for (code, want) in langs.as_object().expect("语言表不是对象") {
            let Some(text) = want.as_str() else { continue };

            // 反查按标识顺序取先命中者；同名译文（如「天马」与「流马」在
            // 部分语言下同形）只要能落到某个译文相同的标识即可
            match key_of(text) {
                Some(found) => {
                    let same_text = translate_key(found, lang_of(code)) == Some(text)
                        || x_iztro::i18n::lookup::all_keys()
                            .iter()
                            .any(|k| *k == found && *k == key);
                    if !same_text {
                        failures.push(format!("{key}/{code}: 「{text}」反查到 {found}，译文不符"));
                    }
                }
                None => failures.push(format!("{key}/{code}: 「{text}」反查不到标识")),
            }
        }
    }

    assert!(failures.is_empty(), "反查差异:\n{}", failures.join("\n"));
}

/// 反查必须与 iztro 的 `kot` 逐例同解。
///
/// 同形译名（en-US 的 `horse` 既是生肖马也是天马、ko-KR 的 `사` 既是巳也是死）
/// 落到哪个标识，取决于扫描时语言与标识的先后。上面那条只要求「反查到译文相同
/// 的标识」，查不出顺序分叉，故此处逐例比对 kot 的实际取值。
#[test]
fn key_lookup_matches_kot() {
    let raw =
        fs::read_to_string(KOT_GOLDEN).expect("缺少 i18n_kot.json，先跑 node generate_i18n.mjs");
    let cases: Vec<Value> = serde_json::from_str(&raw).expect("i18n_kot.json 不是合法 JSON");

    let mut failures = Vec::new();
    for case in &cases {
        let text = case["text"].as_str().expect("缺 text");
        let want = case["kot"].as_str().expect("缺 kot");
        let got = key_of(text);
        if got != Some(want) {
            let lang = case["lang"].as_str().unwrap_or("?");
            failures.push(format!(
                "「{text}」[{lang}]: iztro kot={want}，x-iztro key_of={got:?}"
            ));
        }
    }

    assert!(
        failures.is_empty(),
        "反查取值与 kot 不一致（{} / {} 例）:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}

/// 限定标识名后的反查对应 iztro `kot` 的第二个参数，用于消歧同形译名。
#[test]
fn filtered_lookup_disambiguates_homonyms() {
    // en-US 下 horse 既是生肖马也是天马，dragon 既是生肖龙也是青龙
    assert_eq!(key_of("horse"), Some("horse"));
    assert_eq!(key_of_in("horse", "Min"), Some("tianmaMin"));
    assert_eq!(key_of("dragon"), Some("dragon"));
    assert_eq!(key_of_in("dragon", "qing"), Some("qinglong"));

    // 时辰与运限层级在 ko-KR 下同为「유시」
    assert_eq!(key_of("유시"), Some("hourly"));
    assert_eq!(key_of_in("유시", "Hour"), Some("roosterHour"));

    // 限定后无匹配则返回 None，不退回未限定的结果
    assert_eq!(key_of_in("horse", "Palace"), None);
}

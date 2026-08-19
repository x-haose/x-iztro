//! 知识包：内嵌默认包的完整性、键与内核标识的一致性、合并语义与绑定入口。

use std::ffi::{CStr, CString};

use serde_json::{Value, json};
use x_iztro::ffi::{iztro_free_string, iztro_query};
use x_iztro::pattern::ALL_PATTERNS;
use x_iztro::{KnowledgePack, Language, Mutagen, Palace, PatternKey, StarKey};

fn query(payload: Value) -> Result<Value, String> {
    let input = CString::new(payload.to_string()).unwrap();
    let ptr = unsafe { iztro_query(input.as_ptr()) };
    let out = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
    unsafe { iztro_free_string(ptr) };
    let out: Value = serde_json::from_str(&out).unwrap();
    match out.get("error") {
        Some(e) => Err(e.as_str().unwrap_or_default().to_string()),
        None => Ok(out["value"].clone()),
    }
}

#[test]
fn builtin_zh_cn_covers_core_entries_with_valid_keys() {
    let p = KnowledgePack::builtin(Language::ZhCN).unwrap();
    assert_eq!(p.id, "iztro-docs");
    assert_eq!(p.source.license.as_deref(), Some("MIT"));
    assert!(p.source.commit.as_deref().is_some_and(|c| c.len() >= 7));

    for k in p.stars.keys() {
        assert!(StarKey::from_key(k).is_some(), "unknown star key {k}");
    }
    for k in p.patterns.keys() {
        assert!(PatternKey::from_key(k).is_some(), "unknown pattern key {k}");
    }
    for k in p.palaces.keys() {
        assert!(Palace::from_key(k).is_some(), "unknown palace key {k}");
    }
    for k in p.mutagens.keys() {
        assert!(Mutagen::from_key(k).is_some(), "unknown mutagen key {k}");
    }

    // 主星与辅星各 14 颗都有条目、有正文、有阴阳五行；全部格局、十二宫、四化都有条目
    let majors: Vec<_> = StarKey::from_key("ziweiMaj").into_iter().collect();
    assert!(!majors.is_empty());
    let mut major_n = 0;
    let mut minor_n = 0;
    for (k, e) in &p.stars {
        match e.category.as_deref() {
            Some("major") => major_n += 1,
            Some("minor") => minor_n += 1,
            _ => continue,
        }
        assert!(
            e.intro.as_deref().is_some_and(|s| !s.trim().is_empty()),
            "{k} intro"
        );
        assert!(e.name.is_some(), "{k} name");
        let a = &e.attributes;
        assert!(
            matches!(a.yin_yang.as_deref(), Some("yin" | "yang")),
            "{k} yinYang {:?}",
            a.yin_yang
        );
        assert!(
            matches!(
                a.five_elements.as_deref(),
                Some("wood" | "fire" | "earth" | "metal" | "water")
            ),
            "{k} fiveElements {:?}",
            a.five_elements
        );
    }
    assert_eq!((major_n, minor_n), (14, 14));
    for k in ALL_PATTERNS {
        let e = p
            .pattern(k)
            .unwrap_or_else(|| panic!("pattern {}", k.as_key()));
        assert!(
            e.intro.is_some() || e.quotes.is_some(),
            "pattern {} has no text",
            k.as_key()
        );
    }
    assert_eq!(p.palaces.len(), 12);
    assert_eq!(p.mutagens.len(), 4);
    assert!(
        p.star(StarKey::ZiweiMaj)
            .unwrap()
            .combinations
            .contains_key("tianfuMaj")
    );
    assert!(p.star_intro(StarKey::TianjiMaj).is_some());
    assert!(p.pattern_intro(PatternKey::ZiFuTongGong).is_some());
    assert!(KnowledgePack::builtin(Language::EnUS).is_none());
}

#[test]
fn ffi_returns_builtin_and_merges_overlays() {
    let builtin = query(json!({"kind": "knowledgePack", "language": "zh-CN"})).unwrap();
    assert_eq!(builtin["id"], "iztro-docs");
    let err = query(json!({"kind": "knowledgePack", "language": "en-US"})).unwrap_err();
    assert!(err.contains("no builtin knowledge pack"), "{err}");

    let overlay = json!({
        "schema": 1, "id": "mine", "version": "1", "language": "zh-CN", "extends": "iztro-docs",
        "stars": {"ziweiMaj": {"intro": "我的紫微"}},
        "patterns": {"zi_fu_tong_gong": {"intro": "我的紫府同宫"}}
    });
    let merged = query(json!({
        "kind": "mergeKnowledgePacks",
        "knowledgePacks": [builtin.clone(), overlay]
    }))
    .unwrap();
    assert_eq!(merged["id"], "mine");
    assert_eq!(merged["stars"]["ziweiMaj"]["intro"], "我的紫微");
    assert_eq!(merged["stars"]["ziweiMaj"]["name"], "紫微");
    assert_eq!(merged["stars"]["tianjiMaj"], builtin["stars"]["tianjiMaj"]);
    assert_eq!(
        merged["patterns"]["zi_fu_tong_gong"]["intro"],
        "我的紫府同宫"
    );
    assert_eq!(
        merged["patterns"]["zi_fu_tong_gong"]["quotes"],
        builtin["patterns"]["zi_fu_tong_gong"]["quotes"]
    );

    let err = query(json!({"kind": "mergeKnowledgePacks", "knowledgePacks": []})).unwrap_err();
    assert!(err.contains("at least one"), "{err}");
    let err = query(json!({"kind": "mergeKnowledgePacks", "knowledgePacks": [{"schema": 9}]}))
        .unwrap_err();
    assert!(err.contains("schema"), "{err}");
}

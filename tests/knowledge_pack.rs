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

    // 全部星耀条目都要有类别、名称与正文，属性字段一旦出现就必须落在值域内；
    // 主星与辅星各 14 颗还必须有阴阳五行（杂耀与神煞类的阴阳五行门派表述不全，允许缺省）
    let majors: Vec<_> = StarKey::from_key("ziweiMaj").into_iter().collect();
    assert!(!majors.is_empty());
    let mut major_n = 0;
    let mut minor_n = 0;
    for (k, e) in &p.stars {
        let category = e.category.as_deref();
        assert!(
            matches!(
                category,
                Some("major" | "minor" | "adjective" | "dec" | "flow")
            ),
            "{k} category {category:?}"
        );
        assert!(
            e.intro.as_deref().is_some_and(|s| !s.trim().is_empty()),
            "{k} intro"
        );
        assert!(e.name.is_some(), "{k} name");
        let a = &e.attributes;
        if let Some(v) = a.yin_yang.as_deref() {
            assert!(matches!(v, "yin" | "yang"), "{k} yinYang {v:?}");
        }
        if let Some(v) = a.five_elements.as_deref() {
            assert!(
                matches!(v, "wood" | "fire" | "earth" | "metal" | "water"),
                "{k} fiveElements {v:?}"
            );
        }
        if matches!(category, Some("major" | "minor")) {
            match category {
                Some("major") => major_n += 1,
                _ => minor_n += 1,
            }
            assert!(a.yin_yang.is_some(), "{k} 主辅星必须有阴阳");
            assert!(a.five_elements.is_some(), "{k} 主辅星必须有五行");
        }
    }
    assert_eq!((major_n, minor_n), (14, 14));

    // 反向覆盖：内核的每个星耀标识都必须有知识条目——上面的正向校验只保证
    // 「包里的键都合法」，漏写条目（尤其新增 StarKey 时）靠这条兜住
    for star in x_iztro::data::stars::ALL_STARS {
        assert!(
            p.stars.contains_key(star.as_key()),
            "star {} has no knowledge entry",
            star.as_key()
        );
    }

    // 流耀条目与内核对照表一一对应：每颗流耀都有 flow 类条目，且内核能给出其本命对应
    for (flow, natal) in x_iztro::astro::horoscope::flow_star_counterparts() {
        let e = p.stars.get(flow.as_key()).expect("flow star entry");
        assert_eq!(e.category.as_deref(), Some("flow"), "{}", flow.as_key());
        assert!(
            StarKey::from_key(natal.as_key()).is_some(),
            "counterpart {}",
            natal.as_key()
        );
    }
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
    assert_eq!(p.concepts.len(), 49);
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

/// 默认包文本的口吻红线：教学博客式的对话残留、页面布局引用与站点功能引用一律不许出现。
/// 默认包是手工维护的释义文本，此断言防止后续编辑（尤其同步上游时）把原文口吻带回来。
#[test]
fn builtin_texts_keep_reference_tone() {
    const BANNED: &[&str] = &[
        "读者",
        "大家肯定",
        "大家可能",
        "大家仔细",
        "你妈妈",
        "我们看到",
        "上面的名片表",
        "见上表",
        "上表中",
        "如上表",
        "见下表",
        "下表中",
        "从上面",
        "前面说的",
        "前面的逻辑",
        "前面提到",
        "详细介绍见",
        "详见",
        "参见",
        "本站",
        "点击",
        "小编",
        "单车变摩托",
        "宝马变奥拓",
        "韭菜",
        "摆烂",
        "C位",
        "小透明",
    ];
    let p = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let mut texts: Vec<(String, &str)> = Vec::new();
    for (k, e) in &p.stars {
        if let Some(t) = e.intro.as_deref() {
            texts.push((format!("stars.{k}.intro"), t));
        }
        for (ck, cv) in &e.combinations {
            texts.push((format!("stars.{k}.combinations.{ck}"), cv));
        }
    }
    for (k, e) in &p.patterns {
        if let Some(t) = e.intro.as_deref() {
            texts.push((format!("patterns.{k}.intro"), t));
        }
        if let Some(t) = e.conditions.as_deref() {
            texts.push((format!("patterns.{k}.conditions"), t));
        }
    }
    for (k, e) in p.palaces.iter().chain(p.mutagens.iter()) {
        if let Some(t) = e.intro.as_deref() {
            texts.push((format!("{k}.intro"), t));
        }
    }
    for (k, e) in &p.concepts {
        if let Some(t) = e.intro.as_deref() {
            texts.push((format!("concepts.{k}.intro"), t));
        }
    }
    let mut violations = Vec::new();
    for (loc, t) in &texts {
        for w in BANNED {
            if t.contains(w) {
                violations.push(format!("{loc}: {w}"));
            }
        }
    }
    assert!(
        violations.is_empty(),
        "口吻红线违例:
{}",
        violations.join(
            "
"
        )
    );
    assert!(
        p.source.adapted.as_deref().is_some_and(|a| !a.is_empty()),
        "默认包文本经改写，source.adapted 必须注明"
    );
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

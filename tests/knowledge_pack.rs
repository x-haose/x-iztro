//! 知识包：内嵌默认包的完整性、键与内核标识的一致性、合并语义与绑定入口。

use std::ffi::{CStr, CString};

use serde_json::{Value, json};
use x_iztro::ffi::{iztro_free_string, iztro_query};
use x_iztro::pattern::ALL_PATTERNS;
use x_iztro::{Config, Gender, KnowledgePack, Language, Mutagen, Palace, PatternKey, StarKey};

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
    assert_eq!(p.concepts.len(), 51);
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
        if let Some(t) = e.summary.as_deref() {
            texts.push((format!("stars.{k}.summary"), t));
        }
        for (ck, cv) in e.combinations.iter().chain(&e.combination_summaries) {
            texts.push((format!("stars.{k}.combinations.{ck}"), cv));
        }
    }
    for (k, e) in &p.patterns {
        for (field, t) in [
            ("intro", &e.intro),
            ("summary", &e.summary),
            ("conditions", &e.conditions),
        ] {
            if let Some(t) = t.as_deref() {
                texts.push((format!("patterns.{k}.{field}"), t));
            }
        }
    }
    for (k, e) in p.palaces.iter().chain(p.mutagens.iter()) {
        for (field, t) in [("intro", &e.intro), ("summary", &e.summary)] {
            if let Some(t) = t.as_deref() {
                texts.push((format!("{k}.{field}"), t));
            }
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

/// 默认包的摘要写作规则（SCHEMA §5.1）：正文超过上限的条目都有摘要、整包可用于摘要层；
/// 正文不超过上限的条目不写摘要（同一内容不存两份）；摘要单段、宽度不超过 100（中文 50 字）；
/// 组合摘要只对应已有的组合解读。
#[test]
fn builtin_summaries_follow_authoring_rules() {
    use unicode_width::UnicodeWidthStr;
    use x_iztro::knowledge::SUMMARY_FALLBACK_WIDTH;
    const SUMMARY_MAX_WIDTH: usize = 100;

    let p = KnowledgePack::builtin(Language::ZhCN).unwrap();
    assert_eq!(p.missing_summaries(), Vec::<String>::new());

    let mut pairs: Vec<(String, &str, Option<&str>)> = Vec::new();
    for (k, e) in &p.stars {
        pairs.push((
            format!("stars.{k}"),
            e.intro.as_deref().unwrap_or(""),
            e.summary.as_deref(),
        ));
        for other in e.combination_summaries.keys() {
            assert!(
                e.combinations.contains_key(other),
                "stars.{k}.combinationSummaries.{other} 没有对应的组合解读"
            );
        }
        for (other, text) in &e.combinations {
            pairs.push((
                format!("stars.{k}.combinations.{other}"),
                text,
                e.combination_summaries.get(other).map(String::as_str),
            ));
        }
    }
    for (k, e) in &p.patterns {
        pairs.push((
            format!("patterns.{k}"),
            e.intro.as_deref().unwrap_or(""),
            e.summary.as_deref(),
        ));
    }
    for (section, entries) in [("palaces", &p.palaces), ("mutagens", &p.mutagens)] {
        for (k, e) in entries {
            pairs.push((
                format!("{section}.{k}"),
                e.intro.as_deref().unwrap_or(""),
                e.summary.as_deref(),
            ));
        }
    }
    for (path, body, summary) in pairs {
        let Some(summary) = summary else { continue };
        assert!(
            body.width() > SUMMARY_FALLBACK_WIDTH,
            "{path} 正文不超过上限，摘要层直接取正文，不应另写摘要"
        );
        assert!(
            summary.width() <= SUMMARY_MAX_WIDTH && !summary.contains('\n'),
            "{path} 摘要应为单段且宽度不超过 {SUMMARY_MAX_WIDTH}：{summary}"
        );
    }
}

/// 默认包的主题：宫位标识都认得、每个主题至少取一宫、都写了互不相同的显示顺序，
/// 且在测试盘上两种释义深度都渲染得出。
#[test]
fn builtin_topics_resolve_on_a_chart() {
    use x_iztro::models::astrolabe::PalaceTarget;
    use x_iztro::{TextOptions, by_solar};

    let p = KnowledgePack::builtin(Language::ZhCN).unwrap();
    assert!(!p.topics.is_empty());
    let chart = by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();
    let full = TextOptions::new().knowledge(p);
    let summary = TextOptions::new().knowledge_summary(p).unwrap();
    let mut orders = std::collections::BTreeSet::new();
    for (slug, topic) in &p.topics {
        let keys: Vec<&String> = topic
            .surround
            .iter()
            .chain(&topic.palaces)
            .flatten()
            .collect();
        assert!(!keys.is_empty(), "topics.{slug} 没有取任何宫");
        for key in keys {
            assert!(
                PalaceTarget::from_key(key).is_some(),
                "topics.{slug} 的宫位标识 {key} 不认识"
            );
        }
        assert!(
            topic.title.is_some() && topic.intro.is_some(),
            "topics.{slug}"
        );
        let order = topic
            .order
            .unwrap_or_else(|| panic!("topics.{slug} 没有 order"));
        assert!(
            orders.insert(order),
            "topics.{slug} 的 order {order} 与其他主题相同"
        );
        for opts in [&full, &summary] {
            chart
                .topic_to_text(slug, opts)
                .unwrap_or_else(|e| panic!("topics.{slug}: {e}"));
        }
    }
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

/// 按盘取材的往返：子包里的每颗星都在盘上、盘上每颗有条目的星都进了子包，
/// 同宫主星的组合解读保留而不同宫的剔除，宫位与术语不进子包。
#[test]
fn for_astrolabe_selects_exactly_the_chart_material() {
    use x_iztro::{Astrolabe, by_solar};
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let chart: Astrolabe = by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();
    let sub = pack.for_astrolabe(&chart);

    let mut on_chart: Vec<StarKey> = Vec::new();
    for p in &chart.palaces {
        on_chart.extend(p.major_stars.iter().map(|s| s.key));
        on_chart.extend(p.minor_stars.iter().map(|s| s.key));
        on_chart.extend(p.adjective_stars.iter().map(|s| s.key));
        on_chart.extend([p.changsheng12, p.boshi12, p.suiqian12, p.jiangqian12]);
    }
    for key in sub.stars.keys() {
        let star = StarKey::from_key(key).unwrap();
        assert!(on_chart.contains(&star), "{key} 不在盘上却进了子包");
    }
    for star in &on_chart {
        if pack.star(*star).is_some() {
            assert!(
                sub.star(*star).is_some(),
                "{} 在盘上却没进子包",
                star.as_key()
            );
        }
    }
    // 双星组合：只保留同宫的对方主星
    for (key, entry) in &sub.stars {
        let star = StarKey::from_key(key).unwrap();
        let palace = chart
            .palaces
            .iter()
            .find(|p| p.major_stars.iter().any(|s| s.key == star));
        for other in entry.combinations.keys() {
            let other = StarKey::from_key(other).unwrap();
            assert!(
                palace.is_some_and(|p| p.major_stars.iter().any(|s| s.key == other)),
                "{key} 的组合解读对方 {} 不在同宫",
                other.as_key()
            );
        }
    }
    // 格局：命中的才在
    let hits: Vec<PatternKey> = chart.patterns().iter().map(|h| h.key).collect();
    for key in sub.patterns.keys() {
        assert!(hits.contains(&PatternKey::from_key(key).unwrap()));
    }
    for hit in &hits {
        assert!(sub.pattern(*hit).is_some());
    }
    assert_eq!(sub.mutagens.len(), 4);
    assert!(sub.palaces.is_empty() && sub.concepts.is_empty());
    assert_eq!(
        (sub.id.as_str(), sub.schema),
        (pack.id.as_str(), pack.schema)
    );

    // 格局口径随入参：Positional 口径下多命中的格局要进子包
    let positional = x_iztro::PatternConfig {
        brightness_source: x_iztro::BrightnessSource::Positional,
        ..x_iztro::PatternConfig::default()
    };
    let sub_pos = pack.for_astrolabe_with(&chart, &positional);
    let hits_pos: Vec<PatternKey> = chart
        .patterns_with(&positional)
        .iter()
        .map(|h| h.key)
        .collect();
    for hit in &hits_pos {
        assert!(
            sub_pos.pattern(*hit).is_some(),
            "{} 未按口径进入子包",
            hit.as_key()
        );
    }
    assert_eq!(sub_pos.patterns.len(), {
        let mut uniq = hits_pos.clone();
        uniq.sort_by_key(|k| k.as_key());
        uniq.dedup();
        uniq.len()
    });

    // 运限取材：在本命之上多出流耀条目与各层格局
    let h = chart.horoscope("2025-1-1", 0).unwrap();
    let sub_h = pack.for_horoscope(&h);
    assert!(sub_h.stars.len() > sub.stars.len());
    assert!(
        sub_h
            .stars
            .keys()
            .any(|k| k.starts_with("liu") || k.starts_with("yun"))
    );
    for (k, e) in &sub.stars {
        assert_eq!(sub_h.stars.get(k), Some(e), "运限子包必须包含本命子包 {k}");
    }
}

/// bridge：to_text kind 的 `knowledge` 入参与 `knowledgeForChart` kind。
#[test]
fn ffi_text_with_knowledge_and_chart_selection() {
    let base = json!({
        "solarDate": "2000-8-16", "timeIndex": 2, "gender": "female",
        "fixLeap": true, "language": "zh-CN",
    });
    let run = |extra: Value| {
        let mut q = base.clone();
        for (k, v) in extra.as_object().unwrap() {
            q[k] = v.clone();
        }
        query(q)
    };

    let plain = run(json!({"kind": "astrolabeToText"})).unwrap();
    let with = run(json!({"kind": "astrolabeToText", "knowledge": "builtin"})).unwrap();
    let with = with.as_str().unwrap();
    // 事实文本是带释义文本的有序子序列（同一渲染器）
    let mut lines = with.lines();
    for want in plain.as_str().unwrap().lines() {
        assert!(
            lines.any(|got| got == want),
            "带释义文本缺行或顺序不同：{want:?}"
        );
    }
    assert!(with.contains("## 四化释义"));
    assert!(with.contains("**禄**: "));

    // 自定义包：覆盖后的释义进文本
    let builtin = query(json!({"kind": "knowledgePack", "language": "zh-CN"})).unwrap();
    let overlay = json!({
        "schema": 1, "id": "mine", "version": "1", "language": "zh-CN",
        "stars": {"wuquMaj": {"intro": "我的武曲释义"}}
    });
    let merged = query(json!({
        "kind": "mergeKnowledgePacks", "knowledgePacks": [builtin, overlay]
    }))
    .unwrap();
    let custom = run(json!({"kind": "astrolabeToText", "knowledge": merged.clone()})).unwrap();
    assert!(custom.as_str().unwrap().contains("我的武曲释义"));

    // 六个 kind 都收 knowledge
    for (kind, extra) in [
        (
            "horoscopeToText",
            json!({"targetDate": "2025-1-1", "targetTimeIndex": 0}),
        ),
        ("palaceToText", json!({"palaceKey": "soulPalace"})),
        (
            "surroundedPalacesToText",
            json!({"palaceKey": "soulPalace"}),
        ),
        ("patternsToText", json!({})),
        (
            "horoscopePatternsToText",
            json!({"scope": "decadal", "targetDate": "2025-1-1"}),
        ),
    ] {
        let mut q = extra.clone();
        q["kind"] = json!(kind);
        q["knowledge"] = json!("builtin");
        let text = run(q).unwrap();
        assert!(text.as_str().unwrap().contains("**"), "{kind} 应带释义");
    }

    // 子包 kind：本命与运限两种取材
    let sub = run(json!({"kind": "knowledgeForChart", "knowledge": "builtin"})).unwrap();
    assert_eq!(sub["mutagens"].as_object().unwrap().len(), 4);
    assert!(sub["palaces"].as_object().is_none_or(|m| m.is_empty()));
    let sub_h = run(json!({
        "kind": "knowledgeForChart", "knowledge": "builtin",
        "targetDate": "2025-1-1", "targetTimeIndex": 0
    }))
    .unwrap();
    assert!(sub_h["stars"].as_object().unwrap().len() > sub["stars"].as_object().unwrap().len());

    // 子包 kind 转发 patternConfig：两种口径的格局集合与 patterns kind 一致
    for cfg in [json!(null), json!({"brightnessSource": "positional"})] {
        let sub =
            run(json!({"kind": "knowledgeForChart", "knowledge": "builtin", "patternConfig": cfg}))
                .unwrap();
        let hits = run(json!({"kind": "patterns", "patternConfig": cfg})).unwrap();
        let mut expected: Vec<String> = hits
            .as_array()
            .unwrap()
            .iter()
            .map(|h| h["key"].as_str().unwrap().to_string())
            .collect();
        expected.sort();
        expected.dedup();
        let mut got: Vec<String> = sub["patterns"]
            .as_object()
            .unwrap()
            .keys()
            .cloned()
            .collect();
        got.sort();
        assert_eq!(got, expected, "patternConfig={cfg}");
    }

    // 错误路径：英文盘无内嵌包、非法 knowledge 串、子包 kind 缺 knowledge
    let mut en = base.clone();
    en["language"] = json!("en-US");
    en["kind"] = json!("astrolabeToText");
    en["knowledge"] = json!("builtin");
    assert!(query(en).unwrap_err().contains("no builtin knowledge pack"));
    assert!(run(json!({"kind": "astrolabeToText", "knowledge": "nope"})).is_err());
    assert!(run(json!({"kind": "knowledgeForChart"})).is_err());
}

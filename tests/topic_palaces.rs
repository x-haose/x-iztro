//! 按主题选宫：四侧共用用例 `tests/data/topic_palaces_cases.json`。每条用例的选宫结果都按该层的宫名表、
//! 本命的身宫与来因宫标记、各宫宫干的四化表另行推一遍并逐项核对，再与用例里的期望（四侧共同的基线）比对；
//! 另核本命层与 `topic_to_text` 逐字一致、运限层文本按该层宫名成段，以及层级比查询精度细时报错。
//!
//! 期望缺失或与当前输出不同都是失败；有意改动后用 `UPDATE_TOPIC_CASES=1` 重跑本测试写入。

use serde_json::{Value, json};
use x_iztro::data::stars::MUTAGEN;
use x_iztro::{
    Astrolabe, Config, Gender, HoroscopeRef, KnowledgePack, Language, Palace, PalaceTarget,
    PatternConfig, Scope, StemSource, TextDetail, TextOptions, TopicGroupKind, TopicSelection,
    by_solar, patterns_at, translate_pattern,
};

const CASES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/topic_palaces_cases.json"
);

fn chart(c: &Value) -> Astrolabe {
    let gender = match c["gender"].as_str().unwrap() {
        "male" => Gender::Male,
        _ => Gender::Female,
    };
    by_solar(
        c["solarDate"].as_str().unwrap(),
        c["timeIndex"].as_u64().unwrap() as u8,
        gender,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap()
}

fn horoscope<'a>(chart: &'a Astrolabe, t: &Value) -> HoroscopeRef<'a> {
    match (
        t.get("targetDate"),
        t.get("targetYear"),
        t.get("targetMonth"),
    ) {
        (Some(d), _, _) => chart
            .horoscope(
                d.as_str().unwrap(),
                t["targetTimeIndex"].as_u64().map(|i| i as u8),
            )
            .unwrap(),
        (None, Some(y), Some(m)) => chart
            .horoscope_of_month(y.as_i64().unwrap(), m.as_u64().unwrap() as u32)
            .unwrap(),
        (None, Some(y), None) => chart.horoscope_of_year(y.as_i64().unwrap()).unwrap(),
        _ => panic!("用例的 target 不完整：{t}"),
    }
}

fn pack(case: &Value) -> KnowledgePack {
    match case.get("knowledge") {
        Some(v) => KnowledgePack::from_value(v).unwrap(),
        None => KnowledgePack::builtin(Language::ZhCN).unwrap().clone(),
    }
}

/// 按用例取选宫结果，连同所选层（本命为 `None`）与该层宫名表
fn select(case: &Value, chart: &Astrolabe, pack: &KnowledgePack) -> TopicSelection {
    let topic = case["topic"].as_str().unwrap();
    match case.get("target") {
        None => chart.topic_palaces(pack, topic).unwrap(),
        Some(t) => {
            let scope = Scope::from_key(case["scope"].as_str().unwrap()).unwrap();
            horoscope(chart, t)
                .topic_palaces(pack, topic, scope)
                .unwrap()
        }
    }
}

/// 不经选宫逻辑另行推出各组本宫、各宫的层宫名、宫干飞化与三类四化，与选宫结果逐项核对。
fn verify_by_hand(case: &Value, chart: &Astrolabe, pack: &KnowledgePack, got: &TopicSelection) {
    let name = case["name"].as_str().unwrap();
    let entry = pack.topic(case["topic"].as_str().unwrap()).unwrap();
    let h = case.get("target").map(|t| horoscope(chart, t));
    let scope = case.get("scope").map_or(Scope::Origin, |s| {
        Scope::from_key(s.as_str().unwrap()).unwrap()
    });
    // 该层宫名表：本命取本命宫名，运限层取该层重排宫名
    let names: Vec<Palace> = match (&h, scope) {
        (Some(h), s) if s != Scope::Origin => {
            let item = match s {
                Scope::Decadal => &h.decadal,
                Scope::Yearly => &h.yearly.base,
                Scope::Monthly => &h.monthly,
                Scope::Daily => &h.daily,
                _ => &h.hourly,
            };
            item.palace_names.clone()
        }
        _ => chart.palaces.iter().map(|p| p.name).collect(),
    };
    let target = |key: &str| -> Option<usize> {
        match PalaceTarget::from_key(key).unwrap() {
            PalaceTarget::Name(p) => names.iter().position(|n| *n == p),
            PalaceTarget::Body => chart.palaces.iter().position(|p| p.is_body_palace),
            PalaceTarget::Original => chart.palaces.iter().position(|p| p.is_original_palace),
            PalaceTarget::Index(i) => Some(i),
        }
    };
    let mut want_groups = Vec::new();
    for key in entry.surround.iter().flatten() {
        if let Some(i) = target(key) {
            want_groups.push((
                TopicGroupKind::Surround,
                i,
                Some((i + 6) % 12),
                Some((i + 8) % 12),
                Some((i + 4) % 12),
            ));
        }
    }
    for key in entry.palaces.iter().flatten() {
        if let Some(i) = target(key) {
            want_groups.push((TopicGroupKind::Only, i, None, None, None));
        }
    }
    let got_groups: Vec<_> = got
        .groups
        .iter()
        .map(|g| (g.kind, g.target, g.opposite, g.wealth, g.career))
        .collect();
    assert_eq!(got_groups, want_groups, "{name}: 各组位置");

    let mut want_order: Vec<usize> = Vec::new();
    for (_, t, o, w, c) in &want_groups {
        for i in [Some(*t), *o, *w, *c].into_iter().flatten() {
            if !want_order.contains(&i) {
                want_order.push(i);
            }
        }
    }
    let got_order: Vec<usize> = got.palaces.iter().map(|p| p.index).collect();
    assert_eq!(got_order, want_order, "{name}: 选中宫及成段顺序");

    for p in &got.palaces {
        assert_eq!(p.name, names[p.index], "{name}: 宫 {} 的层宫名", p.index);
        assert_eq!(
            p.natal_name, chart.palaces[p.index].name,
            "{name}: 本命宫名"
        );
        let stem = chart.palaces[p.index].heavenly_stem;
        assert_eq!(p.stem, stem, "{name}: 宫干飞化取本命宫干");
        for (k, f) in p.flying.iter().enumerate() {
            let star = chart.config.mutagens_of(stem)[k];
            assert_eq!((f.mutagen, f.star), (MUTAGEN[k], star), "{name}: 宫干四化");
            assert_eq!(
                f.palace,
                chart.star(star).map(|s| s.palace().index),
                "{name}: 飞化落宫"
            );
        }
    }

    let birth = chart
        .config
        .mutagens_of(chart.raw_dates.chinese_date.yearly.0);
    let want_birth: Vec<_> = birth
        .iter()
        .zip(MUTAGEN)
        .filter_map(|(star, m)| {
            let i = chart.star(*star)?.palace().index;
            want_order.contains(&i).then_some((m, *star, i))
        })
        .collect();
    let got_birth: Vec<_> = got
        .birth_mutagens
        .iter()
        .map(|m| (m.mutagen, m.star, m.palace))
        .collect();
    assert_eq!(got_birth, want_birth, "{name}: 生年四化");

    let mut want_scope = Vec::new();
    if let Some(h) = &h {
        for (s, item) in [
            (Scope::Decadal, &h.decadal),
            (Scope::Yearly, &h.yearly.base),
            (Scope::Monthly, &h.monthly),
            (Scope::Daily, &h.daily),
            (Scope::Hourly, &h.hourly),
        ] {
            if s.as_key() == "decadal" && scope == Scope::Origin {
                break;
            }
            for (star, m) in item.mutagen.iter().zip(MUTAGEN) {
                if let Some(i) = chart.star(*star).map(|x| x.palace().index)
                    && want_order.contains(&i)
                {
                    want_scope.push((s, item.name_key, item.heavenly_stem, m, *star, i));
                }
            }
            if s == scope {
                break;
            }
        }
    }
    let got_scope: Vec<_> = got
        .scope_mutagens
        .iter()
        .map(|m| {
            (
                m.scope,
                m.name,
                m.stem,
                m.landing.mutagen,
                m.landing.star,
                m.landing.palace,
            )
        })
        .collect();
    assert_eq!(got_scope, want_scope, "{name}: 运限四化");
    assert_eq!(
        (got.palace_stem_source, got.scope_mutagen_stem_source),
        (StemSource::NatalPalace, StemSource::Scope),
        "{name}: 天干取法"
    );

    // 格局：该层视角的命中（只读大限时去掉要读更细一层的）里成格宫在选中宫内的
    let config = PatternConfig::default();
    let mut want_hits = match &h {
        Some(h) if scope != Scope::Origin => patterns_at(chart, h.data(), scope, &config),
        _ => chart.patterns_with(&config),
    };
    if scope == Scope::Decadal {
        want_hits.retain(|hit| !hit.key.needs_finer_layer());
    }
    want_hits.retain(|hit| want_order.contains(&hit.palace));
    assert_eq!(got.patterns, want_hits, "{name}: 格局");
}

/// 主题文本「格局」一节逐条所列的格局名，按出现顺序；没有这一节为空
fn text_pattern_names(text: &str) -> Vec<String> {
    let Some(start) = text.find("\n## 格局\n") else {
        return Vec::new();
    };
    text[start + "\n## 格局\n".len()..]
        .lines()
        .take_while(|line| line.starts_with("- **"))
        .map(|line| line["- **".len()..line[4..].find("**").unwrap() + 4].to_string())
        .collect()
}

/// 结构化结果的格局与主题文本「格局」一节逐条对应：默认包全部主题 × 本命与五个运限层 × 几张盘。
#[test]
fn selection_patterns_match_topic_text_section() {
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let opts = TextOptions::new().knowledge(pack);
    let mut with_hits = 0;
    for (date, time, gender) in [
        ("2000-8-16", 2, "female"),
        ("1997-3-4", 1, "male"),
        ("1985-6-10", 8, "female"),
        ("1964-11-2", 6, "male"),
    ] {
        let c = chart(&json!({"solarDate": date, "timeIndex": time, "gender": gender}));
        let h = c.horoscope("2026-11-20", 5).unwrap();
        for (slug, _) in pack.topics_in_order() {
            for scope in [
                Scope::Origin,
                Scope::Decadal,
                Scope::Yearly,
                Scope::Monthly,
                Scope::Daily,
                Scope::Hourly,
            ] {
                let got = h.topic_palaces(pack, slug, scope).unwrap();
                let text = h.topic_to_text(slug, scope, &opts).unwrap();
                let want: Vec<String> = got
                    .patterns
                    .iter()
                    .map(|hit| translate_pattern(hit.key, Language::ZhCN).to_string())
                    .collect();
                assert_eq!(
                    text_pattern_names(&text),
                    want,
                    "{date} {slug} {}",
                    scope.as_key()
                );
                with_hits += usize::from(!want.is_empty());
            }
        }
    }
    assert!(with_hits > 0, "用例里须有带格局的主题");
}

#[test]
fn shared_cases_match_hand_derivation_and_baseline() {
    let mut doc: Value = serde_json::from_str(&std::fs::read_to_string(CASES).unwrap()).unwrap();
    let update = std::env::var_os("UPDATE_TOPIC_CASES").is_some();
    let mut saw_childhood = false;
    for case in doc["cases"].as_array_mut().unwrap() {
        let chart = chart(&case["chart"]);
        let pack = pack(case);
        let got = select(case, &chart, &pack);
        verify_by_hand(case, &chart, &pack, &got);
        saw_childhood |= got
            .scope_mutagens
            .iter()
            .any(|m| m.name.as_key() == "childhood");
        let dto = serde_json::to_value(got.to_dto()).unwrap();
        if update {
            case["expect"] = dto;
        } else {
            assert_eq!(dto, case["expect"], "{}", case["name"]);
        }
    }
    assert!(saw_childhood, "用例须含未起运（童限）的一层");
    if update {
        std::fs::write(CASES, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    }
}

/// 维护者报告的盘：1997-3-4 丑时男，农历 2026 年十月流月看兄弟，取流月兄弟宫而不是本命兄弟宫。
#[test]
fn monthly_siblings_is_the_monthly_layer_palace() {
    let chart = chart(&json!({"solarDate": "1997-3-4", "timeIndex": 1, "gender": "male"}));
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let h = chart.horoscope_of_month(2026, 10).unwrap();
    let got = h.topic_palaces(pack, "siblings", Scope::Monthly).unwrap();
    let monthly = h
        .monthly
        .palace_names
        .iter()
        .position(|p| *p == Palace::Siblings)
        .unwrap();
    let natal = chart.palace(Palace::Siblings).unwrap().index;
    assert_ne!(monthly, natal);
    assert_eq!(got.groups[0].target, monthly);
    let text = h
        .topic_to_text(
            "siblings",
            Scope::Monthly,
            &TextOptions::new().knowledge(pack),
        )
        .unwrap();
    assert!(text.contains("## 流月兄弟 三方四正"), "{text}");
    assert!(text.contains("### 本宫 · 流月兄弟 (本命"), "{text}");
}

/// 本命层（含运限的 origin）与 `topic_to_text` 逐字一致：默认包全部主题 × 两种详略。
#[test]
fn origin_scope_text_equals_natal_topic_text() {
    let chart = chart(&json!({"solarDate": "2000-8-16", "timeIndex": 2, "gender": "female"}));
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let h = chart.horoscope("2025-1-1", 0).unwrap();
    for detail in [TextDetail::Full, TextDetail::Overview] {
        let opts = TextOptions::new().knowledge(pack).detail(detail);
        for (slug, _) in pack.topics_in_order() {
            assert_eq!(
                h.topic_to_text(slug, Scope::Origin, &opts).unwrap(),
                chart.topic_to_text(slug, &opts).unwrap(),
                "{slug}"
            );
        }
    }
}

/// 层级比查询精度细报错；未给知识包报错。
#[test]
fn errors() {
    let chart = chart(&json!({"solarDate": "2000-8-16", "timeIndex": 2, "gender": "female"}));
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let h = chart.horoscope_of_year(2025).unwrap();
    let err = h
        .topic_palaces(pack, "romance", Scope::Monthly)
        .unwrap_err();
    assert!(
        err.to_string()
            .contains("only specifies layers down to 'yearly'"),
        "{err}"
    );
    let err = h
        .topic_to_text("romance", Scope::Yearly, &TextOptions::new())
        .unwrap_err();
    assert!(
        err.to_string().contains("requires a knowledge pack"),
        "{err}"
    );
    let err = chart.topic_palaces(pack, "nope").unwrap_err();
    assert!(err.to_string().contains("unknown topic 'nope'"), "{err}");
}

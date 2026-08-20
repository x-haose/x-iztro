//! 格局引擎的对外接口面：Rust 方法、DTO 形状、bridge 分派与口径入参校验。

use std::ffi::{CStr, CString};

use serde_json::{Value, json};
use x_iztro::ffi::{iztro_free_string, iztro_query};
use x_iztro::{
    BrightnessSource, Config, Gender, Language, PatternConfig, PatternKey, Scope, by_solar,
};

fn chart() -> x_iztro::Astrolabe {
    by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap()
}

/// 经统一查询入口 `iztro_query` 发起查询；`extra` 覆盖或补充默认入参。
/// 返回 `Ok(结果)` 或 `Err(错误消息)`。
fn query(kind: &str, extra: Value) -> Result<Value, String> {
    let mut v = json!({
        "kind": kind,
        "solarDate": "2000-8-16",
        "timeIndex": 2,
        "gender": "female",
        "fixLeap": true,
        "language": "zh-CN",
    });
    v.as_object_mut()
        .unwrap()
        .extend(extra.as_object().unwrap().clone());
    let input = CString::new(v.to_string()).unwrap();
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
fn rust_api_natal_and_horoscope_agree_with_dto() {
    let a = chart();
    let hits = a.patterns();
    assert!(!hits.is_empty(), "该盘本命应有命中，空列表会让下方核对空转");
    let dto = a.patterns_dto(&PatternConfig::default());
    assert_eq!(hits.len(), dto.len());
    for (h, d) in hits.iter().zip(&dto) {
        assert_eq!(h.key.as_key(), d.key);
        assert_eq!(d.scope, "origin");
        assert_eq!(h.palace, d.palace_index);
        assert_eq!(h.stars.len(), d.stars.len());
        assert!(!d.name.is_empty() && !d.palace_name.is_empty());
    }
    let h = a.horoscope("2026-8-19", 3).unwrap();
    let dec = h.patterns(Scope::Decadal);
    assert!(
        !dec.is_empty(),
        "该盘大限层应有命中，空列表会让下方核对空转"
    );
    assert!(dec.iter().all(|x| x.scope == Scope::Decadal));
    assert_eq!(h.patterns(Scope::Origin), hits);
}

/// `flow_stars` 开关的行为：流曜等同本命辅星只在开关开时成立。
///
/// 该盘流年层的禄马交驰由禄存与流马同宫构成（流马是天马的流曜等价），
/// 关掉开关后流马不再算天马，这条命中随之消失；证据里不含流曜的命中不受影响。
#[test]
fn flow_stars_switch_changes_horoscope_judgement() {
    let a = chart();
    let h = a.horoscope("2026-8-19", 3).unwrap();

    let on = h.patterns_with(Scope::Yearly, &PatternConfig::default());
    let with_flow: Vec<_> = on
        .iter()
        .filter(|x| x.key == PatternKey::LuMaJiaoChi)
        .collect();
    assert!(
        with_flow
            .iter()
            .any(|x| x.stars.iter().any(|s| s.star == x_iztro::StarKey::Liuma)),
        "默认口径下该盘流年禄马交驰的证据应含流马"
    );

    let off = h.patterns_with(
        Scope::Yearly,
        &PatternConfig {
            flow_stars: false,
            ..PatternConfig::default()
        },
    );
    assert!(
        !off.iter().any(|x| x.key == PatternKey::LuMaJiaoChi),
        "关掉 flow_stars 后流马不算天马，该盘流年禄马交驰应消失"
    );
    assert!(
        off.iter().any(|x| x.key == PatternKey::FuXiangChaoYuan),
        "不依赖流曜的命中（府相朝垣）不应受开关影响"
    );
}

#[test]
fn bridge_patterns_returns_translated_hits() {
    let v = query("patterns", json!({})).unwrap();
    let arr = v.as_array().expect("array");
    assert!(!arr.is_empty(), "该盘本命应有命中，空列表会让下方核对空转");
    let a = chart();
    assert_eq!(arr.len(), a.patterns().len());
    for hit in arr {
        assert!(PatternKey::from_key(hit["key"].as_str().unwrap()).is_some());
        assert_eq!(hit["scope"], "origin");
        assert!(hit["palaceIndex"].as_u64().unwrap() < 12);
        assert!(hit["stars"].is_array());
    }
    let en = query("patterns", json!({"language": "en-US"})).unwrap();
    for (zh, en) in arr.iter().zip(en.as_array().unwrap()) {
        assert_eq!(zh["key"], en["key"]);
        assert_ne!(zh["name"], en["name"]);
    }
}

#[test]
fn bridge_horoscope_patterns_needs_scope_and_target() {
    let v = query(
        "horoscopePatterns",
        json!({"targetDate": "2026-8-19", "targetTimeIndex": 3, "scope": "decadal"}),
    )
    .unwrap();
    let arr = v.as_array().unwrap();
    assert!(!arr.is_empty(), "该盘大限层应有命中，空列表会让断言空转");
    assert!(arr.iter().all(|h| h["scope"] == "decadal"));
    let err = query(
        "horoscopePatterns",
        json!({"targetDate": "2026-8-19", "targetTimeIndex": 3, "scope": "nope"}),
    )
    .unwrap_err();
    assert!(err.contains("unknown scope"), "{err}");
}

#[test]
fn bridge_pattern_config_is_partial_and_strict() {
    let ok = query(
        "patterns",
        json!({"patternConfig": {"brightnessSource": "positional"}}),
    );
    assert!(ok.is_ok());
    let bad = query("patterns", json!({"patternConfig": {"nope": 1}})).unwrap_err();
    assert!(bad.contains("patternConfig"), "{bad}");
    let cfg: PatternConfig = serde_json::from_value(json!({"borrow": false})).unwrap();
    assert!(!cfg.borrow && cfg.flow_stars && cfg.brightness_source == BrightnessSource::Table);
}

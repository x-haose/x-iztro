//! 知识包校验：四侧共用用例 `tests/data/knowledge_check_cases.json`，默认包无问题，
//! 构造入口按 error 级问题拒绝，以及 bridge kind `checkKnowledgePack`。

use std::ffi::{CStr, CString};

use serde_json::{Value, json};
use x_iztro::ffi::{iztro_free_string, iztro_query};
use x_iztro::knowledge::{IssueLevel, KnowledgeIssue, PathSegment, check, check_json};
use x_iztro::{KnowledgePack, Language};

fn cases() -> Vec<Value> {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/knowledge_check_cases.json"
    );
    let doc: Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    doc["cases"].as_array().unwrap().clone()
}

fn base_of(case: &Value) -> Option<&'static KnowledgePack> {
    match case.get("base").and_then(Value::as_str) {
        Some("builtin") => KnowledgePack::builtin(Language::ZhCN),
        Some(other) => panic!("unknown base {other}"),
        None => None,
    }
}

/// 问题落成与用例同形的 JSON：`level`、`path`、`code`，用例给了 `params` 才带上比对。
fn shape(issue: &KnowledgeIssue, with_params: bool) -> Value {
    let path: Vec<Value> = issue
        .path
        .iter()
        .map(|s| match s {
            PathSegment::Key(k) => json!(k),
            PathSegment::Index(i) => json!(i),
        })
        .collect();
    let mut v = json!({ "level": issue.level.as_key(), "path": path, "code": issue.code.as_key() });
    if with_params {
        v["params"] = Value::Object(issue.params.clone());
    }
    v
}

#[test]
fn shared_cases_report_exactly_the_expected_issues() {
    let mut failures = Vec::new();
    for case in cases() {
        let name = case["name"].as_str().unwrap();
        let base = base_of(&case);
        let issues = match case.get("packJson").and_then(Value::as_str) {
            Some(text) => check_json(text, base),
            None => check(&case["pack"], base),
        };
        let expect = case["expect"].as_array().unwrap();
        let got: Vec<Value> = issues
            .iter()
            .zip(expect.iter().chain(std::iter::repeat(&Value::Null)))
            .map(|(i, e)| shape(i, e.get("params").is_some()))
            .collect();
        if got != *expect {
            failures.push(format!(
                "{name}\n  expect {}\n  got    {}",
                Value::Array(expect.clone()),
                Value::Array(got)
            ));
        }
    }
    assert!(failures.is_empty(), "\n{}", failures.join("\n"));
}

#[test]
fn every_issue_code_has_a_positive_case() {
    let all = cases();
    let seen: Vec<&str> = all
        .iter()
        .flat_map(|c| c["expect"].as_array().unwrap())
        .map(|e| e["code"].as_str().unwrap())
        .collect();
    for code in x_iztro::knowledge::IssueCode::KEYS {
        assert!(seen.contains(code), "{code} 没有正例");
    }
}

#[test]
fn builtin_pack_has_no_issues() {
    let v = KnowledgePack::builtin_value(Language::ZhCN).unwrap();
    assert_eq!(check(v, None), Vec::new());
}

#[test]
fn constructors_reject_errors_and_accept_warnings() {
    for case in cases() {
        let name = case["name"].as_str().unwrap();
        if case.get("base").is_some() {
            continue;
        }
        let has_error = case["expect"]
            .as_array()
            .unwrap()
            .iter()
            .any(|e| e["level"] == "error");
        let parsed = match case.get("packJson").and_then(Value::as_str) {
            Some(text) => KnowledgePack::from_json(text),
            None => KnowledgePack::from_value(&case["pack"]),
        };
        assert_eq!(parsed.is_err(), has_error, "{name}: {parsed:?}");
        if let Err(e) = parsed {
            assert!(e.starts_with("invalid knowledge pack: "), "{name}: {e}");
        }
    }
    // 报错逐条列出全部 error 级问题与路径
    let err = KnowledgePack::from_value(
        &json!({"schema": 1, "id": "", "language": "zh-CN", "version": "1", "stars": {"ziweiMjj": {}}}),
    )
    .unwrap_err();
    assert!(
        err.contains("id: required field") && err.contains("stars.ziweiMjj: unknown key"),
        "{err}"
    );
    let issue = check(&json!({"schema": 1}), None)
        .into_iter()
        .find(|i| i.level == IssueLevel::Error)
        .unwrap();
    assert_eq!(issue.to_string(), "id: required field is missing or empty");
}

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
fn bridge_kind_returns_issues_and_rejects_bad_base() {
    let got = query(json!({"kind": "checkKnowledgePack", "packJson": "[]"})).unwrap();
    assert_eq!(
        got,
        json!([{"level": "error", "path": [], "code": "not_object", "params": {},
                "message": "(pack): expected a JSON object"}])
    );
    let got = query(json!({
        "kind": "checkKnowledgePack",
        "packJson": r#"{"schema":1,"id":"t","version":"1","language":"zh-CN","patterns":{"zi_fu_tong_gong":{"quotes":[1]}}}"#,
    }))
    .unwrap();
    assert_eq!(
        got[0]["path"],
        json!(["patterns", "zi_fu_tong_gong", "quotes", 0])
    );
    let builtin = query(json!({"kind": "knowledgePack", "language": "zh-CN"})).unwrap();
    let got = query(json!({
        "kind": "checkKnowledgePack",
        "packJson": r#"{"schema":1,"id":"t","version":"1","language":"en-US"}"#,
        "basePack": builtin,
    }))
    .unwrap();
    assert_eq!(got[0]["code"], "language_differs_from_base");
    let err =
        query(json!({"kind": "checkKnowledgePack", "packJson": "{}", "basePack": {"schema": 1}}))
            .unwrap_err();
    assert!(err.starts_with("basePack: invalid knowledge pack"), "{err}");
    let err = query(json!({"kind": "checkKnowledgePack"})).unwrap_err();
    assert!(err.contains("packJson is required"), "{err}");
    // 合并时覆盖包有 error 级问题：报出是第几个包
    let err = query(json!({
        "kind": "mergeKnowledgePacks",
        "knowledgePacks": [{"schema": 1, "id": "b", "version": "1", "language": "zh-CN"},
                           {"schema": 1, "id": "o", "version": "1", "language": "zh-CN", "stars": {"x": {}}}]
    }))
    .unwrap_err();
    assert!(
        err.starts_with("knowledgePacks[1]: invalid knowledge pack"),
        "{err}"
    );
}

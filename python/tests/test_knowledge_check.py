"""知识包校验：四侧共用用例 `tests/data/knowledge_check_cases.json`（Rust `tests/knowledge_check.rs`、
Go `knowledge_check_test.go`、JS `knowledge-check.test.ts` 读同一份），默认包无问题，构造入口按 error 级问题拒绝。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

from x_iztro import IztroError, KnowledgeIssue, KnowledgeIssueCode, KnowledgePack

_SHARED = json.loads(
    (Path(__file__).resolve().parents[2] / "tests" / "data" / "knowledge_check_cases.json").read_text(
        encoding="utf-8"
    )
)
PARSE_CASES = _SHARED["parseCases"]
TOPIC_ORDER_CASES = _SHARED["topicOrderCases"]

# packJson 用例给的是 JSON 原文，适用于收文本的入口（Rust check_json、Go CheckKnowledgePack）；
# Python 的 check 收对象，非法 JSON 在 json.loads 阶段就报 JSONDecodeError
CASES = [c for c in _SHARED["cases"] if "pack" in c]


def _shape(issue: KnowledgeIssue, with_params: bool) -> dict[str, Any]:
    d: dict[str, Any] = {"level": issue.level, "path": list(issue.path), "code": issue.code.value}
    if with_params:
        d["params"] = issue.params
    return d


def _check(case: dict[str, Any]) -> list[KnowledgeIssue]:
    base = KnowledgePack.builtin() if case.get("base") == "builtin" else None
    return KnowledgePack.check(case["pack"], base=base)


@pytest.mark.parametrize("case", CASES, ids=[c["name"] for c in CASES])
def test_shared_case(case: dict[str, Any]) -> None:
    issues = _check(case)
    expect = case["expect"]
    got = [_shape(i, j < len(expect) and "params" in expect[j]) for j, i in enumerate(issues)]
    assert got == expect


def test_builtin_pack_has_no_issues() -> None:
    assert KnowledgePack.check(KnowledgePack.builtin().to_dict()) == []


@pytest.mark.parametrize(
    "case", [c for c in CASES if "base" not in c], ids=lambda c: c["name"]
)
def test_from_dict_rejects_errors_and_accepts_warnings(case: dict[str, Any]) -> None:
    has_error = any(e["level"] == "error" for e in case["expect"])
    if not has_error:
        KnowledgePack.from_dict(case["pack"])
        return
    with pytest.raises(IztroError) as e:
        KnowledgePack.from_dict(case["pack"])
    assert e.value.code == "invalid_argument"
    assert str(e.value).startswith("invalid knowledge pack: ")
    assert [i.code for i in e.value.issues] == [KnowledgeIssueCode(x["code"]) for x in case["expect"]]


def test_merged_checks_overlay_objects() -> None:
    with pytest.raises(IztroError) as e:
        KnowledgePack.builtin().merged({"schema": 1, "id": "o", "version": "1", "language": "zh-CN", "stars": {"x": {}}})
    assert e.value.issues[0].code == KnowledgeIssueCode.UNKNOWN_KEY


def test_usage_errors_raise() -> None:
    with pytest.raises(TypeError):
        KnowledgePack.check({"schema": 1, "bad": object()})
    with pytest.raises(TypeError):
        KnowledgePack.check({}, base={"schema": 1})  # type: ignore[arg-type]


@pytest.mark.parametrize("case", PARSE_CASES, ids=[c["name"] for c in PARSE_CASES])
def test_shared_parse_case(case: dict[str, Any]) -> None:
    pack = KnowledgePack.from_json(case["packJson"])
    star = pack.star(case["star"])
    assert star.intro == case["intro"]
    assert star.combinations == case["combinations"]


def test_from_dict_holds_a_normalized_copy() -> None:
    raw = {"schema": 1, "id": "t", "version": "1", "language": "zh-CN", "extra": 1,
           "stars": {"ziweiMaj": {"intro": "a"}}}
    pack = KnowledgePack.from_dict(raw)
    raw["stars"]["ziweiMaj"]["intro"] = "b"
    assert pack.star_intro("ziweiMaj") == "a"
    assert "extra" not in pack.to_dict()


def test_from_json_reports_invalid_json_as_an_issue() -> None:
    with pytest.raises(IztroError) as e:
        KnowledgePack.from_json("{bad")
    assert e.value.issues[0].code == KnowledgeIssueCode.INVALID_JSON


def test_non_finite_numbers_are_rejected_like_javascript() -> None:
    pack = {"schema": 1, "id": "t", "version": "1", "language": "zh-CN", "stars": {"ziweiMaj": {"intro": float("nan")}}}
    for call in (lambda: KnowledgePack.check(pack), lambda: KnowledgePack.from_dict(pack)):
        with pytest.raises(IztroError) as e:
            call()
        assert e.value.code == "invalid_argument"


@pytest.mark.parametrize("case", TOPIC_ORDER_CASES, ids=[c["name"] for c in TOPIC_ORDER_CASES])
def test_shared_topic_order_case(case: dict[str, Any]) -> None:
    base, *overlays = case["packs"]
    pack = KnowledgePack.from_dict(base).merged(*overlays)
    assert [t.slug for t in pack.topics()] == case["topics"]

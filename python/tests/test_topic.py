"""按主题选宫：四侧共用用例 `tests/data/topic_palaces_cases.json`（Rust `tests/topic_palaces.rs` 按各层宫名另行
推得并核对期望，Go `topic_test.go`、JS `topic.test.ts` 读同一份），以及层级比查询精度细时报错。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

from x_iztro import Astro, IztroError, KnowledgePack, TopicSelection

CASES = json.loads(
    (Path(__file__).resolve().parents[2] / "tests" / "data" / "topic_palaces_cases.json").read_text(
        encoding="utf-8"
    )
)["cases"]


def _select_and_text(case: dict[str, Any]) -> tuple[TopicSelection, str]:
    c = case["chart"]
    chart = Astro().by_solar(c["solarDate"], c["timeIndex"], c["gender"])
    knowledge = KnowledgePack.from_dict(case["knowledge"]) if "knowledge" in case else True
    target = case.get("target")
    topic = case["topic"]
    if target is None:
        return (
            chart.topic_palaces(topic, knowledge=knowledge),
            chart.topic_to_text(topic, knowledge=knowledge),
        )
    if "targetDate" in target:
        h = chart.horoscope(target["targetDate"], target["targetTimeIndex"])
    elif "targetMonth" in target:
        h = chart.horoscope_of_month(target["targetYear"], target["targetMonth"])
    else:
        h = chart.horoscope_of_year(target["targetYear"])
    return (
        h.topic_palaces(topic, case["scope"], knowledge=knowledge),
        h.topic_to_text(topic, case["scope"], knowledge=knowledge),
    )


def _text_pattern_lines(text: str) -> list[str]:
    """主题文本「格局」一节的逐条行"""
    if "\n## 格局\n" not in text:
        return []
    lines = text.split("\n## 格局\n", 1)[1].split("\n")
    out = []
    for line in lines:
        if not line.startswith("- **"):
            break
        out.append(line)
    return out


@pytest.mark.parametrize("case", CASES, ids=[c["name"] for c in CASES])
def test_shared_case(case: dict[str, Any]) -> None:
    selection, text = _select_and_text(case)
    assert selection == TopicSelection._from_dict(case["expect"])
    assert len(_text_pattern_lines(text)) == len(selection.patterns)


def test_scope_finer_than_precision_is_rejected() -> None:
    h = Astro().by_solar("2000-8-16", 2, "female").horoscope_of_year(2025)
    with pytest.raises(IztroError, match="only specifies layers down to 'yearly'"):
        h.topic_palaces("romance", "monthly")
    with pytest.raises(IztroError, match="only specifies layers down to 'yearly'"):
        h.topic_to_text("romance", "monthly")

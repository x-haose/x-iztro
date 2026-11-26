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


def _select(case: dict[str, Any]) -> TopicSelection:
    c = case["chart"]
    chart = Astro().by_solar(c["solarDate"], c["timeIndex"], c["gender"])
    knowledge = KnowledgePack.from_dict(case["knowledge"]) if "knowledge" in case else True
    target = case.get("target")
    if target is None:
        return chart.topic_palaces(case["topic"], knowledge=knowledge)
    if "targetDate" in target:
        h = chart.horoscope(target["targetDate"], target["targetTimeIndex"])
    elif "targetMonth" in target:
        h = chart.horoscope_of_month(target["targetYear"], target["targetMonth"])
    else:
        h = chart.horoscope_of_year(target["targetYear"])
    return h.topic_palaces(case["topic"], case["scope"], knowledge=knowledge)


@pytest.mark.parametrize("case", CASES, ids=[c["name"] for c in CASES])
def test_shared_case(case: dict[str, Any]) -> None:
    assert _select(case) == TopicSelection._from_dict(case["expect"])


def test_scope_finer_than_precision_is_rejected() -> None:
    h = Astro().by_solar("2000-8-16", 2, "female").horoscope_of_year(2025)
    with pytest.raises(IztroError, match="only specifies layers down to 'yearly'"):
        h.topic_palaces("romance", "monthly")
    with pytest.raises(IztroError, match="only specifies layers down to 'yearly'"):
        h.topic_to_text("romance", "monthly")

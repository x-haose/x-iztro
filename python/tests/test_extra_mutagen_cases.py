"""附加四化星的四侧共用用例 `tests/data/extra_mutagen_cases.json`（期望由 Rust `tests/extra_mutagen_cases.rs`
生成并核对，Go `extra_mutagen_cases_test.go`、JS `extra-mutagen-cases.test.ts` 读同一份）。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path
from typing import Any

import pytest

from x_iztro import Astro, ChartConfig, Mutagen

CASES = json.loads(
    (Path(__file__).resolve().parents[2] / "tests" / "data" / "extra_mutagen_cases.json").read_text(
        encoding="utf-8"
    )
)["cases"]


@pytest.mark.parametrize("case", CASES, ids=[c["name"] for c in CASES])
def test_shared_case(case: dict[str, Any]) -> None:
    c = case["chart"]
    chart = Astro().by_solar(
        c["solarDate"],
        c["timeIndex"],
        c["gender"],
        config=ChartConfig(extra_mutagens=case["config"]["extraMutagens"]),
    )
    for want in case["palaces"]:
        p = chart.palaces[want["index"]]
        assert p.mutagen_star_keys == want["mutagenStarKeys"]
        for m in Mutagen:
            assert p.mutagen_stars(m) == want["mutagenStars"][m.value], (want["index"], m)
            assert p.self_mutaged(m) == want["selfMutaged"][m.value], (want["index"], m)
        places = [q.index if q is not None else None for q in p.mutaged_places()]
        assert places == want["mutagedPlaces"]

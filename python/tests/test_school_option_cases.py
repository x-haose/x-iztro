"""流派口径开关的四侧共用用例 `tests/data/school_option_cases.json`（期望由 Rust `tests/school_option_cases.rs`
生成并核对，Go `school_option_cases_test.go`、JS `school-option-cases.test.ts` 读同一份）。

配置经 `ChartConfig` / `PatternConfig` 的字段传入，字段漏接或漏发即与期望不符。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any

import pytest

from x_iztro import Astro, Astrolabe, ChartConfig, PatternConfig

DOC = json.loads(
    (Path(__file__).resolve().parents[2] / "tests" / "data" / "school_option_cases.json").read_text(
        encoding="utf-8"
    )
)
TARGET_DATE, TARGET_TIME_INDEX = DOC["target"]


def _snake(key: str) -> str:
    return re.sub(r"(?<!^)(?=[A-Z])", "_", key).lower()


def _chart(case: dict[str, Any]) -> Astrolabe:
    c = case["chart"]
    config = ChartConfig(**{_snake(k): v for k, v in case["config"].items()})
    return Astro().by_solar(c["solarDate"], c["timeIndex"], c["gender"], config=config)


def _projection(chart: Astrolabe) -> dict[str, Any]:
    lunar = chart.raw_dates.lunar_date
    return {
        "lunarDate": [lunar.lunar_year, lunar.lunar_month, lunar.lunar_day, lunar.is_leap],
        "yearlyKeys": list(chart.raw_dates.chinese_date.yearly_keys),
        "palaces": [
            {
                "stars": [s.key for s in p.major_stars + p.minor_stars + p.adjective_stars],
                "changsheng12Key": p.changsheng12_key,
                "suiqian12Key": p.suiqian12_key,
                "jiangqian12Key": p.jiangqian12_key,
                "isOriginalPalace": p.is_original_palace,
                "isSecondaryOriginalPalace": p.is_secondary_original_palace,
            }
            for p in chart.palaces
        ],
        "monthlyIndex": chart.horoscope(TARGET_DATE, TARGET_TIME_INDEX).monthly.index,
    }


@pytest.mark.parametrize("case", DOC["cases"], ids=[c["name"] for c in DOC["cases"]])
def test_shared_case(case: dict[str, Any]) -> None:
    assert _projection(_chart(case)) == case["expect"]


@pytest.mark.parametrize(
    "case", DOC["patternCases"], ids=[c["name"] for c in DOC["patternCases"]]
)
def test_shared_pattern_case(case: dict[str, Any]) -> None:
    config = PatternConfig(**{_snake(k): v for k, v in case["patternConfig"].items()})
    hits = [[h.key, h.variant] for h in _chart(case).patterns(config)]
    assert hits == case["expect"]

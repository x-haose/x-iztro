"""夹宫与三个运限列表对照 JS iztro 的金标逐项比。

金标 `tests/golden/chart_lists.json` 由 `generate_chart_lists.mjs` 生成，Rust 的
`tests/golden_chart_lists.rs` 读同一份文件——两侧因此断言在同一组取值上。
取样盘含晚子时（列表用的时辰取自时柱地支，与出生入参差 12）与闰月出生。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from x_iztro import Astro

GOLDEN = (
    Path(__file__).resolve().parents[2] / "tests" / "golden" / "chart_lists.json"
)

astro = Astro()


def _cases() -> list[dict]:
    assert GOLDEN.exists(), (
        f"缺少金标 {GOLDEN}：先跑 node tests/golden/generate_chart_lists.mjs"
    )
    cases = json.loads(GOLDEN.read_text(encoding="utf-8"))
    assert cases, "金标为空"
    return cases


CASES = _cases()
IDS = [f"{c['birth']['d']}-t{c['birth']['t']}-{c['birth']['g']}" for c in CASES]


def _chart(birth: dict):
    return astro.by_solar(
        birth["d"], birth["t"], "male" if birth["g"] == "男" else "female"
    )


@pytest.mark.parametrize("case", CASES, ids=IDS)
def test_flanking_palaces_match_js(case):
    a = _chart(case["birth"])
    got = [
        {
            "index": i,
            "previous": {"index": f.previous.index, "name": f.previous.name},
            "next": {"index": f.next.index, "name": f.next.name},
        }
        for i, f in ((i, a.flanking_palaces(i)) for i in range(12))
    ]
    assert got == case["flanking"]


@pytest.mark.parametrize("case", CASES, ids=IDS)
def test_decadal_list_matches_js(case):
    got = [
        {
            "index": d.index,
            "name": d.name,
            "palaceName": d.palace_name,
            "ageRange": list(d.age_range),
            "yearRange": list(d.year_range),
            "heavenlyStem": d.heavenly_stem,
            "earthlyBranch": d.earthly_branch,
            "palaceNames": d.palace_names,
            "mutagen": d.mutagen,
        }
        for d in _chart(case["birth"]).decadal_list()
    ]
    assert got == case["decadals"]


@pytest.mark.parametrize("case", CASES, ids=IDS)
def test_yearly_list_matches_js(case):
    a = _chart(case["birth"])
    for ordinal, want in case["yearly"].items():
        got = [
            {
                "index": y.index,
                "age": y.age,
                "year": y.year,
                "heavenlyStem": y.heavenly_stem,
                "earthlyBranch": y.earthly_branch,
                "palaceNames": y.palace_names,
                "mutagen": y.mutagen,
            }
            for y in a.yearly_list(int(ordinal))
        ]
        assert got == want, f"大限#{ordinal}"


@pytest.mark.parametrize("case", CASES, ids=IDS)
def test_monthly_list_matches_js(case):
    # 金标键形如 "2020_true"：年份 + monthlyList 的 fixLeap 入参（与排盘的同名开关无关）
    a = _chart(case["birth"])
    for key, want in case["monthly"].items():
        year, fix_leap = key.split("_")
        got = [
            {
                "index": m.index,
                "age": m.age,
                "year": m.year,
                "month": m.month,
                "isLeapMonth": m.is_leap_month,
                "part": m.part,
                "dayRange": list(m.day_range),
                "heavenlyStem": m.heavenly_stem,
                "earthlyBranch": m.earthly_branch,
                "mutagen": m.mutagen,
            }
            for m in a.monthly_list(int(year), fix_leap == "true")
        ]
        assert got == want, key

"""日期写法与运限省略时辰对照 JS iztro 的金标。

金标 `tests/golden/inputs.json` 由 `generate_inputs.mjs` 生成，Rust 的
`tests/golden_inputs.rs` 与 Go 的 `inputs_golden_test.go` 读同一份。这里验 Python 侧
自己的那部分：`horoscope(td)` 省略时辰时由目标日期的时间换算，运限对象再发起计算时
转发原始目标参数；以及各种日期写法的排盘回显。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from x_iztro import Astro, IztroError

GOLDEN = Path(__file__).resolve().parents[2] / "tests" / "golden" / "inputs.json"
DATA = json.loads(GOLDEN.read_text(encoding="utf-8"))
astro = Astro()


@pytest.mark.parametrize("case", DATA["solar"], ids=[c["input"] for c in DATA["solar"]])
def test_solar_date_grammar(case: dict) -> None:
    if "error" in case:
        with pytest.raises(IztroError):
            astro.by_solar(case["input"], 2, "female")
        return
    chart = astro.by_solar(case["input"], 2, "female")
    assert chart.solar_date == case["solarDate"]


HOROSCOPE = [c for c in DATA["horoscope"] if "error" not in c]


@pytest.mark.parametrize("case", HOROSCOPE, ids=[f"{c['td']}-{c['tt']}" for c in HOROSCOPE])
def test_horoscope_target_time(case: dict) -> None:
    birth = case["p"]
    chart = astro.by_solar(birth["d"], birth["t"], "female")
    h = chart.horoscope(case["td"], case["tt"])
    assert h.solar_date == case["sd"]
    assert h.hourly.heavenly_stem + h.hourly.earthly_branch == case["hr"]["hs"] + case["hr"]["eb"]
    assert h.daily.heavenly_stem + h.daily.earthly_branch == case["da"]["hs"] + case["da"]["eb"]
    # 再发起计算转发原始目标参数：运限文本里流时一节的干支须与本次运限一致
    hourly = next(line for line in h.to_text().splitlines() if line.startswith("## 流时"))
    assert f"({case['hr']['hs']}{case['hr']['eb']})" in hourly


def test_horoscope_rejects_bad_targets() -> None:
    chart = astro.by_solar("2000-8-16", 2, "female")
    for case in (c for c in DATA["horoscope"] if "error" in c):
        with pytest.raises(IztroError):
            chart.horoscope(case["td"], case["tt"])

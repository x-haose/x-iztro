"""历法查询入口对照 lunar-typescript 的金标。

金标 `tests/golden/calendar_api.csv` 由 `generate_calendar_api.mjs` 生成，每个农历年一行规范化串哈希；
Rust 的 `tests/calendar_api.rs`、Go 的 `calendar_test.go` 与 JS 的 `calendar.test.ts` 按同一规则重建。
这里按年重建规范化串，验证 Python 包装的字段映射与闰月入参，并检查每一天公历换回原农历日期。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import hashlib
from pathlib import Path

import pytest

from x_iztro import (
    EarthlyBranch,
    HeavenlyStem,
    IztroError,
    RawLunarDate,
    RawSolarDate,
    lunar_months,
    lunar_to_solar,
    lunar_year_pillar,
    solar_to_lunar,
)

GOLDEN = Path(__file__).resolve().parents[2] / "tests" / "golden" / "calendar_api.csv"
EXPECTED = dict(line.split(",") for line in GOLDEN.read_text().splitlines())

# 闰月年、无闰月年、缺三月的 237 年、改历的 1582 年、尾月落在公历 10000 年的 9999 年
YEARS = [1, 237, 1582, 2023, 2024, 9999]


def _ymd(d: RawSolarDate) -> str:
    return f"{d.solar_year:04d}-{d.solar_month:02d}-{d.solar_day:02d}"


def _year_lines(y: int) -> list[str]:
    months = lunar_months(y)
    lines = [f"M {m.month},{int(m.is_leap)},{m.day_count},{_ymd(m.first_day)}" for m in months]
    stem, branch = lunar_year_pillar(y)
    lines.append(f"P {list(HeavenlyStem).index(stem)},{list(EarthlyBranch).index(branch)}")
    for m in months:
        for d in range(1, m.day_count + 1):
            solar = lunar_to_solar(y, m.month, d, is_leap=m.is_leap)
            if solar.solar_year <= 9999:
                back = solar_to_lunar(solar.solar_year, solar.solar_month, solar.solar_day)
                assert back == RawLunarDate(y, m.month, d, m.is_leap), _ymd(solar)
            lines.append(f"D {m.month},{int(m.is_leap)},{d},{_ymd(solar)}")
    return lines


@pytest.mark.parametrize("year", YEARS)
def test_matches_lunar_typescript(year: int) -> None:
    digest = hashlib.sha256("\n".join(_year_lines(year)).encode()).hexdigest()[:32]
    assert digest == EXPECTED[str(year)]


def test_year_pillar_returns_enums() -> None:
    assert lunar_year_pillar(2024) == (HeavenlyStem.JIA, EarthlyBranch.CHEN)


@pytest.mark.parametrize(
    ("call", "code"),
    [
        (lambda: solar_to_lunar(1582, 10, 10), "invalid_date"),
        (lambda: solar_to_lunar(2023, 2, 29), "invalid_date"),
        (lambda: lunar_to_solar(2024, 2, 1, is_leap=True), "invalid_date"),
        (lambda: lunar_to_solar(237, 3, 1), "invalid_date"),
        (lambda: lunar_months(10000), "invalid_argument"),
        (lambda: lunar_year_pillar(0), "invalid_argument"),
    ],
)
def test_rejects_dates_that_do_not_exist(call, code: str) -> None:
    with pytest.raises(IztroError) as e:
        call()
    assert e.value.code == code

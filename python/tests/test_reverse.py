"""反推：八字与星盘特征的往返一致性、入参校验（与 Rust 侧 tests/reverse.rs 同一批断言面）。"""

from __future__ import annotations

import pytest

from x_iztro import (
    Astro,
    IztroError,
    ReverseCriteria,
    StarPosition,
    reverse_chart,
    solar_dates_by_bazi,
)
from x_iztro.enums import EarthlyBranch, HeavenlyStem, MajorStar, MinorStar


def test_bazi_roundtrip():
    a = Astro().by_solar("2000-8-16", 2, "female")
    p = a.raw_dates.chinese_date
    got = solar_dates_by_bazi(p.yearly_keys, p.monthly_keys, p.daily_keys, p.hourly_keys)
    pairs = [(c.solar_date, c.time_index) for c in got]
    assert ("2000-8-16", 2) in pairs
    assert len(got) == 3  # 60 年周期在 1900-2100 内的三个解


def test_bazi_rejects_mismatched_polarity():
    with pytest.raises(IztroError, match="polarity"):
        solar_dates_by_bazi(
            (HeavenlyStem.JIA, EarthlyBranch.CHOU),
            (HeavenlyStem.JIA, EarthlyBranch.ZI),
            (HeavenlyStem.JIA, EarthlyBranch.ZI),
            (HeavenlyStem.JIA, EarthlyBranch.ZI),
        )


def test_reverse_chart_roundtrip():
    a = Astro().by_solar("2000-8-16", 2, "female")
    ziwei = a.star(MajorStar.ZIWEI)
    r = reverse_chart(
        ReverseCriteria(
            soul_branch=a.earthly_branch_of_soul_palace_key,
            five_elements_class=a.five_elements_class_key,
            stars=[
                StarPosition(star=MajorStar.ZIWEI, branch=ziwei.palace().earthly_branch_key),
                StarPosition(
                    star=MinorStar.LUCUN,
                    branch=a.star(MinorStar.LUCUN).palace().earthly_branch_key,
                ),
            ],
            year_range=(1999, 2001),
        )
    )
    assert any(c.solar_date == "2000-8-16" and c.time_index == 2 for c in r.candidates)
    assert not r.truncated
    # 每个候选正排后须满足条件
    for c in r.candidates[:5]:
        b = Astro().by_solar(c.solar_date, c.time_index, "female")
        assert b.earthly_branch_of_soul_palace_key == a.earthly_branch_of_soul_palace_key
        assert b.star(MajorStar.ZIWEI).palace().earthly_branch_key == ziwei.palace().earthly_branch_key


def test_reverse_chart_limit_and_errors():
    r = reverse_chart(
        ReverseCriteria(
            stars=[StarPosition(star=MinorStar.HUOXING, branch=EarthlyBranch.YIN)],
            limit=10,
        )
    )
    assert len(r.candidates) == 10 and r.truncated
    with pytest.raises(IztroError):
        reverse_chart(ReverseCriteria())
    with pytest.raises(IztroError):
        reverse_chart(
            ReverseCriteria(stars=[StarPosition(star="liulu", branch=EarthlyBranch.ZI)])
        )

"""夹宫与大限/流年/流月列表的端到端测试。

四组查询都经绑定层无状态再发起排盘，因此这里同时盯两件事：返回值的项数与
字段类型符合契约，以及排盘上下文（重排起点、修正闰月开关）真的被转发——
上下文丢失时结果仍是一份「看起来合理」的错答案，只有拿它同原盘结果对比才看得出来。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import pytest

from x_iztro import Astro, FlankingPalaces
from x_iztro._x_iztro import IztroError
from x_iztro.enums import (
    EarthlyBranch,
    HeavenlyStem,
    MonthPart,
    Mutagen,
    PalaceName,
)
from x_iztro.horoscope import DecadalListItem, MonthlyListItem, YearlyListItem

astro = Astro()

# 闰四月年，用来分辨修正闰月开关是否随查询转发
LEAP_YEAR = 2001


def chart(**kwargs):
    return astro.by_solar("2000-8-16", 2, "female", **kwargs)


# ------ 夹宫 ------


def test_flanking_palaces_are_the_neighbours():
    a = chart()
    for palace in a.palaces:
        f = a.flanking_palaces(palace.index)
        assert isinstance(f, FlankingPalaces)
        # 十二宫首尾相连：0 号宫的前一宫是 11 号宫
        assert f.previous.index == (palace.index - 1) % 12
        assert f.next.index == (palace.index + 1) % 12


def test_flanking_palaces_accept_every_addressing_form():
    a = chart()
    soul = a.palace(PalaceName.SOUL)
    by_index = a.flanking_palaces(soul.index)
    for target in (PalaceName.SOUL, soul.name, PalaceName.BODY):
        assert a.flanking_palaces(target) is not None
    assert a.flanking_palaces(PalaceName.SOUL) == by_index
    assert a.flanking_palaces("不是宫名") is None


def test_flanking_palaces_keep_the_chart_backlink():
    a = chart()
    f = a.flanking_palaces(PalaceName.SOUL)
    # 两宫须挂回本盘，否则 to_text、对宫等依赖全盘的查询在夹宫上失效
    assert f.astrolabe() is a
    assert f.next.astrolabe() is a
    assert f.previous.to_text()


def test_flanking_palace_star_judgements():
    a = chart()
    f = a.flanking_palaces(PalaceName.SOUL)
    stars = [s.key for p in (f.previous, f.next) for s in p.major_stars + p.minor_stars]
    assert stars, "命宫的夹宫在这张盘上有星，用例前提不成立"
    assert f.have(stars)
    assert f.have_one_of([stars[0], "wenquMin"])
    assert not f.not_have([stars[0]])
    assert f.not_have(["不存在的星"])


def test_flanking_palace_mutagen_judgements():
    a = chart()
    hits = 0
    for palace in a.palaces:
        f = a.flanking_palaces(palace.index)
        for m in Mutagen:
            present = f.previous.has_mutagen(m) or f.next.has_mutagen(m)
            assert f.have_mutagen(m) is present
            assert f.not_have_mutagen(m) is not present
            hits += present
    # 四化必落在四个宫上，每宫又是两宫的夹宫，命中数为正——全 False 的实现在此暴露
    assert hits == 8


def test_flanking_palaces_follow_rearranged_layout():
    a = chart()
    r = a.rearranged(HeavenlyStem.JIA, EarthlyBranch.ZI)
    f = r.flanking_palaces(PalaceName.SOUL)
    # 重排盘的宫名布局与原盘不同，夹宫必须按重排后的盘取
    assert f.previous.name_key == r.palaces[f.previous.index].name_key
    assert f.previous.name_key != a.palaces[f.previous.index].name_key


# ------ 大限列表 ------


def test_decadal_list_covers_twelve_palaces_in_order():
    a = chart()
    items = a.decadal_list()
    assert len(items) == 12
    assert all(isinstance(d, DecadalListItem) for d in items)
    assert [d.index for d in items] == sorted(
        (p.index for p in a.palaces), key=lambda i: a.palaces[i].decadal.range[0]
    )
    for i, d in enumerate(items):
        assert d.name_key == "decadal"
        assert d.palace_name_key == a.palaces[d.index].name_key
        assert d.palace_name == a.palaces[d.index].name
        assert d.age_range == a.palaces[d.index].decadal.range
        assert d.year_range[1] - d.year_range[0] == 9
        assert len(d.palace_names) == 12 and len(d.palace_name_keys) == 12
        assert len(d.mutagen) == 4 and len(d.mutagen_star_keys) == 4
        if i:
            # 起运先后排列，且相邻两限首尾相接
            assert d.age_range[0] == items[i - 1].age_range[1] + 1
            assert d.year_range[0] == items[i - 1].year_range[1] + 1


def test_decadal_list_follows_rearranged_layout():
    a = chart()
    r = a.rearranged(HeavenlyStem.JIA, EarthlyBranch.ZI)
    assert [d.index for d in r.decadal_list()] != [d.index for d in a.decadal_list()]


# ------ 流年列表 ------


def test_yearly_list_spans_the_decadal():
    a = chart()
    decadal = a.decadal_list()[0]
    items = a.yearly_list(0)
    assert len(items) == 10
    assert all(isinstance(y, YearlyListItem) for y in items)
    assert [y.age for y in items] == list(
        range(decadal.age_range[0], decadal.age_range[1] + 1)
    )
    assert [y.year for y in items] == list(
        range(decadal.year_range[0], decadal.year_range[1] + 1)
    )
    assert all(y.name_key == "yearly" for y in items)


def test_yearly_list_addressing_forms_agree():
    a = chart()
    for ordinal, decadal in enumerate(a.decadal_list()):
        assert a.yearly_list(ordinal) == a.yearly_list(decadal.palace_name_key)


def test_yearly_list_requires_addressing():
    a = chart()
    with pytest.raises(IztroError) as e:
        a.yearly_list()
    assert e.value.code == "invalid_argument"
    with pytest.raises(IztroError):
        a.yearly_list(12)
    with pytest.raises(IztroError):
        a.yearly_list("不是宫名")


# ------ 流月列表 ------


def test_monthly_list_without_leap_month_has_twelve_items():
    items = chart().monthly_list(2000)
    assert len(items) == 12
    assert all(isinstance(m, MonthlyListItem) for m in items)
    assert [m.month for m in items] == list(range(1, 13))
    assert all(m.part == MonthPart.NORMAL and not m.is_leap_month for m in items)
    assert all(m.day_range[0] == 1 and m.day_range[1] in (29, 30) for m in items)
    assert all(m.year == 2000 and m.age == 1 for m in items)
    assert all(m.name_key == "monthly" for m in items)


@pytest.mark.parametrize(
    ("fix_leap", "count", "parts"),
    [
        (True, 14, [MonthPart.FIRST, MonthPart.SECOND]),
        (False, 13, [MonthPart.NORMAL]),
    ],
)
def test_monthly_list_leap_month_splits_by_fix_leap(fix_leap, count, parts):
    items = chart().monthly_list(LEAP_YEAR, fix_leap)
    assert len(items) == count
    leap = [m for m in items if m.is_leap_month]
    assert [m.part for m in leap] == parts
    assert [m.month for m in leap] == [4] * len(parts)
    # 拆段时前半段初一至十五、后半段十六至月末，不拆时整月一段
    assert leap[0].day_range[0] == 1
    assert leap[-1].day_range[1] >= 29
    # 闰月紧跟同月号的常规月
    assert items[items.index(leap[0]) - 1].month == 4


def test_monthly_list_fix_leap_is_independent_of_the_chart():
    # 排盘的 fix_leap 决定闰月下半月按哪个月安星，与本列表拆不拆闰月是两件事
    for chart_fix_leap in (True, False):
        a = chart(fix_leap=chart_fix_leap)
        assert len(a.monthly_list(LEAP_YEAR)) == 14, "缺省即拆段，与 iztro 的默认一致"
        assert len(a.monthly_list(LEAP_YEAR, False)) == 13


def test_monthly_list_rejects_unsupported_year():
    with pytest.raises(IztroError):
        chart().monthly_list(99999)

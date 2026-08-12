"""
Python 绑定端到端金标测试。

对照 tests/golden/ 下由 JS iztro 生成的数据，验证 Python 侧从原生模块到
dataclass 的完整链路（排盘、运限、配置、方法层）。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from rs_iztro import Astro

GOLDEN = Path(__file__).resolve().parents[2] / "tests" / "golden"

astro = Astro()


def _tier1_cases(limit: int) -> list[dict]:
    data = json.loads((GOLDEN / "tier1_data.json").read_text())
    # 均匀抽样，覆盖不同年份与时辰
    step = max(1, len(data) // limit)
    return data[::step][:limit]


@pytest.mark.parametrize("case", _tier1_cases(60), ids=lambda c: f"{c['params']['solar_date']}-t{c['params']['time_index']}")
def test_astrolabe_matches_tier1(case: dict) -> None:
    p = case["params"]
    gender = "male" if p["gender"] == "男" else "female"
    a = astro.by_solar(p["solar_date"], p["time_index"], gender)

    assert a.soul == case["soul_star"]
    assert a.body == case["body_star"]
    assert a.five_elements_class == case["five_elements_class"]
    assert a.lunar_date == case["lunar_date"]
    assert a.chinese_date == case["chinese_date"]
    assert a.time == case["time"]
    assert a.sign == case["sign"]
    assert a.zodiac == case["zodiac"]

    for palace, exp in zip(a.palaces, case["palaces"]):
        assert palace.name == exp["name"]
        assert palace.heavenly_stem == exp["heavenly_stem"]
        assert palace.earthly_branch == exp["earthly_branch"]
        assert palace.is_body_palace == exp["is_body_palace"]
        assert [s.name for s in palace.major_stars] == [s["name"] for s in exp["major_stars"]]
        assert {s.name for s in palace.minor_stars} == {s["name"] for s in exp["minor_stars"]}
        assert {s.name for s in palace.adjective_stars} == {s["name"] for s in exp["adjective_stars"]}
        assert palace.changsheng12 == exp["changsheng12"]
        assert palace.boshi12 == exp["boshi12"]
        assert list(palace.decadal.range) == exp["decadal_range"]
        assert palace.ages == exp["ages"]


def _horoscope_cases(limit: int) -> list[dict]:
    data = json.loads((GOLDEN / "horoscope_data.json").read_text())
    step = max(1, len(data) // limit)
    return data[::step][:limit]


@pytest.mark.parametrize("case", _horoscope_cases(40), ids=lambda c: f"{c['p']['d']}-{c['td']}")
def test_horoscope_matches_golden(case: dict) -> None:
    p = case["p"]
    gender = "male" if p["g"] == 0 else "female"
    a = astro.by_solar(p["d"], p["t"], gender)
    h = astro.get_horoscope(a, case["td"], case["tt"])

    assert h.lunar_date == case["ld"]
    for scope_key, item in [
        ("dec", h.decadal), ("age", h.age), ("yr", h.yearly),
        ("mo", h.monthly), ("da", h.daily), ("hr", h.hourly),
    ]:
        exp = case[scope_key]
        assert item.index == exp["i"], scope_key
        assert item.name == exp["n"], scope_key
        assert item.heavenly_stem == exp["hs"], scope_key
        assert item.earthly_branch == exp["eb"], scope_key
        assert item.mutagen == exp["m"], scope_key
        if "s" in exp:
            assert item.stars is not None
            for pi, group in enumerate(exp["s"]):
                assert sorted(s.name for s in item.stars[pi]) == group, f"{scope_key} p{pi}"

    assert h.age.nominal_age == case["age"]["na"]
    assert h.yearly.yearly_dec_star is not None
    assert h.yearly.yearly_dec_star.suiqian12 == case["yr"]["sq"]
    assert h.yearly.yearly_dec_star.jiangqian12 == case["yr"]["jq"]


def test_zhongzhou_config() -> None:
    """中州派配置生效：岁前十二神含岁破、命主按年支。"""
    from rs_iztro import ChartConfig
    from rs_iztro.enums import Algorithm

    a = astro.by_solar("1990-11-5", 4, "male", config=ChartConfig(algorithm=Algorithm.ZHONGZHOU))
    assert a.config.algorithm == Algorithm.ZHONGZHOU
    assert "岁破" in {p.suiqian12 for p in a.palaces}


def test_by_lunar_round_trip() -> None:
    """农历入口与阳历入口指向同一天时结果一致。"""
    solar = astro.by_solar("2000-8-16", 2, "female")
    lunar = astro.by_lunar("2000-7-17", 2, "female")
    assert lunar.solar_date == solar.solar_date
    assert lunar.chinese_date == solar.chinese_date
    assert [p.name for p in lunar.palaces] == [p.name for p in solar.palaces]


def test_palace_methods() -> None:
    """宫位查询与飞星方法链路。"""
    a = astro.by_solar("2000-8-16", 2, "female")
    soul = a.palace("命宫")
    assert soul is not None
    assert soul.has([s.name for s in soul.major_stars])
    sp = a.surrounded_palaces(soul.index)
    assert sp.target.index == soul.index
    places = soul.mutaged_places(a.palaces)
    assert len(places) == 4 and all(p is not None for p in places)


def test_prompts() -> None:
    """两个 Prompt 接口输出结构化文本。"""
    a = astro.by_solar("2000-8-16", 2, "female")
    natal = astro.astrolabe_to_prompt(a)
    assert "=== 基本信息 ===" in natal and "十二宫" in natal
    fortune = astro.horoscope_to_prompt(a, "2024-1-1", 0)
    assert "=== 运限 ===" in fortune and "流年" in fortune


def test_enums_work_across_languages() -> None:
    """枚举基于语言无关 key：在任何输出语言的星盘上判断结果一致。"""
    from rs_iztro.enums import MajorStar, Mutagen as Mu, PalaceName

    results = {
        lang: astro.by_solar("2000-8-16", 2, "female", language=lang)
        for lang in ("zh_cn", "en_us", "ja_jp")
    }
    for lang, a in results.items():
        soul_palace = a.palace(PalaceName.SOUL)
        assert soul_palace is not None, lang
        assert soul_palace.name_key == PalaceName.SOUL, lang
        assert soul_palace.has([MajorStar.ZIWEI]), lang
        assert a.soul_key == MajorStar.POJUN, lang
        found = a.star_in_palace(MajorStar.WUQU)
        assert found is not None and found[0].with_mutagen(Mu.QUAN), lang

    # 各语言的判断结果互相一致
    zh, en = results["zh_cn"], results["en_us"]
    for m in Mu:
        assert [p.has_mutagen(m) for p in zh.palaces] == [p.has_mutagen(m) for p in en.palaces]


def test_flies_to_across_languages() -> None:
    """四化飞星基于天干 key 表，非中文星盘同样正确。"""
    from rs_iztro.enums import Mutagen as Mu

    a = astro.by_solar("2000-8-16", 2, "female", language="en_us")
    soul = a.palace(0)
    places = soul.mutaged_places(a.palaces)
    assert len(places) == 4 and all(p is not None for p in places)
    assert any(soul.flies_to(t, Mu.LU) for t in a.palaces)


def test_chart_config_typed() -> None:
    """ChartConfig 类型化配置入参。"""
    from rs_iztro import ChartConfig
    from rs_iztro.enums import Algorithm, Suiqian12, YearDivide

    a = astro.by_solar(
        "1990-11-5", 4, "male",
        config=ChartConfig(algorithm=Algorithm.ZHONGZHOU, year_divide=YearDivide.EXACT),
    )
    assert a.config.algorithm == Algorithm.ZHONGZHOU
    assert a.config.year_divide == YearDivide.EXACT
    assert Suiqian12.SUIPO in {p.suiqian12_key for p in a.palaces}

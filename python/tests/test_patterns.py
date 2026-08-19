"""格局的端到端测试。

金标是 `tests/golden/pattern_snapshots/*.json`（由 Rust 的 `tests/pattern_snapshot.rs` 写出）：
每份含一张盘的排盘入参与本命、大限、流年三层的命中 DTO。本测试按文件里的入参重新排盘，
用同样的口径取命中，与文件逐键逐值比对——Go 侧的 `pattern_golden_test.go` 读同一批文件，
三侧因此断言在同一组取值上。

运行前置：PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python
"""

from __future__ import annotations

import json
from pathlib import Path

import pytest

from x_iztro import Astro, Astrolabe, ChartConfig
from x_iztro._x_iztro import IztroError
from x_iztro.enums import BrightnessSource, PalaceName, PatternKey, Scope
from x_iztro.pattern import PatternConfig

SNAPSHOTS = (
    Path(__file__).resolve().parents[2] / "tests" / "golden" / "pattern_snapshots"
)

astro = Astro()


def _snapshot_files() -> list[Path]:
    files = sorted(SNAPSHOTS.glob("*.json"))
    assert files, f"快照目录为空：{SNAPSHOTS}（先跑 cargo test --test pattern_snapshot）"
    return files


def _chart_config(payload: dict) -> ChartConfig:
    """快照里的 camelCase 排盘配置 → ChartConfig。"""
    return ChartConfig(
        year_divide=payload["yearDivide"],
        horoscope_divide=payload["horoscopeDivide"],
        age_divide=payload["ageDivide"],
        day_divide=payload["dayDivide"],
        algorithm=payload["algorithm"],
        astro_type=payload["astroType"],
    )


def _pattern_config(payload: dict) -> PatternConfig:
    """快照里的 camelCase 判定口径 → PatternConfig。"""
    return PatternConfig(
        brightness_source=BrightnessSource(payload["brightnessSource"]),
        borrow=payload["borrow"],
        flow_stars=payload["flowStars"],
    )


def _chart(params: dict) -> Astrolabe:
    return astro.by_solar(
        params["solarDate"],
        params["timeIndex"],
        params["gender"],
        fix_leap=params["fixLeap"],
        language=params["language"],
        config=_chart_config(params["config"]),
    )


@pytest.mark.parametrize("path", _snapshot_files(), ids=lambda p: p.stem)
def test_patterns_match_snapshot(path: Path):
    """三层命中与快照逐键逐值一致。"""
    snapshot = json.loads(path.read_text())
    params = snapshot["params"]
    chart = _chart(params)
    config = _pattern_config(params["patternConfig"])

    horoscope = chart.horoscope(params["targetDate"], params["targetTimeIndex"])
    actual = {
        "origin": chart.patterns(config),
        "decadal": horoscope.patterns(Scope.DECADAL, config),
        "yearly": horoscope.patterns(Scope.YEARLY, config),
    }

    for layer, hits in actual.items():
        expected = snapshot[layer]
        assert len(hits) == len(expected), f"{path.stem} {layer}：命中数不一致"
        for hit, want in zip(hits, expected):
            assert hit.to_dict() == want, f"{path.stem} {layer}：{hit.key} 的 DTO 不一致"
            # dataclass 字段与 DTO 同源，抽查几项确保包装层没有落字段
            assert hit.key == want["key"]
            assert hit.scope == want["scope"]
            assert hit.palace_index == want["palaceIndex"]
            assert hit.palace_name_key == want["palaceNameKey"]
            assert hit.variant == want.get("variant")
            assert [s.key for s in hit.stars] == [s["key"] for s in want["stars"]]


@pytest.fixture(scope="module")
def chart():
    return astro.by_solar("2000-8-16", 2, "female")


def test_pattern_key_and_palace_name_are_comparable(chart):
    """命中可直接与枚举比较，判断方法与 key 一致。"""
    hit = next(h for h in chart.patterns() if h.key == PatternKey.FU_XIANG_CHAO_YUAN)

    assert hit.is_(PatternKey.FU_XIANG_CHAO_YUAN)
    assert hit.is_("fu_xiang_chao_yuan")
    assert not hit.is_(PatternKey.ZI_FU_TONG_GONG)
    assert hit.in_palace(PalaceName.SOUL)
    assert hit.in_palace(hit.palace_name)
    assert hit.scope == Scope.ORIGIN
    assert all(0 <= s.palace_index < 12 for s in hit.stars)


def test_horoscope_scope_selects_layer_and_rejects_unknown(chart):
    """运限层由 scope 选定；未知 scope 报错而非静默取默认。"""
    horoscope = chart.horoscope("2026-8-19", 3)

    assert all(h.scope == Scope.DECADAL for h in horoscope.patterns(Scope.DECADAL))
    assert all(h.scope == Scope.YEARLY for h in horoscope.patterns("yearly"))
    assert horoscope.patterns(Scope.ORIGIN) == chart.patterns()

    with pytest.raises(IztroError, match="scope"):
        horoscope.patterns("nope")


def test_pattern_config_changes_judgement():
    """口径经绑定层传到内核：位置法亮度会改判日月类格局。"""
    # 该盘命宫三方四正的太阴在酉：亮度表判「不」不成格，位置法判明成格
    chart = astro.by_solar("1985-1-3", 7, "female")
    positional = PatternConfig(brightness_source=BrightnessSource.POSITIONAL)

    table_keys = {h.key for h in chart.patterns()}
    positional_keys = {h.key for h in chart.patterns(positional)}

    assert PatternKey.RI_YUE_BING_MING not in table_keys
    assert PatternKey.RI_YUE_BING_MING in positional_keys
    assert PatternKey.DAN_CHI_GUI_CHI in positional_keys
    assert PatternConfig().to_dict() == {
        "brightnessSource": "table",
        "borrow": True,
        "flowStars": True,
    }


def test_chart_config_reaches_pattern_query():
    """排盘配置随命中查询一起送回内核：中州派与默认派的命中不同。"""
    # 该盘的空亡星安法两派不同，默认派命宫空亡逢破军成「生不逢时」，中州派不成
    params = {"solar_date": "1985-6-11", "time_index": 11, "gender": "male"}
    default = astro.by_solar(params["solar_date"], params["time_index"], params["gender"])
    zhongzhou = astro.by_solar(
        params["solar_date"],
        params["time_index"],
        params["gender"],
        config=ChartConfig(algorithm="zhongzhou"),
    )

    assert PatternKey.SHENG_BU_FENG_SHI in {h.key for h in default.patterns()}
    assert PatternKey.SHENG_BU_FENG_SHI not in {h.key for h in zhongzhou.patterns()}

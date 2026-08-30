"""知识包：默认包可取、查询类型化、合并走内核且语义与 Rust 一致。"""

from __future__ import annotations

from pathlib import Path

import pytest

from x_iztro import Astro, IztroError, KnowledgePack, PatternKey
from x_iztro.enums import BrightnessSource, MajorStar, Mutagen, PalaceName, Scope
from x_iztro.pattern import PatternConfig

SNAPSHOTS = Path(__file__).resolve().parents[2] / "tests" / "golden" / "text_snapshots"


@pytest.fixture(scope="module")
def pack() -> KnowledgePack:
    return KnowledgePack.builtin()


@pytest.fixture(scope="module")
def chart():
    """与 `tests/text_snapshot.rs` 同一张盘：2000-8-16 寅时女命，fix_leap=True。"""
    return Astro().by_solar("2000-8-16", 2, "female", fix_leap=True)


@pytest.fixture(scope="module")
def horoscope(chart):
    """同快照的运限目标：2025-1-1 时辰 0。"""
    return chart.horoscope("2025-1-1", 0)


def _snapshot(name: str) -> str:
    return (SNAPSHOTS / f"{name}.txt").read_text()


def _texts(chart, horoscope, **kw) -> dict[str, str]:
    """五类语义化文本，键与快照文件名前缀一致。"""
    soul = chart.palace(PalaceName.SOUL)
    return {
        "astrolabe": chart.to_text(**kw),
        "horoscope": horoscope.to_text(**kw),
        "patterns": chart.patterns_to_text(**kw),
        "palace": soul.to_text(**kw),
        "surrounded": soul.surrounded_palaces().to_text(**kw),
    }


def test_builtin_metadata_and_entries(pack):
    assert pack.schema == 1
    assert pack.id == "iztro-docs"
    assert pack.language == "zh-CN"
    assert pack.source.license == "MIT"
    ziwei = pack.star(MajorStar.ZIWEI)
    assert ziwei is not None and ziwei.name == "紫微" and ziwei.category == "major"
    assert ziwei.attributes.yin_yang == "yin" and ziwei.attributes.five_elements == "earth"
    assert "帝王星" in (ziwei.attributes.aliases or [])
    assert MajorStar.TIANFU in ziwei.combinations
    assert pack.pattern(PatternKey.ZI_FU_TONG_GONG).name == "紫府同宫"
    assert pack.pattern_intro(PatternKey.ZI_FU_TONG_GONG)
    assert pack.palace(PalaceName.SOUL).name == "命宫"
    assert pack.mutagen(Mutagen.JI).intro
    assert pack.concept("tong-gong") is not None
    # 概念条目数钉死：绑定层若丢字段或默认包被误删条目，这里立即报
    assert len(pack.to_dict()["concepts"]) == 49
    assert len(pack.patterns()) == 64
    assert pack.star("nope") is None
    with pytest.raises(IztroError):
        KnowledgePack.builtin("en-US")


def test_pattern_hits_can_be_explained_with_pack(pack):
    chart = Astro().by_solar("2000-8-16", 2, "female")
    for hit in chart.patterns():
        assert pack.pattern(hit.key) is not None


def test_merge_overrides_only_given_fields(pack):
    overlay = KnowledgePack.from_dict(
        {
            "schema": 1,
            "id": "mine",
            "version": "1",
            "language": "zh-CN",
            "extends": "iztro-docs",
            "stars": {"ziweiMaj": {"intro": "我的紫微", "attributes": {"aliases": ["我的别号"]}}},
            "patterns": {"zi_fu_tong_gong": {"intro": "我的紫府同宫"}},
        }
    )
    merged = pack.merged(overlay)
    assert merged.id == "mine"
    ziwei = merged.star(MajorStar.ZIWEI)
    assert ziwei.intro == "我的紫微"
    assert ziwei.name == "紫微"
    assert ziwei.attributes.aliases == ["我的别号"]
    assert ziwei.attributes.chemistry == pack.star(MajorStar.ZIWEI).attributes.chemistry
    assert merged.pattern_intro(PatternKey.ZI_FU_TONG_GONG) == "我的紫府同宫"
    assert merged.pattern(PatternKey.ZI_FU_TONG_GONG).quotes == pack.pattern(PatternKey.ZI_FU_TONG_GONG).quotes
    assert pack.star_intro(MajorStar.ZIWEI) != "我的紫微"
    # 也可以直接传 dict，且 to_dict/from_json 往返一致
    again = pack.merged(overlay.to_dict())
    assert again.to_dict() == merged.to_dict()
    assert KnowledgePack.from_json(merged.to_json()).to_dict() == merged.to_dict()
    with pytest.raises(IztroError):
        pack.merged({"schema": 99})


# ============================================================
# 知识包融入 to_text
# ============================================================


def test_text_with_builtin_knowledge_matches_rust_snapshots(chart, horoscope):
    """`knowledge=True` 的五类输出与 Rust 快照逐字节相同；快照由 Rust 侧生成。"""
    for kind, text in _texts(chart, horoscope, knowledge=True).items():
        assert text == _snapshot(f"{kind}_knowledge_zh-CN"), kind


def test_text_with_pack_instance_equals_builtin_flag(chart, horoscope, pack):
    """传 `KnowledgePack.builtin()` 实例与传 True 同解（实例走 "builtin" 短路）。"""
    assert _texts(chart, horoscope, knowledge=pack) == _texts(chart, horoscope, knowledge=True)


def test_text_without_knowledge_is_unchanged(chart, horoscope):
    """不带释义的形态不受影响：默认、None、False 都与事实快照一致，`str()` 亦然。"""
    for kw in ({}, {"knowledge": None}, {"knowledge": False}):
        for kind, text in _texts(chart, horoscope, **kw).items():
            assert text == _snapshot(f"{kind}_zh-CN"), (kind, kw)
    assert str(chart) == _snapshot("astrolabe_zh-CN")
    assert str(horoscope) == _snapshot("horoscope_zh-CN")


def test_for_astrolabe_takes_only_what_is_on_the_chart(chart, pack):
    """子包：星条目 ⊆ 盘上的星，盘上有条目的星全在，四化四条，不含宫位与术语。"""
    sub = pack.for_astrolabe(chart)
    assert isinstance(sub, KnowledgePack)
    assert sub.id == pack.id and sub.language == pack.language

    on_chart: set[str] = set()
    for p in chart.palaces:
        on_chart |= {s.key for s in p.major_stars + p.minor_stars + p.adjective_stars}
        on_chart |= {p.changsheng12_key, p.boshi12_key, p.jiangqian12_key, p.suiqian12_key}
    sub_keys = {e.key for e in sub.stars()}
    assert sub_keys <= on_chart
    assert {k for k in on_chart if pack.star(k) is not None} == sub_keys
    assert {hit.key for hit in chart.patterns()} == {e.key for e in sub.patterns()}

    raw = sub.to_dict()
    assert len(raw["mutagens"]) == 4
    assert not raw.get("palaces") and not raw.get("concepts")
    # 子包仍是标准包：能再合并、序列化、往返
    assert KnowledgePack.from_json(sub.to_json()).to_dict() == raw
    assert sub.merged({"schema": 1, "id": "x", "version": "1", "language": "zh-CN"}).id == "x"


def test_for_horoscope_extends_the_natal_subpack(chart, horoscope, pack):
    """运限子包在本命子包之上再加流耀与各层格局。"""
    natal = pack.for_astrolabe(chart)
    flow = pack.for_horoscope(horoscope)
    assert {e.key for e in natal.stars()} <= {e.key for e in flow.stars()}
    assert {e.key for e in natal.patterns()} <= {e.key for e in flow.patterns()}
    assert any(e.category == "flow" for e in flow.stars())
    assert len(flow.to_dict()["mutagens"]) == 4


def test_for_chart_honours_pattern_config(pack):
    """子包格局按传入口径判定：位置法亮度改判日月类格局，本命与运限子包都随之变化；
    省略与显式 None 同为默认口径。"""
    chart = Astro().by_solar("1985-1-3", 7, "female")
    horoscope = chart.horoscope("2025-1-1", 0)
    positional = PatternConfig(brightness_source=BrightnessSource.POSITIONAL)

    natal_keys = {e.key for e in pack.for_astrolabe(chart, positional).patterns()}
    assert natal_keys == {h.key for h in chart.patterns(positional)}
    assert PatternKey.RI_YUE_BING_MING in natal_keys
    assert PatternKey.RI_YUE_BING_MING not in {e.key for e in pack.for_astrolabe(chart).patterns()}
    assert pack.for_astrolabe(chart, None).to_dict() == pack.for_astrolabe(chart).to_dict()

    flow_keys = {e.key for e in pack.for_horoscope(horoscope, positional).patterns()}
    expected = {h.key for h in chart.patterns(positional)}
    for scope in (Scope.DECADAL, Scope.YEARLY, Scope.MONTHLY, Scope.DAILY, Scope.HOURLY):
        expected |= {h.key for h in horoscope.patterns(scope, positional)}
    assert flow_keys == expected
    assert pack.for_horoscope(horoscope, None).to_dict() == pack.for_horoscope(horoscope).to_dict()


def test_builtin_knowledge_for_english_chart_is_rejected(pack):
    """英文盘没有内嵌包：`knowledge=True` 报 invalid_argument，不静默回退；
    zh-CN 的内嵌包实例语言与盘不同，则按包对象发送而非 "builtin"，照常可用。"""
    en = Astro().by_solar("2000-8-16", 2, "female", language="en-US")
    with pytest.raises(ValueError) as info:
        en.to_text(knowledge=True)
    assert info.value.code == "invalid_argument"
    assert "=== " in en.to_text(knowledge=pack)[len(en.to_text()):]
    assert pack.for_astrolabe(en).stars()


def test_custom_pack_text_uses_overridden_entries(chart, pack):
    """传合并后的自定义包：释义节用覆盖后的文本，且 `to_text` 不改盘面事实节。"""
    mine = pack.merged(
        {
            "schema": 1,
            "id": "mine",
            "version": "1",
            "language": "zh-CN",
            "extends": "iztro-docs",
            "stars": {"ziweiMaj": {"intro": "我的紫微释义"}},
        }
    )
    text = chart.to_text(knowledge=mine)
    assert "我的紫微释义" in text
    assert text.startswith(_snapshot("astrolabe_zh-CN"))
    assert "我的紫微释义" not in chart.to_text(knowledge=True)
    with pytest.raises(TypeError):
        chart.to_text(knowledge="builtin")

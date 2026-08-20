"""知识包：默认包可取、查询类型化、合并走内核且语义与 Rust 一致。"""

from __future__ import annotations

import pytest

from x_iztro import Astro, IztroError, KnowledgePack, PatternKey
from x_iztro.enums import MajorStar, Mutagen, PalaceName


@pytest.fixture(scope="module")
def pack() -> KnowledgePack:
    return KnowledgePack.builtin()


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

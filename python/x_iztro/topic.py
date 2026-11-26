"""
按主题选宫的结构化结果（`Astrolabe.topic_palaces`、`Horoscope.topic_palaces`）。

本命层按本命宫名定位；运限层按该层重排的宫名定位，三方四正以该层的本宫起算。`bodyPalace` /
`originalPalace` 是本命盘上的标记，在任何一层都落在本命那一宫。全部字段是语言无关标识与宫位索引
（0-11，寅宫为 0），与主题文本（`topic_to_text`）同一套选宫。
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class TopicGroup:
    """主题定义里的一组宫落到盘上的位置"""

    kind: str
    """`"surround"`（连同该层的三方四正）或 `"only"`（只取本宫）"""
    key: str
    """主题定义里的宫位标识：宫名标识、`bodyPalace` 或 `originalPalace`"""
    target: int
    """本宫索引"""
    opposite: int | None
    """对宫索引；`only` 组为 None"""
    wealth: int | None
    """财帛位索引；`only` 组为 None"""
    career: int | None
    """官禄位索引；`only` 组为 None"""

    @classmethod
    def _from_dict(cls, d: dict) -> TopicGroup:
        return cls(
            kind=d["kind"],
            key=d["key"],
            target=d["target"],
            opposite=d["opposite"],
            wealth=d["wealth"],
            career=d["career"],
        )


@dataclass(frozen=True, slots=True)
class Flight:
    """一宫宫干飞出的一化"""

    mutagen_key: str
    """四化标识（`Mutagen` 枚举值域）"""
    star_key: str
    """化出的星耀标识"""
    palace_index: int | None
    """星所在的宫位索引；星不在盘上时为 None"""
    self_mutaged: bool
    """是否自化（落回本宫）"""

    @classmethod
    def _from_dict(cls, d: dict) -> Flight:
        return cls(
            mutagen_key=d["mutagenKey"],
            star_key=d["starKey"],
            palace_index=d["palaceIndex"],
            self_mutaged=d["selfMutaged"],
        )


@dataclass(frozen=True, slots=True)
class TopicPalace:
    """选中的一宫"""

    index: int
    """宫位索引"""
    role: str
    """角色：`"target"` / `"opposite"` / `"wealth"` / `"career"` / `"related"`，取首次出现的那一组"""
    group: int
    """首次出现的组在 `TopicSelection.groups` 里的下标"""
    name_key: str
    """所选层的宫名标识（`PalaceName` 枚举值域）"""
    natal_name_key: str
    """本命宫名标识"""
    stem_key: str
    """宫干飞化所用天干的标识（`HeavenlyStem` 枚举值域），取法见 `TopicSelection.palace_stem_source`"""
    flying: list[Flight]
    """宫干飞化，按禄、权、科、忌，每类先四槽之星、后附加四化星（默认四化表下即四条）"""

    @classmethod
    def _from_dict(cls, d: dict) -> TopicPalace:
        return cls(
            index=d["index"],
            role=d["role"],
            group=d["group"],
            name_key=d["nameKey"],
            natal_name_key=d["natalNameKey"],
            stem_key=d["stemKey"],
            flying=[Flight._from_dict(f) for f in d["flying"]],
        )


@dataclass(frozen=True, slots=True)
class MutagenLanding:
    """生年四化落在选中宫的一条"""

    mutagen_key: str
    """四化标识"""
    star_key: str
    """化出的星耀标识"""
    palace_index: int
    """星所在的宫位索引"""

    @classmethod
    def _from_dict(cls, d: dict) -> MutagenLanding:
        return cls(mutagen_key=d["mutagenKey"], star_key=d["starKey"], palace_index=d["palaceIndex"])


@dataclass(frozen=True, slots=True)
class ScopeMutagenLanding:
    """运限某层的四化落在选中宫的一条"""

    scope: str
    """层级标识（`decadal` / `yearly` / `monthly` / `daily` / `hourly`）"""
    name_key: str
    """层级名标识：大限层未起运时为 `childhood`"""
    stem_key: str
    """化出这条四化的天干标识，取法见 `TopicSelection.scope_mutagen_stem_source`"""
    mutagen_key: str
    """四化标识"""
    star_key: str
    """化出的星耀标识"""
    palace_index: int
    """星所在的宫位索引"""

    @classmethod
    def _from_dict(cls, d: dict) -> ScopeMutagenLanding:
        return cls(
            scope=d["scope"],
            name_key=d["nameKey"],
            stem_key=d["stemKey"],
            mutagen_key=d["mutagenKey"],
            star_key=d["starKey"],
            palace_index=d["palaceIndex"],
        )


@dataclass(frozen=True, slots=True)
class TopicSelection:
    """一个主题在某一层选中的宫与相关四化"""

    topic: str
    """主题 slug"""
    scope: str
    """所选层的层级标识；本命为 `origin`"""
    groups: list[TopicGroup]
    """主题定义的各组，先 `surround` 后 `palaces`，各按定义顺序；来因宫不在盘上时不成组"""
    palaces: list[TopicPalace]
    """选中的宫，每宫一次，即主题文本的成段顺序"""
    palace_stem_source: str
    """宫干飞化所用天干的取法：`"natalPalace"`（本命各宫宫干，运限各层沿用）"""
    scope_mutagen_stem_source: str
    """运限四化所用天干的取法：`"scope"`（该层自身的干：大限宫干、流年年干、流月月干……）"""
    birth_mutagens: list[MutagenLanding]
    """生年四化落在选中宫的，按禄、权、科、忌"""
    scope_mutagens: list[ScopeMutagenLanding]
    """大限至所选层各层四化落在选中宫的，按层级再按禄、权、科、忌；本命层为空"""

    @classmethod
    def _from_dict(cls, d: dict) -> TopicSelection:
        return cls(
            topic=d["topic"],
            scope=d["scope"],
            groups=[TopicGroup._from_dict(g) for g in d["groups"]],
            palaces=[TopicPalace._from_dict(p) for p in d["palaces"]],
            palace_stem_source=d["palaceStemSource"],
            scope_mutagen_stem_source=d["scopeMutagenStemSource"],
            birth_mutagens=[MutagenLanding._from_dict(m) for m in d["birthMutagens"]],
            scope_mutagens=[ScopeMutagenLanding._from_dict(m) for m in d["scopeMutagens"]],
        )

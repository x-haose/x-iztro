"""
宫位与大限。

宫位是星盘的基本单元：星耀落在宫位上，四化飞星在宫位之间发生，
大限与小限也按宫位排布。判断方法一律基于语言无关标识。
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING

from x_iztro.enums import _MUTAGEN_INDEX, Mutagen, _as_mutagen_list, _or_all_mutagens
from x_iztro.star_object import Star, _star_identifiers

if TYPE_CHECKING:
    from x_iztro.astrolabe import Astrolabe
    from x_iztro.knowledge import KnowledgePack
    from x_iztro.surpalaces import SurroundedPalaces


@dataclass(frozen=True, slots=True)
class Decadal:
    """大限"""

    range: tuple[int, int]
    """大限起止年龄 (起始, 截止)"""

    heavenly_stem: str
    """大限天干（翻译文本）"""

    heavenly_stem_key: str
    """大限天干标识（`HeavenlyStem` 枚举值域）"""

    earthly_branch: str
    """大限地支（翻译文本）"""

    earthly_branch_key: str
    """大限地支标识（`EarthlyBranch` 枚举值域）"""

    @classmethod
    def _from_dict(cls, d: dict) -> Decadal:
        r = d["range"]
        return cls(
            range=(r[0], r[1]),
            heavenly_stem=d["heavenlyStem"],
            heavenly_stem_key=d["heavenlyStemKey"],
            earthly_branch=d["earthlyBranch"],
            earthly_branch_key=d["earthlyBranchKey"],
        )


# ============================================================
# Palace
# ============================================================

@dataclass(frozen=True, slots=True)
class Palace:
    """宫位"""

    index: int
    """宫位索引 (0-11)"""

    name: str
    """宫位名称（按排盘语言翻译）"""

    name_key: str
    """语言无关宫位标识（`PalaceName` 枚举值域）"""

    is_body_palace: bool
    """是否身宫"""

    is_original_palace: bool
    """是否来因宫"""

    heavenly_stem: str
    """宫位天干（翻译文本）"""

    heavenly_stem_key: str
    """宫位天干标识（`HeavenlyStem` 枚举值域）"""

    earthly_branch: str
    """宫位地支（翻译文本）"""

    earthly_branch_key: str
    """宫位地支标识（`EarthlyBranch` 枚举值域）"""

    major_stars: list[Star]
    """主星列表"""

    minor_stars: list[Star]
    """辅星列表"""

    adjective_stars: list[Star]
    """杂耀列表"""

    changsheng12: str
    """长生十二神（翻译文本）"""

    changsheng12_key: str
    """长生十二神标识（`Changsheng12` 枚举值域）"""

    boshi12: str
    """博士十二神（翻译文本）"""

    boshi12_key: str
    """博士十二神标识（`Boshi12` 枚举值域）"""

    jiangqian12: str
    """将前十二神（翻译文本）"""

    jiangqian12_key: str
    """将前十二神标识（`Jiangqian12` 枚举值域）"""

    suiqian12: str
    """岁前十二神（翻译文本）"""

    suiqian12_key: str
    """岁前十二神标识（`Suiqian12` 枚举值域）"""

    decadal: Decadal
    """大限信息"""

    ages: list[int]
    """小限经过年龄"""

    mutagen_star_keys: list[str]
    """本宫天干化出的四颗星标识，顺序为禄、权、科、忌。

    由排盘时生效的四化表算得，因此自定义四化表（`ChartConfig.mutagens`）
    会反映在这里，飞星判断随之改变。
    """

    _astrolabe: Astrolabe | None = field(default=None, compare=False, repr=False)
    """宫位所属星盘，由星盘构造时回填"""

    # ------ 星耀判断 ------

    def has(self, stars: list[str]) -> bool:
        """判断宫位是否包含指定的 **所有** 星耀（接受星耀枚举或当前语言的星名）"""
        identifiers = self._all_star_identifiers()
        return all(s in identifiers for s in stars)

    def not_have(self, stars: list[str]) -> bool:
        """判断宫位是否 **不** 包含指定的所有星耀"""
        identifiers = self._all_star_identifiers()
        return all(s not in identifiers for s in stars)

    def has_one_of(self, stars: list[str]) -> bool:
        """判断宫位是否包含指定星耀中的 **至少一颗**"""
        identifiers = self._all_star_identifiers()
        return any(s in identifiers for s in stars)

    def has_mutagen(self, mutagen: Mutagen) -> bool:
        """判断宫位是否有指定四化（只检查主星和辅星）"""
        return any(
            s.mutagen_key == mutagen for s in self.major_stars + self.minor_stars
        )

    def not_have_mutagen(self, mutagen: Mutagen) -> bool:
        """判断宫位是否没有指定四化"""
        return not self.has_mutagen(mutagen)

    def is_empty(self, exclude_stars: list[str] | None = None) -> bool:
        """
        判断宫位是否为空宫（无十四主星）。

        Args:
            exclude_stars: 视为「不空」的星耀；宫内出现其中任一颗即不作空宫论
        """
        if self.major_stars:
            return False
        if exclude_stars and self.has_one_of(exclude_stars):
            return False
        return True

    def astrolabe(self) -> Astrolabe | None:
        """宫位所属星盘；脱离星盘单独构造的宫位返回 None"""
        return self._astrolabe

    def to_text(self, *, knowledge: bool | KnowledgePack | None = None) -> str:
        """
        本宫的语义化文本：面向语言模型与人的单宫完整描述。

        从所属星盘的排盘上下文（含重排起点）无状态再发起计算，
        文本按排盘语言输出。

        Args:
            knowledge: 释义材料（True 取排盘语言的内嵌包，或给 KnowledgePack）；
                给出时追加本宫星耀的释义节

        Raises:
            ValueError: 宫位脱离星盘单独构造，无排盘上下文可转发
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        if self._astrolabe is None:
            raise ValueError("to_text 需要所属星盘：请从 Astrolabe 的宫位查询获取宫位")
        return self._astrolabe._context_query(
            "palaceToText", palace_index=self.index, knowledge=knowledge
        )

    def opposite_palace(self) -> Palace | None:
        """
        本宫的对宫，即索引 +6 的那一宫；脱离星盘单独构造的宫位返回 None。

        对宫与本宫永远相对而看：命宫对迁移、财帛对福德，依此类推。
        """
        if self._astrolabe is None:
            return None
        return self._astrolabe.palaces[(self.index + 6) % 12]

    def surrounded_palaces(self) -> SurroundedPalaces | None:
        """本宫的三方四正；脱离星盘单独构造的宫位返回 None"""
        if self._astrolabe is None:
            return None
        return self._astrolabe.surrounded_palaces(self.index)

    # ------ 四化飞星 ------

    def mutagen_stars(self, mutagens: Mutagen | list[Mutagen]) -> list[str]:
        """
        本宫天干在指定四化位上对应的星耀标识。

        顺序与传入顺序一致；查不到对应星耀的四化位会被跳过。
        """
        out: list[str] = []
        for m in _as_mutagen_list(mutagens):
            idx = _MUTAGEN_INDEX.get(m)
            if idx is None or idx >= len(self.mutagen_star_keys):
                continue
            out.append(self.mutagen_star_keys[idx])
        return out

    def flies_to(
        self,
        target: Palace | int | str,
        mutagens: Mutagen | list[Mutagen],
    ) -> bool:
        """
        判断本宫天干的指定四化星是否 **全部** 落在目标宫位。

        Args:
            target: 目标宫位对象、宫位索引，或宫位标识/宫名
            mutagens: 四化类型，单个或列表

        四化列表为空时返回 False。
        """
        to = self._resolve_palace(target)
        if to is None:
            return False
        stars = self.mutagen_stars(mutagens)
        if not stars:
            return False
        return to.has(stars)

    def flies_one_of_to(
        self,
        target: Palace | int | str,
        mutagens: Mutagen | list[Mutagen],
    ) -> bool:
        """
        判断本宫天干的指定四化星是否有 **任意一颗** 落在目标宫位。

        四化列表为空时返回 True。
        """
        to = self._resolve_palace(target)
        if to is None:
            return False
        stars = self.mutagen_stars(mutagens)
        if not stars:
            return True
        return to.has_one_of(stars)

    def not_fly_to(
        self,
        target: Palace | int | str,
        mutagens: Mutagen | list[Mutagen],
    ) -> bool:
        """
        判断本宫天干的指定四化星是否 **一颗都不** 落在目标宫位。

        四化列表为空时返回 True。
        """
        to = self._resolve_palace(target)
        if to is None:
            return False
        stars = self.mutagen_stars(mutagens)
        if not stars:
            return True
        return to.not_have(stars)

    def self_mutaged(self, mutagens: Mutagen | list[Mutagen]) -> bool:
        """判断本宫天干的指定四化星是否 **全部** 落在本宫（自化）"""
        return self.has(self.mutagen_stars(mutagens))

    def self_mutaged_one_of(
        self, mutagens: Mutagen | list[Mutagen] | None = None
    ) -> bool:
        """
        判断本宫是否有指定四化中的任意一种自化。

        不传或传空表示检查全部四化。
        """
        return self.has_one_of(self.mutagen_stars(_or_all_mutagens(mutagens)))

    def not_self_mutaged(
        self, mutagens: Mutagen | list[Mutagen] | None = None
    ) -> bool:
        """
        判断本宫是否没有指定四化中的任何一种自化。

        不传或传空表示检查全部四化。
        """
        return self.not_have(self.mutagen_stars(_or_all_mutagens(mutagens)))

    def mutaged_places(
        self, all_palaces: list[Palace] | None = None
    ) -> list[Palace | None]:
        """
        查看本宫天干四化分别飞入哪些宫位。

        Args:
            all_palaces: 检索范围；不传则取本宫所属星盘的十二宫

        Returns:
            长度为 4 的列表 [禄飞入宫, 权飞入宫, 科飞入宫, 忌飞入宫]，
            未找到时为 None。
        """
        if all_palaces is None:
            if self._astrolabe is None:
                return []
            all_palaces = self._astrolabe.palaces

        result: list[Palace | None] = []
        for star_key in self.mutagen_stars(list(Mutagen)):
            result.append(next((p for p in all_palaces if p.has([star_key])), None))
        return result

    def _resolve_palace(self, target: Palace | int | str) -> Palace | None:
        """把宫位索引或宫名解析为宫位对象；宫位对象原样返回。"""
        if isinstance(target, Palace):
            return target
        if self._astrolabe is None:
            return None
        return self._astrolabe.palace(target)

    # ------ 内部方法 ------

    def _all_star_identifiers(self) -> set[str]:
        return _star_identifiers(
            self.major_stars + self.minor_stars + self.adjective_stars
        )

    @classmethod
    def _from_dict(cls, d: dict) -> Palace:
        return cls(
            index=d["index"],
            name=d["name"],
            name_key=d["nameKey"],
            is_body_palace=d["isBodyPalace"],
            is_original_palace=d["isOriginalPalace"],
            heavenly_stem=d["heavenlyStem"],
            heavenly_stem_key=d["heavenlyStemKey"],
            earthly_branch=d["earthlyBranch"],
            earthly_branch_key=d["earthlyBranchKey"],
            major_stars=[Star._from_dict(s) for s in d["majorStars"]],
            minor_stars=[Star._from_dict(s) for s in d["minorStars"]],
            adjective_stars=[Star._from_dict(s) for s in d["adjectiveStars"]],
            changsheng12=d["changsheng12"],
            changsheng12_key=d["changsheng12Key"],
            boshi12=d["boshi12"],
            boshi12_key=d["boshi12Key"],
            jiangqian12=d["jiangqian12"],
            jiangqian12_key=d["jiangqian12Key"],
            suiqian12=d["suiqian12"],
            suiqian12_key=d["suiqian12Key"],
            decadal=Decadal._from_dict(d["decadal"]),
            ages=list(d["ages"]),
            mutagen_star_keys=list(d["mutagenStarKeys"]),
        )

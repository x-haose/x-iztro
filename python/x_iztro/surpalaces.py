"""
三方四正。

紫微斗数不单看一宫：本宫、对宫、财帛位、官禄位四宫合看，
`SurroundedPalaces` 把这四宫打包，判断方法对四宫取并集。
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import TYPE_CHECKING

from x_iztro.enums import Mutagen
from x_iztro.palace import Palace
from x_iztro.star_object import _star_identifiers

if TYPE_CHECKING:
    from x_iztro.knowledge import KnowledgePack


@dataclass(frozen=True, slots=True)
class SurroundedPalaces:
    """三方四正宫位"""

    target: Palace
    """本宫"""

    opposite: Palace
    """对宫"""

    wealth: Palace
    """财帛位（三方）"""

    career: Palace
    """官禄位（三方）"""

    def to_text(self, *, knowledge: bool | KnowledgePack | None = None) -> str:
        """
        三方四正的语义化文本：本宫、对宫、财帛位、官禄位四宫合看的完整描述。

        从本宫所属星盘的排盘上下文（含重排起点）无状态再发起计算，
        文本按排盘语言输出。

        Args:
            knowledge: 释义材料（True 取排盘语言的内嵌包，或给 KnowledgePack）；
                给出时追加四宫星耀的释义节

        Raises:
            ValueError: 本宫脱离星盘单独构造，无排盘上下文可转发
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        astrolabe = self.target.astrolabe()
        if astrolabe is None:
            raise ValueError(
                "to_text 需要所属星盘：请从 Astrolabe.surrounded_palaces 获取三方四正"
            )
        return astrolabe._context_query(
            "surroundedPalacesToText", palace_index=self.target.index, knowledge=knowledge
        )

    def have(self, stars: list[str]) -> bool:
        """判断三方四正是否包含指定的 **所有** 星耀（接受星耀枚举或当前语言的星名）"""
        identifiers = self._all_star_identifiers()
        return all(s in identifiers for s in stars)

    def not_have(self, stars: list[str]) -> bool:
        """判断三方四正是否 **不** 包含指定的所有星耀"""
        identifiers = self._all_star_identifiers()
        return all(s not in identifiers for s in stars)

    def have_one_of(self, stars: list[str]) -> bool:
        """判断三方四正是否包含指定星耀中的 **至少一颗**"""
        identifiers = self._all_star_identifiers()
        return any(s in identifiers for s in stars)

    def have_mutagen(self, mutagen: Mutagen) -> bool:
        """判断三方四正中是否有指定四化"""
        return any(p.has_mutagen(mutagen) for p in self._all_palaces())

    def not_have_mutagen(self, mutagen: Mutagen) -> bool:
        """判断三方四正中是否没有指定四化"""
        return not self.have_mutagen(mutagen)

    def _all_palaces(self) -> list[Palace]:
        return [self.target, self.opposite, self.wealth, self.career]

    def _all_star_identifiers(self) -> set[str]:
        out: set[str] = set()
        for p in self._all_palaces():
            out |= _star_identifiers(
                p.major_stars + p.minor_stars + p.adjective_stars
            )
        return out

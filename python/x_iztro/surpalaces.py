"""
三方四正。

紫微斗数不单看一宫：本宫、对宫、财帛位、官禄位四宫合看，
`SurroundedPalaces` 把这四宫打包，判断方法对四宫取并集。
"""

from __future__ import annotations

from dataclasses import dataclass

from x_iztro.enums import Mutagen
from x_iztro.palace import Palace
from x_iztro.star_object import _star_identifiers


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

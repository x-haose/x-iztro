"""
星耀对象。

星耀在星盘上不是孤立的：它知道自己落在哪一宫、属于哪张盘，
因此除了自身的亮度与四化，还能直接问出所在宫位的对宫与三方四正。
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import TYPE_CHECKING

from x_iztro.enums import Brightness, Mutagen, ScopeLiteral, StarTypeLiteral

if TYPE_CHECKING:
    from x_iztro.astrolabe import Astrolabe
    from x_iztro.palace import Palace
    from x_iztro.surpalaces import SurroundedPalaces


@dataclass(frozen=True, slots=True)
class Star:
    """星耀"""

    key: str
    """语言无关星耀标识（`MajorStar`/`MinorStar`/`AdjectiveStar` 枚举值域）"""

    name: str
    """星耀名称（按排盘语言翻译）"""

    type: StarTypeLiteral
    """星耀类型（`StarType` 枚举值域）"""

    scope: ScopeLiteral
    """作用范围（`Scope` 枚举值域）"""

    brightness: str | None = None
    """亮度显示文本（如「庙」），无亮度为 None"""

    brightness_key: str | None = None
    """语言无关亮度标识（`Brightness` 枚举值域），无亮度为 None"""

    mutagen: str | None = None
    """四化显示文本（如「禄」），无四化为 None"""

    mutagen_key: str | None = None
    """语言无关四化标识（`Mutagen` 枚举值域），无四化为 None"""

    _palace: Palace | None = field(default=None, compare=False, repr=False)
    """星耀所在宫位，由星盘构造时回填"""

    _astrolabe: Astrolabe | None = field(default=None, compare=False, repr=False)
    """星耀所属星盘，由星盘构造时回填"""

    def with_brightness(self, brightness: Brightness | list[Brightness]) -> bool:
        """判断星耀是否具有指定亮度"""
        if isinstance(brightness, list):
            return self.brightness_key in brightness
        return self.brightness_key == brightness

    def with_mutagen(self, mutagen: Mutagen | list[Mutagen]) -> bool:
        """判断星耀是否具有指定四化"""
        if isinstance(mutagen, list):
            return self.mutagen_key in mutagen
        return self.mutagen_key == mutagen

    def palace(self) -> Palace | None:
        """星耀所在的宫位；脱离星盘单独构造的星耀返回 None"""
        return self._palace

    def opposite_palace(self) -> Palace | None:
        """星耀所在宫位的对宫"""
        if self._palace is None or self._astrolabe is None:
            return None
        return self._astrolabe.palaces[(self._palace.index + 6) % 12]

    def surrounded_palaces(self) -> SurroundedPalaces | None:
        """星耀所在宫位的三方四正"""
        if self._palace is None or self._astrolabe is None:
            return None
        return self._astrolabe.surrounded_palaces(self._palace.index)

    @classmethod
    def _from_dict(cls, d: dict) -> Star:
        return cls(
            key=d["key"],
            name=d["name"],
            type=d["type"],
            scope=d["scope"],
            brightness=d.get("brightness") or None,
            brightness_key=d.get("brightnessKey"),
            mutagen=d.get("mutagen") or None,
            mutagen_key=d.get("mutagenKey"),
        )


def _star_identifiers(stars: list[Star]) -> set[str]:
    """星耀集合的可匹配标识：key 与翻译名皆可作为查询词。"""
    out: set[str] = set()
    for s in stars:
        out.add(s.key)
        out.add(s.name)
    return out

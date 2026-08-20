"""
格局。

格局判定在 Rust 内核完成（规则见文档站「格局」一页），这里只把命中结果
包成类型化对象。本命盘调用 `Astrolabe.patterns()`，运限某层视角调用
`Horoscope.patterns(scope)`；两者都是无状态接口，从星盘自带的排盘上下文重新发起计算。
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from x_iztro.enums import (
    Brightness,
    BrightnessSource,
    Mutagen,
    PalaceName,
    PatternKey,
    Scope,
    ScopeLiteral,
)


@dataclass(frozen=True, slots=True)
class PatternConfig:
    """
    格局判定口径。

    多口径的格局一律以 `PatternHit.variant` 报出，这里只放会改变事实判定本身的开关。
    """

    brightness_source: BrightnessSource = BrightnessSource.TABLE
    """日月明暗依据：`TABLE` 按 iztro 亮度表（默认），`POSITIONAL` 按传统位置"""

    borrow: bool = True
    """空宫是否借对宫主星参与判定"""

    flow_stars: bool = True
    """运限视角下流曜（运禄/流禄等）是否等同对应本命辅星参与判定"""

    def to_dict(self) -> dict[str, Any]:
        """绑定层入参（camelCase 键）"""
        return {
            "brightnessSource": str(self.brightness_source),
            "borrow": self.borrow,
            "flowStars": self.flow_stars,
        }


@dataclass(frozen=True, slots=True)
class PatternStar:
    """参与成格的一颗星及其落宫"""

    key: str
    """语言无关星耀标识"""

    name: str
    """星耀名称（按排盘语言翻译）"""

    palace_index: int
    """落宫索引 (0-11，寅宫为 0)"""

    brightness: str | None = None
    """亮度显示文本，无亮度为 None"""

    brightness_key: str | None = None
    """语言无关亮度标识（`Brightness` 枚举值域），无亮度为 None"""

    mutagen: str | None = None
    """判定视角下的四化显示文本（本命为生年四化，运限为该层四化），无四化为 None"""

    mutagen_key: str | None = None
    """语言无关四化标识（`Mutagen` 枚举值域），无四化为 None"""

    def has_brightness(self, brightness: Brightness | str) -> bool:
        """亮度是否为指定值"""
        return self.brightness_key is not None and self.brightness_key == brightness

    def has_mutagen(self, mutagen: Mutagen | str) -> bool:
        """四化是否为指定值"""
        return self.mutagen_key is not None and self.mutagen_key == mutagen

    @classmethod
    def _from_dict(cls, d: dict) -> PatternStar:
        return cls(
            key=d["key"],
            name=d["name"],
            palace_index=d["palaceIndex"],
            brightness=d.get("brightness"),
            brightness_key=d.get("brightnessKey"),
            mutagen=d.get("mutagen"),
            mutagen_key=d.get("mutagenKey"),
        )


@dataclass(frozen=True, slots=True)
class PatternHit:
    """一次格局命中"""

    key: str
    """语言无关格局标识（`PatternKey` 枚举值域）"""

    name: str
    """格局名称（按排盘语言翻译）"""

    scope: ScopeLiteral
    """判定视角（`Scope` 枚举值域）：本命为 origin，运限为该层"""

    palace_index: int
    """成格所在宫位索引 (0-11)：多数为命宫，「身命」类可为身宫，任一宫成格的为实际落宫"""

    palace_name: str
    """成格所在宫位在该视角下的宫名（按排盘语言翻译）"""

    palace_name_key: str
    """语言无关宫名标识（`PalaceName` 枚举值域）"""

    broken: bool
    """页面称「破格 / 加杀平常」的条件是否触发：成格照报，仅作标记"""

    stars: list[PatternStar]
    """参与成格的星与落宫"""

    variant: str | None = None
    """多口径格局命中的口径（如君臣庆会四形式），单口径为 None"""

    _raw: dict[str, Any] = field(default_factory=dict, compare=False, repr=False)
    """绑定层返回的原始 DTO"""

    def is_(self, key: PatternKey | str) -> bool:
        """是否为指定格局"""
        return self.key == key

    def in_palace(self, name: PalaceName | str) -> bool:
        """成格宫位在该视角下是否为指定宫名（标识或当前语言宫名）"""
        return name == self.palace_name_key or name == self.palace_name

    def to_dict(self) -> dict[str, Any]:
        """命中的 DTO（camelCase 键，值按排盘语言翻译）"""
        return dict(self._raw)

    @classmethod
    def _from_dict(cls, d: dict) -> PatternHit:
        return cls(
            _raw=d,
            key=d["key"],
            name=d["name"],
            scope=d["scope"],
            palace_index=d["palaceIndex"],
            palace_name=d["palaceName"],
            palace_name_key=d["palaceNameKey"],
            broken=bool(d["broken"]),
            stars=[PatternStar._from_dict(s) for s in d["stars"]],
            variant=d.get("variant"),
        )


def _pattern_config(config: PatternConfig | None) -> dict | None:
    """口径转为绑定层入参。"""
    return config.to_dict() if config is not None else None


def _scope_key(scope: Scope | ScopeLiteral) -> str:
    return str(scope)

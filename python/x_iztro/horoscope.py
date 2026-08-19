"""
运限。

大限、小限、流年、流月、流日、流时六个层级各有自己的宫名排布与四化，
流耀则按层级挂在十二宫上；查询方法可免传星盘，取发起本次查询的那张盘。
"""

from __future__ import annotations

import copy
import json
from dataclasses import dataclass, field
from typing import TYPE_CHECKING, Any

from x_iztro.enums import _MUTAGEN_INDEX, Mutagen, PalaceName, Scope, ScopeLiteral
from x_iztro.star_object import Star, _star_identifiers

if TYPE_CHECKING:
    from x_iztro.astrolabe import Astrolabe
    from x_iztro.palace import Palace
    from x_iztro.pattern import PatternConfig, PatternHit
    from x_iztro.surpalaces import SurroundedPalaces


@dataclass(frozen=True, slots=True)
class HoroscopeItem:
    """运限项（大限/小限/流年/流月/流日/流时）"""

    index: int
    """所在宫位的索引 (0-11)"""

    name: str
    """层级显示名（大限/童限/小限/流年/流月/流日/流时，按排盘语言翻译）"""

    heavenly_stem: str
    """该运限天干（翻译文本）"""

    heavenly_stem_key: str
    """该运限天干标识（`HeavenlyStem` 枚举值域）"""

    earthly_branch: str
    """该运限地支（翻译文本）"""

    earthly_branch_key: str
    """该运限地支标识（`EarthlyBranch` 枚举值域）"""

    palace_names: list[str]
    """该运限的十二宫名列表（翻译文本，按宫位索引排列）"""

    palace_name_keys: list[str]
    """该运限的十二宫标识列表（`PalaceName` 枚举值域）"""

    mutagen: list[str]
    """四化星名列表 [禄, 权, 科, 忌]（翻译文本）"""

    mutagen_keys: list[str]
    """四化星标识列表 [禄, 权, 科, 忌]（星耀 key）"""

    stars: list[list[Star]] | None = None
    """流耀，12 个宫位各一组星耀列表，或 None"""

    def palace_index_by_name(self, name: PalaceName | str) -> int | None:
        """通过宫位标识（或当前语言宫名）查找在该运限中的宫位索引"""
        for i, key in enumerate(self.palace_name_keys):
            if name == key or name == self.palace_names[i]:
                return i
        return None

    @classmethod
    def _from_dict(cls, d: dict) -> HoroscopeItem:
        stars = None
        if d.get("stars"):
            stars = [[Star._from_dict(s) for s in group] for group in d["stars"]]
        return cls(
            index=d["index"],
            name=d["name"],
            heavenly_stem=d["heavenlyStem"],
            heavenly_stem_key=d["heavenlyStemKey"],
            earthly_branch=d["earthlyBranch"],
            earthly_branch_key=d["earthlyBranchKey"],
            palace_names=list(d["palaceNames"]),
            palace_name_keys=list(d["palaceNameKeys"]),
            mutagen=list(d["mutagen"]),
            mutagen_keys=list(d["mutagenKeys"]),
            stars=stars,
        )


# ============================================================
# YearlyDecStar
# ============================================================

@dataclass(frozen=True, slots=True)
class YearlyDecStar:
    """流年十二神（按目标年支排布，索引即宫位索引）"""

    jiangqian12: list[str]
    """将前十二神（翻译文本）"""

    jiangqian12_keys: list[str]
    """将前十二神标识（`Jiangqian12` 枚举值域）"""

    suiqian12: list[str]
    """岁前十二神（翻译文本）"""

    suiqian12_keys: list[str]
    """岁前十二神标识（`Suiqian12` 枚举值域）"""

    @classmethod
    def _from_dict(cls, d: dict) -> YearlyDecStar:
        return cls(
            jiangqian12=list(d["jiangqian12"]),
            jiangqian12_keys=list(d["jiangqian12Keys"]),
            suiqian12=list(d["suiqian12"]),
            suiqian12_keys=list(d["suiqian12Keys"]),
        )


# ============================================================
# HoroscopeYearly / AgeItem
# ============================================================

@dataclass(frozen=True, slots=True)
class HoroscopeYearly(HoroscopeItem):
    """流年运限（含流年十二神）"""

    yearly_dec_star: YearlyDecStar | None = None
    """流年十二神（岁前/将前十二神按目标年支排布）"""

    @classmethod
    def _from_dict(cls, d: dict) -> HoroscopeYearly:
        base = HoroscopeItem._from_dict(d)
        return cls(
            index=base.index,
            name=base.name,
            heavenly_stem=base.heavenly_stem,
            heavenly_stem_key=base.heavenly_stem_key,
            earthly_branch=base.earthly_branch,
            earthly_branch_key=base.earthly_branch_key,
            palace_names=base.palace_names,
            palace_name_keys=base.palace_name_keys,
            mutagen=base.mutagen,
            mutagen_keys=base.mutagen_keys,
            stars=base.stars,
            yearly_dec_star=YearlyDecStar._from_dict(d["yearlyDecStar"]),
        )


@dataclass(frozen=True, slots=True)
class AgeItem(HoroscopeItem):
    """小限"""

    nominal_age: int = 0
    """虚岁"""

    @classmethod
    def _from_dict(cls, d: dict) -> AgeItem:
        base = HoroscopeItem._from_dict(d)
        return cls(
            index=base.index,
            name=base.name,
            heavenly_stem=base.heavenly_stem,
            heavenly_stem_key=base.heavenly_stem_key,
            earthly_branch=base.earthly_branch,
            earthly_branch_key=base.earthly_branch_key,
            palace_names=base.palace_names,
            palace_name_keys=base.palace_name_keys,
            mutagen=base.mutagen,
            mutagen_keys=base.mutagen_keys,
            stars=base.stars,
            nominal_age=d["nominalAge"],
        )


# ============================================================
# Horoscope
# ============================================================

@dataclass(frozen=True, slots=True)
class Horoscope:
    """运限"""

    lunar_date: str
    """目标农历日期"""

    solar_date: str
    """目标阳历日期"""

    decadal: HoroscopeItem
    """大限（未起运时为童限）"""

    age: AgeItem
    """小限"""

    yearly: HoroscopeYearly
    """流年"""

    monthly: HoroscopeItem
    """流月"""

    daily: HoroscopeItem
    """流日"""

    hourly: HoroscopeItem
    """流时"""

    _astrolabe: Astrolabe | None = field(default=None, compare=False, repr=False)
    """发起这次运限查询的星盘；由 `Astrolabe.horoscope` 回填"""

    _raw: dict[str, Any] = field(default_factory=dict, compare=False, repr=False)
    """绑定层返回的原始 DTO，供 `to_dict` / `to_json` 导出"""

    _target_time_index: int = field(default=0, compare=False, repr=False)
    """发起本次查询的目标时辰索引；再次发起运限层计算（如格局）时用"""

    def to_dict(self) -> dict[str, Any]:
        """
        运限的 JS iztro 兼容 DTO（camelCase 键，值按排盘语言翻译）的深拷贝。

        导出 JSON 请用本方法或 `to_json`：`dataclasses.asdict()` 会顺着
        `_astrolabe` 回指引用无限递归。
        """
        return copy.deepcopy(self._raw)

    def to_json(self, **kwargs: Any) -> str:
        """
        运限 DTO 的 JSON 字符串，默认不转义非 ASCII 字符。

        Args:
            kwargs: 透传给 `json.dumps`，如 `indent=2`
        """
        kwargs.setdefault("ensure_ascii", False)
        return json.dumps(self._raw, **kwargs)

    def astrolabe(self) -> Astrolabe | None:
        """发起这次运限查询的星盘；脱离星盘单独构造的运限返回 None"""
        return self._astrolabe

    def _chart(self, astrolabe: Astrolabe | None) -> Astrolabe | None:
        """显式传入的星盘优先，否则用发起查询的那张盘。"""
        return astrolabe if astrolabe is not None else self._astrolabe

    def scope_item(self, scope: Scope | ScopeLiteral) -> HoroscopeItem | None:
        """获取指定范围的运限项"""
        return {
            "decadal": self.decadal,
            "yearly": self.yearly,
            "monthly": self.monthly,
            "daily": self.daily,
            "hourly": self.hourly,
        }.get(scope)

    def age_palace(self, astrolabe: Astrolabe | None = None) -> Palace | None:
        """获取小限宫位；不传星盘则取发起本次查询的那张盘"""
        chart = self._chart(astrolabe)
        if chart is None:
            return None
        return chart.palaces[self.age.index]

    def palace(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        astrolabe: Astrolabe | None = None,
    ) -> Palace | None:
        """
        获取指定运限范围下的宫位。

        Args:
            name: 宫位标识（`PalaceName` 枚举，或当前语言的宫名）
            scope: 运限范围
            astrolabe: 星盘对象；不传则取发起本次查询的那张盘
        """
        chart = self._chart(astrolabe)
        if chart is None:
            return None
        if scope == "origin":
            return chart.palace(name)
        item = self.scope_item(scope)
        if item is None:
            return None
        idx = item.palace_index_by_name(name)
        if idx is None:
            return None
        return chart.palaces[idx]

    def surround_palaces(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        astrolabe: Astrolabe | None = None,
    ) -> SurroundedPalaces | None:
        """获取指定运限范围下某宫的三方四正；不传星盘则取发起本次查询的那张盘"""
        chart = self._chart(astrolabe)
        if chart is None:
            return None
        p = self.palace(name, scope, chart)
        if p is None:
            return None
        return chart.surrounded_palaces(p.index)

    def has_horoscope_mutagen(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        mutagen: Mutagen,
        astrolabe: Astrolabe | None = None,
    ) -> bool:
        """
        判断指定运限范围下某宫是否有运限四化。

        Args:
            name: 宫位标识
            scope: 运限范围（origin 总是返回 False）
            mutagen: 四化类型
            astrolabe: 星盘对象；不传则取发起本次查询的那张盘
        """
        if scope == "origin":
            return False
        item = self.scope_item(scope)
        if item is None:
            return False
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        idx = _MUTAGEN_INDEX.get(mutagen)
        if idx is None or idx >= len(item.mutagen_keys):
            return False
        star_key = item.mutagen_keys[idx]
        return any(
            s.key == star_key for s in p.major_stars + p.minor_stars
        )

    def has_horoscope_stars(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe | None = None,
    ) -> bool:
        """判断指定运限宫位是否包含指定的所有流耀（接受星耀枚举或翻译名）"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        identifiers = self._collect_horoscope_star_identifiers(p.index)
        return all(s in identifiers for s in stars)

    def has_one_of_horoscope_stars(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe | None = None,
    ) -> bool:
        """判断指定运限宫位是否包含指定流耀中的任意一颗（接受星耀枚举或翻译名）"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        identifiers = self._collect_horoscope_star_identifiers(p.index)
        return any(s in identifiers for s in stars)

    def not_have_horoscope_stars(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe | None = None,
    ) -> bool:
        """判断指定运限宫位是否不包含指定的所有流耀"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        identifiers = self._collect_horoscope_star_identifiers(p.index)
        return all(s not in identifiers for s in stars)

    def patterns(
        self,
        scope: Scope | ScopeLiteral,
        config: PatternConfig | None = None,
        astrolabe: Astrolabe | None = None,
    ) -> list[PatternHit]:
        """
        某运限层视角的格局命中。

        以该层命宫为命宫、合并该层流曜与四化后重跑全部规则，另加两条行运格
        （禄衰马困、风云际会）；`Scope.ORIGIN` 等同本命盘。

        Args:
            scope: 运限层级
            config: 判定口径；不传取默认
            astrolabe: 星盘；省略时取发起本次运限查询的那张盘

        Returns:
            命中列表，`PatternHit.scope` 即该层

        Raises:
            ValueError: 既未传星盘、本运限也不是由星盘发起
        """
        from x_iztro._bridge import query
        from x_iztro.pattern import PatternHit, _pattern_config, _scope_key

        chart = self._chart(astrolabe)
        if chart is None:
            raise ValueError("patterns() 需要星盘：传入 astrolabe，或由 Astrolabe.horoscope 发起运限")
        data = query(
            "horoscopePatterns",
            solar_date=chart.solar_date,
            time_index=chart.time_index,
            gender=chart.gender_key,
            fix_leap=chart.fix_leap,
            language=chart.language,
            config=chart._config_payload(),
            target_date=self.solar_date,
            target_time_index=self._target_time_index,
            scope=_scope_key(scope),
            pattern_config=_pattern_config(config),
        )
        return [PatternHit._from_dict(d) for d in data]

    def _collect_horoscope_star_identifiers(self, palace_idx: int) -> set[str]:
        """收集大限和流年在指定宫位的所有流耀标识"""
        out: set[str] = set()
        if self.decadal.stars and palace_idx < len(self.decadal.stars):
            out |= _star_identifiers(self.decadal.stars[palace_idx])
        if self.yearly.stars and palace_idx < len(self.yearly.stars):
            out |= _star_identifiers(self.yearly.stars[palace_idx])
        return out

    @classmethod
    def _from_dict(
        cls, d: dict, astrolabe: Astrolabe | None = None, target_time_index: int = 0
    ) -> Horoscope:
        return cls(
            _raw=d,
            _target_time_index=target_time_index,
            lunar_date=d["lunarDate"],
            solar_date=d["solarDate"],
            decadal=HoroscopeItem._from_dict(d["decadal"]),
            age=AgeItem._from_dict(d["age"]),
            yearly=HoroscopeYearly._from_dict(d["yearly"]),
            monthly=HoroscopeItem._from_dict(d["monthly"]),
            daily=HoroscopeItem._from_dict(d["daily"]),
            hourly=HoroscopeItem._from_dict(d["hourly"]),
            _astrolabe=astrolabe,
        )

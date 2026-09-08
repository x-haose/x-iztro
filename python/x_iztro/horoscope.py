"""
运限。

大限、小限、流年、流月、流日、流时六个层级各有自己的宫名排布与四化，
流耀则按层级挂在十二宫上；查询方法可免传星盘，取发起本次查询的那张盘。
"""

from __future__ import annotations

import copy
import json
from dataclasses import dataclass, field, fields
from typing import TYPE_CHECKING, Any

from x_iztro.enums import _MUTAGEN_INDEX, Mutagen, PalaceName, Scope, ScopeLiteral
from x_iztro.star_object import Star, _star_identifiers

if TYPE_CHECKING:
    from x_iztro.astrolabe import Astrolabe
    from x_iztro.knowledge import KnowledgePack
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

    name_key: str
    """层级的语言无关标识。

    大限层未起运时为 "childhood"（童限）而非 "decadal"——两者是不同的解盘语义，
    程序判断一律用本字段而非译文。
    """

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

    mutagen_star_keys: list[str]
    """被化的四颗星标识列表 [禄, 权, 科, 忌]（星耀 key，与 `Palace.mutagen_star_keys`
    同名同义；单数的 mutagen_key 才是四化类型 `Mutagen` 的标识）"""

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
            name_key=d["nameKey"],
            heavenly_stem=d["heavenlyStem"],
            heavenly_stem_key=d["heavenlyStemKey"],
            earthly_branch=d["earthlyBranch"],
            earthly_branch_key=d["earthlyBranchKey"],
            palace_names=list(d["palaceNames"]),
            palace_name_keys=list(d["palaceNameKeys"]),
            mutagen=list(d["mutagen"]),
            mutagen_star_keys=list(d["mutagenStarKeys"]),
            stars=stars,
        )


def _base_fields(d: dict) -> dict[str, Any]:
    """通用运限字段（`HoroscopeItem` 的全部字段）的关键字形态。

    带扩展字段的运限类都以通用字段打底，由本函数一次取全，
    基类新增字段时不必逐个类补抄。
    """
    base = HoroscopeItem._from_dict(d)
    return {f.name: getattr(base, f.name) for f in fields(HoroscopeItem)}


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
        return cls(
            **_base_fields(d),
            yearly_dec_star=YearlyDecStar._from_dict(d["yearlyDecStar"]),
        )


@dataclass(frozen=True, slots=True)
class AgeItem(HoroscopeItem):
    """小限"""

    nominal_age: int = 0
    """虚岁"""

    @classmethod
    def _from_dict(cls, d: dict) -> AgeItem:
        return cls(**_base_fields(d), nominal_age=d["nominalAge"])


# ============================================================
# 运限列表项
# ============================================================

@dataclass(frozen=True, slots=True)
class DecadalListItem(HoroscopeItem):
    """大限列表项：一整个大限，含起止虚岁与起止农历年份"""

    palace_name: str = ""
    """该大限所在的本命宫名（按排盘语言翻译）"""

    palace_name_key: str = ""
    """本命宫名的语言无关标识（`PalaceName` 枚举值域）"""

    age_range: tuple[int, int] = (0, 0)
    """起止虚岁 (起, 止)，含两端"""

    year_range: tuple[int, int] = (0, 0)
    """起止农历年份 (起, 止)，含两端"""

    @classmethod
    def _from_dict(cls, d: dict) -> DecadalListItem:
        return cls(
            **_base_fields(d),
            palace_name=d["palaceName"],
            palace_name_key=d["palaceNameKey"],
            age_range=(d["ageRange"][0], d["ageRange"][1]),
            year_range=(d["yearRange"][0], d["yearRange"][1]),
        )


@dataclass(frozen=True, slots=True)
class YearlyListItem(HoroscopeItem):
    """流年列表项：某个大限内的一个流年"""

    age: int = 0
    """该流年对应的虚岁"""

    year: int = 0
    """农历年份"""

    @classmethod
    def _from_dict(cls, d: dict) -> YearlyListItem:
        return cls(**_base_fields(d), age=d["age"], year=d["year"])


@dataclass(frozen=True, slots=True)
class MonthlyListItem(HoroscopeItem):
    """流月列表项：某个农历年的一个流月段"""

    age: int = 0
    """该流月对应的虚岁"""

    year: int = 0
    """农历年份"""

    month: int = 0
    """农历月份（正月为 1）；闰月与同月号的常规月共用本字段，以 `is_leap_month` 区分"""

    is_leap_month: bool = False
    """本段是否属于闰月"""

    part: str = ""
    """分段标识（`MonthPart` 枚举值域）：整月、闰月前半、闰月后半"""

    day_range: tuple[int, int] = (0, 0)
    """本段覆盖的农历日区间 (起, 止)，含两端"""

    @classmethod
    def _from_dict(cls, d: dict) -> MonthlyListItem:
        return cls(
            **_base_fields(d),
            age=d["age"],
            year=d["year"],
            month=d["month"],
            is_leap_month=d["isLeapMonth"],
            part=d["part"],
            day_range=(d["dayRange"][0], d["dayRange"][1]),
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

    def to_text(
        self,
        *,
        knowledge: bool | KnowledgePack | None = None,
        config: PatternConfig | None = None,
    ) -> str:
        """
        运限的语义化文本：面向语言模型与人的完整描述，`str(horoscope)` 等价于不带释义的形态。

        与 `to_dict`/`to_json`（机器结构）、翻译字段（展示文本）同源，
        是同一份运限的第三种投影。文本是 Markdown 子集（标题、列表、表格），按排盘语言输出。

        Args:
            knowledge: 释义材料（True 取排盘语言的内嵌包，或给 KnowledgePack）；
                给出时各层事实之后紧跟该层流耀释义与格局释义
            config: 格局判定口径，与 `patterns(config)` 同形态；None 取默认口径

        Raises:
            ValueError: 本运限不是由星盘发起（脱离星盘无排盘上下文可转发）
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        from x_iztro.pattern import _pattern_config

        return self._context_query(
            "horoscopeToText",
            None,
            knowledge=knowledge,
            pattern_config=_pattern_config(config),
        )

    def __str__(self) -> str:
        # 脱离星盘的运限没有排盘上下文可转发，str() 必须仍是安全操作：
        # 回退到默认 repr 而不是从 __str__ 里抛异常
        if self._astrolabe is None:
            return super().__repr__()
        return self.to_text()

    def astrolabe(self) -> Astrolabe | None:
        """发起这次运限查询的星盘；脱离星盘单独构造的运限返回 None"""
        return self._astrolabe

    def _context_query(self, kind: str, astrolabe: Astrolabe | None, **extra: Any) -> Any:
        """以发起本次运限查询的排盘上下文调用绑定层：星盘上下文（含重排起点）
        委托给 `Astrolabe._context_query`（上下文键只维护那一处），再附加目标日期。"""
        chart = self._chart(astrolabe)
        if chart is None:
            raise ValueError(
                f"{kind} 需要星盘：传入 astrolabe，或由 Astrolabe.horoscope 发起运限"
            )
        return chart._context_query(
            kind,
            target_date=self.solar_date,
            target_time_index=self._target_time_index,
            **extra,
        )

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
        if idx is None or idx >= len(item.mutagen_star_keys):
            return False
        star_key = item.mutagen_star_keys[idx]
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
        from x_iztro.pattern import PatternHit, _pattern_config, _scope_key

        data = self._context_query(
            "horoscopePatterns",
            astrolabe,
            scope=_scope_key(scope),
            pattern_config=_pattern_config(config),
        )
        return [PatternHit._from_dict(d) for d in data]

    def patterns_to_text(
        self,
        scope: Scope | ScopeLiteral,
        config: PatternConfig | None = None,
        astrolabe: Astrolabe | None = None,
        *,
        knowledge: bool | KnowledgePack | None = None,
    ) -> str:
        """
        某运限层视角格局命中的语义化文本。

        与 `patterns` 同一套判定（含重排上下文与判定口径），输出面向语言模型
        与人的文本而非结构化命中列表；宫名按该层重排后的宫名书写。

        Args:
            scope: 运限层级
            config: 判定口径；不传取默认
            astrolabe: 星盘；省略时取发起本次运限查询的那张盘
            knowledge: 释义材料（True 取排盘语言的内嵌包，或给 KnowledgePack）；
                给出时格局列表之后紧跟各格局的释义

        Raises:
            ValueError: 既未传星盘、本运限也不是由星盘发起
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        from x_iztro.pattern import _pattern_config, _scope_key

        return self._context_query(
            "horoscopePatternsToText",
            astrolabe,
            scope=_scope_key(scope),
            pattern_config=_pattern_config(config),
            knowledge=knowledge,
        )

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

"""
rs-iztro 数据模型。

从 Rust 返回的星盘数据 1:1 映射的 frozen dataclass，零外部依赖。
每个对象同时携带两层信息：

- 翻译字段（`name`、`brightness` 等）：按排盘语言本地化的展示文本；
- 标识字段（`key`、`name_key` 等 `*_key`）：语言无关的 iztro i18n key。

所有判断方法（`has`/`has_mutagen`/`flies_to`/`palace` 查询等）基于标识字段
比较，传入 `rs_iztro.enums` 的枚举成员即可在任何输出语言的星盘上正确工作；
为方便中文场景，接受星耀/宫位参数的方法同时兼容当前语言的翻译名。
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Literal

from rs_iztro.enums import (
    Brightness,
    Mutagen,
    PalaceName,
    Scope,
)

# ============================================================
# 类型别名
# ============================================================

TimeIndexType = Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
GenderType = Literal["male", "female"]
LanguageType = Literal["zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", "vi_vn"]
StarTypeLiteral = Literal[
    "major", "soft", "tough", "adjective", "flower", "helper", "lucun", "tianma"
]
ScopeLiteral = Literal["origin", "decadal", "yearly", "monthly", "daily", "hourly"]


# ============================================================
# 天干四化表（天干 key → [禄, 权, 科, 忌] 的星耀 key）
# ============================================================

_MUTAGEN_TABLE: dict[str, tuple[str, str, str, str]] = {
    "jiaHeavenly": ("lianzhenMaj", "pojunMaj", "wuquMaj", "taiyangMaj"),
    "yiHeavenly": ("tianjiMaj", "tianliangMaj", "ziweiMaj", "taiyinMaj"),
    "bingHeavenly": ("tiantongMaj", "tianjiMaj", "wenchangMin", "lianzhenMaj"),
    "dingHeavenly": ("taiyinMaj", "tiantongMaj", "tianjiMaj", "jumenMaj"),
    "wuHeavenly": ("tanlangMaj", "taiyinMaj", "youbiMin", "tianjiMaj"),
    "jiHeavenly": ("wuquMaj", "tanlangMaj", "tianliangMaj", "wenquMin"),
    "gengHeavenly": ("taiyangMaj", "wuquMaj", "taiyinMaj", "tiantongMaj"),
    "xinHeavenly": ("jumenMaj", "taiyangMaj", "wenquMin", "wenchangMin"),
    "renHeavenly": ("tianliangMaj", "ziweiMaj", "zuofuMin", "wuquMaj"),
    "guiHeavenly": ("pojunMaj", "jumenMaj", "taiyinMaj", "tanlangMaj"),
}

_MUTAGEN_INDEX: dict[str, int] = {
    Mutagen.LU: 0,
    Mutagen.QUAN: 1,
    Mutagen.KE: 2,
    Mutagen.JI: 3,
}


# ============================================================
# Star
# ============================================================

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


# ============================================================
# RawDates / ChartConfig
# ============================================================

@dataclass(frozen=True, slots=True)
class RawLunarDate:
    """数字化农历生日"""

    lunar_year: int
    """农历年"""

    lunar_month: int
    """农历月（1-12，闰月与否见 is_leap）"""

    lunar_day: int
    """农历日（1-30）"""

    is_leap: bool
    """是否闰月"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawLunarDate:
        return cls(
            lunar_year=d["lunarYear"],
            lunar_month=d["lunarMonth"],
            lunar_day=d["lunarDay"],
            is_leap=d["isLeap"],
        )


@dataclass(frozen=True, slots=True)
class RawChineseDate:
    """四柱干支（每柱为 [天干, 地支]，未本地化的干支原文）"""

    yearly: tuple[str, str]
    """年柱"""

    monthly: tuple[str, str]
    """月柱"""

    daily: tuple[str, str]
    """日柱"""

    hourly: tuple[str, str]
    """时柱"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawChineseDate:
        return cls(
            yearly=tuple(d["yearly"]),
            monthly=tuple(d["monthly"]),
            daily=tuple(d["daily"]),
            hourly=tuple(d["hourly"]),
        )


@dataclass(frozen=True, slots=True)
class RawDates:
    """结构化的出生日期信息"""

    lunar_date: RawLunarDate
    """数字化农历生日"""

    chinese_date: RawChineseDate
    """四柱干支"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawDates:
        return cls(
            lunar_date=RawLunarDate._from_dict(d["lunarDate"]),
            chinese_date=RawChineseDate._from_dict(d["chineseDate"]),
        )


@dataclass(frozen=True, slots=True)
class ChartConfig:
    """排盘配置。字段取值见 `rs_iztro.enums` 的对应枚举；默认值与 JS iztro 一致。"""

    year_divide: str = "normal"
    """年分界点（`YearDivide`）：normal=正月初一 / exact=立春"""

    horoscope_divide: str = "normal"
    """运限分界点（`HoroscopeDivide`）：normal=初一 / exact=节气"""

    age_divide: str = "normal"
    """虚岁分界点（`AgeDivide`）：normal=跨年即加 / birthday=过生日才加"""

    day_divide: str = "forward"
    """晚子时归属（`DayDivide`）：forward=归次日 / current=归当天"""

    algorithm: str = "default"
    """算法派别（`Algorithm`）：default / zhongzhou"""

    def to_json(self) -> str:
        """转为绑定层接受的 config JSON"""
        import json
        return json.dumps({
            "yearDivide": self.year_divide,
            "horoscopeDivide": self.horoscope_divide,
            "ageDivide": self.age_divide,
            "dayDivide": self.day_divide,
            "algorithm": self.algorithm,
        })

    @classmethod
    def _from_dict(cls, d: dict) -> ChartConfig:
        return cls(
            year_divide=d["yearDivide"],
            horoscope_divide=d["horoscopeDivide"],
            age_divide=d["ageDivide"],
            day_divide=d["dayDivide"],
            algorithm=d["algorithm"],
        )


# ============================================================
# Decadal
# ============================================================

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

    def is_empty(self) -> bool:
        """判断宫位是否为空宫（无主星）"""
        return len(self.major_stars) == 0

    # ------ 四化飞星 ------

    def flies_to(self, target: Palace, mutagen: Mutagen) -> bool:
        """
        判断本宫天干四化是否飞入目标宫位。

        Args:
            target: 目标宫位
            mutagen: 四化类型（`Mutagen` 枚举）
        """
        star_key = self._mutagen_star(mutagen)
        if star_key is None:
            return False
        return target.has([star_key])

    def self_mutaged(self, mutagen: Mutagen) -> bool:
        """判断本宫是否自化（天干四化星落在本宫）"""
        return self.flies_to(self, mutagen)

    def self_mutaged_one_of(self) -> bool:
        """判断本宫是否有任意一种自化"""
        return any(self.self_mutaged(m) for m in Mutagen)

    def not_self_mutaged(self) -> bool:
        """判断本宫是否没有任何自化"""
        return not self.self_mutaged_one_of()

    def mutaged_places(self, all_palaces: list[Palace]) -> list[Palace | None]:
        """
        查看本宫天干四化分别飞入哪些宫位。

        Returns:
            长度为 4 的列表 [禄飞入宫, 权飞入宫, 科飞入宫, 忌飞入宫]，
            未找到时为 None。
        """
        star_keys = _MUTAGEN_TABLE.get(self.heavenly_stem_key, ())
        result: list[Palace | None] = []
        for star_key in star_keys:
            found = None
            for p in all_palaces:
                if p.has([star_key]):
                    found = p
                    break
            result.append(found)
        return result

    # ------ 内部方法 ------

    def _all_star_identifiers(self) -> set[str]:
        return _star_identifiers(
            self.major_stars + self.minor_stars + self.adjective_stars
        )

    def _mutagen_star(self, mutagen: Mutagen) -> str | None:
        """根据宫干和四化类型，返回对应的星耀标识"""
        stars = _MUTAGEN_TABLE.get(self.heavenly_stem_key)
        if stars is None:
            return None
        idx = _MUTAGEN_INDEX.get(mutagen)
        if idx is None:
            return None
        return stars[idx]

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
        )


# ============================================================
# SurroundedPalaces (三方四正)
# ============================================================

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


# ============================================================
# HoroscopeItem
# ============================================================

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

    def scope_item(self, scope: Scope | ScopeLiteral) -> HoroscopeItem | None:
        """获取指定范围的运限项"""
        return {
            "decadal": self.decadal,
            "yearly": self.yearly,
            "monthly": self.monthly,
            "daily": self.daily,
            "hourly": self.hourly,
        }.get(scope)

    def age_palace(self, astrolabe: Astrolabe) -> Palace:
        """获取小限宫位"""
        return astrolabe.palaces[self.age.index]

    def palace(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        astrolabe: Astrolabe,
    ) -> Palace | None:
        """
        获取指定运限范围下的宫位。

        Args:
            name: 宫位标识（`PalaceName` 枚举，或当前语言的宫名）
            scope: 运限范围
            astrolabe: 星盘对象
        """
        if scope == "origin":
            return astrolabe.palace(name)
        item = self.scope_item(scope)
        if item is None:
            return None
        idx = item.palace_index_by_name(name)
        if idx is None:
            return None
        return astrolabe.palaces[idx]

    def surround_palaces(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        astrolabe: Astrolabe,
    ) -> SurroundedPalaces | None:
        """获取指定运限范围下某宫的三方四正"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return None
        return astrolabe.surrounded_palaces(p.index)

    def has_horoscope_mutagen(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        mutagen: Mutagen,
        astrolabe: Astrolabe,
    ) -> bool:
        """
        判断指定运限范围下某宫是否有运限四化。

        Args:
            name: 宫位标识
            scope: 运限范围（origin 总是返回 False）
            mutagen: 四化类型
            astrolabe: 星盘对象
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
        astrolabe: Astrolabe,
    ) -> bool:
        """判断指定运限宫位是否包含指定的所有流耀（接受星耀枚举或翻译名）"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        identifiers = self._collect_horoscope_star_identifiers(p.index)
        return all(s in identifiers for s in stars)

    def not_have_horoscope_stars(
        self,
        name: PalaceName | str,
        scope: Scope | ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe,
    ) -> bool:
        """判断指定运限宫位是否不包含指定的所有流耀"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        identifiers = self._collect_horoscope_star_identifiers(p.index)
        return all(s not in identifiers for s in stars)

    def _collect_horoscope_star_identifiers(self, palace_idx: int) -> set[str]:
        """收集大限和流年在指定宫位的所有流耀标识"""
        out: set[str] = set()
        if self.decadal.stars and palace_idx < len(self.decadal.stars):
            out |= _star_identifiers(self.decadal.stars[palace_idx])
        if self.yearly.stars and palace_idx < len(self.yearly.stars):
            out |= _star_identifiers(self.yearly.stars[palace_idx])
        return out

    @classmethod
    def _from_dict(cls, d: dict) -> Horoscope:
        return cls(
            lunar_date=d["lunarDate"],
            solar_date=d["solarDate"],
            decadal=HoroscopeItem._from_dict(d["decadal"]),
            age=AgeItem._from_dict(d["age"]),
            yearly=HoroscopeYearly._from_dict(d["yearly"]),
            monthly=HoroscopeItem._from_dict(d["monthly"]),
            daily=HoroscopeItem._from_dict(d["daily"]),
            hourly=HoroscopeItem._from_dict(d["hourly"]),
        )


# ============================================================
# Astrolabe (主对象)
# ============================================================

@dataclass(frozen=True, slots=True)
class Astrolabe:
    """星盘"""

    gender: str
    """性别（按排盘语言翻译）"""

    gender_key: str
    """机器可读性别（`Gender` 枚举值域）"""

    solar_date: str
    """阳历日期"""

    lunar_date: str
    """农历日期"""

    chinese_date: str
    """干支纪年日期"""

    raw_dates: RawDates
    """结构化的农历生日与四柱干支"""

    time: str
    """时辰"""

    time_range: str
    """时辰对应的时间段"""

    sign: str
    """星座"""

    zodiac: str
    """生肖"""

    earthly_branch_of_soul_palace: str
    """命宫地支（翻译文本）"""

    earthly_branch_of_soul_palace_key: str
    """命宫地支标识（`EarthlyBranch` 枚举值域）"""

    earthly_branch_of_body_palace: str
    """身宫地支（翻译文本）"""

    earthly_branch_of_body_palace_key: str
    """身宫地支标识（`EarthlyBranch` 枚举值域）"""

    soul: str
    """命主星（翻译文本）"""

    soul_key: str
    """命主星标识（星耀 key）"""

    body: str
    """身主星（翻译文本）"""

    body_key: str
    """身主星标识（星耀 key）"""

    five_elements_class: str
    """五行局（翻译文本）"""

    five_elements_class_key: str
    """五行局标识（"water2nd" 等）"""

    palaces: list[Palace]
    """十二宫数据"""

    time_index: int
    """出生时辰索引 (0-12)"""

    fix_leap: bool
    """是否修正闰月"""

    language: str
    """排盘语言（`Language` 枚举值域）"""

    config: ChartConfig
    """排盘配置"""

    # ------ 宫位查询 ------

    def palace(self, index_or_name: int | PalaceName | str) -> Palace | None:
        """
        通过索引、宫位标识或当前语言宫名获取宫位。

        Args:
            index_or_name: 宫位索引 (0-11)、`PalaceName` 枚举，或当前语言的宫名
        """
        if isinstance(index_or_name, int):
            if 0 <= index_or_name < len(self.palaces):
                return self.palaces[index_or_name]
            return None
        for p in self.palaces:
            if index_or_name in (p.name_key, p.name):
                return p
        return None

    def surrounded_palaces(self, index: int) -> SurroundedPalaces:
        """
        获取指定宫位的三方四正。

        Args:
            index: 宫位索引 (0-11)

        Returns:
            SurroundedPalaces 包含本宫、对宫、财帛位、官禄位
        """
        return SurroundedPalaces(
            target=self.palaces[index],
            opposite=self.palaces[(index + 6) % 12],
            wealth=self.palaces[(index + 8) % 12],
            career=self.palaces[(index + 4) % 12],
        )

    # ------ 星耀查询 ------

    def star(self, star: str) -> Star | None:
        """通过星耀标识（或当前语言星名）查找星耀"""
        found = self.star_in_palace(star)
        return found[0] if found else None

    def star_in_palace(self, star: str) -> tuple[Star, Palace] | None:
        """
        通过星耀标识（或当前语言星名）查找星耀及其所在宫位。

        Returns:
            (Star, Palace) 元组，未找到时返回 None
        """
        for p in self.palaces:
            for s in p.major_stars + p.minor_stars + p.adjective_stars:
                if star in (s.key, s.name):
                    return (s, p)
        return None

    @classmethod
    def _from_dict(cls, d: dict) -> Astrolabe:
        return cls(
            gender=d["gender"],
            gender_key=d["genderKey"],
            solar_date=d["solarDate"],
            lunar_date=d["lunarDate"],
            chinese_date=d["chineseDate"],
            raw_dates=RawDates._from_dict(d["rawDates"]),
            time=d["time"],
            time_range=d["timeRange"],
            sign=d["sign"],
            zodiac=d["zodiac"],
            earthly_branch_of_soul_palace=d["earthlyBranchOfSoulPalace"],
            earthly_branch_of_soul_palace_key=d["earthlyBranchOfSoulPalaceKey"],
            earthly_branch_of_body_palace=d["earthlyBranchOfBodyPalace"],
            earthly_branch_of_body_palace_key=d["earthlyBranchOfBodyPalaceKey"],
            soul=d["soul"],
            soul_key=d["soulKey"],
            body=d["body"],
            body_key=d["bodyKey"],
            five_elements_class=d["fiveElementsClass"],
            five_elements_class_key=d["fiveElementsClassKey"],
            palaces=[Palace._from_dict(p) for p in d["palaces"]],
            time_index=d["timeIndex"],
            fix_leap=d["fixLeap"],
            language=d["language"],
            config=ChartConfig._from_dict(d["config"]),
        )

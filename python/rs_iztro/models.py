"""
rs-iztro 数据模型

从 Rust 结构体 1:1 映射，dataclasses（零外部依赖）。
覆盖所有字段、方法、枚举，IDE 100% 自动补全。
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Literal


# ============================================================
# 类型别名
# ============================================================

TimeIndexType = Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
GenderType = Literal["male", "female"]
LanguageType = Literal["zh_cn", "zh_tw", "en_us", "ja_jp", "ko_kr", "vi_vn"]
AlgorithmType = Literal["default", "zhongzhou"]
StarTypeLiteral = Literal["major", "soft", "tough", "adjective", "flower", "helper", "lucun", "tianma"]
ScopeLiteral = Literal["origin", "decadal", "yearly", "monthly", "daily", "hourly"]
MutagenLiteral = Literal["禄", "权", "科", "忌"]
BrightnessLiteral = Literal["庙", "旺", "得", "利", "平", "不", "陷"]


# ============================================================
# 天干四化表（heavenly stem → [禄, 权, 科, 忌] 的星耀名）
# ============================================================

_MUTAGEN_TABLE: dict[str, list[str]] = {
    "甲": ["廉贞", "破军", "武曲", "太阳"],
    "乙": ["天机", "天梁", "紫微", "太阴"],
    "丙": ["天同", "天机", "文昌", "廉贞"],
    "丁": ["太阴", "天同", "天机", "巨门"],
    "戊": ["贪狼", "太阴", "右弼", "天机"],
    "己": ["武曲", "贪狼", "天梁", "文曲"],
    "庚": ["太阳", "武曲", "太阴", "天同"],
    "辛": ["巨门", "太阳", "文曲", "文昌"],
    "壬": ["天梁", "紫微", "左辅", "武曲"],
    "癸": ["破军", "巨门", "太阴", "贪狼"],
}


# ============================================================
# Star
# ============================================================

@dataclass(frozen=True, slots=True)
class Star:
    """星耀"""

    name: str
    """星耀名称"""

    type: StarTypeLiteral
    """星耀类型：major/soft/tough/adjective/flower/helper/lucun/tianma"""

    scope: ScopeLiteral
    """作用范围：origin/decadal/yearly/monthly/daily/hourly"""

    brightness: str | None = None
    """亮度：庙/旺/得/利/平/不/陷，None 表示无亮度"""

    mutagen: str | None = None
    """四化：禄/权/科/忌，None 表示无四化"""

    def with_brightness(self, brightness: BrightnessLiteral | list[BrightnessLiteral]) -> bool:
        """判断星耀是否具有指定亮度"""
        if isinstance(brightness, list):
            return self.brightness in brightness
        return self.brightness == brightness

    def with_mutagen(self, mutagen: MutagenLiteral | list[MutagenLiteral]) -> bool:
        """判断星耀是否具有指定四化"""
        if isinstance(mutagen, list):
            return self.mutagen in mutagen
        return self.mutagen == mutagen

    @classmethod
    def _from_dict(cls, d: dict) -> Star:
        return cls(
            name=d["name"],
            type=d["type"],
            scope=d["scope"],
            brightness=d.get("brightness") or None,
            mutagen=d.get("mutagen") or None,
        )


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
    """排盘配置（字符串取值，与 config JSON 的键值一致）"""

    year_divide: str
    """年分界点：normal | exact"""

    horoscope_divide: str
    """运限分界点：normal | exact"""

    age_divide: str
    """虚岁分界点：normal | birthday"""

    day_divide: str
    """晚子时归属：forward | current"""

    algorithm: str
    """算法派别：default | zhongzhou"""

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
    """大限天干"""

    earthly_branch: str
    """大限地支"""

    @classmethod
    def _from_dict(cls, d: dict) -> Decadal:
        r = d["range"]
        return cls(
            range=(r[0], r[1]),
            heavenly_stem=d["heavenlyStem"],
            earthly_branch=d["earthlyBranch"],
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
    """宫位名称：命宫/兄弟/夫妻/子女/财帛/疾厄/迁移/交友/官禄/田宅/福德/父母"""

    is_body_palace: bool
    """是否身宫"""

    is_original_palace: bool
    """是否来因宫"""

    heavenly_stem: str
    """宫位天干"""

    earthly_branch: str
    """宫位地支"""

    major_stars: list[Star]
    """主星列表"""

    minor_stars: list[Star]
    """辅星列表"""

    adjective_stars: list[Star]
    """杂耀列表"""

    changsheng12: str
    """长生十二神"""

    boshi12: str
    """博士十二神"""

    jiangqian12: str
    """将前十二神"""

    suiqian12: str
    """岁前十二神"""

    decadal: Decadal
    """大限信息"""

    ages: list[int]
    """小限经过年龄"""

    # ------ 星耀判断 ------

    def has(self, stars: list[str]) -> bool:
        """判断宫位是否包含指定的 **所有** 星耀"""
        names = self._all_star_names()
        return all(s in names for s in stars)

    def not_have(self, stars: list[str]) -> bool:
        """判断宫位是否 **不** 包含指定的所有星耀"""
        names = self._all_star_names()
        return all(s not in names for s in stars)

    def has_one_of(self, stars: list[str]) -> bool:
        """判断宫位是否包含指定星耀中的 **至少一颗**"""
        names = self._all_star_names()
        return any(s in names for s in stars)

    def has_mutagen(self, mutagen: MutagenLiteral) -> bool:
        """判断宫位是否有指定四化（只检查主星和辅星）"""
        return any(s.mutagen == mutagen for s in self.major_stars + self.minor_stars)

    def not_have_mutagen(self, mutagen: MutagenLiteral) -> bool:
        """判断宫位是否没有指定四化"""
        return not self.has_mutagen(mutagen)

    def is_empty(self) -> bool:
        """判断宫位是否为空宫（无主星）"""
        return len(self.major_stars) == 0

    # ------ 四化飞星 ------

    def flies_to(self, target: Palace, mutagen: MutagenLiteral) -> bool:
        """
        判断本宫天干四化是否飞入目标宫位。

        Args:
            target: 目标宫位
            mutagen: 四化类型（禄/权/科/忌）
        """
        star_name = self._mutagen_star(mutagen)
        if star_name is None:
            return False
        return target.has([star_name])

    def self_mutaged(self, mutagen: MutagenLiteral) -> bool:
        """判断本宫是否自化（天干四化星落在本宫）"""
        return self.flies_to(self, mutagen)

    def self_mutaged_one_of(self) -> bool:
        """判断本宫是否有任意一种自化"""
        return any(self.self_mutaged(m) for m in ("禄", "权", "科", "忌"))

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
        stems = _MUTAGEN_TABLE.get(self.heavenly_stem, [])
        result: list[Palace | None] = []
        for star_name in stems:
            found = None
            for p in all_palaces:
                if p.has([star_name]):
                    found = p
                    break
            result.append(found)
        return result

    # ------ 内部方法 ------

    def _all_star_names(self) -> set[str]:
        return {s.name for s in self.major_stars + self.minor_stars + self.adjective_stars}

    def _mutagen_star(self, mutagen: MutagenLiteral) -> str | None:
        """根据天干和四化类型，返回对应的星耀名"""
        stems = _MUTAGEN_TABLE.get(self.heavenly_stem)
        if stems is None:
            return None
        idx = {"禄": 0, "权": 1, "科": 2, "忌": 3}.get(mutagen)
        if idx is None:
            return None
        return stems[idx]

    @classmethod
    def _from_dict(cls, d: dict) -> Palace:
        return cls(
            index=d["index"],
            name=d["name"],
            is_body_palace=d["isBodyPalace"],
            is_original_palace=d["isOriginalPalace"],
            heavenly_stem=d["heavenlyStem"],
            earthly_branch=d["earthlyBranch"],
            major_stars=[Star._from_dict(s) for s in d["majorStars"]],
            minor_stars=[Star._from_dict(s) for s in d["minorStars"]],
            adjective_stars=[Star._from_dict(s) for s in d["adjectiveStars"]],
            changsheng12=d["changsheng12"],
            boshi12=d["boshi12"],
            jiangqian12=d["jiangqian12"],
            suiqian12=d["suiqian12"],
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
        """判断三方四正是否包含指定的 **所有** 星耀"""
        names = self._all_star_names()
        return all(s in names for s in stars)

    def not_have(self, stars: list[str]) -> bool:
        """判断三方四正是否 **不** 包含指定的所有星耀"""
        names = self._all_star_names()
        return all(s not in names for s in stars)

    def have_one_of(self, stars: list[str]) -> bool:
        """判断三方四正是否包含指定星耀中的 **至少一颗**"""
        names = self._all_star_names()
        return any(s in names for s in stars)

    def have_mutagen(self, mutagen: MutagenLiteral) -> bool:
        """判断三方四正中是否有指定四化"""
        return any(p.has_mutagen(mutagen) for p in self._all_palaces())

    def not_have_mutagen(self, mutagen: MutagenLiteral) -> bool:
        """判断三方四正中是否没有指定四化"""
        return not self.have_mutagen(mutagen)

    def _all_palaces(self) -> list[Palace]:
        return [self.target, self.opposite, self.wealth, self.career]

    def _all_star_names(self) -> set[str]:
        names: set[str] = set()
        for p in self._all_palaces():
            names.update(s.name for s in p.major_stars + p.minor_stars + p.adjective_stars)
        return names


# ============================================================
# HoroscopeItem
# ============================================================

@dataclass(frozen=True, slots=True)
class HoroscopeItem:
    """运限项（大限/流年/流月/流日/流时）"""

    index: int
    """所在宫位的索引 (0-11)"""

    name: str
    """运限名称"""

    heavenly_stem: str
    """该运限天干"""

    earthly_branch: str
    """该运限地支"""

    palace_names: list[str]
    """该运限的十二宫名称列表（按宫位索引排列）"""

    mutagen: list[str]
    """四化星名列表 [禄, 权, 科, 忌]"""

    stars: list[list[Star]] | None = None
    """流耀，12 个宫位各一组星耀列表，或 None"""

    def palace_index_by_name(self, name: str) -> int | None:
        """通过宫位名称查找在该运限中的索引"""
        try:
            return self.palace_names.index(name)
        except ValueError:
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
            earthly_branch=d["earthlyBranch"],
            palace_names=list(d["palaceNames"]),
            mutagen=list(d["mutagen"]),
            stars=stars,
        )


# ============================================================
# YearlyDecStar
# ============================================================

@dataclass(frozen=True, slots=True)
class YearlyDecStar:
    """流年十二神"""

    jiangqian12: list[str]
    """将前十二神"""

    suiqian12: list[str]
    """岁前十二神"""

    @classmethod
    def _from_dict(cls, d: dict) -> YearlyDecStar:
        return cls(
            jiangqian12=list(d["jiangqian12"]),
            suiqian12=list(d["suiqian12"]),
        )


# ============================================================
# HoroscopeYearly
# ============================================================

@dataclass(frozen=True, slots=True)
class HoroscopeYearly(HoroscopeItem):
    """流年运限（含流年十二神）"""

    yearly_dec_star: YearlyDecStar | None = None
    """流年十二神（岁前/将前十二神按目标年支排布）"""

    @classmethod
    def _from_dict(cls, d: dict) -> HoroscopeYearly:
        stars = None
        if d.get("stars"):
            stars = [[Star._from_dict(s) for s in group] for group in d["stars"]]
        yds = YearlyDecStar._from_dict(d["yearlyDecStar"])
        return cls(
            index=d["index"],
            name=d["name"],
            heavenly_stem=d["heavenlyStem"],
            earthly_branch=d["earthlyBranch"],
            palace_names=list(d["palaceNames"]),
            mutagen=list(d["mutagen"]),
            stars=stars,
            yearly_dec_star=yds,
        )


# ============================================================
# AgeItem (小限)
# ============================================================

@dataclass(frozen=True, slots=True)
class AgeItem(HoroscopeItem):
    """小限"""

    nominal_age: int = 0
    """虚岁"""

    @classmethod
    def _from_dict(cls, d: dict) -> AgeItem:
        stars = None
        if d.get("stars"):
            stars = [[Star._from_dict(s) for s in group] for group in d["stars"]]
        return cls(
            index=d["index"],
            name=d["name"],
            heavenly_stem=d["heavenlyStem"],
            earthly_branch=d["earthlyBranch"],
            palace_names=list(d["palaceNames"]),
            mutagen=list(d["mutagen"]),
            stars=stars,
            nominal_age=d["nominalAge"],
        )


# ============================================================
# Horoscope
# ============================================================

@dataclass(frozen=True, slots=True)
class Horoscope:
    """运限"""

    lunar_date: str
    """农历日期"""

    solar_date: str
    """阳历日期"""

    decadal: HoroscopeItem
    """大限"""

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

    def scope_item(self, scope: ScopeLiteral) -> HoroscopeItem | None:
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
        name: str,
        scope: ScopeLiteral,
        astrolabe: Astrolabe,
    ) -> Palace | None:
        """
        获取指定运限范围下的宫位。

        Args:
            name: 宫位名称（如 "命宫"）
            scope: 运限范围（origin/decadal/yearly/monthly/daily/hourly）
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
        name: str,
        scope: ScopeLiteral,
        astrolabe: Astrolabe,
    ) -> SurroundedPalaces | None:
        """获取指定运限范围下某宫的三方四正"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return None
        return astrolabe.surrounded_palaces(p.index)

    def has_horoscope_mutagen(
        self,
        name: str,
        scope: ScopeLiteral,
        mutagen: MutagenLiteral,
        astrolabe: Astrolabe,
    ) -> bool:
        """
        判断指定运限范围下某宫是否有运限四化。

        Args:
            name: 宫位名称
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
        idx = {"禄": 0, "权": 1, "科": 2, "忌": 3}.get(mutagen)
        if idx is None or idx >= len(item.mutagen):
            return False
        star_name = item.mutagen[idx]
        return any(
            s.name == star_name
            for s in p.major_stars + p.minor_stars
        )

    def has_horoscope_stars(
        self,
        name: str,
        scope: ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe,
    ) -> bool:
        """判断指定运限宫位是否包含指定的所有流耀"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        keys = self._collect_horoscope_star_names(p.index)
        return all(s in keys for s in stars)

    def not_have_horoscope_stars(
        self,
        name: str,
        scope: ScopeLiteral,
        stars: list[str],
        astrolabe: Astrolabe,
    ) -> bool:
        """判断指定运限宫位是否不包含指定的所有流耀"""
        p = self.palace(name, scope, astrolabe)
        if p is None:
            return False
        keys = self._collect_horoscope_star_names(p.index)
        return all(s not in keys for s in stars)

    def _collect_horoscope_star_names(self, palace_idx: int) -> set[str]:
        """收集大限和流年在指定宫位的所有流耀名"""
        names: set[str] = set()
        if self.decadal.stars and palace_idx < len(self.decadal.stars):
            names.update(s.name for s in self.decadal.stars[palace_idx])
        if self.yearly.stars and palace_idx < len(self.yearly.stars):
            names.update(s.name for s in self.yearly.stars[palace_idx])
        return names

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
    """性别"""

    solar_date: str
    """阳历日期"""

    lunar_date: str
    """农历日期"""

    chinese_date: str
    """干支纪年日期"""

    time: str
    """时辰"""

    time_range: str
    """时辰对应的时间段"""

    sign: str
    """星座"""

    zodiac: str
    """生肖"""

    earthly_branch_of_soul_palace: str
    """命宫地支"""

    earthly_branch_of_body_palace: str
    """身宫地支"""

    soul: str
    """命主星"""

    body: str
    """身主星"""

    five_elements_class: str
    """五行局"""

    palaces: list[Palace]
    """十二宫数据"""

    raw_dates: RawDates
    """结构化的农历生日与四柱干支"""

    gender_key: str
    """机器可读性别：male | female"""

    time_index: int
    """出生时辰索引 (0-12)"""

    fix_leap: bool
    """是否修正闰月"""

    language: str
    """排盘语言（"zh_cn" 等）"""

    config: ChartConfig
    """排盘配置"""

    # ------ 宫位查询 ------

    def palace(self, index_or_name: int | str) -> Palace | None:
        """
        通过索引或名称获取宫位。

        Args:
            index_or_name: 宫位索引 (0-11) 或名称（如 "命宫"）
        """
        if isinstance(index_or_name, int):
            if 0 <= index_or_name < len(self.palaces):
                return self.palaces[index_or_name]
            return None
        for p in self.palaces:
            if p.name == index_or_name:
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

    def star(self, star_name: str) -> Star | None:
        """通过名称查找星耀（遍历所有宫位）"""
        for p in self.palaces:
            for s in p.major_stars + p.minor_stars + p.adjective_stars:
                if s.name == star_name:
                    return s
        return None

    def star_in_palace(self, star_name: str) -> tuple[Star, Palace] | None:
        """
        通过名称查找星耀及其所在宫位。

        Returns:
            (Star, Palace) 元组，未找到时返回 None
        """
        for p in self.palaces:
            for s in p.major_stars + p.minor_stars + p.adjective_stars:
                if s.name == star_name:
                    return (s, p)
        return None

    @classmethod
    def _from_dict(cls, d: dict) -> Astrolabe:
        return cls(
            gender=d["gender"],
            solar_date=d["solarDate"],
            lunar_date=d["lunarDate"],
            chinese_date=d["chineseDate"],
            time=d["time"],
            time_range=d["timeRange"],
            sign=d["sign"],
            zodiac=d["zodiac"],
            earthly_branch_of_soul_palace=d["earthlyBranchOfSoulPalace"],
            earthly_branch_of_body_palace=d["earthlyBranchOfBodyPalace"],
            soul=d["soul"],
            body=d["body"],
            five_elements_class=d["fiveElementsClass"],
            palaces=[Palace._from_dict(p) for p in d["palaces"]],
            raw_dates=RawDates._from_dict(d["rawDates"]),
            gender_key=d["genderKey"],
            time_index=d["timeIndex"],
            fix_leap=d["fixLeap"],
            language=d["language"],
            config=ChartConfig._from_dict(d["config"]),
        )

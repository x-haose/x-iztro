"""
x-iztro: 紫微斗数 Rust 核心库 Python 绑定

使用方式::

    from x_iztro import Astro
    from x_iztro.enums import PalaceName, Mutagen, MajorStar

    astro = Astro()
    result = astro.by_solar("2000-8-16", 2, "female")

    # 属性访问 + IDE 自动补全
    print(result.solar_date)
    print(result.palaces[0].name)

    # 宫位查询
    soul = result.palace(PalaceName.SOUL)
    soul.has([MajorStar.ZIWEI])
    soul.has_mutagen(Mutagen.LU)

    # 三方四正
    sp = result.surrounded_palaces(soul.index)
    sp.have_mutagen(Mutagen.JI)

    # 运限
    h = astro.get_horoscope(result, "2024-1-1", 0)
    print(h.yearly.name)
"""

# 数据模型
from x_iztro.models import (
    Astrolabe,
    Palace,
    Star,
    Decadal,
    SurroundedPalaces,
    Horoscope,
    HoroscopeItem,
    HoroscopeYearly,
    AgeItem,
    YearlyDecStar,
    RawDates,
    RawLunarDate,
    RawChineseDate,
    ChartConfig,
)

# 枚举与常量
from x_iztro.enums import (
    Gender,
    Language,
    Algorithm,
    YearDivide,
    HoroscopeDivide,
    AgeDivide,
    DayDivide,
    HeavenlyStem,
    EarthlyBranch,
    PalaceName,
    FiveElementsClass,
    Mutagen,
    Brightness,
    StarType,
    Scope,
    MajorStar,
    MinorStar,
    AdjectiveStar,
    Changsheng12,
    Boshi12,
    Suiqian12,
    Jiangqian12,
)

# 主类
from x_iztro.astro import Astro

__all__ = [
    # 主类
    "Astro",
    # 数据模型
    "Astrolabe",
    "Palace",
    "Star",
    "Decadal",
    "SurroundedPalaces",
    "Horoscope",
    "HoroscopeItem",
    "HoroscopeYearly",
    "AgeItem",
    "YearlyDecStar",
    "RawDates",
    "RawLunarDate",
    "RawChineseDate",
    "ChartConfig",
    # 枚举
    "Gender",
    "Language",
    "Algorithm",
    "YearDivide",
    "HoroscopeDivide",
    "AgeDivide",
    "DayDivide",
    "HeavenlyStem",
    "EarthlyBranch",
    "PalaceName",
    "FiveElementsClass",
    "Mutagen",
    "Brightness",
    "StarType",
    "Scope",
    "MajorStar",
    "MinorStar",
    "AdjectiveStar",
    "Changsheng12",
    "Boshi12",
    "Suiqian12",
    "Jiangqian12",
]

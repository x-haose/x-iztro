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

    # 导出与 JS iztro 一致的 JSON
    print(result.to_json(indent=2))

    # 入参非法时抛 IztroError（继承 ValueError），code 为机器可读分类
    try:
        astro.by_solar("not-a-date", 2, "female")
    except IztroError as e:
        print(e.code)          # invalid_date
"""

# 原生扩展模块抛出的异常类型
from x_iztro._x_iztro import IztroError

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
    PatternConfig,
    PatternHit,
    PatternStar,
)

# 枚举与常量
from x_iztro.enums import (
    Gender,
    Language,
    Algorithm,
    AstroType,
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
    HoroscopeStar,
    PatternKey,
    BrightnessSource,
)

# 主类
from x_iztro import data, i18n, plugin, query, star, utils
from x_iztro.astro import Astro

# 轻量查询
from x_iztro.query import (
    get_zodiac_by_solar_date,
    get_sign_by_solar_date,
    get_sign_by_lunar_date,
    get_major_star_by_solar_date,
    get_major_star_by_lunar_date,
)

__all__ = [
    # 主类
    "Astro",
    # 异常
    "IztroError",
    # 轻量查询
    "get_zodiac_by_solar_date",
    "get_sign_by_solar_date",
    "get_sign_by_lunar_date",
    "get_major_star_by_solar_date",
    "get_major_star_by_lunar_date",
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
    "PatternConfig",
    "PatternHit",
    "PatternStar",
    # 枚举
    "Gender",
    "Language",
    "Algorithm",
    "AstroType",
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
    "HoroscopeStar",
    "PatternKey",
    "BrightnessSource",
    # 子模块：安星、数据表、翻译
    "star",
    "data",
    "i18n",
    "utils",
    "query",
    "plugin",
]

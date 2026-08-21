"""
不需要完整星盘的轻量查询：生肖、星座、命宫主星。

与 `Astro` 的排盘方法共用同一套核心逻辑，因此结果与完整排盘的对应字段永远一致。
"""

from __future__ import annotations

from x_iztro._bridge import query
from x_iztro.models import ChartConfig, LanguageType, TimeIndexType


def _config(config: ChartConfig | None) -> dict | None:
    """排盘配置转为绑定层入参。"""
    return config.to_dict() if config is not None else None


def get_zodiac_by_solar_date(
    solar_date: str,
    language: LanguageType = "zh-CN",
    config: ChartConfig | None = None,
) -> str:
    """
    通过阳历日期取生肖。

    生肖由年支决定，换年时点受 `ChartConfig.year_divide` 影响 ——
    正月初一与立春之间的生日会随配置得到不同结果。

    Args:
        solar_date: 阳历日期，如 "2000-8-16"
        language: 输出语言
        config: 排盘配置

    Raises:
        IztroError: 日期非法
    """
    return query(
        "zodiacBySolar", solar_date=solar_date, language=language, config=_config(config)
    )["text"]


def get_sign_by_solar_date(
    solar_date: str,
    language: LanguageType = "zh-CN",
) -> str:
    """
    通过阳历日期取星座。

    星座只由公历日期决定，与配置和时辰无关。

    Raises:
        IztroError: 日期非法
    """
    return query("signBySolar", solar_date=solar_date, language=language)["text"]


def get_sign_by_lunar_date(
    lunar_date: str,
    is_leap_month: bool = False,
    language: LanguageType = "zh-CN",
) -> str:
    """
    通过农历日期取星座。

    Args:
        lunar_date: 农历日期，如 "2000-7-17"
        is_leap_month: 是否闰月（该月无闰月时不生效）
        language: 输出语言

    Raises:
        IztroError: 日期非法
    """
    return query(
        "signByLunar",
        lunar_date=lunar_date,
        is_leap_month=is_leap_month,
        language=language,
    )["text"]


def get_major_star_by_solar_date(
    solar_date: str,
    time_index: TimeIndexType,
    *,
    fix_leap: bool = True,
    language: LanguageType = "zh-CN",
    config: ChartConfig | None = None,
) -> str:
    """
    通过阳历日期取命宫主星，多颗以逗号分隔。

    `time_index` 之后的参数只能按关键字传入。

    命宫为空宫时借对宫主星，与 iztro 行为一致。

    Raises:
        IztroError: 日期或时辰索引非法
    """
    return query(
        "majorStarBySolar",
        solar_date=solar_date,
        time_index=time_index,
        fix_leap=fix_leap,
        language=language,
        config=_config(config),
    )["text"]


def get_major_star_keys_by_solar_date(
    solar_date: str,
    time_index: TimeIndexType,
    *,
    fix_leap: bool = True,
    config: ChartConfig | None = None,
) -> list[str]:
    """
    通过阳历日期取命宫主星的语言无关标识列表（`MajorStar` 枚举值域）。

    与 `get_major_star_by_solar_date` 同一结果的标识投影：
    命宫为空宫时同样借对宫主星。标识与输出语言无关，故不收 language
    （绑定层要求给定语言，内部固定传 zh-CN，不影响 keys 取值）。

    Raises:
        IztroError: 日期或时辰索引非法
    """
    return query(
        "majorStarBySolar",
        solar_date=solar_date,
        time_index=time_index,
        fix_leap=fix_leap,
        language="zh-CN",
        config=_config(config),
    )["keys"]


def get_major_star_by_lunar_date(
    lunar_date: str,
    time_index: TimeIndexType,
    *,
    is_leap_month: bool = False,
    fix_leap: bool = True,
    language: LanguageType = "zh-CN",
    config: ChartConfig | None = None,
) -> str:
    """
    通过农历日期取命宫主星，多颗以逗号分隔。

    `time_index` 之后的参数只能按关键字传入（两个相邻布尔位置传参易写反）。

    命宫为空宫时借对宫主星；`is_leap_month` 在该月没有闰月时不生效。

    Raises:
        IztroError: 日期或时辰索引非法
    """
    return query(
        "majorStarByLunar",
        lunar_date=lunar_date,
        time_index=time_index,
        is_leap_month=is_leap_month,
        fix_leap=fix_leap,
        language=language,
        config=_config(config),
    )["text"]


def get_major_star_keys_by_lunar_date(
    lunar_date: str,
    time_index: TimeIndexType,
    *,
    is_leap_month: bool = False,
    fix_leap: bool = True,
    config: ChartConfig | None = None,
) -> list[str]:
    """
    通过农历日期取命宫主星的语言无关标识列表（`MajorStar` 枚举值域）。

    与 `get_major_star_by_lunar_date` 同一结果的标识投影：
    命宫为空宫时同样借对宫主星；`is_leap_month` 在该月没有闰月时不生效。
    标识与输出语言无关，故不收 language
    （绑定层要求给定语言，内部固定传 zh-CN，不影响 keys 取值）。

    Raises:
        IztroError: 日期或时辰索引非法
    """
    return query(
        "majorStarByLunar",
        lunar_date=lunar_date,
        time_index=time_index,
        is_leap_month=is_leap_month,
        fix_leap=fix_leap,
        language="zh-CN",
        config=_config(config),
    )["keys"]

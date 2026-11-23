"""
历法查询：农历年的月份表、公历与农历互换、农历年的年干支。

与排盘同一份历法内核，结果与星盘的 `raw_dates.lunar_date`、年柱逐日一致。支持公历与农历
1-9999 年；1582-10-15 以前按儒略历，1582-10-05 至 10-14 不存在。

报错与排盘入口一致：给出完整日期的换算在日期不存在时抛 `IztroError`（code 为 `invalid_date`），
只给年份的查询在年份越界时抛 `IztroError`（code 为 `invalid_argument`）。
"""

from __future__ import annotations

from dataclasses import dataclass

from x_iztro._bridge import query
from x_iztro.astrolabe import RawLunarDate
from x_iztro.enums import EarthlyBranch, HeavenlyStem


@dataclass(frozen=True, slots=True)
class RawSolarDate:
    """数字化公历日期；1582-10-15 以前为儒略历日期"""

    solar_year: int
    """公历年；农历 9999 年最后一两个月落在公历 10000 年"""

    solar_month: int
    """公历月（1-12）"""

    solar_day: int
    """公历日（1-31）"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawSolarDate:
        return cls(
            solar_year=d["solarYear"], solar_month=d["solarMonth"], solar_day=d["solarDay"]
        )


@dataclass(frozen=True, slots=True)
class LunarMonthInfo:
    """农历年里的一个月"""

    month: int
    """月份（1-12）；闰月取所闰的月份，如闰四月为 4"""

    is_leap: bool
    """是否闰月"""

    day_count: int
    """天数：29（小月）或 30（大月）"""

    first_day: RawSolarDate
    """初一对应的公历日期"""

    @classmethod
    def _from_dict(cls, d: dict) -> LunarMonthInfo:
        return cls(
            month=d["month"],
            is_leap=d["isLeap"],
            day_count=d["dayCount"],
            first_day=RawSolarDate._from_dict(d["firstDay"]),
        )


def lunar_months(year: int) -> list[LunarMonthInfo]:
    """
    农历 `year` 年的全部月份，按时间先后排列；闰月紧跟在所闰的月之后。

    一般为 12 个月，有闰月的年为 13 个月。个别古代年份例外，与 lunar-typescript 相同：
    如农历 237 年只有 11 个月、没有三月。

    Raises:
        IztroError: `year` 不在 1-9999（invalid_argument）
    """
    return [LunarMonthInfo._from_dict(m) for m in query("lunarMonths", year=year)]


def solar_to_lunar(year: int, month: int, day: int) -> RawLunarDate:
    """
    公历日期对应的农历日期。

    公历 1 年 1 月初的几十天属农历 0 年（冬月、腊月），此时 `lunar_year` 为 0。

    Raises:
        IztroError: 年不在 1-9999、月日不存在，或落在改历空缺 1582-10-05 至 10-14（invalid_date）
    """
    return RawLunarDate._from_dict(query("solarToLunar", year=year, month=month, day=day))


def lunar_to_solar(year: int, month: int, day: int, *, is_leap: bool = False) -> RawSolarDate:
    """
    农历日期对应的公历日期。

    `is_leap` 只表示「这一天在闰月里」，只能按关键字传入。排盘入口的 `fix_leap`（闰月十五之后视作次月）
    管的是星曜按哪个月安，不改变日期本身，这里没有对应参数。该年没有这个闰月时报错，
    不退回同号的常规月。

    Raises:
        IztroError: 年不在 1-9999、该年没有这个月（含不闰此月）、日超出当月天数（invalid_date）
    """
    return RawSolarDate._from_dict(
        query("lunarToSolar", year=year, lunar_month=month, lunar_day=day, is_leap=is_leap)
    )


def lunar_year_pillar(year: int) -> tuple[HeavenlyStem, EarthlyBranch]:
    """
    农历 `year` 年的年干支（公历 4 年为甲子），返回 (天干, 地支) 标识。

    年干支按年号循环，与分界口径无关；口径决定这一柱从哪天起算：正月初一分界（默认）为农历
    `year` 年正月初一至除夕，立春分界为公历 `year` 年立春当日至次年立春前一日。

    Raises:
        IztroError: `year` 不在 1-9999（invalid_argument）
    """
    stem, branch = query("lunarYearPillar", year=year)
    return HeavenlyStem(stem), EarthlyBranch(branch)

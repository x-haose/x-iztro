"""
排盘算法的公开工具函数。

参数与返回值中的枚举一律使用语言无关 key（见 `x_iztro.enums`），
因此这些函数与输出语言无关，可直接与星盘对象的 `*_key` 字段互操作。
"""

from __future__ import annotations

from dataclasses import dataclass

from x_iztro.enums import (
    Brightness,
    EarthlyBranch,
    HeavenlyStem,
    Mutagen,
    PalaceName,
)
from x_iztro.models import ChartConfig, Decadal, Star

from x_iztro._bridge import query, typed


def _config(config: ChartConfig | None) -> dict | None:
    """排盘配置转为绑定层入参。"""
    return config.to_dict() if config is not None else None


def fix_index(index: int, max: int = 12) -> int:
    """把索引约束到 0..max 的循环区间；负数也能正确回绕。"""
    return query("fixIndex", index=index, max=max)


def earthly_branch_to_palace_index(branch: EarthlyBranch | str) -> int:
    """地支标识转宫位索引（寅宫为 0）。"""
    return query("earthlyBranchToPalaceIndex", branch_key=branch)


def time_to_index(hour: int) -> int:
    """小时（0-23）转时辰索引：0 为早子时，12 为晚子时。"""
    return query("timeToIndex", hour=hour)


def get_age_index(branch: EarthlyBranch | str) -> int:
    """由生年地支取小限起始宫位索引。"""
    return query("getAgeIndex", branch_key=branch)


def get_brightness(
    star: str,
    palace_index: int,
    config: ChartConfig | None = None,
) -> Brightness | None:
    """
    星耀落在指定宫位时的亮度标识；该星没有亮度表时返回 None。

    Args:
        star: 星耀标识（`MajorStar` / `MinorStar` 等枚举值域）
        palace_index: 宫位索引，越界会对 12 取模
    """
    value = query("getBrightness", star_key=star, index=palace_index, config=_config(config))
    return Brightness(value) if value is not None else None


def get_mutagen(
    star: str,
    stem: HeavenlyStem | str,
    config: ChartConfig | None = None,
) -> Mutagen | None:
    """指定天干下某颗星化什么；不在该天干四化表内时返回 None。"""
    value = query("getMutagen", star_key=star, stem_key=stem, config=_config(config))
    return Mutagen(value) if value is not None else None


def get_mutagens_by_heavenly_stem(
    stem: HeavenlyStem | str,
    config: ChartConfig | None = None,
) -> list[str]:
    """指定天干化出的四颗星，顺序为禄、权、科、忌。"""
    return query("getMutagensByHeavenlyStem", stem_key=stem, config=_config(config))


@dataclass(frozen=True, slots=True)
class SoulAndBody:
    """命宫与身宫的落点。"""

    soul_index: int
    """命宫的宫位索引"""

    body_index: int
    """身宫的宫位索引"""

    heavenly_stem_of_soul: str
    """命宫天干标识（`HeavenlyStem` 枚举值域）"""

    earthly_branch_of_soul: str
    """命宫地支标识（`EarthlyBranch` 枚举值域）"""


@dataclass(frozen=True, slots=True)
class DecadalsAndAges:
    """十二宫的大限与小限，两组都按宫位索引排列。"""

    decadals: list[Decadal]
    """每宫的大限：岁数区间与该宫干支"""

    ages: list[list[int]]
    """每宫的小限岁数列表"""


def get_soul_and_body(
    month_index: int,
    time_index: int,
    yearly_stem: HeavenlyStem | str,
) -> SoulAndBody:
    """
    由农历月索引、时辰索引与年干推命宫与身宫。

    Args:
        month_index: 农历月索引（正月为 0），可用 `fix_lunar_month_index` 求得
        time_index: 时辰索引 0-12
        yearly_stem: 年干标识

    Returns:
        SoulAndBody：两宫的宫位索引与命宫干支标识
    """
    return typed(
        SoulAndBody,
        query(
            "getSoulAndBody",
            month_index=month_index,
            time_index=time_index,
            stem_key=yearly_stem,
        ),
    )


def get_five_elements_class(
    stem: HeavenlyStem | str,
    branch: EarthlyBranch | str,
) -> str:
    """由命宫干支推五行局标识（`FiveElementsClass` 枚举值域）。"""
    return query("getFiveElementsClass", stem_key=stem, branch_key=branch)


def get_palace_names(soul_index: int) -> list[PalaceName]:
    """
    由命宫索引推十二宫名标识，按宫位索引排列。

    返回值的第 i 项即 `astrolabe.palaces[i]` 的宫名。
    """
    return [PalaceName(name) for name in query("getPalaceNames", soul_index=soul_index)]


def get_decadals_and_ages(
    soul_index: int,
    five_elements_class: str,
    gender: str,
    yearly_stem: HeavenlyStem | str,
    yearly_branch: EarthlyBranch | str,
) -> DecadalsAndAges:
    """
    由命宫索引与五行局推十二宫的大限与小限。

    大限起运岁数由五行局决定，顺逆由性别阴阳与年支阴阳决定；
    小限起宫由年支决定。

    Args:
        soul_index: 命宫宫位索引
        five_elements_class: 五行局标识（`FiveElementsClass` 枚举值域）
        gender: "male" 或 "female"
        yearly_stem: 年干标识
        yearly_branch: 年支标识

    Returns:
        DecadalsAndAges：每宫的大限与小限，均按宫位索引排列
    """
    result = query(
        "getHoroscope",
        soul_index=soul_index,
        five_elements_class=five_elements_class,
        gender=gender,
        stem_key=yearly_stem,
        branch_key=yearly_branch,
    )
    return DecadalsAndAges(
        decadals=[Decadal._from_dict(d) for d in result["decadals"]],
        ages=[list(ages) for ages in result["ages"]],
    )


def fix_lunar_month_index(
    lunar_month: int,
    lunar_day: int,
    is_leap: bool,
    time_index: int,
    fix_leap: bool,
) -> int:
    """
    修正后的农历月索引（正月为 0）。

    修正闰月时，闰月十五日之后按下月算；晚子时（索引 12）不参与修正。
    """
    return query(
        "fixLunarMonthIndex",
        lunar_month=lunar_month,
        lunar_day=lunar_day,
        is_leap=is_leap,
        time_index=time_index,
        fix_leap=fix_leap,
    )


def fix_lunar_day_index(lunar_day: int, time_index: int) -> int:
    """修正后的农历日索引；晚子时属次日，因此不减一。"""
    return query("fixLunarDayIndex", lunar_day=lunar_day, time_index=time_index)


def translate_chinese_date(
    pillars: list[tuple[str, str]],
    language: str = "zh-CN",
) -> str:
    """
    按语言拼接四柱干支展示串（星盘 `chinese_date` 字段即由此生成）。

    Args:
        pillars: 四柱标识 [年, 月, 日, 时]，每柱为 (天干, 地支)
        language: 输出语言

    Returns:
        词条均为单字符时柱内紧凑相连、柱间空格（如「庚辰 甲申 丁未 庚子」）；
        任一词条为多字符时柱内空格、柱间「 - 」。

    Raises:
        IztroError: 柱数不为四、某柱不是两项，或干支标识非法
    """
    return query(
        "translateChineseDate",
        pillars=[[str(s), str(b)] for s, b in pillars],
        language=language,
    )


def merge_stars(*groups: list[list[Star]]) -> list[list[Star]]:
    """
    把多组「十二宫星耀」按宫位合并成一组。

    安星是分批进行的（主星、辅星、杂耀各出一组十二宫列表），
    本函数按宫位索引把它们首尾相接，顺序为传入顺序。

    Args:
        groups: 若干组十二宫星耀列表，每组长度须为 12

    Raises:
        ValueError: 某一组的长度不是 12
    """
    merged: list[list[Star]] = [[] for _ in range(12)]
    for group in groups:
        if len(group) != 12:
            raise ValueError(
                f"每组星耀须为十二宫，实际 {len(group)} 项"
            )
        for i, stars in enumerate(group):
            merged[i].extend(stars)
    return merged

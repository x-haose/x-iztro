"""
排盘算法用到的查表与顺序常量。

取值一律为语言无关标识，可直接同星盘对象的 `*_key` 字段比对。
表是排盘的输入而非输出，因此与输出语言无关。
"""

from __future__ import annotations

from typing import Any

from x_iztro._bridge import query


def stars_info() -> dict[str, dict[str, Any]]:
    """
    星耀基础信息：十四主星与文昌、文曲、火星、铃星、擎羊、陀罗共二十颗。

    Returns:
        星耀标识 → {brightness: 十二宫亮度标识（无亮度的宫为 None）,
        fiveElements: 五行, yinYang: 阴阳}。五行与阴阳在原表中部分未填，为 None。
    """
    return query("starsInfo")


def heavenly_stems() -> dict[str, dict[str, Any]]:
    """
    天干信息表。

    Returns:
        天干标识 → {yinYang: 阴阳, fiveElements: 五行, crash: 对冲天干标识
        （戊己无对冲，为 None）, mutagen: 四化四星标识，顺序为禄权科忌}
    """
    return query("heavenlyStems")


def earthly_branches() -> dict[str, dict[str, Any]]:
    """
    地支信息表。

    Returns:
        地支标识 → {yinYang: 阴阳, fiveElements: 五行, crash: 对冲地支标识,
        soul: 命主星标识, body: 身主星标识, inside: 对应脏腑,
        outside: 对应身体部位, healthTip: 健康提示}。
        后三项只有中文一种写法，不参与国际化。
    """
    return query("earthlyBranches")


def constants() -> dict[str, Any]:
    """
    顺序常量与推算规则表。

    Returns:
        LANGUAGES 支持的语言代码、HEAVENLY_STEMS 天干顺序、
        EARTHLY_BRANCHES 地支顺序、ZODIAC 生肖、SIGNS 星座、PALACES 十二宫名、
        GENDER 男女各自的阴阳、CHINESE_TIME 时辰标识、TIME_RANGE 时辰区间、
        TIGER_RULE 五虎遁（年干推正月干）、RAT_RULE 五鼠遁（日干推子时干）、
        MUTAGEN 四化顺序
    """
    return query("constants")

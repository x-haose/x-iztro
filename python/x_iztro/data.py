"""
排盘算法用到的查表与顺序常量。

取值一律为语言无关标识，可直接同星盘对象的 `*_key` 字段比对。
表是排盘的输入而非输出，因此与输出语言无关。
"""

from __future__ import annotations

from dataclasses import dataclass

from x_iztro._bridge import query, typed


@dataclass(frozen=True, slots=True)
class StarInfo:
    """星耀基础信息。"""

    brightness: list[str | None]
    """十二宫亮度标识（`Brightness` 枚举值域），索引 0 为寅宫；该宫无亮度时为 None"""

    five_elements: str | None
    """五行；原表中太阳、七杀与六颗辅星未填，为 None"""

    yin_yang: str | None
    """阴阳；原表中部分星耀未填，为 None"""


@dataclass(frozen=True, slots=True)
class HeavenlyStemInfo:
    """天干信息。"""

    yin_yang: str
    """阴阳"""

    five_elements: str
    """五行"""

    crash: str | None
    """对冲天干标识；戊己无对冲，为 None"""

    mutagen: list[str]
    """四化四星标识，顺序为禄、权、科、忌"""


@dataclass(frozen=True, slots=True)
class EarthlyBranchInfo:
    """地支信息。"""

    yin_yang: str
    """阴阳"""

    five_elements: str
    """五行"""

    crash: str
    """对冲地支标识"""

    soul: str
    """命主星标识"""

    body: str
    """身主星标识"""

    inside: str
    """对应脏腑；只有中文一种写法，不参与国际化"""

    outside: str
    """对应身体部位；只有中文一种写法，不参与国际化"""

    health_tip: str
    """健康提示；只有中文一种写法，不参与国际化"""


@dataclass(frozen=True, slots=True)
class Constants:
    """顺序常量与推算规则表。"""

    languages: list[str]
    """支持的语言代码"""

    heavenly_stems: list[str]
    """天干顺序"""

    earthly_branches: list[str]
    """地支顺序"""

    zodiac: list[str]
    """生肖标识，按地支顺序"""

    signs: list[str]
    """星座标识，按黄道顺序"""

    palaces: list[str]
    """十二宫名，按命宫起的顺序"""

    gender: dict[str, str]
    """男女各自的阴阳"""

    chinese_time: list[str]
    """时辰标识，索引 0-12"""

    time_range: list[str]
    """时辰对应的时间区间"""

    tiger_rule: dict[str, str]
    """五虎遁：年干推正月天干"""

    rat_rule: dict[str, str]
    """五鼠遁：日干推子时天干"""

    mutagen: list[str]
    """四化顺序：禄、权、科、忌"""

    five_elements_class: dict[str, int]
    """五行局标识对应的局数 2-6，与 `FiveElementsClass.number` 同源"""


def stars_info() -> dict[str, StarInfo]:
    """
    星耀基础信息：十四主星与文昌、文曲、火星、铃星、擎羊、陀罗共二十颗。

    Returns:
        星耀标识 → StarInfo
    """
    return {key: typed(StarInfo, value) for key, value in query("starsInfo").items()}


def heavenly_stems() -> dict[str, HeavenlyStemInfo]:
    """
    天干信息表。

    Returns:
        天干标识 → HeavenlyStemInfo
    """
    return {
        key: typed(HeavenlyStemInfo, value) for key, value in query("heavenlyStems").items()
    }


def earthly_branches() -> dict[str, EarthlyBranchInfo]:
    """
    地支信息表。

    Returns:
        地支标识 → EarthlyBranchInfo
    """
    return {
        key: typed(EarthlyBranchInfo, value) for key, value in query("earthlyBranches").items()
    }


def constants() -> Constants:
    """
    顺序常量与推算规则表。

    Returns:
        Constants：语言、干支、生肖、星座、十二宫、性别阴阳、时辰、
        五虎遁、五鼠遁、四化顺序与五行局局数
    """
    raw = query("constants")
    return Constants(
        languages=raw["LANGUAGES"],
        heavenly_stems=raw["HEAVENLY_STEMS"],
        earthly_branches=raw["EARTHLY_BRANCHES"],
        zodiac=raw["ZODIAC"],
        signs=raw["SIGNS"],
        palaces=raw["PALACES"],
        gender=raw["GENDER"],
        chinese_time=raw["CHINESE_TIME"],
        time_range=raw["TIME_RANGE"],
        tiger_rule=raw["TIGER_RULE"],
        rat_rule=raw["RAT_RULE"],
        mutagen=raw["MUTAGEN"],
        five_elements_class=raw["FIVE_ELEMENTS_CLASS"],
    )

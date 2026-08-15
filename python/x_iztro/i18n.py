"""
标识与译名的双向查找。

星盘对象的每个字段都同时给出译名与 `*_key` 标识，通常不必再手工翻译；
这两个函数用于手上只有标识（或只有某种语言的译名）、需要换算的场合。

覆盖星耀、宫位（含身宫、来因宫）、天干、地支、亮度、四化、五行局、性别、
生肖、时辰、星座、运限层级共十一类，合计 260 个标识。
"""

from __future__ import annotations

from x_iztro._bridge import query
from x_iztro.models import LanguageType


def translate(key: str, language: LanguageType = "zh-CN") -> str | None:
    """
    标识译成指定语言的文本；未知标识返回 None。

    Args:
        key: 语言无关标识，如 "ziweiMaj"、"soulPalace"、"jiaHeavenly"
        language: 输出语言
    """
    return query("translate", key=key, language=language)


def key_of(text: str, key_filter: str | None = None) -> str | None:
    """
    由任意语言的译名反查标识；查不到返回 None。

    逐语言、每种语言内逐标识比对取先命中者，顺序与 iztro 的 `kot` 一致。

    同一译名在多个类目下同形时（en-US 的 "horse" 既是生肖马也是天马），
    用 `key_filter` 限定标识名须含的子串来消歧：`"Maj"` 只看十四主星、
    `"Min"` 只看辅星、`"Heavenly"` / `"Earthly"` 只看干支、`"Palace"` 只看宫位、
    `"Hour"` 只看时辰。限定后无匹配返回 None，不退回未限定的结果。

    Args:
        text: 任一支持语言下的译名，如 "紫微"、"emperor"、"자미"
        key_filter: 标识名须含的子串；None 表示不限定
    """
    return query("keyOf", text=text, key_filter=key_filter)


def all_keys() -> list[str]:
    """
    全部 260 个可翻译标识。

    顺序即 `key_of` 的反查次序，与 iztro 各语言翻译文件的合并次序一致：
    运限层级、生肖、时辰、星座、五行局、天干、地支、亮度、四化、星耀、宫位、性别。
    """
    return query("allKeys")

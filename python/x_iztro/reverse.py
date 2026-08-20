"""
反推：由八字四柱或星盘特征反查候选生辰。

计算全部在 Rust 内核完成（剪枝枚举 + 正排终验，与正向排盘零分歧），
这里只做类型化封装。星盘布局与性别无关，反推的目标是生辰，故不收性别。
"""

from __future__ import annotations

from dataclasses import dataclass, field
from typing import Any

from x_iztro.config import ChartConfig
from x_iztro.enums import EarthlyBranch, FiveElementsClass, HeavenlyStem

Pillar = tuple[HeavenlyStem | str, EarthlyBranch | str]
"""一柱干支：(天干标识, 地支标识)，如 `(HeavenlyStem.GENG, EarthlyBranch.CHEN)`"""


@dataclass(frozen=True, slots=True)
class BirthCandidate:
    """一个候选生辰，可直接交给 `Astro.by_solar` 排盘"""

    solar_date: str
    """公历日期，`YYYY-M-D`"""

    time_index: int
    """时辰索引 0-12（0 为早子时，12 为晚子时）"""

    @classmethod
    def _from_dict(cls, d: dict) -> BirthCandidate:
        return cls(solar_date=d["solarDate"], time_index=d["timeIndex"])


@dataclass(frozen=True, slots=True)
class StarPosition:
    """一颗星与其落宫地支：星盘特征反推的原子条件"""

    star: str
    """星耀标识（须为本命盘星耀，流耀不接受）"""

    branch: EarthlyBranch | str
    """落宫地支标识"""


@dataclass(frozen=True, slots=True)
class ReverseCriteria:
    """
    星盘特征反推的条件集。

    全部字段可选，但至少要给一个条件；条件越具体（尤其命宫地支、生年四化、
    主星落宫），反推越快。
    """

    soul_branch: EarthlyBranch | str | None = None
    """命宫地支"""

    body_branch: EarthlyBranch | str | None = None
    """身宫地支"""

    five_elements_class: FiveElementsClass | str | None = None
    """五行局"""

    stars: list[StarPosition] = field(default_factory=list)
    """星耀落宫条件，全部须同时满足"""

    mutagens: tuple[str | None, str | None, str | None, str | None] = (None, None, None, None)
    """生年四化 [禄, 权, 科, 忌] 各自是哪颗星（星耀标识），可只给其中几个"""

    year_range: tuple[int, int] = (1900, 2100)
    """公历年闭区间（含两端），须落在 1583-9999 内"""

    fix_leap: bool = True
    """是否修正闰月（与排盘入参同义）"""

    limit: int = 0
    """候选数上限：达到即停止搜索并置 `ReverseResult.truncated`；0 取内核默认（512）"""

    def _payload(self) -> dict[str, Any]:
        return {
            "soulBranch": _key(self.soul_branch),
            "bodyBranch": _key(self.body_branch),
            "fiveElementsClass": _key(self.five_elements_class),
            "stars": [{"star": str(p.star), "branch": str(p.branch)} for p in self.stars],
            "mutagens": [_key(m) for m in self.mutagens],
            "yearRange": list(self.year_range),
            "fixLeap": self.fix_leap,
            "limit": self.limit,
        }


@dataclass(frozen=True, slots=True)
class ReverseResult:
    """星盘特征反推的结果"""

    candidates: list[BirthCandidate]
    """满足全部条件的候选生辰，按枚举序排列：农历年升序，年内依 月→时辰→日；
    同一年内不保证公历日期升序"""

    truncated: bool
    """是否因达到候选数上限而提前截断；截断时枚举序更靠后的解未被搜索，
    其中可能包含公历日期更早的解"""


def _key(v: object | None) -> str | None:
    return None if v is None else str(v)


def solar_dates_by_bazi(
    yearly: Pillar,
    monthly: Pillar,
    daily: Pillar,
    hourly: Pillar,
    *,
    year_range: tuple[int, int] = (1900, 2100),
    config: ChartConfig | None = None,
) -> list[BirthCandidate]:
    """
    由八字四柱反查公历生辰。

    四柱按 `config` 的分界口径解释（年柱 `year_divide`、月柱 `horoscope_divide`、
    晚子归属 `day_divide`），与星盘 `raw_dates.chinese_date` 同一套语义。
    一组四柱在范围内通常每约 60 年出现一次；子时因早晚子之分可能给出两个候选。

    Args:
        yearly/monthly/daily/hourly: 四柱干支，各为 (天干标识, 地支标识)
        year_range: 公历年闭区间（含两端），须落在 1583-9999 内
        config: 排盘配置（`ChartConfig`）；不传取默认

    Returns:
        候选生辰列表（公历日期串；子时因早晚子之分可能给出两个候选）

    Raises:
        IztroError: 干支阴阳不配、年份范围非法
    """
    from x_iztro._bridge import query

    data = query(
        "solarDatesByBazi",
        pillars=[[str(s), str(b)] for (s, b) in (yearly, monthly, daily, hourly)],
        start_year=year_range[0],
        end_year=year_range[1],
        config=config.to_dict() if config is not None else None,
    )
    return [BirthCandidate._from_dict(d) for d in data]


def reverse_chart(
    criteria: ReverseCriteria,
    config: ChartConfig | None = None,
) -> ReverseResult:
    """
    由星盘特征反查候选生辰。

    排盘配置贯穿判定（四化表、派别、分界都按它算），候选用同一配置排盘必满足全部条件。

    Args:
        criteria: 条件集，至少要给一个条件
        config: 排盘配置（`ChartConfig`）；不传取默认

    Returns:
        `ReverseResult`：候选列表与截断标志

    Raises:
        IztroError: 条件为空、包含流耀、年份范围非法
    """
    from x_iztro._bridge import query

    data = query(
        "reverseChart",
        reverse_criteria=criteria._payload(),
        config=config.to_dict() if config is not None else None,
    )
    return ReverseResult(
        candidates=[BirthCandidate._from_dict(d) for d in data["candidates"]],
        truncated=bool(data["truncated"]),
    )

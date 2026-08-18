"""
按出生数据安星。

不排整盘、只取某一组星耀时用这些函数。返回的星耀标识与索引都与语言无关，
可直接同星盘对象的 `*_key` 字段比对。

索引一律为宫位索引：0 为寅宫，11 为丑宫。
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any

from x_iztro._bridge import query, typed
from x_iztro.enums import EarthlyBranch, FiveElementsClass, HeavenlyStem, Scope
from x_iztro.models import ChartConfig, Star, TimeIndexType


def _config(config: ChartConfig | None) -> dict | None:
    """排盘配置转为绑定层入参。"""
    return config.to_dict() if config is not None else None


def _birth(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str,
    fix_leap: bool,
    language: str,
    config: ChartConfig | None,
    from_stem: str | None,
    from_branch: str | None,
) -> dict[str, Any]:
    """出生数据部分的公共入参。"""
    return {
        "solar_date": solar_date,
        "time_index": time_index,
        "gender": gender,
        "fix_leap": fix_leap,
        "language": language,
        "config": _config(config),
        "from_stem": from_stem,
        "from_branch": from_branch,
    }


def _stars(groups: list[list[dict]]) -> list[list[Star]]:
    """绑定层的星耀分布转为类型化对象。"""
    return [[Star._from_dict(s) for s in palace] for palace in groups]

# ============================================================
# 落宫结果类型
# ============================================================

@dataclass(frozen=True, slots=True)
class StartIndex:
    """紫微、天府的起始宫位索引。"""

    ziwei_index: int
    """紫微所在宫位索引"""

    tianfu_index: int
    """天府所在宫位索引"""


@dataclass(frozen=True, slots=True)
class LuYangTuoMaIndex:
    """禄存、擎羊、陀罗、天马的宫位索引。"""

    lu_index: int
    """禄存所在宫位索引"""

    yang_index: int
    """擎羊所在宫位索引"""

    tuo_index: int
    """陀罗所在宫位索引"""

    ma_index: int
    """天马所在宫位索引"""


@dataclass(frozen=True, slots=True)
class KuiYueIndex:
    """天魁、天钺的宫位索引。"""

    kui_index: int
    """天魁所在宫位索引"""

    yue_index: int
    """天钺所在宫位索引"""


@dataclass(frozen=True, slots=True)
class ChangQuIndex:
    """文昌、文曲的宫位索引。"""

    chang_index: int
    """文昌所在宫位索引"""

    qu_index: int
    """文曲所在宫位索引"""


@dataclass(frozen=True, slots=True)
class KongJieIndex:
    """地空、地劫的宫位索引。"""

    kong_index: int
    """地空所在宫位索引"""

    jie_index: int
    """地劫所在宫位索引"""


@dataclass(frozen=True, slots=True)
class TimelyStarIndex:
    """台辅、封诰的宫位索引。"""

    taifu_index: int
    """台辅所在宫位索引"""

    fenggao_index: int
    """封诰所在宫位索引"""


@dataclass(frozen=True, slots=True)
class LuanXiIndex:
    """红鸾、天喜的宫位索引。"""

    hongluan_index: int
    """红鸾所在宫位索引"""

    tianxi_index: int
    """天喜所在宫位索引"""


@dataclass(frozen=True, slots=True)
class DailyStarIndex:
    """日系星的宫位索引。"""

    santai_index: int
    """三台所在宫位索引"""

    bazuo_index: int
    """八座所在宫位索引"""

    enguang_index: int
    """恩光所在宫位索引"""

    tiangui_index: int
    """天贵所在宫位索引"""


@dataclass(frozen=True, slots=True)
class MonthlyStarIndex:
    """月系星的宫位索引。"""

    yuejie_index: int
    """解神所在宫位索引"""

    tianyao_index: int
    """天姚所在宫位索引"""

    tianxing_index: int
    """天刑所在宫位索引"""

    yinsha_index: int
    """阴煞所在宫位索引"""

    tianyue_index: int
    """天月所在宫位索引"""

    tianwu_index: int
    """天巫所在宫位索引"""


@dataclass(frozen=True, slots=True)
class YearlyStarIndex:
    """年系杂耀的宫位索引。"""

    xianchi_index: int
    """咸池所在宫位索引"""

    huagai_index: int
    """华盖所在宫位索引"""

    guchen_index: int
    """孤辰所在宫位索引"""

    guasu_index: int
    """寡宿所在宫位索引"""

    tiancai_index: int
    """天才所在宫位索引"""

    tianshou_index: int
    """天寿所在宫位索引"""

    tianchu_index: int
    """天厨所在宫位索引"""

    posui_index: int
    """破碎所在宫位索引"""

    feilian_index: int
    """蜚廉所在宫位索引"""

    longchi_index: int
    """龙池所在宫位索引"""

    fengge_index: int
    """凤阁所在宫位索引"""

    tianku_index: int
    """天哭所在宫位索引"""

    tianxu_index: int
    """天虚所在宫位索引"""

    tianguan_index: int
    """天官所在宫位索引"""

    tianfu_index: int
    """天福所在宫位索引"""

    tiande_index: int
    """天德所在宫位索引"""

    yuede_index: int
    """月德所在宫位索引"""

    tiankong_index: int
    """天空所在宫位索引"""

    jielu_index: int
    """截路所在宫位索引"""

    kongwang_index: int
    """空亡所在宫位索引"""

    xunkong_index: int
    """旬空所在宫位索引"""

    tianshang_index: int
    """天伤所在宫位索引"""

    tianshi_index: int
    """天使所在宫位索引"""

    jiekong_index: int
    """截空（中州派）所在宫位索引"""

    jiesha_adj_index: int
    """劫杀（中州派）所在宫位索引"""

    nianjie_index: int
    """年解所在宫位索引"""

    dahao_adj_index: int
    """大耗（中州派杂耀）所在宫位索引"""


@dataclass(frozen=True, slots=True)
class Yearly12:
    """岁前12神与将前12神，从寅宫起按宫位索引排列。"""

    suiqian12: list[str]
    """岁前12神标识（`Suiqian12` 枚举值域）"""

    jiangqian12: list[str]
    """将前12神标识（`Jiangqian12` 枚举值域）"""


@dataclass(frozen=True, slots=True)
class ZuoYouIndex:
    """左辅、右弼的宫位索引。"""

    zuo_index: int
    """左辅所在宫位索引"""

    you_index: int
    """右弼所在宫位索引"""


@dataclass(frozen=True, slots=True)
class HuoLingIndex:
    """火星、铃星的宫位索引。"""

    huo_index: int
    """火星所在宫位索引"""

    ling_index: int
    """铃星所在宫位索引"""


@dataclass(frozen=True, slots=True)
class HuagaiXianchiIndex:
    """华盖、咸池的宫位索引。"""

    huagai_index: int
    """华盖所在宫位索引"""

    xianchi_index: int
    """咸池所在宫位索引"""


@dataclass(frozen=True, slots=True)
class GuGuaIndex:
    """孤辰、寡宿的宫位索引。"""

    guchen_index: int
    """孤辰所在宫位索引"""

    guasu_index: int
    """寡宿所在宫位索引"""


@dataclass(frozen=True, slots=True)
class TianshiTianshangIndex:
    """天伤、天使的宫位索引。"""

    tianshang_index: int
    """天伤所在宫位索引"""

    tianshi_index: int
    """天使所在宫位索引"""


def get_start_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
    from_stem: str | None = None,
    from_branch: str | None = None,
) -> StartIndex:
    """
    紫微、天府的起始宫位索引。

    `from_stem` 与 `from_branch` 同时给出时改用该干支起五行局（中州派地盘、人盘）。

    Returns:
        StartIndex：紫微与天府的宫位索引
    """
    return typed(
        StartIndex,
        query(
            "getStartIndex",
            **_birth(
                solar_date, time_index, gender, fix_leap, language, config, from_stem, from_branch
            ),
        ),
    )


def get_lu_yang_tuo_ma_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> LuYangTuoMaIndex:
    """禄存、擎羊、陀罗、天马的宫位索引（按年干支）。"""
    return typed(
        LuYangTuoMaIndex,
        query(
            "getLuYangTuoMaIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_kui_yue_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> KuiYueIndex:
    """天魁、天钺的宫位索引（按年干）。"""
    return typed(
        KuiYueIndex,
        query(
            "getKuiYueIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_chang_qu_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> ChangQuIndex:
    """文昌、文曲的宫位索引（按时支）。"""
    return typed(
        ChangQuIndex,
        query(
            "getChangQuIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_kong_jie_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> KongJieIndex:
    """地空、地劫的宫位索引（按时支）。"""
    return typed(
        KongJieIndex,
        query(
            "getKongJieIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_timely_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> TimelyStarIndex:
    """台辅、封诰的宫位索引（按时支）。"""
    return typed(
        TimelyStarIndex,
        query(
            "getTimelyStarIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_luan_xi_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> LuanXiIndex:
    """红鸾、天喜的宫位索引（按年支）。"""
    return typed(
        LuanXiIndex,
        query(
            "getLuanXiIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_daily_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> DailyStarIndex:
    """日系星索引：三台、八座、恩光、天贵。"""
    return typed(
        DailyStarIndex,
        query(
            "getDailyStarIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_monthly_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> MonthlyStarIndex:
    """月系星索引：解神、天姚、天刑、阴煞、天月、天巫。"""
    return typed(
        MonthlyStarIndex,
        query(
            "getMonthlyStarIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_yearly_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> YearlyStarIndex:
    """年系杂耀的宫位索引（一整组，年支按 `horoscope_divide` 分界）。"""
    return typed(
        YearlyStarIndex,
        query(
            "getYearlyStarIndex",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_major_star(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
    from_stem: str | None = None,
    from_branch: str | None = None,
) -> list[list[Star]]:
    """十四主星在十二宫的分布，按宫位索引排列。"""
    return _stars(
        query(
            "getMajorStar",
            **_birth(
                solar_date, time_index, gender, fix_leap, language, config, from_stem, from_branch
            ),
        )
    )


def get_minor_star(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> list[list[Star]]:
    """十四辅星在十二宫的分布，按宫位索引排列。"""
    return _stars(
        query(
            "getMinorStar",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        )
    )


def get_adjective_star(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> list[list[Star]]:
    """杂耀在十二宫的分布，按宫位索引排列。"""
    return _stars(
        query(
            "getAdjectiveStar",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        )
    )


def get_changsheng12(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
    from_stem: str | None = None,
    from_branch: str | None = None,
) -> list[str]:
    """长生12神标识，从寅宫起按宫位索引排列。"""
    return query(
        "getChangsheng12",
        **_birth(
            solar_date, time_index, gender, fix_leap, language, config, from_stem, from_branch
        ),
    )


def get_boshi12(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> list[str]:
    """博士12神标识，从寅宫起按宫位索引排列。"""
    return query(
        "getBoShi12",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_yearly12(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> Yearly12:
    """
    岁前12神与将前12神标识，从寅宫起按宫位索引排列。

    流年神煞按 `horoscope_divide` 分界取年支。

    Returns:
        Yearly12：岁前12神与将前12神两组标识
    """
    return typed(
        Yearly12,
        query(
            "getYearly12",
            **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
        ),
    )


def get_horoscope_star(
    stem: HeavenlyStem | str,
    branch: EarthlyBranch | str,
    scope: Scope | str,
    language: str = "zh-CN",
) -> list[list[Star]]:
    """
    流耀在十二宫的分布：魁钺昌曲禄羊陀马鸾喜，流年层级另加年解。

    Args:
        stem: 该运限层级的天干标识
        branch: 该运限层级的地支标识
        scope: 运限层级标识，决定星名（如大限为运魁、流年为流魁）
    """
    return _stars(
        query(
            "getHoroscopeStar",
            stem_key=str(stem),
            branch_key=str(branch),
            scope=str(scope),
            language=language,
        )
    )


def get_changsheng12_start_index(five_elements_class: FiveElementsClass | str) -> int:
    """长生12神的起始宫位索引：水二局在申、木三局在亥、金四局在巳、土五局在申、火六局在寅。"""
    return query("getChangesheng12StartIndex", five_elements_class=str(five_elements_class))


def get_jiangqian12_start_index(branch: EarthlyBranch | str) -> int:
    """将前12神的起始宫位索引：寅午戌年在午、申子辰年在子、巳酉丑年在酉、亥卯未年在卯。"""
    return query("getJiangqian12StartIndex", branch_key=str(branch))


def get_zuo_you_index(lunar_month: int) -> ZuoYouIndex:
    """
    左辅、右弼的宫位索引（按农历月份）。

    Args:
        lunar_month: 经闰月修正后的农历月份 1-12，即 `fix_lunar_month_index` 结果加一

    Returns:
        ZuoYouIndex：左辅与右弼的宫位索引
    """
    return typed(ZuoYouIndex, query("getZuoYouIndex", lunar_month=lunar_month))


def get_huo_ling_index(branch: EarthlyBranch | str, time_index: TimeIndexType) -> HuoLingIndex:
    """
    火星、铃星的宫位索引（按年支与时辰）。

    Args:
        branch: 年支标识
        time_index: 时辰索引 0-12

    Returns:
        HuoLingIndex：火星与铃星的宫位索引
    """
    return typed(
        HuoLingIndex, query("getHuoLingIndex", branch_key=str(branch), time_index=time_index)
    )


def get_huagai_xianchi_index(branch: EarthlyBranch | str) -> HuagaiXianchiIndex:
    """
    华盖、咸池的宫位索引（按年支）。

    Returns:
        HuagaiXianchiIndex：华盖与咸池的宫位索引
    """
    return typed(HuagaiXianchiIndex, query("getHuagaiXianchiIndex", branch_key=str(branch)))


def get_gu_gua_index(branch: EarthlyBranch | str) -> GuGuaIndex:
    """
    孤辰、寡宿的宫位索引（按年支）。

    Returns:
        GuGuaIndex：孤辰与寡宿的宫位索引
    """
    return typed(GuGuaIndex, query("getGuGuaIndex", branch_key=str(branch)))


def get_jiesha_adj_index(branch: EarthlyBranch | str) -> int:
    """劫煞（杂耀）的宫位索引（按年支）。取值只有 0、3、6、9 四种。"""
    return query("getJieshaAdjIndex", branch_key=str(branch))


def get_dahao_index(branch: EarthlyBranch | str) -> int:
    """大耗（杂耀）的宫位索引（按年支）。"""
    return query("getDahaoIndex", branch_key=str(branch))


def get_nianjie_index(branch: EarthlyBranch | str) -> int:
    """年解的宫位索引（按年支）。"""
    return query("getNianjieIndex", branch_key=str(branch))


def get_tianshi_tianshang_index(
    gender: str,
    branch: EarthlyBranch | str,
    soul_index: int,
    config: ChartConfig | None = None,
) -> TianshiTianshangIndex:
    """
    天伤、天使的宫位索引（按性别、年支与命宫位置）。

    二者夹迁移宫：通行派天伤居仆役位、天使居疾厄位；中州派在阴男阳女
    （生年地支阴阳与性别阴阳不同）时二者对调，由 `config.algorithm` 决定走哪一派。

    Args:
        gender: "male" 或 "female"
        branch: 年支标识
        soul_index: 命宫宫位索引
        config: 排盘配置，None 取默认

    Returns:
        TianshiTianshangIndex：天伤与天使的宫位索引
    """
    return typed(
        TianshiTianshangIndex,
        query(
            "getTianshiTianshangIndex",
            gender=gender,
            branch_key=str(branch),
            soul_index=soul_index,
            config=_config(config),
        ),
    )


def get_chang_qu_index_by_heavenly_stem(stem: HeavenlyStem | str) -> ChangQuIndex:
    """
    文昌、文曲的宫位索引（按天干）。

    运限层级的流昌流曲用这一支，本命盘的昌曲按时支走 `get_chang_qu_index`。

    Returns:
        ChangQuIndex：文昌与文曲的宫位索引
    """
    return typed(ChangQuIndex, query("getChangQuIndexByHeavenlyStem", stem_key=str(stem)))

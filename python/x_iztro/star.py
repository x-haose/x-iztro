"""
按出生数据安星。

不排整盘、只取某一组星耀时用这些函数。返回的星耀标识与索引都与语言无关，
可直接同星盘对象的 `*_key` 字段比对。

索引一律为宫位索引：0 为寅宫，11 为丑宫。
"""

from __future__ import annotations

from typing import Any

from x_iztro._bridge import query
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


def get_start_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
    from_stem: str | None = None,
    from_branch: str | None = None,
) -> dict[str, int]:
    """
    紫微、天府的起始宫位索引。

    `from_stem` 与 `from_branch` 同时给出时改用该干支起五行局（中州派地盘、人盘）。

    Returns:
        含 ziweiIndex、tianfuIndex 的 dict
    """
    return query(
        "getStartIndex",
        **_birth(
            solar_date, time_index, gender, fix_leap, language, config, from_stem, from_branch
        ),
    )


def get_lu_yang_tuo_ma_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """禄存、擎羊、陀罗、天马的宫位索引（按年干支）。"""
    return query(
        "getLuYangTuoMaIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_kui_yue_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """天魁、天钺的宫位索引（按年干）。"""
    return query(
        "getKuiYueIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_chang_qu_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """文昌、文曲的宫位索引（按时支）。"""
    return query(
        "getChangQuIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_kong_jie_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """地空、地劫的宫位索引（按时支）。"""
    return query(
        "getKongJieIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_timely_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """台辅、封诰的宫位索引（按时支）。"""
    return query(
        "getTimelyStarIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_luan_xi_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """红鸾、天喜的宫位索引（按年支）。"""
    return query(
        "getLuanXiIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_daily_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """日系星索引：三台、八座、恩光、天贵。"""
    return query(
        "getDailyStarIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_monthly_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """月系星索引：解神、天姚、天刑、阴煞、天月、天巫。"""
    return query(
        "getMonthlyStarIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
    )


def get_yearly_star_index(
    solar_date: str,
    time_index: TimeIndexType,
    gender: str = "male",
    fix_leap: bool = True,
    language: str = "zh-CN",
    config: ChartConfig | None = None,
) -> dict[str, int]:
    """年系杂耀的宫位索引（一整组，年支按 `horoscope_divide` 分界）。"""
    return query(
        "getYearlyStarIndex",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
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
) -> dict[str, list[str]]:
    """
    岁前12神与将前12神标识，从寅宫起按宫位索引排列。

    流年神煞按 `horoscope_divide` 分界取年支。

    Returns:
        含 suiqian12、jiangqian12 的 dict
    """
    return query(
        "getYearly12",
        **_birth(solar_date, time_index, gender, fix_leap, language, config, None, None),
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


def get_zuo_you_index(lunar_month: int) -> dict[str, int]:
    """
    左辅、右弼的宫位索引（按农历月份）。

    Args:
        lunar_month: 经闰月修正后的农历月份 1-12，即 `fix_lunar_month_index` 结果加一

    Returns:
        含 zuoIndex、youIndex 的 dict
    """
    return query("getZuoYouIndex", lunar_month=lunar_month)


def get_huo_ling_index(branch: EarthlyBranch | str, time_index: TimeIndexType) -> dict[str, int]:
    """
    火星、铃星的宫位索引（按年支与时辰）。

    Args:
        branch: 年支标识
        time_index: 时辰索引 0-12

    Returns:
        含 huoIndex、lingIndex 的 dict
    """
    return query("getHuoLingIndex", branch_key=str(branch), time_index=time_index)


def get_huagai_xianchi_index(branch: EarthlyBranch | str) -> dict[str, int]:
    """
    华盖、咸池的宫位索引（按年支）。

    Returns:
        含 huagaiIndex、xianchiIndex 的 dict
    """
    return query("getHuagaiXianchiIndex", branch_key=str(branch))


def get_gu_gua_index(branch: EarthlyBranch | str) -> dict[str, int]:
    """
    孤辰、寡宿的宫位索引（按年支）。

    Returns:
        含 guchenIndex、guasuIndex 的 dict
    """
    return query("getGuGuaIndex", branch_key=str(branch))


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
) -> dict[str, int]:
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
        含 tianshangIndex、tianshiIndex 的 dict
    """
    return query(
        "getTianshiTianshangIndex",
        gender=gender,
        branch_key=str(branch),
        soul_index=soul_index,
        config=_config(config),
    )


def get_chang_qu_index_by_heavenly_stem(stem: HeavenlyStem | str) -> dict[str, int]:
    """
    文昌、文曲的宫位索引（按天干）。

    运限层级的流昌流曲用这一支，本命盘的昌曲按时支走 `get_chang_qu_index`。

    Returns:
        含 changIndex、quIndex 的 dict
    """
    return query("getChangQuIndexByHeavenlyStem", stem_key=str(stem))

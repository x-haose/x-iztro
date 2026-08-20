"""
x-iztro 排盘主类

封装 Rust 原生模块，返回类型化的 dataclass 对象。
运限与 Prompt 为无状态接口：从星盘对象自带的排盘上下文重新发起计算，
无需在 Python 与 Rust 之间往返完整星盘数据。
"""

from __future__ import annotations

from x_iztro import _bridge as bridge
from x_iztro.models import (
    Astrolabe,
    ChartConfig,
    GenderType,
    LanguageType,
    TimeIndexType,
    Horoscope,
)


def _config(config: ChartConfig | None) -> dict | None:
    """排盘配置转为绑定层入参。"""
    return config.to_dict() if config is not None else None


class Astro:
    """紫微斗数排盘主类"""

    def by_solar(
        self,
        solar_date: str,
        time_index: TimeIndexType,
        gender: GenderType,
        *,
        fix_leap: bool = True,
        language: LanguageType = "zh-CN",
        config: ChartConfig | None = None,
    ) -> Astrolabe:
        """
        阳历排盘

        `gender` 之后的参数只能按关键字传入：布尔与语言码相邻，位置传参极易写反且不报错。

        Args:
            solar_date: 阳历日期，如 "2000-8-16"
            time_index: 时辰索引 (0-12)
            gender: "male" 或 "female"
            fix_leap: 是否修正闰月
            language: 输出语言
            config: 排盘配置，如
                ChartConfig(algorithm=Algorithm.ZHONGZHOU, year_divide=YearDivide.EXACT)

        Returns:
            Astrolabe 星盘对象

        Raises:
            IztroError: 入参非法（日期格式/范围、时辰索引、性别、语言或配置）
        """
        data = bridge.by_solar(
            solar_date=solar_date,
            time_index=time_index,
            gender=gender,
            fix_leap=fix_leap,
            language=language,
            config=_config(config),
        )
        return Astrolabe._from_dict(data, config)

    def by_lunar(
        self,
        lunar_date: str,
        time_index: TimeIndexType,
        gender: GenderType,
        *,
        is_leap_month: bool = False,
        fix_leap: bool = True,
        language: LanguageType = "zh-CN",
        config: ChartConfig | None = None,
    ) -> Astrolabe:
        """
        农历排盘

        `gender` 之后的参数只能按关键字传入：`is_leap_month` 与 `fix_leap` 两个布尔相邻，
        位置传参写反了不报错、盘会静默错一个月。

        Args:
            lunar_date: 农历日期，如 "2000-7-17"
            time_index: 时辰索引 (0-12)
            gender: "male" 或 "female"
            is_leap_month: 是否闰月（该月无闰月时不生效）
            fix_leap: 是否修正闰月
            language: 输出语言
            config: 排盘配置

        Returns:
            Astrolabe 星盘对象

        Raises:
            IztroError: 入参非法（日期格式/范围、时辰索引、性别、语言或配置）
        """
        data = bridge.by_lunar(
            lunar_date=lunar_date,
            time_index=time_index,
            gender=gender,
            is_leap_month=is_leap_month,
            fix_leap=fix_leap,
            language=language,
            config=_config(config),
        )
        return Astrolabe._from_dict(data, config)

    def get_horoscope(
        self,
        astrolabe: Astrolabe,
        target_date: str | None = None,
        target_time_index: TimeIndexType | None = None,
    ) -> Horoscope:
        """
        计算运限（从星盘自带的排盘上下文无状态发起）

        Args:
            astrolabe: 星盘对象（由 by_solar 或 by_lunar 返回）
            target_date: 目标阳历日期，如 "2024-1-1"；不传取今天
            target_time_index: 目标时辰索引 (0-12)；不传取此刻所属时辰

        Returns:
            Horoscope 运限对象，持有传入的星盘

        Raises:
            IztroError: 入参非法（日期格式/范围、时辰索引、性别、语言或配置）
        """
        return astrolabe.horoscope(target_date, target_time_index)

    def astrolabe_to_prompt(self, astrolabe: Astrolabe) -> str:
        """
        生成本命盘 AI Prompt

        Args:
            astrolabe: 星盘对象

        Returns:
            结构化文本 prompt

        Raises:
            IztroError: 入参非法（日期格式/范围、时辰索引、性别、语言或配置）
        """
        return bridge.query(
            "astrolabeToPrompt",
            solar_date=astrolabe.solar_date,
            time_index=astrolabe.time_index,
            gender=astrolabe.gender_key,
            fix_leap=astrolabe.fix_leap,
            language=astrolabe.language,
            config=astrolabe._config_payload(),
            from_stem=astrolabe._from_stem,
            from_branch=astrolabe._from_branch,
        )

    def horoscope_to_prompt(
        self,
        astrolabe: Astrolabe,
        target_date: str,
        target_time_index: TimeIndexType,
    ) -> str:
        """
        生成运限 AI Prompt

        Args:
            astrolabe: 星盘对象
            target_date: 目标阳历日期
            target_time_index: 目标时辰索引 (0-12)

        Returns:
            结构化文本 prompt

        Raises:
            IztroError: 入参非法（日期格式/范围、时辰索引、性别、语言或配置）
        """
        return bridge.query(
            "horoscopeToPrompt",
            solar_date=astrolabe.solar_date,
            time_index=astrolabe.time_index,
            gender=astrolabe.gender_key,
            fix_leap=astrolabe.fix_leap,
            language=astrolabe.language,
            config=astrolabe._config_payload(),
            from_stem=astrolabe._from_stem,
            from_branch=astrolabe._from_branch,
            target_date=target_date,
            target_time_index=target_time_index,
        )

"""
rs-iztro 排盘主类

封装 Rust 原生模块，返回类型化的 dataclass 对象。
运限与 Prompt 为无状态接口：从星盘对象自带的排盘上下文重新发起计算，
无需在 Python 与 Rust 之间往返完整星盘数据。
"""

from __future__ import annotations

from rs_iztro.models import (
    Astrolabe,
    GenderType,
    LanguageType,
    TimeIndexType,
    Horoscope,
)


def _get_native():
    """延迟加载 Rust 原生扩展模块，避免循环导入。"""
    import rs_iztro._rs_iztro as mod
    return mod


def _config_json(config: dict | None) -> str | None:
    """用户 config dict（camelCase 键，可部分给出）转为绑定层 JSON。"""
    if config is None:
        return None
    import json
    return json.dumps(config)


class Astro:
    """紫微斗数排盘主类"""

    def by_solar(
        self,
        solar_date: str,
        time_index: TimeIndexType,
        gender: GenderType,
        fix_leap: bool = True,
        language: LanguageType = "zh_cn",
        config: dict | None = None,
    ) -> Astrolabe:
        """
        阳历排盘

        Args:
            solar_date: 阳历日期，如 "2000-8-16"
            time_index: 时辰索引 (0-12)
            gender: "male" 或 "female"
            fix_leap: 是否修正闰月
            language: 输出语言
            config: 排盘配置（camelCase 键，可部分给出），如
                {"algorithm": "zhongzhou", "yearDivide": "exact"}

        Returns:
            Astrolabe 星盘对象
        """
        data = _get_native().by_solar(
            solar_date, time_index, gender, fix_leap, language, _config_json(config),
        )
        return Astrolabe._from_dict(data)

    def by_lunar(
        self,
        lunar_date: str,
        time_index: TimeIndexType,
        gender: GenderType,
        is_leap_month: bool = False,
        fix_leap: bool = True,
        language: LanguageType = "zh_cn",
        config: dict | None = None,
    ) -> Astrolabe:
        """
        农历排盘

        Args:
            lunar_date: 农历日期，如 "2000-7-17"
            time_index: 时辰索引 (0-12)
            gender: "male" 或 "female"
            is_leap_month: 是否闰月（该月无闰月时不生效）
            fix_leap: 是否修正闰月
            language: 输出语言
            config: 排盘配置（camelCase 键，可部分给出）

        Returns:
            Astrolabe 星盘对象
        """
        data = _get_native().by_lunar(
            lunar_date, time_index, gender, is_leap_month, fix_leap, language,
            _config_json(config),
        )
        return Astrolabe._from_dict(data)

    def get_horoscope(
        self,
        astrolabe: Astrolabe,
        target_date: str,
        target_time_index: TimeIndexType,
    ) -> Horoscope:
        """
        计算运限（从星盘自带的排盘上下文无状态发起）

        Args:
            astrolabe: 星盘对象（由 by_solar 或 by_lunar 返回）
            target_date: 目标阳历日期，如 "2024-1-1"
            target_time_index: 目标时辰索引 (0-12)

        Returns:
            Horoscope 运限对象
        """
        data = _get_native().get_horoscope(
            astrolabe.solar_date,
            astrolabe.time_index,
            astrolabe.gender_key,
            astrolabe.fix_leap,
            astrolabe.language,
            astrolabe.config.to_json(),
            target_date,
            target_time_index,
        )
        return Horoscope._from_dict(data)

    def astrolabe_to_prompt(self, astrolabe: Astrolabe) -> str:
        """
        生成本命盘 AI Prompt

        Args:
            astrolabe: 星盘对象

        Returns:
            结构化文本 prompt
        """
        return _get_native().astrolabe_to_prompt(
            astrolabe.solar_date,
            astrolabe.time_index,
            astrolabe.gender_key,
            astrolabe.fix_leap,
            astrolabe.language,
            astrolabe.config.to_json(),
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
        """
        return _get_native().horoscope_to_prompt(
            astrolabe.solar_date,
            astrolabe.time_index,
            astrolabe.gender_key,
            astrolabe.fix_leap,
            astrolabe.language,
            astrolabe.config.to_json(),
            target_date,
            target_time_index,
        )

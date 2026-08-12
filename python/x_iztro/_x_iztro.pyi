"""x-iztro Rust 原生模块的类型存根。"""

from typing import Any

def by_solar(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    config_json: str | None = None,
) -> dict[str, Any]:
    """阳历排盘，返回星盘 dict"""
    ...

def by_lunar(
    lunar_date: str,
    time_index: int,
    gender: str,
    is_leap_month: bool,
    fix_leap: bool,
    language: str,
    config_json: str | None = None,
) -> dict[str, Any]:
    """农历排盘，返回星盘 dict"""
    ...

def get_horoscope(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    config_json: str | None,
    target_date: str,
    target_time_index: int,
) -> dict[str, Any]:
    """计算运限，返回运限 dict"""
    ...

def astrolabe_to_prompt(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    config_json: str | None = None,
) -> str:
    """生成本命盘 AI Prompt"""
    ...

def horoscope_to_prompt(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    config_json: str | None,
    target_date: str,
    target_time_index: int,
) -> str:
    """生成运限 AI Prompt"""
    ...

def by_solar_json(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    config_json: str | None = None,
) -> str:
    """阳历排盘，返回星盘 JSON 字符串"""
    ...

def by_lunar_json(
    lunar_date: str,
    time_index: int,
    gender: str,
    is_leap_month: bool,
    fix_leap: bool,
    language: str,
    config_json: str | None = None,
) -> str:
    """农历排盘，返回星盘 JSON 字符串"""
    ...

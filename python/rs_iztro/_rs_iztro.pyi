"""
rs-iztro Rust 原生模块类型存根

由 Rust PyO3 编译生成的原生扩展模块的类型提示。
"""

from typing import Any

def by_solar(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    algorithm: str,
) -> dict[str, Any]:
    """阳历排盘，返回 dict"""
    ...

def by_lunar(
    lunar_date: str,
    time_index: int,
    gender: str,
    is_leap_month: bool,
    fix_leap: bool,
    language: str,
    algorithm: str,
) -> dict[str, Any]:
    """农历排盘，返回 dict"""
    ...

def get_horoscope(
    astrolabe: dict[str, Any],
    target_date: str,
    time_index: int,
    language: str,
) -> dict[str, Any]:
    """计算运限，返回 dict"""
    ...

def astrolabe_to_prompt(
    astrolabe: dict[str, Any],
    language: str,
) -> str:
    """生成 AI Prompt"""
    ...

def by_solar_json(
    solar_date: str,
    time_index: int,
    gender: str,
    fix_leap: bool,
    language: str,
    algorithm: str,
) -> str:
    """阳历排盘，返回 JSON 字符串"""
    ...

def by_lunar_json(
    lunar_date: str,
    time_index: int,
    gender: str,
    is_leap_month: bool,
    fix_leap: bool,
    language: str,
    algorithm: str,
) -> str:
    """农历排盘，返回 JSON 字符串"""
    ...

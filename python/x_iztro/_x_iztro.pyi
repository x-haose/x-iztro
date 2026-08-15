"""
Rust 原生扩展模块的类型存根。

入参为 camelCase 键的 dict，出参为 camelCase 键、值按语言翻译的原生对象；
字段与取值约定见 Rust 侧的 `crate::bridge`。上层请用 `x_iztro._bridge`，
它负责把 snake_case 关键字转成这里要求的 camelCase。
"""

from typing import Any

__version__: str

def by_solar(input: dict[str, Any]) -> dict[str, Any]:
    """阳历排盘，返回星盘 DTO。"""

def by_lunar(input: dict[str, Any]) -> dict[str, Any]:
    """农历排盘，返回星盘 DTO。"""

def get_horoscope(input: dict[str, Any]) -> dict[str, Any]:
    """运限，返回运限 DTO。"""

def query(input: dict[str, Any]) -> Any:
    """统一查询，由入参的 kind 分派。"""

def by_solar_json(input: dict[str, Any]) -> str:
    """阳历排盘，返回星盘 DTO 的 JSON 字符串。"""

def by_lunar_json(input: dict[str, Any]) -> str:
    """农历排盘，返回星盘 DTO 的 JSON 字符串。"""

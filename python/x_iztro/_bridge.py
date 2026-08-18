"""
绑定层调用入口。

Rust 侧的 `x_iztro._x_iztro` 只暴露六个函数，其中 `query` 由 `kind` 分派到
全部轻量查询、工具函数、安星、数据表与翻译。本模块把入参的 snake_case
关键字转成绑定层约定的 camelCase，并剔除未给出的可选项。

Go 绑定走的是同一个 Rust 分派函数，因此两侧的行为不会分叉。
"""

from __future__ import annotations

from typing import Any, TypeVar

T = TypeVar("T")


def _native():
    """延迟加载 Rust 原生扩展模块，避免循环导入。"""
    import x_iztro._x_iztro as mod

    return mod


def _camel(name: str) -> str:
    """snake_case 转 camelCase。"""
    head, *rest = name.split("_")
    return head + "".join(part.title() for part in rest)


def _snake(name: str) -> str:
    """camelCase 转 snake_case。"""
    out = []
    for ch in name:
        if ch.isupper():
            out.append("_")
            out.append(ch.lower())
        else:
            out.append(ch)
    return "".join(out)


def typed(cls: type[T], payload: dict[str, Any]) -> T:
    """把 camelCase 键的绑定层结果落成具名 dataclass。

    dataclass 的字段名一律是绑定层键名的 snake_case 形式，多一个键或少一个键
    都会在构造时立即报错，因此契约漂移不会被静默吞掉。
    """
    return cls(**{_snake(k): v for k, v in payload.items()})


def _payload(params: dict[str, Any]) -> dict[str, Any]:
    """剔除未给出的可选项，其余键转 camelCase。"""
    return {_camel(k): v for k, v in params.items() if v is not None}


def query(kind: str, **params: Any) -> Any:
    """按 `kind` 调用绑定层的统一查询入口。"""
    return _native().query({"kind": kind, **_payload(params)})


def by_solar(**params: Any) -> Any:
    """阳历排盘，返回 DTO dict。"""
    return _native().by_solar(_payload(params))


def by_lunar(**params: Any) -> Any:
    """农历排盘，返回 DTO dict。"""
    return _native().by_lunar(_payload(params))


def by_solar_json(**params: Any) -> str:
    """阳历排盘，返回 DTO JSON 字符串。"""
    return _native().by_solar_json(_payload(params))


def by_lunar_json(**params: Any) -> str:
    """农历排盘，返回 DTO JSON 字符串。"""
    return _native().by_lunar_json(_payload(params))


def horoscope(**params: Any) -> Any:
    """运限，返回 DTO dict。"""
    return _native().get_horoscope(_payload(params))

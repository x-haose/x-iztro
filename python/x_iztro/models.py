"""
x-iztro 数据模型的聚合入口。

模型按领域分在 `enums` / `config` / `star_object` / `palace` / `surpalaces` /
`horoscope` / `pattern` / `astrolabe` 八个模块里（与 Rust 侧 `src/models/` 对应），本模块把它们
汇总到一个命名空间，`x_iztro.models.Astrolabe` 与 `x_iztro.Astrolabe` 两条路径等价。

从 Rust 返回的星盘数据 1:1 映射为 frozen dataclass，零外部依赖。
每个对象同时携带两层信息：

- 翻译字段（`name`、`brightness` 等）：按排盘语言本地化的展示文本；
- 标识字段（`key`、`name_key` 等 `*_key`）：语言无关的 iztro i18n key。

所有判断方法（`has`/`has_mutagen`/`flies_to`/`palace` 查询等）基于标识字段
比较，传入 `x_iztro.enums` 的枚举成员即可在任何输出语言的星盘上正确工作；
为方便中文场景，接受星耀/宫位参数的方法同时兼容当前语言的翻译名。
"""

from __future__ import annotations

from x_iztro.astrolabe import Astrolabe, RawChineseDate, RawDates, RawLunarDate
from x_iztro.config import ChartConfig
from x_iztro.horoscope import (
    AgeItem,
    Horoscope,
    HoroscopeItem,
    HoroscopeYearly,
    YearlyDecStar,
)
from x_iztro.palace import Decadal, Palace
from x_iztro.pattern import PatternConfig, PatternHit, PatternStar
from x_iztro.star_object import Star
from x_iztro.surpalaces import SurroundedPalaces
from x_iztro.enums import (
    GenderType,
    LanguageType,
    ScopeLiteral,
    StarTypeLiteral,
    TimeIndexType,
)

__all__ = [
    "PatternConfig",
    "PatternHit",
    "PatternStar",
    # 类型别名
    "TimeIndexType",
    "GenderType",
    "LanguageType",
    "StarTypeLiteral",
    "ScopeLiteral",
    # 配置
    "ChartConfig",
    # 星盘与出生日期
    "Astrolabe",
    "RawDates",
    "RawLunarDate",
    "RawChineseDate",
    # 宫位与星耀
    "Palace",
    "Decadal",
    "Star",
    "SurroundedPalaces",
    # 运限
    "Horoscope",
    "HoroscopeItem",
    "HoroscopeYearly",
    "AgeItem",
    "YearlyDecStar",
]

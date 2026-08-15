"""
插件：给星盘补自定义分析方法。

一个插件就是一个函数，
接受 `Astrolabe` 类并往上挂方法；加载后所有星盘实例都有这些方法，
包括加载之前已经排好的盘。

```python
from x_iztro import Astro, Astrolabe
from x_iztro.plugin import load_plugin

def my_plugin(cls: type[Astrolabe]) -> None:
    def major_star(self) -> str:
        soul = self.palace(PalaceName.SOUL)
        source = soul.opposite_palace() if soul.is_empty() else soul
        return ",".join(s.name for s in source.major_stars)

    cls.major_star = major_star

load_plugin(my_plugin)
Astro().by_solar("2000-8-16", 2, "female").major_star()   # '紫微'
```

模型是 `slots=True` 的 frozen dataclass —— 实例上挂不了属性，
但类上挂方法不受影响，这正是插件需要的粒度：
插件改的是「所有星盘都有这个方法」，不是「这一张盘多了个字段」。
"""

from __future__ import annotations

from collections.abc import Callable, Iterable

from x_iztro.models import Astrolabe

Plugin = Callable[[type[Astrolabe]], None]
"""插件签名：接受 `Astrolabe` 类，往上挂方法。"""


def load_plugin(plugin: Plugin) -> None:
    """
    加载单个插件。

    Args:
        plugin: 接受 `Astrolabe` 类的函数

    Raises:
        TypeError: plugin 不可调用
    """
    if not callable(plugin):
        raise TypeError(f"插件须为可调用对象，实际 {type(plugin).__name__}")
    plugin(Astrolabe)


def load_plugins(plugins: Iterable[Plugin]) -> None:
    """按顺序加载多个插件；任一插件不可调用即报错，且不会加载后续插件。"""
    for plugin in plugins:
        load_plugin(plugin)

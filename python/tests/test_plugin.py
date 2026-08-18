"""插件机制测试。

实现的插件与 `tests/extension.rs`（Rust 扩展 trait）、
`go/iztro/plugin_test.go`（Go 嵌入）是同一个，断言同一组取值。
"""

from __future__ import annotations

import pytest

from x_iztro import Astro, Astrolabe
from x_iztro.enums import FiveElementsClass, PalaceName
from x_iztro.plugin import load_plugin, load_plugins


def my_analysis(cls: type[Astrolabe]) -> None:
    """插件：命宫主星（空宫借对宫）。"""

    def major_star(self: Astrolabe) -> str:
        soul = self.palace(PalaceName.SOUL)
        source = soul.opposite_palace() if soul.is_empty() else soul
        return ",".join(s.name for s in source.major_stars)

    cls.major_star = major_star


def five_elements(cls: type[Astrolabe]) -> None:
    """插件：五行局的局数。"""

    def five_elements_value(self: Astrolabe) -> int:
        return FiveElementsClass(self.five_elements_class_key).number

    cls.five_elements_value = five_elements_value


@pytest.fixture(scope="module", autouse=True)
def _loaded():
    load_plugins([my_analysis, five_elements])


def test_plugin_methods_are_available_on_charts():
    chart = Astro().by_solar("2000-8-16", 2, "female")

    assert chart.major_star() == "紫微"
    assert chart.five_elements_value() == 3


def test_plugin_output_follows_chart_language():
    chart = Astro().by_solar("2000-8-16", 2, "female", language="en-US")

    assert chart.major_star() == "emperor"
    assert chart.five_elements_value() == 3


def test_plugin_applies_to_charts_created_before_loading():
    """插件挂在类上，因此先排的盘也拿得到新方法。"""
    chart = Astro().by_solar("2000-8-16", 2, "female")

    def late_plugin(cls: type[Astrolabe]) -> None:
        cls.palace_count = lambda self: len(self.palaces)

    load_plugin(late_plugin)

    assert chart.palace_count() == 12


def test_load_plugin_rejects_non_callable():
    with pytest.raises(TypeError):
        load_plugin("not a plugin")

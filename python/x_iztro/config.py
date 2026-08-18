"""
排盘配置。

六个开关决定算法分界与盘型，两张覆盖表用于替换内置的四化与亮度数据；
取值一律用 `x_iztro.enums` 的枚举成员或等价字符串。
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True, slots=True)
class ChartConfig:
    """排盘配置。字段取值见 `x_iztro.enums` 的对应枚举；默认值与 JS iztro 一致。"""

    year_divide: str = "normal"
    """年分界点（`YearDivide`）：normal=正月初一 / exact=立春"""

    horoscope_divide: str = "normal"
    """运限分界点（`HoroscopeDivide`）：normal=初一 / exact=节气"""

    age_divide: str = "normal"
    """虚岁分界点（`AgeDivide`）：normal=跨年即加 / birthday=过生日才加"""

    day_divide: str = "forward"
    """晚子时归属（`DayDivide`）：forward=归次日 / current=归当天"""

    algorithm: str = "default"
    """算法派别（`Algorithm`）：default / zhongzhou"""

    astro_type: str = "heaven"
    """排盘视角（`AstroType`）：heaven=天盘 / earth=地盘 / human=人盘"""

    mutagens: dict[str, list[str]] | None = None
    """自定义四化表：天干标识 → 四颗星标识（禄、权、科、忌）。

    按天干整表替换默认值，未列出的天干仍用默认表；键与值都用语言无关标识。
    """

    brightness: dict[str, list[str]] | None = None
    """自定义亮度表：星耀标识 → 十二宫亮度标识（十二项，空串表示该宫无亮度）。

    按星耀整表替换默认值，未列出的星耀仍用默认表；索引 0 为寅宫。
    """

    def to_dict(self) -> dict:
        """转为绑定层接受的 config 对象；枚举成员一律落成其字符串值。"""
        payload: dict = {
            "yearDivide": str(self.year_divide),
            "horoscopeDivide": str(self.horoscope_divide),
            "ageDivide": str(self.age_divide),
            "dayDivide": str(self.day_divide),
            "algorithm": str(self.algorithm),
            "astroType": str(self.astro_type),
        }
        if self.mutagens:
            payload["mutagens"] = {
                str(k): [str(v) for v in vs] for k, vs in self.mutagens.items()
            }
        if self.brightness:
            payload["brightness"] = {
                str(k): [str(v) for v in vs] for k, vs in self.brightness.items()
            }
        return payload

    @classmethod
    def _from_dict(cls, d: dict) -> ChartConfig:
        return cls(
            year_divide=d["yearDivide"],
            horoscope_divide=d["horoscopeDivide"],
            age_divide=d["ageDivide"],
            day_divide=d["dayDivide"],
            algorithm=d["algorithm"],
            astro_type=d["astroType"],
        )

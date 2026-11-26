"""
排盘配置。

开关决定算法分界、盘型与各项流派口径，覆盖表用于替换或补充内置的四化与亮度数据；
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
    """晚子时归属（`DayDivide`）：forward=日数归次日 / current=归当天 / forwardCarry=整个日期归次日"""

    algorithm: str = "default"
    """算法派别（`Algorithm`）：default / zhongzhou"""

    astro_type: str = "heaven"
    """排盘视角（`AstroType`）：heaven=天盘 / earth=地盘 / human=人盘"""

    void_placement: str = "default"
    """截空与旬空的安法（`VoidPlacement`）：default=同 iztro / zhengPang=各安正空、傍空两宫"""

    leap_divide: str = "default"
    """闰月出生时安星用哪个月（`LeapDivide`）：default=按 fix_leap / nextMonth=整月按下月"""

    tianma_placement: str = "yearly"
    """天马的安法（`TianmaPlacement`）：yearly=年马 / monthly=月马 / both=年马另安月天马"""

    changsheng_direction: str = "yinYang"
    """长生十二神方向（`ChangshengDirection`）：yinYang=阳男阴女顺 / gender=男顺女逆 / clockwise=一律顺行"""

    tianshang_tianshi: str = "default"
    """天伤、天使的安法（`TianshangTianshi`）：default=同 iztro / tianshiFriends=天使仆役、天伤疾厄"""

    tianwu_placement: str = "siShenYinHai"
    """天巫的安法（`TianwuPlacement`）：siShenYinHai=巳申寅亥 / siShenHaiYin=巳申亥寅"""

    yearly12_placement: str = "natalAndYearly"
    """岁前、将前十二神（`Yearly12Placement`）：natalAndYearly=本命与流年都安 / yearlyOnly=只在流年安"""

    original_palace: str = "primary"
    """来因宫的标法（`OriginalPalaceMarking`）：primary=只标一宫 / withSecondary=另标辛年丑宫、壬年子宫为辅来因宫"""

    dou_jun_month: str = "default"
    """推斗君时生月的取法（`DouJunMonth`）：default=同 iztro / natal=与本命盘同一规则"""

    mutagens: dict[str, list[str]] | None = None
    """自定义四化表：天干标识 → 四颗星标识（禄、权、科、忌）。

    按天干整表替换默认值，未列出的天干仍用默认表；键与值都用语言无关标识。
    """

    extra_mutagens: dict[str, dict[str, list[str]]] | None = None
    """附加四化星：天干标识 → 四化标识 → 星耀标识列表。

    加在该天干生效的四化（`mutagens` 或默认表）之外，与四槽之星同化一类，
    如 `{"gengHeavenly": {"sihuaJi": ["tianxiangMaj"]}}` 令庚干天同、天相同化忌。
    同一天干里一颗星至多出现一次（四槽与附加星合计），否则排盘报错。
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
            "voidPlacement": str(self.void_placement),
            "leapDivide": str(self.leap_divide),
            "tianmaPlacement": str(self.tianma_placement),
            "changshengDirection": str(self.changsheng_direction),
            "tianshangTianshi": str(self.tianshang_tianshi),
            "tianwuPlacement": str(self.tianwu_placement),
            "yearly12Placement": str(self.yearly12_placement),
            "originalPalace": str(self.original_palace),
            "douJunMonth": str(self.dou_jun_month),
        }
        if self.mutagens:
            payload["mutagens"] = {
                str(k): [str(v) for v in vs] for k, vs in self.mutagens.items()
            }
        if self.extra_mutagens:
            payload["extraMutagens"] = {
                str(stem): {str(m): [str(v) for v in vs] for m, vs in groups.items()}
                for stem, groups in self.extra_mutagens.items()
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
            void_placement=d.get("voidPlacement", "default"),
            leap_divide=d.get("leapDivide", "default"),
            tianma_placement=d.get("tianmaPlacement", "yearly"),
            changsheng_direction=d.get("changshengDirection", "yinYang"),
            tianshang_tianshi=d.get("tianshangTianshi", "default"),
            tianwu_placement=d.get("tianwuPlacement", "siShenYinHai"),
            yearly12_placement=d.get("yearly12Placement", "natalAndYearly"),
            original_palace=d.get("originalPalace", "primary"),
            dou_jun_month=d.get("douJunMonth", "default"),
        )

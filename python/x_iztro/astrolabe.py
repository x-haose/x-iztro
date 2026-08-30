"""
星盘与出生日期。

星盘是排盘结果的入口：基本信息、十二宫、以及从本盘再发起的重排、运限与
Prompt。十二宫按需构建，构建时回填宫位与星耀对本盘的反向引用。
"""

from __future__ import annotations

import copy
import json
from dataclasses import dataclass, field
from typing import Any

from x_iztro.config import ChartConfig
from x_iztro.enums import PalaceName
from x_iztro.horoscope import Horoscope
from x_iztro.knowledge import KnowledgePack, _wire
from x_iztro.palace import Palace
from x_iztro.pattern import PatternConfig, PatternHit
from x_iztro.star_object import Star
from x_iztro.surpalaces import SurroundedPalaces


@dataclass(frozen=True, slots=True)
class RawLunarDate:
    """数字化农历生日"""

    lunar_year: int
    """农历年"""

    lunar_month: int
    """农历月（1-12，闰月与否见 is_leap）"""

    lunar_day: int
    """农历日（1-30）"""

    is_leap: bool
    """是否闰月"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawLunarDate:
        return cls(
            lunar_year=d["lunarYear"],
            lunar_month=d["lunarMonth"],
            lunar_day=d["lunarDay"],
            is_leap=d["isLeap"],
        )


@dataclass(frozen=True, slots=True)
class RawChineseDate:
    """四柱干支（每柱为 [天干, 地支]，未本地化的干支原文）"""

    yearly: tuple[str, str]
    """年柱"""

    monthly: tuple[str, str]
    """月柱"""

    daily: tuple[str, str]
    """日柱"""

    hourly: tuple[str, str]
    """时柱"""

    yearly_keys: tuple[str, str]
    """年柱的语言无关标识（`HeavenlyStem`、`EarthlyBranch` 枚举值域）"""

    monthly_keys: tuple[str, str]
    """月柱的语言无关标识"""

    daily_keys: tuple[str, str]
    """日柱的语言无关标识"""

    hourly_keys: tuple[str, str]
    """时柱的语言无关标识"""

    def pillar_keys(self) -> list[tuple[str, str]]:
        """四柱标识 [年, 月, 日, 时]，可直接交给 `translate_chinese_date`"""
        return [self.yearly_keys, self.monthly_keys, self.daily_keys, self.hourly_keys]

    @classmethod
    def _from_dict(cls, d: dict) -> RawChineseDate:
        return cls(
            yearly=tuple(d["yearly"]),
            monthly=tuple(d["monthly"]),
            daily=tuple(d["daily"]),
            hourly=tuple(d["hourly"]),
            yearly_keys=tuple(d["yearlyKeys"]),
            monthly_keys=tuple(d["monthlyKeys"]),
            daily_keys=tuple(d["dailyKeys"]),
            hourly_keys=tuple(d["hourlyKeys"]),
        )


@dataclass(frozen=True, slots=True)
class RawDates:
    """结构化的出生日期信息"""

    lunar_date: RawLunarDate
    """数字化农历生日"""

    chinese_date: RawChineseDate
    """四柱干支"""

    @classmethod
    def _from_dict(cls, d: dict) -> RawDates:
        return cls(
            lunar_date=RawLunarDate._from_dict(d["lunarDate"]),
            chinese_date=RawChineseDate._from_dict(d["chineseDate"]),
        )


@dataclass(frozen=True, slots=True)
class Astrolabe:
    """星盘"""

    gender: str
    """性别（按排盘语言翻译）"""

    gender_key: str
    """机器可读性别（`Gender` 枚举值域）"""

    solar_date: str
    """阳历日期"""

    lunar_date: str
    """农历日期"""

    chinese_date: str
    """干支纪年日期"""

    raw_dates: RawDates
    """结构化的农历生日与四柱干支"""

    time: str
    """时辰"""

    time_range: str
    """时辰对应的时间段"""

    sign: str
    """星座"""

    sign_key: str
    """星座的语言无关标识（"aries" … "pisces"）"""

    zodiac: str
    """生肖"""

    zodiac_key: str
    """生肖的语言无关标识（"rat" … "pig"）"""

    earthly_branch_of_soul_palace: str
    """命宫地支（翻译文本）"""

    earthly_branch_of_soul_palace_key: str
    """命宫地支标识（`EarthlyBranch` 枚举值域）"""

    earthly_branch_of_body_palace: str
    """身宫地支（翻译文本）"""

    earthly_branch_of_body_palace_key: str
    """身宫地支标识（`EarthlyBranch` 枚举值域）"""

    soul: str
    """命主星（翻译文本）"""

    soul_key: str
    """命主星标识（星耀 key）"""

    body: str
    """身主星（翻译文本）"""

    body_key: str
    """身主星标识（星耀 key）"""

    five_elements_class: str
    """五行局（翻译文本）"""

    five_elements_class_key: str
    """五行局标识（"water2nd" 等）"""

    time_index: int
    """出生时辰索引 (0-12)"""

    fix_leap: bool
    """是否修正闰月"""

    language: str
    """排盘语言（`Language` 枚举值域）"""

    config: ChartConfig
    """排盘配置（只含六个开关；自定义四化与亮度表见 `_input_config`）"""

    _raw: dict[str, Any] = field(default_factory=dict, compare=False, repr=False)
    """绑定层返回的原始 DTO，惰性构建十二宫与 `to_dict` 均取自它"""

    _input_config: ChartConfig | None = field(default=None, compare=False, repr=False)
    """排盘时用户传入的配置原件。

    `config` 由 DTO 还原，不含自定义四化与亮度表；重排、运限、Prompt 这些
    需要再次发起计算的接口必须沿用原件，否则覆盖表会在第二次调用时丢失。
    """

    _palaces: list[Palace] | None = field(default=None, compare=False, repr=False)
    """十二宫的惰性缓存，首次访问 `palaces` 时构建"""

    _from_stem: str | None = field(default=None, compare=False, repr=False)
    """重排起点天干标识；本盘由 `rearranged` 产生时记录，非重排盘为 None。

    绑定层调用无状态，格局判定等再次发起计算的接口必须转发重排起点，
    让内核先重排再判——否则判的是原盘。
    """

    _from_branch: str | None = field(default=None, compare=False, repr=False)
    """重排起点地支标识，与 `_from_stem` 成对；非重排盘为 None"""

    # ------ 十二宫 ------

    @property
    def palaces(self) -> list[Palace]:
        """
        十二宫数据，按宫位索引排列。

        首次访问时才从原始 DTO 构建并回填宫位、星耀对本盘的反向引用——
        只读日期、命主身主等字段的调用不必为此付出转换开销。
        """
        palaces = self._palaces
        if palaces is None:
            palaces = [Palace._from_dict(p) for p in self._raw["palaces"]]
            object.__setattr__(self, "_palaces", palaces)
            self._link(palaces)
        return palaces

    def to_dict(self) -> dict[str, Any]:
        """
        星盘的 JS iztro 兼容 DTO（camelCase 键，值按排盘语言翻译）的深拷贝。

        导出 JSON 请用本方法或 `to_json`：`dataclasses.asdict()` 会顺着宫位与
        星耀对本盘的回指引用无限递归。
        """
        return copy.deepcopy(self._raw)

    def to_json(self, **kwargs: Any) -> str:
        """
        星盘 DTO 的 JSON 字符串，默认不转义非 ASCII 字符。

        内容与 iztro 的 `JSON.stringify(astrolabe)` 逐键逐值对应。

        Args:
            kwargs: 透传给 `json.dumps`，如 `indent=2`
        """
        kwargs.setdefault("ensure_ascii", False)
        return json.dumps(self._raw, **kwargs)

    def _config_payload(self) -> dict:
        """再次发起计算时使用的配置：优先用户原件，其次由 DTO 还原的开关。"""
        source = self._input_config if self._input_config is not None else self.config
        return source.to_dict()

    def _context_query(self, kind: str, **extra: Any) -> Any:
        """以本盘的排盘上下文（含重排起点）调用绑定层的统一查询入口。

        `extra` 里的 `knowledge`（True / KnowledgePack / None）在这里落成线协议值：
        与本盘同语言的内嵌包只发 "builtin"，其余发包对象——释义类 kind 都经此处，
        转换只维护这一处。
        """
        from x_iztro._bridge import query

        if "knowledge" in extra:
            extra["knowledge"] = _wire(extra["knowledge"], self.language)
        return query(
            kind,
            solar_date=self.solar_date,
            time_index=self.time_index,
            gender=self.gender_key,
            fix_leap=self.fix_leap,
            language=self.language,
            config=self._config_payload(),
            from_stem=self._from_stem,
            from_branch=self._from_branch,
            **extra,
        )

    def to_text(
        self,
        *,
        knowledge: bool | KnowledgePack | None = None,
        config: PatternConfig | None = None,
    ) -> str:
        """
        星盘的语义化文本：面向语言模型与人的完整描述，`str(astrolabe)` 等价于不带释义的形态。

        与 `to_dict`/`to_json`（机器结构）、翻译字段（展示文本）同源，
        是同一张盘的第三种投影。文本是 Markdown 子集（标题、列表、表格），
        按排盘语言输出；重排盘按重排后的布局描述。

        Args:
            knowledge: 释义材料。True 取排盘语言的内嵌知识包，KnowledgePack 用该包
                （自定义或合并后的包）；给出时每宫事实之后紧跟该宫星耀释义、格局之后
                紧跟格局释义，文末另起四化释义一节，不给只输出盘面事实
            config: 格局判定口径，与 `patterns(config)` 同形态；None 取默认口径

        Raises:
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        from x_iztro.pattern import _pattern_config

        return self._context_query(
            "astrolabeToText",
            knowledge=knowledge,
            pattern_config=_pattern_config(config),
        )

    def __str__(self) -> str:
        return self.to_text()

    # ------ 宫位查询 ------

    def palace(self, index_or_name: int | PalaceName | str) -> Palace | None:
        """
        通过索引、宫位标识或当前语言宫名获取宫位。

        Args:
            index_or_name: 宫位索引 (0-11)、`PalaceName` 枚举，或当前语言的宫名
        """
        if isinstance(index_or_name, int):
            if 0 <= index_or_name < len(self.palaces):
                return self.palaces[index_or_name]
            return None
        if index_or_name == PalaceName.BODY:
            return next((p for p in self.palaces if p.is_body_palace), None)
        if index_or_name == PalaceName.ORIGINAL:
            return next((p for p in self.palaces if p.is_original_palace), None)
        for p in self.palaces:
            if index_or_name in (p.name_key, p.name):
                return p
        return None

    def surrounded_palaces(
        self, index_or_name: int | PalaceName | str
    ) -> SurroundedPalaces | None:
        """
        获取指定宫位的三方四正：本宫、对宫、财帛位、官禄位。

        Args:
            index_or_name: 宫位索引 (0-11)、`PalaceName` 枚举，或当前语言的宫名

        Returns:
            SurroundedPalaces；宫位定位不到时返回 None
        """
        palace = self.palace(index_or_name)
        if palace is None:
            return None
        index = palace.index
        return SurroundedPalaces(
            target=self.palaces[index % 12],
            opposite=self.palaces[(index + 6) % 12],
            wealth=self.palaces[(index + 8) % 12],
            career=self.palaces[(index + 4) % 12],
        )

    def is_surrounded(
        self, index_or_name: int | PalaceName | str, stars: list[str]
    ) -> bool:
        """判断指定宫位的三方四正是否包含 **全部** 指定星耀"""
        sp = self.surrounded_palaces(index_or_name)
        return sp.have(stars) if sp else False

    def is_surrounded_one_of(
        self, index_or_name: int | PalaceName | str, stars: list[str]
    ) -> bool:
        """判断指定宫位的三方四正是否包含指定星耀中的 **任意一颗**"""
        sp = self.surrounded_palaces(index_or_name)
        return sp.have_one_of(stars) if sp else False

    def not_surrounded(
        self, index_or_name: int | PalaceName | str, stars: list[str]
    ) -> bool:
        """判断指定宫位的三方四正是否 **一颗都不** 包含指定星耀"""
        sp = self.surrounded_palaces(index_or_name)
        return sp.not_have(stars) if sp else False

    # ------ 星耀查询 ------

    def star(self, star: str) -> Star | None:
        """通过星耀标识（或当前语言星名）查找星耀"""
        found = self.star_in_palace(star)
        return found[0] if found else None

    def star_in_palace(self, star: str) -> tuple[Star, Palace] | None:
        """
        通过星耀标识（或当前语言星名）查找星耀及其所在宫位。

        Returns:
            (Star, Palace) 元组，未找到时返回 None
        """
        for p in self.palaces:
            for s in p.major_stars + p.minor_stars + p.adjective_stars:
                if star in (s.key, s.name):
                    return (s, p)
        return None

    def rearranged(self, from_stem: str, from_branch: str) -> Astrolabe:
        """
        以指定干支为命宫重排本盘，返回新盘；本盘不变。

        传入的干支决定五行局，进而决定紫微天府落点、十二宫名、长生十二神与大限小限；
        辅星、杂耀（天伤天使天才除外）、博士十二神、岁前将前十二神沿用原盘。

        常规的天盘、地盘、人盘用 `ChartConfig(astro_type=...)` 指定即可，
        本方法用于从任意干支起盘。

        Args:
            from_stem: 天干标识（`HeavenlyStem` 枚举值域）
            from_branch: 地支标识（`EarthlyBranch` 枚举值域）

        Raises:
            IztroError: 干支标识非法
        """
        from x_iztro._bridge import by_solar

        data = by_solar(
            solar_date=self.solar_date,
            time_index=self.time_index,
            gender=self.gender_key,
            fix_leap=self.fix_leap,
            language=self.language,
            config=self._config_payload(),
            from_stem=str(from_stem),
            from_branch=str(from_branch),
        )
        chart = Astrolabe._from_dict(data, self._input_config)
        # 记录重排起点，供格局判定等再次发起计算的接口转发（frozen dataclass 构造后写入）
        object.__setattr__(chart, "_from_stem", str(from_stem))
        object.__setattr__(chart, "_from_branch", str(from_branch))
        return chart

    def horoscope(
        self,
        target_date: str | None = None,
        target_time_index: int | None = None,
    ) -> Horoscope:
        """
        以本命盘为起点计算目标日期的运限，结果持有本盘。

        两个参数都可省略，省略时取本地时钟的当前日期与当前时辰。

        Args:
            target_date: 目标阳历日期，如 "2024-1-1"；不传取今天
            target_time_index: 目标时辰索引 (0-12)；不传取此刻所属时辰

        Returns:
            Horoscope 运限对象；其查询方法不必再传星盘

        Raises:
            IztroError: 目标日期或时辰索引非法
        """
        from x_iztro._bridge import horoscope as bridge_horoscope
        from x_iztro.utils import time_to_index

        if target_date is None or target_time_index is None:
            from datetime import datetime

            now = datetime.now()
            if target_date is None:
                target_date = f"{now.year}-{now.month}-{now.day}"
            if target_time_index is None:
                target_time_index = time_to_index(now.hour)

        data = bridge_horoscope(
            solar_date=self.solar_date,
            time_index=self.time_index,
            gender=self.gender_key,
            fix_leap=self.fix_leap,
            language=self.language,
            config=self._config_payload(),
            from_stem=self._from_stem,
            from_branch=self._from_branch,
            target_date=target_date,
            target_time_index=target_time_index,
        )
        return Horoscope._from_dict(data, self, target_time_index)

    def patterns(self, config: PatternConfig | None = None) -> list[PatternHit]:
        """
        本命盘的全部格局命中。

        判定在 Rust 内核完成，规则口径见文档站「格局」一页；
        多口径格局以 `PatternHit.variant` 区分。
        重排盘（`rearranged` 产生）按重排后的布局判定。

        Args:
            config: 判定口径；不传取默认（亮度按 iztro 表、借宫、流曜参与）

        Returns:
            命中列表，按《格局》页条目顺序
        """
        from x_iztro.pattern import _pattern_config

        data = self._context_query("patterns", pattern_config=_pattern_config(config))
        return [PatternHit._from_dict(d) for d in data]

    def patterns_to_text(
        self,
        config: PatternConfig | None = None,
        *,
        knowledge: bool | KnowledgePack | None = None,
    ) -> str:
        """
        本命盘格局命中的语义化文本。

        与 `patterns` 同一套判定（含重排上下文与判定口径），输出面向语言模型
        与人的文本而非结构化命中列表。

        Args:
            config: 判定口径；不传取默认
            knowledge: 释义材料（True 取排盘语言的内嵌包，或给 KnowledgePack）；
                给出时格局列表之后紧跟各格局的释义

        Raises:
            IztroError: `knowledge=True` 而排盘语言没有内嵌包（目前只有 zh-CN）
        """
        from x_iztro.pattern import _pattern_config

        return self._context_query(
            "patternsToText",
            pattern_config=_pattern_config(config),
            knowledge=knowledge,
        )

    def _link(self, palaces: list[Palace]) -> None:
        """
        回填宫位与星耀对星盘的反向引用。

        `Palace.astrolabe()`、`Star.palace()`、按索引或宫名飞星等查询依赖这层引用；
        模型是 frozen dataclass，因此用 `object.__setattr__` 在构造后写入。
        """
        for palace in palaces:
            object.__setattr__(palace, "_astrolabe", self)
            for star in palace.major_stars + palace.minor_stars + palace.adjective_stars:
                object.__setattr__(star, "_palace", palace)
                object.__setattr__(star, "_astrolabe", self)

    @classmethod
    def _from_dict(cls, d: dict, config: ChartConfig | None = None) -> Astrolabe:
        """
        由绑定层的 DTO 构造星盘。

        Args:
            d: 绑定层返回的星盘 DTO
            config: 排盘时用户传入的配置原件，用于后续再次发起计算
        """
        return cls(
            gender=d["gender"],
            gender_key=d["genderKey"],
            solar_date=d["solarDate"],
            lunar_date=d["lunarDate"],
            chinese_date=d["chineseDate"],
            raw_dates=RawDates._from_dict(d["rawDates"]),
            time=d["time"],
            time_range=d["timeRange"],
            sign=d["sign"],
            sign_key=d["signKey"],
            zodiac=d["zodiac"],
            zodiac_key=d["zodiacKey"],
            earthly_branch_of_soul_palace=d["earthlyBranchOfSoulPalace"],
            earthly_branch_of_soul_palace_key=d["earthlyBranchOfSoulPalaceKey"],
            earthly_branch_of_body_palace=d["earthlyBranchOfBodyPalace"],
            earthly_branch_of_body_palace_key=d["earthlyBranchOfBodyPalaceKey"],
            soul=d["soul"],
            soul_key=d["soulKey"],
            body=d["body"],
            body_key=d["bodyKey"],
            five_elements_class=d["fiveElementsClass"],
            five_elements_class_key=d["fiveElementsClassKey"],
            time_index=d["timeIndex"],
            fix_leap=d["fixLeap"],
            language=d["language"],
            config=ChartConfig._from_dict(d["config"]),
            _raw=d,
            _input_config=config,
        )

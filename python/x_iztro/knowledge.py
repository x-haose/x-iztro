"""
知识包：语言无关标识 → 解读文本与门派属性。

内核只负责事实判定，星耀怎么解读、格局意味着什么、宫位与四化的含义属于门派观点，
全部放在知识包里。格式见仓库 `knowledge/SCHEMA.md`；内核内嵌一份默认包
（源自 iztro-docs《学习》页，MIT），使用者可整包替换或用覆盖包逐条合并。

所有键都是 x-iztro 的语言无关标识（`MajorStar`/`PatternKey`/`PalaceName`/`Mutagen` 枚举值域），
文本字段为 Markdown。合并在 Rust 内核完成，三语言规则一致。
"""

from __future__ import annotations

import copy
import json
from dataclasses import dataclass, field
from typing import Any

from x_iztro.enums import LanguageType


@dataclass(frozen=True, slots=True)
class Source:
    """包的来源与许可信息"""

    name: str | None = None
    url: str | None = None
    commit: str | None = None
    license: str | None = None
    author: str | None = None
    retrieved_at: str | None = None

    @classmethod
    def _from_dict(cls, d: dict | None) -> Source:
        d = d or {}
        return cls(
            name=d.get("name"),
            url=d.get("url"),
            commit=d.get("commit"),
            license=d.get("license"),
            author=d.get("author"),
            retrieved_at=d.get("retrievedAt"),
        )


@dataclass(frozen=True, slots=True)
class StarAttributes:
    """星耀的门派属性（全部可选）"""

    yin_yang: str | None = None
    """阴阳（yin / yang）"""
    five_elements: str | None = None
    """五行（wood / fire / earth / metal / water）"""
    stem: str | None = None
    """五行所带天干（jia…gui）"""
    five_elements_note: str | None = None
    """五行的补充说明"""
    dipper: str | None = None
    """斗分"""
    chemistry: str | None = None
    """化气"""
    career: str | None = None
    """职业（主何事）"""
    duty: str | None = None
    """职务"""
    aliases: list[str] | None = None
    """别号"""
    element_color: str | None = None
    """五行色"""
    energy_color: str | None = None
    """能量色"""

    @classmethod
    def _from_dict(cls, d: dict | None) -> StarAttributes:
        d = d or {}
        return cls(
            yin_yang=d.get("yinYang"),
            five_elements=d.get("fiveElements"),
            stem=d.get("stem"),
            five_elements_note=d.get("fiveElementsNote"),
            dipper=d.get("dipper"),
            chemistry=d.get("chemistry"),
            career=d.get("career"),
            duty=d.get("duty"),
            aliases=list(d["aliases"]) if d.get("aliases") is not None else None,
            element_color=d.get("elementColor"),
            energy_color=d.get("energyColor"),
        )


@dataclass(frozen=True, slots=True)
class StarEntry:
    """一颗星耀的知识条目"""

    key: str
    """星耀标识"""
    name: str | None = None
    """该语言的显示名"""
    category: str | None = None
    """类别（major / minor / adjective / dec）"""
    group: str | None = None
    """分组（杂耀的分类、神煞的组别）"""
    attributes: StarAttributes = field(default_factory=StarAttributes)
    """门派属性"""
    intro: str | None = None
    """解读正文"""
    combinations: dict[str, str] = field(default_factory=dict)
    """与另一颗主星同宫的组合解读，键为对方星耀标识"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> StarEntry:
        return cls(
            key=key,
            name=d.get("name"),
            category=d.get("category"),
            group=d.get("group"),
            attributes=StarAttributes._from_dict(d.get("attributes")),
            intro=d.get("intro"),
            combinations=dict(d.get("combinations") or {}),
        )


@dataclass(frozen=True, slots=True)
class PatternEntry:
    """一条格局的知识条目"""

    key: str
    """格局标识"""
    name: str | None = None
    """该语言的显示名"""
    quotes: list[str] | None = None
    """古籍引文"""
    conditions: str | None = None
    """来源对成立条件的文字描述"""
    intro: str | None = None
    """解读正文"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> PatternEntry:
        return cls(
            key=key,
            name=d.get("name"),
            quotes=list(d["quotes"]) if d.get("quotes") is not None else None,
            conditions=d.get("conditions"),
            intro=d.get("intro"),
        )


@dataclass(frozen=True, slots=True)
class TextEntry:
    """只有名称与正文的条目（宫位、四化）"""

    key: str
    """标识"""
    name: str | None = None
    """该语言的显示名"""
    intro: str | None = None
    """正文"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> TextEntry:
        return cls(key=key, name=d.get("name"), intro=d.get("intro"))


@dataclass(frozen=True, slots=True)
class ConceptEntry:
    """术语与基础概念条目"""

    slug: str
    """条目 slug"""
    title: str | None = None
    """标题"""
    intro: str | None = None
    """正文"""

    @classmethod
    def _from_dict(cls, slug: str, d: dict) -> ConceptEntry:
        return cls(slug=slug, title=d.get("title"), intro=d.get("intro"))


class KnowledgePack:
    """
    一份知识包。持有原始 dict（`to_dict()` 深拷贝导出），查询方法返回类型化条目。

    构造：`KnowledgePack.builtin()` 取内嵌默认包，`from_dict` / `from_json` 读自己的包，
    `merged(*overlays)` 叠加覆盖包。
    """

    __slots__ = ("_raw",)

    def __init__(self, raw: dict[str, Any]) -> None:
        self._raw = raw

    # ------ 构造 ------

    @classmethod
    def builtin(cls, language: LanguageType = "zh-CN") -> KnowledgePack:
        """
        内嵌的默认包（源自 iztro-docs，MIT）。

        Raises:
            IztroError: 该语言没有默认包（目前只有 zh-CN）
        """
        from x_iztro._bridge import query

        return cls(query("knowledgePack", language=language))

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> KnowledgePack:
        """由包对象构造（保留引用，不复制）"""
        return cls(d)

    @classmethod
    def from_json(cls, text: str) -> KnowledgePack:
        """由 JSON 文本构造"""
        return cls(json.loads(text))

    def to_dict(self) -> dict[str, Any]:
        """包对象的深拷贝"""
        return copy.deepcopy(self._raw)

    def to_json(self, **kwargs: Any) -> str:
        """包的 JSON 文本；`kwargs` 透传给 `json.dumps`"""
        kwargs.setdefault("ensure_ascii", False)
        return json.dumps(self._raw, **kwargs)

    def merged(self, *overlays: KnowledgePack | dict[str, Any]) -> KnowledgePack:
        """
        以本包为底，依次叠加覆盖包，返回新包（本包不变）。

        合并规则见 knowledge/SCHEMA.md：覆盖包的非空字段覆盖同键条目的对应字段，
        `attributes` / `combinations` 逐字段合并，数组字段整体替换。

        Raises:
            IztroError: 某个包不符合格式或格式版本过新
        """
        from x_iztro._bridge import query

        packs = [self._raw] + [o._raw if isinstance(o, KnowledgePack) else o for o in overlays]
        return KnowledgePack(query("mergeKnowledgePacks", knowledge_packs=packs))

    # ------ 元信息 ------

    @property
    def schema(self) -> int:
        """格式版本"""
        return int(self._raw.get("schema", 0))

    @property
    def id(self) -> str:
        """包标识"""
        return str(self._raw.get("id", ""))

    @property
    def version(self) -> str:
        """包版本"""
        return str(self._raw.get("version", ""))

    @property
    def language(self) -> str:
        """文本语言"""
        return str(self._raw.get("language", ""))

    @property
    def extends(self) -> str | None:
        """覆盖包所覆盖的包标识"""
        return self._raw.get("extends")

    @property
    def source(self) -> Source:
        """来源与许可"""
        return Source._from_dict(self._raw.get("source"))

    # ------ 查询 ------

    def star(self, key: str) -> StarEntry | None:
        """星耀条目（键为星耀标识）"""
        d = (self._raw.get("stars") or {}).get(str(key))
        return StarEntry._from_dict(str(key), d) if d is not None else None

    def pattern(self, key: str) -> PatternEntry | None:
        """格局条目（键为格局标识）"""
        d = (self._raw.get("patterns") or {}).get(str(key))
        return PatternEntry._from_dict(str(key), d) if d is not None else None

    def palace(self, key: str) -> TextEntry | None:
        """宫位条目（键为宫位标识）"""
        d = (self._raw.get("palaces") or {}).get(str(key))
        return TextEntry._from_dict(str(key), d) if d is not None else None

    def mutagen(self, key: str) -> TextEntry | None:
        """四化条目（键为四化标识）"""
        d = (self._raw.get("mutagens") or {}).get(str(key))
        return TextEntry._from_dict(str(key), d) if d is not None else None

    def concept(self, slug: str) -> ConceptEntry | None:
        """术语条目"""
        d = (self._raw.get("concepts") or {}).get(slug)
        return ConceptEntry._from_dict(slug, d) if d is not None else None

    def stars(self) -> list[StarEntry]:
        """全部星耀条目"""
        return [StarEntry._from_dict(k, v) for k, v in (self._raw.get("stars") or {}).items()]

    def patterns(self) -> list[PatternEntry]:
        """全部格局条目"""
        return [PatternEntry._from_dict(k, v) for k, v in (self._raw.get("patterns") or {}).items()]

    def star_intro(self, key: str) -> str | None:
        """星耀解读正文"""
        e = self.star(key)
        return e.intro if e is not None else None

    def pattern_intro(self, key: str) -> str | None:
        """格局解读正文"""
        e = self.pattern(key)
        return e.intro if e is not None else None

    def __repr__(self) -> str:
        return f"KnowledgePack(id={self.id!r}, version={self.version!r}, language={self.language!r})"

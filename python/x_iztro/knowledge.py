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
from typing import TYPE_CHECKING, Any

from x_iztro._bridge import invalid_argument
from x_iztro.enums import KnowledgeIssueCode, LanguageType

if TYPE_CHECKING:
    from x_iztro.astrolabe import Astrolabe
    from x_iztro.horoscope import Horoscope
    from x_iztro.pattern import PatternConfig

_BUILTIN = "builtin"
"""绑定层 `knowledge` 入参的字面量：让内核取盘语言的内嵌包，不搬整包"""


def _wire(knowledge: bool | KnowledgePack | None, language: str) -> str | dict[str, Any] | None:
    """
    把 `knowledge` 参数落成绑定层 `knowledge` 入参：`True` 与「就是 `language` 的内嵌包」
    的实例都发 `"builtin"`（内核自取，避免每次搬整包），其他实例发包对象，None/False 不发。

    Raises:
        TypeError: 既不是布尔也不是 KnowledgePack
    """
    if knowledge is None or knowledge is False:
        return None
    if knowledge is True:
        return _BUILTIN
    if isinstance(knowledge, KnowledgePack):
        return _BUILTIN if knowledge._builtin_language == language else knowledge._raw
    raise TypeError(f"knowledge 须为 True 或 KnowledgePack，收到 {type(knowledge).__name__}")


@dataclass(frozen=True, slots=True)
class KnowledgeIssue:
    """知识包校验的一条问题（`KnowledgePack.check`）"""

    level: str
    """`"error"`：违反规范的「必须 / 不得」或内核用到该处时会失败，构造知识包时拒绝；
    `"warning"`：「应 / 不应」、被丢弃的字段、启发式匹配与相对底包的提醒，只报告"""

    path: tuple[str | int, ...]
    """问题所在的 JSON 路径：对象键为 str、数组下标为 int；空元组指包本身"""

    code: KnowledgeIssueCode
    """问题种类，出处与 `params` 的键见文档站「知识包 · 校验」"""

    params: dict[str, Any]
    """补充数据，如 `{"key": "ziweiMjj", "suggestion": "ziweiMaj"}`"""

    message: str
    """带路径的英文说明，供日志；界面应按 `code` 与 `params` 组织文案"""

    @classmethod
    def _from_dict(cls, d: dict[str, Any]) -> KnowledgeIssue:
        return cls(
            level=d["level"],
            path=tuple(d["path"]),
            code=KnowledgeIssueCode(d["code"]),
            params=d["params"],
            message=d["message"],
        )


def _rejected(issues: list[KnowledgeIssue]) -> None:
    """有 error 级问题时抛 `IztroError`（code 为 invalid_argument），全部问题挂在 `issues` 属性上"""
    errors = [i for i in issues if i.level == "error"]
    if errors:
        err = invalid_argument("invalid knowledge pack: " + "; ".join(i.message for i in errors))
        err.issues = issues
        raise err


@dataclass(frozen=True, slots=True)
class Source:
    """包的来源与许可信息

    Attributes:
        name: 来源名称（如 iztro-docs）
        url: 来源仓库或页面地址
        commit: 提取时锁定的来源 commit
        license: 来源的许可协议（如 MIT）
        author: 来源作者
        retrieved_at: 提取日期（YYYY-MM-DD）
        adapted: 改写说明——文本经过整理改写时注明
    """

    name: str | None = None
    url: str | None = None
    commit: str | None = None
    license: str | None = None
    author: str | None = None
    retrieved_at: str | None = None
    adapted: str | None = None

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
            adapted=d.get("adapted"),
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
    """类别（major / minor / adjective / dec / flow）"""
    group: str | None = None
    """分组（杂耀的分类、神煞的组别）"""
    attributes: StarAttributes = field(default_factory=StarAttributes)
    """门派属性"""
    intro: str | None = None
    """解读正文"""
    summary: str | None = None
    """正文的摘要（单段）；正文不超过摘要层上限（显示宽度 240，中文 120 字）的条目不写"""
    combinations: dict[str, str] = field(default_factory=dict)
    """与另一颗主星同宫的组合解读，键为对方星耀标识"""
    combination_summaries: dict[str, str] = field(default_factory=dict)
    """组合解读的摘要，与 `combinations` 同键成对"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> StarEntry:
        return cls(
            key=key,
            name=d.get("name"),
            category=d.get("category"),
            group=d.get("group"),
            attributes=StarAttributes._from_dict(d.get("attributes")),
            intro=d.get("intro"),
            summary=d.get("summary"),
            combinations=dict(d.get("combinations") or {}),
            combination_summaries=dict(d.get("combinationSummaries") or {}),
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
    summary: str | None = None
    """正文的摘要（单段）；正文不超过摘要层上限的条目不写"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> PatternEntry:
        return cls(
            key=key,
            name=d.get("name"),
            quotes=list(d["quotes"]) if d.get("quotes") is not None else None,
            conditions=d.get("conditions"),
            intro=d.get("intro"),
            summary=d.get("summary"),
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
    summary: str | None = None
    """正文的摘要（单段）；正文不超过摘要层上限的条目不写"""

    @classmethod
    def _from_dict(cls, key: str, d: dict) -> TextEntry:
        return cls(key=key, name=d.get("name"), intro=d.get("intro"), summary=d.get("summary"))


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


@dataclass(frozen=True, slots=True)
class TopicEntry:
    """解读主题（如婚恋、财运）：读这类问题要看哪些宫，由 `Astrolabe.topic_to_text` 按它选宫"""

    slug: str
    """主题 slug"""
    title: str | None = None
    """显示名"""
    surround: list[str] | None = None
    """连同各自三方四正一起取的宫位标识，按出现顺序渲染"""
    palaces: list[str] | None = None
    """只取本宫的宫位标识（可含 `bodyPalace` / `originalPalace`），排在 `surround` 各组之后"""
    intro: str | None = None
    """这个主题看什么、为什么取这些宫"""
    summary: str | None = None
    """正文的摘要（单段）；正文不超过摘要层上限的条目不写"""

    @classmethod
    def _from_dict(cls, slug: str, d: dict) -> TopicEntry:
        return cls(
            slug=slug,
            title=d.get("title"),
            surround=list(d["surround"]) if d.get("surround") is not None else None,
            palaces=list(d["palaces"]) if d.get("palaces") is not None else None,
            intro=d.get("intro"),
            summary=d.get("summary"),
        )


class KnowledgePack:
    """
    一份知识包。持有原始 dict（`to_dict()` 深拷贝导出），查询方法返回类型化条目。

    构造：`KnowledgePack.builtin()` 取内嵌默认包，`from_dict` / `from_json` 读自己的包，
    `merged(*overlays)` 叠加覆盖包，`for_astrolabe` / `for_horoscope` 按盘取材成子包。
    """

    __slots__ = ("_raw", "_builtin_language")

    def __init__(self, raw: dict[str, Any], *, builtin_language: str | None = None) -> None:
        self._raw = raw
        # 本实例就是该语言的内嵌包时记语言，传给内核时只发 "builtin" 而不搬整包；
        # 自定义包与合并结果为 None
        self._builtin_language = builtin_language

    # ------ 构造 ------

    @classmethod
    def builtin(cls, language: LanguageType = "zh-CN") -> KnowledgePack:
        """
        内嵌的默认包（源自 iztro-docs，MIT）。

        Raises:
            IztroError: 该语言没有默认包（目前只有 zh-CN）
        """
        from x_iztro._bridge import query

        return cls(query("knowledgePack", language=language), builtin_language=language)

    def for_astrolabe(
        self, chart: Astrolabe, config: PatternConfig | None = None
    ) -> KnowledgePack:
        """
        按本命盘取材的子包：只含盘上出现的星（十二宫主辅杂与四组十二神；同宫主星的
        双星组合解读保留，不同宫的剔除）、按 `config` 口径命中的格局、四化四条；
        不含宫位与术语条目。元信息沿用本包，结果仍是标准包，可再合并、序列化或按 key 查。
        重排盘（`rearranged` 产生）按重排后的布局取材。

        Args:
            config: 格局判定口径，与 `Astrolabe.patterns` 同一参数；不传取默认
        """
        from x_iztro.pattern import _pattern_config

        return KnowledgePack(
            chart._context_query(
                "knowledgeForChart", knowledge=self, pattern_config=_pattern_config(config)
            )
        )

    def for_horoscope(
        self, horoscope: Horoscope, config: PatternConfig | None = None
    ) -> KnowledgePack:
        """
        按运限取材的子包：在 `for_astrolabe` 之上再加各层流耀的条目与各层视角命中的格局，
        只取到运限的查询精度（`Horoscope.precision()`）那一层。

        Args:
            config: 格局判定口径，与 `Horoscope.patterns` 同一参数；不传取默认

        Raises:
            ValueError: 运限不是由星盘发起（脱离星盘无排盘上下文可转发）
        """
        from x_iztro.pattern import _pattern_config

        return KnowledgePack(
            horoscope._context_query(
                "knowledgeForChart", None, knowledge=self, pattern_config=_pattern_config(config)
            )
        )

    @classmethod
    def from_dict(cls, d: dict[str, Any]) -> KnowledgePack:
        """
        由包对象构造（保留引用，不复制）。先经 `check` 校验，有 error 级问题即拒绝。

        Raises:
            IztroError: 有 error 级问题（code 为 invalid_argument）；全部问题在异常的 `issues` 属性里
            TypeError: `d` 含 JSON 无法表示的值
        """
        _rejected(cls.check(d))
        return cls(d)

    @classmethod
    def from_json(cls, text: str) -> KnowledgePack:
        """
        由 JSON 文本构造，校验同 `from_dict`。

        Raises:
            IztroError: 有 error 级问题（code 为 invalid_argument）；全部问题在异常的 `issues` 属性里
            json.JSONDecodeError: JSON 语法非法
        """
        return cls.from_dict(json.loads(text))

    @staticmethod
    def check(raw: Any, *, base: KnowledgePack | None = None) -> list[KnowledgeIssue]:
        """
        逐条校验一份包，一次返回全部问题（不在第一处停）。输入不是对象等也作为 error 级问题返回。

        Args:
            raw: 待校验的包对象（通常是 `json.load` 的结果）
            base: 它要叠加的底包；给出时另报相对底包的提醒（空串覆盖底包的值、语言或 `extends`
                与底包不符），摘要规则按合并后的条目判断

        Raises:
            TypeError: `raw` 含 JSON 无法表示的值，或 `base` 不是 KnowledgePack
        """
        from x_iztro._bridge import query

        if base is not None and not isinstance(base, KnowledgePack):
            raise TypeError(f"base 须为 KnowledgePack，收到 {type(base).__name__}")
        params: dict[str, Any] = {"pack_json": json.dumps(raw, ensure_ascii=False)}
        if base is not None:
            params["base_pack"] = base._raw
        return [KnowledgeIssue._from_dict(i) for i in query("checkKnowledgePack", **params)]

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
        `attributes` / `combinations` 逐字段合并，数组字段整体替换；覆盖条目换了正文却没给
        对应摘要时，合并结果去掉描述旧正文的摘要。

        Raises:
            IztroError: 某个覆盖包对象有 error 级问题（同 `from_dict`）
        """
        from x_iztro._bridge import query

        packs = [self._raw] + [
            (o if isinstance(o, KnowledgePack) else KnowledgePack.from_dict(o))._raw for o in overlays
        ]
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

    def topic(self, slug: str) -> TopicEntry | None:
        """主题条目"""
        d = (self._raw.get("topics") or {}).get(slug)
        return TopicEntry._from_dict(slug, d) if d is not None else None

    def topics(self) -> list[TopicEntry]:
        """全部主题条目"""
        return [TopicEntry._from_dict(k, v) for k, v in (self._raw.get("topics") or {}).items()]

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

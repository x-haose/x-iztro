//! 知识包校验：逐条列出包里不符合 `knowledge/SCHEMA.md` 的地方，带 JSON 路径。
//!
//! 规则只取自 SCHEMA.md 的规定与 `tests/knowledge_pack.rs` 对默认包的断言，每个
//! [`IssueCode`] 的文档写明出处。级别按规范措辞定：违反「必须 / 不得」、或内核用到该处时会失败的为
//! [`IssueLevel::Error`]，构造知识包（[`KnowledgePack::from_value`] 等）遇到即拒绝；「应 / 不应」、
//! 解析时被静默丢弃的字段、按字面匹配的启发式规则与相对底包的提醒为 [`IssueLevel::Warning`]，
//! 只报告不拒绝。
//!
//! 校验在 JSON 值上逐字段进行，一次返回全部问题；输入不是对象、JSON 非法也作为问题返回。

use std::fmt;

use serde_json::{Map, Value, json};
use unicode_width::UnicodeWidthStr;

use super::{
    KnowledgePack, SCHEMA_VERSION, SUMMARY_FALLBACK_WIDTH, drop_stale_summaries, merge_json,
};
use crate::data::keyed::keyed_enum;
use crate::data::stars::StarKey;
use crate::data::types::{Language, Mutagen, Palace};
use crate::pattern::PatternKey;

/// 摘要的宽度上限（SCHEMA §5.1：单段、宽度不超过 100，中文 50 字）。
pub const SUMMARY_MAX_WIDTH: usize = 100;

/// 问题的级别。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IssueLevel {
    /// 违反规范的「必须 / 不得」，或内核用到该处时会失败；构造知识包时拒绝
    Error,
    /// 「应 / 不应」、被静默丢弃的字段、启发式匹配与相对底包的提醒；只报告
    Warning,
}

keyed_enum!(IssueLevel, "issue level" {
    Error => "error",
    Warning => "warning",
});

/// 问题的种类。每种的级别固定（[`IssueCode::level`]），`params` 的键见各变体说明。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum IssueCode {
    /// 文本不是合法 JSON（§7「拒绝非法 JSON」）。params：`message`
    InvalidJson,
    /// 包或条目不是 JSON 对象（§2「包是一个 JSON 对象」）
    NotObject,
    /// `schema` 缺失、为 null 或 0（§2、§7「缺失或为 0 的包必须被拒绝」）
    SchemaMissing,
    /// `schema` 大于本库支持的版本（§7）。params：`value`、`supported`
    SchemaUnsupported,
    /// 字段类型与规范不符（§2、§4 各表的「类型」列）。params：`expected`（`string` / `integer` /
    /// `object` / `array` / `string or null`）
    WrongType,
    /// 必须的顶层字段 `id` / `version` / `language` 缺失或为空串（§2「必须」）
    RequiredMissing,
    /// `language` 不是 x-iztro 语言码（§2）。params：`value`，可能有 `suggestion`
    UnknownLanguage,
    /// 条目键不是内核认识的标识（§3；默认包测试逐段断言键合法）。params：`key`，可能有 `suggestion`
    UnknownKey,
    /// 双星组合的键是星耀标识但不是主星（§4.1「键为对方主星的星耀标识」）。params：`key`
    CombinationKeyNotMajor,
    /// 取值不在规定集合内：星耀 `category`（§4.1）、`yinYang` / `fiveElements` / `stem`（§4.1.1）。
    /// params：`value`、`allowed`，可能有 `suggestion`
    InvalidValue,
    /// 术语或主题的 slug 不是小写字母、数字与连字符（§4.4、§4.5）。params：`key`
    InvalidSlug,
    /// 主题的宫位不是宫位标识、`bodyPalace` 或 `originalPalace`（§4.5；生成主题文本时即报错）。
    /// params：`value`，可能有 `suggestion`
    UnknownPalace,
    /// 正文含 HTML 标签（§5「不得使用 HTML 标签」）。params：`snippet`
    HtmlInText,
    /// 正文含 Markdown 图片（§5「不得使用图片」）。params：`snippet`
    ImageInText,
    /// 正文含站外链接（§5「不得使用站外链接」；裸网址在 Markdown 渲染时同样成为链接）。params：`url`
    ExternalLink,
    /// 规范之外的字段：解析时静默丢弃，合并后不存在（§2、§7 实现行为）。params：可能有 `suggestion`
    UnknownField,
    /// 字段为空字符串（§4「不得用空字符串表达缺省」；能否用空串表达「有值」只有写包者知道）
    EmptyString,
    /// 正文宽度超过 240 却没有摘要（§5.1「应」；请求摘要层时内核据此报错）。params：`width`、`limit`
    SummaryMissing,
    /// 正文宽度不超过 240 却另写了摘要（§5.1「不应」）。params：`width`、`limit`
    SummaryNotNeeded,
    /// 摘要宽度超过 100（§5.1「应」）。params：`width`、`limit`
    SummaryTooWide,
    /// 摘要不是单段（§5.1「应为单段」）
    SummaryMultiParagraph,
    /// 摘要没有对应的正文：条目无 `intro`，或 `combinationSummaries` 的键不在 `combinations` 里
    /// （§5.1 正文与摘要成对；默认包测试断言组合摘要只对应已有组合）
    SummaryWithoutBody,
    /// 正文依赖页面上下文，如「见上表」「点击」（§5「不得依赖页面上下文」；按字面匹配）。params：`phrase`
    PageReference,
    /// 主题一个宫都不取（默认包测试断言每个主题至少取一宫；否则主题文本只有说明）
    TopicWithoutPalaces,
    /// 以空字符串覆盖底包里有值的字段（§4、§6.1：空串视为有值并覆盖底包）；给了底包时才报。params：`base`
    OverridesWithEmpty,
    /// 语言与底包不同（§6.2：合并结果取覆盖包的语言，文本会混用两种语言）；给了底包时才报。
    /// params：`value`、`base`
    LanguageDiffersFromBase,
    /// `extends` 与底包的 `id` 不同（§2：`extends` 声明底包的 `id`）；给了底包时才报。params：`value`、`base`
    ExtendsDiffersFromBase,
}

keyed_enum!(IssueCode, "knowledge issue code" {
    InvalidJson => "invalid_json",
    NotObject => "not_object",
    SchemaMissing => "schema_missing",
    SchemaUnsupported => "schema_unsupported",
    WrongType => "wrong_type",
    RequiredMissing => "required_missing",
    UnknownLanguage => "unknown_language",
    UnknownKey => "unknown_key",
    CombinationKeyNotMajor => "combination_key_not_major",
    InvalidValue => "invalid_value",
    InvalidSlug => "invalid_slug",
    UnknownPalace => "unknown_palace",
    HtmlInText => "html_in_text",
    ImageInText => "image_in_text",
    ExternalLink => "external_link",
    UnknownField => "unknown_field",
    EmptyString => "empty_string",
    SummaryMissing => "summary_missing",
    SummaryNotNeeded => "summary_not_needed",
    SummaryTooWide => "summary_too_wide",
    SummaryMultiParagraph => "summary_multi_paragraph",
    SummaryWithoutBody => "summary_without_body",
    PageReference => "page_reference",
    TopicWithoutPalaces => "topic_without_palaces",
    OverridesWithEmpty => "overrides_with_empty",
    LanguageDiffersFromBase => "language_differs_from_base",
    ExtendsDiffersFromBase => "extends_differs_from_base",
});

impl IssueCode {
    /// 该种问题的级别
    pub fn level(self) -> IssueLevel {
        use IssueCode::*;
        match self {
            InvalidJson
            | NotObject
            | SchemaMissing
            | SchemaUnsupported
            | WrongType
            | RequiredMissing
            | UnknownLanguage
            | UnknownKey
            | CombinationKeyNotMajor
            | InvalidValue
            | InvalidSlug
            | UnknownPalace
            | HtmlInText
            | ImageInText
            | ExternalLink => IssueLevel::Error,
            UnknownField
            | EmptyString
            | SummaryMissing
            | SummaryNotNeeded
            | SummaryTooWide
            | SummaryMultiParagraph
            | SummaryWithoutBody
            | PageReference
            | TopicWithoutPalaces
            | OverridesWithEmpty
            | LanguageDiffersFromBase
            | ExtendsDiffersFromBase => IssueLevel::Warning,
        }
    }
}

/// JSON 路径的一段：对象的键或数组的下标。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    /// 对象的键
    Key(String),
    /// 数组的下标（从 0 起）
    Index(usize),
}

impl From<&str> for PathSegment {
    fn from(key: &str) -> Self {
        PathSegment::Key(key.to_string())
    }
}

impl From<usize> for PathSegment {
    fn from(index: usize) -> Self {
        PathSegment::Index(index)
    }
}

/// 一条校验问题。
#[derive(Debug, Clone, PartialEq)]
pub struct KnowledgeIssue {
    /// 级别，等于 `code.level()`
    pub level: IssueLevel,
    /// 问题所在的 JSON 路径；空路径指包本身
    pub path: Vec<PathSegment>,
    /// 问题种类
    pub code: IssueCode,
    /// 补充数据，键见 [`IssueCode`] 各变体说明
    pub params: Map<String, Value>,
}

impl KnowledgeIssue {
    /// 路径的点分写法，如 `stars.ziweiMaj.combinations.tianfuMaj`、`patterns.x.quotes[2]`；空路径为 `(pack)`
    pub fn path_string(&self) -> String {
        let mut out = String::new();
        for seg in &self.path {
            match seg {
                PathSegment::Key(k) => {
                    if !out.is_empty() {
                        out.push('.');
                    }
                    out.push_str(k);
                }
                PathSegment::Index(i) => out.push_str(&format!("[{i}]")),
            }
        }
        if out.is_empty() {
            out.push_str("(pack)");
        }
        out
    }

    /// 英文说明（不含路径），供日志与报错信息；界面应按 `code` 与 `params` 自行组织文案
    pub fn message(&self) -> String {
        use IssueCode::*;
        let p = |k: &str| match self.params.get(k) {
            Some(Value::String(s)) => s.clone(),
            Some(v) => v.to_string(),
            None => String::new(),
        };
        let hint = match self.params.get("suggestion") {
            Some(Value::String(s)) => format!(" (did you mean '{s}'?)"),
            _ => String::new(),
        };
        match self.code {
            InvalidJson => format!("invalid JSON: {}", p("message")),
            NotObject => "expected a JSON object".to_string(),
            SchemaMissing => "knowledge pack must declare \"schema\" (currently 1)".to_string(),
            SchemaUnsupported => format!(
                "knowledge pack schema {} is newer than supported {}",
                p("value"),
                p("supported")
            ),
            WrongType => format!("expected {}", p("expected")),
            RequiredMissing => "required field is missing or empty".to_string(),
            UnknownLanguage => format!("unknown language '{}'{hint}", p("value")),
            UnknownKey => format!("unknown key '{}'{hint}", p("key")),
            CombinationKeyNotMajor => format!("'{}' is not a major star", p("key")),
            InvalidValue => format!("invalid value '{}'{hint}", p("value")),
            InvalidSlug => format!(
                "slug '{}' must use lowercase letters, digits and hyphens",
                p("key")
            ),
            UnknownPalace => format!("unknown palace '{}'{hint}", p("value")),
            HtmlInText => format!("HTML is not allowed: {}", p("snippet")),
            ImageInText => format!("images are not allowed: {}", p("snippet")),
            ExternalLink => format!("external links are not allowed: {}", p("url")),
            UnknownField => format!("unknown field, dropped when parsed{hint}"),
            EmptyString => "empty string counts as a value; omit the field instead".to_string(),
            SummaryMissing => format!(
                "body is {} wide (over {}) and needs a summary",
                p("width"),
                p("limit")
            ),
            SummaryNotNeeded => format!(
                "body is only {} wide (at most {}); the summary layer uses it directly",
                p("width"),
                p("limit")
            ),
            SummaryTooWide => format!("summary is {} wide (over {})", p("width"), p("limit")),
            SummaryMultiParagraph => "summary should be a single paragraph".to_string(),
            SummaryWithoutBody => "summary has no matching body".to_string(),
            PageReference => format!("text refers to page context: '{}'", p("phrase")),
            TopicWithoutPalaces => "topic selects no palace".to_string(),
            OverridesWithEmpty => format!("empty string overrides the base value '{}'", p("base")),
            LanguageDiffersFromBase => format!(
                "language '{}' differs from the base pack's '{}'",
                p("value"),
                p("base")
            ),
            ExtendsDiffersFromBase => format!(
                "extends '{}' differs from the base pack id '{}'",
                p("value"),
                p("base")
            ),
        }
    }
}

impl fmt::Display for KnowledgeIssue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.path_string(), self.message())
    }
}

/// 校验一份包的 JSON 值；`base` 为它要叠加的底包时，另报相对底包的提醒，摘要规则按合并后的条目判断。
pub fn check(raw: &Value, base: Option<&KnowledgePack>) -> Vec<KnowledgeIssue> {
    let base_value = base.map(|b| serde_json::to_value(b).expect("知识包只含普通可序列化字段"));
    let mut c = Checker {
        issues: Vec::new(),
        path: Vec::new(),
        base,
        base_value: base_value.as_ref(),
        overlay_without_base: false,
    };
    c.pack(raw);
    c.issues
}

/// 校验 JSON 文本：非法 JSON 作为 [`IssueCode::InvalidJson`] 返回，其余同 [`check`]。
pub fn check_json(json: &str, base: Option<&KnowledgePack>) -> Vec<KnowledgeIssue> {
    match serde_json::from_str::<Value>(json) {
        Ok(v) => check(&v, base),
        Err(e) => vec![issue(
            Vec::new(),
            IssueCode::InvalidJson,
            json!({ "message": e.to_string() }),
        )],
    }
}

fn issue(path: Vec<PathSegment>, code: IssueCode, params: Value) -> KnowledgeIssue {
    KnowledgeIssue {
        level: code.level(),
        path,
        code,
        params: match params {
            Value::Object(m) => m,
            _ => Map::new(),
        },
    }
}

/// 顶层字段（§2）。
const TOP_FIELDS: &[&str] = &[
    "schema", "id", "version", "language", "extends", "source", "stars", "patterns", "palaces",
    "mutagens", "concepts", "topics",
];
/// `source` 的字段（§2.1）。
const SOURCE_FIELDS: &[&str] = &[
    "name",
    "url",
    "commit",
    "license",
    "author",
    "retrievedAt",
    "adapted",
];
/// 星耀 `category` 的取值（§4.1）。
const CATEGORIES: &[&str] = &["major", "minor", "adjective", "dec", "flow"];
/// `attributes.yinYang` 的取值（§4.1.1）。
const YIN_YANG: &[&str] = &["yin", "yang"];
/// `attributes.fiveElements` 的取值（§4.1.1）。
const FIVE_ELEMENTS: &[&str] = &["wood", "fire", "earth", "metal", "water"];
/// `attributes.stem` 的取值（§4.1.1）。
const STEMS: &[&str] = &[
    "jia", "yi", "bing", "ding", "wu", "ji", "geng", "xin", "ren", "gui",
];
/// 主题宫位里除宫名标识外的写法（§4.5）。
const PALACE_TARGET_EXTRAS: &[&str] = &["bodyPalace", "originalPalace"];
/// §5「不得依赖页面上下文」的字面线索：取默认包口吻红线（`tests/knowledge_pack.rs`）里指向页面布局与
/// 站点功能的那部分。红线里其余的口语与对话残留是默认包「第三人称释义口吻」（§8）的写法要求，
/// 规范没有要求用户包采用，不在此列。
const PAGE_REFERENCES: &[&str] = &[
    "上面的名片表",
    "见上表",
    "上表中",
    "如上表",
    "见下表",
    "下表中",
    "从上面",
    "前面说的",
    "前面的逻辑",
    "前面提到",
    "详细介绍见",
    "详见",
    "参见",
    "本站",
    "点击",
];

/// 条目字段的种类。
#[derive(Clone, Copy)]
enum Field {
    /// 普通字符串（名称、标题、属性说明）
    Plain,
    /// 正文类字符串，按 §5 检查：`intro`、`summary`、`conditions`（§5 列出的正文类字段）
    Text,
    /// 取值受限的字符串
    Enum(&'static [&'static str]),
    /// 字符串数组（别号、古籍引文；引文是逐字转录的纯文本，不在 §5 正文类字段之列）
    Strings,
    /// 门派属性对象（§4.1.1）
    Attributes,
    /// 键为对方主星、值为正文的映射（双星组合与其摘要）
    Combinations,
    /// 主题的宫位数组
    PalaceTargets,
}

const STAR_FIELDS: &[(&str, Field)] = &[
    ("name", Field::Plain),
    ("category", Field::Enum(CATEGORIES)),
    ("group", Field::Plain),
    ("attributes", Field::Attributes),
    ("intro", Field::Text),
    ("summary", Field::Text),
    ("combinations", Field::Combinations),
    ("combinationSummaries", Field::Combinations),
];
const ATTRIBUTE_FIELDS: &[(&str, Field)] = &[
    ("yinYang", Field::Enum(YIN_YANG)),
    ("fiveElements", Field::Enum(FIVE_ELEMENTS)),
    ("stem", Field::Enum(STEMS)),
    ("fiveElementsNote", Field::Plain),
    ("dipper", Field::Plain),
    ("chemistry", Field::Plain),
    ("career", Field::Plain),
    ("duty", Field::Plain),
    ("aliases", Field::Strings),
    ("elementColor", Field::Plain),
    ("energyColor", Field::Plain),
];
const PATTERN_FIELDS: &[(&str, Field)] = &[
    ("name", Field::Plain),
    ("quotes", Field::Strings),
    ("conditions", Field::Text),
    ("intro", Field::Text),
    ("summary", Field::Text),
];
const TEXT_ENTRY_FIELDS: &[(&str, Field)] = &[
    ("name", Field::Plain),
    ("intro", Field::Text),
    ("summary", Field::Text),
];
const CONCEPT_FIELDS: &[(&str, Field)] = &[("title", Field::Plain), ("intro", Field::Text)];
const TOPIC_FIELDS: &[(&str, Field)] = &[
    ("title", Field::Plain),
    ("surround", Field::PalaceTargets),
    ("palaces", Field::PalaceTargets),
    ("intro", Field::Text),
    ("summary", Field::Text),
];

/// 条目段：段名、键的合法集合（`None` 为 slug）、条目字段表、是否有摘要规则。
struct Section {
    name: &'static str,
    keys: Option<&'static [&'static str]>,
    fields: &'static [(&'static str, Field)],
    summaries: bool,
}

const SECTIONS: &[Section] = &[
    Section {
        name: "stars",
        keys: Some(StarKey::KEYS),
        fields: STAR_FIELDS,
        summaries: true,
    },
    Section {
        name: "patterns",
        keys: Some(PatternKey::KEYS),
        fields: PATTERN_FIELDS,
        summaries: true,
    },
    Section {
        name: "palaces",
        keys: Some(Palace::KEYS),
        fields: TEXT_ENTRY_FIELDS,
        summaries: true,
    },
    Section {
        name: "mutagens",
        keys: Some(Mutagen::KEYS),
        fields: TEXT_ENTRY_FIELDS,
        summaries: true,
    },
    Section {
        name: "concepts",
        keys: None,
        fields: CONCEPT_FIELDS,
        summaries: false,
    },
    Section {
        name: "topics",
        keys: None,
        fields: TOPIC_FIELDS,
        summaries: true,
    },
];

/// 主星的星耀标识：iztro 的主星标识以 `Maj` 结尾（辅星 `Min`）。
fn major_star_keys() -> Vec<&'static str> {
    StarKey::KEYS
        .iter()
        .copied()
        .filter(|k| k.ends_with("Maj"))
        .collect()
}

fn palace_target_keys() -> Vec<&'static str> {
    Palace::KEYS
        .iter()
        .chain(PALACE_TARGET_EXTRAS)
        .copied()
        .collect()
}

struct Checker<'a> {
    issues: Vec<KnowledgeIssue>,
    path: Vec<PathSegment>,
    base: Option<&'a KnowledgePack>,
    base_value: Option<&'a Value>,
    /// 声明了 `extends` 却没给底包：只凭这一份判断不了正文与摘要是否成对
    overlay_without_base: bool,
}

impl Checker<'_> {
    fn report(&mut self, code: IssueCode, params: Value) {
        self.issues.push(issue(self.path.clone(), code, params));
    }

    fn at<R>(&mut self, seg: impl Into<PathSegment>, f: impl FnOnce(&mut Self) -> R) -> R {
        self.path.push(seg.into());
        let r = f(self);
        self.path.pop();
        r
    }

    fn pack(&mut self, raw: &Value) {
        let Some(obj) = raw.as_object() else {
            self.report(IssueCode::NotObject, Value::Null);
            return;
        };
        self.unknown_fields(obj, TOP_FIELDS);
        self.schema(obj.get("schema"));
        for field in ["id", "version"] {
            self.at(field, |c| {
                c.required_string(obj.get(field));
            });
        }
        self.at("language", |c| {
            if let Some(lang) = c.required_string(obj.get("language")) {
                c.language(lang);
            }
        });
        self.at("extends", |c| c.extends(obj.get("extends")));
        self.overlay_without_base =
            self.base.is_none() && obj.get("extends").is_some_and(Value::is_string);
        self.at("source", |c| c.source(obj.get("source")));
        for section in SECTIONS {
            self.at(section.name, |c| c.section(section, obj.get(section.name)));
        }
    }

    fn schema(&mut self, v: Option<&Value>) {
        self.at("schema", |c| match v {
            None | Some(Value::Null) => c.report(IssueCode::SchemaMissing, Value::Null),
            Some(Value::Number(n)) => match n.as_u64() {
                Some(0) => c.report(IssueCode::SchemaMissing, Value::Null),
                Some(s) if s > u64::from(SCHEMA_VERSION) => c.report(
                    IssueCode::SchemaUnsupported,
                    json!({ "value": s, "supported": SCHEMA_VERSION }),
                ),
                Some(_) => {}
                None => c.report(IssueCode::WrongType, json!({ "expected": "integer" })),
            },
            Some(_) => c.report(IssueCode::WrongType, json!({ "expected": "integer" })),
        });
    }

    /// 必须的非空字符串（§2）；返回取值供后续规则使用。
    fn required_string<'v>(&mut self, v: Option<&'v Value>) -> Option<&'v str> {
        match v {
            None | Some(Value::Null) => {
                self.report(IssueCode::RequiredMissing, Value::Null);
                None
            }
            Some(Value::String(s)) if s.is_empty() => {
                self.report(IssueCode::RequiredMissing, Value::Null);
                None
            }
            Some(Value::String(s)) => Some(s),
            Some(_) => {
                self.report(IssueCode::WrongType, json!({ "expected": "string" }));
                None
            }
        }
    }

    fn language(&mut self, lang: &str) {
        if Language::from_key(lang).is_none() {
            let suggestion = Language::from_code(lang)
                .map(Language::as_key)
                .or_else(|| closest(lang, Language::KEYS));
            self.report(
                IssueCode::UnknownLanguage,
                with_suggestion(json!({ "value": lang }), suggestion),
            );
            return;
        }
        if let Some(base) = self.base
            && !base.language.is_empty()
            && base.language != lang
        {
            self.report(
                IssueCode::LanguageDiffersFromBase,
                json!({ "value": lang, "base": base.language }),
            );
        }
    }

    fn extends(&mut self, v: Option<&Value>) {
        match v {
            None | Some(Value::Null) => {}
            Some(Value::String(s)) if s.is_empty() => self.empty_string(),
            Some(Value::String(s)) => {
                if let Some(base) = self.base
                    && !base.id.is_empty()
                    && base.id != *s
                {
                    self.report(
                        IssueCode::ExtendsDiffersFromBase,
                        json!({ "value": s, "base": base.id }),
                    );
                }
            }
            Some(_) => self.report(
                IssueCode::WrongType,
                json!({ "expected": "string or null" }),
            ),
        }
    }

    fn source(&mut self, v: Option<&Value>) {
        let obj = match v {
            None | Some(Value::Null) => return,
            Some(Value::Object(o)) => o,
            Some(_) => {
                self.report(IssueCode::WrongType, json!({ "expected": "object" }));
                return;
            }
        };
        self.unknown_fields(obj, SOURCE_FIELDS);
        for field in SOURCE_FIELDS {
            if let Some(v) = obj.get(*field) {
                self.at(*field, |c| c.plain(v));
            }
        }
    }

    fn section(&mut self, section: &Section, v: Option<&Value>) {
        let obj = match v {
            None | Some(Value::Null) => return,
            Some(Value::Object(o)) => o,
            Some(_) => {
                self.report(IssueCode::WrongType, json!({ "expected": "object" }));
                return;
            }
        };
        for (key, entry) in obj {
            self.at(key.as_str(), |c| {
                match section.keys {
                    Some(keys) if !keys.contains(&key.as_str()) => c.report(
                        IssueCode::UnknownKey,
                        with_suggestion(json!({ "key": key }), closest(key, keys)),
                    ),
                    Some(_) => {}
                    None if !is_slug(key) => {
                        c.report(IssueCode::InvalidSlug, json!({ "key": key }));
                    }
                    None => {}
                }
                let Some(fields) = entry.as_object() else {
                    c.report(IssueCode::WrongType, json!({ "expected": "object" }));
                    return;
                };
                c.entry(section.fields, fields);
                if section.summaries || section.name == "topics" {
                    let effective = c.effective_entry(section.name, key, entry);
                    if section.summaries {
                        c.summaries(&effective);
                    }
                    if section.name == "topics" {
                        c.topic_palaces(&effective);
                    }
                }
            });
        }
    }

    /// 给了底包时，条目按 §6.1 合并到底包同键条目上的结果；否则为条目本身。
    fn effective_entry(&self, section: &str, key: &str, entry: &Value) -> Value {
        let Some(base_entry) = self
            .base_value
            .and_then(|b| b.get(section))
            .and_then(|s| s.get(key))
        else {
            return entry.clone();
        };
        // 合并时 null 等同缺省、保留底包值（§6.1），先去掉再按值合并
        let overlay = without_nulls(entry);
        let mut merged = base_entry.clone();
        merge_json(&mut merged, &overlay);
        drop_stale_summaries(&mut merged, base_entry, &overlay);
        merged
    }

    fn entry(&mut self, spec: &[(&'static str, Field)], obj: &Map<String, Value>) {
        let names: Vec<&str> = spec.iter().map(|(n, _)| *n).collect();
        self.unknown_fields(obj, &names);
        for (name, kind) in spec {
            if let Some(v) = obj.get(*name) {
                self.at(*name, |c| c.field(*kind, v));
            }
        }
    }

    fn field(&mut self, kind: Field, v: &Value) {
        if v.is_null() {
            return;
        }
        match kind {
            Field::Plain => self.plain(v),
            Field::Text => {
                if let Some(s) = self.plain_str(v) {
                    self.text(s);
                }
            }
            Field::Enum(allowed) => {
                if let Some(s) = self.plain_str(v)
                    && !s.is_empty()
                    && !allowed.contains(&s)
                {
                    self.report(
                        IssueCode::InvalidValue,
                        with_suggestion(
                            json!({ "value": s, "allowed": allowed }),
                            closest(s, allowed),
                        ),
                    );
                }
            }
            Field::Strings => {
                let Some(items) = v.as_array() else {
                    self.report(IssueCode::WrongType, json!({ "expected": "array" }));
                    return;
                };
                for (i, item) in items.iter().enumerate() {
                    self.at(i, |c| match item {
                        Value::String(s) if s.is_empty() => c.empty_string(),
                        Value::String(_) => {}
                        _ => c.report(IssueCode::WrongType, json!({ "expected": "string" })),
                    });
                }
            }
            Field::Attributes => {
                let Some(obj) = v.as_object() else {
                    self.report(IssueCode::WrongType, json!({ "expected": "object" }));
                    return;
                };
                self.entry(ATTRIBUTE_FIELDS, obj);
            }
            Field::Combinations => {
                let Some(obj) = v.as_object() else {
                    self.report(IssueCode::WrongType, json!({ "expected": "object" }));
                    return;
                };
                let majors = major_star_keys();
                for (other, text) in obj {
                    self.at(other.as_str(), |c| {
                        if StarKey::from_key(other).is_none() {
                            c.report(
                                IssueCode::UnknownKey,
                                with_suggestion(json!({ "key": other }), closest(other, &majors)),
                            );
                        } else if !majors.contains(&other.as_str()) {
                            c.report(IssueCode::CombinationKeyNotMajor, json!({ "key": other }));
                        }
                        if !text.is_null()
                            && let Some(s) = c.plain_str(text)
                        {
                            c.text(s);
                        }
                    });
                }
            }
            Field::PalaceTargets => {
                let Some(items) = v.as_array() else {
                    self.report(IssueCode::WrongType, json!({ "expected": "array" }));
                    return;
                };
                let targets = palace_target_keys();
                for (i, item) in items.iter().enumerate() {
                    self.at(i, |c| match item.as_str() {
                        Some(s) if !targets.contains(&s) => c.report(
                            IssueCode::UnknownPalace,
                            with_suggestion(json!({ "value": s }), closest(s, &targets)),
                        ),
                        Some(_) => {}
                        None => c.report(IssueCode::WrongType, json!({ "expected": "string" })),
                    });
                }
            }
        }
    }

    /// 字符串字段：类型不符或为空串时报告。
    fn plain(&mut self, v: &Value) {
        if !v.is_null() {
            self.plain_str(v);
        }
    }

    fn plain_str<'v>(&mut self, v: &'v Value) -> Option<&'v str> {
        match v {
            Value::String(s) if s.is_empty() => {
                self.empty_string();
                None
            }
            Value::String(s) => Some(s),
            _ => {
                self.report(IssueCode::WrongType, json!({ "expected": "string" }));
                None
            }
        }
    }

    /// 空串：底包同一路径有非空值时报覆盖，否则报空串本身。
    fn empty_string(&mut self) {
        let base_value = self.base_value.and_then(|b| lookup(b, &self.path));
        match base_value {
            Some(Value::String(s)) if !s.is_empty() => {
                let s = s.clone();
                self.report(IssueCode::OverridesWithEmpty, json!({ "base": s }));
            }
            _ => self.report(IssueCode::EmptyString, Value::Null),
        }
    }

    /// §5 文本规则：HTML、图片、站外链接、页面上下文。每条规则在一个字段里只报第一处。
    fn text(&mut self, s: &str) {
        // 行内代码原样显示、不被渲染，其中的 `<T>`、网址与字样都不算
        let s = &mask_code_spans(s);
        if let Some(tag) = find_html(s) {
            self.report(IssueCode::HtmlInText, json!({ "snippet": tag }));
        }
        if let Some(img) = find_image(s) {
            self.report(IssueCode::ImageInText, json!({ "snippet": img }));
        }
        if let Some(url) = find_url(s) {
            self.report(IssueCode::ExternalLink, json!({ "url": url }));
        }
        if let Some(phrase) = PAGE_REFERENCES.iter().find(|p| s.contains(*p)) {
            self.report(IssueCode::PageReference, json!({ "phrase": phrase }));
        }
    }

    /// §5.1 摘要规则，作用于（合并后的）条目。
    fn summaries(&mut self, entry: &Value) {
        let body = entry.get("intro").and_then(Value::as_str);
        let summary = entry.get("summary").and_then(Value::as_str);
        self.summary_pair(body, summary, "summary");
        let empty = Map::new();
        let bodies = entry
            .get("combinations")
            .and_then(Value::as_object)
            .unwrap_or(&empty);
        let summaries = entry
            .get("combinationSummaries")
            .and_then(Value::as_object)
            .unwrap_or(&empty);
        for (other, text) in bodies {
            if let Some(text) = text.as_str() {
                let s = summaries.get(other).and_then(Value::as_str);
                self.at("combinations", |c| {
                    c.at(other.as_str(), |c| c.summary_pair(Some(text), s, ""))
                });
            }
        }
        for (other, s) in summaries {
            if let Some(s) = s.as_str()
                && !bodies.get(other).is_some_and(Value::is_string)
            {
                self.at("combinationSummaries", |c| {
                    c.at(other.as_str(), |c| {
                        if !c.overlay_without_base {
                            c.report(IssueCode::SummaryWithoutBody, Value::Null);
                        }
                        c.summary_shape(s, "");
                    })
                });
            }
        }
    }

    /// 一对正文与摘要；`summary_field` 为摘要所在的字段名（组合解读的摘要在另一个映射里，传空串，
    /// 摘要类问题的路径落在 `combinationSummaries.<键>`）。
    fn summary_pair(&mut self, body: Option<&str>, summary: Option<&str>, summary_field: &str) {
        let limit = SUMMARY_FALLBACK_WIDTH;
        match (body, summary) {
            (Some(b), None) if b.width() > limit => self.report(
                IssueCode::SummaryMissing,
                json!({ "width": b.width(), "limit": limit }),
            ),
            (Some(b), Some(s)) => {
                if b.width() <= limit {
                    self.summary_at(summary_field, |c| {
                        c.report(
                            IssueCode::SummaryNotNeeded,
                            json!({ "width": b.width(), "limit": limit }),
                        )
                    });
                }
                self.summary_shape(s, summary_field);
            }
            (None, Some(s)) => {
                if !self.overlay_without_base {
                    self.summary_at(summary_field, |c| {
                        c.report(IssueCode::SummaryWithoutBody, Value::Null)
                    });
                }
                self.summary_shape(s, summary_field);
            }
            _ => {}
        }
    }

    fn summary_shape(&mut self, s: &str, summary_field: &str) {
        if s.width() > SUMMARY_MAX_WIDTH {
            self.summary_at(summary_field, |c| {
                c.report(
                    IssueCode::SummaryTooWide,
                    json!({ "width": s.width(), "limit": SUMMARY_MAX_WIDTH }),
                )
            });
        }
        if s.contains('\n') {
            self.summary_at(summary_field, |c| {
                c.report(IssueCode::SummaryMultiParagraph, Value::Null)
            });
        }
    }

    /// 摘要问题的路径：条目的摘要字段，或（组合解读）把当前的 `combinations.<键>` 换成
    /// `combinationSummaries.<键>`。
    fn summary_at(&mut self, summary_field: &str, f: impl FnOnce(&mut Self)) {
        if !summary_field.is_empty() {
            self.at(summary_field, f);
            return;
        }
        let n = self.path.len();
        let saved = self.path[n - 2].clone();
        self.path[n - 2] = PathSegment::Key("combinationSummaries".to_string());
        f(self);
        self.path[n - 2] = saved;
    }

    fn topic_palaces(&mut self, entry: &Value) {
        if self.overlay_without_base {
            return;
        }
        let any = ["surround", "palaces"].iter().any(|f| {
            entry
                .get(*f)
                .and_then(Value::as_array)
                .is_some_and(|a| !a.is_empty())
        });
        if !any {
            self.report(IssueCode::TopicWithoutPalaces, Value::Null);
        }
    }

    fn unknown_fields(&mut self, obj: &Map<String, Value>, known: &[&str]) {
        for key in obj.keys() {
            if !known.contains(&key.as_str()) {
                self.at(key.as_str(), |c| {
                    c.report(
                        IssueCode::UnknownField,
                        with_suggestion(json!({}), closest(key, known)),
                    )
                });
            }
        }
    }
}

/// 去掉对象里（逐层）值为 null 的键。
fn without_nulls(v: &Value) -> Value {
    match v {
        Value::Object(m) => Value::Object(
            m.iter()
                .filter(|(_, v)| !v.is_null())
                .map(|(k, v)| (k.clone(), without_nulls(v)))
                .collect(),
        ),
        other => other.clone(),
    }
}

/// 按路径取 JSON 值。
fn lookup<'v>(root: &'v Value, path: &[PathSegment]) -> Option<&'v Value> {
    path.iter().try_fold(root, |v, seg| match seg {
        PathSegment::Key(k) => v.get(k.as_str()),
        PathSegment::Index(i) => v.get(*i),
    })
}

fn with_suggestion(mut params: Value, suggestion: Option<&str>) -> Value {
    if let (Some(s), Value::Object(m)) = (suggestion, &mut params) {
        m.insert("suggestion".to_string(), json!(s));
    }
    params
}

/// §4.4：slug 由小写字母、数字与连字符组成。
fn is_slug(key: &str) -> bool {
    !key.is_empty()
        && key
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

/// 最接近 `word` 的候选，供「是不是想写」提示。
///
/// 先找只差大小写的候选；否则取编辑距离最小者，距离用 Damerau-Levenshtein 的受限形式
/// （插入、删除、替换、相邻换位各计 1——换位是标识拼写里最常见的手误），上限为
/// `max(字符数, 3) / 3`，与 rustc 给「未找到名字」提示时的阈值相同；超过上限不给提示，
/// 免得把无关的词硬凑成建议。同距离取候选表里靠前的。
pub(crate) fn closest<'k>(word: &str, candidates: &[&'k str]) -> Option<&'k str> {
    if let Some(c) = candidates.iter().find(|c| c.eq_ignore_ascii_case(word)) {
        return Some(c);
    }
    let limit = word.chars().count().max(3) / 3;
    candidates
        .iter()
        .map(|c| (osa_distance(word, c), *c))
        .filter(|(d, _)| *d <= limit)
        .min_by_key(|(d, _)| *d)
        .map(|(_, c)| c)
}

/// 受限 Damerau-Levenshtein 距离（optimal string alignment），按 Unicode 标量值计。
fn osa_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (n, m) = (a.len(), b.len());
    let mut d = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in d[0].iter_mut().enumerate() {
        *cell = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut v = (d[i - 1][j] + 1)
                .min(d[i][j - 1] + 1)
                .min(d[i - 1][j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                v = v.min(d[i - 2][j - 2] + 1);
            }
            d[i][j] = v;
        }
    }
    d[n][m]
}

/// 把行内代码（CommonMark：一串反引号到下一串等长反引号之间）连同反引号换成空格。
/// 前面有未被转义的反斜杠的反引号（`\``）是字面的反引号，不作开头；行内代码不跨段落，
/// 遇到空行（只含空白的行）仍未配对即按原文保留。
fn mask_code_spans(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let mut out = chars.clone();
    let run_len = |at: usize| chars[at..].iter().take_while(|c| **c == '`').count();
    // `at` 处的字符前有奇数个反斜杠，即被转义
    let escaped = |at: usize| chars[..at].iter().rev().take_while(|c| **c == '\\').count() % 2 == 1;
    // `at` 处是换行，且下一行只含空白（段落在此结束）
    let blank_line_at = |at: usize| {
        chars[at] == '\n'
            && chars[at + 1..]
                .iter()
                .take_while(|c| **c != '\n')
                .all(|c| c.is_whitespace())
            && chars[at + 1..].contains(&'\n')
    };
    let mut i = 0;
    while i < chars.len() {
        if chars[i] != '`' {
            i += 1;
            continue;
        }
        if escaped(i) {
            i += 1;
            continue;
        }
        let n = run_len(i);
        let mut j = i + n;
        let mut close = None;
        while j < chars.len() {
            if chars[j] == '`' {
                let m = run_len(j);
                if m == n {
                    close = Some(j);
                    break;
                }
                j += m;
            } else if blank_line_at(j) {
                break;
            } else {
                j += 1;
            }
        }
        match close {
            Some(j) => {
                for c in &mut out[i..j + n] {
                    *c = ' ';
                }
                i = j + n;
            }
            None => i += n,
        }
    }
    out.into_iter().collect()
}

/// 截取片段供参数展示：最多 40 个字符。
fn snippet(s: &str) -> String {
    s.chars().take(40).collect()
}

/// HTML 标签或注释：`<` 后紧跟字母（或 `/字母`、`!--`），到下一个 `>` 为止且其间没有 `<` 与换行。
fn find_html(s: &str) -> Option<String> {
    for (i, _) in s.match_indices('<') {
        let rest = &s[i + 1..];
        let start_ok = rest.starts_with("!--")
            || rest
                .strip_prefix('/')
                .unwrap_or(rest)
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_alphabetic());
        if !start_ok {
            continue;
        }
        if let Some(end) = rest.find('>') {
            let inner = &rest[..end];
            if !inner.contains('<') && !inner.contains('\n') {
                return Some(snippet(&s[i..i + 1 + end + 1]));
            }
        }
    }
    None
}

/// Markdown 图片 `![说明](地址)`。
fn find_image(s: &str) -> Option<String> {
    for (i, _) in s.match_indices("![") {
        let rest = &s[i..];
        let line = rest.split('\n').next().unwrap_or(rest);
        if let Some(close) = line.find("](")
            && let Some(end) = line[close..].find(')')
        {
            return Some(snippet(&line[..close + end + 1]));
        }
    }
    None
}

/// 站外链接：`http://`、`https://` 开头的网址（Markdown 链接、自动链接与裸网址都会渲染成链接），
/// 以及 `](//` 形式的协议相对链接。
fn find_url(s: &str) -> Option<String> {
    let lower = s.to_ascii_lowercase();
    let start = ["http://", "https://"]
        .iter()
        .filter_map(|p| lower.find(p))
        .min()
        .or_else(|| lower.find("](//").map(|i| i + 2))?;
    let url: String = s[start..]
        .chars()
        .take_while(|c| {
            !c.is_whitespace() && !matches!(c, ')' | '>' | '"' | '\'' | '）' | '，' | '。')
        })
        .collect();
    Some(snippet(&url))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closest_uses_transposition_and_a_third_of_the_length() {
        assert_eq!(closest("ziweiMjj", StarKey::KEYS), Some("ziweiMaj"));
        assert_eq!(closest("ziwieMaj", StarKey::KEYS), Some("ziweiMaj"));
        assert_eq!(closest("ZIWEIMAJ", StarKey::KEYS), Some("ziweiMaj"));
        assert_eq!(closest("intor", &["intro", "summary"]), Some("intro"));
        assert_eq!(closest("xyz", &["intro", "summary"]), None);
        assert_eq!(osa_distance("ab", "ba"), 1);
        assert_eq!(osa_distance("kitten", "sitting"), 3);
    }

    #[test]
    fn code_spans_are_masked() {
        assert_eq!(mask_code_spans("a `<b>` c"), "a       c");
        assert_eq!(
            mask_code_spans("``x ` y`` z"),
            format!("{}z", " ".repeat(10))
        );
        assert_eq!(mask_code_spans("未闭合 `<b>"), "未闭合 `<b>");
        // 转义的反引号不作开头
        let s = "单独一个 \\` 号，<b>粗</b> 与 `x``";
        assert!(mask_code_spans(s).contains("<b>"));
        assert_eq!(find_html(&mask_code_spans(s)).as_deref(), Some("<b>"));
        assert_eq!(mask_code_spans("a\\`b` `c`"), "a\\`b   c`");
        // 不跨空行配对
        assert_eq!(mask_code_spans("`a\n\n<b>` x"), "`a\n\n<b>` x");
        assert_eq!(mask_code_spans("`a\nb` x"), format!("{}x", " ".repeat(6)));
    }

    #[test]
    fn text_scanners_find_the_first_occurrence() {
        assert_eq!(find_html("a <b>粗</b>").as_deref(), Some("<b>"));
        assert_eq!(find_html("1 < 2 > 0"), None);
        assert_eq!(find_html("x<!-- c -->").as_deref(), Some("<!-- c -->"));
        assert_eq!(
            find_image("见 ![图](a.png) 后").as_deref(),
            Some("![图](a.png)")
        );
        assert_eq!(find_image("![没有地址]"), None);
        assert_eq!(
            find_url("详 [x](https://e.com/a) 止").as_deref(),
            Some("https://e.com/a")
        );
        assert_eq!(find_url("<HTTP://E.COM>").as_deref(), Some("HTTP://E.COM"));
        assert_eq!(find_url("[x](//e.com)").as_deref(), Some("//e.com"));
        assert_eq!(find_url("纯文本"), None);
    }
}

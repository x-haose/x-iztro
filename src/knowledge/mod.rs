//! 知识包：语言无关标识 → 解读文本与门派属性。
//!
//! 内核只负责事实判定，星耀怎么解读、格局意味着什么、宫位与四化的含义属于门派观点，
//! 全部放在知识包里。格式见仓库 `knowledge/SCHEMA.md`；内核内嵌一份默认包
//! （源自 iztro-docs《学习》页，MIT），使用者可整包替换或用覆盖包逐条合并。
//!
//! 所有键都是 x-iztro 的语言无关标识：`StarKey::as_key`、`PatternKey::as_key`、
//! `Palace::as_key`、`Mutagen::as_key`；文本字段为 Markdown。

use std::collections::BTreeMap;
use std::sync::OnceLock;

mod check;

pub use check::{
    IssueCode, IssueLevel, KnowledgeIssue, PathSegment, SUMMARY_MAX_WIDTH, check, check_json,
};

use serde::{Deserialize, Serialize};
use unicode_width::UnicodeWidthStr;

use crate::data::stars::StarKey;
use crate::data::types::{HOROSCOPE_SCOPES, Language, Mutagen, Palace};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::HoroscopeRef;
use crate::models::palace::PalaceData;
use crate::pattern::{PatternConfig, PatternKey, patterns_at};

/// 一宫之内出现的全部星耀标识：主星、辅星、杂耀与四组十二神各一位。
pub(crate) fn palace_star_keys(palace: &PalaceData) -> impl Iterator<Item = StarKey> + '_ {
    palace
        .major_stars
        .iter()
        .chain(&palace.minor_stars)
        .chain(&palace.adjective_stars)
        .map(|s| s.key)
        .chain([
            palace.changsheng12,
            palace.boshi12,
            palace.suiqian12,
            palace.jiangqian12,
        ])
}

/// 当前支持的知识包格式版本。
pub const SCHEMA_VERSION: u32 = 1;

/// 摘要层的正文上限，单位是显示宽度（UAX #11：宽字符与全角字符计 2，其余计 1；中文 120 字）。
/// 不超过它的正文本身就是摘要长度，条目不另写摘要；超过它的正文要在摘要层出现，条目必须带摘要。
pub const SUMMARY_FALLBACK_WIDTH: usize = 240;

/// 一段正文在摘要层的取材：有摘要取摘要，否则正文不超过 [`SUMMARY_FALLBACK_WIDTH`] 时取正文本身；
/// 正文过长又无摘要返回 `None`。
fn brief<'a>(body: &'a str, summary: Option<&'a str>) -> Option<&'a str> {
    summary.or_else(|| (body.width() <= SUMMARY_FALLBACK_WIDTH).then_some(body))
}

/// 包的来源与许可信息。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Source {
    /// 来源名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 来源地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 来源版本（如 git commit）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commit: Option<String>,
    /// 许可证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<String>,
    /// 作者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    /// 取得日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieved_at: Option<String>,
    /// 改写说明：文本经过整理改写时注明（如「文本由 x-iztro 在原文基础上整理改写」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adapted: Option<String>,
}

/// 星耀的门派属性（全部可选）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct StarAttributes {
    /// 阴阳（`yin` / `yang`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yin_yang: Option<String>,
    /// 五行（`wood` / `fire` / `earth` / `metal` / `water`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub five_elements: Option<String>,
    /// 五行所带天干（`jia`…`gui`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stem: Option<String>,
    /// 五行的补充说明（如「气为水」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub five_elements_note: Option<String>,
    /// 斗分（如「中天星系」「南斗第三星」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dipper: Option<String>,
    /// 化气（如「尊贵」「善」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chemistry: Option<String>,
    /// 职业（主何事，如「官禄主」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub career: Option<String>,
    /// 职务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty: Option<String>,
    /// 别号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    /// 五行色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub element_color: Option<String>,
    /// 能量色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_color: Option<String>,
}

/// 一颗星耀的知识条目。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct StarEntry {
    /// 该语言的显示名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 类别（`major` / `minor` / `adjective` / `dec` / `flow`）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// 分组（杂耀的分类、神煞的组别）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// 门派属性
    #[serde(deserialize_with = "null_as_default")]
    pub attributes: StarAttributes,
    /// 解读正文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 正文的摘要（单段）；正文不超过 [`SUMMARY_FALLBACK_WIDTH`] 的条目不写，摘要层直接取正文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 与另一颗主星同宫的组合解读，键为对方星耀标识
    #[serde(
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "null_as_default"
    )]
    pub combinations: BTreeMap<String, String>,
    /// 组合解读的摘要，与 `combinations` 同键成对
    #[serde(
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "null_as_default"
    )]
    pub combination_summaries: BTreeMap<String, String>,
}

impl StarEntry {
    /// 摘要层的解读：摘要，或不超过上限的正文。包已通过 [`KnowledgePack::missing_summaries`]
    /// 校验时，`None` 只表示没有正文。
    pub fn brief(&self) -> Option<&str> {
        brief(self.intro.as_deref()?, self.summary.as_deref())
    }

    /// 与对方主星 `other`（星耀标识）同宫的组合解读在摘要层的取材，规则同 [`Self::brief`]。
    pub fn combination_brief(&self, other: &str) -> Option<&str> {
        brief(
            self.combinations.get(other)?,
            self.combination_summaries.get(other).map(String::as_str),
        )
    }
}

/// 一条格局的知识条目。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PatternEntry {
    /// 该语言的显示名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 古籍引文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quotes: Option<Vec<String>>,
    /// 来源对成立条件的文字描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<String>,
    /// 解读正文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 正文的摘要（单段）；正文不超过 [`SUMMARY_FALLBACK_WIDTH`] 的条目不写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl PatternEntry {
    /// 摘要层的解读，规则同 [`StarEntry::brief`]；成立条件不在摘要层。
    pub fn brief(&self) -> Option<&str> {
        brief(self.intro.as_deref()?, self.summary.as_deref())
    }
}

/// 只有名称与正文的条目（宫位、四化）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TextEntry {
    /// 该语言的显示名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 正文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 正文的摘要（单段）；正文不超过 [`SUMMARY_FALLBACK_WIDTH`] 的条目不写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl TextEntry {
    /// 摘要层的正文，规则同 [`StarEntry::brief`]。
    pub fn brief(&self) -> Option<&str> {
        brief(self.intro.as_deref()?, self.summary.as_deref())
    }
}

/// 术语与基础概念条目。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConceptEntry {
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 正文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
}

/// 一个解读主题（如婚恋、财运）：读这类问题要看哪些宫。主题取哪些宫属于论法，故放在包里由门派表态，
/// 内核只按定义选宫渲染（[`crate::text::topic_to_text`]）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct TopicEntry {
    /// 显示名（如「婚恋」）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 连同各自三方四正一起取的宫位，宫位标识（`Palace::as_key`），按出现顺序渲染
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surround: Option<Vec<String>>,
    /// 只取本宫的宫位：宫位标识，或 `bodyPalace` / `originalPalace`；排在 `surround` 各组之后
    #[serde(skip_serializing_if = "Option::is_none")]
    pub palaces: Option<Vec<String>>,
    /// 这个主题看什么、为什么取这些宫
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<String>,
    /// 正文的摘要（单段）；正文不超过 [`SUMMARY_FALLBACK_WIDTH`] 的条目不写
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
}

impl TopicEntry {
    /// 摘要层的说明，规则同 [`StarEntry::brief`]。
    pub fn brief(&self) -> Option<&str> {
        brief(self.intro.as_deref()?, self.summary.as_deref())
    }
}

/// 一份知识包。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct KnowledgePack {
    /// 格式版本
    pub schema: u32,
    /// 包标识
    pub id: String,
    /// 包版本
    pub version: String,
    /// 文本语言（x-iztro 语言码，如 `zh-CN`）
    pub language: String,
    /// 覆盖包所覆盖的包标识；独立包为 `None`
    pub extends: Option<String>,
    /// 来源与许可
    #[serde(deserialize_with = "null_as_default")]
    pub source: Source,
    /// 星耀条目，键为 `StarKey::as_key`
    #[serde(deserialize_with = "null_as_default")]
    pub stars: BTreeMap<String, StarEntry>,
    /// 格局条目，键为 `PatternKey::as_key`
    #[serde(deserialize_with = "null_as_default")]
    pub patterns: BTreeMap<String, PatternEntry>,
    /// 宫位条目，键为 `Palace::as_key`
    #[serde(deserialize_with = "null_as_default")]
    pub palaces: BTreeMap<String, TextEntry>,
    /// 四化条目，键为 `Mutagen::as_key`
    #[serde(deserialize_with = "null_as_default")]
    pub mutagens: BTreeMap<String, TextEntry>,
    /// 术语与基础概念，键为 slug
    #[serde(deserialize_with = "null_as_default")]
    pub concepts: BTreeMap<String, ConceptEntry>,
    /// 解读主题，键为 slug
    #[serde(deserialize_with = "null_as_default")]
    pub topics: BTreeMap<String, TopicEntry>,
}

/// 允许 JSON 里把映射类字段写成 `null`（等同缺省），便于各语言的默认序列化产物直接互通。
fn null_as_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    Ok(Option::<T>::deserialize(d)?.unwrap_or_default())
}

/// 内嵌默认包的原文（zh-CN，源自 iztro-docs）。
static BUILTIN_ZH_CN: &str = include_str!("../data/knowledge/iztro_docs.zh-CN.json");

impl KnowledgePack {
    /// 内嵌的默认包；该语言没有默认包时返回 `None`（目前只有 zh-CN）。
    pub fn builtin(language: Language) -> Option<&'static KnowledgePack> {
        static ZH_CN: OnceLock<KnowledgePack> = OnceLock::new();
        match language {
            Language::ZhCN => Some(ZH_CN.get_or_init(|| {
                serde_json::from_str(BUILTIN_ZH_CN).expect("内嵌默认知识包与格式一致")
            })),
            _ => None,
        }
    }

    /// 内嵌默认包的 JSON 原文，供绑定层直接透传。
    pub fn builtin_json(language: Language) -> Option<&'static str> {
        match language {
            Language::ZhCN => Some(BUILTIN_ZH_CN),
            _ => None,
        }
    }

    /// 内嵌默认包解析出的 JSON 值，供绑定层透传；每种语言一个缓存位，
    /// 缓存与语言分支同处，新增内嵌语言包不会漏掉缓存分键。
    pub fn builtin_value(language: Language) -> Option<&'static serde_json::Value> {
        static ZH_CN: OnceLock<serde_json::Value> = OnceLock::new();
        match language {
            Language::ZhCN => Some(ZH_CN.get_or_init(|| {
                serde_json::from_str(BUILTIN_ZH_CN).expect("内嵌默认知识包与格式一致")
            })),
            _ => None,
        }
    }

    /// 由 JSON 解析一份包。
    ///
    /// # Errors
    /// JSON 非法，或 [`check`] 报出 [`IssueLevel::Error`] 级问题（缺 schema、字段类型不符、未知标识、
    /// 正文含 HTML 等）时返回错误，信息逐条列出全部 error 级问题及其路径。
    pub fn from_json(json: &str) -> Result<KnowledgePack, String> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| format!("invalid knowledge pack: invalid JSON: {e}"))?;
        Self::from_value(&value)
    }

    /// 由已解析的 JSON 值解析一份包（绑定层免字符串往返，也不复制值树）；校验同 [`Self::from_json`]。
    ///
    /// # Errors
    /// 同 [`Self::from_json`]。
    pub fn from_value(value: &serde_json::Value) -> Result<KnowledgePack, String> {
        let errors: Vec<String> = check(value, None)
            .into_iter()
            .filter(|i| i.level == IssueLevel::Error)
            .map(|i| i.to_string())
            .collect();
        if !errors.is_empty() {
            return Err(format!("invalid knowledge pack: {}", errors.join("; ")));
        }
        // check 覆盖了全部字段的类型；仍解析失败说明两者脱节，作为内部错误报出
        KnowledgePack::deserialize(value).map_err(|e| {
            format!("internal: knowledge pack passed the checks but failed to parse: {e}")
        })
    }

    /// 校验一份包的 JSON 值，一次返回全部问题；详见 [`check`]。
    pub fn check(raw: &serde_json::Value, base: Option<&KnowledgePack>) -> Vec<KnowledgeIssue> {
        check(raw, base)
    }

    /// 序列化为 JSON。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("知识包只含普通可序列化字段")
    }

    /// 把覆盖包合并到本包：逐段按键合并，覆盖包的非空字段覆盖同键条目的对应字段，
    /// `attributes` 与 `combinations` 逐字段合并，数组字段整体替换；覆盖条目换了正文
    /// （`intro` 或某条组合解读）却没给对应摘要时，去掉合并结果里描述旧正文的摘要。
    /// 合并后 `id` / `version` / `language` / `source` 取覆盖包的（若非空），`extends` 保留本包的。
    pub fn merge(&mut self, overlay: &KnowledgePack) {
        for (k, e) in &overlay.stars {
            merge_entry(self.stars.entry(k.clone()).or_default(), e);
        }
        for (k, e) in &overlay.patterns {
            merge_entry(self.patterns.entry(k.clone()).or_default(), e);
        }
        for (k, e) in &overlay.palaces {
            merge_entry(self.palaces.entry(k.clone()).or_default(), e);
        }
        for (k, e) in &overlay.mutagens {
            merge_entry(self.mutagens.entry(k.clone()).or_default(), e);
        }
        for (k, e) in &overlay.concepts {
            merge_entry(self.concepts.entry(k.clone()).or_default(), e);
        }
        for (k, e) in &overlay.topics {
            merge_entry(self.topics.entry(k.clone()).or_default(), e);
        }
        if !overlay.id.is_empty() {
            self.id = overlay.id.clone();
        }
        if !overlay.version.is_empty() {
            self.version = overlay.version.clone();
        }
        if !overlay.language.is_empty() {
            self.language = overlay.language.clone();
        }
        if overlay.source != Source::default() {
            self.source = overlay.source.clone();
        }
    }

    /// 本包叠加若干覆盖包后的新包（本包不变）。
    pub fn merged(&self, overlays: &[&KnowledgePack]) -> KnowledgePack {
        let mut out = self.clone();
        for o in overlays {
            out.merge(o);
        }
        out
    }

    /// 按本命盘取材（默认格局口径）：只含盘上出现的星（十二宫的主辅杂与四组十二神；
    /// 同宫主星之间的双星组合解读一并保留，不同宫的组合剔除）、命中的格局、四化四条。
    /// 宫位、术语与主题条目与盘无关，不进子包，按需从整包直接查。
    ///
    /// 返回的是标准 [`KnowledgePack`]（元信息沿用本包），可继续合并、序列化或按 key 查。
    pub fn for_astrolabe(&self, chart: &Astrolabe) -> KnowledgePack {
        self.for_astrolabe_with(chart, &PatternConfig::default())
    }

    /// 按本命盘取材，格局按指定口径判定；与 [`Astrolabe::patterns_with`] 同口径的
    /// 文本或 DTO 配同一口径的子包，释义才不会缺项。
    pub fn for_astrolabe_with(&self, chart: &Astrolabe, config: &PatternConfig) -> KnowledgePack {
        let mut out = self.skeleton();
        for palace in &chart.palaces {
            let majors: Vec<StarKey> = palace.major_stars.iter().map(|s| s.key).collect();
            for star in palace_star_keys(palace) {
                self.copy_star(&mut out, star, &majors);
            }
        }
        for hit in chart.patterns_with(config) {
            self.copy_pattern(&mut out, hit.key);
        }
        // 四化四条恒在盘上，但仍按 key 取——与星、格局同一口径，包里的非法键不进子包
        for m in [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            if let Some(entry) = self.mutagens.get(m.as_key()) {
                out.mutagens.insert(m.as_key().to_string(), entry.clone());
            }
        }
        out
    }

    /// 按运限取材（默认格局口径）：在 [`Self::for_astrolabe`] 之上，再加大限至查询精度
    /// （[`HoroscopeRef::precision`]）各层流耀的条目与各层视角命中的格局。按年、按月取的运限里
    /// 比精度细的层级是内核挑的代表日的，不取。
    pub fn for_horoscope(&self, horoscope: &HoroscopeRef) -> KnowledgePack {
        self.for_horoscope_with(horoscope, &PatternConfig::default())
    }

    /// 按运限取材，格局按指定口径判定；取材范围同 [`Self::for_horoscope`]。
    pub fn for_horoscope_with(
        &self,
        horoscope: &HoroscopeRef,
        config: &PatternConfig,
    ) -> KnowledgePack {
        let chart = horoscope.astrolabe();
        let precision = horoscope.precision();
        let mut out = self.for_astrolabe_with(chart, config);
        for scope in HOROSCOPE_SCOPES
            .into_iter()
            .filter(|s| s.depth() <= precision.depth())
        {
            if let Some(item) = horoscope.scope_item(scope) {
                for star in item.stars.iter().flatten().flatten() {
                    self.copy_star(&mut out, star.key, &[]);
                }
            }
            for hit in patterns_at(chart, horoscope.data(), scope, config) {
                self.copy_pattern(&mut out, hit.key);
            }
        }
        out
    }

    /// 只带元信息、各段为空的同源包。
    fn skeleton(&self) -> KnowledgePack {
        KnowledgePack {
            schema: self.schema,
            id: self.id.clone(),
            version: self.version.clone(),
            language: self.language.clone(),
            extends: self.extends.clone(),
            source: self.source.clone(),
            stars: BTreeMap::new(),
            patterns: BTreeMap::new(),
            palaces: BTreeMap::new(),
            mutagens: BTreeMap::new(),
            concepts: BTreeMap::new(),
            topics: BTreeMap::new(),
        }
    }

    /// 把一颗星的条目复制进子包；`same_palace_majors` 是该星同宫的主星，
    /// 双星组合解读及其摘要只保留对方主星确在同宫的那些。已复制过的星不重复处理。
    fn copy_star(&self, out: &mut KnowledgePack, star: StarKey, same_palace_majors: &[StarKey]) {
        let key = star.as_key();
        if out.stars.contains_key(key) {
            return;
        }
        let Some(entry) = self.stars.get(key) else {
            return;
        };
        let mut entry = entry.clone();
        let same_palace = |other: &String| same_palace_majors.iter().any(|m| m.as_key() == other);
        entry.combinations.retain(|other, _| same_palace(other));
        entry
            .combination_summaries
            .retain(|other, _| same_palace(other));
        out.stars.insert(key.to_string(), entry);
    }

    /// 把一条格局的条目复制进子包（已有则跳过）。
    fn copy_pattern(&self, out: &mut KnowledgePack, pattern: PatternKey) {
        let key = pattern.as_key();
        if !out.patterns.contains_key(key)
            && let Some(entry) = self.patterns.get(key)
        {
            out.patterns.insert(key.to_string(), entry.clone());
        }
    }

    /// 星耀条目。
    pub fn star(&self, key: StarKey) -> Option<&StarEntry> {
        self.stars.get(key.as_key())
    }

    /// 格局条目。
    pub fn pattern(&self, key: PatternKey) -> Option<&PatternEntry> {
        self.patterns.get(key.as_key())
    }

    /// 宫位条目。
    pub fn palace(&self, palace: Palace) -> Option<&TextEntry> {
        self.palaces.get(palace.as_key())
    }

    /// 四化条目。
    pub fn mutagen(&self, mutagen: Mutagen) -> Option<&TextEntry> {
        self.mutagens.get(mutagen.as_key())
    }

    /// 主题条目。
    pub fn topic(&self, slug: &str) -> Option<&TopicEntry> {
        self.topics.get(slug)
    }

    /// 星耀解读正文。
    pub fn star_intro(&self, key: StarKey) -> Option<&str> {
        self.star(key)?.intro.as_deref()
    }

    /// 格局解读正文。
    pub fn pattern_intro(&self, key: PatternKey) -> Option<&str> {
        self.pattern(key)?.intro.as_deref()
    }

    /// 摘要层缺摘要的条目路径（正文超过 [`SUMMARY_FALLBACK_WIDTH`] 却没有摘要），形如
    /// `stars.ziweiMaj`、`stars.ziweiMaj.combinations.tianfuMaj`、`patterns.zi_fu_tong_gong`、`topics.marriage`；
    /// 按段、按键排序。空列表即整包可用于摘要层；术语条目不进文本，不在检查之列。
    pub fn missing_summaries(&self) -> Vec<String> {
        let mut out = Vec::new();
        for (key, e) in &self.stars {
            if e.intro.is_some() && e.brief().is_none() {
                out.push(format!("stars.{key}"));
            }
            for other in e.combinations.keys() {
                if e.combination_brief(other).is_none() {
                    out.push(format!("stars.{key}.combinations.{other}"));
                }
            }
        }
        for (key, e) in &self.patterns {
            if e.intro.is_some() && e.brief().is_none() {
                out.push(format!("patterns.{key}"));
            }
        }
        for (section, entries) in [("palaces", &self.palaces), ("mutagens", &self.mutagens)] {
            for (key, e) in entries {
                if e.intro.is_some() && e.brief().is_none() {
                    out.push(format!("{section}.{key}"));
                }
            }
        }
        for (key, e) in &self.topics {
            if e.intro.is_some() && e.brief().is_none() {
                out.push(format!("topics.{key}"));
            }
        }
        out
    }
}

/// JSON 值合并：对象逐键递归，数组与标量整体覆盖。
///
/// 条目字段一律 `skip_serializing_if` 跳过空值，覆盖条目的缺省字段不会出现在
/// 值里，「非空字段覆盖、缺省保留、数组整体替换、null 等同缺省」由此自然成立。
fn merge_json(target: &mut serde_json::Value, overlay: &serde_json::Value) {
    if let (serde_json::Value::Object(t), serde_json::Value::Object(o)) = (&mut *target, overlay) {
        for (k, ov) in o {
            match t.get_mut(k) {
                Some(tv) if tv.is_object() && ov.is_object() => merge_json(tv, ov),
                _ => {
                    t.insert(k.clone(), ov.clone());
                }
            }
        }
    } else {
        *target = overlay.clone();
    }
}

/// 条目里正文字段与其摘要字段的对应（JSON 键）。
const SUMMARY_OF: [(&str, &str); 1] = [("intro", "summary")];
/// 条目里正文映射与其摘要映射的对应：两个映射同键成对。
const SUMMARY_MAP_OF: [(&str, &str); 1] = [("combinations", "combinationSummaries")];

/// 合并后去掉过期摘要：覆盖条目给了与底条目不同的正文却没给对应摘要时，合并结果里的摘要
/// 描述的是被换掉的旧正文，删去。`before` 是合并前的底条目。
fn drop_stale_summaries(
    merged: &mut serde_json::Value,
    before: &serde_json::Value,
    overlay: &serde_json::Value,
) {
    for (body, summary) in SUMMARY_OF {
        let rewritten = overlay
            .get(body)
            .is_some_and(|text| before.get(body) != Some(text));
        if rewritten
            && overlay.get(summary).is_none()
            && let Some(m) = merged.as_object_mut()
        {
            m.remove(summary);
        }
    }
    for (bodies, summaries) in SUMMARY_MAP_OF {
        let Some(changed) = overlay.get(bodies).and_then(|v| v.as_object()) else {
            continue;
        };
        for (key, text) in changed {
            let rewritten = before.get(bodies).and_then(|m| m.get(key)) != Some(text);
            let given = overlay.get(summaries).and_then(|m| m.get(key)).is_some();
            if rewritten
                && !given
                && let Some(m) = merged.get_mut(summaries).and_then(|v| v.as_object_mut())
            {
                m.remove(key);
            }
        }
    }
}

/// 把覆盖条目的非空字段并进底条目：经 JSON 值合并再落回类型——
/// 合并逻辑与结构体字段定义永不脱节，schema 新增字段自动参与合并，
/// 无须（也不许再有）逐字段手抄的合并清单。合并后按正文与摘要的成对关系去掉过期摘要。
fn merge_entry<T: serde::Serialize + serde::de::DeserializeOwned>(target: &mut T, overlay: &T) {
    let before = serde_json::to_value(&*target).expect("知识包条目只含普通可序列化字段");
    let ov = serde_json::to_value(overlay).expect("知识包条目只含普通可序列化字段");
    let mut tv = before.clone();
    merge_json(&mut tv, &ov);
    drop_stale_summaries(&mut tv, &before, &ov);
    *target = serde_json::from_value(tv).expect("合并结果仍符合条目结构");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_parses_and_only_zh_cn_exists() {
        let p = KnowledgePack::builtin(Language::ZhCN).expect("zh-CN builtin");
        assert_eq!(p.schema, SCHEMA_VERSION);
        assert_eq!(p.language, "zh-CN");
        assert!(KnowledgePack::builtin(Language::EnUS).is_none());
        assert!(KnowledgePack::builtin_json(Language::ZhCN).is_some());
    }

    #[test]
    fn merge_overrides_fields_and_keeps_others() {
        let base = KnowledgePack::from_json(
            r#"{"schema":1,"id":"base","version":"1","language":"zh-CN",
                "stars":{"ziweiMaj":{"name":"紫微","intro":"底","attributes":{"chemistry":"尊贵","aliases":["帝王星"]},
                          "combinations":{"tianfuMaj":"底组合"}}},
                "patterns":{"zi_fu_tong_gong":{"intro":"底格局","quotes":["a"]}}}"#,
        )
        .unwrap();
        let overlay = KnowledgePack::from_json(
            r#"{"schema":1,"id":"mine","version":"2","language":"zh-CN","extends":"base",
                "stars":{"ziweiMaj":{"intro":"我的","attributes":{"aliases":["我的别号"]},
                          "combinations":{"pojunMaj":"我的组合"}},
                         "tianjiMaj":{"intro":"新星"}},
                "patterns":{"zi_fu_tong_gong":{"quotes":["b","c"]}}}"#,
        )
        .unwrap();
        let m = base.merged(&[&overlay]);
        let zi = m.star(StarKey::ZiweiMaj).unwrap();
        assert_eq!(zi.intro.as_deref(), Some("我的"));
        assert_eq!(zi.name.as_deref(), Some("紫微"));
        assert_eq!(zi.attributes.chemistry.as_deref(), Some("尊贵"));
        assert_eq!(
            zi.attributes.aliases.as_deref(),
            Some(&["我的别号".to_string()][..])
        );
        assert_eq!(zi.combinations.len(), 2);
        assert_eq!(m.star_intro(StarKey::TianjiMaj), Some("新星"));
        let p = m.pattern(PatternKey::ZiFuTongGong).unwrap();
        assert_eq!(p.intro.as_deref(), Some("底格局"));
        assert_eq!(p.quotes.as_ref().unwrap().len(), 2);
        assert_eq!(m.id, "mine");
        assert_eq!(m.extends, None);
        assert_eq!(base.star_intro(StarKey::ZiweiMaj), Some("底"));
    }

    /// SCHEMA.md §2/§3/§6 的实现行为承诺：未知顶层字段静默丢弃、未知条目键拒绝、
    /// 覆盖包可新增条目、空字符串视为有值参与覆盖。
    #[test]
    fn schema_documented_behaviors_hold() {
        let p = KnowledgePack::from_json(
            r#"{"schema":1,"id":"x","version":"1","language":"zh-CN","futureField":{"a":1},
                "stars":{"ziweiMaj":{"intro":"已知键"}}}"#,
        )
        .unwrap();
        assert!(!p.to_json().contains("futureField"), "未知顶层字段应被丢弃");
        assert_eq!(p.star_intro(StarKey::ZiweiMaj), Some("已知键"));
        let err = KnowledgePack::from_json(
            r#"{"schema":1,"id":"x","version":"1","language":"zh-CN",
                "stars":{"notAStar":{"intro":"未知键"}}}"#,
        )
        .unwrap_err();
        assert!(err.contains("stars.notAStar: unknown key"), "{err}");
        let overlay = KnowledgePack::from_json(
            r#"{"schema":1,"id":"o","version":"1","language":"zh-CN",
                "stars":{"tianjiMaj":{"intro":"新增条目"},"ziweiMaj":{"intro":""}}}"#,
        )
        .unwrap();
        let m = p.merged(&[&overlay]);
        assert_eq!(
            m.star_intro(StarKey::TianjiMaj),
            Some("新增条目"),
            "覆盖包可新增条目"
        );
        assert_eq!(
            m.star_intro(StarKey::ZiweiMaj),
            Some(""),
            "空串视为有值并覆盖"
        );
    }

    /// 覆盖包换了正文没给摘要，旧摘要随之去掉；正文没变或同时给了摘要则保留覆盖后的值。
    /// 组合解读与组合摘要按键成对，同一规则。
    #[test]
    fn merge_drops_summaries_of_rewritten_bodies() {
        let base = KnowledgePack::from_json(
            r#"{"schema":1,"id":"b","version":"1","language":"zh-CN",
                "stars":{"ziweiMaj":{"intro":"底正文","summary":"底摘要",
                          "combinations":{"tianfuMaj":"底组合","pojunMaj":"底紫破"},
                          "combinationSummaries":{"tianfuMaj":"底组合摘要","pojunMaj":"底紫破摘要"}},
                         "tianjiMaj":{"intro":"天机正文","summary":"天机摘要"},
                         "taiyangMaj":{"intro":"太阳正文","summary":"太阳摘要"}},
                "patterns":{"zi_fu_tong_gong":{"intro":"底格局","summary":"底格局摘要"}}}"#,
        )
        .unwrap();
        let overlay = KnowledgePack::from_json(
            r#"{"schema":1,"id":"o","version":"1","language":"zh-CN",
                "stars":{"ziweiMaj":{"intro":"新正文","combinations":{"tianfuMaj":"新组合"}},
                         "tianjiMaj":{"intro":"天机正文"},
                         "taiyangMaj":{"intro":"新太阳","summary":"新太阳摘要"}},
                "patterns":{"zi_fu_tong_gong":{"intro":"新格局"}}}"#,
        )
        .unwrap();
        let m = base.merged(&[&overlay]);
        let zi = m.star(StarKey::ZiweiMaj).unwrap();
        assert_eq!(zi.summary, None);
        assert_eq!(zi.combination_summaries.get("tianfuMaj"), None);
        assert_eq!(
            zi.combination_summaries.get("pojunMaj").map(String::as_str),
            Some("底紫破摘要")
        );
        let tianji = m.star(StarKey::TianjiMaj).unwrap();
        assert_eq!(tianji.summary.as_deref(), Some("天机摘要"), "正文未变");
        let taiyang = m.star(StarKey::TaiyangMaj).unwrap();
        assert_eq!(taiyang.summary.as_deref(), Some("新太阳摘要"));
        assert_eq!(m.pattern(PatternKey::ZiFuTongGong).unwrap().summary, None);
    }

    /// 摘要层取材：有摘要取摘要；无摘要时正文不超过上限取正文，超过即缺项。
    /// 上限按显示宽度：120 个汉字恰好到上限，240 个拉丁字母同样到上限。
    #[test]
    fn brief_falls_back_to_short_bodies_only() {
        let short = "字".repeat(120);
        let long = "字".repeat(121);
        let latin = "a".repeat(240);
        let entry = |intro: &str, summary: Option<&str>| StarEntry {
            intro: Some(intro.to_string()),
            summary: summary.map(str::to_string),
            ..StarEntry::default()
        };
        assert_eq!(entry(&short, None).brief(), Some(short.as_str()));
        assert_eq!(entry(&latin, None).brief(), Some(latin.as_str()));
        assert_eq!(entry(&long, None).brief(), None);
        assert_eq!(entry(&long, Some("摘要")).brief(), Some("摘要"));
        assert_eq!(StarEntry::default().brief(), None);

        let mut pack =
            KnowledgePack::from_json(r#"{"schema":1,"id":"x","version":"1","language":"zh-CN"}"#)
                .unwrap();
        let mut zi = entry(&short, None);
        zi.combinations.insert("tianfuMaj".into(), long.clone());
        zi.combinations.insert("pojunMaj".into(), short.clone());
        pack.stars.insert("ziweiMaj".into(), zi);
        pack.stars.insert("tianjiMaj".into(), entry(&long, None));
        pack.patterns.insert(
            "zi_fu_tong_gong".into(),
            PatternEntry {
                intro: Some(long.clone()),
                ..PatternEntry::default()
            },
        );
        pack.mutagens.insert(
            "sihuaLu".into(),
            TextEntry {
                intro: Some(long.clone()),
                summary: Some("禄".into()),
                ..TextEntry::default()
            },
        );
        assert_eq!(
            pack.missing_summaries(),
            [
                "stars.tianjiMaj",
                "stars.ziweiMaj.combinations.tianfuMaj",
                "patterns.zi_fu_tong_gong"
            ]
        );
    }

    #[test]
    fn newer_schema_is_rejected_and_bad_json_is_error() {
        assert!(KnowledgePack::from_json(r#"{"schema":99}"#).is_err());
        assert!(
            KnowledgePack::from_json(r#"{"id":"x"}"#).is_err(),
            "schema 缺失必须拒绝"
        );
        assert!(KnowledgePack::from_json("nope").is_err());
        let p =
            KnowledgePack::from_json(r#"{"schema":1,"id":"x","version":"1","language":"zh-CN"}"#)
                .unwrap();
        assert!(p.stars.is_empty());
        assert_eq!(KnowledgePack::from_json(&p.to_json()).unwrap(), p);
        // 映射类字段写成 null 等同缺省（Go 等语言默认序列化 nil map 即为 null）
        let n = KnowledgePack::from_json(
            r#"{"schema":1,"id":"x","version":"1","language":"zh-CN","source":null,
                "stars":{"ziweiMaj":{"attributes":null,"combinations":null}},"patterns":null,
                "palaces":null,"mutagens":null,"concepts":null}"#,
        )
        .unwrap();
        assert!(n.patterns.is_empty() && n.star(StarKey::ZiweiMaj).is_some());
    }
}

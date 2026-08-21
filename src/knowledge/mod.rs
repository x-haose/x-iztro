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

use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::{Language, Mutagen, Palace};
use crate::pattern::PatternKey;

/// 当前支持的知识包格式版本。
pub const SCHEMA_VERSION: u32 = 1;

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
    /// 类别（`major` / `minor` / `adjective` / `dec`）
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
    /// 与另一颗主星同宫的组合解读，键为对方星耀标识
    #[serde(
        skip_serializing_if = "BTreeMap::is_empty",
        deserialize_with = "null_as_default"
    )]
    pub combinations: BTreeMap<String, String>,
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
const BUILTIN_ZH_CN: &str = include_str!("../data/knowledge/iztro_docs.zh-CN.json");

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

    /// 由 JSON 解析一份包；格式版本高于本库支持的返回错误。
    pub fn from_json(json: &str) -> Result<KnowledgePack, String> {
        let pack: KnowledgePack =
            serde_json::from_str(json).map_err(|e| format!("invalid knowledge pack: {e}"))?;
        Self::validated(pack)
    }

    /// 由已解析的 JSON 值解析一份包（绑定层免字符串往返）；校验同 [`Self::from_json`]。
    pub fn from_value(value: serde_json::Value) -> Result<KnowledgePack, String> {
        let pack: KnowledgePack =
            serde_json::from_value(value).map_err(|e| format!("invalid knowledge pack: {e}"))?;
        Self::validated(pack)
    }

    /// 解析共用的格式版本校验：schema 必须声明且不高于本库支持的版本。
    fn validated(pack: KnowledgePack) -> Result<KnowledgePack, String> {
        if pack.schema == 0 {
            return Err("knowledge pack must declare \"schema\" (currently 1)".to_string());
        }
        if pack.schema > SCHEMA_VERSION {
            return Err(format!(
                "knowledge pack schema {} is newer than supported {SCHEMA_VERSION}",
                pack.schema
            ));
        }
        Ok(pack)
    }

    /// 序列化为 JSON。
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("知识包只含普通可序列化字段")
    }

    /// 把覆盖包合并到本包：逐段按键合并，覆盖包的非空字段覆盖同键条目的对应字段，
    /// `attributes` 与 `combinations` 逐字段合并，数组字段整体替换。
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

    /// 星耀解读正文。
    pub fn star_intro(&self, key: StarKey) -> Option<&str> {
        self.star(key)?.intro.as_deref()
    }

    /// 格局解读正文。
    pub fn pattern_intro(&self, key: PatternKey) -> Option<&str> {
        self.pattern(key)?.intro.as_deref()
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

/// 把覆盖条目的非空字段并进底条目：经 JSON 值合并再落回类型——
/// 合并逻辑与结构体字段定义永不脱节，schema 新增字段自动参与合并，
/// 无须（也不许再有）逐字段手抄的合并清单。
fn merge_entry<T: serde::Serialize + serde::de::DeserializeOwned>(target: &mut T, overlay: &T) {
    let mut tv = serde_json::to_value(&*target).expect("知识包条目只含普通可序列化字段");
    let ov = serde_json::to_value(overlay).expect("知识包条目只含普通可序列化字段");
    merge_json(&mut tv, &ov);
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

    /// SCHEMA.md §2/§3/§6 的实现行为承诺：未知顶层字段静默丢弃、未知条目键保留但查询不到、
    /// 覆盖包可新增条目、空字符串视为有值参与覆盖。
    #[test]
    fn schema_documented_behaviors_hold() {
        let p = KnowledgePack::from_json(
            r#"{"schema":1,"id":"x","version":"1","language":"zh-CN","futureField":{"a":1},
                "stars":{"notAStar":{"intro":"未知键"},"ziweiMaj":{"intro":"已知键"}}}"#,
        )
        .unwrap();
        assert!(!p.to_json().contains("futureField"), "未知顶层字段应被丢弃");
        assert!(p.stars.contains_key("notAStar"), "未知条目键应被保留");
        assert_eq!(p.star_intro(StarKey::ZiweiMaj), Some("已知键"));
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
        assert!(m.stars.contains_key("notAStar"), "未知键参与合并后仍保留");
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

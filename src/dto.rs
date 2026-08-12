//! JS iztro 兼容的序列化 DTO。
//!
//! 核心模型（`Astrolabe`/`HoroscopeData`）以枚举承载数据，供 Rust 调用方做
//! 类型安全查询；跨语言绑定（FFI/PyO3）输出的 JSON 则来自本模块的 DTO：
//! 键为 camelCase、值为按排盘语言翻译的字符串，结构与 JS iztro 的
//! `JSON.stringify` 输出一致（不含其 `plugins`/`copyright` 及运限对象内嵌的
//! `astrolabe` 等实现细节字段）。
//!
//! 在 JS 字段集之外附加两类扩展：
//! - 排盘上下文（`genderKey`/`timeIndex`/`fixLeap`/`language`/`config`），
//!   使消费方能以纯参数（无状态）方式发起运限计算；
//! - 语言无关标识（星/宫/干支/四化/亮度的 `*key`/`*Key(s)` 字段，取值为
//!   iztro i18n key），供强类型绑定做跨语言的身份判断与枚举映射。

use serde::{Deserialize, Serialize};

use crate::data::types::*;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::{HoroscopeData, HoroscopeItem};
use crate::models::palace::PalaceData;
use crate::models::star::Star;

/// 星耀 DTO。
/// 主星/辅星的 `brightness` 恒存在（无亮度为空串）；无四化时省略 `mutagen` 键；
/// 杂耀与运限流耀省略 `brightness` 与 `mutagen`。
/// `key`/`brightnessKey`/`mutagenKey` 为语言无关标识（rs-iztro 扩展），
/// 供强类型绑定做跨语言的身份判断。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarDto {
    /// 语言无关星耀标识（iztro i18n key，如 "ziweiMaj"）
    pub key: String,
    /// 名称（按排盘语言翻译）
    pub name: String,
    /// 星耀类型（major/soft/tough/adjective/flower/helper/lucun/tianma）
    #[serde(rename = "type")]
    pub star_type: String,
    /// 作用范围（origin/decadal/yearly/monthly/daily/hourly）
    pub scope: String,
    /// 亮度显示文本；主星辅星恒有该键（无亮度为空串），杂耀与流耀省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<String>,
    /// 语言无关亮度标识（"miao" 等），无亮度时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness_key: Option<String>,
    /// 四化显示文本；四化候选星恒有该键（无四化为空串），其余省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutagen: Option<String>,
    /// 语言无关四化标识（"sihuaLu" 等），无四化时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutagen_key: Option<String>,
}

/// 大限区间与干支 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DecadalDto {
    /// 大限起止虚岁（起始、截止，含两端）
    pub range: [u32; 2],
    /// 天干（按排盘语言翻译）
    pub heavenly_stem: String,
    /// 语言无关天干标识（"jiaHeavenly" 等）
    pub heavenly_stem_key: String,
    /// 地支（按排盘语言翻译）
    pub earthly_branch: String,
    /// 语言无关地支标识（"ziEarthly" 等）
    pub earthly_branch_key: String,
}

/// 宫位 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PalaceDto {
    /// 宫位索引（0-11，寅宫为 0）
    pub index: usize,
    /// 名称（按排盘语言翻译）
    pub name: String,
    /// 语言无关宫位标识（"soulPalace" 等）
    pub name_key: String,
    /// 是否身宫
    pub is_body_palace: bool,
    /// 是否来因宫
    pub is_original_palace: bool,
    /// 天干（按排盘语言翻译）
    pub heavenly_stem: String,
    /// 语言无关天干标识
    pub heavenly_stem_key: String,
    /// 地支（按排盘语言翻译）
    pub earthly_branch: String,
    /// 语言无关地支标识
    pub earthly_branch_key: String,
    /// 主星列表（按安放顺序）
    pub major_stars: Vec<StarDto>,
    /// 辅星列表（按安放顺序）
    pub minor_stars: Vec<StarDto>,
    /// 杂耀列表（按安放顺序）
    pub adjective_stars: Vec<StarDto>,
    /// 长生十二神（按排盘语言翻译）
    pub changsheng12: String,
    /// 长生十二神的语言无关标识
    pub changsheng12_key: String,
    /// 博士十二神（按排盘语言翻译）
    pub boshi12: String,
    /// 博士十二神的语言无关标识
    pub boshi12_key: String,
    /// 将前十二神（按排盘语言翻译）
    pub jiangqian12: String,
    /// 将前十二神的语言无关标识
    pub jiangqian12_key: String,
    /// 岁前十二神（按排盘语言翻译）
    pub suiqian12: String,
    /// 岁前十二神的语言无关标识
    pub suiqian12_key: String,
    /// 大限信息
    pub decadal: DecadalDto,
    /// 小限经过的虚岁列表
    pub ages: Vec<u32>,
}

/// 数字化农历生日 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawLunarDateDto {
    /// 农历年
    pub lunar_year: i64,
    /// 农历月（1-12，闰月与否见 is_leap）
    pub lunar_month: u32,
    /// 农历日（1-30）
    pub lunar_day: u32,
    /// 是否闰月
    pub is_leap: bool,
}

/// 四柱干支 DTO（每柱为 [天干, 地支] 两元素数组）。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawChineseDateDto {
    /// 年柱（天干、地支，干支原文）
    pub yearly: [String; 2],
    /// 月柱（天干、地支，干支原文）
    pub monthly: [String; 2],
    /// 日柱（天干、地支，干支原文）
    pub daily: [String; 2],
    /// 时柱（天干、地支，干支原文）
    pub hourly: [String; 2],
}

/// 结构化出生日期 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawDatesDto {
    /// 数字化农历生日
    pub lunar_date: RawLunarDateDto,
    /// 四柱干支
    pub chinese_date: RawChineseDateDto,
}

/// 排盘配置 DTO（字符串取值，与 JS iztro 的 config() 取值一致）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigDto {
    /// "normal" | "exact"
    pub year_divide: String,
    /// "normal" | "exact"
    pub horoscope_divide: String,
    /// "normal" | "birthday"
    pub age_divide: String,
    /// "forward" | "current"
    pub day_divide: String,
    /// "default" | "zhongzhou"
    pub algorithm: String,
}

/// 星盘 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AstrolabeDto {
    /// 性别（按排盘语言翻译）
    pub gender: String,
    /// 阳历日期（"YYYY-M-D"）
    pub solar_date: String,
    /// 农历日期中文表示
    pub lunar_date: String,
    /// 干支纪日四柱展示串
    pub chinese_date: String,
    /// 结构化的出生日期信息
    pub raw_dates: RawDatesDto,
    /// 时辰名称（按排盘语言）
    pub time: String,
    /// 时辰对应的时间段
    pub time_range: String,
    /// 星座（按排盘语言）
    pub sign: String,
    /// 生肖（按排盘语言）
    pub zodiac: String,
    /// 命宫地支（按排盘语言翻译）
    pub earthly_branch_of_soul_palace: String,
    /// 命宫地支的语言无关标识
    pub earthly_branch_of_soul_palace_key: String,
    /// 身宫地支（按排盘语言翻译）
    pub earthly_branch_of_body_palace: String,
    /// 身宫地支的语言无关标识
    pub earthly_branch_of_body_palace_key: String,
    /// 命主星（按排盘语言翻译）
    pub soul: String,
    /// 命主星的语言无关标识
    pub soul_key: String,
    /// 身主星（按排盘语言翻译）
    pub body: String,
    /// 身主星的语言无关标识
    pub body_key: String,
    /// 五行局（按排盘语言翻译）
    pub five_elements_class: String,
    /// 五行局的语言无关标识（"water2nd" 等）
    pub five_elements_class_key: String,
    /// 十二宫数据
    pub palaces: Vec<PalaceDto>,
    /// rs-iztro 扩展：机器可读性别（"male"/"female"，无状态运限所需）
    pub gender_key: String,
    /// rs-iztro 扩展：出生时辰索引（无状态运限所需）
    pub time_index: u8,
    /// rs-iztro 扩展：是否修正闰月
    pub fix_leap: bool,
    /// rs-iztro 扩展：排盘语言（"zh_cn" 等）
    pub language: String,
    /// rs-iztro 扩展：排盘配置
    pub config: ConfigDto,
}

/// 运限单层级 DTO（大限/流年/流月/流日/流时）。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoroscopeScopeDto {
    /// 宫位索引（0-11，寅宫为 0）
    pub index: usize,
    /// 名称（按排盘语言翻译）
    pub name: String,
    /// 天干（按排盘语言翻译）
    pub heavenly_stem: String,
    /// 语言无关天干标识
    pub heavenly_stem_key: String,
    /// 地支（按排盘语言翻译）
    pub earthly_branch: String,
    /// 语言无关地支标识
    pub earthly_branch_key: String,
    /// 该运限的十二宫名（按宫位索引排列，翻译文本）
    pub palace_names: Vec<String>,
    /// 十二宫名的语言无关标识
    pub palace_name_keys: Vec<String>,
    /// 四化星名（禄、权、科、忌，翻译文本）
    pub mutagen: Vec<String>,
    /// 四化星（禄、权、科、忌）的语言无关标识
    pub mutagen_keys: Vec<String>,
    /// 流耀在十二宫的分布；无流耀的层级省略该键
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stars: Option<Vec<Vec<StarDto>>>,
}

/// 小限 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AgeDto {
    /// 通用运限字段
    #[serde(flatten)]
    pub base: HoroscopeScopeDto,
    /// 虚岁
    pub nominal_age: u32,
}

/// 流年十二神 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YearlyDecStarDto {
    /// 岁前十二神（按宫位索引排列，翻译文本）
    pub suiqian12: Vec<String>,
    /// 岁前十二神的语言无关标识
    #[serde(rename = "suiqian12Keys")]
    pub suiqian12_keys: Vec<String>,
    /// 将前十二神（按宫位索引排列，翻译文本）
    pub jiangqian12: Vec<String>,
    /// 将前十二神的语言无关标识
    #[serde(rename = "jiangqian12Keys")]
    pub jiangqian12_keys: Vec<String>,
}

/// 流年 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YearlyDto {
    /// 通用运限字段
    #[serde(flatten)]
    pub base: HoroscopeScopeDto,
    /// 流年十二神
    pub yearly_dec_star: YearlyDecStarDto,
}

/// 运限 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HoroscopeDto {
    /// 农历日期中文表示
    pub lunar_date: String,
    /// 阳历日期（"YYYY-M-D"）
    pub solar_date: String,
    /// 大限（未起运时为童限）
    pub decadal: HoroscopeScopeDto,
    /// 小限
    pub age: AgeDto,
    /// 流年
    pub yearly: YearlyDto,
    /// 流月
    pub monthly: HoroscopeScopeDto,
    /// 流日
    pub daily: HoroscopeScopeDto,
    /// 流时
    pub hourly: HoroscopeScopeDto,
}

// ============================================================
// 枚举 → 字符串
// ============================================================

/// StarType 的 JS 取值
fn star_type_str(t: StarType) -> &'static str {
    match t {
        StarType::Major => "major",
        StarType::Soft => "soft",
        StarType::Tough => "tough",
        StarType::Adjective => "adjective",
        StarType::Flower => "flower",
        StarType::Helper => "helper",
        StarType::Lucun => "lucun",
        StarType::Tianma => "tianma",
    }
}

/// Scope 的 JS 取值
fn scope_str(s: Scope) -> &'static str {
    match s {
        Scope::Origin => "origin",
        Scope::Decadal => "decadal",
        Scope::Yearly => "yearly",
        Scope::Monthly => "monthly",
        Scope::Daily => "daily",
        Scope::Hourly => "hourly",
    }
}

/// Language 的绑定层取值
pub fn language_str(l: Language) -> &'static str {
    match l {
        Language::ZhCN => "zh_cn",
        Language::ZhTW => "zh_tw",
        Language::EnUS => "en_us",
        Language::JaJP => "ja_jp",
        Language::KoKR => "ko_kr",
        Language::ViVN => "vi_vn",
    }
}

/// 从 panic 载荷提取人类可读消息（downcast String/&str，兜底固定文案）。
/// 供各绑定层把核心计算的 panic 转为对外错误。
pub(crate) fn panic_message(panic: &(dyn std::any::Any + Send)) -> &str {
    panic
        .downcast_ref::<String>()
        .map(String::as_str)
        .or_else(|| panic.downcast_ref::<&str>().copied())
        .unwrap_or("computation panicked")
}

/// config JSON 的部分键补丁：缺省键取默认值。
#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
struct ConfigPatch {
    year_divide: Option<String>,
    horoscope_divide: Option<String>,
    age_divide: Option<String>,
    day_divide: Option<String>,
    algorithm: Option<String>,
}

/// 解析绑定层的 config JSON（如 `{"algorithm":"zhongzhou"}`）。
/// `None` 或空串返回默认配置；未出现的键取默认值；非法取值报错。
pub fn parse_config_json(json: Option<&str>) -> Result<Config, String> {
    let json = match json {
        None => return Ok(Config::default()),
        Some(s) if s.trim().is_empty() => return Ok(Config::default()),
        Some(s) => s,
    };
    let patch: ConfigPatch =
        serde_json::from_str(json).map_err(|e| format!("Invalid config JSON: {e}"))?;

    let mut config = Config::default();
    if let Some(v) = patch.year_divide {
        config.year_divide = match v.as_str() {
            "normal" => YearDivide::Normal,
            "exact" => YearDivide::Exact,
            _ => {
                return Err(format!(
                    "Invalid yearDivide '{v}'. Expected 'normal' or 'exact'."
                ));
            }
        };
    }
    if let Some(v) = patch.horoscope_divide {
        config.horoscope_divide = match v.as_str() {
            "normal" => HoroscopeDivide::Normal,
            "exact" => HoroscopeDivide::Exact,
            _ => {
                return Err(format!(
                    "Invalid horoscopeDivide '{v}'. Expected 'normal' or 'exact'."
                ));
            }
        };
    }
    if let Some(v) = patch.age_divide {
        config.age_divide = match v.as_str() {
            "normal" => AgeDivide::Normal,
            "birthday" => AgeDivide::Birthday,
            _ => {
                return Err(format!(
                    "Invalid ageDivide '{v}'. Expected 'normal' or 'birthday'."
                ));
            }
        };
    }
    if let Some(v) = patch.day_divide {
        config.day_divide = match v.as_str() {
            "forward" => DayDivide::Forward,
            "current" => DayDivide::Current,
            _ => {
                return Err(format!(
                    "Invalid dayDivide '{v}'. Expected 'forward' or 'current'."
                ));
            }
        };
    }
    if let Some(v) = patch.algorithm {
        config.algorithm = match v.as_str() {
            "default" => Algorithm::Default,
            "zhongzhou" => Algorithm::Zhongzhou,
            _ => {
                return Err(format!(
                    "Invalid algorithm '{v}'. Expected 'default' or 'zhongzhou'."
                ));
            }
        };
    }
    Ok(config)
}

impl From<Config> for ConfigDto {
    fn from(c: Config) -> Self {
        ConfigDto {
            year_divide: match c.year_divide {
                YearDivide::Normal => "normal",
                YearDivide::Exact => "exact",
            }
            .to_string(),
            horoscope_divide: match c.horoscope_divide {
                HoroscopeDivide::Normal => "normal",
                HoroscopeDivide::Exact => "exact",
            }
            .to_string(),
            age_divide: match c.age_divide {
                AgeDivide::Normal => "normal",
                AgeDivide::Birthday => "birthday",
            }
            .to_string(),
            day_divide: match c.day_divide {
                DayDivide::Forward => "forward",
                DayDivide::Current => "current",
            }
            .to_string(),
            algorithm: match c.algorithm {
                Algorithm::Default => "default",
                Algorithm::Zhongzhou => "zhongzhou",
            }
            .to_string(),
        }
    }
}

// ============================================================
// 模型 → DTO
// ============================================================

/// 是否属于四化候选星（十四主星与文昌/文曲/左辅/右弼）。
/// 这些星的 DTO 恒带 mutagen 键（无四化为空串），其余星省略该键。
fn is_mutagen_candidate(key: crate::data::stars::StarKey) -> bool {
    use crate::data::stars::StarKey::*;
    matches!(
        key,
        ZiweiMaj
            | TianjiMaj
            | TaiyangMaj
            | WuquMaj
            | TiantongMaj
            | LianzhenMaj
            | TianfuMaj
            | TaiyinMaj
            | TanlangMaj
            | JumenMaj
            | TianxiangMaj
            | TianliangMaj
            | QishaMaj
            | PojunMaj
            | WenchangMin
            | WenquMin
            | ZuofuMin
            | YoubiMin
    )
}

/// 主星/辅星条目：brightness 恒存在（无亮度为空串）；
/// 四化候选星恒带 mutagen 键（无四化为空串），其余省略。
fn primary_star_dto(s: &Star, lang: Language) -> StarDto {
    StarDto {
        key: s.key.as_key().to_string(),
        name: s.name.clone(),
        star_type: star_type_str(s.star_type).to_string(),
        scope: scope_str(s.scope).to_string(),
        brightness: Some(
            s.brightness
                .map(|b| translate_brightness(b, lang).to_string())
                .unwrap_or_default(),
        ),
        brightness_key: s.brightness.map(|b| b.as_key().to_string()),
        mutagen: if is_mutagen_candidate(s.key) {
            Some(
                s.mutagen
                    .map(|m| translate_mutagen(m, lang).to_string())
                    .unwrap_or_default(),
            )
        } else {
            None
        },
        mutagen_key: s.mutagen.map(|m| m.as_key().to_string()),
    }
}

/// 杂耀/流耀条目：仅 name/type/scope 三键。
fn bare_star_dto(s: &Star) -> StarDto {
    StarDto {
        key: s.key.as_key().to_string(),
        name: s.name.clone(),
        star_type: star_type_str(s.star_type).to_string(),
        scope: scope_str(s.scope).to_string(),
        brightness: None,
        brightness_key: None,
        mutagen: None,
        mutagen_key: None,
    }
}

fn pillar(p: (HeavenlyStem, EarthlyBranch), lang: Language) -> [String; 2] {
    [
        translate_heavenly_stem(p.0, lang).to_string(),
        translate_earthly_branch(p.1, lang).to_string(),
    ]
}

fn palace_dto(p: &PalaceData, lang: Language) -> PalaceDto {
    PalaceDto {
        index: p.index,
        name: translate_palace(p.name, lang).to_string(),
        name_key: p.name.as_key().to_string(),
        is_body_palace: p.is_body_palace,
        is_original_palace: p.is_original_palace,
        heavenly_stem: translate_heavenly_stem(p.heavenly_stem, lang).to_string(),
        heavenly_stem_key: p.heavenly_stem.as_key().to_string(),
        earthly_branch: translate_earthly_branch(p.earthly_branch, lang).to_string(),
        earthly_branch_key: p.earthly_branch.as_key().to_string(),
        major_stars: p
            .major_stars
            .iter()
            .map(|s| primary_star_dto(s, lang))
            .collect(),
        minor_stars: p
            .minor_stars
            .iter()
            .map(|s| primary_star_dto(s, lang))
            .collect(),
        adjective_stars: p.adjective_stars.iter().map(bare_star_dto).collect(),
        changsheng12: translate_star(p.changsheng12, lang).to_string(),
        changsheng12_key: p.changsheng12.as_key().to_string(),
        boshi12: translate_star(p.boshi12, lang).to_string(),
        boshi12_key: p.boshi12.as_key().to_string(),
        jiangqian12: translate_star(p.jiangqian12, lang).to_string(),
        jiangqian12_key: p.jiangqian12.as_key().to_string(),
        suiqian12: translate_star(p.suiqian12, lang).to_string(),
        suiqian12_key: p.suiqian12.as_key().to_string(),
        decadal: DecadalDto {
            range: [p.decadal.range.0, p.decadal.range.1],
            heavenly_stem: translate_heavenly_stem(p.decadal.heavenly_stem, lang).to_string(),
            heavenly_stem_key: p.decadal.heavenly_stem.as_key().to_string(),
            earthly_branch: translate_earthly_branch(p.decadal.earthly_branch, lang).to_string(),
            earthly_branch_key: p.decadal.earthly_branch.as_key().to_string(),
        },
        ages: p.ages.clone(),
    }
}

impl Astrolabe {
    /// 转为 JS iztro 兼容的序列化 DTO（值按排盘语言翻译）。
    pub fn to_dto(&self) -> AstrolabeDto {
        let lang = self.language;
        AstrolabeDto {
            gender: translate_gender(self.gender, lang).to_string(),
            solar_date: self.solar_date.clone(),
            lunar_date: self.lunar_date.clone(),
            chinese_date: self.chinese_date.clone(),
            raw_dates: RawDatesDto {
                lunar_date: RawLunarDateDto {
                    lunar_year: self.raw_dates.lunar_date.lunar_year,
                    lunar_month: self.raw_dates.lunar_date.lunar_month,
                    lunar_day: self.raw_dates.lunar_date.lunar_day,
                    is_leap: self.raw_dates.lunar_date.is_leap,
                },
                // rawDates 的四柱为未本地化的干支原文（任何输出语言下均为中文）
                chinese_date: RawChineseDateDto {
                    yearly: pillar(self.raw_dates.chinese_date.yearly, Language::ZhCN),
                    monthly: pillar(self.raw_dates.chinese_date.monthly, Language::ZhCN),
                    daily: pillar(self.raw_dates.chinese_date.daily, Language::ZhCN),
                    hourly: pillar(self.raw_dates.chinese_date.hourly, Language::ZhCN),
                },
            },
            time: self.time.clone(),
            time_range: self.time_range.clone(),
            sign: self.sign.clone(),
            zodiac: self.zodiac.clone(),
            earthly_branch_of_soul_palace: translate_earthly_branch(
                self.earthly_branch_of_soul_palace,
                lang,
            )
            .to_string(),
            earthly_branch_of_soul_palace_key: self
                .earthly_branch_of_soul_palace
                .as_key()
                .to_string(),
            earthly_branch_of_body_palace: translate_earthly_branch(
                self.earthly_branch_of_body_palace,
                lang,
            )
            .to_string(),
            earthly_branch_of_body_palace_key: self
                .earthly_branch_of_body_palace
                .as_key()
                .to_string(),
            soul: translate_star(self.soul, lang).to_string(),
            soul_key: self.soul.as_key().to_string(),
            body: translate_star(self.body, lang).to_string(),
            body_key: self.body.as_key().to_string(),
            five_elements_class: translate_five_elements_class(self.five_elements_class, lang)
                .to_string(),
            five_elements_class_key: self.five_elements_class.as_key().to_string(),
            palaces: self.palaces.iter().map(|p| palace_dto(p, lang)).collect(),
            gender_key: match self.gender {
                Gender::Male => "male",
                Gender::Female => "female",
            }
            .to_string(),
            time_index: self.time_index,
            fix_leap: self.fix_leap,
            language: language_str(lang).to_string(),
            config: self.config.into(),
        }
    }
}

fn scope_dto(item: &HoroscopeItem, lang: Language) -> HoroscopeScopeDto {
    HoroscopeScopeDto {
        index: item.index,
        name: item.name.clone(),
        heavenly_stem: translate_heavenly_stem(item.heavenly_stem, lang).to_string(),
        heavenly_stem_key: item.heavenly_stem.as_key().to_string(),
        earthly_branch: translate_earthly_branch(item.earthly_branch, lang).to_string(),
        earthly_branch_key: item.earthly_branch.as_key().to_string(),
        palace_names: item
            .palace_names
            .iter()
            .map(|p| translate_palace(*p, lang).to_string())
            .collect(),
        palace_name_keys: item
            .palace_names
            .iter()
            .map(|p| p.as_key().to_string())
            .collect(),
        mutagen: item
            .mutagen
            .iter()
            .map(|k| translate_star(*k, lang).to_string())
            .collect(),
        mutagen_keys: item
            .mutagen
            .iter()
            .map(|k| k.as_key().to_string())
            .collect(),
        stars: item.stars.as_ref().map(|groups| {
            groups
                .iter()
                .map(|g| g.iter().map(bare_star_dto).collect())
                .collect()
        }),
    }
}

impl HoroscopeData {
    /// 转为 JS iztro 兼容的序列化 DTO（值按给定语言翻译）。
    pub fn to_dto(&self, lang: Language) -> HoroscopeDto {
        HoroscopeDto {
            lunar_date: self.lunar_date.clone(),
            solar_date: self.solar_date.clone(),
            decadal: scope_dto(&self.decadal, lang),
            age: AgeDto {
                base: scope_dto(&self.age.base, lang),
                nominal_age: self.age.nominal_age,
            },
            yearly: YearlyDto {
                base: scope_dto(&self.yearly.base, lang),
                yearly_dec_star: YearlyDecStarDto {
                    suiqian12: self
                        .yearly
                        .yearly_dec_star
                        .suiqian12
                        .iter()
                        .map(|k| translate_star(*k, lang).to_string())
                        .collect(),
                    suiqian12_keys: self
                        .yearly
                        .yearly_dec_star
                        .suiqian12
                        .iter()
                        .map(|k| k.as_key().to_string())
                        .collect(),
                    jiangqian12: self
                        .yearly
                        .yearly_dec_star
                        .jiangqian12
                        .iter()
                        .map(|k| translate_star(*k, lang).to_string())
                        .collect(),
                    jiangqian12_keys: self
                        .yearly
                        .yearly_dec_star
                        .jiangqian12
                        .iter()
                        .map(|k| k.as_key().to_string())
                        .collect(),
                },
            },
            monthly: scope_dto(&self.monthly, lang),
            daily: scope_dto(&self.daily, lang),
            hourly: scope_dto(&self.hourly, lang),
        }
    }
}

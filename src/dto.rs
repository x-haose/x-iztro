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
//!   iztro i18n key），供强类型绑定做跨语言的身份判断与枚举映射；宫位另带
//!   `mutagenStarKeys`（本宫天干化出的禄权科忌四星），绑定层据此做飞星判断。
//!
//! 出错时三条绑定出口（FFI/wasm/PyO3）返回同一形状的错误 JSON
//! `{"error": "<描述>", "code": "<分类>"}`，分类取值见 [`crate::error::BridgeError`]。

use serde::{Deserialize, Serialize};

use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::*;
use crate::error::BridgeError;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace,
    translate_pattern, translate_star,
};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::{HoroscopeData, HoroscopeItem};
use crate::models::palace::PalaceData;
use crate::models::star::Star;

/// 星耀 DTO。
/// 主星/辅星的 `brightness` 恒存在（无亮度为空串）；无四化时省略 `mutagen` 键；
/// 杂耀与运限流耀省略 `brightness` 与 `mutagen`。
/// `key`/`brightnessKey`/`mutagenKey` 为语言无关标识（x-iztro 扩展），
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
    /// x-iztro 扩展：本宫天干化出的四颗星的语言无关标识，顺序为禄、权、科、忌。
    /// 取自排盘时生效的四化表（含 `Config` 的自定义覆盖），
    /// 绑定层据此做飞星判断，无须各自再抄一份四化表。
    pub mutagen_star_keys: [String; 4],
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
#[serde(rename_all = "camelCase")]
pub struct RawChineseDateDto {
    /// 年柱（天干、地支，干支原文）
    pub yearly: [String; 2],
    /// 月柱（天干、地支，干支原文）
    pub monthly: [String; 2],
    /// 日柱（天干、地支，干支原文）
    pub daily: [String; 2],
    /// 时柱（天干、地支，干支原文）
    pub hourly: [String; 2],
    /// 年柱的语言无关标识（天干、地支）
    pub yearly_keys: [String; 2],
    /// 月柱的语言无关标识
    pub monthly_keys: [String; 2],
    /// 日柱的语言无关标识
    pub daily_keys: [String; 2],
    /// 时柱的语言无关标识
    pub hourly_keys: [String; 2],
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
    /// "heaven" | "earth" | "human"
    pub astro_type: String,
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
    /// 星座的语言无关标识（"aries" … "pisces"）
    pub sign_key: String,
    /// 生肖（按排盘语言）
    pub zodiac: String,
    /// 生肖的语言无关标识（"rat" … "pig"）
    pub zodiac_key: String,
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
    /// x-iztro 扩展：机器可读性别（"male"/"female"，无状态运限所需）
    pub gender_key: String,
    /// x-iztro 扩展：出生时辰索引（无状态运限所需）
    pub time_index: u8,
    /// x-iztro 扩展：是否修正闰月
    pub fix_leap: bool,
    /// x-iztro 扩展：排盘语言（"zh-CN" 等）
    pub language: String,
    /// x-iztro 扩展：排盘配置
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
    /// 名称的语言无关标识；大限层未起运时为 "childhood"（童限），
    /// 与 "decadal" 是不同的解盘语义，程序判断一律用本字段而非译文
    pub name_key: String,
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
    /// 四化星（禄、权、科、忌）的语言无关标识（StarKey，与 `PalaceDto` 的
    /// mutagenStarKeys 同名同义；单数 mutagenKey 才是四化类型 Mutagen 的标识）
    pub mutagen_star_keys: Vec<String>,
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

/// 绑定层出口的错误 JSON：`{"error": "<message>", "code": "<code>"}`。
///
/// 用结构体而非 map 序列化，键序固定为 error 先、code 后；消息经 serde
/// 转义，引号、反斜杠与控制字符一律安全。
pub(crate) fn error_json(err: &BridgeError) -> String {
    #[derive(Serialize)]
    struct ErrorJson<'a> {
        error: &'a str,
        code: &'a str,
    }
    serde_json::to_string(&ErrorJson {
        error: &err.message,
        code: err.code,
    })
    .expect("错误 JSON 只含两个字符串字段，序列化不会失败")
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
    astro_type: Option<String>,
    /// 天干标识 → 四化星标识数组（禄权科忌四项）
    mutagens: Option<std::collections::HashMap<String, Vec<String>>>,
    /// 星耀标识 → 十二宫亮度标识数组（十二项，空串表示该宫无亮度）
    brightness: Option<std::collections::HashMap<String, Vec<String>>>,
}

/// 解析绑定层的 config JSON（如 `{"algorithm":"zhongzhou"}`）。
/// `None` 或空串返回默认配置；未出现的键取默认值；非法取值报错。
///
/// # Errors
/// JSON 语法错误或任一键取值不在允许集合内时返回 `invalid_argument`。
pub fn parse_config_json(json: Option<&str>) -> Result<Config, BridgeError> {
    let json = match json {
        None => return Ok(Config::default()),
        Some(s) if s.trim().is_empty() => return Ok(Config::default()),
        Some(s) => s,
    };
    let patch: ConfigPatch = serde_json::from_str(json)
        .map_err(|e| BridgeError::invalid_argument(format!("invalid config JSON: {e}")))?;
    config_from_patch(patch)
}

/// 由已反序列化的补丁构造配置，供 `parse_config_json` 与 `serde_json::Value`
/// 入参共用——后者不必先序列化回字符串再解析一遍。
///
/// # Errors
/// 任一键取值不在允许集合内时返回 `invalid_argument`。
pub fn parse_config_value(value: &serde_json::Value) -> Result<Config, BridgeError> {
    let patch: ConfigPatch = serde_json::from_value(value.clone())
        .map_err(|e| BridgeError::invalid_argument(format!("invalid config: {e}")))?;
    config_from_patch(patch)
}

/// 开关键取值非法的统一报错：`invalid <field> '<value>': expected <expected>`
fn bad_switch(field: &str, value: &str, expected: &str) -> BridgeError {
    BridgeError::invalid_argument(format!("invalid {field} '{value}': expected {expected}"))
}

fn config_from_patch(patch: ConfigPatch) -> Result<Config, BridgeError> {
    let mut config = Config::default();
    if let Some(v) = patch.year_divide {
        config.year_divide = YearDivide::from_key(&v)
            .ok_or_else(|| bad_switch("yearDivide", &v, "'normal' or 'exact'"))?;
    }
    if let Some(v) = patch.horoscope_divide {
        config.horoscope_divide = HoroscopeDivide::from_key(&v)
            .ok_or_else(|| bad_switch("horoscopeDivide", &v, "'normal' or 'exact'"))?;
    }
    if let Some(v) = patch.age_divide {
        config.age_divide = AgeDivide::from_key(&v)
            .ok_or_else(|| bad_switch("ageDivide", &v, "'normal' or 'birthday'"))?;
    }
    if let Some(v) = patch.day_divide {
        config.day_divide = DayDivide::from_key(&v)
            .ok_or_else(|| bad_switch("dayDivide", &v, "'forward' or 'current'"))?;
    }
    if let Some(v) = patch.algorithm {
        config.algorithm = Algorithm::from_key(&v)
            .ok_or_else(|| bad_switch("algorithm", &v, "'default' or 'zhongzhou'"))?;
    }
    if let Some(v) = patch.astro_type {
        config.astro_type = AstroType::from_key(&v)
            .ok_or_else(|| bad_switch("astroType", &v, "'heaven', 'earth' or 'human'"))?;
    }

    // 自定义四化与亮度表：键与值都是语言无关标识
    if let Some(map) = patch.mutagens {
        for (stem_key, star_keys) in map {
            let stem = HeavenlyStem::from_key(&stem_key).ok_or_else(|| {
                BridgeError::invalid_argument(format!(
                    "invalid mutagens key '{stem_key}': unknown heavenly stem"
                ))
            })?;
            if star_keys.len() != 4 {
                return Err(BridgeError::invalid_argument(format!(
                    "invalid mutagens for '{stem_key}': expected 4 stars (lu, quan, ke, ji), got {}",
                    star_keys.len()
                )));
            }
            let mut stars = [StarKey::ZiweiMaj; 4];
            for (i, key) in star_keys.iter().enumerate() {
                stars[i] = StarKey::from_key(key).ok_or_else(|| {
                    BridgeError::invalid_argument(format!(
                        "invalid mutagens for '{stem_key}': unknown star '{key}'"
                    ))
                })?;
            }
            config = config.with_mutagens(stem, stars);
        }
    }
    if let Some(map) = patch.brightness {
        for (star_key, brightness_keys) in map {
            let star = StarKey::from_key(&star_key).ok_or_else(|| {
                BridgeError::invalid_argument(format!(
                    "invalid brightness key '{star_key}': unknown star"
                ))
            })?;
            if brightness_keys.len() != 12 {
                return Err(BridgeError::invalid_argument(format!(
                    "invalid brightness for '{star_key}': expected 12 entries, got {}",
                    brightness_keys.len()
                )));
            }
            let mut table = [None; 12];
            for (i, key) in brightness_keys.iter().enumerate() {
                // 空串表示该宫位无亮度
                if key.is_empty() {
                    continue;
                }
                table[i] = Some(Brightness::from_key(key).ok_or_else(|| {
                    BridgeError::invalid_argument(format!(
                        "invalid brightness for '{star_key}': unknown brightness '{key}'"
                    ))
                })?);
            }
            config = config.with_brightness(star, table);
        }
    }

    Ok(config)
}

impl From<Config> for ConfigDto {
    fn from(c: Config) -> Self {
        ConfigDto {
            year_divide: c.year_divide.as_key().to_string(),
            horoscope_divide: c.horoscope_divide.as_key().to_string(),
            age_divide: c.age_divide.as_key().to_string(),
            day_divide: c.day_divide.as_key().to_string(),
            algorithm: c.algorithm.as_key().to_string(),
            astro_type: c.astro_type.as_key().to_string(),
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
        star_type: StarType::as_key(s.star_type).to_string(),
        scope: Scope::as_key(s.scope).to_string(),
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
        star_type: StarType::as_key(s.star_type).to_string(),
        scope: Scope::as_key(s.scope).to_string(),
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

/// 四柱的语言无关标识，与 `pillar` 一一对应。
fn pillar_keys(p: (HeavenlyStem, EarthlyBranch)) -> [String; 2] {
    [p.0.as_key().to_string(), p.1.as_key().to_string()]
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
        mutagen_star_keys: {
            let stars = p.mutagen_stars(&MUTAGEN);
            std::array::from_fn(|i| stars[i].as_key().to_string())
        },
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
                    yearly_keys: pillar_keys(self.raw_dates.chinese_date.yearly),
                    monthly_keys: pillar_keys(self.raw_dates.chinese_date.monthly),
                    daily_keys: pillar_keys(self.raw_dates.chinese_date.daily),
                    hourly_keys: pillar_keys(self.raw_dates.chinese_date.hourly),
                },
            },
            time: self.time.clone(),
            time_range: self.time_range.clone(),
            sign: self.sign.clone(),
            sign_key: self.sign_key.clone(),
            zodiac: self.zodiac.clone(),
            zodiac_key: self.zodiac_key.clone(),
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
            language: lang.as_code().to_string(),
            config: self.config.clone().into(),
        }
    }
}

fn scope_dto(item: &HoroscopeItem, lang: Language) -> HoroscopeScopeDto {
    HoroscopeScopeDto {
        index: item.index,
        name: item.name.clone(),
        name_key: item.name_key.as_key().to_string(),
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
        mutagen_star_keys: item
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

// ============================================================
// 格局
// ============================================================

/// 参与成格的一颗星 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternStarDto {
    /// 语言无关星耀标识
    pub key: String,
    /// 名称（按排盘语言翻译）
    pub name: String,
    /// 落宫索引（0-11，寅宫为 0）
    pub palace_index: usize,
    /// 亮度显示文本，无亮度时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness: Option<String>,
    /// 语言无关亮度标识，无亮度时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brightness_key: Option<String>,
    /// 判定视角下的四化显示文本，无四化时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutagen: Option<String>,
    /// 语言无关四化标识，无四化时省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mutagen_key: Option<String>,
}

/// 一次格局命中 DTO。
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternHitDto {
    /// 语言无关格局标识（如 "zi_fu_tong_gong"）
    pub key: String,
    /// 格局名称（按排盘语言翻译）
    pub name: String,
    /// 判定视角（origin/decadal/yearly/…）。
    ///
    /// 命名是双轨约定（翻译文本用本名、语言无关标识带 `Key` 后缀）的例外：
    /// 视角没有翻译文本形态，本字段直接装语言无关标识而不叫 `scopeKey`，
    /// 与运限流耀条目的 `scope` 键同形。
    pub scope: String,
    /// 成格所在宫位索引（0-11，寅宫为 0）
    pub palace_index: usize,
    /// 成格所在宫位在该视角下的宫名（按排盘语言翻译）
    pub palace_name: String,
    /// 语言无关宫名标识
    pub palace_name_key: String,
    /// 多口径格局命中的口径，单口径省略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
    /// 页面称「破格 / 加杀平常」的条件是否触发
    pub broken: bool,
    /// 参与成格的星与落宫
    pub stars: Vec<PatternStarDto>,
}

impl crate::pattern::PatternHit {
    /// 转为 DTO：`palace_names` 是该视角下按宫位索引排列的十二宫名
    /// （本命为 `Astrolabe.palaces[i].name`，运限为 `HoroscopeItem.palace_names`）。
    pub fn to_dto(&self, palace_names: &[Palace], lang: Language) -> PatternHitDto {
        let palace_name = palace_names
            .get(self.palace)
            .copied()
            .unwrap_or(Palace::Soul);
        PatternHitDto {
            key: self.key.as_key().to_string(),
            name: translate_pattern(self.key, lang).to_string(),
            scope: self.scope.as_key().to_string(),
            palace_index: self.palace,
            palace_name: translate_palace(palace_name, lang).to_string(),
            palace_name_key: palace_name.as_key().to_string(),
            variant: self.variant.map(str::to_string),
            broken: self.broken,
            stars: self
                .stars
                .iter()
                .map(|s| PatternStarDto {
                    key: s.star.as_key().to_string(),
                    name: translate_star(s.star, lang).to_string(),
                    palace_index: s.palace,
                    brightness: s
                        .brightness
                        .map(|b| translate_brightness(b, lang).to_string()),
                    brightness_key: s.brightness.map(|b| b.as_key().to_string()),
                    mutagen: s.mutagen.map(|m| translate_mutagen(m, lang).to_string()),
                    mutagen_key: s.mutagen.map(|m| m.as_key().to_string()),
                })
                .collect(),
        }
    }
}

impl Astrolabe {
    /// 本命格局命中的 DTO 列表。
    pub fn patterns_dto(&self, config: &crate::pattern::PatternConfig) -> Vec<PatternHitDto> {
        let names: Vec<Palace> = self.palaces.iter().map(|p| p.name).collect();
        self.patterns_with(config)
            .iter()
            .map(|h| h.to_dto(&names, self.language))
            .collect()
    }
}

impl HoroscopeData {
    /// 某运限层视角格局命中的 DTO 列表；`Scope::Origin` 等同本命。
    pub fn patterns_dto(
        &self,
        astrolabe: &Astrolabe,
        scope: Scope,
        config: &crate::pattern::PatternConfig,
    ) -> Vec<PatternHitDto> {
        let names: Vec<Palace> = match self.scope_item(scope) {
            Some(item) => item.palace_names.clone(),
            None => astrolabe.palaces.iter().map(|p| p.name).collect(),
        };
        crate::pattern::patterns_at(astrolabe, self, scope, config)
            .iter()
            .map(|h| h.to_dto(&names, astrolabe.language))
            .collect()
    }
}

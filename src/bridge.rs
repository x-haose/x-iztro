//! 绑定层共用的编组与分派
//!
//! Python 与 Go 的对外能力必须一致，这里是保证一致的地方：两侧都把入参落成
//! 本模块的结构体，调用同一个函数，拿到同一个 `serde_json::Value`。
//! 各自的绑定文件只剩下语言特有的部分——wasm 的内存协定、PyO3 的异常类型。
//!
//! 入参键一律 camelCase；标识（星耀、干支、宫位……）收发都用语言无关 key。

use serde::Deserialize;
use serde_json::{Value, json};

use crate::data::constants::*;
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::heavenly_stems::get_heavenly_stem_info;
use crate::data::stars::{STARS_WITH_INFO, StarKey, get_star_info};
use crate::data::types::*;
use crate::dto::parse_config_value;
use crate::error::BridgeError;
use crate::i18n::lookup::{key_of, key_of_in, translate_key};
use crate::models::astrolabe::Astrolabe;
use crate::models::star::Star;
use crate::star::decorative::{get_changsheng12_start_index, get_jiangqian12_start_index};
use crate::star::query::{self as star_query, StarParam};

// ============================================================
// 入参
// ============================================================

/// 阳历排盘入参
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SolarChartInput {
    /// 公历日期，格式 `YYYY-M-D`
    pub solar_date: String,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
    /// 性别，`male` 或 `female`
    pub gender: String,
    /// 是否调整农历闰月（该月非闰月则不生效）
    pub fix_leap: bool,
    /// 输出语言代码，如 `zh-CN`
    pub language: String,
    /// 排盘配置，部分键对象；省略取默认
    pub config: Option<Value>,
    /// 重排起始天干标识；与 `from_branch` 同时给出时按该干支重排星盘
    pub from_stem: Option<String>,
    /// 重排起始地支标识
    pub from_branch: Option<String>,
}

/// 农历排盘入参
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct LunarChartInput {
    /// 农历日期，格式 `YYYY-M-D`
    pub lunar_date: String,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
    /// 性别，`male` 或 `female`
    pub gender: String,
    /// 农历日期是否为闰月
    pub is_leap_month: bool,
    /// 是否调整农历闰月（该月非闰月则不生效）
    pub fix_leap: bool,
    /// 输出语言代码，如 `zh-CN`
    pub language: String,
    /// 排盘配置，部分键对象；省略取默认
    pub config: Option<Value>,
    /// 起五行局的天干标识；与 `from_branch` 同时给出
    pub from_stem: Option<String>,
    /// 起五行局的地支标识；与 `from_stem` 同时给出
    pub from_branch: Option<String>,
}

/// 运限入参：本命盘参数 + 运限目标
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct HoroscopeInput {
    /// 公历日期，格式 `YYYY-M-D`
    pub solar_date: String,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
    /// 性别，`male` 或 `female`
    pub gender: String,
    /// 是否调整农历闰月（该月非闰月则不生效）
    pub fix_leap: bool,
    /// 输出语言代码，如 `zh-CN`
    pub language: String,
    /// 排盘配置，部分键对象；省略取默认
    pub config: Option<Value>,
    /// 运限目标日期，格式 `YYYY-M-D`
    pub target_date: String,
    /// 运限目标时辰索引 0-12
    pub target_time_index: u8,
}

/// 查询入参
///
/// `kind` 决定调用哪个查询，其余字段按需取用——一个入口承载全部查询，
/// 免去为每个函数各开一套编组。
#[derive(Deserialize, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct QueryInput {
    /// 查询种类，决定调用哪个函数
    pub kind: String,

    /// 公历日期，格式 `YYYY-M-D`
    pub solar_date: String,
    /// 农历日期，格式 `YYYY-M-D`
    pub lunar_date: String,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
    /// 农历日期是否为闰月
    pub is_leap_month: bool,
    /// 是否调整农历闰月（该月非闰月则不生效）
    pub fix_leap: bool,
    /// 输出语言代码，如 `zh-CN`
    pub language: String,
    /// 排盘配置，部分键对象；省略取默认
    pub config: Option<Value>,
    /// 性别，`male` 或 `female`
    pub gender: String,

    // 工具函数用
    /// 宫位索引，或 `fixIndex` 的待修正索引
    pub index: i32,
    #[serde(default = "default_max")]
    /// `fixIndex` 的取模上界
    pub max: i32,
    /// 小时数 0-23
    pub hour: u8,
    /// 星耀标识
    pub star_key: String,
    /// 天干标识
    pub stem_key: String,
    /// 地支标识
    pub branch_key: String,
    /// 运限层级标识
    pub scope: String,
    /// 农历月索引（0-based）
    pub month_index: usize,
    /// 命宫宫位索引
    pub soul_index: usize,
    /// 农历月份
    pub lunar_month: u32,
    /// 农历日
    pub lunar_day: u32,
    /// 该农历月是否为闰月
    pub is_leap: bool,
    /// 五行局标识
    pub five_elements_class: String,
    /// 四柱干支标识 [年, 月, 日, 时]，每柱为 [天干, 地支]
    pub pillars: Vec<Vec<String>>,

    // 安星用：中州派从别的干支起五行局
    /// 起五行局的天干标识；与 `from_branch` 同时给出
    pub from_stem: Option<String>,
    /// 起五行局的地支标识；与 `from_stem` 同时给出
    pub from_branch: Option<String>,

    // i18n 用
    /// 待翻译的标识
    pub key: String,
    /// 待反查标识的译名
    pub text: String,
    /// 反查时限定标识名须含的子串；留空则不限定
    pub key_filter: String,

    // Prompt 与运限用
    /// 运限目标日期，格式 `YYYY-M-D`
    pub target_date: String,
    /// 运限目标时辰索引 0-12
    pub target_time_index: u8,

    // 格局用
    /// 格局判定口径，部分键对象（brightnessSource/borrow/flowStars）；省略取默认
    pub pattern_config: Option<Value>,
}

/// `fixIndex` 的 max 默认值，与 iztro 一致
fn default_max() -> i32 {
    12
}

// ============================================================
// 解析
// ============================================================

/// DTO 序列化失败只可能源于库内部缺陷（DTO 全是可序列化的普通结构），
/// 归为 `internal`。
fn serialize_failed(e: serde_json::Error) -> BridgeError {
    BridgeError::internal(format!("failed to serialize result: {e}"))
}

fn parse_gender(s: &str) -> Result<Gender, BridgeError> {
    match s.to_ascii_lowercase().as_str() {
        "male" => Ok(Gender::Male),
        "female" => Ok(Gender::Female),
        _ => Err(BridgeError::invalid_argument(format!(
            "invalid gender '{s}': expected 'male' or 'female'"
        ))),
    }
}

fn parse_language(s: &str) -> Result<Language, BridgeError> {
    Language::from_code(s).ok_or_else(|| {
        BridgeError::invalid_argument(format!(
            "invalid language '{s}': expected one of zh-CN, zh-TW, en-US, ja-JP, ko-KR, vi-VN"
        ))
    })
}

/// 纯计算的查询与输出语言无关，允许省略 language
fn parse_language_or_default(s: &str) -> Result<Language, BridgeError> {
    if s.is_empty() {
        Ok(Language::ZhCN)
    } else {
        parse_language(s)
    }
}

fn parse_config(v: &Option<Value>) -> Result<Config, BridgeError> {
    match v {
        None => Ok(Config::default()),
        Some(Value::Null) => Ok(Config::default()),
        Some(value) => parse_config_value(value),
    }
}

/// 格局判定口径：省略或 null 取默认；对象按 camelCase 部分键解析，未知键报错。
fn parse_pattern_config(v: &Option<Value>) -> Result<crate::pattern::PatternConfig, BridgeError> {
    match v {
        None | Some(Value::Null) => Ok(crate::pattern::PatternConfig::default()),
        Some(value) => serde_json::from_value(value.clone())
            .map_err(|e| BridgeError::invalid_argument(format!("invalid patternConfig: {e}"))),
    }
}

fn parse_star_key(key: &str) -> Result<StarKey, BridgeError> {
    StarKey::from_key(key)
        .ok_or_else(|| BridgeError::invalid_argument(format!("unknown star key '{key}'")))
}

fn parse_stem_key(key: &str) -> Result<HeavenlyStem, BridgeError> {
    HeavenlyStem::from_key(key)
        .ok_or_else(|| BridgeError::invalid_argument(format!("unknown heavenly stem key '{key}'")))
}

fn parse_branch_key(key: &str) -> Result<EarthlyBranch, BridgeError> {
    EarthlyBranch::from_key(key)
        .ok_or_else(|| BridgeError::invalid_argument(format!("unknown earthly branch key '{key}'")))
}

fn parse_scope_key(key: &str) -> Result<Scope, BridgeError> {
    Scope::from_key(key)
        .ok_or_else(|| BridgeError::invalid_argument(format!("unknown scope '{key}'")))
}

/// 解析四柱干支标识：四柱各两项，顺序为年、月、日、时
fn parse_pillars(
    pillars: &[Vec<String>],
) -> Result<[(HeavenlyStem, EarthlyBranch); 4], BridgeError> {
    if pillars.len() != 4 {
        return Err(BridgeError::invalid_argument(format!(
            "invalid pillars: expected 4 entries (yearly, monthly, daily, hourly), got {}",
            pillars.len()
        )));
    }
    let mut out = [(HeavenlyStem::Jia, EarthlyBranch::Zi); 4];
    for (i, pillar) in pillars.iter().enumerate() {
        if pillar.len() != 2 {
            return Err(BridgeError::invalid_argument(format!(
                "invalid pillars[{i}]: expected 2 entries (stem, branch), got {}",
                pillar.len()
            )));
        }
        out[i] = (parse_stem_key(&pillar[0])?, parse_branch_key(&pillar[1])?);
    }
    Ok(out)
}

/// 解析重排起始干支：两者必须同时给出或同时省略
fn parse_from(
    stem: &Option<String>,
    branch: &Option<String>,
) -> Result<Option<(HeavenlyStem, EarthlyBranch)>, BridgeError> {
    match (stem, branch) {
        (None, None) => Ok(None),
        (Some(s), Some(b)) => Ok(Some((
            HeavenlyStem::from_key(s).ok_or_else(|| {
                BridgeError::invalid_argument(format!(
                    "invalid fromStem '{s}': unknown heavenly stem"
                ))
            })?,
            EarthlyBranch::from_key(b).ok_or_else(|| {
                BridgeError::invalid_argument(format!(
                    "invalid fromBranch '{b}': unknown earthly branch"
                ))
            })?,
        ))),
        _ => Err(BridgeError::invalid_argument(
            "invalid rearrange target: fromStem and fromBranch must be given together",
        )),
    }
}

/// 按可选的起始干支重排星盘；两者都未给出时原样返回
fn apply_rearrange(
    astrolabe: Astrolabe,
    stem: &Option<String>,
    branch: &Option<String>,
) -> Result<Astrolabe, BridgeError> {
    match parse_from(stem, branch)? {
        None => Ok(astrolabe),
        Some((s, b)) => Ok(astrolabe.rearranged(s, b)),
    }
}

// ============================================================
// 排盘与运限
// ============================================================

/// 阳历排盘，返回星盘 DTO
pub fn by_solar(input: &SolarChartInput) -> Result<Value, BridgeError> {
    let astrolabe = crate::by_solar(
        &input.solar_date,
        input.time_index,
        parse_gender(&input.gender)?,
        input.fix_leap,
        parse_language(&input.language)?,
        parse_config(&input.config)?,
    )?;
    let astrolabe = apply_rearrange(astrolabe, &input.from_stem, &input.from_branch)?;

    serde_json::to_value(astrolabe.to_dto()).map_err(serialize_failed)
}

/// 农历排盘，返回星盘 DTO
pub fn by_lunar(input: &LunarChartInput) -> Result<Value, BridgeError> {
    let astrolabe = crate::by_lunar(
        &input.lunar_date,
        input.time_index,
        parse_gender(&input.gender)?,
        LeapMonth::from_flags(input.is_leap_month, input.fix_leap),
        parse_language(&input.language)?,
        parse_config(&input.config)?,
    )?;
    let astrolabe = apply_rearrange(astrolabe, &input.from_stem, &input.from_branch)?;

    serde_json::to_value(astrolabe.to_dto()).map_err(serialize_failed)
}

/// 运限，返回运限 DTO
pub fn horoscope(input: &HoroscopeInput) -> Result<Value, BridgeError> {
    let language = parse_language(&input.language)?;
    let astrolabe = crate::by_solar(
        &input.solar_date,
        input.time_index,
        parse_gender(&input.gender)?,
        input.fix_leap,
        language,
        parse_config(&input.config)?,
    )?;
    let horoscope = crate::get_horoscope(
        &astrolabe,
        &input.target_date,
        input.target_time_index,
        language,
    )?;

    serde_json::to_value(horoscope.to_dto(language)).map_err(serialize_failed)
}

// ============================================================
// 查询分派
// ============================================================

/// 按 `kind` 分派查询
///
/// 覆盖 iztro 的 `astro` 轻量查询、`astro/palace`、`util`、`star`、`data`、
/// `i18n` 六组对外函数，以及 x-iztro 自己的 Prompt 生成。
pub fn query(input: &QueryInput) -> Result<Value, BridgeError> {
    match input.kind.as_str() {
        // ---- 轻量查询：结果按语言翻译 ----
        "zodiacBySolar" | "signBySolar" | "signByLunar" | "majorStarBySolar"
        | "majorStarByLunar" | "astrolabeToPrompt" | "horoscopeToPrompt" => translated(input),

        // ---- 格局 ----
        "patterns" | "horoscopePatterns" => patterns(input),

        // ---- util 与 astro/palace ----
        "fixIndex"
        | "earthlyBranchToPalaceIndex"
        | "timeToIndex"
        | "getAgeIndex"
        | "getBrightness"
        | "getMutagen"
        | "getMutagensByHeavenlyStem"
        | "getSoulAndBody"
        | "getFiveElementsClass"
        | "getPalaceNames"
        | "getHoroscope"
        | "fixLunarMonthIndex"
        | "fixLunarDayIndex"
        | "translateChineseDate" => util(input),

        // ---- star ----
        k if k.starts_with("get") && is_star_kind(k) => star(input),

        // ---- data ----
        "starsInfo" | "heavenlyStems" | "earthlyBranches" | "constants" => {
            Ok(data(input.kind.as_str()))
        }

        // ---- i18n ----
        "translate" => Ok(json!(translate_key(
            &input.key,
            parse_language_or_default(&input.language)?
        ))),
        "allKeys" => Ok(json!(crate::i18n::lookup::all_keys())),
        "keyOf" => Ok(json!(if input.key_filter.is_empty() {
            key_of(&input.text)
        } else {
            key_of_in(&input.text, &input.key_filter)
        })),

        other => Err(BridgeError::invalid_argument(format!(
            "unknown query kind '{other}'"
        ))),
    }
}

fn is_star_kind(kind: &str) -> bool {
    matches!(
        kind,
        "getStartIndex"
            | "getLuYangTuoMaIndex"
            | "getKuiYueIndex"
            | "getChangQuIndex"
            | "getKongJieIndex"
            | "getTimelyStarIndex"
            | "getLuanXiIndex"
            | "getDailyStarIndex"
            | "getMonthlyStarIndex"
            | "getYearlyStarIndex"
            | "getMajorStar"
            | "getMinorStar"
            | "getAdjectiveStar"
            | "getChangsheng12"
            | "getBoShi12"
            | "getYearly12"
            | "getHoroscopeStar"
            | "getChangesheng12StartIndex"
            | "getJiangqian12StartIndex"
            | "getChangQuIndexByHeavenlyStem"
            | "getHuoLingIndex"
            | "getZuoYouIndex"
            | "getHuagaiXianchiIndex"
            | "getGuGuaIndex"
            | "getJieshaAdjIndex"
            | "getDahaoIndex"
            | "getNianjieIndex"
            | "getTianshiTianshangIndex"
    )
}

/// 需要排盘的查询：结果都是按语言翻译的字符串
fn translated(input: &QueryInput) -> Result<Value, BridgeError> {
    let language = parse_language(&input.language)?;
    let config = parse_config(&input.config)?;

    let text = match input.kind.as_str() {
        "zodiacBySolar" => crate::get_zodiac_by_solar_date(&input.solar_date, language, config),
        "signBySolar" => crate::get_sign_by_solar_date(&input.solar_date, language),
        "signByLunar" => {
            crate::get_sign_by_lunar_date(&input.lunar_date, input.is_leap_month, language)
        }
        "majorStarBySolar" => crate::get_major_star_by_solar_date(
            &input.solar_date,
            input.time_index,
            input.fix_leap,
            language,
            config,
        ),
        "majorStarByLunar" => crate::get_major_star_by_lunar_date(
            &input.lunar_date,
            input.time_index,
            LeapMonth::from_flags(input.is_leap_month, input.fix_leap),
            language,
            config,
        ),
        "astrolabeToPrompt" => crate::by_solar(
            &input.solar_date,
            input.time_index,
            parse_gender(&input.gender)?,
            input.fix_leap,
            language,
            config,
        )
        .map(|a| crate::astrolabe_to_prompt(&a, language)),
        "horoscopeToPrompt" => crate::by_solar(
            &input.solar_date,
            input.time_index,
            parse_gender(&input.gender)?,
            input.fix_leap,
            language,
            config,
        )
        .and_then(|a| {
            crate::get_horoscope(&a, &input.target_date, input.target_time_index, language)
                .map(|h| crate::horoscope_to_prompt(&a, &h, language))
        }),
        other => {
            return Err(BridgeError::invalid_argument(format!(
                "unknown query kind '{other}'"
            )));
        }
    };

    Ok(json!(text?))
}

/// 格局判定：`patterns` 为本命，`horoscopePatterns` 为运限某层视角（`scope`）。
/// 返回命中 DTO 数组，名称按语言翻译、标识语言无关。
fn patterns(input: &QueryInput) -> Result<Value, BridgeError> {
    let language = parse_language(&input.language)?;
    let config = parse_config(&input.config)?;
    let pattern_config = parse_pattern_config(&input.pattern_config)?;
    let astrolabe = crate::by_solar(
        &input.solar_date,
        input.time_index,
        parse_gender(&input.gender)?,
        input.fix_leap,
        language,
        config,
    )?;
    let hits = match input.kind.as_str() {
        "patterns" => astrolabe.patterns_dto(&pattern_config),
        _ => {
            let scope = parse_scope_key(&input.scope)?;
            let horoscope = crate::get_horoscope(
                &astrolabe,
                &input.target_date,
                input.target_time_index,
                language,
            )?;
            horoscope.patterns_dto(&astrolabe, scope, &pattern_config)
        }
    };
    serde_json::to_value(hits).map_err(serialize_failed)
}

/// 纯计算的工具函数：收发都用语言无关 key
fn util(input: &QueryInput) -> Result<Value, BridgeError> {
    let config = parse_config(&input.config)?;

    let value = match input.kind.as_str() {
        "fixIndex" => {
            if input.max <= 0 {
                return Err(BridgeError::invalid_argument(format!(
                    "invalid max '{}': expected a positive integer",
                    input.max
                )));
            }
            json!(crate::utils::fix_index(input.index, input.max))
        }
        "earthlyBranchToPalaceIndex" => json!(crate::utils::earthly_branch_to_palace_index(
            parse_branch_key(&input.branch_key)?
        )),
        "timeToIndex" => {
            if input.hour > 23 {
                return Err(BridgeError::invalid_argument(format!(
                    "invalid hour '{}': expected 0-23",
                    input.hour
                )));
            }
            json!(crate::utils::time_to_index(input.hour))
        }
        "getAgeIndex" => json!(crate::utils::get_age_index(parse_branch_key(
            &input.branch_key
        )?)),
        "getBrightness" => json!(
            crate::utils::get_brightness(parse_star_key(&input.star_key)?, input.index, &config)
                .map(|b| b.as_key())
        ),
        "getMutagen" => json!(
            crate::utils::get_mutagen(
                parse_star_key(&input.star_key)?,
                parse_stem_key(&input.stem_key)?,
                &config
            )
            .map(|m| m.as_key())
        ),
        "getMutagensByHeavenlyStem" => json!(
            crate::utils::get_mutagens_by_heavenly_stem(parse_stem_key(&input.stem_key)?, &config)
                .iter()
                .map(|s| s.as_key())
                .collect::<Vec<_>>()
        ),
        "getSoulAndBody" => {
            if input.time_index > 12 {
                return Err(BridgeError::invalid_argument(format!(
                    "invalid timeIndex '{}': expected 0-12",
                    input.time_index
                )));
            }
            let r = crate::get_soul_and_body(
                input.month_index,
                input.time_index,
                parse_stem_key(&input.stem_key)?,
            );
            json!({
                "soulIndex": r.soul_index,
                "bodyIndex": r.body_index,
                "heavenlyStemOfSoul": r.heavenly_stem_of_soul.as_key(),
                "earthlyBranchOfSoul": r.earthly_branch_of_soul.as_key(),
            })
        }
        "getFiveElementsClass" => json!(
            crate::get_five_elements_class(
                parse_stem_key(&input.stem_key)?,
                parse_branch_key(&input.branch_key)?
            )
            .as_key()
        ),
        "getPalaceNames" => json!(
            crate::get_palace_names(input.soul_index)
                .iter()
                .map(|p| p.as_key())
                .collect::<Vec<_>>()
        ),
        "getHoroscope" => {
            let (decadals, ages) = crate::astro::palace::get_decadals_and_ages(
                input.soul_index,
                parse_five_elements_class(&input.five_elements_class)?,
                parse_gender(&input.gender)?,
                parse_stem_key(&input.stem_key)?,
                parse_branch_key(&input.branch_key)?,
            );
            // 字段形状与星盘上的大限一致：译名与标识并列，
            // 免得同名字段在两处装不同的东西。
            let language = parse_language_or_default(&input.language)?;
            json!({
                "decadals": decadals.iter().map(|d| json!({
                    "range": d.range,
                    "heavenlyStem": crate::i18n::translate_heavenly_stem(d.heavenly_stem, language),
                    "heavenlyStemKey": d.heavenly_stem.as_key(),
                    "earthlyBranch": crate::i18n::translate_earthly_branch(d.earthly_branch, language),
                    "earthlyBranchKey": d.earthly_branch.as_key(),
                })).collect::<Vec<_>>(),
                "ages": ages,
            })
        }
        "fixLunarMonthIndex" => json!(crate::astro::builder::fix_lunar_month_index(
            input.lunar_month,
            input.lunar_day,
            input.is_leap,
            input.time_index,
            input.fix_leap,
        )),
        "fixLunarDayIndex" => json!(crate::astro::builder::fix_lunar_day_index(
            input.lunar_day,
            input.time_index
        )),
        "translateChineseDate" => json!(crate::utils::translate_chinese_date(
            parse_pillars(&input.pillars)?,
            parse_language(&input.language)?
        )),
        other => {
            return Err(BridgeError::invalid_argument(format!(
                "unknown query kind '{other}'"
            )));
        }
    };

    Ok(value)
}

fn parse_five_elements_class(key: &str) -> Result<FiveElementsClass, BridgeError> {
    FiveElementsClass::from_key(key).ok_or_else(|| {
        BridgeError::invalid_argument(format!("unknown five elements class '{key}'"))
    })
}

/// 十二宫星耀 → 每宫的标识与译名
fn star_groups(groups: &[Vec<Star>; 12]) -> Value {
    json!(
        groups
            .iter()
            .map(|palace| palace
                .iter()
                .map(|s| json!({
                    "key": s.key.as_key(),
                    "name": s.name,
                    "type": s.star_type.as_key(),
                    "scope": s.scope.as_key(),
                }))
                .collect::<Vec<_>>())
            .collect::<Vec<_>>()
    )
}

fn shen_keys(shen: &[StarKey; 12]) -> Value {
    json!(shen.iter().map(|k| k.as_key()).collect::<Vec<_>>())
}

/// 安星查询分派
fn star(input: &QueryInput) -> Result<Value, BridgeError> {
    let config = parse_config(&input.config)?;
    let language = parse_language_or_default(&input.language)?;

    // 不依赖出生数据的几个：直接由标识算
    match input.kind.as_str() {
        "getChangesheng12StartIndex" => {
            return Ok(json!(get_changsheng12_start_index(
                parse_five_elements_class(&input.five_elements_class)?
            )));
        }
        "getJiangqian12StartIndex" => {
            return Ok(json!(get_jiangqian12_start_index(parse_branch_key(
                &input.branch_key
            )?)));
        }
        "getHoroscopeStar" => {
            return Ok(star_groups(&crate::astro::horoscope::get_horoscope_stars(
                parse_stem_key(&input.stem_key)?,
                parse_branch_key(&input.branch_key)?,
                parse_scope_key(&input.scope)?,
                language,
            )));
        }
        "getChangQuIndexByHeavenlyStem" => {
            let r =
                crate::star::location::get_chang_qu_index_by_stem(parse_stem_key(&input.stem_key)?);
            return Ok(json!({ "changIndex": r.chang, "quIndex": r.qu }));
        }
        "getZuoYouIndex" => {
            let r = crate::star::location::get_zuo_you_index(input.lunar_month);
            return Ok(json!({ "zuoIndex": r.zuo, "youIndex": r.you }));
        }
        "getHuoLingIndex" => {
            let r = crate::star::location::get_huo_ling_index(
                parse_branch_key(&input.branch_key)?,
                input.time_index,
            );
            return Ok(json!({ "huoIndex": r.huo, "lingIndex": r.ling }));
        }
        "getHuagaiXianchiIndex" => {
            let r = crate::star::location::get_huagai_xianchi_index(parse_branch_key(
                &input.branch_key,
            )?);
            return Ok(json!({ "huagaiIndex": r.huagai, "xianchiIndex": r.xianchi }));
        }
        "getGuGuaIndex" => {
            let r = crate::star::location::get_gu_gua_index(parse_branch_key(&input.branch_key)?);
            return Ok(json!({ "guchenIndex": r.guchen, "guasuIndex": r.guasu }));
        }
        "getJieshaAdjIndex" => {
            return Ok(json!(crate::star::location::get_jiesha_adj_index(
                parse_branch_key(&input.branch_key)?
            )));
        }
        "getDahaoIndex" => {
            return Ok(json!(crate::star::location::get_dahao_index(
                parse_branch_key(&input.branch_key)?
            )));
        }
        "getNianjieIndex" => {
            return Ok(json!(crate::star::location::get_nianjie_index(
                parse_branch_key(&input.branch_key)?
            )));
        }
        "getTianshiTianshangIndex" => {
            let (shang, shi) = crate::star::location::get_tianshang_tianshi_index(
                parse_gender(&input.gender)?,
                parse_branch_key(&input.branch_key)?,
                input.soul_index,
                config.algorithm,
            );
            return Ok(json!({ "tianshangIndex": shang, "tianshiIndex": shi }));
        }
        _ => {}
    }

    // 其余按出生数据安星
    let param = StarParam {
        solar_date: &input.solar_date,
        time_index: input.time_index,
        gender: parse_gender(&input.gender)?,
        fix_leap: input.fix_leap,
        from: parse_from(&input.from_stem, &input.from_branch)?,
        language,
        config: &config,
    };
    let value = match input.kind.as_str() {
        "getStartIndex" => {
            let r = star_query::get_start_index(&param)?;
            json!({ "ziweiIndex": r.ziwei, "tianfuIndex": r.tianfu })
        }
        "getLuYangTuoMaIndex" => {
            let r = star_query::get_lu_yang_tuo_ma_index(&param)?;
            json!({ "luIndex": r.lu, "yangIndex": r.yang, "tuoIndex": r.tuo, "maIndex": r.ma })
        }
        "getKuiYueIndex" => {
            let r = star_query::get_kui_yue_index(&param)?;
            json!({ "kuiIndex": r.kui, "yueIndex": r.yue })
        }
        "getChangQuIndex" => {
            let r = star_query::get_chang_qu_index(&param)?;
            json!({ "changIndex": r.chang, "quIndex": r.qu })
        }
        "getKongJieIndex" => {
            let r = star_query::get_kong_jie_index(&param)?;
            json!({ "kongIndex": r.kong, "jieIndex": r.jie })
        }
        "getTimelyStarIndex" => {
            let r = star_query::get_timely_star_index(&param)?;
            json!({ "taifuIndex": r.taifu, "fenggaoIndex": r.fenggao })
        }
        "getLuanXiIndex" => {
            let r = star_query::get_luan_xi_index(&param)?;
            json!({ "hongluanIndex": r.hongluan, "tianxiIndex": r.tianxi })
        }
        "getDailyStarIndex" => {
            let r = star_query::get_daily_star_index(&param)?;
            json!({
                "santaiIndex": r.santai,
                "bazuoIndex": r.bazuo,
                "enguangIndex": r.enguang,
                "tianguiIndex": r.tiangui,
            })
        }
        "getMonthlyStarIndex" => {
            let r = star_query::get_monthly_star_index(&param)?;
            json!({
                "yuejieIndex": r.jieshen,
                "tianyaoIndex": r.tianyao,
                "tianxingIndex": r.tianxing,
                "yinshaIndex": r.yinsha,
                "tianyueIndex": r.tianyue,
                "tianwuIndex": r.tianwu,
            })
        }
        "getYearlyStarIndex" => {
            let r = star_query::get_yearly_star_index(&param)?;
            json!({
                "xianchiIndex": r.xianchi,
                "huagaiIndex": r.huagai,
                "guchenIndex": r.guchen,
                "guasuIndex": r.guasu,
                "tiancaiIndex": r.tiancai,
                "tianshouIndex": r.tianshou,
                "tianchuIndex": r.tianchu,
                "posuiIndex": r.posui,
                "feilianIndex": r.feilian,
                "longchiIndex": r.longchi,
                "fenggeIndex": r.fengge,
                "tiankuIndex": r.tianku,
                "tianxuIndex": r.tianxu,
                "tianguanIndex": r.tianguan,
                "tianfuIndex": r.tianfu,
                "tiandeIndex": r.tiande,
                "yuedeIndex": r.yuede,
                "tiankongIndex": r.tiankong,
                "jieluIndex": r.jielu,
                "kongwangIndex": r.kongwang,
                "xunkongIndex": r.xunkong,
                "tianshangIndex": r.tianshang,
                "tianshiIndex": r.tianshi,
                "jiekongIndex": r.jiekong,
                "jieshaAdjIndex": r.jiesha,
                "nianjieIndex": r.nianjie,
                "dahaoAdjIndex": r.dahao,
            })
        }
        "getMajorStar" => star_groups(&star_query::get_major_stars(&param)?),
        "getMinorStar" => star_groups(&star_query::get_minor_stars(&param)?),
        "getAdjectiveStar" => star_groups(&star_query::get_adjective_stars(&param)?),
        "getChangsheng12" => shen_keys(&star_query::get_changsheng12(&param)?),
        "getBoShi12" => shen_keys(&star_query::get_boshi12(&param)?),
        "getYearly12" => {
            let (suiqian12, jiangqian12) = star_query::get_yearly12(&param)?;
            json!({ "suiqian12": shen_keys(&suiqian12), "jiangqian12": shen_keys(&jiangqian12) })
        }
        other => {
            return Err(BridgeError::invalid_argument(format!(
                "unknown query kind '{other}'"
            )));
        }
    };

    Ok(value)
}

// ============================================================
// 数据表
// ============================================================

/// 导出 `data` 模块的查表，键与取值一律语言无关
fn data(kind: &str) -> Value {
    match kind {
        "starsInfo" => json!(
            STARS_WITH_INFO
                .iter()
                .map(|star| {
                    let info = get_star_info(*star).expect("STARS_WITH_INFO 内必有记录");
                    (
                        star.as_key().to_string(),
                        json!({
                            "brightness": info.brightness.iter()
                                .map(|b| b.map(|b| b.as_key()))
                                .collect::<Vec<_>>(),
                            "fiveElements": info.five_elements.map(|f| f.as_str()),
                            "yinYang": info.yin_yang.map(|y| y.as_str()),
                        }),
                    )
                })
                .collect::<serde_json::Map<_, _>>()
        ),
        "heavenlyStems" => json!(
            HEAVENLY_STEMS
                .iter()
                .map(|stem| {
                    let info = get_heavenly_stem_info(*stem);
                    (
                        stem.as_key().to_string(),
                        json!({
                            "yinYang": info.yin_yang.as_str(),
                            "fiveElements": info.five_elements.as_str(),
                            "crash": info.crash.map(|c| c.as_key()),
                            "mutagen": info.mutagen.iter().map(|s| s.as_key()).collect::<Vec<_>>(),
                        }),
                    )
                })
                .collect::<serde_json::Map<_, _>>()
        ),
        "earthlyBranches" => json!(
            EARTHLY_BRANCHES
                .iter()
                .map(|branch| {
                    let info = get_earthly_branch_info(*branch);
                    (
                        branch.as_key().to_string(),
                        json!({
                            "yinYang": info.yin_yang.as_str(),
                            "fiveElements": info.five_elements.as_str(),
                            "crash": info.crash.as_key(),
                            "soul": info.soul.as_key(),
                            "body": info.body.as_key(),
                            "inside": info.inside,
                            "outside": info.outside,
                            "healthTip": info.health_tip,
                        }),
                    )
                })
                .collect::<serde_json::Map<_, _>>()
        ),
        _ => json!({
            "LANGUAGES": LANGUAGES,
            "HEAVENLY_STEMS": HEAVENLY_STEMS.iter().map(|s| s.as_key()).collect::<Vec<_>>(),
            "EARTHLY_BRANCHES": EARTHLY_BRANCHES.iter().map(|b| b.as_key()).collect::<Vec<_>>(),
            "ZODIAC": ZODIAC,
            "SIGNS": SIGNS,
            "PALACES": PALACES.iter().map(|p| p.as_key()).collect::<Vec<_>>(),
            "GENDER": {
                "male": Gender::Male.yin_yang().as_str(),
                "female": Gender::Female.yin_yang().as_str(),
            },
            "CHINESE_TIME": CHINESE_TIME,
            "TIME_RANGE": TIME_RANGES,
            "TIGER_RULE": rule_map(&TIGER_RULE),
            "RAT_RULE": rule_map(&RAT_RULE),
            "MUTAGEN": crate::data::stars::MUTAGEN.iter().map(|m| m.as_key()).collect::<Vec<_>>(),
            // 五行局标识 → 局数：局数即大限每步的年数，也是紫微星起盘的除数
            "FIVE_ELEMENTS_CLASS": FIVE_ELEMENTS_CLASSES
                .iter()
                .map(|c| (c.as_key().to_string(), json!(*c as u8)))
                .collect::<serde_json::Map<_, _>>(),
        }),
    }
}

/// 五虎遁、五鼠遁：按天干标识索引的映射
fn rule_map(rule: &[HeavenlyStem; 10]) -> Value {
    json!(
        HEAVENLY_STEMS
            .iter()
            .zip(rule.iter())
            .map(|(from, to)| (from.as_key().to_string(), json!(to.as_key())))
            .collect::<serde_json::Map<_, _>>()
    )
}

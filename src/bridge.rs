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
    /// 重排起点天干标识；与 `from_branch` 同时给出时先按该干支重排星盘再算运限
    pub from_stem: Option<String>,
    /// 重排起点地支标识；与 `from_stem` 同时给出
    pub from_branch: Option<String>,
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
    /// 宫位标识：宫名 key（`soulPalace` 等）或 `bodyPalace`/`originalPalace`；
    /// 与 `palace_index` 二选一，都缺省即报错——静默取默认宫会把漏传变成错答案
    pub palace_key: String,
    /// 盘上宫位索引（0-11，寅宫为 0）；与 `palace_key` 二选一，后者优先
    pub palace_index: Option<i64>,
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

    // 安星与格局用：中州派安星从别的干支起五行局；格局判定按该干支重排后再判
    /// 起五行局/重排起点的天干标识；与 `from_branch` 同时给出
    pub from_stem: Option<String>,
    /// 起五行局/重排起点的地支标识；与 `from_stem` 同时给出
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

    // 知识包用
    /// 待合并的知识包对象列表：第一个为底包，其余依次作为覆盖包
    pub knowledge_packs: Vec<Value>,

    // 反推用
    /// 反推范围起始公历年（含）
    pub start_year: i64,
    /// 反推范围截止公历年（含）
    pub end_year: i64,
    /// 星盘特征反推的条件对象（camelCase 键，含 yearRange/limit 等）
    pub reverse_criteria: Option<Value>,
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

/// 要求 JSON 值为字符串并取出，否则报带实际值的 invalid_argument。
fn expect_str<'a>(v: &'a Value, name: &str) -> Result<&'a str, BridgeError> {
    v.as_str().ok_or_else(|| {
        BridgeError::invalid_argument(format!("{name} must be a string key, got {v}"))
    })
}

/// 星盘特征反推条件：标识字段一律收语言无关 key。
///
/// 未知键与错型键都报错——静默回落默认值会把拼写/类型错误变成貌似成功的
/// 错答案；每条错误信息都带上实际收到的值。
fn parse_reverse_criteria(
    v: &Value,
) -> Result<crate::astro::reverse::ReverseCriteria, BridgeError> {
    use crate::astro::reverse::{ReverseCriteria, StarPosition};
    let obj = v.as_object().ok_or_else(|| {
        BridgeError::invalid_argument("reverseCriteria must be an object".to_string())
    })?;
    const KNOWN: [&str; 8] = [
        "soulBranch",
        "bodyBranch",
        "fiveElementsClass",
        "stars",
        "mutagens",
        "yearRange",
        "fixLeap",
        "limit",
    ];
    if let Some(unknown) = obj.keys().find(|k| !KNOWN.contains(&k.as_str())) {
        return Err(BridgeError::invalid_argument(format!(
            "unknown reverseCriteria key '{unknown}'"
        )));
    }
    let mut criteria = ReverseCriteria::default();
    let branch = |v: &Value, name: &str| -> Result<EarthlyBranch, BridgeError> {
        parse_branch_key(expect_str(v, name)?)
            .map_err(|e| BridgeError::invalid_argument(format!("{name}: {e}")))
    };
    if let Some(v) = obj.get("soulBranch").filter(|v| !v.is_null()) {
        criteria.soul_branch = Some(branch(v, "soulBranch")?);
    }
    if let Some(v) = obj.get("bodyBranch").filter(|v| !v.is_null()) {
        criteria.body_branch = Some(branch(v, "bodyBranch")?);
    }
    if let Some(v) = obj.get("fiveElementsClass").filter(|v| !v.is_null()) {
        let key = expect_str(v, "fiveElementsClass")?;
        criteria.five_elements_class = Some(FiveElementsClass::from_key(key).ok_or_else(|| {
            BridgeError::invalid_argument(format!("unknown five elements class key '{key}'"))
        })?);
    }
    if let Some(v) = obj.get("stars").filter(|v| !v.is_null()) {
        for (i, item) in v
            .as_array()
            .ok_or_else(|| BridgeError::invalid_argument("stars must be an array".to_string()))?
            .iter()
            .enumerate()
        {
            if !item.is_object() {
                return Err(BridgeError::invalid_argument(format!(
                    "stars[{i}] must be an object with star and branch keys, got {item}"
                )));
            }
            criteria.stars.push(StarPosition {
                star: parse_star_key(expect_str(&item["star"], "stars[].star")?)?,
                branch: branch(&item["branch"], "stars[].branch")?,
            });
        }
    }
    if let Some(v) = obj.get("mutagens").filter(|v| !v.is_null()) {
        let arr = v.as_array().ok_or_else(|| {
            BridgeError::invalid_argument("mutagens must be an array of 4".to_string())
        })?;
        if arr.len() != 4 {
            return Err(BridgeError::invalid_argument(
                "mutagens must have exactly 4 entries (lu, quan, ke, ji)".to_string(),
            ));
        }
        for (slot, item) in criteria.mutagens.iter_mut().zip(arr) {
            if !item.is_null() {
                *slot = Some(parse_star_key(expect_str(item, "mutagens[]")?)?);
            }
        }
    }
    if let Some(v) = obj.get("yearRange").filter(|v| !v.is_null()) {
        let arr = v.as_array().filter(|a| a.len() == 2).ok_or_else(|| {
            BridgeError::invalid_argument(format!(
                "yearRange must be [startYear, endYear], got {v}"
            ))
        })?;
        let year = |v: &Value| -> Result<i64, BridgeError> {
            v.as_i64().ok_or_else(|| {
                BridgeError::invalid_argument(format!(
                    "yearRange entries must be integer years, got {v}"
                ))
            })
        };
        criteria.year_range = (year(&arr[0])?, year(&arr[1])?);
    }
    if let Some(v) = obj.get("fixLeap").filter(|v| !v.is_null()) {
        criteria.fix_leap = v.as_bool().ok_or_else(|| {
            BridgeError::invalid_argument(format!("fixLeap must be a boolean, got {v}"))
        })?;
    }
    if let Some(v) = obj.get("limit").filter(|v| !v.is_null()) {
        let n = v.as_u64().ok_or_else(|| {
            BridgeError::invalid_argument(format!("limit must be a non-negative integer, got {v}"))
        })?;
        // wasm32 上 usize 为 32 位，超出即报错而不是截断成另一个上限
        criteria.limit = usize::try_from(n)
            .map_err(|_| BridgeError::invalid_argument(format!("limit {n} is out of range")))?;
    }
    Ok(criteria)
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
    HeavenlyStem::from_key(key).ok_or_else(|| {
        BridgeError::invalid_argument(format!(
            "unknown heavenly stem key '{key}' (expect language-neutral keys like 'jiaHeavenly', not translated names)"
        ))
    })
}

fn parse_branch_key(key: &str) -> Result<EarthlyBranch, BridgeError> {
    EarthlyBranch::from_key(key).ok_or_else(|| {
        BridgeError::invalid_argument(format!(
            "unknown earthly branch key '{key}' (expect language-neutral keys like 'ziEarthly', not translated names)"
        ))
    })
}

fn parse_scope_key(key: &str) -> Result<Scope, BridgeError> {
    Scope::from_key(key).ok_or_else(|| {
        BridgeError::invalid_argument(format!(
            "unknown scope '{key}' (expected one of: origin, decadal, yearly, monthly, daily, hourly)"
        ))
    })
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
        Some((s, b)) => Ok(astrolabe.rearranged(s, b)?),
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

/// 运限，返回运限 DTO；给出 `fromStem`/`fromBranch` 时先按该干支重排星盘再算——
/// 重排改变五行局与命宫，大限步长与各层宫名随之不同。
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
    let astrolabe = apply_rearrange(astrolabe, &input.from_stem, &input.from_branch)?;
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
        | "majorStarByLunar" => translated(input),

        // ---- 语义化文本（to_text）----
        "astrolabeToText"
        | "horoscopeToText"
        | "palaceToText"
        | "surroundedPalacesToText"
        | "patternsToText"
        | "horoscopePatternsToText" => to_text(input),

        // ---- 格局 ----
        "patterns" | "horoscopePatterns" => patterns(input),

        // ---- 反推 ----
        "solarDatesByBazi" => {
            let config = parse_config(&input.config)?;
            let p = parse_pillars(&input.pillars)?;
            let got = crate::astro::reverse::solar_dates_by_bazi(
                p[0],
                p[1],
                p[2],
                p[3],
                (input.start_year, input.end_year),
                &config,
            )?;
            serde_json::to_value(got).map_err(serialize_failed)
        }
        "reverseChart" => {
            let config = parse_config(&input.config)?;
            let criteria = match &input.reverse_criteria {
                Some(v) => parse_reverse_criteria(v)?,
                None => {
                    return Err(BridgeError::invalid_argument("reverseCriteria is required"));
                }
            };
            let got = crate::astro::reverse::reverse_chart(&criteria, &config)?;
            serde_json::to_value(got).map_err(serialize_failed)
        }

        // ---- 知识包 ----
        "knowledgePack" => {
            let language = parse_language(&input.language)?;
            let json =
                crate::knowledge::KnowledgePack::builtin_json(language).ok_or_else(|| {
                    BridgeError::invalid_argument(format!(
                        "no builtin knowledge pack for language '{}'",
                        input.language
                    ))
                })?;
            serde_json::from_str(json).map_err(serialize_failed)
        }
        "mergeKnowledgePacks" => merge_knowledge_packs(&input.knowledge_packs),

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
        "starsInfo"
        | "heavenlyStems"
        | "earthlyBranches"
        | "constants"
        | "flowStarCounterparts" => Ok(data(input.kind.as_str())),

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

/// 需要排盘的轻量查询：返回 `{text, keys}` 双轨——`text` 按语言翻译
/// （与 iztro 同名函数的返回一致），`keys` 是同一结果的语言无关标识列表
fn translated(input: &QueryInput) -> Result<Value, BridgeError> {
    use crate::astro::query::{
        major_star_chart_by_lunar, major_star_chart_by_solar, major_star_keys_of_soul_palace,
        major_stars_of_soul_palace, sign_chart_by_lunar, sign_chart_by_solar, zodiac_chart,
    };

    let language = parse_language(&input.language)?;
    let config = parse_config(&input.config)?;

    // 排盘配方（固定时辰/性别/闰月口径）与 query.rs 的同名查询共用一份，
    // 保证绑定层与 Rust 原生入口对同一日期永远同答案
    let (text, keys) = match input.kind.as_str() {
        "zodiacBySolar" => {
            let a = zodiac_chart(&input.solar_date, language, config)?;
            (a.zodiac, vec![a.zodiac_key])
        }
        "signBySolar" => {
            let a = sign_chart_by_solar(&input.solar_date, language)?;
            (a.sign, vec![a.sign_key])
        }
        "signByLunar" => {
            let a = sign_chart_by_lunar(&input.lunar_date, input.is_leap_month, language)?;
            (a.sign, vec![a.sign_key])
        }
        "majorStarBySolar" => {
            let a = major_star_chart_by_solar(
                &input.solar_date,
                input.time_index,
                input.fix_leap,
                language,
                config,
            )?;
            (
                major_stars_of_soul_palace(&a),
                major_star_keys_of_soul_palace(&a),
            )
        }
        "majorStarByLunar" => {
            let a = major_star_chart_by_lunar(
                &input.lunar_date,
                input.time_index,
                LeapMonth::from_flags(input.is_leap_month, input.fix_leap),
                language,
                config,
            )?;
            (
                major_stars_of_soul_palace(&a),
                major_star_keys_of_soul_palace(&a),
            )
        }
        other => {
            return Err(BridgeError::invalid_argument(format!(
                "unknown query kind '{other}'"
            )));
        }
    };

    Ok(json!({ "text": text, "keys": keys }))
}

/// 宫位寻址：`palace_key` 非空时按宫名 key 或 `bodyPalace`/`originalPalace` 定位，
/// 留空时按 `index`（0-11）取宫
fn parse_palace_target(
    input: &QueryInput,
) -> Result<crate::models::astrolabe::PalaceTarget, BridgeError> {
    use crate::models::astrolabe::PalaceTarget;
    if input.palace_key.is_empty() {
        // 显式寻址是硬要求：缺省时静默取某个默认宫，会把「漏传/键名拼错」
        // 变成貌似成功的错答案
        let raw = input.palace_index.ok_or_else(|| {
            BridgeError::invalid_argument(
                "palace addressing is required: pass 'palaceKey' or 'palaceIndex'",
            )
        })?;
        let index = usize::try_from(raw)
            .ok()
            .filter(|i| *i < 12)
            .ok_or_else(|| {
                BridgeError::invalid_argument(format!("invalid palaceIndex '{raw}': expected 0-11"))
            })?;
        return Ok(PalaceTarget::Index(index));
    }
    match input.palace_key.as_str() {
        "bodyPalace" => Ok(PalaceTarget::Body),
        "originalPalace" => Ok(PalaceTarget::Original),
        key => Palace::from_key(key).map(PalaceTarget::Name).ok_or_else(|| {
            BridgeError::invalid_argument(format!(
                "invalid palaceKey '{key}': expected a palace name key like 'soulPalace', or 'bodyPalace'/'originalPalace'"
            ))
        }),
    }
}

/// 语义化文本（to_text）：走完整排盘；给出 `fromStem`/`fromBranch` 时先重排再生成，
/// 与 patterns/horoscope 的重排语义一致
fn to_text(input: &QueryInput) -> Result<Value, BridgeError> {
    let language = parse_language(&input.language)?;
    let config = parse_config(&input.config)?;
    let astrolabe = crate::by_solar(
        &input.solar_date,
        input.time_index,
        parse_gender(&input.gender)?,
        input.fix_leap,
        language,
        config,
    )?;
    let astrolabe = apply_rearrange(astrolabe, &input.from_stem, &input.from_branch)?;

    let text = match input.kind.as_str() {
        "astrolabeToText" => crate::astrolabe_to_text(&astrolabe, language),
        "horoscopeToText" => {
            let h = crate::get_horoscope(
                &astrolabe,
                &input.target_date,
                input.target_time_index,
                language,
            )?;
            crate::horoscope_to_text(&astrolabe, &h, language)
        }
        "palaceToText" => {
            let target = parse_palace_target(input)?;
            let p = astrolabe.palace(target).ok_or_else(|| {
                BridgeError::invalid_argument("palace not found on this chart".to_string())
            })?;
            crate::palace_to_text(&p, language)
        }
        "surroundedPalacesToText" => {
            let target = parse_palace_target(input)?;
            let sp = astrolabe.surrounded_palaces(target).ok_or_else(|| {
                BridgeError::invalid_argument("palace not found on this chart".to_string())
            })?;
            crate::surrounded_palaces_to_text(&sp, language)
        }
        "patternsToText" => {
            let pattern_config = parse_pattern_config(&input.pattern_config)?;
            let hits = astrolabe.patterns_with(&pattern_config);
            let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
            crate::patterns_to_text(&hits, &names, language)
        }
        "horoscopePatternsToText" => {
            let pattern_config = parse_pattern_config(&input.pattern_config)?;
            let scope = parse_scope_key(&input.scope)?;
            let h = crate::get_horoscope(
                &astrolabe,
                &input.target_date,
                input.target_time_index,
                language,
            )?;
            let hits = crate::pattern::patterns_at(&astrolabe, &h, scope, &pattern_config);
            // 宫名按判定视角取：本命视角用本命宫名，运限层用该层重排宫名
            let names: Vec<Palace> = match h.scope_item(scope) {
                Some(item) => item.palace_names.clone(),
                None => astrolabe.palaces.iter().map(|p| p.name).collect(),
            };
            crate::patterns_to_text(&hits, &names, language)
        }
        other => {
            return Err(BridgeError::internal(format!(
                "to_text dispatched with non-text kind '{other}'"
            )));
        }
    };

    Ok(json!(text))
}

/// 格局判定：`patterns` 为本命，`horoscopePatterns` 为运限某层视角（`scope`）。
/// 给出 `fromStem`/`fromBranch` 时先按该干支重排星盘再判——重排改变五行局与
/// 十二宫名，格局结论随之不同，判原盘会静默给出错的盘的答案。
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
    let astrolabe = apply_rearrange(astrolabe, &input.from_stem, &input.from_branch)?;
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

/// 合并知识包：第一个为底包，其余依次覆盖；返回合并后的包对象。
fn merge_knowledge_packs(packs: &[Value]) -> Result<Value, BridgeError> {
    let mut parsed = packs.iter().map(|v| {
        crate::knowledge::KnowledgePack::from_json(&v.to_string())
            .map_err(BridgeError::invalid_argument)
    });
    let Some(base) = parsed.next() else {
        return Err(BridgeError::invalid_argument(
            "knowledgePacks must contain at least one pack",
        ));
    };
    let mut base = base?;
    for overlay in parsed {
        base.merge(&overlay?);
    }
    serde_json::to_value(base).map_err(serialize_failed)
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
        // 流耀 → 对应本命辅星的全量对照（50 条）：流耀没有知识包条目，
        // 释义按对应本命辅星查，这张表就是官方对照
        "flowStarCounterparts" => json!(
            crate::astro::horoscope::flow_star_counterparts()
                .into_iter()
                .map(|(flow, natal)| (flow.as_key().to_string(), natal.as_key().to_string()))
                .collect::<std::collections::BTreeMap<_, _>>()
        ),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 错型的反推条件必须报 invalid_argument，且信息里带上实际收到的值。
    #[test]
    fn reverse_criteria_rejects_wrong_types() {
        let cases: &[(Value, &str)] = &[
            (json!({"fixLeap": 0}), "fixLeap must be a boolean, got 0"),
            (
                json!({"limit": "5"}),
                "limit must be a non-negative integer, got \"5\"",
            ),
            (
                json!({"limit": -1}),
                "limit must be a non-negative integer, got -1",
            ),
            (
                json!({"yearRange": "1990-2000"}),
                "yearRange must be [startYear, endYear], got \"1990-2000\"",
            ),
            (
                json!({"yearRange": [1990, "2000"]}),
                "yearRange entries must be integer years, got \"2000\"",
            ),
            (
                json!({"soulBranch": 5}),
                "soulBranch must be a string key, got 5",
            ),
            (
                json!({"fiveElementsClass": true}),
                "fiveElementsClass must be a string key, got true",
            ),
            (
                json!({"mutagens": ["ziweiMaj", 1, null, null]}),
                "mutagens[] must be a string key, got 1",
            ),
            (
                json!({"stars": [42]}),
                "stars[0] must be an object with star and branch keys, got 42",
            ),
            (
                json!({"stars": [{"star": "ziweiMaj"}]}),
                "stars[].branch must be a string key, got null",
            ),
        ];
        for (criteria, want) in cases {
            let err = parse_reverse_criteria(criteria).unwrap_err();
            assert!(
                err.message.contains(want),
                "criteria {criteria}: got message {:?}, want it to contain {want:?}",
                err.message
            );
        }
    }

    /// `patterns` kind 带 `fromStem`/`fromBranch` 时按重排后的盘判定，
    /// 与 Rust 原生「rearranged 后调 patterns_dto」逐字节一致。
    #[test]
    fn patterns_kind_applies_rearrange() {
        let q = |v: Value| query(&serde_json::from_value::<QueryInput>(v).unwrap()).unwrap();
        let base = json!({
            "kind": "patterns",
            "solarDate": "1990-4-23",
            "timeIndex": 2,
            "gender": "male",
            "language": "zh-CN",
        });
        let plain = q(base.clone());
        let mut with_from = base;
        with_from["fromStem"] = json!("gengHeavenly");
        with_from["fromBranch"] = json!("chenEarthly");
        let rearranged = q(with_from);
        assert_ne!(plain, rearranged, "重排后的格局结论应与原盘不同");

        let native = crate::by_solar(
            "1990-4-23",
            2,
            Gender::Male,
            false,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap()
        .rearranged(
            HeavenlyStem::from_key("gengHeavenly").unwrap(),
            EarthlyBranch::from_key("chenEarthly").unwrap(),
        )
        .unwrap()
        .patterns_dto(&crate::pattern::PatternConfig::default());
        assert_eq!(rearranged, serde_json::to_value(native).unwrap());
    }

    /// `horoscope` kind 与各 to_text kind 带 `fromStem`/`fromBranch` 时按重排后的盘计算，
    /// 运限结果与 Rust 原生「rearranged 后调 get_horoscope」逐字节一致。
    #[test]
    fn horoscope_and_text_kinds_apply_rearrange() {
        let base = json!({
            "solarDate": "1990-4-23",
            "timeIndex": 2,
            "gender": "male",
            "language": "zh-CN",
            "targetDate": "2024-6-1",
            "targetTimeIndex": 0,
        });
        let run = |v: &Value| {
            horoscope(&serde_json::from_value::<HoroscopeInput>(v.clone()).unwrap()).unwrap()
        };
        let plain = run(&base);
        let mut with_from = base.clone();
        with_from["fromStem"] = json!("gengHeavenly");
        with_from["fromBranch"] = json!("chenEarthly");
        let rearranged = run(&with_from);
        assert_ne!(plain, rearranged, "重排后的运限应与原盘不同");

        let native_chart = crate::by_solar(
            "1990-4-23",
            2,
            Gender::Male,
            false,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap()
        .rearranged(
            HeavenlyStem::from_key("gengHeavenly").unwrap(),
            EarthlyBranch::from_key("chenEarthly").unwrap(),
        )
        .unwrap();
        let native = crate::get_horoscope(&native_chart, "2024-6-1", 0, Language::ZhCN).unwrap();
        assert_eq!(
            rearranged,
            serde_json::to_value(native.to_dto(Language::ZhCN)).unwrap()
        );

        for kind in ["astrolabeToText", "horoscopeToText"] {
            let mut q = json!({
                "kind": kind,
                "solarDate": "1990-4-23",
                "timeIndex": 2,
                "gender": "male",
                "language": "zh-CN",
                "targetDate": "2024-6-1",
                "targetTimeIndex": 0,
            });
            let plain = query(&serde_json::from_value::<QueryInput>(q.clone()).unwrap()).unwrap();
            q["fromStem"] = json!("gengHeavenly");
            q["fromBranch"] = json!("chenEarthly");
            let rearranged = query(&serde_json::from_value::<QueryInput>(q).unwrap()).unwrap();
            assert_ne!(plain, rearranged, "{kind}: 重排后的文本应与原盘不同");
        }
    }

    /// 单宫/三方四正/格局的 to_text kind：宫位寻址两种写法一致，非法寻址报错
    #[test]
    fn palace_and_pattern_text_kinds() {
        let base = json!({
            "solarDate": "2000-8-16",
            "timeIndex": 2,
            "gender": "female",
            "language": "zh-CN",
        });
        let run = |extra: Value| {
            let mut q = base.clone();
            for (k, v) in extra.as_object().unwrap() {
                q[k] = v.clone();
            }
            query(&serde_json::from_value::<QueryInput>(q).unwrap())
        };

        // 命宫按 key 寻址与按索引寻址取到同一段文本
        let by_key = run(json!({"kind": "palaceToText", "palaceKey": "soulPalace"})).unwrap();
        let text = by_key.as_str().unwrap();
        assert!(text.starts_with("--- 命宫"));
        let soul_index = text_soul_index();
        let by_index = run(json!({"kind": "palaceToText", "palaceIndex": soul_index})).unwrap();
        assert_eq!(by_key, by_index);

        // 身宫寻址
        let body = run(json!({"kind": "palaceToText", "palaceKey": "bodyPalace"})).unwrap();
        assert!(body.as_str().unwrap().contains("[身宫]"));

        // 三方四正
        let sp =
            run(json!({"kind": "surroundedPalacesToText", "palaceKey": "soulPalace"})).unwrap();
        assert!(sp.as_str().unwrap().starts_with("本宫: 命宫"));

        // 本命与运限视角的格局文本
        let hits = run(json!({"kind": "patternsToText"})).unwrap();
        assert!(hits.is_string());
        let h = run(json!({
            "kind": "horoscopePatternsToText",
            "scope": "decadal",
            "targetDate": "2024-10-1",
            "targetTimeIndex": 0,
        }))
        .unwrap();
        assert!(h.is_string());

        // 非法宫位 key 与越界索引都报 invalid_argument
        assert!(run(json!({"kind": "palaceToText", "palaceKey": "nope"})).is_err());
        assert!(run(json!({"kind": "palaceToText", "palaceIndex": 12})).is_err());
        // 寻址缺省不许静默取默认宫
        assert!(run(json!({"kind": "palaceToText"})).is_err());
        assert!(run(json!({"kind": "surroundedPalacesToText"})).is_err());
    }

    /// 测试盘（2000-8-16 女，寅时）命宫的宫位索引
    fn text_soul_index() -> usize {
        let chart = crate::by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            false,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        chart
            .palaces
            .iter()
            .position(|p| p.name == Palace::Soul)
            .unwrap()
    }
}

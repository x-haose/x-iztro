//! 反推：由八字四柱或星盘特征反查候选生辰。
//!
//! 两个入口：[`solar_dates_by_bazi`] 收四柱干支，返回按 `Config` 分界口径能得出该四柱的
//! 全部公历生辰；[`reverse_chart`] 收一组星盘特征（命宫身宫地支、五行局、星耀落宫、生年四化），
//! 返回能排出满足全部特征星盘的候选生辰。
//!
//! 两者都是「剪枝枚举 + 正排终验」：剪枝只用便宜的查表快判缩小范围（保守，宁多留不错杀），
//! 每个幸存候选再用与正排完全相同的函数（[`four_pillars`] / [`by_solar`]）验证，
//! 因此结果与正向排盘零分歧。星盘布局与性别无关（性别只影响大限行进方向），
//! 反推的目标是生辰，故不收性别。

use std::sync::OnceLock;

use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::lunar_month::LunarMonthRefHelper;
use lunar_rust::lunar_year::{self, LunarYearRefHelper};
use lunar_rust::solar::SolarRefHelper;
use lunar_rust::{lunar, solar};
use serde::{Deserialize, Serialize};

use crate::astro::builder::{by_solar, four_pillars};
use crate::astro::palace::{get_five_elements_class, get_soul_and_body};
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::models::astrolabe::Astrolabe;
use crate::star::location::{
    get_chang_qu_index, get_huo_ling_index, get_kong_jie_index, get_kui_yue_index,
    get_lu_yang_tuo_ma_index, get_luan_xi_index, get_start_index, get_zuo_you_index,
};
use crate::utils::{fix_index, get_mutagens_by_heavenly_stem};

/// 支持反推的公历年范围（与排盘一致）。
const SUPPORTED_YEARS: std::ops::RangeInclusive<i64> = 1583..=9999;

/// [`ReverseCriteria::limit`] 为 0 时采用的候选数上限。
pub const DEFAULT_REVERSE_LIMIT: usize = 512;

/// 一个候选生辰：公历日期与时辰索引，可直接交给排盘入口。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BirthCandidate {
    /// 公历日期，`YYYY-M-D`
    pub solar_date: String,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
}

// ============================================================
// 八字反推
// ============================================================

/// 由八字四柱反查公历生辰。
///
/// 四柱按 `config` 的分界口径解释（`year_divide` 年柱、`horoscope_divide` 月柱、
/// `day_divide` 晚子归属），与排盘输出的 `raw_dates.chinese_date` 同一套语义，
/// 因此任何盘的四柱反查结果必包含该盘的生辰。八字精确到时辰，一组四柱在范围内
/// 通常每约 60 年出现一次；子时因早晚子之分可能给出两个候选。
///
/// `year_range` 为公历年闭区间（含两端），须落在 1583-9999 内。
///
/// # Errors
/// 干支阴阳不配（如甲丑）、年份范围非法时返回 [`IztroError::InvalidArgument`]。
pub fn solar_dates_by_bazi(
    yearly: (HeavenlyStem, EarthlyBranch),
    monthly: (HeavenlyStem, EarthlyBranch),
    daily: (HeavenlyStem, EarthlyBranch),
    hourly: (HeavenlyStem, EarthlyBranch),
    year_range: (i64, i64),
    config: &Config,
) -> Result<Vec<BirthCandidate>, IztroError> {
    for (name, (stem, branch)) in [
        ("yearly", yearly),
        ("monthly", monthly),
        ("daily", daily),
        ("hourly", hourly),
    ] {
        if stem.index() % 2 != branch.index() % 2 {
            return Err(IztroError::InvalidArgument(format!(
                "invalid {name} pillar: stem and branch must have the same polarity"
            )));
        }
    }
    validate_year_range(year_range)?;
    let target = [yearly, monthly, daily, hourly];
    let target_day60 = sexagenary_index(daily);
    let mut out = Vec::new();

    for year in year_range.0..=year_range.1 {
        // 年柱粗筛：公历年 year 内的日子按任一分界口径，年柱只可能是「该年」或「上一年」的干支
        // （年初分界点之前归上一年）。不匹配即整年跳过。
        if sexagenary_index(year_pillar(year)) != sexagenary_index(yearly)
            && sexagenary_index(year_pillar(year - 1)) != sexagenary_index(yearly)
        {
            continue;
        }
        for month in 1..=12u32 {
            for day in 1..=days_in_month(year, month) {
                // 日柱粗筛（纯算术 60 周期）：匹配当日（普通时辰与早子）或次日（晚子归次日）才细验。
                let day60 = day_sexagenary_index(year, month, day);
                let hour_branch = hourly.1;
                let mut candidates: [Option<u8>; 2] = [None, None];
                if hour_branch == EarthlyBranch::Zi {
                    if day60 == target_day60 {
                        candidates[0] = Some(0);
                    }
                    if (day60 + 1) % 60 == target_day60 {
                        candidates[1] = Some(12);
                    }
                } else if day60 == target_day60 {
                    candidates[0] = Some(hour_branch.index() as u8);
                }
                for time_index in candidates.into_iter().flatten() {
                    let date = format!("{year}-{month}-{day}");
                    // 终验：与正排同一函数算完整四柱，全部相等才是解
                    if four_pillars(&date, time_index, config).is_ok_and(|p| p == target) {
                        out.push(BirthCandidate {
                            solar_date: date,
                            time_index,
                        });
                    }
                }
            }
        }
    }
    Ok(out)
}

/// 公历年 `year` 当年的年柱干支（大部分日子所属；分界点前的日子属上一年）。
fn year_pillar(year: i64) -> (HeavenlyStem, EarthlyBranch) {
    (
        HeavenlyStem::from_index(fix_index((year - 4) as i32, 10)),
        EarthlyBranch::from_index(fix_index((year - 4) as i32, 12)),
    )
}

/// 干支的六十甲子序（甲子=0）。
fn sexagenary_index((stem, branch): (HeavenlyStem, EarthlyBranch)) -> usize {
    // 同奇偶才构成合法组合；序号是同余方程 x≡s (mod 10), x≡b (mod 12) 的唯一解
    fix_index(6 * stem.index() as i32 - 5 * branch.index() as i32, 60)
}

/// 公历日期的日柱六十甲子序：以 lunar_rust 给出的锚点日校准后纯算术推算。
fn day_sexagenary_index(year: i64, month: u32, day: u32) -> usize {
    static ANCHOR: OnceLock<i64> = OnceLock::new();
    let anchor = *ANCHOR.get_or_init(|| {
        // 锚点：2000-01-01 正午的日干支（正午不受晚子归属影响）
        let l = lunar::from_solar(&solar::from_ymdhms(2000, 1, 1, 12, 0, 0));
        let stem = crate::astro::builder::parse_heavenly_stem(&l.get_day_gan()).expect("锚点日干");
        let branch =
            crate::astro::builder::parse_earthly_branch(&l.get_day_zhi()).expect("锚点日支");
        sexagenary_index((stem, branch)) as i64 - jdn(2000, 1, 1)
    });
    fix_index((anchor + jdn(year, month, day)) as i32, 60)
}

/// 格里历儒略日数（Fliegel-Van Flandern）。
fn jdn(year: i64, month: u32, day: u32) -> i64 {
    let (y, m, d) = (year, month as i64, day as i64);
    let a = (14 - m) / 12;
    let y2 = y + 4800 - a;
    let m2 = m + 12 * a - 3;
    d + (153 * m2 + 2) / 5 + 365 * y2 + y2 / 4 - y2 / 100 + y2 / 400 - 32045
}

/// 格里历每月天数。
fn days_in_month(year: i64, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        _ => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
    }
}

fn validate_year_range((start, end): (i64, i64)) -> Result<(), IztroError> {
    if start > end || !SUPPORTED_YEARS.contains(&start) || !SUPPORTED_YEARS.contains(&end) {
        return Err(IztroError::InvalidArgument(format!(
            "invalid year range {start}-{end}: expected {}-{} with start <= end",
            SUPPORTED_YEARS.start(),
            SUPPORTED_YEARS.end()
        )));
    }
    Ok(())
}

// ============================================================
// 星盘特征反推
// ============================================================

/// 一颗星与其落宫地支：星盘特征反推的原子条件。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarPosition {
    /// 星耀（须为本命盘星耀，流耀不接受）
    pub star: StarKey,
    /// 落宫地支
    pub branch: EarthlyBranch,
}

/// 星盘特征反推的条件集。全部字段可选，但至少要给一个条件；
/// 条件越具体（尤其命宫地支、生年四化、主星落宫），反推越快。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReverseCriteria {
    /// 命宫地支
    pub soul_branch: Option<EarthlyBranch>,
    /// 身宫地支
    pub body_branch: Option<EarthlyBranch>,
    /// 五行局
    pub five_elements_class: Option<FiveElementsClass>,
    /// 星耀落宫条件，全部须同时满足
    pub stars: Vec<StarPosition>,
    /// 生年四化 [禄, 权, 科, 忌] 各自是哪颗星，可只给其中几个
    pub mutagens: [Option<StarKey>; 4],
    /// 公历年闭区间（含两端），须落在 1583-9999 内
    pub year_range: (i64, i64),
    /// 是否修正闰月（与排盘入参同义）
    pub fix_leap: bool,
    /// 候选数上限：达到即停止搜索并置 [`ReverseResult::truncated`]；0 取 [`DEFAULT_REVERSE_LIMIT`]
    pub limit: usize,
}

impl Default for ReverseCriteria {
    fn default() -> Self {
        ReverseCriteria {
            soul_branch: None,
            body_branch: None,
            five_elements_class: None,
            stars: Vec::new(),
            mutagens: [None; 4],
            year_range: (1900, 2100),
            fix_leap: true,
            limit: 0,
        }
    }
}

/// 星盘特征反推的结果。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReverseResult {
    /// 满足全部条件的候选生辰，按日期升序
    pub candidates: Vec<BirthCandidate>,
    /// 是否因达到候选数上限而提前截断（截断时更晚的解未被搜索）
    pub truncated: bool,
}

/// 由星盘特征反查候选生辰。
///
/// 排盘配置随 `config` 传入并贯穿判定（四化表、派别、分界都按它算），
/// 候选用同一 `config` 排盘必满足全部条件。
///
/// # Errors
/// 条件为空、包含流耀、年份范围非法时返回 [`IztroError::InvalidArgument`]。
pub fn reverse_chart(
    criteria: &ReverseCriteria,
    config: &Config,
) -> Result<ReverseResult, IztroError> {
    validate_criteria(criteria)?;
    let limit = if criteria.limit == 0 {
        DEFAULT_REVERSE_LIMIT
    } else {
        criteria.limit
    };
    let mut out = Vec::new();
    let mut truncated = false;

    'year: for year in criteria.year_range.0..=criteria.year_range.1 {
        // 农历年与公历年同号枚举；候选的安星年干支按分界口径可能取本年或上一年干支，
        // 年层剪枝对两者都放行（保守），归属差异由终验兜住。
        let stems = year_stem_candidates(year, config);
        if !year_prefilter(criteria, &stems, config) {
            continue;
        }
        let leap_month = lunar_year::LunarYear::from_lunar_year(year)
            .get_leap_months()
            .abs();
        for month in 1..=12i64 {
            for is_leap in [false, true] {
                if is_leap && month != leap_month {
                    continue;
                }
                let signed_month = if is_leap { -month } else { month };
                let Some(month_ref) =
                    lunar_year::LunarYear::from_lunar_year(year).get_month(signed_month)
                else {
                    continue;
                };
                let month_day_count = month_ref.get_day_count() as u32;
                for time_index in 0..=12u8 {
                    // 月×时层剪枝：命宫身宫、五行局与月系/时系星。
                    // 闰月与下半月修正可能使安星月 +1，两种月索引都放行（保守）。
                    let month_indices =
                        [fix_index(month as i32 - 1, 12), fix_index(month as i32, 12)];
                    let fecs =
                        month_time_prefilter(criteria, &stems, &month_indices, time_index, config);
                    if fecs.is_empty() {
                        continue;
                    }
                    for day in 1..=month_day_count {
                        if !day_prefilter(criteria, day, time_index, month_day_count, &fecs) {
                            continue;
                        }
                        // 终验：农历转公历后完整排盘，逐条核对全部条件
                        let l = lunar::from_ymd(year, signed_month, day as i64);
                        let s = l.get_solar();
                        let date = format!("{}-{}-{}", s.get_year(), s.get_month(), s.get_day());
                        let Ok(chart) = by_solar(
                            &date,
                            time_index,
                            Gender::Male,
                            criteria.fix_leap,
                            Language::ZhCN,
                            config.clone(),
                        ) else {
                            continue;
                        };
                        if matches_criteria(&chart, criteria, config) {
                            out.push(BirthCandidate {
                                solar_date: date,
                                time_index,
                            });
                            if out.len() >= limit {
                                truncated = true;
                                break 'year;
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(ReverseResult {
        candidates: out,
        truncated,
    })
}

/// 候选星盘是否满足全部条件（终验，与正排零分歧的唯一依据）。
fn matches_criteria(chart: &Astrolabe, criteria: &ReverseCriteria, config: &Config) -> bool {
    if let Some(b) = criteria.soul_branch
        && chart.earthly_branch_of_soul_palace != b
    {
        return false;
    }
    if let Some(b) = criteria.body_branch
        && chart.earthly_branch_of_body_palace != b
    {
        return false;
    }
    if let Some(f) = criteria.five_elements_class
        && chart.five_elements_class != f
    {
        return false;
    }
    let mutagens = get_mutagens_by_heavenly_stem(chart.raw_dates.chinese_date.yearly.0, config);
    for (want, got) in criteria.mutagens.iter().zip(mutagens) {
        if let Some(star) = want
            && *star != got
        {
            return false;
        }
    }
    criteria.stars.iter().all(|p| {
        chart
            .star(p.star)
            .is_some_and(|s| s.palace().earthly_branch == p.branch)
    })
}

/// 该公历/农历年编号下，候选的安星年干支：`Exact` 分界时年初日子可能沿用上一年干支。
fn year_stem_candidates(year: i64, config: &Config) -> Vec<(HeavenlyStem, EarthlyBranch)> {
    match config.year_divide {
        YearDivide::Normal => vec![year_pillar(year)],
        YearDivide::Exact => vec![year_pillar(year), year_pillar(year - 1)],
    }
}

/// 年层剪枝：生年四化与年系星（禄存羊陀天马魁钺、红鸾天喜）在任一候选年干支下可满足才保留。
fn year_prefilter(
    criteria: &ReverseCriteria,
    stems: &[(HeavenlyStem, EarthlyBranch)],
    config: &Config,
) -> bool {
    stems.iter().any(|(stem, branch)| {
        let mutagens = get_mutagens_by_heavenly_stem(*stem, config);
        let mutagen_ok = criteria
            .mutagens
            .iter()
            .zip(mutagens)
            .all(|(want, got)| want.is_none_or(|w| w == got));
        mutagen_ok
            && criteria.stars.iter().all(|p| {
                let want = branch_to_palace(p.branch);
                match p.star {
                    StarKey::LucunMin => get_lu_yang_tuo_ma_index(*stem, *branch).lu == want,
                    StarKey::QingyangMin => get_lu_yang_tuo_ma_index(*stem, *branch).yang == want,
                    StarKey::TuoluoMin => get_lu_yang_tuo_ma_index(*stem, *branch).tuo == want,
                    StarKey::TianmaMin => get_lu_yang_tuo_ma_index(*stem, *branch).ma == want,
                    StarKey::TiankuiMin => get_kui_yue_index(*stem).kui == want,
                    StarKey::TianyueMin => get_kui_yue_index(*stem).yue == want,
                    StarKey::Hongluan => get_luan_xi_index(*branch).hongluan == want,
                    StarKey::Tianxi => get_luan_xi_index(*branch).tianxi == want,
                    _ => true,
                }
            })
    })
}

/// 月×时层剪枝：命宫身宫地支、五行局与月系（左辅右弼）、时系（文昌文曲地空地劫）、
/// 支+时系（火星铃星）条件。返回可行组合对应的五行局取值集合（供日层用），空集即整枝剪掉。
fn month_time_prefilter(
    criteria: &ReverseCriteria,
    stems: &[(HeavenlyStem, EarthlyBranch)],
    month_indices: &[usize; 2],
    time_index: u8,
    config: &Config,
) -> Vec<u32> {
    let _ = config;
    let mut fecs: Vec<u32> = Vec::new();
    for (stem, branch) in stems {
        for &mi in month_indices {
            let sb = get_soul_and_body(mi, time_index, *stem);
            if let Some(b) = criteria.soul_branch
                && sb.earthly_branch_of_soul != b
            {
                continue;
            }
            if let Some(b) = criteria.body_branch
                && EarthlyBranch::from_index(fix_index(sb.body_index as i32 + 2, 12)) != b
            {
                continue;
            }
            let fec = get_five_elements_class(sb.heavenly_stem_of_soul, sb.earthly_branch_of_soul);
            if let Some(f) = criteria.five_elements_class
                && fec != f
            {
                continue;
            }
            let stars_ok = criteria.stars.iter().all(|p| {
                let want = branch_to_palace(p.branch);
                match p.star {
                    StarKey::ZuofuMin => get_zuo_you_index(mi as u32 + 1).zuo == want,
                    StarKey::YoubiMin => get_zuo_you_index(mi as u32 + 1).you == want,
                    StarKey::WenchangMin => get_chang_qu_index(time_index).chang == want,
                    StarKey::WenquMin => get_chang_qu_index(time_index).qu == want,
                    StarKey::DikongMin => get_kong_jie_index(time_index).kong == want,
                    StarKey::DijieMin => get_kong_jie_index(time_index).jie == want,
                    StarKey::HuoxingMin => get_huo_ling_index(*branch, time_index).huo == want,
                    StarKey::LingxingMin => get_huo_ling_index(*branch, time_index).ling == want,
                    _ => true,
                }
            });
            if !stars_ok {
                continue;
            }
            if !fecs.contains(&(fec as u32)) {
                fecs.push(fec as u32);
            }
        }
    }
    fecs
}

/// 主星与其相对紫微（负偏移）或天府（正偏移）的位次。
const MAJOR_OFFSETS: [(StarKey, i32, bool); 14] = [
    (StarKey::ZiweiMaj, 0, false),
    (StarKey::TianjiMaj, 1, false),
    (StarKey::TaiyangMaj, 3, false),
    (StarKey::WuquMaj, 4, false),
    (StarKey::TiantongMaj, 5, false),
    (StarKey::LianzhenMaj, 8, false),
    (StarKey::TianfuMaj, 0, true),
    (StarKey::TaiyinMaj, 1, true),
    (StarKey::TanlangMaj, 2, true),
    (StarKey::JumenMaj, 3, true),
    (StarKey::TianxiangMaj, 4, true),
    (StarKey::TianliangMaj, 5, true),
    (StarKey::QishaMaj, 6, true),
    (StarKey::PojunMaj, 10, true),
];

/// 日层剪枝：主星落宫条件在任一可行五行局下成立才保留该日。
fn day_prefilter(
    criteria: &ReverseCriteria,
    day: u32,
    time_index: u8,
    month_day_count: u32,
    fecs: &[u32],
) -> bool {
    let majors: Vec<&StarPosition> = criteria
        .stars
        .iter()
        .filter(|p| MAJOR_OFFSETS.iter().any(|(k, _, _)| *k == p.star))
        .collect();
    if majors.is_empty() {
        return true;
    }
    fecs.iter().any(|fec| {
        let si = get_start_index(day, time_index, month_day_count, *fec);
        majors.iter().all(|p| {
            let (_, offset, from_tianfu) = MAJOR_OFFSETS
                .iter()
                .find(|(k, _, _)| *k == p.star)
                .expect("majors 已按 MAJOR_OFFSETS 过滤");
            let idx = if *from_tianfu {
                fix_index(si.tianfu as i32 + offset, 12)
            } else {
                fix_index(si.ziwei as i32 - offset, 12)
            };
            idx == branch_to_palace(p.branch)
        })
    })
}

/// 地支转宫位索引（寅宫为 0）。
fn branch_to_palace(branch: EarthlyBranch) -> usize {
    fix_index(branch.index() as i32 - 2, 12)
}

fn validate_criteria(criteria: &ReverseCriteria) -> Result<(), IztroError> {
    validate_year_range(criteria.year_range)?;
    let empty = criteria.soul_branch.is_none()
        && criteria.body_branch.is_none()
        && criteria.five_elements_class.is_none()
        && criteria.stars.is_empty()
        && criteria.mutagens.iter().all(Option::is_none);
    if empty {
        return Err(IztroError::InvalidArgument(
            "reverse criteria must contain at least one condition".to_string(),
        ));
    }
    for p in &criteria.stars {
        if is_flow_star(p.star) {
            return Err(IztroError::InvalidArgument(format!(
                "star '{}' is a horoscope-scope star and never appears on the natal chart",
                p.star.as_key()
            )));
        }
    }
    Ok(())
}

/// 是否运限流曜（大限运曜、流年流曜、小限流曜）：它们不出现在本命盘上。
fn is_flow_star(key: StarKey) -> bool {
    use StarKey::*;
    matches!(
        key,
        Yunkui
            | Yunyue
            | Yunchang
            | Yunqu
            | Yunluan
            | Yunxi
            | Yunlu
            | Yunyang
            | Yuntuo
            | Yunma
            | Liukui
            | Liuyue
            | Liuchang
            | Liuqu
            | Liuluan
            | Liuxi
            | Liulu
            | Liuyang
            | Liutuo
            | Liuma
            | Shikui
            | Shiyue
            | Shichang
            | Shiqu
            | Shiluan
            | Shixi
            | Shilu
            | Shiyang
            | Shituo
            | Shima
    )
}

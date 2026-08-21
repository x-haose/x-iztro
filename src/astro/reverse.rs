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
use lunar_rust::{lunar, solar};
use serde::{Deserialize, Serialize};

use crate::astro::builder::{by_solar, four_pillars};
use crate::astro::lunar_table;
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
                // 日柱粗筛（纯算术 60 周期）。子时的日柱归属随 `day_divide`：
                // 晚子归次日（Forward）时 t=12 的日柱取次日干支；晚子归当天（Current）时
                // 正排把 t>=12 归一为早子，(D,0) 与 (D,12) 四柱完全相同，日柱匹配当日即
                // 同时给出两个时辰。非子时只匹配当日。
                let day60 = day_sexagenary_index(year, month, day);
                let hour_branch = hourly.1;
                let mut candidates: [Option<u8>; 2] = [None, None];
                if hour_branch == EarthlyBranch::Zi {
                    match config.day_divide {
                        DayDivide::Current => {
                            if day60 == target_day60 {
                                candidates = [Some(0), Some(12)];
                            }
                        }
                        DayDivide::Forward => {
                            if day60 == target_day60 {
                                candidates[0] = Some(0);
                            }
                            if (day60 + 1) % 60 == target_day60 {
                                candidates[1] = Some(12);
                            }
                        }
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
    /// 候选公历日期所属年份的闭区间（含两端），须落在 1583-9999 内
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
    /// 满足全部条件的候选生辰，按枚举序排列：农历年升序，年内依 月→时辰→日；
    /// 同一年内不保证公历日期升序
    pub candidates: Vec<BirthCandidate>,
    /// 是否因达到候选数上限而提前截断；截断时枚举序更靠后的解未被搜索，
    /// 其中可能包含公历日期更早的解
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

    // 解析域：条件先按安星几何锁定各维度；主星条件互斥则整个查询无解
    let domains = Domains::resolve(criteria);
    if matches!(domains.ziwei_palace, Some(Err(()))) {
        return Ok(ReverseResult {
            candidates: out,
            truncated,
        });
    }

    // 公历闭区间 [start, end] 的日子分布在农历年 [start-1, end]：公历年 Y 的年初
    // （元旦到春节前）属农历年 Y-1，而农历年 end+1 的首日（春节）恒落在公历 end+1 年内。
    // 按农历年枚举并在首端扩一年即覆盖全区间，候选再逐个按公历年份过滤，结果严格落在区间内。
    let (start_year, end_year) = criteria.year_range;
    'year: for year in (start_year - 1)..=end_year {
        // 候选的安星年干支按分界口径可能取相邻年份干支，
        // 年层剪枝对全部候选干支放行（保守），归属差异由终验兜住。
        // 年层条件只依赖 (年干, 年支)：不满足的候选干支组在此剔除，
        // 不再流入按「组 × 月 × 时」相乘的内层循环。逐组精确复用
        // year_prefilter 判定——剪枝几何与年层校验只此一份，不另抄副本
        let stems: Vec<(HeavenlyStem, EarthlyBranch)> = year_stem_candidates(year, config)
            .into_iter()
            .filter(|pair| year_prefilter(criteria, std::slice::from_ref(pair), config))
            .collect();
        if stems.is_empty() {
            continue;
        }
        // 闰月只查一次修正视图：非闰月年直接跳过全部闰月试探
        let leap_month = lunar_table::leap_month(year);
        for month in 1..=12i64 {
            if let Some(months) = &domains.months
                && !months.contains(&month)
            {
                continue;
            }
            for is_leap in [false, true] {
                if is_leap && leap_month != Some(month) {
                    continue;
                }
                let signed_month = if is_leap { -month } else { month };
                // 月天数走 lunar_table 修正视图（lunar_rust 的 1602 闰二月月界缺陷在
                // 那里归位），枚举的农历日标签与正排上下文完全同源
                let Some(month_day_count) = lunar_table::month_day_count(year, signed_month) else {
                    continue;
                };
                let month_day_count = month_day_count as u32;
                for time_index in 0..=12u8 {
                    if let Some(times) = &domains.times
                        && !times.contains(&time_index)
                    {
                        continue;
                    }
                    // 月×时层剪枝：命宫身宫、五行局与月系/时系星。
                    // 闰月与下半月修正可能使安星月 +1，两种月索引都放行（保守）。
                    let month_indices =
                        [fix_index(month as i32 - 1, 12), fix_index(month as i32, 12)];
                    let fecs = month_time_prefilter(criteria, &stems, &month_indices, time_index);
                    if fecs.is_empty() {
                        continue;
                    }
                    // 紫微起宫与日系杂耀区分早晚子，日层剪枝须收与正排一致的生效时辰
                    let eff_t = effective_time_index(config.day_divide, time_index);
                    for day in 1..=month_day_count {
                        if !day_prefilter(&domains, day, eff_t, month_day_count, &fecs)
                            || !daily_star_prefilter(criteria, month as usize - 1, day, eff_t)
                        {
                            continue;
                        }
                        // 日已在 1..=month_day_count 内，满足 solar_date_of 的调用前提
                        let date = lunar_table::solar_date_of(year, signed_month, day as i64);
                        let solar_year: i64 = date
                            .split('-')
                            .next()
                            .and_then(|y| y.parse().ok())
                            .expect("solar_date_of 返回自产 Y-M-D 格式，首段恒为年份数字");
                        if solar_year < start_year || solar_year > end_year {
                            continue;
                        }
                        // 终验：农历转公历后完整排盘，逐条核对全部条件
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

/// 参与日敏感安星的生效时辰，与正排上下文派生的 `effective_time_index` 同一语义：
/// `Current` 分界下晚子（>=12）按早子（0）参与推算。紫微起宫（`get_start_index`）与
/// 日系杂耀（`get_daily_star_index`）区分 t=12 与 t=0，日层剪枝必须收归一后的值；
/// 其余安星函数对时辰做模 12 处理、早晚子同位，收原始时辰即可。
fn effective_time_index(day_divide: DayDivide, time_index: u8) -> u8 {
    if day_divide == DayDivide::Current && time_index >= 12 {
        0
    } else {
        time_index
    }
}

/// 农历年编号 `year` 下，候选的安星年干支。
///
/// `Exact`（立春分界）时同一农历年横跨两个立春：正月里立春前的日子沿用上一年干支；
/// 春节晚于立春的年份，腊月里下一个立春之后的日子已属下一年干支。三个候选都放行
/// （保守，归属差异由终验兜住）。`Normal`（正月初一分界）与农历年一致，只有本年。
fn year_stem_candidates(year: i64, config: &Config) -> Vec<(HeavenlyStem, EarthlyBranch)> {
    match config.year_divide {
        YearDivide::Normal => vec![year_pillar(year)],
        YearDivide::Exact => vec![
            year_pillar(year),
            year_pillar(year - 1),
            year_pillar(year + 1),
        ],
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
) -> Vec<u32> {
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

/// 由星耀落宫条件解析出的枚举域：每个维度先按安星几何锁定到最小集合，
/// 枚举骨架只在锁剩的域上跑。域为 `None` 表示该维度无条件约束（全域）。
/// 任何反查都是「对小域试正向安星函数」——与排盘共用同一实现，不会与正排脱节。
///
/// 干系/支系星（禄存羊陀魁钺、天马鸾喜）不在此锁域：年干支候选每年至多 3 组，
/// `year_prefilter` 对每组逐星精确复验，年层再做存在性预筛纯属同一几何的手抄副本。
struct Domains {
    /// 主星条件归一化出的紫微落宫；`Some(Err)` 表示多颗主星条件互斥，整个查询无解
    ziwei_palace: Option<Result<usize, ()>>,
    /// 月系星（左辅/右弼）锁定的农历月集合（1-12；含闰月修正的 ±1 保守扩张）
    months: Option<Vec<i64>>,
    /// 时系星（文昌/文曲/地空/地劫）锁定的时辰集合
    times: Option<Vec<u8>>,
}

impl Domains {
    /// 从条件集解析各维度域。
    fn resolve(criteria: &ReverseCriteria) -> Domains {
        let mut ziwei: Option<Result<usize, ()>> = None;
        for p in &criteria.stars {
            if let Some((offset, from_tianfu)) = major_offset(p.star) {
                // 主星相对紫微/天府的偏移固定，任何主星落宫都唯一反解出紫微落宫
                let palace = branch_to_palace(p.branch) as i32;
                let z = if from_tianfu {
                    fix_index(12 - fix_index(palace - offset, 12) as i32, 12)
                } else {
                    fix_index(palace + offset, 12)
                };
                ziwei = Some(match ziwei {
                    Some(Ok(prev)) if prev != z => Err(()),
                    Some(Err(())) => Err(()),
                    _ => Ok(z),
                });
            }
        }
        let mut months: Option<Vec<i64>> = None;
        let mut times: Option<Vec<u8>> = None;
        for p in &criteria.stars {
            let want = branch_to_palace(p.branch);
            match p.star {
                StarKey::ZuofuMin | StarKey::YoubiMin => {
                    let base: Vec<i64> = (1..=12i64)
                        .filter(|m| {
                            let zy = get_zuo_you_index(*m as u32);
                            let idx = if p.star == StarKey::ZuofuMin {
                                zy.zuo
                            } else {
                                zy.you
                            };
                            idx == want
                        })
                        .collect();
                    // 闰月下半月按下月安星：真实农历月可能比安星月小 1，保守并入
                    let mut set: Vec<i64> = base
                        .iter()
                        .flat_map(|m| [*m, fix_index(*m as i32 - 2, 12) as i64 + 1])
                        .collect();
                    set.sort_unstable();
                    set.dedup();
                    months = Some(match months.take() {
                        Some(prev) => prev.into_iter().filter(|m| set.contains(m)).collect(),
                        None => set,
                    });
                }
                StarKey::WenchangMin
                | StarKey::WenquMin
                | StarKey::DikongMin
                | StarKey::DijieMin => {
                    let set: Vec<u8> = (0..=12u8)
                        .filter(|t| {
                            let idx = match p.star {
                                StarKey::WenchangMin => get_chang_qu_index(*t).chang,
                                StarKey::WenquMin => get_chang_qu_index(*t).qu,
                                StarKey::DikongMin => get_kong_jie_index(*t).kong,
                                _ => get_kong_jie_index(*t).jie,
                            };
                            idx == want
                        })
                        .collect();
                    times = Some(match times.take() {
                        Some(prev) => prev.into_iter().filter(|t| set.contains(t)).collect(),
                        None => set,
                    });
                }
                _ => {}
            }
        }
        Domains {
            ziwei_palace: ziwei,
            months,
            times,
        }
    }
}

/// 主星相对紫微/天府的位次：`(位次, 是否天府系)`；非主星返回 `None`。
///
/// 位次直接取安星表（`star::major` 的 [`ZIWEI_GROUP`]/[`TIANFU_GROUP`]），
/// 不另抄一份——手抄副本与安星表漂移时，剪枝会静默错杀真解。
fn major_offset(key: StarKey) -> Option<(i32, bool)> {
    use crate::star::major::{TIANFU_GROUP, ZIWEI_GROUP};
    ZIWEI_GROUP
        .iter()
        .find(|(_, k)| *k == key)
        .map(|(o, _)| (*o as i32, false))
        .or_else(|| {
            TIANFU_GROUP
                .iter()
                .find(|(_, k)| *k == key)
                .map(|(o, _)| (*o as i32, true))
        })
}

/// 日层剪枝：主星条件已归一化为紫微落宫，任一可行五行局下紫微落到该宫才保留该日。
/// `effective_ti` 是 [`effective_time_index`] 归一后的生效时辰（`get_start_index` 区分早晚子）。
fn day_prefilter(
    domains: &Domains,
    day: u32,
    effective_ti: u8,
    month_day_count: u32,
    fecs: &[u32],
) -> bool {
    let Some(Ok(ziwei)) = domains.ziwei_palace else {
        return true;
    };
    fecs.iter()
        .any(|fec| get_start_index(day, effective_ti, month_day_count, *fec).ziwei == ziwei)
}

/// 日层剪枝（日系杂耀）：三台/八座随左辅右弼逐日行、恩光/天贵随文昌文曲逐日行，
/// 条件涉及这四颗时按（月, 日, 时）现算落宫比对。安星月与真实农历月可能差 1（闰月修正），
/// 两种月索引任一成立即放行（保守）。
/// `effective_ti` 是 [`effective_time_index`] 归一后的生效时辰（`get_daily_star_index` 区分早晚子）。
fn daily_star_prefilter(
    criteria: &ReverseCriteria,
    month_index: usize,
    day: u32,
    effective_ti: u8,
) -> bool {
    let involved = criteria.stars.iter().any(|p| {
        matches!(
            p.star,
            StarKey::Santai | StarKey::Bazuo | StarKey::Engguang | StarKey::Tiangui
        )
    });
    if !involved {
        return true;
    }
    [month_index, month_index + 1].iter().any(|mi| {
        let zy = get_zuo_you_index(fix_index(*mi as i32, 12) as u32 + 1);
        let cq = get_chang_qu_index(effective_ti);
        let daily = crate::star::location::get_daily_star_index(
            day,
            effective_ti,
            zy.zuo,
            zy.you,
            cq.chang,
            cq.qu,
        );
        criteria.stars.iter().all(|p| {
            let want = branch_to_palace(p.branch);
            match p.star {
                StarKey::Santai => daily.santai == want,
                StarKey::Bazuo => daily.bazuo == want,
                StarKey::Engguang => daily.enguang == want,
                StarKey::Tiangui => daily.tiangui == want,
                _ => true,
            }
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
        if is_twelve_gods_star(p.star) {
            return Err(IztroError::InvalidArgument(format!(
                "star '{}' belongs to a per-palace twelve-gods cycle (changsheng12/boshi12/jiangqian12/suiqian12) and cannot be used as a reverse criterion",
                p.star.as_key()
            )));
        }
    }
    Ok(())
}

/// 是否只作十二神出现的标识（长生12神、博士12神、岁前12神、将前12神中
/// 不与宫内杂耀共用 key 的 41 个）。
/// 十二神以每宫单值字段（`changsheng12` 等）挂在宫位上，不进入宫内星耀列表，
/// [`Astrolabe::star`] 查不到，作为落宫条件恒不可满足，故在校验层拒绝。
/// 五个共用 key 不在此列：咸池/华盖/天德（每盘安放）与龙德/大耗（中州派盘
/// 安放）同时是宫内杂耀，[`Astrolabe::star`] 可查到，按杂耀落宫正常终验。
fn is_twelve_gods_star(key: StarKey) -> bool {
    use StarKey::*;
    matches!(
        key,
        // 长生12神
        Changsheng | Muyu | Guandai | Linguan | Diwang | Shuai | Bing | Si | Mu | Jue | Tai | Yang
            // 博士12神（含与岁前12共用的小耗/病符/岁破；大耗兼宫内杂耀，不拒）
            | Boshi | Lishi | Qinglong | Xiaohao | Jiangjun | Zhoushu | Faylian | Xishen
            | Bingfu | Suipo | Fubing | Guanfu
            // 岁前12神（龙德/天德兼宫内杂耀，不拒）
            | Suijian | Huiqi | Sangmen | Guansuo | Gwanfu | Baihu | Diaoke
            // 将前12神（华盖/咸池兼宫内杂耀，不拒）
            | Jiangxing | Panan | Suiyi | Xiishen | Jiesha | Zhaisha | Tiansha
            | Zhibei | Yuesha | Wangshen
    )
}

/// 是否运限流曜：五个层级（运/流/月/日/时）共 50 颗，一律不出现在本命盘上。
///
/// 判定复用全量流曜对照表 [`crate::astro::horoscope::natal_counterpart_of_flow_star`]，
/// 不另抄一份清单——手抄清单漏层级时，含该流曜的条件会通过校验并静默得到空结果。
fn is_flow_star(key: StarKey) -> bool {
    crate::astro::horoscope::natal_counterpart_of_flow_star(key).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn year_stem_candidates_by_divide() {
        let normal = Config::default();
        assert_eq!(year_stem_candidates(2000, &normal), vec![year_pillar(2000)]);
        let exact = Config {
            year_divide: YearDivide::Exact,
            ..Config::default()
        };
        // 立春分界下同一农历年横跨两个立春：本年、上一年（正月头）、下一年（腊月尾）
        assert_eq!(
            year_stem_candidates(2014, &exact),
            vec![year_pillar(2014), year_pillar(2013), year_pillar(2015)]
        );
    }

    #[test]
    fn effective_time_index_normalizes_late_zi_only_under_current() {
        assert_eq!(effective_time_index(DayDivide::Current, 12), 0);
        assert_eq!(effective_time_index(DayDivide::Current, 0), 0);
        assert_eq!(effective_time_index(DayDivide::Current, 6), 6);
        assert_eq!(effective_time_index(DayDivide::Forward, 12), 12);
        assert_eq!(effective_time_index(DayDivide::Forward, 6), 6);
    }
}

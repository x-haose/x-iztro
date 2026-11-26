//! 排盘主流程
//!
//! 提供 `by_solar` 和 `by_lunar` 两个入口函数，从阳历或阴历日期生成完整的紫微斗数星盘。

use crate::astro::context::{self, AstroContext};
use crate::astro::date_input::{self, DateInput};
use crate::astro::palace::{get_decadals_and_ages, get_palace_names};
use crate::calendar::{self, SolarTime};
use crate::data::constants::{SIGNS, TIGER_RULE, TIME_RANGES, ZODIAC};
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::i18n::{translate_lunar_date, translate_sign, translate_time, translate_zodiac};
use crate::models::astrolabe::{Astrolabe, RawChineseDate, RawDates, RawLunarDate};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::star::adjective::get_adjective_stars;
use crate::star::decorative::{get_boshi12, get_changsheng12, get_yearly12};
use crate::star::location::{
    get_chang_qu_index, get_daily_star_index, get_huo_ling_index, get_kong_jie_index,
    get_kui_yue_index, get_lu_yang_tuo_ma_index, get_luan_xi_index, get_monthly_star_index,
    get_start_index, get_tianma_index, get_timely_star_index, get_yearly_star_index,
    get_zuo_you_index,
};
use crate::star::major::get_major_stars;
use crate::star::minor::get_minor_stars;
use crate::utils::{fix_index, translate_chinese_date};

// ============================================================
// 辅助函数
// ============================================================

/// 支持的年份范围（公历与农历同），与 iztro 一致。1582-10-15 以前的公历按儒略历。
pub(crate) const SUPPORTED_YEARS: std::ops::RangeInclusive<i64> = 1..=9999;

/// 校验时辰索引在 0-12（0=早子时，12=晚子时）。
pub(crate) fn validate_time_index(time_index: u8) -> Result<(), IztroError> {
    if time_index > 12 {
        return Err(IztroError::InvalidTimeIndex(time_index));
    }
    Ok(())
}

/// 解析并校验公历日期串（文法见 [`date_input`]），返回年月日与时间部分的小时。
/// 拒绝文法不符、不存在的日期（含改历空缺 1582-10-05 至 10-14）与超出 [`SUPPORTED_YEARS`] 的年份。
pub(crate) fn parse_solar_input(solar_date: &str) -> Result<DateInput, IztroError> {
    let err = |detail: &str| {
        IztroError::InvalidDate(format!("invalid solar date '{solar_date}': {detail}"))
    };
    let input = date_input::parse(solar_date).map_err(err)?;
    let DateInput {
        year, month, day, ..
    } = input;
    if !SUPPORTED_YEARS.contains(&year) {
        return Err(err(&format!(
            "year must be within {}-{}",
            SUPPORTED_YEARS.start(),
            SUPPORTED_YEARS.end()
        )));
    }
    if !(1..=12).contains(&month) {
        return Err(err("month must be within 1-12"));
    }
    if day < 1 || day > calendar::days_in_month(year, month) {
        return Err(err("day is out of range for that month"));
    }
    if calendar::is_reform_gap(year, month, day) {
        return Err(err(
            "1582-10-05 to 1582-10-14 do not exist (the Julian calendar ends on 1582-10-04)",
        ));
    }
    Ok(input)
}

/// 解析并校验公历日期串，只取年月日；时间部分不参与排盘。
pub(crate) fn parse_solar_date(solar_date: &str) -> Result<(i64, i64, i64), IztroError> {
    parse_solar_input(solar_date).map(|d| (d.year, d.month, d.day))
}

/// 解析并校验农历日期串（文法见 [`date_input`]，时间部分不参与排盘），返回 (年, 带符号月, 日)。
/// 月为负值表示闰月；该年该月并非闰月时 `is_leap_month` 不生效，
/// 与 JS iztro 行为一致。日按该月天数校验（大月 30 / 小月 29）。
fn parse_lunar_date(lunar_date: &str, is_leap_month: bool) -> Result<(i64, i64, i64), IztroError> {
    let err = |detail: &str| {
        IztroError::InvalidDate(format!("invalid lunar date '{lunar_date}': {detail}"))
    };
    let DateInput {
        year, month, day, ..
    } = date_input::parse(lunar_date).map_err(err)?;
    if !SUPPORTED_YEARS.contains(&year) {
        return Err(err(&format!(
            "year must be within {}-{}",
            SUPPORTED_YEARS.start(),
            SUPPORTED_YEARS.end()
        )));
    }
    if !(1..=12).contains(&month) {
        return Err(err("month must be within 1-12"));
    }
    let signed_month = if is_leap_month && calendar::leap_month(year) == Some(month) {
        -month
    } else {
        month
    };
    let day_count = calendar::month_day_count(year, signed_month)
        .ok_or_else(|| err("month does not exist in that lunar year"))?;
    if day < 1 || day > day_count {
        return Err(err("day is out of range for that lunar month"));
    }
    Ok((year, signed_month, day))
}

/// 将中文天干字符串转换为枚举
pub fn parse_heavenly_stem(s: &str) -> Option<HeavenlyStem> {
    match s {
        "甲" => Some(HeavenlyStem::Jia),
        "乙" => Some(HeavenlyStem::Yi),
        "丙" => Some(HeavenlyStem::Bing),
        "丁" => Some(HeavenlyStem::Ding),
        "戊" => Some(HeavenlyStem::Wu),
        "己" => Some(HeavenlyStem::Ji),
        "庚" => Some(HeavenlyStem::Geng),
        "辛" => Some(HeavenlyStem::Xin),
        "壬" => Some(HeavenlyStem::Ren),
        "癸" => Some(HeavenlyStem::Gui),
        _ => None,
    }
}

/// 将中文地支字符串转换为枚举
pub fn parse_earthly_branch(s: &str) -> Option<EarthlyBranch> {
    match s {
        "子" => Some(EarthlyBranch::Zi),
        "丑" => Some(EarthlyBranch::Chou),
        "寅" => Some(EarthlyBranch::Yin),
        "卯" => Some(EarthlyBranch::Mao),
        "辰" => Some(EarthlyBranch::Chen),
        "巳" => Some(EarthlyBranch::Si),
        "午" => Some(EarthlyBranch::Wu),
        "未" => Some(EarthlyBranch::Wei),
        "申" => Some(EarthlyBranch::Shen),
        "酉" => Some(EarthlyBranch::You),
        "戌" => Some(EarthlyBranch::Xu),
        "亥" => Some(EarthlyBranch::Hai),
        _ => None,
    }
}

/// 计算修正后的农历月索引
///
/// 对应 TS: fixLunarMonthIndex
/// 返回值为 0-based 月索引（正月=0）
pub fn fix_lunar_month_index(
    lunar_month: u32,
    lunar_day: u32,
    is_leap: bool,
    time_index: u8,
    fix_leap: bool,
) -> usize {
    birth_month_index(
        lunar_month,
        lunar_day,
        is_leap,
        fix_leap,
        LeapDivide::Default,
        time_index == 12,
    )
}

/// 安星用的生月索引（0-based，正月=0），本命盘与推斗君共用。
///
/// 闰月出生时：`LeapDivide::NextMonth` 整月按下月；`LeapDivide::Default` 在 `fix_leap` 打开且日数
/// 过十五时按下月，`keep_leap_month` 为真时例外、仍按本月（本命盘的晚子时即此例外）。
/// 非闰月原样返回。
pub(crate) fn birth_month_index(
    lunar_month: u32,
    lunar_day: u32,
    is_leap: bool,
    fix_leap: bool,
    leap_divide: LeapDivide,
    keep_leap_month: bool,
) -> usize {
    let next_month = is_leap
        && match leap_divide {
            LeapDivide::Default => fix_leap && lunar_day > 15 && !keep_leap_month,
            LeapDivide::NextMonth => true,
        };
    fix_index(lunar_month as i32 - 1 + i32::from(next_month), 12)
}

/// 计算修正后的农历日索引
///
/// 对应 TS: fixLunarDayIndex
/// 晚子时(time_index=12)属于次日，所以不减1
/// 注意：此函数当前未在排盘主流程中直接使用，
/// 因为 get_daily_star_index 内部已自行处理该逻辑。
/// 保留供外部调用使用。
pub fn fix_lunar_day_index(lunar_day: u32, time_index: u8) -> u32 {
    if time_index >= 12 {
        lunar_day
    } else {
        lunar_day.saturating_sub(1)
    }
}

/// 参与推算的时辰索引：`day_divide` 为 `Current` 或 `ForwardCarry` 时晚子（>=12）归零按早子算
/// （`ForwardCarry` 另把日期换成次日，见 [`effective_solar_date`]），其余原样。
/// 入参与展示仍用原始时辰。排盘上下文、运限、重排、反推剪枝四处共用此一份，不另抄副本——
/// 手抄的第二份一旦与这里漂移，晚子时的盘会静默按另一天排。
pub(crate) fn effective_time_index(day_divide: DayDivide, time_index: u8) -> u8 {
    match day_divide {
        DayDivide::Current | DayDivide::ForwardCarry if time_index >= 12 => 0,
        _ => time_index,
    }
}

/// 晚子时是否整个日期归次日：`day_divide=ForwardCarry` 且为晚子时（>=12）。
pub(crate) fn late_zi_carries(day_divide: DayDivide, time_index: u8) -> bool {
    day_divide == DayDivide::ForwardCarry && time_index >= 12
}

/// 参与推算的公历日期：晚子时整个日期归次日（[`late_zi_carries`]）时取次日，其余原样。
///
/// 与 [`effective_time_index`] 配对使用：两者合起来即「晚子时按次日早子时排」。
pub(crate) fn effective_solar_date(
    day_divide: DayDivide,
    date: (i64, i64, i64),
    time_index: u8,
) -> (i64, i64, i64) {
    if late_zi_carries(day_divide, time_index) {
        shift_solar_date(date, 1)
    } else {
        date
    }
}

/// 公历日期前后挪 `days` 天：按儒略日推，跨月、跨年与 1582 年改历空缺都随之处理；
/// 9999-12-31 的次日是 10000-1-1（农历仍在 9999 年内）。
pub(crate) fn shift_solar_date((year, month, day): (i64, i64, i64), days: i64) -> (i64, i64, i64) {
    let noon = SolarTime::new(year, month, day, 12, 0);
    let shifted = SolarTime::from_julian_day(noon.julian_day() + days as f64);
    (shifted.year, shifted.month, shifted.day)
}

/// 排盘时刻的分钟数。
///
/// lunar-lite 取时辰整点后的 30 分（`ganzhi.js` 的 `Solar.fromYmdHms(…, 30, 0)`）。
/// 按节气取干支是时刻级比较：用 0 分会让节气时刻落在 `HH:00`~`HH:30` 的那些日子
/// 判到节气的另一侧，`horoscope_divide=Exact` 下的月柱静默差一个月。
/// 农历年月日与日柱不受影响——前者是日期级，后者分界在 23:00，两个分钟数同侧。
const CHART_MINUTE: i64 = 30;

/// 时辰索引转小时数：0 → 0 点，1-11 → 2t-1 点，12 → 23 点。
pub(crate) fn time_index_to_hour(time_index: u8) -> i64 {
    match time_index {
        0 => 0,
        12 => 23,
        i => (i as i64) * 2 - 1,
    }
}

/// 排盘时刻：某日某时辰的代表时刻（该时辰首个整点后 [`CHART_MINUTE`] 分）。
pub(crate) fn chart_moment(year: i64, month: i64, day: i64, time_index: u8) -> SolarTime {
    SolarTime::new(
        year,
        month,
        day,
        time_index_to_hour(time_index),
        CHART_MINUTE,
    )
}

// ============================================================
// 主入口
// ============================================================

/// 十二宫的星耀与四组十二神安放结果
struct PlacedStars {
    /// 各宫主星
    major: [Vec<Star>; 12],
    /// 各宫辅星
    minor: [Vec<Star>; 12],
    /// 各宫杂耀
    adjective: [Vec<Star>; 12],
    /// 各宫长生十二神
    changsheng12: [StarKey; 12],
    /// 各宫博士十二神
    boshi12: [StarKey; 12],
    /// 各宫岁前十二神与将前十二神；本命不安（`Yearly12Placement::YearlyOnly`）时为 `None`
    yearly12: Option<([StarKey; 12], [StarKey; 12])>,
}

/// 四柱中随日期与配置推算的三柱；年柱直接取 [`AstroContext`] 的安星年干支
struct Pillars {
    /// 月柱
    monthly: (HeavenlyStem, EarthlyBranch),
    /// 日柱
    daily: (HeavenlyStem, EarthlyBranch),
    /// 时柱
    hourly: (HeavenlyStem, EarthlyBranch),
}

/// 安放全部星耀与四组十二神
///
/// 各星先由 `star::location` 算出落宫索引，再交给 `star` 下的安放函数落到宫上。
/// 两套年干支分工固定：年系杂耀与岁前/将前十二神用流年干支（`horoscope_divide`
/// 分界），主辅星、红鸾天喜与长生/博士十二神用安星年干支（`year_divide` 分界）。
fn place_stars(
    ctx: &AstroContext,
    gender: Gender,
    language: Language,
    config: &Config,
) -> PlacedStars {
    let effective_ti = ctx.effective_time_index;

    // 紫微天府起始宫位（晚子时按次日起，跨月回卷需要当月农历总天数）
    let start_idx = get_start_index(
        ctx.lunar_day,
        effective_ti,
        ctx.month_day_count,
        ctx.five_elements_class.value() as u32,
    );

    // 各星耀落宫索引
    let lu_yang_tuo_ma = get_lu_yang_tuo_ma_index(ctx.yearly_stem, ctx.yearly_branch);
    let kui_yue = get_kui_yue_index(ctx.yearly_stem);
    let zuo_you = get_zuo_you_index(ctx.month_index as u32 + 1);
    let chang_qu = get_chang_qu_index(effective_ti);
    let kong_jie = get_kong_jie_index(effective_ti);
    let huo_ling = get_huo_ling_index(ctx.yearly_branch, effective_ti);
    let daily_stars = get_daily_star_index(
        ctx.lunar_day,
        effective_ti,
        zuo_you.zuo,
        zuo_you.you,
        chang_qu.chang,
        chang_qu.qu,
    );
    let timely_stars = get_timely_star_index(effective_ti);
    let yearly_stars = get_yearly_star_index(
        ctx.soul_body.soul_index,
        ctx.soul_body.body_index,
        ctx.flow_yearly_stem,
        ctx.flow_yearly_branch,
        gender,
        config,
    );
    let monthly_stars = get_monthly_star_index(ctx.month_index, config.tianwu_placement);
    let (tianma, yuema) =
        get_tianma_index(lu_yang_tuo_ma.ma, ctx.month_index, config.tianma_placement);

    let major = get_major_stars(
        start_idx.ziwei,
        start_idx.tianfu,
        ctx.yearly_stem,
        language,
        config,
    );

    let minor = get_minor_stars(
        zuo_you.zuo,
        zuo_you.you,
        chang_qu.chang,
        chang_qu.qu,
        kui_yue.kui,
        kui_yue.yue,
        lu_yang_tuo_ma.lu,
        lu_yang_tuo_ma.yang,
        lu_yang_tuo_ma.tuo,
        tianma,
        kong_jie.kong,
        kong_jie.jie,
        huo_ling.huo,
        huo_ling.ling,
        ctx.yearly_stem,
        language,
        config,
    );

    let changsheng12 = get_changsheng12(
        ctx.five_elements_class,
        gender,
        ctx.yearly_branch,
        config.changsheng_direction,
    );
    let boshi12 = get_boshi12(lu_yang_tuo_ma.lu, gender, ctx.yearly_branch);
    let (suiqian12, jiangqian12) = get_yearly12(ctx.flow_yearly_branch, config.algorithm);

    // 杂耀的安放顺序决定宫内排列，岁前十二神参与其中（天空随岁建同宫）
    let luan_xi = get_luan_xi_index(ctx.yearly_branch);
    let adjective = get_adjective_stars(
        &yearly_stars,
        &monthly_stars,
        &daily_stars,
        timely_stars.taifu,
        timely_stars.fenggao,
        luan_xi.hongluan,
        luan_xi.tianxi,
        yearly_stars.xianchi,
        &suiqian12,
        yuema,
        language,
        config,
    );

    let natal_yearly12 = match config.yearly12_placement {
        Yearly12Placement::NatalAndYearly => Some((suiqian12, jiangqian12)),
        Yearly12Placement::YearlyOnly => None,
    };
    PlacedStars {
        major,
        minor,
        adjective,
        changsheng12,
        boshi12,
        yearly12: natal_yearly12,
    }
}

/// 组装十二宫
///
/// 宫支自寅宫起顺行固定，宫干由年干经五虎遁起排；宫名自命宫起逆行排布。
/// 身宫落在 [`AstroContext`] 给出的宫位索引上；来因宫是宫干与年干相同的那一宫，
/// 子丑二宫（五虎遁排到子丑与寅卯重干）不标来因宫，`original_palace=WithSecondary` 时
/// 标为辅来因宫（只有辛年丑宫、壬年子宫）。
fn build_palaces(
    ctx: &AstroContext,
    stars: &PlacedStars,
    gender: Gender,
    config: &Config,
) -> [PalaceData; 12] {
    let (decadals, ages) = get_decadals_and_ages(
        ctx.soul_body.soul_index,
        ctx.five_elements_class,
        gender,
        ctx.yearly_stem,
        ctx.yearly_branch,
    );
    let palace_names = get_palace_names(ctx.soul_body.soul_index);
    let start_stem = TIGER_RULE[ctx.yearly_stem.index()];

    std::array::from_fn(|i| {
        let palace_stem =
            HeavenlyStem::from_index(fix_index(start_stem.index() as i32 + i as i32, 10));
        let palace_branch = EarthlyBranch::from_index(fix_index(2 + i as i32, 12));
        let stem_matches = palace_stem == ctx.yearly_stem;
        let zi_chou = matches!(palace_branch, EarthlyBranch::Zi | EarthlyBranch::Chou);

        PalaceData {
            index: i,
            name: palace_names[i],
            is_body_palace: ctx.soul_body.body_index == i,
            is_original_palace: stem_matches && !zi_chou,
            is_secondary_original_palace: stem_matches
                && zi_chou
                && config.original_palace == OriginalPalaceMarking::WithSecondary,
            heavenly_stem: palace_stem,
            earthly_branch: palace_branch,
            major_stars: stars.major[i].clone(),
            minor_stars: stars.minor[i].clone(),
            adjective_stars: stars.adjective[i].clone(),
            changsheng12: stars.changsheng12[i],
            boshi12: stars.boshi12[i],
            jiangqian12: stars.yearly12.map(|(_, jiang)| jiang[i]),
            suiqian12: stars.yearly12.map(|(sui, _)| sui[i]),
            decadal: decadals[i].clone(),
            ages: ages[i].clone(),
            overrides: config.overrides.clone(),
        }
    })
}

/// 推算八字四柱中的月、日、时三柱
///
/// 月柱按 `horoscope_divide` 分界：初一分界时由年干经五虎遁推，闰月下半月归下月；
/// 节气分界时取交节时刻换月的月柱。日柱晚子时归次日，时柱天干随日柱推算。
fn build_pillars(ctx: &AstroContext, config: &Config) -> Pillars {
    let monthly = match config.horoscope_divide {
        HoroscopeDivide::Normal => {
            let month_fix: i32 = if ctx.is_leap && ctx.lunar_day > 15 {
                1
            } else {
                0
            };
            (
                HeavenlyStem::from_index(fix_index(
                    TIGER_RULE[ctx.yearly_stem.index()].index() as i32 + ctx.lunar_month as i32 - 1
                        + month_fix,
                    10,
                )),
                EarthlyBranch::from_index(fix_index(
                    2 + ctx.lunar_month as i32 - 1 + month_fix,
                    12,
                )),
            )
        }
        HoroscopeDivide::Exact => calendar::month_exact(&ctx.moment, ctx.lunar_year),
    };

    Pillars {
        monthly,
        daily: ctx.lunar.day_exact,
        hourly: ctx.lunar.time,
    }
}

/// 一次生辰的完整四柱 [年, 月, 日, 时]（按 `config` 的分界口径，与排盘 `raw_dates` 一致）。
///
/// 反推的终验入口：枚举出的候选生辰经此函数算出四柱后与目标比对。
///
/// # Errors
/// 日期非法或时辰越界时返回 [`IztroError`]。
pub(crate) fn four_pillars(
    solar_date: &str,
    time_index: u8,
    config: &Config,
) -> Result<[(HeavenlyStem, EarthlyBranch); 4], IztroError> {
    let ctx = context::derive(solar_date, time_index, true, config)?;
    let pillars = build_pillars(&ctx, config);
    Ok([
        (ctx.yearly_stem, ctx.yearly_branch),
        pillars.monthly,
        pillars.daily,
        pillars.hourly,
    ])
}

/// 通过阳历日期排盘
///
/// # 参数
/// - `solar_date`: 阳历日期字符串（`Y-M-D`，分隔符可为 `-` `/` `.`（前后一致），可带 ` HH:mm` 或 ` HH:mm:ss`；时间不参与排盘；支持 1-9999 年，1582-10-15 以前按儒略历），原样记入星盘的 `solar_date`
/// - `time_index`: 时辰索引 (0=早子, 1=丑, ..., 12=晚子)
/// - `gender`: 性别
/// - `fix_leap`: 是否修正闰月
/// - `language`: 语言
/// - `config`: 排盘配置（分界点与算法派别）
///
/// # Errors
/// 日期格式非法、日期不存在或超出支持范围、时辰索引越界时返回 [`IztroError`]。
pub fn by_solar(
    solar_date: &str,
    time_index: u8,
    gender: Gender,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    // 1. 派生上下文：生效时辰、农历年月日、两套年干支、月索引、命身宫、五行局
    let ctx = context::derive(solar_date, time_index, fix_leap, &config)?;

    // 2. 安星与组装十二宫
    let stars = place_stars(&ctx, gender, language, &config);
    let palaces = build_palaces(&ctx, &stars, gender, &config);

    // 3. 命主星与身主星：命主取命宫地支（中州派取年支），身主恒取年支
    let soul_palace_branch =
        EarthlyBranch::from_index(fix_index(ctx.soul_body.soul_index as i32 + 2, 12));
    let body_palace_branch =
        EarthlyBranch::from_index(fix_index(ctx.soul_body.body_index as i32 + 2, 12));
    let soul_star = if config.algorithm == Algorithm::Zhongzhou {
        get_earthly_branch_info(ctx.yearly_branch).soul
    } else {
        get_earthly_branch_info(soul_palace_branch).soul
    };
    let body_star = get_earthly_branch_info(ctx.yearly_branch).body;

    // 4. 八字四柱：年柱与安星年干支同分界，其余三柱另算
    let pillars = build_pillars(&ctx, &config);
    let chinese_date = translate_chinese_date(
        [
            (ctx.yearly_stem, ctx.yearly_branch),
            pillars.monthly,
            pillars.daily,
            pillars.hourly,
        ],
        language,
    );

    // 5. 展示字段：星座、生肖、时辰、农历日期串（按排盘语言）
    let sign_index = calendar::sign_index(ctx.solar_date.1, ctx.solar_date.2);
    let raw_lunar_date = RawLunarDate {
        lunar_year: ctx.lunar_year,
        lunar_month: ctx.lunar_month,
        lunar_day: ctx.lunar_day,
        is_leap: ctx.is_leap,
    };
    let chart = Astrolabe {
        gender,
        solar_date: solar_date.to_string(),
        lunar_date: translate_lunar_date(
            raw_lunar_date.lunar_year,
            raw_lunar_date.lunar_month,
            raw_lunar_date.lunar_day,
            raw_lunar_date.is_leap,
            language,
        ),
        chinese_date,
        time: translate_time(time_index, language).to_string(),
        time_range: TIME_RANGES[time_index as usize].to_string(),
        sign: translate_sign(sign_index, language).to_string(),
        sign_key: SIGNS[sign_index].to_string(),
        zodiac: translate_zodiac(ctx.yearly_branch, language).to_string(),
        zodiac_key: ZODIAC[ctx.yearly_branch.index()].to_string(),
        earthly_branch_of_soul_palace: soul_palace_branch,
        earthly_branch_of_body_palace: body_palace_branch,
        soul: soul_star,
        body: body_star,
        five_elements_class: ctx.five_elements_class,
        palaces,
        raw_dates: RawDates {
            lunar_date: raw_lunar_date,
            chinese_date: RawChineseDate {
                yearly: (ctx.yearly_stem, ctx.yearly_branch),
                monthly: pillars.monthly,
                daily: pillars.daily,
                hourly: pillars.hourly,
            },
        },
        time_index,
        fix_leap,
        language,
        config,
    };

    // 6. 地盘与人盘以身宫、福德宫的干支重排；天盘即上面排好的结果
    Ok(match chart.config.astro_type {
        AstroType::Heaven => chart,
        AstroType::Earth => rearrange_from_palace(&chart, |p| p.is_body_palace)?,
        AstroType::Human => rearrange_from_palace(&chart, |p| p.name == Palace::Spirit)?,
    })
}

/// 以满足 `pick` 的那一宫的干支重排星盘；十二宫必有身宫与福德宫，故必然命中。
fn rearrange_from_palace(
    chart: &Astrolabe,
    pick: impl Fn(&PalaceData) -> bool,
) -> Result<Astrolabe, IztroError> {
    let from = chart
        .palaces
        .iter()
        .find(|p| pick(p))
        .expect("十二宫必然包含身宫与福德宫");
    chart.rearranged(from.heavenly_stem, from.earthly_branch)
}

/// 通过农历日期排盘
///
/// # 参数
/// - `lunar_date`: 农历日期字符串（写法同公历）
/// - `time_index`: 时辰索引 (0=早子, 1=丑, ..., 12=晚子)
/// - `gender`: 性别
/// - `leap`: 输入月是否闰月及闰月处理方式（见 [`LeapMonth`]）
/// - `language`: 语言
/// - `config`: 排盘配置（分界点与算法派别）
pub fn by_lunar(
    lunar_date: &str,
    time_index: u8,
    gender: Gender,
    leap: LeapMonth,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    validate_time_index(time_index)?;

    // 1. 解析并校验农历日期（闰月用负数月号表示）
    let (year, lunar_month, day) = parse_lunar_date(lunar_date, leap.is_leap_month())?;

    // 2. 转换为阳历（日期串不带前导零）；月与日已在第 1 步校验，必有对应日期
    let (sy, sm, sd) = calendar::solar_date_of(year, lunar_month, day)
        .expect("parse_lunar_date 已校验该农历月存在且日在当月天数内");

    // 3. 用阳历日期排盘
    by_solar(
        &format!("{sy}-{sm}-{sd}"),
        time_index,
        gender,
        leap.fix_leap(),
        language,
        config,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_heavenly_stem() {
        assert_eq!(parse_heavenly_stem("甲"), Some(HeavenlyStem::Jia));
        assert_eq!(parse_heavenly_stem("癸"), Some(HeavenlyStem::Gui));
        assert_eq!(parse_heavenly_stem("X"), None);
    }

    #[test]
    fn test_parse_earthly_branch() {
        assert_eq!(parse_earthly_branch("子"), Some(EarthlyBranch::Zi));
        assert_eq!(parse_earthly_branch("亥"), Some(EarthlyBranch::Hai));
        assert_eq!(parse_earthly_branch("X"), None);
    }

    #[test]
    fn test_fix_lunar_month_index() {
        // 正月，非闰月
        assert_eq!(fix_lunar_month_index(1, 15, false, 0, true), 0);
        // 七月，非闰月
        assert_eq!(fix_lunar_month_index(7, 17, false, 0, true), 6);
        // 闰月，fix_leap=true，日>15，时辰非晚子
        assert_eq!(fix_lunar_month_index(4, 16, true, 0, true), 4);
        // 闰月，fix_leap=true，日<=15
        assert_eq!(fix_lunar_month_index(4, 15, true, 0, true), 3);
        // 闰月，fix_leap=false
        assert_eq!(fix_lunar_month_index(4, 16, true, 0, false), 3);
    }

    #[test]
    fn test_fix_lunar_day_index() {
        assert_eq!(fix_lunar_day_index(15, 0), 14);
        assert_eq!(fix_lunar_day_index(15, 12), 15);
        assert_eq!(fix_lunar_day_index(1, 0), 0);
    }

    #[test]
    fn test_time_index_to_hour() {
        assert_eq!(time_index_to_hour(0), 0);
        assert_eq!(time_index_to_hour(1), 1);
        assert_eq!(time_index_to_hour(2), 3);
        assert_eq!(time_index_to_hour(6), 11);
        assert_eq!(time_index_to_hour(12), 23);
    }

    #[test]
    fn test_by_solar_basic() {
        // 2000年8月16日，子时，男，default
        let astrolabe = by_solar(
            "2000-8-16",
            0,
            Gender::Male,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        assert_eq!(astrolabe.gender, Gender::Male);
        assert_eq!(astrolabe.solar_date, "2000-8-16");
        assert_eq!(astrolabe.palaces.len(), 12);

        // 验证12宫都有名称
        for p in &astrolabe.palaces {
            assert!(p.index < 12);
        }
    }

    #[test]
    fn test_by_solar_has_all_stars() {
        let astrolabe = by_solar(
            "2000-8-16",
            0,
            Gender::Male,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();

        // 应有14颗主星
        let total_major: usize = astrolabe.palaces.iter().map(|p| p.major_stars.len()).sum();
        assert_eq!(total_major, 14);

        // 应有14颗辅星
        let total_minor: usize = astrolabe.palaces.iter().map(|p| p.minor_stars.len()).sum();
        assert_eq!(total_minor, 14);
    }

    #[test]
    fn test_by_lunar_basic() {
        // 农历2000年七月十七
        let astrolabe = by_lunar(
            "2000-7-17",
            0,
            Gender::Male,
            LeapMonth::NotLeap,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        assert_eq!(astrolabe.gender, Gender::Male);
        assert_eq!(astrolabe.palaces.len(), 12);
    }

    #[test]
    fn test_by_solar_late_zi() {
        // 晚子时
        let astrolabe = by_solar(
            "2000-8-16",
            12,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        assert_eq!(astrolabe.time, "晚子时");
        assert_eq!(astrolabe.time_range, "23:00~00:00");
    }

    #[test]
    fn test_by_solar_zhongzhou() {
        let astrolabe = by_solar(
            "2000-8-16",
            0,
            Gender::Male,
            true,
            Language::ZhCN,
            Config {
                algorithm: Algorithm::Zhongzhou,
                ..Config::default()
            },
        )
        .unwrap();
        assert_eq!(astrolabe.palaces.len(), 12);
    }

    #[test]
    fn test_exactly_one_body_palace() {
        let astrolabe = by_solar(
            "1990-5-15",
            3,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let body_count = astrolabe
            .palaces
            .iter()
            .filter(|p| p.is_body_palace)
            .count();
        assert_eq!(body_count, 1);
    }
}

//! 排盘主流程
//!
//! 提供 `by_solar` 和 `by_lunar` 两个入口函数，从阳历或阴历日期生成完整的紫微斗数星盘。

use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::lunar_year::{self, LunarYearRefHelper};
use lunar_rust::solar::SolarRefHelper;
use lunar_rust::{lunar, solar};

use crate::astro::context::{self, AstroContext};
use crate::astro::lunar_table;
use crate::astro::palace::{get_decadals_and_ages, get_palace_names};
use crate::data::constants::{SIGNS, TIGER_RULE, TIME_RANGES, ZODIAC};
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::i18n::{translate_sign, translate_time, translate_zodiac};
use crate::models::astrolabe::{Astrolabe, RawChineseDate, RawDates, RawLunarDate};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::star::adjective::get_adjective_stars;
use crate::star::decorative::{get_boshi12, get_changsheng12, get_yearly12};
use crate::star::location::{
    get_chang_qu_index, get_daily_star_index, get_huo_ling_index, get_kong_jie_index,
    get_kui_yue_index, get_lu_yang_tuo_ma_index, get_luan_xi_index, get_monthly_star_index,
    get_start_index, get_timely_star_index, get_yearly_star_index, get_zuo_you_index,
};
use crate::star::major::get_major_stars;
use crate::star::minor::get_minor_stars;
use crate::utils::{fix_index, translate_chinese_date};

// ============================================================
// 辅助函数
// ============================================================

/// lunar_rust 星座中文名转黄道索引（白羊=0 … 双鱼=11）
fn parse_sign_index(name: &str) -> Result<usize, IztroError> {
    const XING_ZUO: [&str; 12] = [
        "白羊", "金牛", "双子", "巨蟹", "狮子", "处女", "天秤", "天蝎", "射手", "摩羯", "水瓶",
        "双鱼",
    ];
    XING_ZUO
        .iter()
        .position(|s| *s == name)
        .ok_or_else(|| lunar_defect("sign", name))
}

/// lunar_rust 交回了本该在表内、实际却查不到的干支或星座原文时的错误。
///
/// 这是依赖库行为超出预期，不是调用方的过错，故不 panic：wasm 下 panic 即
/// trap，且每次 trap 都永久损耗模块实例的栈空间。
fn lunar_defect(what: &str, value: &str) -> IztroError {
    IztroError::Internal(format!("lunar_rust returned an unknown {what}: '{value}'"))
}

/// 把 lunar_rust 交回的天干原文落成枚举，`what` 说明它是哪一柱。
pub(crate) fn stem_of(value: &str, what: &str) -> Result<HeavenlyStem, IztroError> {
    parse_heavenly_stem(value).ok_or_else(|| lunar_defect(&format!("{what} heavenly stem"), value))
}

/// 把 lunar_rust 交回的地支原文落成枚举，`what` 说明它是哪一柱。
pub(crate) fn branch_of(value: &str, what: &str) -> Result<EarthlyBranch, IztroError> {
    parse_earthly_branch(value)
        .ok_or_else(|| lunar_defect(&format!("{what} earthly branch"), value))
}

/// 支持的公历年份范围。下限避开 1582 年格里历改革（lunar_rust 对改革空洞
/// 日期 panic），上限为 lunar_rust 农历表的覆盖终点。
const SUPPORTED_YEARS: std::ops::RangeInclusive<i64> = 1583..=9999;

/// 公历某月天数（格里历闰年规则）；月份非法返回 0。
fn days_in_month(year: i64, month: i64) -> i64 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 0,
    }
}

/// 校验时辰索引在 0-12（0=早子时，12=晚子时）。
pub(crate) fn validate_time_index(time_index: u8) -> Result<(), IztroError> {
    if time_index > 12 {
        return Err(IztroError::InvalidTimeIndex(time_index));
    }
    Ok(())
}

/// 解析并校验 "YYYY-M-D" 公历日期串，返回 (年, 月, 日)。
/// 拒绝格式错误、不存在的日期与超出 [`SUPPORTED_YEARS`] 的年份。
pub(crate) fn parse_solar_date(solar_date: &str) -> Result<(i64, i64, i64), IztroError> {
    let err = |detail: &str| {
        IztroError::InvalidDate(format!("invalid solar date '{solar_date}': {detail}"))
    };
    let parts: Vec<&str> = solar_date.split('-').collect();
    if parts.len() != 3 {
        return Err(err("expected 'YYYY-M-D'"));
    }
    let year: i64 = parts[0].parse().map_err(|_| err("year is not a number"))?;
    let month: i64 = parts[1].parse().map_err(|_| err("month is not a number"))?;
    let day: i64 = parts[2].parse().map_err(|_| err("day is not a number"))?;
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
    if day < 1 || day > days_in_month(year, month) {
        return Err(err("day is out of range for that month"));
    }
    Ok((year, month, day))
}

/// 解析并校验 "YYYY-M-D" 农历日期串，返回 (年, 带符号月, 日)。
/// 月为负值表示闰月；该年该月并非闰月时 `is_leap_month` 不生效，
/// 与 JS iztro 行为一致。日按修正后的农历月表校验（大月 30 / 小月 29，
/// 见 [`lunar_table`]）。
fn parse_lunar_date(lunar_date: &str, is_leap_month: bool) -> Result<(i64, i64, i64), IztroError> {
    let err = |detail: &str| {
        IztroError::InvalidDate(format!("invalid lunar date '{lunar_date}': {detail}"))
    };
    let parts: Vec<&str> = lunar_date.split('-').collect();
    if parts.len() != 3 {
        return Err(err("expected 'YYYY-M-D'"));
    }
    let year: i64 = parts[0].parse().map_err(|_| err("year is not a number"))?;
    let month: i64 = parts[1].parse().map_err(|_| err("month is not a number"))?;
    let day: i64 = parts[2].parse().map_err(|_| err("day is not a number"))?;
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
    let year_has_this_leap = lunar_year::LunarYear::from_lunar_year(year)
        .get_leap_months()
        .abs()
        == month;
    let signed_month = if is_leap_month && year_has_this_leap {
        -month
    } else {
        month
    };
    let day_count = lunar_table::month_day_count(year, signed_month)
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
    let need_to_add = is_leap && fix_leap && lunar_day > 15 && time_index != 12;
    fix_index(lunar_month as i32 - 1 + if need_to_add { 1 } else { 0 }, 12)
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

/// 时辰索引转小时数（用于 lunar_rust 日期创建）
pub(crate) fn time_index_to_hour(time_index: u8) -> i64 {
    match time_index {
        0 => 0,
        12 => 23,
        i => (i as i64) * 2 - 1,
    }
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
    /// 各宫岁前十二神
    suiqian12: [StarKey; 12],
    /// 各宫将前十二神
    jiangqian12: [StarKey; 12],
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
        config.algorithm,
    );
    let monthly_stars = get_monthly_star_index(ctx.month_index);

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
        lu_yang_tuo_ma.ma,
        kong_jie.kong,
        kong_jie.jie,
        huo_ling.huo,
        huo_ling.ling,
        ctx.yearly_stem,
        language,
        config,
    );

    let changsheng12 = get_changsheng12(ctx.five_elements_class, gender, ctx.yearly_branch);
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
        config.algorithm,
        language,
    );

    PlacedStars {
        major,
        minor,
        adjective,
        changsheng12,
        boshi12,
        suiqian12,
        jiangqian12,
    }
}

/// 组装十二宫
///
/// 宫支自寅宫起顺行固定，宫干由年干经五虎遁起排；宫名自命宫起逆行排布。
/// 身宫落在 [`AstroContext`] 给出的宫位索引上；来因宫是宫干与年干相同的那一宫，
/// 子丑二宫无宫干可对（五虎遁排到子丑会与寅卯重干），故排除。
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

        PalaceData {
            index: i,
            name: palace_names[i],
            is_body_palace: ctx.soul_body.body_index == i,
            is_original_palace: palace_branch != EarthlyBranch::Zi
                && palace_branch != EarthlyBranch::Chou
                && palace_stem == ctx.yearly_stem,
            heavenly_stem: palace_stem,
            earthly_branch: palace_branch,
            major_stars: stars.major[i].clone(),
            minor_stars: stars.minor[i].clone(),
            adjective_stars: stars.adjective[i].clone(),
            changsheng12: stars.changsheng12[i],
            boshi12: stars.boshi12[i],
            jiangqian12: stars.jiangqian12[i],
            suiqian12: stars.suiqian12[i],
            decadal: decadals[i].clone(),
            ages: ages[i].clone(),
            overrides: config.overrides.clone(),
        }
    })
}

/// 推算八字四柱中的月、日、时三柱
///
/// 月柱按 `horoscope_divide` 分界：初一分界时由年干经五虎遁推，闰月下半月归下月；
/// 节气分界时取 lunar_rust 的精确时刻月柱。日柱晚子时归次日，时柱天干随日柱推算。
///
/// # Errors
/// lunar_rust 交回表外的干支原文时返回 [`IztroError::Internal`]。
fn build_pillars(
    ctx: &AstroContext,
    lunar_ref: &lunar::LunarRef,
    config: &Config,
) -> Result<Pillars, IztroError> {
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
        HoroscopeDivide::Exact => {
            let gan = lunar_ref.get_month_gan_exact();
            let zhi = lunar_ref.get_month_zhi_exact();
            (stem_of(&gan, "month")?, branch_of(&zhi, "month")?)
        }
    };

    let day_gan = lunar_ref.get_day_gan_exact();
    let day_zhi = lunar_ref.get_day_zhi_exact();
    let time_gan = lunar_ref.get_time_gan();
    let time_zhi = lunar_ref.get_time_zhi();

    Ok(Pillars {
        monthly,
        daily: (stem_of(&day_gan, "day")?, branch_of(&day_zhi, "day")?),
        hourly: (stem_of(&time_gan, "time")?, branch_of(&time_zhi, "time")?),
    })
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
    let (year, month, day) = parse_solar_date(solar_date)?;
    let lunar_ref = lunar::from_solar(&solar::from_ymdhms(
        year,
        month,
        day,
        time_index_to_hour(ctx.effective_time_index),
        0,
        0,
    ));
    let pillars = build_pillars(&ctx, &lunar_ref, config)?;
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
/// - `solar_date`: 阳历日期字符串，格式 "YYYY-M-D"（支持 1583-9999 年）
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

    let (year, month, day) = parse_solar_date(solar_date)?;
    let solar_ref = solar::from_ymd(year, month, day);
    // 时柱随生效时辰推算，因此日期对象要带上对应小时
    let lunar_ref = lunar::from_solar(&solar::from_ymdhms(
        year,
        month,
        day,
        time_index_to_hour(ctx.effective_time_index),
        0,
        0,
    ));

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
    let pillars = build_pillars(&ctx, &lunar_ref, &config)?;
    let chinese_date = translate_chinese_date(
        [
            (ctx.yearly_stem, ctx.yearly_branch),
            pillars.monthly,
            pillars.daily,
            pillars.hourly,
        ],
        language,
    );

    // 5. 展示字段：星座、生肖、时辰、农历日期串
    //    （月/日名取修正后的农历日期，闰月带「闰」前缀；年名不经月表，直接取）
    let sign_index = parse_sign_index(&solar_ref.get_xing_zuo())?;
    let chart = Astrolabe {
        gender,
        solar_date: solar_date.to_string(),
        lunar_date: format!(
            "{}年{}月{}",
            lunar_ref.get_year_in_chinese(),
            lunar_table::month_in_chinese(ctx.lunar_month, ctx.is_leap),
            lunar_table::day_in_chinese(ctx.lunar_day),
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
            lunar_date: RawLunarDate {
                lunar_year: lunar_ref.get_year(),
                lunar_month: ctx.lunar_month,
                lunar_day: ctx.lunar_day,
                is_leap: ctx.is_leap,
            },
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
/// - `lunar_date`: 农历日期字符串，格式 "YYYY-M-D"
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

    // 2. 转换为阳历（日期串不带前导零）
    let solar_date = lunar_table::solar_date_of(year, lunar_month, day);

    // 3. 用阳历日期排盘
    by_solar(
        &solar_date,
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

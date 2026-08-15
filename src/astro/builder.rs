//! 排盘主流程
//!
//! 提供 `by_solar` 和 `by_lunar` 两个入口函数，从阳历或阴历日期生成完整的紫微斗数星盘。

use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::lunar_month::LunarMonthRefHelper;
use lunar_rust::lunar_year::{self, LunarYearRefHelper};
use lunar_rust::solar::SolarRefHelper;
use lunar_rust::{lunar, solar};

use crate::astro::context::{self, AstroContext};
use crate::astro::palace::{get_decadals_and_ages, get_palace_names};
use crate::data::constants::{TIGER_RULE, TIME_RANGES};
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::types::*;
use crate::error::IztroError;
use crate::i18n::{translate_sign, translate_time, translate_zodiac};
use crate::models::astrolabe::{Astrolabe, RawChineseDate, RawDates, RawLunarDate};
use crate::models::palace::PalaceData;
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
fn parse_sign_index(name: &str) -> usize {
    const XING_ZUO: [&str; 12] = [
        "白羊", "金牛", "双子", "巨蟹", "狮子", "处女", "天秤", "天蝎", "射手", "摩羯", "水瓶",
        "双鱼",
    ];
    XING_ZUO
        .iter()
        .position(|s| *s == name)
        .unwrap_or_else(|| panic!("Unknown xing zuo: {name}"))
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
/// 与 JS iztro 行为一致。日按 lunar_rust 月表校验（大月 30 / 小月 29）。
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
    let day_count = lunar_year::LunarYear::from_lunar_year(year)
        .get_month(signed_month)
        .map(|m| m.get_day_count())
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
    let AstroContext {
        effective_time_index: effective_ti,
        lunar_day,
        lunar_month,
        is_leap,
        month_day_count,
        yearly_stem,
        yearly_branch,
        flow_yearly_stem,
        flow_yearly_branch,
        month_index,
        soul_body,
        five_elements_class,
    } = context::derive(solar_date, time_index, fix_leap, &config)?;

    let (year, month, day) = parse_solar_date(solar_date)?;
    let solar_ref = solar::from_ymd(year, month, day);
    // 时柱随生效时辰推算，因此日期对象要带上对应小时
    let lunar_ref = lunar::from_solar(&solar::from_ymdhms(
        year,
        month,
        day,
        time_index_to_hour(effective_ti),
        0,
        0,
    ));

    // 2. 紫微天府起始宫位（晚子时按次日起，跨月回卷需要当月农历总天数）
    let start_idx = get_start_index(
        lunar_day,
        effective_ti,
        month_day_count,
        five_elements_class.value() as u32,
    );

    // 9. 计算各星耀位置索引
    let lu_yang_tuo_ma = get_lu_yang_tuo_ma_index(yearly_stem, yearly_branch);
    let kui_yue = get_kui_yue_index(yearly_stem);
    let zuo_you = get_zuo_you_index(month_index as u32 + 1);
    let chang_qu = get_chang_qu_index(effective_ti);
    let kong_jie = get_kong_jie_index(effective_ti);
    let huo_ling = get_huo_ling_index(yearly_branch, effective_ti);
    let daily_stars = get_daily_star_index(
        lunar_day,
        effective_ti,
        zuo_you.zuo,
        zuo_you.you,
        chang_qu.chang,
        chang_qu.qu,
    );
    let timely_stars = get_timely_star_index(effective_ti);
    let yearly_stars = get_yearly_star_index(
        soul_body.soul_index,
        soul_body.body_index,
        flow_yearly_stem,
        flow_yearly_branch,
        gender,
        config.algorithm,
    );
    let monthly_stars = get_monthly_star_index(month_index);

    // 10. 安主星
    let major_stars = get_major_stars(
        start_idx.ziwei,
        start_idx.tianfu,
        yearly_stem,
        language,
        &config,
    );

    // 11. 安辅星
    let minor_stars = get_minor_stars(
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
        yearly_stem,
        language,
        &config,
    );

    // 12. 流耀（岁前/将前12 属流年神煞，年支按 horoscope_divide）
    let changsheng12 = get_changsheng12(five_elements_class, gender, yearly_branch);
    let boshi12 = get_boshi12(lu_yang_tuo_ma.lu, gender, yearly_branch);
    let (suiqian12, jiangqian12) = get_yearly12(flow_yearly_branch, config.algorithm);

    // 13. 杂耀（红鸾天喜按 year_divide 年支，其余年系星按 horoscope_divide 年支）
    let luan_xi = get_luan_xi_index(yearly_branch);
    let adjective_stars = get_adjective_stars(
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

    // 14. 大限和小限
    let (decadals, ages) = get_decadals_and_ages(
        soul_body.soul_index,
        five_elements_class,
        gender,
        yearly_stem,
        yearly_branch,
    );

    // 15. 十二宫名称
    let palace_names = get_palace_names(soul_body.soul_index);

    // 16. 组装十二宫
    let start_stem = TIGER_RULE[yearly_stem.index()];
    let mut palaces = Vec::with_capacity(12);

    for i in 0..12usize {
        let palace_stem_index = fix_index(start_stem.index() as i32 + i as i32, 10);
        let palace_stem = HeavenlyStem::from_index(palace_stem_index);
        let palace_branch = EarthlyBranch::from_index(fix_index(2 + i as i32, 12));

        // 判断是否为身宫所在宫
        let is_body_palace = soul_body.body_index == i;

        // 判断是否为原始宫位（来因宫）
        // 条件：不在子宫或丑宫，且宫位天干与年干相同
        let is_original_palace = palace_branch != EarthlyBranch::Zi
            && palace_branch != EarthlyBranch::Chou
            && palace_stem == yearly_stem;

        palaces.push(PalaceData {
            index: i,
            name: palace_names[i],
            is_body_palace,
            is_original_palace,
            heavenly_stem: palace_stem,
            earthly_branch: palace_branch,
            major_stars: major_stars[i].clone(),
            minor_stars: minor_stars[i].clone(),
            adjective_stars: adjective_stars[i].clone(),
            changsheng12: changsheng12[i],
            boshi12: boshi12[i],
            jiangqian12: jiangqian12[i],
            suiqian12: suiqian12[i],
            decadal: decadals[i].clone(),
            ages: ages[i].clone(),
            overrides: config.overrides.clone(),
        });
    }

    // 17. 命主星与身主星
    let soul_palace_branch =
        EarthlyBranch::from_index(fix_index(soul_body.soul_index as i32 + 2, 12));
    let body_palace_branch =
        EarthlyBranch::from_index(fix_index(soul_body.body_index as i32 + 2, 12));

    let soul_star = if config.algorithm == Algorithm::Zhongzhou {
        get_earthly_branch_info(yearly_branch).soul
    } else {
        get_earthly_branch_info(soul_palace_branch).soul
    };
    let body_star = get_earthly_branch_info(yearly_branch).body;

    // 18. 干支纪日（八字四柱）
    //     年柱与安星年干支同分界；月柱按运限分界配置推算
    //     （初一分界以五虎遁推，闰月下半月归下月；节气分界取精确时刻月柱）；
    //     日柱晚子时归次日；时柱天干随日柱推算
    let (month_pillar_stem, month_pillar_branch) = match config.horoscope_divide {
        HoroscopeDivide::Normal => {
            let month_fix: i32 = if is_leap && lunar_day > 15 { 1 } else { 0 };
            (
                HeavenlyStem::from_index(fix_index(
                    TIGER_RULE[yearly_stem.index()].index() as i32 + lunar_month as i32 - 1
                        + month_fix,
                    10,
                )),
                EarthlyBranch::from_index(fix_index(2 + lunar_month as i32 - 1 + month_fix, 12)),
            )
        }
        HoroscopeDivide::Exact => {
            let gan = lunar_ref.get_month_gan_exact();
            let zhi = lunar_ref.get_month_zhi_exact();
            (
                parse_heavenly_stem(&gan).unwrap_or_else(|| panic!("Unknown month stem: {gan}")),
                parse_earthly_branch(&zhi).unwrap_or_else(|| panic!("Unknown month branch: {zhi}")),
            )
        }
    };

    let day_gan_str = lunar_ref.get_day_gan_exact();
    let day_zhi_str = lunar_ref.get_day_zhi_exact();
    let day_pillar_stem = parse_heavenly_stem(&day_gan_str)
        .unwrap_or_else(|| panic!("Unknown day stem: {day_gan_str}"));
    let day_pillar_branch = parse_earthly_branch(&day_zhi_str)
        .unwrap_or_else(|| panic!("Unknown day branch: {day_zhi_str}"));

    let time_gan_str = lunar_ref.get_time_gan();
    let time_zhi_str = lunar_ref.get_time_zhi();
    let time_pillar_stem = parse_heavenly_stem(&time_gan_str)
        .unwrap_or_else(|| panic!("Unknown time stem: {time_gan_str}"));
    let time_pillar_branch = parse_earthly_branch(&time_zhi_str)
        .unwrap_or_else(|| panic!("Unknown time branch: {time_zhi_str}"));

    let chinese_date = translate_chinese_date(
        [
            (yearly_stem, yearly_branch),
            (month_pillar_stem, month_pillar_branch),
            (day_pillar_stem, day_pillar_branch),
            (time_pillar_stem, time_pillar_branch),
        ],
        language,
    );

    // 19. 星座与生肖
    let sign = translate_sign(parse_sign_index(&solar_ref.get_xing_zuo()), language).to_string();
    let zodiac = translate_zodiac(yearly_branch, language).to_string();

    // 20. 时辰显示
    let time_str = translate_time(time_index, language).to_string();
    let time_range = TIME_RANGES[time_index as usize].to_string();

    // 21. 农历日期字符串（lunar_rust 的月份中文名对闰月自带「闰」前缀）
    let lunar_date_str = format!(
        "{}年{}月{}",
        lunar_ref.get_year_in_chinese(),
        lunar_ref.get_month_in_chinese(),
        lunar_ref.get_day_in_chinese(),
    );

    let chart = Astrolabe {
        gender,
        solar_date: solar_date.to_string(),
        lunar_date: lunar_date_str,
        chinese_date,
        time: time_str,
        time_range,
        sign,
        zodiac,
        earthly_branch_of_soul_palace: soul_palace_branch,
        earthly_branch_of_body_palace: body_palace_branch,
        soul: soul_star,
        body: body_star,
        five_elements_class,
        palaces,
        raw_dates: RawDates {
            lunar_date: RawLunarDate {
                lunar_year: lunar_ref.get_year(),
                lunar_month,
                lunar_day,
                is_leap,
            },
            chinese_date: RawChineseDate {
                yearly: (yearly_stem, yearly_branch),
                monthly: (month_pillar_stem, month_pillar_branch),
                daily: (day_pillar_stem, day_pillar_branch),
                hourly: (time_pillar_stem, time_pillar_branch),
            },
        },
        time_index,
        fix_leap,
        language,
        config,
    };

    // 17. 地盘与人盘以身宫、福德宫的干支重排；天盘即上面排好的结果
    Ok(match chart.config.astro_type {
        AstroType::Heaven => chart,
        AstroType::Earth => rearrange_from_palace(&chart, |p| p.is_body_palace),
        AstroType::Human => rearrange_from_palace(&chart, |p| p.name == Palace::Spirit),
    })
}

/// 以满足 `pick` 的那一宫的干支重排星盘；十二宫必有身宫与福德宫，故必然命中。
fn rearrange_from_palace(chart: &Astrolabe, pick: impl Fn(&PalaceData) -> bool) -> Astrolabe {
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
/// - `is_leap_month`: 是否为闰月
/// - `fix_leap`: 是否修正闰月
/// - `language`: 语言
/// - `config`: 排盘配置（分界点与算法派别）
pub fn by_lunar(
    lunar_date: &str,
    time_index: u8,
    gender: Gender,
    is_leap_month: bool,
    fix_leap: bool,
    language: Language,
    config: Config,
) -> Result<Astrolabe, IztroError> {
    validate_time_index(time_index)?;

    // 1. 解析并校验农历日期（闰月在 lunar_rust 中用负数月号表示）
    let (year, lunar_month, day) = parse_lunar_date(lunar_date, is_leap_month)?;
    let lunar_ref = lunar::from_ymd(year, lunar_month, day);

    // 3. 转换为阳历（日期串不带前导零）
    let solar_ref = lunar_ref.get_solar();
    let solar_date = format!(
        "{}-{}-{}",
        solar_ref.get_year(),
        solar_ref.get_month(),
        solar_ref.get_day(),
    );

    // 4. 用阳历日期排盘
    by_solar(&solar_date, time_index, gender, fix_leap, language, config)
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
            false,
            true,
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

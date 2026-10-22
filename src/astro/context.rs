//! 排盘派生上下文
//!
//! 出生数据（公历日期、时辰、性别、闰月修正、配置）推不出星耀位置，
//! 中间要先落成一组量：生效时辰、农历年月日、两套年干支、月索引、命身宫、五行局。
//! 排盘主流程与 `star` 模块的各安星入口都从这一步出发，因此单独成型。

use crate::astro::builder::{
    chart_moment, effective_time_index, fix_lunar_month_index, parse_solar_date,
    validate_time_index,
};
use crate::astro::palace::{SoulAndBody, get_five_elements_class, get_soul_and_body};
use crate::calendar::{self, Lunar, SolarTime};
use crate::data::types::*;
use crate::error::IztroError;

/// 由出生数据推出的、安星各步骤共用的中间量
pub struct AstroContext {
    /// 参与推算的时辰索引；`day_divide=current` 且为晚子时（>=12）时归零，展示仍用原始时辰
    pub effective_time_index: u8,
    /// 农历年（正月初一换年）
    pub lunar_year: i64,
    /// 农历日
    pub lunar_day: u32,
    /// 农历月（闰月取正数月份，闰否见 `is_leap`）
    pub lunar_month: u32,
    /// 该农历月是否为闰月
    pub is_leap: bool,
    /// 该农历月的总天数，紫微天府起宫跨月回卷时用
    pub month_day_count: u32,
    /// 安星年干：按 `year_divide` 分界，驱动四化、辅星、命主身主、宫干、大限小限
    pub yearly_stem: HeavenlyStem,
    /// 安星年支：同上
    pub yearly_branch: EarthlyBranch,
    /// 流年年干：按 `horoscope_divide` 分界，驱动年系杂耀与岁前/将前12
    pub flow_yearly_stem: HeavenlyStem,
    /// 流年年支：同上
    pub flow_yearly_branch: EarthlyBranch,
    /// 月索引（0-based，已按 `fix_leap` 修正闰月）
    pub month_index: usize,
    /// 命宫与身宫
    pub soul_body: SoulAndBody,
    /// 五行局
    pub five_elements_class: FiveElementsClass,
    /// 排盘时刻：出生日期加生效时辰的代表时刻
    pub(crate) moment: SolarTime,
    /// 排盘时刻的农历日期与各柱
    pub(crate) lunar: Lunar,
}

/// 由公历日期推出排盘上下文
///
/// `solar_date` 为 `Y-M-D`（分隔符可为 `-` `/` `.`，可带时间部分），`time_index` 为 0-12；两者的合法性在此校验。
pub fn derive(
    solar_date: &str,
    time_index: u8,
    fix_leap: bool,
    config: &Config,
) -> Result<AstroContext, IztroError> {
    validate_time_index(time_index)?;
    let (year, month, day) = parse_solar_date(solar_date)?;

    let effective_time_index = effective_time_index(config.day_divide, time_index);
    let moment = chart_moment(year, month, day, effective_time_index);
    let lunar = calendar::lunar_of(&moment);
    let lunar_date = lunar.date;
    let is_leap = lunar_date.month < 0; // 负值表示闰月
    let lunar_month = lunar_date.month.unsigned_abs() as u32;
    let lunar_day = lunar_date.day as u32;

    let (yearly_stem, yearly_branch) = match config.year_divide {
        YearDivide::Normal => lunar.year,
        YearDivide::Exact => calendar::year_by_li_chun(&moment, lunar_date.year),
    };
    let (flow_yearly_stem, flow_yearly_branch) = match config.horoscope_divide {
        HoroscopeDivide::Normal => lunar.year,
        HoroscopeDivide::Exact => calendar::year_by_li_chun(&moment, lunar_date.year),
    };

    let month_index = fix_lunar_month_index(
        lunar_month,
        lunar_day,
        is_leap,
        effective_time_index,
        fix_leap,
    );

    let soul_body = get_soul_and_body(month_index, effective_time_index, yearly_stem);
    let five_elements_class = get_five_elements_class(
        soul_body.heavenly_stem_of_soul,
        soul_body.earthly_branch_of_soul,
    );

    // 月天数取农历年 lunar_date.year 的月表（与 iztro 的 getTotalDaysOfLunarMonth 同源），
    // 农历日期取自公历年的月表；两表在 1-9999 年的取值路径上一致
    let month_day_count =
        calendar::month_day_count(lunar_date.year, lunar_date.month).ok_or_else(|| {
            IztroError::Internal(format!(
                "lunar month {} does not exist in lunar year {}",
                lunar_date.month, lunar_date.year
            ))
        })? as u32;

    Ok(AstroContext {
        effective_time_index,
        lunar_year: lunar_date.year,
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
        moment,
        lunar,
    })
}

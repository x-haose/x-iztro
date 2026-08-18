//! 排盘派生上下文
//!
//! 出生数据（公历日期、时辰、性别、闰月修正、配置）推不出星耀位置，
//! 中间要先落成一组量：生效时辰、农历年月日、两套年干支、月索引、命身宫、五行局。
//! 排盘主流程与 `star` 模块的各安星入口都从这一步出发，因此单独成型。

use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::lunar_month::{self, LunarMonthRefHelper};
use lunar_rust::{lunar, solar};

use crate::astro::builder::{branch_of, fix_lunar_month_index, stem_of};
use crate::astro::palace::{SoulAndBody, get_five_elements_class, get_soul_and_body};
use crate::data::types::*;
use crate::error::IztroError;

/// 由出生数据推出的、安星各步骤共用的中间量
pub struct AstroContext {
    /// 参与推算的时辰索引；`day_divide=current` 且为晚子时（>=12）时归零，展示仍用原始时辰
    pub effective_time_index: u8,
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
}

/// 由公历日期推出排盘上下文
///
/// `solar_date` 为 `YYYY-M-D`，`time_index` 为 0-12；两者的合法性在此校验。
pub fn derive(
    solar_date: &str,
    time_index: u8,
    fix_leap: bool,
    config: &Config,
) -> Result<AstroContext, IztroError> {
    crate::astro::builder::validate_time_index(time_index)?;
    let (year, month, day) = crate::astro::builder::parse_solar_date(solar_date)?;

    let effective_time_index = if config.day_divide == DayDivide::Current && time_index >= 12 {
        0
    } else {
        time_index
    };

    let hour = crate::astro::builder::time_index_to_hour(effective_time_index);
    let solar_with_time = solar::from_ymdhms(year, month, day, hour, 0, 0);
    let lunar_ref = lunar::from_solar(&solar_with_time);

    let lunar_month_raw = lunar_ref.get_month(); // 负值表示闰月
    let is_leap = lunar_month_raw < 0;
    let lunar_month = lunar_month_raw.unsigned_abs() as u32;
    let lunar_day = lunar_ref.get_day() as u32;

    let (yearly_stem_str, yearly_branch_str) = match config.year_divide {
        YearDivide::Normal => (lunar_ref.get_year_gan(), lunar_ref.get_year_zhi()),
        YearDivide::Exact => (
            lunar_ref.get_year_gan_by_li_chun(),
            lunar_ref.get_year_zhi_by_li_chun(),
        ),
    };
    let yearly_stem = stem_of(&yearly_stem_str, "birth year")?;
    let yearly_branch = branch_of(&yearly_branch_str, "birth year")?;

    let (flow_stem_str, flow_branch_str) = match config.horoscope_divide {
        HoroscopeDivide::Normal => (lunar_ref.get_year_gan(), lunar_ref.get_year_zhi()),
        HoroscopeDivide::Exact => (
            lunar_ref.get_year_gan_by_li_chun(),
            lunar_ref.get_year_zhi_by_li_chun(),
        ),
    };
    let flow_yearly_stem = stem_of(&flow_stem_str, "flow year")?;
    let flow_yearly_branch = branch_of(&flow_branch_str, "flow year")?;

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

    let month_day_count =
        lunar_month::from_ym(lunar_ref.get_year(), lunar_month_raw).get_day_count() as u32;

    Ok(AstroContext {
        effective_time_index,
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
    })
}

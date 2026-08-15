//! 按出生数据安星
//!
//! 同层的 `location` / `major` / `minor` / `adjective` / `decorative` 收的是
//! 已经算好的宫位索引，是排盘流水线内部的构件；本模块收出生数据本身，
//! 内部先派生上下文再调那些构件，与 iztro `star` 模块的对外函数一一对应。
//!
//! 单独取某一组星耀而不排整盘时用这一层。

use crate::astro::context::{self, AstroContext};
use crate::astro::palace::get_five_elements_class;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::models::star::Star;
use crate::star::decorative;
use crate::star::location::{
    self, ChangQu, DailyStar, KongJie, KuiYue, LuYangTuoMa, LuanXi, MonthlyStar, StartIndex,
    TimelyStars, YearlyStars,
};

/// 安星参数
///
/// 对应 iztro 的 `AstrolabeParam`，另收 iztro 挂在全局单例上的语言与配置。
pub struct StarParam<'a> {
    /// 公历日期，格式 `YYYY-M-D`
    pub solar_date: &'a str,
    /// 时辰索引 0-12（0 为早子时，12 为晚子时）
    pub time_index: u8,
    /// 性别，决定长生12神与博士12神的顺逆
    pub gender: Gender,
    /// 是否调整农历闰月（该月非闰月则不生效）
    pub fix_leap: bool,
    /// 起五行局的干支：中州派地盘、人盘从别的宫起局；天盘留 `None`，由命宫干支起
    pub from: Option<(HeavenlyStem, EarthlyBranch)>,
    /// 星耀名称的输出语言
    pub language: Language,
    /// 排盘配置
    pub config: &'a Config,
}

impl StarParam<'_> {
    fn derive(&self) -> Result<AstroContext, IztroError> {
        context::derive(self.solar_date, self.time_index, self.fix_leap, self.config)
    }

    /// 起五行局：传了 `from` 就用它，否则用命宫干支
    fn five_elements_class(&self, ctx: &AstroContext) -> FiveElementsClass {
        match self.from {
            Some((stem, branch)) => get_five_elements_class(stem, branch),
            None => ctx.five_elements_class,
        }
    }
}

/// 紫微、天府起始宫位索引
pub fn get_start_index(param: &StarParam) -> Result<StartIndex, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_start_index(
        ctx.lunar_day,
        ctx.effective_time_index,
        ctx.month_day_count,
        param.five_elements_class(&ctx).value() as u32,
    ))
}

/// 禄存、擎羊、陀罗、天马索引（按年干支）
pub fn get_lu_yang_tuo_ma_index(param: &StarParam) -> Result<LuYangTuoMa, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_lu_yang_tuo_ma_index(
        ctx.yearly_stem,
        ctx.yearly_branch,
    ))
}

/// 天魁、天钺索引（按年干）
pub fn get_kui_yue_index(param: &StarParam) -> Result<KuiYue, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_kui_yue_index(ctx.yearly_stem))
}

/// 文昌、文曲索引（按时支）
pub fn get_chang_qu_index(param: &StarParam) -> Result<ChangQu, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_chang_qu_index(ctx.effective_time_index))
}

/// 地空、地劫索引（按时支）
pub fn get_kong_jie_index(param: &StarParam) -> Result<KongJie, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_kong_jie_index(ctx.effective_time_index))
}

/// 台辅、封诰索引（按时支）
pub fn get_timely_star_index(param: &StarParam) -> Result<TimelyStars, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_timely_star_index(ctx.effective_time_index))
}

/// 红鸾、天喜索引（按年支）
pub fn get_luan_xi_index(param: &StarParam) -> Result<LuanXi, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_luan_xi_index(ctx.yearly_branch))
}

/// 日系星索引：三台、八座、恩光、天贵
pub fn get_daily_star_index(param: &StarParam) -> Result<DailyStar, IztroError> {
    let ctx = param.derive()?;
    let zuo_you = location::get_zuo_you_index(ctx.month_index as u32 + 1);
    let chang_qu = location::get_chang_qu_index(ctx.effective_time_index);

    Ok(location::get_daily_star_index(
        ctx.lunar_day,
        ctx.effective_time_index,
        zuo_you.zuo,
        zuo_you.you,
        chang_qu.chang,
        chang_qu.qu,
    ))
}

/// 月系星索引：解神、天姚、天刑、阴煞、天月、天巫
pub fn get_monthly_star_index(param: &StarParam) -> Result<MonthlyStar, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_monthly_star_index(ctx.month_index))
}

/// 年系星索引（一整组年支、年干起的杂耀）
///
/// 年支取 `horoscope_divide` 分界，与排盘主流程一致。
pub fn get_yearly_star_index(param: &StarParam) -> Result<YearlyStars, IztroError> {
    let ctx = param.derive()?;

    Ok(location::get_yearly_star_index(
        ctx.soul_body.soul_index,
        ctx.soul_body.body_index,
        ctx.flow_yearly_stem,
        ctx.flow_yearly_branch,
        param.gender,
        param.config.algorithm,
    ))
}

/// 十四主星在十二宫的分布
pub fn get_major_stars(param: &StarParam) -> Result<[Vec<Star>; 12], IztroError> {
    let ctx = param.derive()?;
    let start = get_start_index(param)?;

    Ok(crate::star::major::get_major_stars(
        start.ziwei,
        start.tianfu,
        ctx.yearly_stem,
        param.language,
        param.config,
    ))
}

/// 十四辅星在十二宫的分布
pub fn get_minor_stars(param: &StarParam) -> Result<[Vec<Star>; 12], IztroError> {
    let ctx = param.derive()?;
    let zuo_you = location::get_zuo_you_index(ctx.month_index as u32 + 1);
    let chang_qu = location::get_chang_qu_index(ctx.effective_time_index);
    let kui_yue = location::get_kui_yue_index(ctx.yearly_stem);
    let lu_yang_tuo_ma = location::get_lu_yang_tuo_ma_index(ctx.yearly_stem, ctx.yearly_branch);
    let kong_jie = location::get_kong_jie_index(ctx.effective_time_index);
    let huo_ling = location::get_huo_ling_index(ctx.yearly_branch, ctx.effective_time_index);

    Ok(crate::star::minor::get_minor_stars(
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
        param.language,
        param.config,
    ))
}

/// 杂耀在十二宫的分布
pub fn get_adjective_stars(param: &StarParam) -> Result<[Vec<Star>; 12], IztroError> {
    let ctx = param.derive()?;
    let yearly_stars = get_yearly_star_index(param)?;
    let monthly_stars = location::get_monthly_star_index(ctx.month_index);
    let daily_stars = get_daily_star_index(param)?;
    let timely_stars = location::get_timely_star_index(ctx.effective_time_index);
    // 红鸾天喜按安星年支，其余年系星按流年年支——与排盘主流程同规
    let luan_xi = location::get_luan_xi_index(ctx.yearly_branch);
    let (suiqian12, _) = decorative::get_yearly12(ctx.flow_yearly_branch, param.config.algorithm);

    Ok(crate::star::adjective::get_adjective_stars(
        &yearly_stars,
        &monthly_stars,
        &daily_stars,
        timely_stars.taifu,
        timely_stars.fenggao,
        luan_xi.hongluan,
        luan_xi.tianxi,
        yearly_stars.xianchi,
        &suiqian12,
        param.config.algorithm,
        param.language,
    ))
}

/// 长生12神从寅宫起的排列
pub fn get_changsheng12(param: &StarParam) -> Result<[StarKey; 12], IztroError> {
    let ctx = param.derive()?;

    Ok(decorative::get_changsheng12(
        param.five_elements_class(&ctx),
        param.gender,
        ctx.yearly_branch,
    ))
}

/// 博士12神从寅宫起的排列
pub fn get_boshi12(param: &StarParam) -> Result<[StarKey; 12], IztroError> {
    let ctx = param.derive()?;
    let lu_yang_tuo_ma = location::get_lu_yang_tuo_ma_index(ctx.yearly_stem, ctx.yearly_branch);

    Ok(decorative::get_boshi12(
        lu_yang_tuo_ma.lu,
        param.gender,
        ctx.yearly_branch,
    ))
}

/// 岁前12神与将前12神从寅宫起的排列
///
/// 流年神煞按 `horoscope_divide` 分界取年支。
pub fn get_yearly12(param: &StarParam) -> Result<([StarKey; 12], [StarKey; 12]), IztroError> {
    let ctx = param.derive()?;

    Ok(decorative::get_yearly12(
        ctx.flow_yearly_branch,
        param.config.algorithm,
    ))
}

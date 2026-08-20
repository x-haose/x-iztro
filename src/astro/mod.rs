// astro 模块
/// 排盘主流程
pub mod builder;
/// 出生数据到安星中间量的派生
pub mod context;
/// 运限计算
pub mod horoscope;
/// 农历月表读取口（对 lunar_rust 月界数据缺陷的修正视图）
pub(crate) mod lunar_table;
/// 命身宫、五行局与大限小限计算
pub mod palace;
/// 生肖、星座、命宫主星等轻量查询
pub mod query;
/// 反推：由八字或星盘特征反查候选生辰
pub mod reverse;

/// 按指定干支重排星盘（中州派地盘、人盘）
pub mod rearrange;
/// 三方四正（结构定义在 [`crate::models::surpalaces`]，此处转发以保持路径稳定）
pub mod surpalaces {
    pub use crate::models::surpalaces::*;
}

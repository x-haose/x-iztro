//! 全部格局规则。每条一个函数，签名 `fn(&ChartView) -> Vec<PatternHit>`，
//! 文档注释写形式化条件与口径来源（iztro-docs《格局》页，引文为《紫微斗数全书》）。
//!
//! 分组：`ziwei` 紫微天府天机系、`sun_moon` 日月系、`major` 武廉贪巨梁杀破系、
//! `lu` 禄与四化系（含两条行运格）、`assist` 辅弼昌曲魁钺羊陀空劫系。

mod assist;
mod lu;
mod major;
mod sun_moon;
mod ziwei;

use super::Rule;

/// 全部规则：五个分组各自维护自己的 `RULES` 切片，这里按《格局》页顺序拼接。
pub fn all() -> impl Iterator<Item = &'static Rule> {
    ziwei::RULES
        .iter()
        .chain(sun_moon::RULES)
        .chain(major::RULES)
        .chain(lu::RULES)
        .chain(assist::RULES)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::ALL_PATTERNS;

    #[test]
    fn rules_cover_every_pattern_key_once() {
        let mut keys: Vec<_> = all().map(|r| r.key).collect();
        let n = keys.len();
        keys.sort_by_key(|k| k.as_key());
        keys.dedup();
        assert_eq!(keys.len(), n, "duplicate rule");
        let missing: Vec<_> = ALL_PATTERNS
            .iter()
            .filter(|p| !all().any(|r| r.key == **p))
            .map(|p| p.as_key())
            .collect();
        assert!(missing.is_empty(), "patterns without rule: {missing:?}");
    }
}

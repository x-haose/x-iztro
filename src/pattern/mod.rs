//! 格局判定引擎。
//!
//! 规则来自 iztro-docs《格局》页（63 条，火贪/铃贪分列为 64 个 [`PatternKey`]），
//! 全部写在 [`rules`] 模块，每条函数注明口径与来源。
//!
//! 本命盘与运限盘共用同一套规则：[`ChartView`] 把「本命十二宫」或「某运限层的合成十二宫
//! （本命星 + 该层流曜 + 该层四化，以运限命宫为命宫）」抽象成同一种视图，
//! 规则只面对视图。因此《格局》页前言所说「本命有此组合，大限又走到即享其益」
//! 自然成立 —— 大限视角下重跑本命规则即可。

pub mod keys;
pub mod rules;
pub mod view;

use serde::{Deserialize, Serialize};

use crate::data::stars::StarKey;
use crate::data::types::{Brightness, Mutagen, Scope};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::{HoroscopeData, HoroscopeRef};

pub use keys::{ALL_PATTERNS, PatternKey};
pub use view::ChartView;

/// 日月亮度的判定依据。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BrightnessSource {
    /// 按星盘亮度表（与 iztro 逐值一致）：庙旺为明，陷与「不」为暗。
    #[default]
    Table,
    /// 按传统位置：太阳寅至午为明、酉至丑为暗；太阴酉至丑为明、卯至未为暗。
    /// 用于与页面示例一致的传统口径 —— iztro 亮度表太阴酉为「不」，按表判《日月并明》示例不成格。
    Positional,
}

/// 格局判定的口径开关。字段极少：多口径的格局一律以 [`PatternHit::variant`] 报出，
/// 这里只放会改变「事实判定」本身的数据口径。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct PatternConfig {
    /// 日月亮度依据。
    pub brightness_source: BrightnessSource,
    /// 空宫是否借对宫主星参与判定。
    pub borrow: bool,
    /// 运限视角下流曜（运禄/流禄、运马/流马、运昌/流昌…）是否等同于对应本命辅星参与判定。
    pub flow_stars: bool,
}

impl Default for PatternConfig {
    fn default() -> Self {
        PatternConfig {
            brightness_source: BrightnessSource::Table,
            borrow: true,
            flow_stars: true,
        }
    }
}

/// 一颗参与成格的星及其落宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarAt {
    /// 星耀
    pub star: StarKey,
    /// 落宫索引（0-11，寅宫为 0）
    pub palace: usize,
    /// 亮度（有亮度表的星）
    pub brightness: Option<Brightness>,
    /// 判定视角下的四化（本命为生年四化，运限为该层四化）
    pub mutagen: Option<Mutagen>,
}

/// 一次格局命中。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PatternHit {
    /// 格局
    pub key: PatternKey,
    /// 判定视角（本命或运限层）
    pub scope: Scope,
    /// 成格所在的宫位索引（0-11，寅宫为 0）：多数为命宫，「身命」类可为身宫，任一宫成格的为实际落宫
    pub palace: usize,
    /// 多口径格局命中的是哪个口径（如君臣庆会四形式、月生沧海命宫/田宅）；单口径为 `None`
    pub variant: Option<&'static str>,
    /// 页面称「破格 / 加杀平常」的条件是否触发：成格照报，仅作标记
    pub broken: bool,
    /// 参与成格的星与落宫
    pub stars: Vec<StarAt>,
}

/// 单条规则：`detect` 在视图上返回全部命中（多数 0 或 1 个）。
pub struct Rule {
    /// 格局
    pub key: PatternKey,
    /// 判定函数
    pub detect: fn(&ChartView) -> Vec<PatternHit>,
}

/// 在视图上跑全部规则：本命视图跳过行运格。
pub fn detect(view: &ChartView) -> Vec<PatternHit> {
    rules::all()
        .filter(|r| view.scope() != Scope::Origin || !r.key.is_horoscope_only())
        .flat_map(|r| (r.detect)(view))
        .collect()
}

impl Astrolabe {
    /// 本命盘的全部格局命中（默认口径）。
    pub fn patterns(&self) -> Vec<PatternHit> {
        self.patterns_with(&PatternConfig::default())
    }

    /// 本命盘的全部格局命中，指定口径。
    pub fn patterns_with(&self, config: &PatternConfig) -> Vec<PatternHit> {
        detect(&ChartView::natal(self, config))
    }
}

impl<'a> HoroscopeRef<'a> {
    /// 指定运限层视角的格局命中（默认口径）：`Scope::Origin` 等同本命。
    pub fn patterns(&self, scope: Scope) -> Vec<PatternHit> {
        self.patterns_with(scope, &PatternConfig::default())
    }

    /// 指定运限层视角的格局命中，指定口径。
    pub fn patterns_with(&self, scope: Scope, config: &PatternConfig) -> Vec<PatternHit> {
        patterns_at(self.astrolabe(), self.data(), scope, config)
    }
}

/// 在某运限层视角上判定格局：`Scope::Origin` 等同本命。
pub fn patterns_at(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    scope: Scope,
    config: &PatternConfig,
) -> Vec<PatternHit> {
    detect(&ChartView::at(astrolabe, horoscope, scope, config))
}

#[cfg(test)]
pub(crate) mod testutil {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::types::{Config, Gender, Language};

    /// 在 1950-2020 年逐日逐时辰（男女各一）搜第一张满足条件的盘；搜不到即 panic。
    pub(crate) fn find_chart(pred: impl Fn(&Astrolabe) -> bool) -> Astrolabe {
        find_charts(1, pred).pop().unwrap()
    }

    /// 同 [`find_chart`]，最多收集 `n` 张（用于分布类断言）。
    pub(crate) fn find_charts(n: usize, pred: impl Fn(&Astrolabe) -> bool) -> Vec<Astrolabe> {
        let mut out = Vec::new();
        for y in 1950..2021 {
            for m in 1..=12 {
                for d in 1..=28 {
                    for h in 0..12u8 {
                        for g in [Gender::Male, Gender::Female] {
                            let a = by_solar(
                                &format!("{y}-{m}-{d}"),
                                h,
                                g,
                                true,
                                Language::ZhCN,
                                Config::default(),
                            )
                            .unwrap();
                            if pred(&a) {
                                out.push(a);
                                if out.len() == n {
                                    return out;
                                }
                            }
                        }
                    }
                }
            }
        }
        assert!(!out.is_empty(), "no chart found");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::types::{Config, Gender, Language, Palace};

    use super::testutil::find_chart;

    fn soul(a: &Astrolabe) -> &crate::models::palace::PalaceData {
        a.palace(Palace::Soul).unwrap().data()
    }

    #[test]
    fn zi_fu_tong_gong_hits_only_when_both_in_soul() {
        let a = find_chart(|a| soul(a).has(&[StarKey::ZiweiMaj, StarKey::TianfuMaj]));
        let hits = a.patterns();
        let hit = hits
            .iter()
            .find(|h| h.key == PatternKey::ZiFuTongGong)
            .expect("hit");
        assert_eq!(hit.palace, soul(&a).index);
        assert_eq!(hit.scope, Scope::Origin);
        assert_eq!(hit.stars.len(), 2);
        assert!(matches!(
            soul(&a).earthly_branch,
            crate::data::types::EarthlyBranch::Yin | crate::data::types::EarthlyBranch::Shen
        ));

        let b = find_chart(|a| {
            soul(a).has(&[StarKey::ZiweiMaj]) && !soul(a).has(&[StarKey::TianfuMaj])
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::ZiFuTongGong)
        );
    }

    #[test]
    fn zuo_you_jia_ming_requires_both_neighbours() {
        let a = find_chart(|a| {
            let s = soul(a).index;
            let p = &a.palaces[(s + 11) % 12];
            let n = &a.palaces[(s + 1) % 12];
            (p.has(&[StarKey::ZuofuMin]) && n.has(&[StarKey::YoubiMin]))
                || (p.has(&[StarKey::YoubiMin]) && n.has(&[StarKey::ZuofuMin]))
        });
        let hits = a.patterns();
        let hit = hits
            .iter()
            .find(|h| h.key == PatternKey::ZuoYouJiaMing)
            .expect("hit");
        let palaces: Vec<_> = hit.stars.iter().map(|s| s.palace).collect();
        assert!(
            palaces.contains(&((hit.palace + 11) % 12))
                && palaces.contains(&((hit.palace + 1) % 12))
        );
    }

    #[test]
    fn ming_lu_an_lu_uses_hidden_palace() {
        let a = find_chart(|a| {
            let v = ChartView::natal(a, &PatternConfig::default());
            let s = v.soul();
            let h = v.hidden(s);
            v.has(s, StarKey::LucunMin) && v.has_mutagen(h, Mutagen::Lu)
        });
        assert!(a.patterns().iter().any(|h| h.key == PatternKey::MingLuAnLu));
    }

    #[test]
    fn hidden_palace_is_six_harmony() {
        let a = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let v = ChartView::natal(&a, &PatternConfig::default());
        use crate::data::types::EarthlyBranch::*;
        for (i, want) in (0..12).map(|i| (i, v.branch(i))).map(|(i, b)| {
            let w = match b {
                Zi => Chou,
                Chou => Zi,
                Yin => Hai,
                Hai => Yin,
                Mao => Xu,
                Xu => Mao,
                Chen => You,
                You => Chen,
                Si => Shen,
                Shen => Si,
                Wu => Wei,
                Wei => Wu,
            };
            (i, w)
        }) {
            assert_eq!(v.branch(v.hidden(i)), want);
        }
    }

    #[test]
    fn horoscope_view_uses_scope_soul_and_mutagen() {
        let a = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let h = a.horoscope("2026-8-19", 3).unwrap();
        let cfg = PatternConfig::default();
        let v = ChartView::at(&a, h.data(), Scope::Decadal, &cfg);
        assert_eq!(v.soul(), h.decadal.index);
        assert_eq!(v.body(), None);
        assert_eq!(v.name_of(v.soul()), Palace::Soul);
        // 该层化禄星在视图里读出的四化就是禄
        let lu = h.decadal.mutagen[0];
        let (p, star) = v.locate(lu).expect("mutagen star on chart");
        assert_eq!(v.mutagen_of(star), Some(Mutagen::Lu));
        assert!(v.has_mutagen(p, Mutagen::Lu));
        // 运限视角与本命视角各自可跑，且 Origin 等同本命
        let _ = h.patterns(Scope::Decadal);
        let _ = h.patterns(Scope::Yearly);
        assert_eq!(h.patterns(Scope::Origin), a.patterns());
    }

    #[test]
    fn positional_brightness_flips_sun_moon_judgement() {
        // 太阴在酉：表判「不」，位置法判明
        let a = find_chart(|a| {
            a.palaces.iter().any(|p| {
                p.earthly_branch == crate::data::types::EarthlyBranch::You
                    && p.has(&[StarKey::TaiyinMaj])
            })
        });
        let table = ChartView::natal(&a, &PatternConfig::default());
        let pos = ChartView::natal(
            &a,
            &PatternConfig {
                brightness_source: BrightnessSource::Positional,
                ..Default::default()
            },
        );
        let you = (0..12)
            .find(|i| table.branch(*i) == crate::data::types::EarthlyBranch::You)
            .unwrap();
        assert!(!table.sun_moon_bright(you, StarKey::TaiyinMaj));
        assert!(pos.sun_moon_bright(you, StarKey::TaiyinMaj));
    }
}

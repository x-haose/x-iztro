//! 日月系格局（《格局》页 #11-#24）。
//!
//! 太阳、太阴的明暗一律走 [`ChartView::sun_moon_bright`] / [`ChartView::sun_moon_dark`]：
//! 默认按 iztro 亮度表（庙旺为明，陷与「不」为暗），
//! [`crate::pattern::BrightnessSource::Positional`] 下按传统位置。

use crate::data::stars::StarKey;
use crate::data::types::{EarthlyBranch, Mutagen, Palace};
use crate::models::star::Star;
use crate::pattern::view::{JI6, KONG_WANG};
use crate::pattern::{ChartView, PatternHit, PatternKey, Rule};

/// 本组规则，按《格局》页顺序。
pub static RULES: &[Rule] = &[
    Rule {
        key: PatternKey::RiYueTongGong,
        detect: ri_yue_tong_gong,
    },
    Rule {
        key: PatternKey::JuRiTongGong,
        detect: ju_ri_tong_gong,
    },
    Rule {
        key: PatternKey::RiZhaoLeiMen,
        detect: ri_zhao_lei_men,
    },
    Rule {
        key: PatternKey::RiYueBingMing,
        detect: ri_yue_bing_ming,
    },
    Rule {
        key: PatternKey::RiYueFanBei,
        detect: ri_yue_fan_bei,
    },
    Rule {
        key: PatternKey::RiYueZhaoBi,
        detect: ri_yue_zhao_bi,
    },
    Rule {
        key: PatternKey::JinCanGuangHui,
        detect: jin_can_guang_hui,
    },
    Rule {
        key: PatternKey::RiYueCangHui,
        detect: ri_yue_cang_hui,
    },
    Rule {
        key: PatternKey::DanChiGuiChi,
        detect: dan_chi_gui_chi,
    },
    Rule {
        key: PatternKey::RiYueJiaMing,
        detect: ri_yue_jia_ming,
    },
    Rule {
        key: PatternKey::RiYueJiaCai,
        detect: ri_yue_jia_cai,
    },
    Rule {
        key: PatternKey::YueLangTianMen,
        detect: yue_lang_tian_men,
    },
    Rule {
        key: PatternKey::YueShengCangHai,
        detect: yue_sheng_cang_hai,
    },
    Rule {
        key: PatternKey::MingZhuChuHai,
        detect: ming_zhu_chu_hai,
    },
];

/// 取命宫三方四正里的太阳与太阴，两颗都满足指定明暗时成对返回 `[(宫, 太阳), (宫, 太阴)]`。
/// `bright` 为真取「皆明」（日月并明），为假取「皆暗」（日月反背）。
fn sun_and_moon<'a>(v: &ChartView<'a>, s: usize, bright: bool) -> Option<[(usize, &'a Star); 2]> {
    let ok = |i: usize, k: StarKey| {
        if bright {
            v.sun_moon_bright(i, k)
        } else {
            v.sun_moon_dark(i, k)
        }
    };
    let sun = v
        .find_in_surround(s, StarKey::TaiyangMaj)
        .filter(|(p, _)| ok(*p, StarKey::TaiyangMaj))?;
    let moon = v
        .find_in_surround(s, StarKey::TaiyinMaj)
        .filter(|(p, _)| ok(*p, StarKey::TaiyinMaj))?;
    Some([sun, moon])
}

/// 宫内的吉星：六吉星或带禄权科的星，取第一颗。
/// 「吉星」的成员按《格局》页善荫朝纲一节的列举（辅弼昌曲魁钺、化禄化权化科）。
fn auspicious<'a>(v: &ChartView<'a>, i: usize) -> Option<&'a Star> {
    v.stars(i).find(|s| {
        JI6.contains(&s.key)
            || matches!(
                v.mutagen_of(s),
                Some(Mutagen::Lu | Mutagen::Quan | Mutagen::Ke)
            )
    })
}

/// 日月夹某宫的共同判定：太阳、太阴分居前后邻宫，该宫不坐空亡星且宫内有吉星。
fn ri_yue_jia(v: &ChartView, key: PatternKey, i: usize) -> Vec<PatternHit> {
    let Some((ps, pm)) = v.jia(i, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj]) else {
        return vec![];
    };
    if v.has_any(i, &KONG_WANG) {
        return vec![];
    }
    let (Some(sun), Some(moon), Some(ji)) = (
        v.find(ps, StarKey::TaiyangMaj),
        v.find(pm, StarKey::TaiyinMaj),
        auspicious(v, i),
    ) else {
        return vec![];
    };
    vec![v.hit(key, i, vec![(ps, sun), (pm, moon), (i, ji)])]
}

/// 日月同宫：太阳、太阴同守命宫（只可能在丑、未）。
///
/// 来源：「日月同临 官居侯伯」。
pub fn ri_yue_tong_gong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match (
        v.find(s, StarKey::TaiyangMaj),
        v.find(s, StarKey::TaiyinMaj),
    ) {
        (Some(sun), Some(moon)) => {
            vec![v.hit(PatternKey::RiYueTongGong, s, vec![(s, sun), (s, moon)])]
        }
        _ => vec![],
    }
}

/// 巨日同宫：太阳、巨门同守命宫（寅、申皆算）。
///
/// 来源：「巨日同宫 官封三代」「巨日命立申宫亦妙」。
pub fn ju_ri_tong_gong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match (v.find(s, StarKey::TaiyangMaj), v.find(s, StarKey::JumenMaj)) {
        (Some(sun), Some(ju)) => {
            vec![v.hit(PatternKey::JuRiTongGong, s, vec![(s, sun), (s, ju)])]
        }
        _ => vec![],
    }
}

/// 日照雷门：太阳、天梁同守命宫且命宫在卯（酉宫的阳梁是日落西山，不算）。
///
/// 来源：「日照雷门 富贵荣华」「日出扶桑 日在卯守命是也」。
/// 口径：只取命宫；古书「守官禄宫亦然」与「昼生」皆不作条件。
pub fn ri_zhao_lei_men(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if v.branch(s) != EarthlyBranch::Mao {
        return vec![];
    }
    match (
        v.find(s, StarKey::TaiyangMaj),
        v.find(s, StarKey::TianliangMaj),
    ) {
        (Some(sun), Some(liang)) => {
            vec![v.hit(PatternKey::RiZhaoLeiMen, s, vec![(s, sun), (s, liang)])]
        }
        _ => vec![],
    }
}

/// 日月并明：命宫三方四正内太阳、太阴皆明。
///
/// 来源：「日月并明 佐九重于尧殿」；作者：「任何太阳星和太阴星庙旺出现在命宫三方四正的，
/// 都应该算日月并明格」。页面两张示例的太阴在酉，iztro 亮度表为「不」，默认口径下不成格。
pub fn ri_yue_bing_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match sun_and_moon(v, s, true) {
        Some(pair) => vec![v.hit(PatternKey::RiYueBingMing, s, pair.to_vec())],
        None => vec![],
    }
}

/// 日月反背：命宫三方四正内太阳、太阴皆暗。
///
/// 来源：「日月最嫌反背」；作者：「只要太阳星和太阴星在命宫三方四正落陷」。
/// 「暗」含亮度「不」——页面石中隐玉一节称午宫巨门为日月反背，其太阳在戌正是「不」。
pub fn ri_yue_fan_bei(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match sun_and_moon(v, s, false) {
        Some(pair) => vec![v.hit(PatternKey::RiYueFanBei, s, pair.to_vec())],
        None => vec![],
    }
}

/// 日月照璧：太阳、太阴同守田宅宫（日月同宫只在丑、未，四墓库之说自然满足）。
///
/// 来源：「日月照璧 日月临田宅宫是也，喜居墓库」。
pub fn ri_yue_zhao_bi(v: &ChartView) -> Vec<PatternHit> {
    let p = v.index_of(Palace::Property);
    match (
        v.find(p, StarKey::TaiyangMaj),
        v.find(p, StarKey::TaiyinMaj),
    ) {
        (Some(sun), Some(moon)) => {
            vec![v.hit(PatternKey::RiYueZhaoBi, p, vec![(p, sun), (p, moon)])]
        }
        _ => vec![],
    }
}

/// 金灿光辉：太阳独坐命宫且命宫在午。
///
/// 来源：「金灿光辉 太阳单守，命在午宫是也」「太阳居午，谓之日丽中天」。
/// 命宫三方四正见煞忌时作者称「破格」，仍报此格，只置 [`PatternHit::broken`]。
pub fn jin_can_guang_hui(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if v.branch(s) != EarthlyBranch::Wu || !v.alone(s, StarKey::TaiyangMaj) {
        return vec![];
    }
    let Some(sun) = v.find(s, StarKey::TaiyangMaj) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::JinCanGuangHui, s, vec![(s, sun)]);
    hit.broken = !v.no_sha(s);
    vec![hit]
}

/// 日月藏辉：日月反背，且命宫三方四正又见巨门。
///
/// 来源：「日月藏辉 日月反背又逢巨暗是也」。作者提到的昼夜生第二口径全书无载，不采。
pub fn ri_yue_cang_hui(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(pair), Some(ju)) = (
        sun_and_moon(v, s, false),
        v.find_in_surround(s, StarKey::JumenMaj),
    ) else {
        return vec![];
    };
    let mut stars = pair.to_vec();
    stars.push(ju);
    vec![v.hit(PatternKey::RiYueCangHui, s, stars)]
}

/// 丹墀桂墀：日月并明，且命宫本身坐着明的太阳（丹墀）或明的太阴（桂墀）。
///
/// 来源：「丹墀桂墀 早遂青云之志」；作者定义为日月并明格中命宫是太阳或太阴的细分，
/// 故明暗集合与日月并明一致（不另收窄到「庙」）。
pub fn dan_chi_gui_chi(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let in_soul =
        v.sun_moon_bright(s, StarKey::TaiyangMaj) || v.sun_moon_bright(s, StarKey::TaiyinMaj);
    match sun_and_moon(v, s, true) {
        Some(pair) if in_soul => vec![v.hit(PatternKey::DanChiGuiChi, s, pair.to_vec())],
        _ => vec![],
    }
}

/// 日月夹命：太阳、太阴分居命宫前后邻宫，命宫不坐空亡星且有吉星。
///
/// 来源：「日月夹命 不坐空亡遇逢本宫有吉星是也」。夹只发生在丑、未。
pub fn ri_yue_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    ri_yue_jia(v, PatternKey::RiYueJiaMing, v.soul())
}

/// 日月夹财：条件同日月夹命，把命宫换成财帛宫。
///
/// 来源：「日月夹财，不权则富」；作者：「星耀分布和日月夹命一样，只是把命宫换成财帛宫」，
/// 故空亡与吉星条件一并移到财帛宫。古书「武守命日月来夹」那一支已由日月夹命覆盖。
pub fn ri_yue_jia_cai(v: &ChartView) -> Vec<PatternHit> {
    ri_yue_jia(v, PatternKey::RiYueJiaCai, v.index_of(Palace::Wealth))
}

/// 月朗天门：太阴守命宫且命宫在亥。
///
/// 来源：「月落亥宫 月在亥守命是也，又名月朗天门」。古书「子生人夜时生」不作条件。
pub fn yue_lang_tian_men(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match v.find(s, StarKey::TaiyinMaj) {
        Some(moon) if v.branch(s) == EarthlyBranch::Hai => {
            vec![v.hit(PatternKey::YueLangTianMen, s, vec![(s, moon)])]
        }
        _ => vec![],
    }
}

/// 月生沧海：天同、太阴同坐子宫，落在田宅宫（`variant = "property"`，全书原文）
/// 或落在命宫（`variant = "soul"`，页面别称「水澄桂萼」）。
///
/// 来源：「月生沧海 月在子宫守田宅是也」「太阴居子，号曰水澄桂萼」。
pub fn yue_sheng_cang_hai(v: &ChartView) -> Vec<PatternHit> {
    [
        (v.soul(), "soul"),
        (v.index_of(Palace::Property), "property"),
    ]
    .into_iter()
    .filter(|(i, _)| v.branch(*i) == EarthlyBranch::Zi)
    .filter_map(|(i, variant)| {
        let (moon, tong) = (
            v.find(i, StarKey::TaiyinMaj)?,
            v.find(i, StarKey::TiantongMaj)?,
        );
        let mut hit = v.hit(PatternKey::YueShengCangHai, i, vec![(i, moon), (i, tong)]);
        hit.variant = Some(variant);
        Some(hit)
    })
    .collect()
}

/// 明珠出海：命宫空宫在未，对宫（迁移丑）坐天同、巨门。
/// 此格局下财帛宫必在卯坐太阳、官禄宫必在亥坐太阴，两颗一并记入证据。
///
/// 来源：「明珠出海 命宫在未宫，太阳星坐卯宫，太阴星坐亥宫」。
pub fn ming_zhu_chu_hai(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if v.branch(s) != EarthlyBranch::Wei || !v.is_empty(s) {
        return vec![];
    }
    let o = v.opposite(s);
    let (Some(tong), Some(ju)) = (
        v.find(o, StarKey::TiantongMaj),
        v.find(o, StarKey::JumenMaj),
    ) else {
        return vec![];
    };
    let mut stars = vec![(o, tong), (o, ju)];
    stars.extend(v.find_in_surround(s, StarKey::TaiyangMaj));
    stars.extend(v.find_in_surround(s, StarKey::TaiyinMaj));
    vec![v.hit(PatternKey::MingZhuChuHai, s, stars)]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::Brightness;
    use crate::data::types::EarthlyBranch::*;
    use crate::models::astrolabe::Astrolabe;
    use crate::pattern::testutil::find_chart;
    use crate::pattern::{BrightnessSource, PatternConfig};

    /// 测试统一用默认口径（iztro 亮度表、借宫开、流曜开）。
    const CFG: PatternConfig = PatternConfig {
        brightness_source: BrightnessSource::Table,
        borrow: true,
        flow_stars: true,
    };

    fn view(a: &Astrolabe) -> ChartView<'_> {
        ChartView::natal(a, &CFG)
    }

    /// 三方四正里该星是否处于庙旺：不经 `sun_moon_bright`，供正例搜盘时独立判定。
    fn miao_wang(v: &ChartView, s: usize, key: StarKey) -> bool {
        v.find_in_surround(s, key)
            .is_some_and(|(_, star)| star.with_brightness(&[Brightness::Miao, Brightness::Wang]))
    }

    /// 三方四正里该星是否落陷（含「不」）。
    fn xian(v: &ChartView, s: usize, key: StarKey) -> bool {
        v.find_in_surround(s, key)
            .is_some_and(|(_, star)| star.with_brightness(&[Brightness::Xian, Brightness::Bu]))
    }

    #[test]
    fn ri_yue_tong_gong_needs_both_in_soul() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let hits = ri_yue_tong_gong(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert!(matches!(v.branch(v.soul()), Chou | Wei));
        assert_eq!(hit.stars.len(), 2);
        assert!(hit.stars.iter().all(|s| s.palace == v.soul()));

        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.has(s, StarKey::TaiyangMaj) && !v.has(s, StarKey::TaiyinMaj)
        });
        assert!(ri_yue_tong_gong(&view(&b)).is_empty());
    }

    #[test]
    fn ju_ri_tong_gong_needs_sun_and_jumen_in_soul() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::JumenMaj)
        });
        let v = view(&a);
        let hits = ju_ri_tong_gong(&v);
        assert_eq!(hits.first().expect("hit").palace, v.soul());
        assert!(matches!(v.branch(v.soul()), Yin | Shen));

        // 反例：太阳与天梁同宫，非巨门
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::TianliangMaj)
        });
        assert!(ju_ri_tong_gong(&view(&b)).is_empty());
    }

    #[test]
    fn ri_zhao_lei_men_only_at_mao() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Mao && v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::TianliangMaj)
        });
        let v = view(&a);
        let hits = ri_zhao_lei_men(&v);
        assert_eq!(hits.first().expect("hit").palace, v.soul());

        // 反例：同为阳梁守命，但在酉宫（日落西山）
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == You && v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::TianliangMaj)
        });
        assert!(ri_zhao_lei_men(&view(&b)).is_empty());
    }

    #[test]
    fn ri_yue_bing_ming_needs_both_bright_in_surround() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            miao_wang(&v, s, StarKey::TaiyangMaj) && miao_wang(&v, s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let hits = ri_yue_bing_ming(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert!(
            hit.stars
                .iter()
                .all(|s| v.surround(v.soul()).contains(&s.palace))
        );

        // 反例：太阳庙旺但太阴不庙旺
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            miao_wang(&v, s, StarKey::TaiyangMaj) && !miao_wang(&v, s, StarKey::TaiyinMaj)
        });
        assert!(ri_yue_bing_ming(&view(&b)).is_empty());
    }

    /// 命宫三方四正的太阴在酉，iztro 亮度表判「不」故默认不成格；
    /// 位置法（酉至丑为月明）下同一张盘成格。
    #[test]
    fn ri_yue_bing_ming_moon_at_you_differs_by_brightness_source() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            let Some((pm, _)) = v.find_in_surround(s, StarKey::TaiyinMaj) else {
                return false;
            };
            v.branch(pm) == You && miao_wang(&v, s, StarKey::TaiyangMaj)
        });
        assert!(ri_yue_bing_ming(&view(&a)).is_empty());

        let positional = PatternConfig {
            brightness_source: BrightnessSource::Positional,
            ..CFG
        };
        let v = ChartView::natal(&a, &positional);
        let hits = ri_yue_bing_ming(&v);
        let hit = hits.first().expect("hit under positional brightness");
        let moon = hit
            .stars
            .iter()
            .find(|s| s.star == StarKey::TaiyinMaj)
            .expect("moon");
        assert_eq!(v.branch(moon.palace), You);
    }

    #[test]
    fn ri_yue_fan_bei_needs_both_dark_in_surround() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            xian(&v, s, StarKey::TaiyangMaj) && xian(&v, s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        assert_eq!(ri_yue_fan_bei(&v).first().expect("hit").palace, v.soul());

        // 反例：太阳落陷但太阴不落陷
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            xian(&v, s, StarKey::TaiyangMaj) && !xian(&v, s, StarKey::TaiyinMaj)
        });
        assert!(ri_yue_fan_bei(&view(&b)).is_empty());
    }

    #[test]
    fn ri_yue_zhao_bi_reports_property_palace() {
        let a = find_chart(|a| {
            let v = view(a);
            let p = v.index_of(Palace::Property);
            v.has(p, StarKey::TaiyangMaj) && v.has(p, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let hits = ri_yue_zhao_bi(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.index_of(Palace::Property));
        assert_ne!(hit.palace, v.soul());

        // 反例：日月同宫但落在命宫
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.has(s, StarKey::TaiyangMaj) && v.has(s, StarKey::TaiyinMaj)
        });
        assert!(ri_yue_zhao_bi(&view(&b)).is_empty());
    }

    #[test]
    fn jin_can_guang_hui_marks_broken_by_sha() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Wu && v.alone(s, StarKey::TaiyangMaj) && !v.no_sha(s)
        });
        let v = view(&a);
        let hits = jin_can_guang_hui(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert!(hit.broken);

        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Wu && v.alone(s, StarKey::TaiyangMaj) && v.no_sha(s)
        });
        assert!(!jin_can_guang_hui(&view(&b)).first().expect("hit").broken);

        // 反例：太阳独坐但不在午宫
        let c = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Chen && v.alone(s, StarKey::TaiyangMaj)
        });
        assert!(jin_can_guang_hui(&view(&c)).is_empty());
    }

    #[test]
    fn ri_yue_cang_hui_needs_fan_bei_plus_jumen() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            xian(&v, s, StarKey::TaiyangMaj)
                && xian(&v, s, StarKey::TaiyinMaj)
                && v.in_surround(s, StarKey::JumenMaj)
        });
        let v = view(&a);
        let hits = ri_yue_cang_hui(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.stars.len(), 3);
        assert!(hit.stars.iter().any(|s| s.star == StarKey::JumenMaj));

        // 反例：日月反背但三方四正不见巨门
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            xian(&v, s, StarKey::TaiyangMaj)
                && xian(&v, s, StarKey::TaiyinMaj)
                && !v.in_surround(s, StarKey::JumenMaj)
        });
        assert!(ri_yue_cang_hui(&view(&b)).is_empty());
    }

    #[test]
    fn dan_chi_gui_chi_needs_bright_sun_or_moon_in_soul() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            miao_wang(&v, s, StarKey::TaiyangMaj)
                && miao_wang(&v, s, StarKey::TaiyinMaj)
                && (v.has(s, StarKey::TaiyangMaj) || v.has(s, StarKey::TaiyinMaj))
        });
        let v = view(&a);
        assert_eq!(dan_chi_gui_chi(&v).first().expect("hit").palace, v.soul());

        // 反例：日月并明成立，但命宫不坐日月
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            miao_wang(&v, s, StarKey::TaiyangMaj)
                && miao_wang(&v, s, StarKey::TaiyinMaj)
                && !v.has(s, StarKey::TaiyangMaj)
                && !v.has(s, StarKey::TaiyinMaj)
        });
        assert!(!ri_yue_bing_ming(&view(&b)).is_empty());
        assert!(dan_chi_gui_chi(&view(&b)).is_empty());
    }

    #[test]
    fn ri_yue_jia_ming_needs_no_kongwang_and_an_auspicious_star() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.jia(s, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                .is_some()
                && !v.has_any(s, &KONG_WANG)
                && auspicious(&v, s).is_some()
        });
        let v = view(&a);
        let hits = ri_yue_jia_ming(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        let palaces: Vec<_> = hit.stars.iter().map(|s| s.palace).collect();
        assert!(palaces.contains(&v.prev(v.soul())) && palaces.contains(&v.next(v.soul())));

        // 反例：日月夹命成立但命宫坐空亡星
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.jia(s, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                .is_some()
                && v.has_any(s, &KONG_WANG)
        });
        assert!(ri_yue_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn ri_yue_jia_cai_reports_wealth_palace() {
        let a = find_chart(|a| {
            let v = view(a);
            let w = v.index_of(Palace::Wealth);
            v.jia(w, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                .is_some()
                && !v.has_any(w, &KONG_WANG)
                && auspicious(&v, w).is_some()
        });
        let v = view(&a);
        let hits = ri_yue_jia_cai(&v);
        assert_eq!(
            hits.first().expect("hit").palace,
            v.index_of(Palace::Wealth)
        );

        // 反例：财帛宫被日月夹但坐空亡星
        let b = find_chart(|a| {
            let v = view(a);
            let w = v.index_of(Palace::Wealth);
            v.jia(w, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                .is_some()
                && v.has_any(w, &KONG_WANG)
        });
        assert!(ri_yue_jia_cai(&view(&b)).is_empty());
    }

    #[test]
    fn yue_lang_tian_men_only_at_hai() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Hai && v.has(s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let hits = yue_lang_tian_men(&v);
        assert_eq!(hits.first().expect("hit").stars[0].star, StarKey::TaiyinMaj);

        // 反例：太阴守命但在子宫
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Zi && v.has(s, StarKey::TaiyinMaj)
        });
        assert!(yue_lang_tian_men(&view(&b)).is_empty());
    }

    #[test]
    fn yue_sheng_cang_hai_reports_soul_and_property_variants() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Zi && v.has(s, StarKey::TaiyinMaj) && v.has(s, StarKey::TiantongMaj)
        });
        let v = view(&a);
        let hits = yue_sheng_cang_hai(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.variant, Some("soul"));
        assert_eq!(hit.palace, v.soul());

        let b = find_chart(|a| {
            let v = view(a);
            let p = v.index_of(Palace::Property);
            v.branch(p) == Zi && v.has(p, StarKey::TaiyinMaj) && v.has(p, StarKey::TiantongMaj)
        });
        let v = view(&b);
        let hits = yue_sheng_cang_hai(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.variant, Some("property"));
        assert_eq!(hit.palace, v.index_of(Palace::Property));

        // 反例：同阴在子，但既非命宫也非田宅宫
        let c = find_chart(|a| {
            let v = view(a);
            let zi = (0..12).find(|i| v.branch(*i) == Zi).unwrap();
            v.has(zi, StarKey::TaiyinMaj)
                && v.has(zi, StarKey::TiantongMaj)
                && zi != v.soul()
                && zi != v.index_of(Palace::Property)
        });
        assert!(yue_sheng_cang_hai(&view(&c)).is_empty());
    }

    #[test]
    fn ming_zhu_chu_hai_needs_empty_wei_soul_facing_tong_ju() {
        let a = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Wei
                && v.is_empty(s)
                && v.has(v.opposite(s), StarKey::TiantongMaj)
                && v.has(v.opposite(s), StarKey::JumenMaj)
        });
        let v = view(&a);
        let hits = ming_zhu_chu_hai(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        // 财帛卯太阳、官禄亥太阴随此格局必然成立，已记入证据
        let sun = hit
            .stars
            .iter()
            .find(|s| s.star == StarKey::TaiyangMaj)
            .expect("sun");
        let moon = hit
            .stars
            .iter()
            .find(|s| s.star == StarKey::TaiyinMaj)
            .expect("moon");
        assert_eq!(v.branch(sun.palace), Mao);
        assert_eq!(v.branch(moon.palace), Hai);

        // 反例：命宫空宫在丑，对宫未坐天同巨门（未宫才是此格的命宫）
        let b = find_chart(|a| {
            let (v, s) = (view(a), view(a).soul());
            v.branch(s) == Chou
                && v.is_empty(s)
                && v.has(v.opposite(s), StarKey::TiantongMaj)
                && v.has(v.opposite(s), StarKey::JumenMaj)
        });
        assert!(ming_zhu_chu_hai(&view(&b)).is_empty());
    }
}

//! 武曲、廉贞、贪狼、巨门、天梁、七杀、破军系格局（《格局》页 #25-#39，行运格禄衰马困在 [`super::lu`]）。

use crate::data::stars::StarKey;
use crate::data::types::{EarthlyBranch, Palace};
use crate::models::star::Star;
use crate::pattern::view::{KONG_WANG, SHA6, SPL};
use crate::pattern::{ChartView, PatternHit, PatternKey, Rule};

/// 本组规则，按《格局》页顺序。
pub static RULES: &[Rule] = &[
    Rule {
        key: PatternKey::WuTanTongXing,
        detect: wu_tan_tong_xing,
    },
    Rule {
        key: PatternKey::LingChangTuoWu,
        detect: ling_chang_tuo_wu,
    },
    Rule {
        key: PatternKey::XingQiuJiaYin,
        detect: xing_qiu_jia_yin,
    },
    Rule {
        key: PatternKey::ShengBuFengShi,
        detect: sheng_bu_feng_shi,
    },
    Rule {
        key: PatternKey::XiongSuChaoYuan,
        detect: xiong_su_chao_yuan,
    },
    Rule {
        key: PatternKey::FuXiangChaoYuan,
        detect: fu_xiang_chao_yuan,
    },
    Rule {
        key: PatternKey::HuoTan,
        detect: huo_tan,
    },
    Rule {
        key: PatternKey::LingTan,
        detect: ling_tan,
    },
    Rule {
        key: PatternKey::ShiZhongYinYu,
        detect: shi_zhong_yin_yu,
    },
    Rule {
        key: PatternKey::LiangMaPiaoDang,
        detect: liang_ma_piao_dang,
    },
    Rule {
        key: PatternKey::YangLiangChangLu,
        detect: yang_liang_chang_lu,
    },
    Rule {
        key: PatternKey::ShaPoLang,
        detect: sha_po_lang,
    },
    Rule {
        key: PatternKey::QiShaChaoDou,
        detect: qi_sha_chao_dou,
    },
    Rule {
        key: PatternKey::YingXingRuMiao,
        detect: ying_xing_ru_miao,
    },
    Rule {
        key: PatternKey::ZhongShuiChaoDong,
        detect: zhong_shui_chao_dong,
    },
];

/// 刑星：天刑与擎羊（刑囚夹印的「刑」，两说并存，见者皆算）。
const XING: [StarKey; 2] = [StarKey::Tianxing, StarKey::QingyangMin];

/// 在 `i` 的三方四正里逐颗找齐 `keys`，缺一颗即 `None`；命中返回每颗星与其落宫。
fn all_in_surround<'a>(
    v: &ChartView<'a>,
    i: usize,
    keys: &[StarKey],
) -> Option<Vec<(usize, &'a Star)>> {
    keys.iter().map(|k| v.find_in_surround(i, *k)).collect()
}

/// 宫内 `keys` 中在场的星及其落宫（一颗都没有则为空）。
fn present<'a>(v: &ChartView<'a>, i: usize, keys: &[StarKey]) -> Vec<(usize, &'a Star)> {
    keys.iter()
        .filter_map(|k| v.find(i, *k).map(|s| (i, s)))
        .collect()
}

/// 武贪同行：武曲、贪狼同守命宫或身宫（二星同宫只可能落丑、未）。
///
/// 来源：「贪武同行威镇边夷」「先贫后富武贪同身命之宫」。古书「身命」按身宫亦论，
/// 命宫与身宫各自成格各报一次。
pub fn wu_tan_tong_xing(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter(|p| matches!(v.branch(*p), EarthlyBranch::Chou | EarthlyBranch::Wei))
        .filter_map(|p| {
            let (Some(wu), Some(tan)) =
                (v.find(p, StarKey::WuquMaj), v.find(p, StarKey::TanlangMaj))
            else {
                return None;
            };
            Some(v.hit(PatternKey::WuTanTongXing, p, vec![(p, wu), (p, tan)]))
        })
        .collect()
}

/// 铃昌陀武：铃星、文昌、陀罗、武曲四星齐会命宫三方四正。
///
/// 来源：「铃昌陀武限至投河」；作者「在命宫三方四正里同时出现」。运限视角下以运限命宫
/// 为命宫、流昌流陀等同本命昌陀，故行运逢此格由视图本身覆盖。
pub fn ling_chang_tuo_wu(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let keys = [
        StarKey::LingxingMin,
        StarKey::WenchangMin,
        StarKey::TuoluoMin,
        StarKey::WuquMaj,
    ];
    match all_in_surround(v, s, &keys) {
        Some(stars) => vec![v.hit(PatternKey::LingChangTuoWu, s, stars)],
        None => vec![],
    }
}

/// 刑囚夹印：廉贞（囚）、天相（印）与刑星（天刑或擎羊）同守命宫或身宫。
///
/// 名为「夹」实为同宫，廉相同宫只可能落子、午。来源：「刑囚夹印 天刑廉贞同临身命
/// 主武勇之人」。作者提及的「刑囚夹忌」「刑忌夹印」两变体页面未给完整定义，不实现。
pub fn xing_qiu_jia_yin(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter_map(|p| {
            let (Some(lian), Some(xiang)) = (
                v.find(p, StarKey::LianzhenMaj),
                v.find(p, StarKey::TianxiangMaj),
            ) else {
                return None;
            };
            let xing = present(v, p, &XING);
            if xing.is_empty() {
                return None;
            }
            let mut stars = vec![(p, lian), (p, xiang)];
            stars.extend(xing);
            Some(v.hit(PatternKey::XingQiuJiaYin, p, stars))
        })
        .collect()
}

/// 生不逢时：命宫空亡星（旬空、空亡、截路、截空）与廉贞同宫。
///
/// 来源：「生不逢时 命坐空亡逢廉贞是也」。另有资料称破军与空亡同宫亦算，
/// 以 `variant = "pojun"` 并报；廉贞形态为主格，无 variant。
pub fn sheng_bu_feng_shi(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let kong = present(v, s, &KONG_WANG);
    if kong.is_empty() {
        return vec![];
    }
    [
        (StarKey::LianzhenMaj, None),
        (StarKey::PojunMaj, Some("pojun")),
    ]
    .into_iter()
    .filter_map(|(key, variant)| {
        let star = v.find(s, key)?;
        let mut stars = vec![(s, star)];
        stars.extend(kong.iter().copied());
        let mut hit = v.hit(PatternKey::ShengBuFengShi, s, stars);
        hit.variant = variant;
        Some(hit)
    })
    .collect()
}

/// 雄宿朝元：廉贞独坐寅或申守命（寅申廉贞必独坐）。
///
/// 来源：「廉贞申未宫无杀富贵声扬播远名 雄宿朝元格，加杀平常」。取作者的寅申口径；
/// 古书「加杀平常」不作成格条件，命宫三方四正见火铃羊陀空劫时记 `broken`。
pub fn xiong_su_chao_yuan(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if !matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen) {
        return vec![];
    }
    let Some(lian) = v.find(s, StarKey::LianzhenMaj) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::XiongSuChaoYuan, s, vec![(s, lian)]);
    hit.broken = v.surround_has_any(s, &SHA6);
    vec![hit]
}

/// 府相朝垣：天府居官禄宫、天相居财帛宫，二宫朝拱命宫。
///
/// 天相恒在天府顺行第四宫，故二条件互相蕴含。来源：「府相朝垣命必荣」
/// 「命无主星，天府居官禄、天相居财帛」。古书「命无主星」不作成格条件（作者示例多有主星），
/// 命宫确为空宫时记 `variant = "soul_empty"`。
pub fn fu_xiang_chao_yuan(v: &ChartView) -> Vec<PatternHit> {
    let (career, wealth) = (v.index_of(Palace::Career), v.index_of(Palace::Wealth));
    let (Some(fu), Some(xiang)) = (
        v.find(career, StarKey::TianfuMaj),
        v.find(wealth, StarKey::TianxiangMaj),
    ) else {
        return vec![];
    };
    let s = v.soul();
    let mut hit = v.hit(
        PatternKey::FuXiangChaoYuan,
        s,
        vec![(career, fu), (wealth, xiang)],
    );
    if v.is_empty(s) {
        hit.variant = Some("soul_empty");
    }
    vec![hit]
}

/// 火贪：贪狼守命与火星同宫；火星只在三方四正会照者记 `variant = "surround"`。
///
/// 来源：「贪狼火星居庙旺名镇诸邦」（同宫）与「火星拱会」「贪狼火星同垣三合者」（会照）。
/// 页面未要求贪狼庙旺，不加亮度条件。
pub fn huo_tan(v: &ChartView) -> Vec<PatternHit> {
    tan_with_fire(v, PatternKey::HuoTan, StarKey::HuoxingMin)
}

/// 铃贪：贪狼守命与铃星同宫；铃星只在三方四正会照者记 `variant = "surround"`。
///
/// 来源：与火贪同形（页面并列两格，古书引文只举火星）。
pub fn ling_tan(v: &ChartView) -> Vec<PatternHit> {
    tan_with_fire(v, PatternKey::LingTan, StarKey::LingxingMin)
}

/// 火贪、铃贪共用判定：贪狼守命，火（铃）星同宫为主格、三方四正会照为宽口径。
fn tan_with_fire(v: &ChartView, key: PatternKey, fire: StarKey) -> Vec<PatternHit> {
    let s = v.soul();
    let Some(tan) = v.find(s, StarKey::TanlangMaj) else {
        return vec![];
    };
    if let Some(f) = v.find(s, fire) {
        return vec![v.hit(key, s, vec![(s, tan), (s, f)])];
    }
    let Some((pf, f)) = v.find_in_surround(s, fire) else {
        return vec![];
    };
    let mut hit = v.hit(key, s, vec![(s, tan), (pf, f)]);
    hit.variant = Some("surround");
    vec![hit]
}

/// 石中隐玉：巨门守命宫或身宫，且该宫在子或午。
///
/// 来源：「子午巨门石中隐玉」「身命居子午宫为石中隐玉格」。古书「身命」按身宫亦论。
pub fn shi_zhong_yin_yu(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter(|p| matches!(v.branch(*p), EarthlyBranch::Zi | EarthlyBranch::Wu))
        .filter_map(|p| {
            let ju = v.find(p, StarKey::JumenMaj)?;
            Some(v.hit(PatternKey::ShiZhongYinYu, p, vec![(p, ju)]))
        })
        .collect()
}

/// 梁马飘荡：天梁与天马同守命宫或身宫。
///
/// 来源：「天梁天马陷飘荡无疑」「天梁天马为人飘荡风流」。古书的「陷」不作条件
/// （页面未采纳），宫位取命身。
pub fn liang_ma_piao_dang(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter_map(|p| {
            let (Some(liang), Some(ma)) = (
                v.find(p, StarKey::TianliangMaj),
                v.find(p, StarKey::TianmaMin),
            ) else {
                return None;
            };
            Some(v.hit(PatternKey::LiangMaPiaoDang, p, vec![(p, liang), (p, ma)]))
        })
        .collect()
}

/// 阳梁昌禄：太阳、天梁、文昌、禄存四星齐会命宫三方四正。
///
/// 来源：「天梁太阳昌禄会胪传第一名」；作者明确「三方四正中集齐」。
pub fn yang_liang_chang_lu(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let keys = [
        StarKey::TaiyangMaj,
        StarKey::TianliangMaj,
        StarKey::WenchangMin,
        StarKey::LucunMin,
    ];
    match all_in_surround(v, s, &keys) {
        Some(stars) => vec![v.hit(PatternKey::YangLiangChangLu, s, stars)],
        None => vec![],
    }
}

/// 杀破狼：七杀、破军、贪狼三星之一坐命宫或身宫（三星恒成三合，见一即三方全见）。
///
/// 来源：「七杀破军宜出外，机月同梁作吏人」；作者「在命宫或身宫形成这个格局都有这个特点」。
/// 证据列出三方四正内的三颗星各自落宫。
pub fn sha_po_lang(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter(|p| v.has_any(*p, &SPL))
        .filter_map(|p| {
            let stars = all_in_surround(v, p, &SPL)?;
            Some(v.hit(PatternKey::ShaPoLang, p, stars))
        })
        .collect()
}

/// 七杀朝斗：七杀守命，且命宫在子、午、寅、申。
///
/// 来源：「七杀朝斗 寅申子午一生爵禄荣昌」。子名两说，取作者所述
/// 寅子为「仰斗」（`variant = "yang_dou"`）、午申为「朝斗」（`variant = "chao_dou"`），
/// 名称之别不影响成格。
pub fn qi_sha_chao_dou(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let variant = match v.branch(s) {
        EarthlyBranch::Zi | EarthlyBranch::Yin => "yang_dou",
        EarthlyBranch::Wu | EarthlyBranch::Shen => "chao_dou",
        _ => return vec![],
    };
    let Some(sha) = v.find(s, StarKey::QishaMaj) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::QiShaChaoDou, s, vec![(s, sha)]);
    hit.variant = Some(variant);
    vec![hit]
}

/// 英星入庙：破军守命，且命宫在子或午（亮度表破军子午皆庙，与格名相符）。
///
/// 来源：「子午破军加官进禄」「破军子午宫无杀官资清显至三公」。古书的「无杀」是吉断加语，
/// 页面未作成格条件，亦未称破格，故不记 `broken`。
pub fn ying_xing_ru_miao(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if !matches!(v.branch(s), EarthlyBranch::Zi | EarthlyBranch::Wu) {
        return vec![];
    }
    let Some(po) = v.find(s, StarKey::PojunMaj) else {
        return vec![];
    };
    vec![v.hit(PatternKey::YingXingRuMiao, s, vec![(s, po)])]
}

/// 众水朝东：破军与文曲同守命宫，且命宫在寅或卯（寅为破军独坐，卯为廉破）。
///
/// 来源：「文耗居寅卯，谓之众水朝东」（破军为耗星）。古书未限命宫，页面两张示例皆命宫，
/// 从示例取命宫。
pub fn zhong_shui_chao_dong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if !matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Mao) {
        return vec![];
    }
    let (Some(po), Some(qu)) = (v.find(s, StarKey::PojunMaj), v.find(s, StarKey::WenquMin)) else {
        return vec![];
    };
    vec![v.hit(PatternKey::ZhongShuiChaoDong, s, vec![(s, po), (s, qu)])]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::astrolabe::Astrolabe;
    use crate::pattern::PatternConfig;
    use crate::pattern::testutil::find_chart;

    /// 默认口径的本命视图。
    fn view(a: &Astrolabe) -> ChartView<'_> {
        ChartView::natal(a, &PatternConfig::default())
    }

    /// 本组规则在本命盘上对某格局的全部命中（只跑本组 [`RULES`]，与其它组无关）。
    fn hits(a: &Astrolabe, key: PatternKey) -> Vec<PatternHit> {
        let v = view(a);
        RULES
            .iter()
            .filter(|r| r.key == key)
            .flat_map(|r| (r.detect)(&v))
            .collect()
    }

    /// 命中里唯一的一个，缺失即失败。
    fn one(a: &Astrolabe, key: PatternKey) -> PatternHit {
        let mut h = hits(a, key);
        assert_eq!(h.len(), 1, "expected exactly one hit of {:?}", key.as_key());
        h.pop().unwrap()
    }

    /// 命中证据里的星耀集合。
    fn star_keys(h: &PatternHit) -> Vec<StarKey> {
        h.stars.iter().map(|s| s.star).collect()
    }

    #[test]
    fn wu_tan_tong_xing_needs_both_stars_together() {
        let a = find_chart(|a| {
            let v = view(a);
            v.soul_and_body()
                .iter()
                .any(|p| v.has_all(*p, &[StarKey::WuquMaj, StarKey::TanlangMaj]))
        });
        let h = one(&a, PatternKey::WuTanTongXing);
        let v = view(&a);
        assert!(v.soul_and_body().contains(&h.palace));
        assert!(matches!(
            v.branch(h.palace),
            EarthlyBranch::Chou | EarthlyBranch::Wei
        ));
        assert_eq!(star_keys(&h), vec![StarKey::WuquMaj, StarKey::TanlangMaj]);
        assert!(h.stars.iter().all(|s| s.palace == h.palace));

        // 反例：命身皆无二星同宫（命宫仅武曲）
        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::WuquMaj)
                && !v
                    .soul_and_body()
                    .iter()
                    .any(|p| v.has_all(*p, &[StarKey::WuquMaj, StarKey::TanlangMaj]))
        });
        assert!(hits(&b, PatternKey::WuTanTongXing).is_empty());
    }

    #[test]
    fn ling_chang_tuo_wu_needs_all_four_in_surround() {
        const FOUR: [StarKey; 4] = [
            StarKey::LingxingMin,
            StarKey::WenchangMin,
            StarKey::TuoluoMin,
            StarKey::WuquMaj,
        ];
        let count = |a: &Astrolabe| {
            let v = view(a);
            FOUR.iter().filter(|k| v.in_surround(v.soul(), **k)).count()
        };
        let a = find_chart(|a| count(a) == 4);
        let h = one(&a, PatternKey::LingChangTuoWu);
        let v = view(&a);
        assert_eq!(h.palace, v.soul());
        assert_eq!(star_keys(&h), FOUR.to_vec());
        assert!(
            h.stars
                .iter()
                .all(|s| v.surround(v.soul()).contains(&s.palace))
        );

        // 反例：三方四正只齐三颗
        let b = find_chart(|a| count(a) == 3);
        assert!(hits(&b, PatternKey::LingChangTuoWu).is_empty());
    }

    #[test]
    fn xing_qiu_jia_yin_needs_lian_xiang_and_a_xing_star() {
        let lian_xiang =
            |v: &ChartView, p: usize| v.has_all(p, &[StarKey::LianzhenMaj, StarKey::TianxiangMaj]);
        let a = find_chart(|a| {
            let v = view(a);
            v.soul_and_body()
                .iter()
                .any(|p| lian_xiang(&v, *p) && v.has_any(*p, &XING))
        });
        let h = one(&a, PatternKey::XingQiuJiaYin);
        let v = view(&a);
        assert!(v.soul_and_body().contains(&h.palace));
        assert!(matches!(
            v.branch(h.palace),
            EarthlyBranch::Zi | EarthlyBranch::Wu
        ));
        assert!(star_keys(&h).starts_with(&[StarKey::LianzhenMaj, StarKey::TianxiangMaj]));
        assert!(h.stars.iter().skip(2).all(|s| XING.contains(&s.star)));

        // 反例：廉相同宫但不见天刑、擎羊
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            lian_xiang(&v, s) && !v.soul_and_body().iter().any(|p| v.has_any(*p, &XING))
        });
        assert!(hits(&b, PatternKey::XingQiuJiaYin).is_empty());
    }

    #[test]
    fn sheng_bu_feng_shi_reports_lianzhen_and_pojun_forms() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::LianzhenMaj) && v.has_any(s, &KONG_WANG)
        });
        let h = one(&a, PatternKey::ShengBuFengShi);
        assert_eq!(h.palace, view(&a).soul());
        assert_eq!(h.variant, None);
        assert_eq!(h.stars[0].star, StarKey::LianzhenMaj);
        assert!(h.stars[1..].iter().all(|s| KONG_WANG.contains(&s.star)));

        // 破军形态：另一口径，以 variant 报出（廉破同宫时两种形态各报一次）
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::PojunMaj)
                && !v.has(s, StarKey::LianzhenMaj)
                && v.has_any(s, &KONG_WANG)
        });
        assert_eq!(one(&b, PatternKey::ShengBuFengShi).variant, Some("pojun"));

        // 反例：命宫廉贞而无空亡星
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::LianzhenMaj) && !v.has_any(s, &KONG_WANG)
        });
        assert!(hits(&c, PatternKey::ShengBuFengShi).is_empty());
    }

    #[test]
    fn xiong_su_chao_yuan_limited_to_yin_shen_and_marks_sha() {
        let at = |a: &Astrolabe, branches: [EarthlyBranch; 2]| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::LianzhenMaj) && branches.contains(&v.branch(s))
        };
        let clean = find_chart(|a| {
            let v = view(a);
            at(a, [EarthlyBranch::Yin, EarthlyBranch::Shen]) && !v.surround_has_any(v.soul(), &SHA6)
        });
        let h = one(&clean, PatternKey::XiongSuChaoYuan);
        assert_eq!(h.palace, view(&clean).soul());
        assert_eq!(star_keys(&h), vec![StarKey::LianzhenMaj]);
        assert!(!h.broken);

        // 加杀：成格照报，只标 broken
        let with_sha = find_chart(|a| {
            let v = view(a);
            at(a, [EarthlyBranch::Yin, EarthlyBranch::Shen]) && v.surround_has_any(v.soul(), &SHA6)
        });
        assert!(one(&with_sha, PatternKey::XiongSuChaoYuan).broken);

        // 反例：廉贞在未（古书口径的宫位，本规则取作者的寅申）
        let wei = find_chart(|a| at(a, [EarthlyBranch::Wei, EarthlyBranch::Wei]));
        assert!(hits(&wei, PatternKey::XiongSuChaoYuan).is_empty());
    }

    #[test]
    fn fu_xiang_chao_yuan_reads_career_and_wealth() {
        let fu_at_career = |a: &Astrolabe| {
            let v = view(a);
            v.has(v.index_of(Palace::Career), StarKey::TianfuMaj)
        };
        let a = find_chart(|a| fu_at_career(a) && !view(a).is_empty(view(a).soul()));
        let h = one(&a, PatternKey::FuXiangChaoYuan);
        let v = view(&a);
        assert_eq!(h.palace, v.soul());
        assert_eq!(h.variant, None);
        assert_eq!(
            star_keys(&h),
            vec![StarKey::TianfuMaj, StarKey::TianxiangMaj]
        );
        assert_eq!(h.stars[0].palace, v.index_of(Palace::Career));
        assert_eq!(h.stars[1].palace, v.index_of(Palace::Wealth));

        // 命宫空宫（古书「命无主星」形态）另标 variant
        let b = find_chart(|a| fu_at_career(a) && view(a).is_empty(view(a).soul()));
        assert_eq!(
            one(&b, PatternKey::FuXiangChaoYuan).variant,
            Some("soul_empty")
        );

        // 反例：官禄宫无天府
        let c = find_chart(|a| !fu_at_career(a));
        assert!(hits(&c, PatternKey::FuXiangChaoYuan).is_empty());
    }

    #[test]
    fn huo_tan_separates_same_palace_from_surround() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(v.soul(), &[StarKey::TanlangMaj, StarKey::HuoxingMin])
        });
        let h = one(&a, PatternKey::HuoTan);
        let v = view(&a);
        assert_eq!(h.palace, v.soul());
        assert_eq!(h.variant, None);
        assert!(h.stars.iter().all(|s| s.palace == v.soul()));

        // 三方会照：贪狼守命、火星在三方四正另一宫
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TanlangMaj)
                && !v.has(s, StarKey::HuoxingMin)
                && v.in_surround(s, StarKey::HuoxingMin)
        });
        let hb = one(&b, PatternKey::HuoTan);
        assert_eq!(hb.variant, Some("surround"));
        assert_ne!(hb.stars[1].palace, hb.palace);

        // 反例：贪狼守命而火星不在三方四正
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TanlangMaj) && !v.in_surround(s, StarKey::HuoxingMin)
        });
        assert!(hits(&c, PatternKey::HuoTan).is_empty());
    }

    #[test]
    fn ling_tan_mirrors_huo_tan() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(v.soul(), &[StarKey::TanlangMaj, StarKey::LingxingMin])
        });
        let h = one(&a, PatternKey::LingTan);
        assert_eq!(h.variant, None);
        assert_eq!(
            star_keys(&h),
            vec![StarKey::TanlangMaj, StarKey::LingxingMin]
        );

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TanlangMaj)
                && !v.has(s, StarKey::LingxingMin)
                && v.in_surround(s, StarKey::LingxingMin)
        });
        assert_eq!(one(&b, PatternKey::LingTan).variant, Some("surround"));

        // 反例：贪狼守命而铃星不在三方四正
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TanlangMaj) && !v.in_surround(s, StarKey::LingxingMin)
        });
        assert!(hits(&c, PatternKey::LingTan).is_empty());
    }

    #[test]
    fn shi_zhong_yin_yu_limited_to_zi_wu() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::JumenMaj)
                && matches!(v.branch(s), EarthlyBranch::Zi | EarthlyBranch::Wu)
        });
        let h = one(&a, PatternKey::ShiZhongYinYu);
        assert_eq!(h.palace, view(&a).soul());
        assert_eq!(star_keys(&h), vec![StarKey::JumenMaj]);

        // 反例：巨门守命于亥
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::JumenMaj)
                && v.branch(s) == EarthlyBranch::Hai
                && !v.soul_and_body().iter().any(|p| {
                    v.has(*p, StarKey::JumenMaj)
                        && matches!(v.branch(*p), EarthlyBranch::Zi | EarthlyBranch::Wu)
                })
        });
        assert!(hits(&b, PatternKey::ShiZhongYinYu).is_empty());
    }

    #[test]
    fn liang_ma_piao_dang_needs_liang_and_ma_together() {
        let a = find_chart(|a| {
            let v = view(a);
            v.soul_and_body()
                .iter()
                .any(|p| v.has_all(*p, &[StarKey::TianliangMaj, StarKey::TianmaMin]))
        });
        let h = one(&a, PatternKey::LiangMaPiaoDang);
        let v = view(&a);
        assert!(v.soul_and_body().contains(&h.palace));
        assert_eq!(
            star_keys(&h),
            vec![StarKey::TianliangMaj, StarKey::TianmaMin]
        );

        // 反例：天梁守命而天马不同宫
        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::TianliangMaj)
                && !v
                    .soul_and_body()
                    .iter()
                    .any(|p| v.has_all(*p, &[StarKey::TianliangMaj, StarKey::TianmaMin]))
        });
        assert!(hits(&b, PatternKey::LiangMaPiaoDang).is_empty());
    }

    #[test]
    fn yang_liang_chang_lu_needs_all_four_in_surround() {
        const FOUR: [StarKey; 4] = [
            StarKey::TaiyangMaj,
            StarKey::TianliangMaj,
            StarKey::WenchangMin,
            StarKey::LucunMin,
        ];
        let count = |a: &Astrolabe| {
            let v = view(a);
            FOUR.iter().filter(|k| v.in_surround(v.soul(), **k)).count()
        };
        let a = find_chart(|a| count(a) == 4);
        let h = one(&a, PatternKey::YangLiangChangLu);
        assert_eq!(h.palace, view(&a).soul());
        assert_eq!(star_keys(&h), FOUR.to_vec());

        let b = find_chart(|a| count(a) == 3);
        assert!(hits(&b, PatternKey::YangLiangChangLu).is_empty());
    }

    #[test]
    fn sha_po_lang_hits_soul_and_body_separately() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_any(v.soul(), &SPL)
        });
        let h = &hits(&a, PatternKey::ShaPoLang)[0];
        let v = view(&a);
        assert_eq!(h.palace, v.soul());
        assert_eq!(star_keys(h), SPL.to_vec());
        assert!(
            h.stars
                .iter()
                .all(|s| v.surround(v.soul()).contains(&s.palace))
        );

        // 身宫成格：命宫不见杀破狼，身宫见
        let b = find_chart(|a| {
            let v = view(a);
            let body = v.body();
            !v.has_any(v.soul(), &SPL) && body.is_some_and(|b| v.has_any(b, &SPL))
        });
        let hb = one(&b, PatternKey::ShaPoLang);
        assert_eq!(Some(hb.palace), view(&b).body());

        // 反例：命身皆不见杀破狼
        let c = find_chart(|a| {
            let v = view(a);
            !v.soul_and_body().iter().any(|p| v.has_any(*p, &SPL))
        });
        assert!(hits(&c, PatternKey::ShaPoLang).is_empty());
    }

    #[test]
    fn qi_sha_chao_dou_names_both_variants() {
        let qi_sha_at = |branch: EarthlyBranch| {
            move |a: &Astrolabe| {
                let v = view(a);
                let s = v.soul();
                v.has(s, StarKey::QishaMaj) && v.branch(s) == branch
            }
        };
        let zi = find_chart(qi_sha_at(EarthlyBranch::Zi));
        assert_eq!(one(&zi, PatternKey::QiShaChaoDou).variant, Some("yang_dou"));
        let shen = find_chart(qi_sha_at(EarthlyBranch::Shen));
        let h = one(&shen, PatternKey::QiShaChaoDou);
        assert_eq!(h.variant, Some("chao_dou"));
        assert_eq!(star_keys(&h), vec![StarKey::QishaMaj]);
        assert_eq!(h.palace, view(&shen).soul());

        // 反例：七杀守命于辰
        let chen = find_chart(qi_sha_at(EarthlyBranch::Chen));
        assert!(hits(&chen, PatternKey::QiShaChaoDou).is_empty());
    }

    #[test]
    fn ying_xing_ru_miao_limited_to_zi_wu() {
        let po_at = |branches: [EarthlyBranch; 2]| {
            move |a: &Astrolabe| {
                let v = view(a);
                let s = v.soul();
                v.has(s, StarKey::PojunMaj) && branches.contains(&v.branch(s))
            }
        };
        let a = find_chart(po_at([EarthlyBranch::Zi, EarthlyBranch::Wu]));
        let h = one(&a, PatternKey::YingXingRuMiao);
        assert_eq!(h.palace, view(&a).soul());
        assert_eq!(
            h.stars[0].brightness,
            Some(crate::data::types::Brightness::Miao)
        );
        assert!(!h.broken);

        // 反例：破军守命于寅（亮度为得，非庙）
        let b = find_chart(po_at([EarthlyBranch::Yin, EarthlyBranch::Yin]));
        assert!(hits(&b, PatternKey::YingXingRuMiao).is_empty());
    }

    #[test]
    fn zhong_shui_chao_dong_needs_pojun_wenqu_in_yin_mao() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has_all(s, &[StarKey::PojunMaj, StarKey::WenquMin])
                && matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Mao)
        });
        let h = one(&a, PatternKey::ZhongShuiChaoDong);
        assert_eq!(h.palace, view(&a).soul());
        assert_eq!(star_keys(&h), vec![StarKey::PojunMaj, StarKey::WenquMin]);

        // 反例：破军守命于寅而文曲不同宫
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::PojunMaj)
                && v.branch(s) == EarthlyBranch::Yin
                && !v.has(s, StarKey::WenquMin)
        });
        assert!(hits(&b, PatternKey::ZhongShuiChaoDong).is_empty());
    }
}

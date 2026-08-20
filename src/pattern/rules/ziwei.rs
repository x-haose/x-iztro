//! 紫微、天府、天机系格局（《格局》页 #1-#10）。

use crate::data::stars::StarKey;
use crate::data::types::EarthlyBranch;
use crate::models::star::Star;
use crate::pattern::view::JYTL;
use crate::pattern::{ChartView, PatternHit, PatternKey, Rule};

/// 本组规则，按《格局》页顺序。
pub static RULES: &[Rule] = &[
    Rule {
        key: PatternKey::JunChenQingHui,
        detect: jun_chen_qing_hui,
    },
    Rule {
        key: PatternKey::ZiFuTongGong,
        detect: zi_fu_tong_gong,
    },
    Rule {
        key: PatternKey::JinYuFuJia,
        detect: jin_yu_fu_jia,
    },
    Rule {
        key: PatternKey::ZiFuJiaMing,
        detect: zi_fu_jia_ming,
    },
    Rule {
        key: PatternKey::JiXiangLiMing,
        detect: ji_xiang_li_ming,
    },
    Rule {
        key: PatternKey::JiJuMaoYou,
        detect: ji_ju_mao_you,
    },
    Rule {
        key: PatternKey::JiYueTongLiang,
        detect: ji_yue_tong_liang,
    },
    Rule {
        key: PatternKey::ShanYinChaoGang,
        detect: shan_yin_chao_gang,
    },
    Rule {
        key: PatternKey::JiJuTongLin,
        detect: ji_ju_tong_lin,
    },
    Rule {
        key: PatternKey::JiJuJuMao,
        detect: ji_ju_ju_mao,
    },
];

/// 机阴：机月同梁在寅申的两种主星组合之一。
const JI_YIN: [StarKey; 2] = [StarKey::TianjiMaj, StarKey::TaiyinMaj];

/// 同梁：机月同梁在寅申的两种主星组合之一。
const TONG_LIANG: [StarKey; 2] = [StarKey::TiantongMaj, StarKey::TianliangMaj];

/// 君臣庆会：紫微或天府与辅佐诸星成下列会合之一。
///
/// `variant` 分列四种形式：
/// - `zi_po_zuo_you_jia`：紫微破军同守命宫（丑未），左辅右弼分夹命宫；
/// - `zi_xiang_chang_qu_axis`：紫微天相同守命宫（辰戌），文昌文曲分居命宫与迁移宫；
/// - `tian_fu_ji_yin_tong_liang_jia`：天府守命，天机、太阴、天同、天梁四星俱全、
///   分布于命宫前后两宫（页面示例即一侧天同太阴、另一侧天机天梁）；夹宫为空宫时
///   借其对宫主星补足——安星几何下四星俱全只出现在天府巳亥守命的布局，
///   且此时命宫前一宫恒为空宫，借宫是此形式成立的必要条件；
/// - `zi_zuo_you_tong_gong`：紫微与左辅右弼三星同守命宫。
///
/// 来源：全书「君臣庆会 紫微左右同守命是也，更会相武阴妙上」「天府天相天梁同君臣庆会」。
/// 前三形式为页面作者所列，并带作者的前提「命宫三方四正不能有煞忌」，不满足即不成格；
/// 第四形式取古书「紫微左右同守命」的字面义，不受该前提约束，只把煞忌记在 `broken` 上：
/// 紫微与左辅右弼同宫只出现在丑未宫且必生于亥时，此时地空恒落命迁线，
/// 若也施加无煞前提，这一形式就成了永不成立的死规则。
pub fn jun_chen_qing_hui(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let clean = v.no_sha(s);
    let key = PatternKey::JunChenQingHui;
    let mut hits = Vec::new();

    let push = |hits: &mut Vec<PatternHit>, variant, stars: Vec<(usize, &Star)>| {
        let mut hit = v.hit(key, s, stars);
        hit.variant = Some(variant);
        hits.push(hit);
    };

    if clean
        && let (Some(zi), Some(po), Some((pz, py))) = (
            v.find(s, StarKey::ZiweiMaj),
            v.find(s, StarKey::PojunMaj),
            v.jia(s, &[StarKey::ZuofuMin], &[StarKey::YoubiMin]),
        )
        && let (Some(zuo), Some(you)) =
            (v.find(pz, StarKey::ZuofuMin), v.find(py, StarKey::YoubiMin))
    {
        push(
            &mut hits,
            "zi_po_zuo_you_jia",
            vec![(s, zi), (s, po), (pz, zuo), (py, you)],
        );
    }

    let o = v.opposite(s);
    if clean
        && let (Some(zi), Some(xiang)) = (
            v.find(s, StarKey::ZiweiMaj),
            v.find(s, StarKey::TianxiangMaj),
        )
        && let Some((chang, qu)) = v
            .find(s, StarKey::WenchangMin)
            .zip(v.find(o, StarKey::WenquMin))
            .map(|(c, q)| ((s, c), (o, q)))
            .or_else(|| {
                v.find(s, StarKey::WenquMin)
                    .zip(v.find(o, StarKey::WenchangMin))
                    .map(|(q, c)| ((o, c), (s, q)))
            })
    {
        push(
            &mut hits,
            "zi_xiang_chang_qu_axis",
            vec![(s, zi), (s, xiang), chang, qu],
        );
    }

    if clean && let Some(fu) = v.find(s, StarKey::TianfuMaj) {
        let (prev, next) = (v.prev(s), v.next(s));
        let four: Option<Vec<_>> = JYTL
            .iter()
            .map(|k| {
                v.find_major_at(prev, *k)
                    .or_else(|| v.find_major_at(next, *k))
            })
            .collect();
        if let Some(four) = four {
            let mut stars = vec![(s, fu)];
            stars.extend(four);
            push(&mut hits, "tian_fu_ji_yin_tong_liang_jia", stars);
        }
    }

    if let (Some(zi), Some(zuo), Some(you)) = (
        v.find(s, StarKey::ZiweiMaj),
        v.find(s, StarKey::ZuofuMin),
        v.find(s, StarKey::YoubiMin),
    ) {
        let mut hit = v.hit(key, s, vec![(s, zi), (s, zuo), (s, you)]);
        hit.variant = Some("zi_zuo_you_tong_gong");
        hit.broken = !clean;
        hits.push(hit);
    }

    hits
}

/// 紫府同宫：紫微、天府同守命宫（只可能在寅、申）。
///
/// 来源：「紫府同宫终身福厚」。
pub fn zi_fu_tong_gong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match (v.find(s, StarKey::ZiweiMaj), v.find(s, StarKey::TianfuMaj)) {
        (Some(zi), Some(fu)) => vec![v.hit(PatternKey::ZiFuTongGong, s, vec![(s, zi), (s, fu)])],
        _ => vec![],
    }
}

/// 金舆扶驾：天府守命于丑或未，太阳、太阴分夹命宫。
///
/// 来源：全书「金舆扶驾 紫微守命前后有日月来夹是也」。按安星诀日月永远夹不到紫微，
/// 页面作者据此把「帝星」改判为天府，本规则采用作者口径。
/// 成格不要求无煞（作者：「这个格局是少数不多不怕煞星的格局」）。
pub fn jin_yu_fu_jia(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if !matches!(v.branch(s), EarthlyBranch::Chou | EarthlyBranch::Wei) {
        return vec![];
    }
    let (Some(fu), Some((pr, pi))) = (
        v.find(s, StarKey::TianfuMaj),
        v.jia(s, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj]),
    ) else {
        return vec![];
    };
    let (Some(sun), Some(moon)) = (
        v.find(pr, StarKey::TaiyangMaj),
        v.find(pi, StarKey::TaiyinMaj),
    ) else {
        return vec![];
    };
    vec![v.hit(
        PatternKey::JinYuFuJia,
        s,
        vec![(s, fu), (pr, sun), (pi, moon)],
    )]
}

/// 紫府夹命：天机、太阴同守命宫，紫微、天府分夹（只可能在寅、申）。
///
/// 来源：「紫府夹命为贵格」。
pub fn zi_fu_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(ji), Some(yin), Some((pz, pf))) = (
        v.find(s, StarKey::TianjiMaj),
        v.find(s, StarKey::TaiyinMaj),
        v.jia(s, &[StarKey::ZiweiMaj], &[StarKey::TianfuMaj]),
    ) else {
        return vec![];
    };
    let (Some(zi), Some(fu)) = (
        v.find(pz, StarKey::ZiweiMaj),
        v.find(pf, StarKey::TianfuMaj),
    ) else {
        return vec![];
    };
    vec![v.hit(
        PatternKey::ZiFuJiaMing,
        s,
        vec![(s, ji), (s, yin), (pz, zi), (pf, fu)],
    )]
}

/// 极向离明：紫微守命于午，且命宫三方四正无煞忌。
///
/// 来源：「紫微居午无杀凑位至三公」「紫微居午无刑忌…刑乃擎羊也」。
/// 「无杀」取作者的「三方四正无煞」，即火铃羊陀空劫与化忌俱不见，非古书窄义的擎羊加化忌。
pub fn ji_xiang_li_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match v.find(s, StarKey::ZiweiMaj) {
        Some(zi) if v.branch(s) == EarthlyBranch::Wu && v.no_sha(s) => {
            vec![v.hit(PatternKey::JiXiangLiMing, s, vec![(s, zi)])]
        }
        _ => vec![],
    }
}

/// 极居卯酉：紫微、贪狼同守命宫于卯或酉。
///
/// 来源：「极居卯酉多为脱俗僧人」。
pub fn ji_ju_mao_you(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if !matches!(v.branch(s), EarthlyBranch::Mao | EarthlyBranch::You) {
        return vec![];
    }
    match (v.find(s, StarKey::ZiweiMaj), v.find(s, StarKey::TanlangMaj)) {
        (Some(zi), Some(tan)) => vec![v.hit(PatternKey::JiJuMaoYou, s, vec![(s, zi), (s, tan)])],
        _ => vec![],
    }
}

/// 机月同梁：两种口径独立判定、命中即报。
///
/// - `variant = None`：命宫在寅或申，宫内为天同天梁或天机太阴（空宫借对宫主星）——
///   页面作者穷举的八种情形，按古书「命在寅申方论」为主口径；
/// - `variant = "surround"`：机月同梁四星齐见命宫三方四正（含借宫）——页面首句的宽口径。
///   它与主口径相交而不互含：天机与天同恒隔四宫、太阴与天梁亦恒隔四宫，四星分属两组三合，
///   命宫落在四星所在两组三合的其余宫位时四星照样齐见，主星却不是同梁或机阴；
///   反之命宫实坐同梁或机阴时四星齐见、两口径并报，而主口径经借宫成立时
///   四星未必齐见三方四正，可只报主口径。
///
/// 来源：「梁同机月寅申位一生利业聪明」「机月同梁作吏人 命在寅申方论」。
pub fn ji_yue_tong_liang(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let mut hits = Vec::new();
    if matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen)
        && let Some(stars) = [TONG_LIANG, JI_YIN].into_iter().find_map(|pair| {
            pair.iter()
                .map(|k| v.find_major_at(s, *k))
                .collect::<Option<Vec<_>>>()
        })
    {
        hits.push(v.hit(PatternKey::JiYueTongLiang, s, stars));
    }
    let wide: Option<Vec<_>> = JYTL
        .iter()
        .map(|k| {
            v.surround(s)
                .into_iter()
                .find_map(|p| v.find_major_at(p, *k))
        })
        .collect();
    if let Some(stars) = wide {
        let mut hit = v.hit(PatternKey::JiYueTongLiang, s, stars);
        hit.variant = Some("surround");
        hits.push(hit);
    }
    hits
}

/// 善荫朝纲：天机、天梁同守命宫或身宫（只可能在辰、戌）。
///
/// 来源：「善荫朝纲，仁慈之长」「机梁守命，加吉曜，富贵慈祥 加刑忌，僧道」。
/// 古书多处作「机梁同照命身」，故命宫、身宫各判一次，命中哪宫 `palace` 即记哪宫。
pub fn shan_yin_chao_gang(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter_map(|p| {
            let (ji, liang) = (
                v.find(p, StarKey::TianjiMaj)?,
                v.find(p, StarKey::TianliangMaj)?,
            );
            Some(v.hit(PatternKey::ShanYinChaoGang, p, vec![(p, ji), (p, liang)]))
        })
        .collect()
}

/// 机巨同临：天机、巨门同守命宫（只可能在卯、酉）。
///
/// 来源：「巨机同宫公卿之位」。另有资料称酉宫的巨机不算此格，故酉宫命中标
/// `variant = "you"` 供取舍，卯宫无异议不标；卯宫的严口径另见 [`ji_ju_ju_mao`]。
pub fn ji_ju_tong_lin(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(ji), Some(ju)) = (v.find(s, StarKey::TianjiMaj), v.find(s, StarKey::JumenMaj)) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::JiJuTongLin, s, vec![(s, ji), (s, ju)]);
    if v.branch(s) == EarthlyBranch::You {
        hit.variant = Some("you");
    }
    vec![hit]
}

/// 机巨居卯：天机、巨门同守命宫于卯。
///
/// 来源：「卯宫机巨武曲逢，辛乙生人福气隆」。
pub fn ji_ju_ju_mao(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if v.branch(s) != EarthlyBranch::Mao {
        return vec![];
    }
    match (v.find(s, StarKey::TianjiMaj), v.find(s, StarKey::JumenMaj)) {
        (Some(ji), Some(ju)) => vec![v.hit(PatternKey::JiJuJuMao, s, vec![(s, ji), (s, ju)])],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::{Palace, Scope};
    use crate::models::astrolabe::Astrolabe;
    use crate::pattern::PatternConfig;
    use crate::pattern::testutil::{find_chart, find_charts};

    fn view(a: &Astrolabe) -> ChartView<'_> {
        ChartView::natal(a, &PatternConfig::default())
    }

    /// 宫内是否有集合任一主星（空宫借对宫），搜盘条件用。
    fn any_major(v: &ChartView, i: usize, keys: &[StarKey]) -> bool {
        keys.iter().any(|k| v.find_major_at(i, *k).is_some())
    }

    /// 君臣庆会形式 C 的四星条件：机、阴、同、梁俱全分布于命宫前后两宫（借宫补空侧）。
    fn tian_fu_four(v: &ChartView, s: usize) -> bool {
        let (p, n) = (v.prev(s), v.next(s));
        JYTL.iter().all(|k| {
            v.find_major_at(p, *k)
                .or_else(|| v.find_major_at(n, *k))
                .is_some()
        })
    }

    fn soul_index(a: &Astrolabe) -> usize {
        a.palaces
            .iter()
            .position(|p| p.name == Palace::Soul)
            .unwrap()
    }

    /// 命中某格的全部 `variant`。
    fn variants(a: &Astrolabe, key: PatternKey) -> Vec<Option<&'static str>> {
        a.patterns()
            .iter()
            .filter(|h| h.key == key)
            .map(|h| h.variant)
            .collect()
    }

    #[test]
    fn jun_chen_qing_hui_zi_po_zuo_you_jia() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.no_sha(s)
                && v.has(s, StarKey::ZiweiMaj)
                && v.has(s, StarKey::PojunMaj)
                && v.jia(s, &[StarKey::ZuofuMin], &[StarKey::YoubiMin])
                    .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hits = a.patterns();
        let hit = hits
            .iter()
            .find(|h| h.key == PatternKey::JunChenQingHui && h.variant == Some("zi_po_zuo_you_jia"))
            .expect("hit");
        assert_eq!(hit.palace, s);
        assert_eq!(hit.scope, Scope::Origin);
        assert_eq!(hit.stars.len(), 4);
        // 辅弼各在命宫一侧
        let sides: Vec<_> = hit
            .stars
            .iter()
            .filter(|st| matches!(st.star, StarKey::ZuofuMin | StarKey::YoubiMin))
            .map(|st| st.palace)
            .collect();
        assert!(sides.contains(&v.prev(s)) && sides.contains(&v.next(s)));
    }

    #[test]
    fn jun_chen_qing_hui_zi_xiang_chang_qu_axis() {
        let a = find_chart(|a| {
            let v = view(a);
            let (s, o) = (v.soul(), v.opposite(v.soul()));
            v.no_sha(s)
                && v.has(s, StarKey::ZiweiMaj)
                && v.has(s, StarKey::TianxiangMaj)
                && ((v.has(s, StarKey::WenchangMin) && v.has(o, StarKey::WenquMin))
                    || (v.has(s, StarKey::WenquMin) && v.has(o, StarKey::WenchangMin)))
        });
        let v = view(&a);
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| {
                h.key == PatternKey::JunChenQingHui && h.variant == Some("zi_xiang_chang_qu_axis")
            })
            .expect("hit");
        let chang = hit
            .stars
            .iter()
            .find(|st| st.star == StarKey::WenchangMin)
            .unwrap();
        let qu = hit
            .stars
            .iter()
            .find(|st| st.star == StarKey::WenquMin)
            .unwrap();
        assert_eq!(qu.palace, v.opposite(chang.palace));
    }

    #[test]
    fn jun_chen_qing_hui_tian_fu_needs_all_four_across_flanks() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.no_sha(s) && v.has(s, StarKey::TianfuMaj) && tian_fu_four(&v, s)
        });
        let v = view(&a);
        let s = v.soul();
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| {
                h.key == PatternKey::JunChenQingHui
                    && h.variant == Some("tian_fu_ji_yin_tong_liang_jia")
            })
            .expect("hit");
        // 四星俱全只出现在天府巳亥守命的布局
        assert!(matches!(
            v.branch(s),
            EarthlyBranch::Si | EarthlyBranch::Hai
        ));
        assert_eq!(hit.stars.len(), 5);
        assert_eq!(hit.stars[0].star, StarKey::TianfuMaj);
        // 借来的星记在它真正的落宫（夹宫的对宫），故落宫不必是相邻宫
        assert!(hit.stars[1..].iter().all(|st| {
            let d = (st.palace + 12 - s) % 12;
            [1, 11, 5, 7].contains(&d)
        }));
    }

    /// 反例：一侧机或阴、另一侧同或梁的「集合任一相夹」不足以成形式 C ——
    /// 安星几何下太阴恒在天府后一宫，该形态在天府守命盘上大量出现，四星俱全才算。
    #[test]
    fn jun_chen_qing_hui_tian_fu_rejects_partial_pairs() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            let (p, n) = (v.prev(s), v.next(s));
            v.no_sha(s)
                && v.has(s, StarKey::TianfuMaj)
                && (any_major(&v, p, &JI_YIN) && any_major(&v, n, &TONG_LIANG)
                    || any_major(&v, p, &TONG_LIANG) && any_major(&v, n, &JI_YIN))
                && !tian_fu_four(&v, s)
        });
        assert!(!a.patterns().iter().any(|h| {
            h.key == PatternKey::JunChenQingHui
                && h.variant == Some("tian_fu_ji_yin_tong_liang_jia")
        }));
    }

    /// 形式 C 命中的盘全部收在天府巳亥守命布局内（枚举抽样验证安星几何结论）。
    #[test]
    fn jun_chen_qing_hui_tian_fu_hits_only_si_hai() {
        let hits = find_charts(8, |a| {
            a.patterns().iter().any(|h| {
                h.key == PatternKey::JunChenQingHui
                    && h.variant == Some("tian_fu_ji_yin_tong_liang_jia")
            })
        });
        for a in &hits {
            let v = view(a);
            assert!(matches!(
                v.branch(v.soul()),
                EarthlyBranch::Si | EarthlyBranch::Hai
            ));
            assert!(v.has(v.soul(), StarKey::TianfuMaj));
        }
    }

    #[test]
    fn jun_chen_qing_hui_zi_zuo_you_tong_gong() {
        // 紫微与辅弼同宫只在丑未宫、亥时（1950-2020 全域实测），地空恒落命迁线，
        // 故此形式永远带 broken；不成格的判法会让它变成死规则。
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(
                v.soul(),
                &[StarKey::ZiweiMaj, StarKey::ZuofuMin, StarKey::YoubiMin],
            )
        });
        let v = view(&a);
        let s = v.soul();
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| {
                h.key == PatternKey::JunChenQingHui && h.variant == Some("zi_zuo_you_tong_gong")
            })
            .expect("hit");
        assert!(hit.stars.iter().all(|st| st.palace == s));
        assert_eq!(hit.broken, !v.no_sha(s));
        assert!(matches!(
            v.branch(s),
            EarthlyBranch::Chou | EarthlyBranch::Wei
        ));
    }

    #[test]
    fn jun_chen_qing_hui_needs_clean_surround() {
        // 紫破坐命、辅弼来夹，但三方四正见煞忌：作者的三形式一个都不报
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            !v.no_sha(s)
                && v.has(s, StarKey::ZiweiMaj)
                && v.has(s, StarKey::PojunMaj)
                && v.jia(s, &[StarKey::ZuofuMin], &[StarKey::YoubiMin])
                    .is_some()
        });
        assert!(!a.patterns().iter().any(|h| {
            h.key == PatternKey::JunChenQingHui && h.variant != Some("zi_zuo_you_tong_gong")
        }));
    }

    #[test]
    fn zi_fu_tong_gong_hits_only_when_both_in_soul() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(v.soul(), &[StarKey::ZiweiMaj, StarKey::TianfuMaj])
        });
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::ZiFuTongGong)
            .expect("hit");
        assert_eq!(hit.palace, soul_index(&a));
        assert!(matches!(
            view(&a).branch(hit.palace),
            EarthlyBranch::Yin | EarthlyBranch::Shen
        ));

        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::ZiweiMaj) && !v.has(v.soul(), StarKey::TianfuMaj)
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::ZiFuTongGong)
        );
    }

    #[test]
    fn jin_yu_fu_jia_needs_tianfu_in_chou_or_wei_jia_by_sun_moon() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            matches!(v.branch(s), EarthlyBranch::Chou | EarthlyBranch::Wei)
                && v.has(s, StarKey::TianfuMaj)
                && v.jia(s, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                    .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JinYuFuJia)
            .expect("hit");
        assert_eq!(hit.palace, s);
        let mut sides: Vec<_> = hit
            .stars
            .iter()
            .filter(|st| st.star != StarKey::TianfuMaj)
            .map(|st| st.palace)
            .collect();
        sides.sort_unstable();
        let mut want = [v.prev(s), v.next(s)];
        want.sort_unstable();
        assert_eq!(sides, want);

        // 命宫在丑未、日月来夹，但守命的不是天府：不成格
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            matches!(v.branch(s), EarthlyBranch::Chou | EarthlyBranch::Wei)
                && !v.has(s, StarKey::TianfuMaj)
                && v.jia(s, &[StarKey::TaiyangMaj], &[StarKey::TaiyinMaj])
                    .is_some()
        });
        assert!(!b.patterns().iter().any(|h| h.key == PatternKey::JinYuFuJia));
    }

    #[test]
    fn zi_fu_jia_ming_requires_ji_yin_in_soul() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(v.soul(), &[StarKey::TianjiMaj, StarKey::TaiyinMaj])
        });
        let v = view(&a);
        let s = v.soul();
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::ZiFuJiaMing)
            .expect("hit");
        assert_eq!(hit.palace, s);
        let zi = hit
            .stars
            .iter()
            .find(|st| st.star == StarKey::ZiweiMaj)
            .unwrap();
        let fu = hit
            .stars
            .iter()
            .find(|st| st.star == StarKey::TianfuMaj)
            .unwrap();
        assert!([v.prev(s), v.next(s)].contains(&zi.palace));
        assert!([v.prev(s), v.next(s)].contains(&fu.palace));

        // 紫府夹命宫、但命宫不是机阴：不成格
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.jia(s, &[StarKey::ZiweiMaj], &[StarKey::TianfuMaj])
                .is_some()
                && !v.has(s, StarKey::TianjiMaj)
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::ZiFuJiaMing)
        );
    }

    #[test]
    fn ji_xiang_li_ming_requires_wu_palace_without_sha() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.branch(s) == EarthlyBranch::Wu && v.has(s, StarKey::ZiweiMaj) && v.no_sha(s)
        });
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JiXiangLiMing)
            .expect("hit");
        assert_eq!(hit.stars.len(), 1);
        assert_eq!(hit.stars[0].star, StarKey::ZiweiMaj);

        // 紫微居午但三方四正见煞忌：不成格
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.branch(s) == EarthlyBranch::Wu && v.has(s, StarKey::ZiweiMaj) && !v.no_sha(s)
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::JiXiangLiMing)
        );
    }

    #[test]
    fn ji_ju_mao_you_requires_zi_tan_in_mao_or_you() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has_all(v.soul(), &[StarKey::ZiweiMaj, StarKey::TanlangMaj])
        });
        let v = view(&a);
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JiJuMaoYou)
            .expect("hit");
        assert!(matches!(
            v.branch(hit.palace),
            EarthlyBranch::Mao | EarthlyBranch::You
        ));

        // 贪狼守命但无紫微：不成格
        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::TanlangMaj) && !v.has(v.soul(), StarKey::ZiweiMaj)
        });
        assert!(!b.patterns().iter().any(|h| h.key == PatternKey::JiJuMaoYou));
    }

    #[test]
    fn ji_yue_tong_liang_hits_yin_shen_soul() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen)
                && !v.is_empty(s)
                && v.has_major(s, StarKey::TiantongMaj)
                && v.has_major(s, StarKey::TianliangMaj)
        });
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JiYueTongLiang && h.variant.is_none())
            .expect("hit");
        assert_eq!(hit.palace, soul_index(&a));
        assert_eq!(hit.stars.len(), 2);
        // 命宫实坐同梁时四星齐见三方四正，宽口径命中随之并报
        assert_eq!(
            variants(&a, PatternKey::JiYueTongLiang),
            vec![None, Some("surround")]
        );

        // 命宫在寅申但主星是杀破狼、四星也不齐见三方四正：两口径都不成格
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen)
                && v.has_major(s, StarKey::QishaMaj)
                && !JYTL
                    .iter()
                    .all(|k| v.surround(s).iter().any(|p| v.has_major(*p, *k)))
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::JiYueTongLiang)
        );
    }

    #[test]
    fn ji_yue_tong_liang_borrows_from_opposite() {
        // 命宫空宫（寅申），借对宫机阴或同梁：成格，证据记在星真正的落宫（对宫）
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen)
                && v.is_empty(s)
                && v.has_major(s, StarKey::TianjiMaj)
                && v.has_major(s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let s = v.soul();
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JiYueTongLiang && h.variant.is_none())
            .expect("hit");
        assert!(hit.stars.iter().all(|st| st.palace == v.opposite(s)));
    }

    /// 宽口径单独命中：四星齐见命宫三方四正、命宫却不在寅申，只报 `surround`。
    #[test]
    fn ji_yue_tong_liang_surround_variant_outside_yin_shen() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            !matches!(v.branch(s), EarthlyBranch::Yin | EarthlyBranch::Shen)
                && JYTL
                    .iter()
                    .all(|k| v.surround(s).iter().any(|p| v.has_major(*p, *k)))
        });
        assert_eq!(
            variants(&a, PatternKey::JiYueTongLiang),
            vec![Some("surround")]
        );
        let v = view(&a);
        let hit = a
            .patterns()
            .into_iter()
            .find(|h| h.key == PatternKey::JiYueTongLiang)
            .expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.stars.len(), 4);
    }

    #[test]
    fn shan_yin_chao_gang_covers_soul_and_body() {
        let a = find_chart(|a| {
            let v = view(a);
            v.soul_and_body()
                .iter()
                .any(|p| v.has_all(*p, &[StarKey::TianjiMaj, StarKey::TianliangMaj]))
        });
        let v = view(&a);
        let hits: Vec<_> = a
            .patterns()
            .into_iter()
            .filter(|h| h.key == PatternKey::ShanYinChaoGang)
            .collect();
        assert!(!hits.is_empty());
        for h in &hits {
            assert!(v.soul_and_body().contains(&h.palace));
            assert!(matches!(
                v.branch(h.palace),
                EarthlyBranch::Chen | EarthlyBranch::Xu
            ));
            assert!(h.stars.iter().all(|st| st.palace == h.palace));
        }

        // 天机守命而命宫身宫皆无天梁：不成格
        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::TianjiMaj)
                && v.soul_and_body()
                    .iter()
                    .all(|p| !v.has(*p, StarKey::TianliangMaj))
        });
        assert!(
            !b.patterns()
                .iter()
                .any(|h| h.key == PatternKey::ShanYinChaoGang)
        );
    }

    #[test]
    fn ji_ju_tong_lin_marks_you_variant() {
        let mao = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.branch(s) == EarthlyBranch::Mao
                && v.has_all(s, &[StarKey::TianjiMaj, StarKey::JumenMaj])
        });
        assert_eq!(variants(&mao, PatternKey::JiJuTongLin), vec![None]);
        assert_eq!(variants(&mao, PatternKey::JiJuJuMao), vec![None]);

        let you = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.branch(s) == EarthlyBranch::You
                && v.has_all(s, &[StarKey::TianjiMaj, StarKey::JumenMaj])
        });
        assert_eq!(variants(&you, PatternKey::JiJuTongLin), vec![Some("you")]);
        // 机巨居卯只认卯宫
        assert!(variants(&you, PatternKey::JiJuJuMao).is_empty());

        // 巨门守命而无天机：两格都不成
        let b = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::JumenMaj) && !v.has(v.soul(), StarKey::TianjiMaj)
        });
        assert!(variants(&b, PatternKey::JiJuTongLin).is_empty());
        assert!(variants(&b, PatternKey::JiJuJuMao).is_empty());
    }
}

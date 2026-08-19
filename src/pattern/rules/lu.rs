//! 禄存与四化系格局（《格局》页 #37、#40-#46，含行运格禄衰马困、风云际会）。

use crate::data::stars::StarKey;
use crate::data::types::{Mutagen, Palace, Scope};
use crate::models::star::Star;
use crate::pattern::view::{KONG_WANG, KONG_YAO};
use crate::pattern::{ChartView, PatternHit, PatternKey, Rule, StarAt};

/// 本组规则，按《格局》页顺序。
pub static RULES: &[Rule] = &[
    Rule {
        key: PatternKey::LuShuaiMaKun,
        detect: lu_shuai_ma_kun,
    },
    Rule {
        key: PatternKey::SanQiJiaHui,
        detect: san_qi_jia_hui,
    },
    Rule {
        key: PatternKey::LuMaJiaoChi,
        detect: lu_ma_jiao_chi,
    },
    Rule {
        key: PatternKey::LuHeYuanYang,
        detect: lu_he_yuan_yang,
    },
    Rule {
        key: PatternKey::MingLuAnLu,
        detect: ming_lu_an_lu,
    },
    Rule {
        key: PatternKey::LuMaPeiYin,
        detect: lu_ma_pei_yin,
    },
    Rule {
        key: PatternKey::LiangChongHuaGai,
        detect: liang_chong_hua_gai,
    },
    Rule {
        key: PatternKey::FengYunJiHui,
        detect: feng_yun_ji_hui,
    },
];

/// 困住天马的煞忌：擎羊、陀罗、天刑、阴煞（化忌另按视角四化判）。
const SHA_JI_MA: [StarKey; 4] = [
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
    StarKey::Tianxing,
    StarKey::Yinsha,
];

/// 耗曜：大耗、小耗、地劫。大耗小耗在默认派是博士十二神（每宫恰一神），
/// 中州派另把大耗作杂耀安入，两种落法都算。
const HAO_YAO: [StarKey; 3] = [StarKey::Dahao, StarKey::Xiaohao, StarKey::DijieMin];

/// 禄衰马困：运限命宫三方四正内，禄存与空曜或耗曜同宫，同时天马与煞忌同宫。
///
/// 来源：「禄衰马困 限逢七杀禄马空亡是也」；作者展开为「通常是三方四正内禄存和空曜
/// （天空，地空，截空，旬空）或者耗曜（大耗，小耗，地劫）同宫的同时天马也和煞忌
/// （化忌，擎羊，陀罗，天刑，阴煞）同宫」。口径随作者展开：不要求见七杀；
/// 禄存与天马含该层流禄流马（视图 `flow_stars` 开关）；化忌取该层四化。
/// 判定层级即视图当前层（大限视图判大限、流年视图判流年）。
pub fn lu_shuai_ma_kun(v: &ChartView) -> Vec<PatternHit> {
    if v.scope() == Scope::Origin {
        return vec![];
    }
    let l = v.soul();
    let Some((p, lucun, decay)) = v.surround(l).into_iter().find_map(|p| {
        let lucun = v.find(p, StarKey::LucunMin)?;
        Some((p, lucun, decayed(v, p)?))
    }) else {
        return vec![];
    };
    let Some((q, ma, trap)) = v.surround(l).into_iter().find_map(|q| {
        let ma = v.find(q, StarKey::TianmaMin)?;
        Some((q, ma, trapped(v, q)?))
    }) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::LuShuaiMaKun, l, vec![(p, lucun)]);
    hit.stars.push(decay);
    hit.stars.push(v.star_at(q, ma));
    hit.stars.push(v.star_at(q, trap));
    vec![hit]
}

/// 禄存所在宫的「衰」：同宫的空曜或耗曜，作为证据返回。
fn decayed(v: &ChartView, i: usize) -> Option<StarAt> {
    if let Some(s) = KONG_YAO.iter().chain(&HAO_YAO).find_map(|k| v.find(i, *k)) {
        return Some(v.star_at(i, s));
    }
    let boshi = v.boshi12(i);
    matches!(boshi, StarKey::Dahao | StarKey::Xiaohao).then(|| StarAt {
        star: boshi,
        palace: i,
        brightness: None,
        mutagen: None,
    })
}

/// 天马所在宫的「困」：同宫的煞忌（擎羊、陀罗、天刑、阴煞或该层化忌）。
fn trapped<'a>(v: &ChartView<'a>, i: usize) -> Option<&'a Star> {
    SHA_JI_MA
        .iter()
        .find_map(|k| v.find(i, *k))
        .or_else(|| v.mutagen_star(i, Mutagen::Ji))
}

/// 三奇加会：化禄、化权、化科齐聚命宫三方四正。
///
/// 来源：作者自述全书无此格；「三奇」= 禄权科。子形态「命宫化科、财帛化禄、官禄化权」
/// 以 `variant = "ke_soul_lu_wealth_quan_career"` 标出。
pub fn san_qi_jia_hui(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(lu), Some(quan), Some(ke)) = (
        v.find_mutagen_in_surround(s, Mutagen::Lu),
        v.find_mutagen_in_surround(s, Mutagen::Quan),
        v.find_mutagen_in_surround(s, Mutagen::Ke),
    ) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::SanQiJiaHui, s, vec![lu, quan, ke]);
    if ke.0 == s && lu.0 == v.index_of(Palace::Wealth) && quan.0 == v.index_of(Palace::Career) {
        hit.variant = Some("ke_soul_lu_wealth_quan_career");
    }
    vec![hit]
}

/// 禄马交驰：禄存与天马同宫（天马只在寅申巳亥，故成格宫必是四生地）。
///
/// 来源：「天马如与禄存同宫，谓之禄马交驰，又曰折鞭马」。宫位不限命宫 ——
/// 页面说的是「禄马交驰的宫位」，任一宫同宫即成格，`palace` 记该宫，可有多个命中。
/// 采希夷「同宫」说，三方会照不算。
pub fn lu_ma_jiao_chi(v: &ChartView) -> Vec<PatternHit> {
    (0..12)
        .filter_map(|i| {
            let lucun = v.find(i, StarKey::LucunMin)?;
            let ma = v.find(i, StarKey::TianmaMin)?;
            Some(v.hit(PatternKey::LuMaJiaoChi, i, vec![(i, lucun), (i, ma)]))
        })
        .collect()
}

/// 禄合鸳鸯：命宫的禄存与化禄成双 —— 同宫，或一在命宫、一在迁移宫对拱。
///
/// 来源：「合禄鸳鸯一世荣」。宫位取命宫（页面未写宫位时与其它格一致）。
/// `variant = None` 为同宫，`variant = "opposite"` 为对拱。
pub fn lu_he_yuan_yang(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let o = v.opposite(s);
    let (cun, hua) = (v.find(s, StarKey::LucunMin), v.mutagen_star(s, Mutagen::Lu));
    if let (Some(cun), Some(hua)) = (cun, hua) {
        return vec![v.hit(PatternKey::LuHeYuanYang, s, vec![(s, cun), (s, hua)])];
    }
    let pair = match (cun, hua) {
        (Some(cun), _) => v
            .mutagen_star(o, Mutagen::Lu)
            .map(|hua| vec![(s, cun), (o, hua)]),
        (_, Some(hua)) => v
            .find(o, StarKey::LucunMin)
            .map(|cun| vec![(s, hua), (o, cun)]),
        _ => None,
    };
    pair.map(|stars| {
        let mut hit = v.hit(PatternKey::LuHeYuanYang, s, stars);
        hit.variant = Some("opposite");
        vec![hit]
    })
    .unwrap_or_default()
}

/// 明禄暗禄：命宫有禄存（或化禄），暗合宫有化禄（或禄存）。
///
/// 来源：「明禄暗禄 命宫中有化禄（或禄存）坐守，而暗合宫中有禄存（或化禄）是也」。
pub fn ming_lu_an_lu(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let h = v.hidden(s);
    let mut hits = vec![];
    if let (Some(cun), Some(hua)) = (v.find(s, StarKey::LucunMin), v.mutagen_star(h, Mutagen::Lu)) {
        hits.push(v.hit(PatternKey::MingLuAnLu, s, vec![(s, cun), (h, hua)]));
    } else if let (Some(hua), Some(cun)) =
        (v.mutagen_star(s, Mutagen::Lu), v.find(h, StarKey::LucunMin))
    {
        hits.push(v.hit(PatternKey::MingLuAnLu, s, vec![(s, hua), (h, cun)]));
    }
    hits
}

/// 禄马佩印：命宫禄存、天马、天相三星同宫。
///
/// 来源：「禄马佩印 马前有禄印星同宫是也」「天相禄马交驰，不落空亡，更坐生乡，
/// 可为贵论」。宫位取命宫；「不落空亡」不作成格条件，命宫见空亡星时 `broken = true`。
pub fn lu_ma_pei_yin(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(cun), Some(ma), Some(xiang)) = (
        v.find(s, StarKey::LucunMin),
        v.find(s, StarKey::TianmaMin),
        v.find(s, StarKey::TianxiangMaj),
    ) else {
        return vec![];
    };
    let mut hit = v.hit(
        PatternKey::LuMaPeiYin,
        s,
        vec![(s, cun), (s, ma), (s, xiang)],
    );
    hit.broken = v.has_any(s, &KONG_WANG);
    vec![hit]
}

/// 两重华盖：命宫禄存与化禄双禄坐守，又见地空或地劫。
///
/// 来源：「两重华盖 谓禄存化禄坐命遇空劫是也」。空曜只取地空地劫（原文即「空劫」），
/// 天空截空旬空不算。
pub fn liang_chong_hua_gai(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(cun), Some(hua)) = (v.find(s, StarKey::LucunMin), v.mutagen_star(s, Mutagen::Lu))
    else {
        return vec![];
    };
    let Some(kong) = [StarKey::DikongMin, StarKey::DijieMin]
        .iter()
        .find_map(|k| v.find(s, *k))
    else {
        return vec![];
    };
    vec![v.hit(
        PatternKey::LiangChongHuaGai,
        s,
        vec![(s, cun), (s, hua), (s, kong)],
    )]
}

/// 风云际会：大限与另一限同时逢禄马 —— 两限命宫三方四正各自见禄存、天马或化禄。
///
/// 来源：「风云际会，身命虽弱二限逢禄马是也」；作者：「本命身宫或命宫较弱，而大限和小限
/// （也有流派取大限与流年）同时走到禄存星、化禄星或天马星所在的有利位置」。
/// 「二限」`variant = None` 取大限 + 小限（小限层无自身四化，化禄按大限四化判），
/// `variant = "yearly"` 取大限 + 流年（该层四化与流曜按流年层）；两者都成立则报两个命中。
/// 只在大限视角判定一次，`palace` 为大限命宫。
/// 「较弱」页面未给定义，此处不判，由调用方按自己的口径决定是否采用。
pub fn feng_yun_ji_hui(v: &ChartView) -> Vec<PatternHit> {
    if v.scope() != Scope::Decadal {
        return vec![];
    }
    let (Some(h), Some((dp, ds))) = (v.horoscope(), meets_lu_ma(v, v.soul())) else {
        return vec![];
    };
    let mut hits = vec![];
    if let Some((ap, age)) = meets_lu_ma(v, h.age.index) {
        hits.push(v.hit(
            PatternKey::FengYunJiHui,
            v.soul(),
            vec![(dp, ds), (ap, age)],
        ));
    }
    let yearly = ChartView::at(v.astrolabe(), h, Scope::Yearly, v.config());
    if let Some((yp, ys)) = meets_lu_ma(&yearly, yearly.soul()) {
        let mut hit = v.hit(PatternKey::FengYunJiHui, v.soul(), vec![(dp, ds)]);
        hit.stars.push(yearly.star_at(yp, ys));
        hit.variant = Some("yearly");
        hits.push(hit);
    }
    hits
}

/// 某限命宫三方四正内「逢禄马」的证据：禄存、天马或化禄，取先见到的一个。
fn meets_lu_ma<'a>(v: &ChartView<'a>, i: usize) -> Option<(usize, &'a Star)> {
    v.find_in_surround(i, StarKey::LucunMin)
        .or_else(|| v.find_in_surround(i, StarKey::TianmaMin))
        .or_else(|| v.find_mutagen_in_surround(i, Mutagen::Lu))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::types::EarthlyBranch;
    use crate::models::astrolabe::Astrolabe;
    use crate::models::horoscope::HoroscopeRef;
    use crate::pattern::PatternConfig;
    use crate::pattern::testutil::find_chart;

    /// 本命视图，默认口径。
    fn natal(a: &Astrolabe) -> ChartView<'_> {
        ChartView::natal(a, &PatternConfig::default())
    }

    /// 测试统一的运限查询日期与时辰。
    fn horo(a: &Astrolabe) -> HoroscopeRef<'_> {
        a.horoscope("2026-8-19", 3).unwrap()
    }

    /// 大限视图，默认口径。
    fn decadal<'a>(h: &'a HoroscopeRef<'a>) -> ChartView<'a> {
        ChartView::at(
            h.astrolabe(),
            h.data(),
            Scope::Decadal,
            &PatternConfig::default(),
        )
    }

    #[test]
    fn san_qi_jia_hui_needs_all_three_mutagens() {
        let a = find_chart(|a| {
            let v = natal(a);
            [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke]
                .iter()
                .all(|m| v.find_mutagen_in_surround(v.soul(), *m).is_some())
        });
        let v = natal(&a);
        let hits = san_qi_jia_hui(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].palace, v.soul());
        assert_eq!(hits[0].stars.len(), 3);
        let ms: Vec<_> = hits[0].stars.iter().map(|s| s.mutagen).collect();
        assert_eq!(
            ms,
            vec![Some(Mutagen::Lu), Some(Mutagen::Quan), Some(Mutagen::Ke)]
        );

        // 差一奇（无化科会照）即不成格
        let b = find_chart(|a| {
            let v = natal(a);
            v.find_mutagen_in_surround(v.soul(), Mutagen::Lu).is_some()
                && v.find_mutagen_in_surround(v.soul(), Mutagen::Quan)
                    .is_some()
                && v.find_mutagen_in_surround(v.soul(), Mutagen::Ke).is_none()
        });
        assert!(san_qi_jia_hui(&natal(&b)).is_empty());
    }

    #[test]
    fn san_qi_jia_hui_marks_ke_soul_lu_wealth_quan_career() {
        let a = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has_mutagen(s, Mutagen::Ke)
                && v.has_mutagen(v.index_of(Palace::Wealth), Mutagen::Lu)
                && v.has_mutagen(v.index_of(Palace::Career), Mutagen::Quan)
        });
        let hits = san_qi_jia_hui(&natal(&a));
        assert_eq!(hits[0].variant, Some("ke_soul_lu_wealth_quan_career"));
    }

    #[test]
    fn lu_ma_jiao_chi_requires_same_palace() {
        let a = find_chart(|a| {
            let v = natal(a);
            (0..12).any(|i| v.has(i, StarKey::LucunMin) && v.has(i, StarKey::TianmaMin))
        });
        let v = natal(&a);
        let hits = lu_ma_jiao_chi(&v);
        assert_eq!(hits.len(), 1);
        let hit = &hits[0];
        // 天马只落四生地，故成格宫必是寅申巳亥
        assert!(matches!(
            v.branch(hit.palace),
            EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Si | EarthlyBranch::Hai
        ));
        assert!(hit.stars.iter().all(|s| s.palace == hit.palace));
        let keys: Vec<_> = hit.stars.iter().map(|s| s.star).collect();
        assert_eq!(keys, vec![StarKey::LucunMin, StarKey::TianmaMin]);

        // 禄存与天马都在盘上但不同宫，不算交驰
        let b = find_chart(|a| {
            let v = natal(a);
            let (Some((lu, _)), Some((ma, _))) =
                (v.locate(StarKey::LucunMin), v.locate(StarKey::TianmaMin))
            else {
                return false;
            };
            lu != ma
        });
        assert!(lu_ma_jiao_chi(&natal(&b)).is_empty());
    }

    #[test]
    fn lu_he_yuan_yang_same_palace() {
        let a = find_chart(|a| {
            let v = natal(a);
            v.has(v.soul(), StarKey::LucunMin) && v.has_mutagen(v.soul(), Mutagen::Lu)
        });
        let v = natal(&a);
        let hits = lu_he_yuan_yang(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, None);
        assert_eq!(hits[0].palace, v.soul());
        assert!(hits[0].stars.iter().all(|s| s.palace == v.soul()));
    }

    #[test]
    fn lu_he_yuan_yang_opposite_palace() {
        let a = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && !v.has_mutagen(s, Mutagen::Lu)
                && v.has_mutagen(v.opposite(s), Mutagen::Lu)
        });
        let v = natal(&a);
        let hits = lu_he_yuan_yang(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("opposite"));
        assert_eq!(hits[0].stars[1].palace, v.opposite(v.soul()));

        // 禄存坐命而化禄既不同宫也不对拱：不成格
        let b = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && !v.has_mutagen(s, Mutagen::Lu)
                && !v.has_mutagen(v.opposite(s), Mutagen::Lu)
        });
        assert!(lu_he_yuan_yang(&natal(&b)).is_empty());
    }

    #[test]
    fn ming_lu_an_lu_pairs_soul_with_hidden_palace() {
        let a = find_chart(|a| {
            let v = natal(a);
            v.has(v.soul(), StarKey::LucunMin) && v.has_mutagen(v.hidden(v.soul()), Mutagen::Lu)
        });
        let v = natal(&a);
        let hits = ming_lu_an_lu(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].palace, v.soul());
        assert_eq!(hits[0].stars[1].palace, v.hidden(v.soul()));

        // 禄存坐命而暗合宫无化禄，命宫也无化禄：不成格
        let b = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && !v.has_mutagen(v.hidden(s), Mutagen::Lu)
                && !v.has_mutagen(s, Mutagen::Lu)
        });
        assert!(ming_lu_an_lu(&natal(&b)).is_empty());
    }

    #[test]
    fn lu_ma_pei_yin_requires_all_three_in_soul() {
        let a = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has(s, StarKey::TianmaMin)
                && v.has(s, StarKey::TianxiangMaj)
        });
        let v = natal(&a);
        let hits = lu_ma_pei_yin(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].palace, v.soul());
        assert_eq!(hits[0].stars.len(), 3);
        assert_eq!(hits[0].broken, v.has_any(v.soul(), &KONG_WANG));

        // 禄马同守命宫但无天相：不成格
        let b = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has(s, StarKey::TianmaMin)
                && !v.has(s, StarKey::TianxiangMaj)
        });
        assert!(lu_ma_pei_yin(&natal(&b)).is_empty());
    }

    #[test]
    fn liang_chong_hua_gai_requires_double_lu_and_kong_jie() {
        let a = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has_mutagen(s, Mutagen::Lu)
                && v.has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
        });
        let v = natal(&a);
        let hits = liang_chong_hua_gai(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].palace, v.soul());
        assert!(matches!(
            hits[0].stars[2].star,
            StarKey::DikongMin | StarKey::DijieMin
        ));

        // 双禄坐命但不见空劫：不成格
        let b = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has_mutagen(s, Mutagen::Lu)
                && !v.has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
        });
        assert!(liang_chong_hua_gai(&natal(&b)).is_empty());
    }

    #[test]
    fn lu_shuai_ma_kun_needs_both_lu_decayed_and_ma_trapped() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            !lu_shuai_ma_kun(&decadal(&h)).is_empty()
        });
        let h = horo(&a);
        let v = decadal(&h);
        let hits = lu_shuai_ma_kun(&v);
        assert_eq!(hits.len(), 1);
        let hit = &hits[0];
        assert_eq!(hit.scope, Scope::Decadal);
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.stars.len(), 4);
        // 流曜等同本命辅星参与判定，故禄/马可能是运禄流禄、运马流马
        assert!(matches!(
            hit.stars[0].star,
            StarKey::LucunMin | StarKey::Yunlu | StarKey::Liulu
        ));
        assert!(matches!(
            hit.stars[2].star,
            StarKey::TianmaMin | StarKey::Yunma | StarKey::Liuma
        ));
        // 禄存与其衰曜同宫、天马与其困曜同宫，四颗都在限命宫三方四正内
        assert_eq!(hit.stars[0].palace, hit.stars[1].palace);
        assert_eq!(hit.stars[2].palace, hit.stars[3].palace);
        let surround = v.surround(v.soul());
        assert!(hit.stars.iter().all(|s| surround.contains(&s.palace)));
        // 行运格不在本命视图报出
        assert!(
            !a.patterns()
                .iter()
                .any(|x| x.key == PatternKey::LuShuaiMaKun)
        );
        assert!(
            h.patterns(Scope::Decadal)
                .iter()
                .any(|x| x.key == PatternKey::LuShuaiMaKun)
        );

        // 大限三方有「禄衰」而天马未被煞忌困：不成格
        let b = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            let l = v.soul();
            v.surround(l)
                .into_iter()
                .any(|p| v.has(p, StarKey::LucunMin) && decayed(&v, p).is_some())
                && v.surround(l)
                    .into_iter()
                    .all(|q| !v.has(q, StarKey::TianmaMin) || trapped(&v, q).is_none())
        });
        let hb = horo(&b);
        assert!(lu_shuai_ma_kun(&decadal(&hb)).is_empty());
    }

    #[test]
    fn feng_yun_ji_hui_pairs_decadal_with_age_or_yearly() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            meets_lu_ma(&v, v.soul()).is_some() && meets_lu_ma(&v, h.age.index).is_some()
        });
        let h = horo(&a);
        let v = decadal(&h);
        let hits = feng_yun_ji_hui(&v);
        let hit = hits
            .iter()
            .find(|x| x.variant.is_none())
            .expect("decadal + age");
        assert_eq!(hit.scope, Scope::Decadal);
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.stars.len(), 2);
        assert!(v.surround(v.soul()).contains(&hit.stars[0].palace));
        assert!(v.surround(h.age.index).contains(&hit.stars[1].palace));
        // 只在大限视角报出，流年视角不重复
        assert!(
            !h.patterns(Scope::Yearly)
                .iter()
                .any(|x| x.key == PatternKey::FengYunJiHui)
        );
        assert!(
            !a.patterns()
                .iter()
                .any(|x| x.key == PatternKey::FengYunJiHui)
        );
    }

    #[test]
    fn feng_yun_ji_hui_yearly_variant() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            let y = ChartView::at(a, h.data(), Scope::Yearly, &PatternConfig::default());
            meets_lu_ma(&v, v.soul()).is_some() && meets_lu_ma(&y, y.soul()).is_some()
        });
        let h = horo(&a);
        let hits = feng_yun_ji_hui(&decadal(&h));
        let hit = hits
            .iter()
            .find(|x| x.variant == Some("yearly"))
            .expect("decadal + yearly");
        assert_eq!(hit.stars.len(), 2);
        let y = ChartView::at(&a, h.data(), Scope::Yearly, &PatternConfig::default());
        assert!(y.surround(y.soul()).contains(&hit.stars[1].palace));
    }

    #[test]
    fn feng_yun_ji_hui_absent_when_decadal_misses_lu_ma() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            meets_lu_ma(&decadal(&h), h.decadal.index).is_none()
        });
        let h = horo(&a);
        assert!(feng_yun_ji_hui(&decadal(&h)).is_empty());
    }
}

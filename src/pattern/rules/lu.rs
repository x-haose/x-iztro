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
/// （化忌，擎羊，陀罗，天刑，阴煞）同宫」。口径随作者展开：不以见七杀为成格条件；
/// 限命宫三方四正又见七杀（古书原文的「限逢七杀」严口径同时满足）时
/// `variant = "qisha"`，七杀一并记入证据。
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
    if let Some((qp, qisha)) = v.find_in_surround(l, StarKey::QishaMaj) {
        hit.variant = Some("qisha");
        hit.stars.push(v.star_at(qp, qisha));
    }
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

/// 禄马交驰：两种口径独立判定、命中即报。
///
/// - `variant = None`：禄存与天马同宫——希夷「同宫」说，为主口径。宫位不限命宫
///   （页面说的是「禄马交驰的宫位」），任一宫同宫即成格，`palace` 记该宫，可有多个命中；
///   天马只在寅申巳亥，同宫成格宫必是四生地；
/// - `variant = "surround"`：命宫三方四正内禄存、天马俱见（不必同宫）——「禄马最喜交驰」
///   的会照宽口径，`palace` 记命宫。同宫恰落命宫三方四正内时两口径并报。
///
/// 来源：「天马如与禄存同宫，谓之禄马交驰，又曰折鞭马」。
pub fn lu_ma_jiao_chi(v: &ChartView) -> Vec<PatternHit> {
    let mut hits: Vec<PatternHit> = (0..12)
        .filter_map(|i| {
            let lucun = v.find(i, StarKey::LucunMin)?;
            let ma = v.find(i, StarKey::TianmaMin)?;
            Some(v.hit(PatternKey::LuMaJiaoChi, i, vec![(i, lucun), (i, ma)]))
        })
        .collect();
    let s = v.soul();
    if let (Some(lucun), Some(ma)) = (
        v.find_in_surround(s, StarKey::LucunMin),
        v.find_in_surround(s, StarKey::TianmaMin),
    ) {
        let mut hit = v.hit(PatternKey::LuMaJiaoChi, s, vec![lucun, ma]);
        hit.variant = Some("surround");
        hits.push(hit);
    }
    hits
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

/// 禄马佩印：禄存、天马、天相三星同宫，任一宫成格，`palace` 记该宫。
///
/// 来源：「禄马佩印 马前有禄印星同宫是也」「天相禄马交驰，不落空亡，更坐生乡，
/// 可为贵论」。引文与父格禄马交驰一样未限宫位，随父格按任一宫判
/// （天马只在寅申巳亥，成格宫必在四生地）；「不落空亡」不作成格条件，
/// 该宫见空亡星时 `broken = true`。
pub fn lu_ma_pei_yin(v: &ChartView) -> Vec<PatternHit> {
    (0..12)
        .filter_map(|i| {
            let cun = v.find(i, StarKey::LucunMin)?;
            let ma = v.find(i, StarKey::TianmaMin)?;
            let xiang = v.find(i, StarKey::TianxiangMaj)?;
            let mut hit = v.hit(
                PatternKey::LuMaPeiYin,
                i,
                vec![(i, cun), (i, ma), (i, xiang)],
            );
            hit.broken = v.has_any(i, &KONG_WANG);
            Some(hit)
        })
        .collect()
}

/// 两重华盖：命宫禄存与化禄双禄坐守，又见空曜。两种口径独立判定、命中即报。
///
/// - `variant = None`：空曜取地空、地劫——原文即「空劫」，为主口径；
/// - `variant = "kong_yao"`：空曜取天空、截空、旬空——公开资料「只要是空曜都算」的宽口径
///   （[`KONG_YAO`] 四曜去掉主口径已收的地空）。
///
/// 两口径证据各记各的空曜，命宫两类空曜俱见时并报。
///
/// 来源：「两重华盖 谓禄存化禄坐命遇空劫是也」。
pub fn liang_chong_hua_gai(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(cun), Some(hua)) = (v.find(s, StarKey::LucunMin), v.mutagen_star(s, Mutagen::Lu))
    else {
        return vec![];
    };
    let mut hits = Vec::new();
    if let Some(kong) = [StarKey::DikongMin, StarKey::DijieMin]
        .iter()
        .find_map(|k| v.find(s, *k))
    {
        hits.push(v.hit(
            PatternKey::LiangChongHuaGai,
            s,
            vec![(s, cun), (s, hua), (s, kong)],
        ));
    }
    if let Some(kong) = KONG_YAO
        .iter()
        .filter(|k| **k != StarKey::DikongMin)
        .find_map(|k| v.find(s, *k))
    {
        let mut hit = v.hit(
            PatternKey::LiangChongHuaGai,
            s,
            vec![(s, cun), (s, hua), (s, kong)],
        );
        hit.variant = Some("kong_yao");
        hits.push(hit);
    }
    hits
}

/// 风云际会：大限与另一限同时逢禄马 —— 两限命宫三方四正各自见禄存、天马或化禄。
///
/// 来源：「风云际会，身命虽弱二限逢禄马是也」；作者：「本命身宫或命宫较弱，而大限和小限
/// （也有流派取大限与流年）同时走到禄存星、化禄星或天马星所在的有利位置」。
///
/// `variant` 同时承载「二限组合」与「逢的松紧」两个口径维度：
/// - `None` / `"same_palace"`：大限 + 小限；
/// - `"yearly"` / `"yearly_same_palace"`：大限 + 流年（该层四化与流曜按流年层）；
/// - 带 `same_palace` 的是「逢禄马」的同宫严口径——两限命宫皆本宫坐禄存、天马或化禄；
///   不带的是三方四正会照的宽口径。两组合都成立则报两个命中。
///
/// 小限宫的化禄按大限四化判——小限层自有按小限宫干起的四化
/// （[`crate::models::horoscope::HoroscopeData::age`] 的 `mutagen`），此处有意不用：
/// 二限逢禄马在大限视角的合成盘上判定，小限只贡献命宫位置。
/// 只在大限视角判定一次，`palace` 为大限命宫。
/// 「较弱」页面未给定义，此处不判，由调用方按自己的口径决定是否采用。
pub fn feng_yun_ji_hui(v: &ChartView) -> Vec<PatternHit> {
    if v.scope() != Scope::Decadal {
        return vec![];
    }
    let (Some(h), Some((dp, ds))) = (v.horoscope(), meets_lu_ma(v, v.soul())) else {
        return vec![];
    };
    let decadal_tight = meets_in_palace(v, v.soul());
    let mut hits = vec![];
    if let Some((ap, age)) = meets_lu_ma(v, h.age.index) {
        let mut hit = v.hit(
            PatternKey::FengYunJiHui,
            v.soul(),
            vec![(dp, ds), (ap, age)],
        );
        if decadal_tight && meets_in_palace(v, h.age.index) {
            hit.variant = Some("same_palace");
        }
        hits.push(hit);
    }
    let yearly = ChartView::at(v.astrolabe(), h, Scope::Yearly, v.config());
    if let Some((yp, ys)) = meets_lu_ma(&yearly, yearly.soul()) {
        let mut hit = v.hit(PatternKey::FengYunJiHui, v.soul(), vec![(dp, ds)]);
        hit.stars.push(yearly.star_at(yp, ys));
        hit.variant = Some(
            if decadal_tight && meets_in_palace(&yearly, yearly.soul()) {
                "yearly_same_palace"
            } else {
                "yearly"
            },
        );
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

/// 某限命宫本宫是否坐禄存、天马或化禄（「逢禄马」的同宫严口径）。
fn meets_in_palace(v: &ChartView, i: usize) -> bool {
    v.has_any(i, &[StarKey::LucunMin, StarKey::TianmaMin]) || v.has_mutagen(i, Mutagen::Lu)
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
    fn lu_ma_jiao_chi_same_palace_is_main_form() {
        let a = find_chart(|a| {
            let v = natal(a);
            (0..12).any(|i| v.has(i, StarKey::LucunMin) && v.has(i, StarKey::TianmaMin))
        });
        let v = natal(&a);
        let hits: Vec<_> = lu_ma_jiao_chi(&v)
            .into_iter()
            .filter(|h| h.variant.is_none())
            .collect();
        assert_eq!(hits.len(), 1);
        let hit = &hits[0];
        // 天马只落四生地，故同宫成格宫必是寅申巳亥
        assert!(matches!(
            v.branch(hit.palace),
            EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Si | EarthlyBranch::Hai
        ));
        assert!(hit.stars.iter().all(|s| s.palace == hit.palace));
        let keys: Vec<_> = hit.stars.iter().map(|s| s.star).collect();
        assert_eq!(keys, vec![StarKey::LucunMin, StarKey::TianmaMin]);

        // 禄存与天马不同宫且不齐见命宫三方四正：两口径都不报
        let b = find_chart(|a| {
            let v = natal(a);
            let (Some((lu, _)), Some((ma, _))) =
                (v.locate(StarKey::LucunMin), v.locate(StarKey::TianmaMin))
            else {
                return false;
            };
            let surround = v.surround(v.soul());
            lu != ma && !(surround.contains(&lu) && surround.contains(&ma))
        });
        assert!(lu_ma_jiao_chi(&natal(&b)).is_empty());
    }

    /// 禄存与天马不同宫、但齐见命宫三方四正：只报 `surround` 宽口径，`palace` 记命宫。
    #[test]
    fn lu_ma_jiao_chi_surround_variant() {
        let a = find_chart(|a| {
            let v = natal(a);
            let (Some((lu, _)), Some((ma, _))) =
                (v.locate(StarKey::LucunMin), v.locate(StarKey::TianmaMin))
            else {
                return false;
            };
            let surround = v.surround(v.soul());
            lu != ma && surround.contains(&lu) && surround.contains(&ma)
        });
        let v = natal(&a);
        let hits = lu_ma_jiao_chi(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("surround"));
        assert_eq!(hits[0].palace, v.soul());
        let surround = v.surround(v.soul());
        assert!(hits[0].stars.iter().all(|s| surround.contains(&s.palace)));
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
    fn lu_ma_pei_yin_hits_any_palace() {
        // 三星同守命宫：`palace` 记命宫
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

        // 三星同守命宫以外的宫：仍成格，`palace` 记实际落宫
        let b = find_chart(|a| {
            let v = natal(a);
            (0..12).any(|i| {
                i != v.soul()
                    && v.has(i, StarKey::LucunMin)
                    && v.has(i, StarKey::TianmaMin)
                    && v.has(i, StarKey::TianxiangMaj)
            })
        });
        let v = natal(&b);
        let hits = lu_ma_pei_yin(&v);
        assert_eq!(hits.len(), 1);
        assert_ne!(hits[0].palace, v.soul());
        assert!(hits[0].stars.iter().all(|s| s.palace == hits[0].palace));
        // 天马只在四生地，成格宫地支必是寅申巳亥
        assert!(matches!(
            v.branch(hits[0].palace),
            EarthlyBranch::Yin | EarthlyBranch::Shen | EarthlyBranch::Si | EarthlyBranch::Hai
        ));

        // 禄马同宫但该宫无天相：不成格
        let c = find_chart(|a| {
            let v = natal(a);
            (0..12).any(|i| {
                v.has(i, StarKey::LucunMin)
                    && v.has(i, StarKey::TianmaMin)
                    && !v.has(i, StarKey::TianxiangMaj)
            })
        });
        assert!(lu_ma_pei_yin(&natal(&c)).is_empty());
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
        let hit = liang_chong_hua_gai(&v)
            .into_iter()
            .find(|h| h.variant.is_none())
            .expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert!(matches!(
            hit.stars[2].star,
            StarKey::DikongMin | StarKey::DijieMin
        ));

        // 双禄坐命但两类空曜都不见：两口径都不报
        let b = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has_mutagen(s, Mutagen::Lu)
                && !v.has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
                && !v.has_any(s, &[StarKey::Tiankong, StarKey::Jiekong, StarKey::Xunkong])
        });
        assert!(liang_chong_hua_gai(&natal(&b)).is_empty());
    }

    /// 双禄坐命、不见空劫但见天空/截空/旬空：只报 `kong_yao` 宽口径。
    #[test]
    fn liang_chong_hua_gai_kong_yao_variant() {
        let a = find_chart(|a| {
            let v = natal(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && v.has_mutagen(s, Mutagen::Lu)
                && !v.has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
                && v.has_any(s, &[StarKey::Tiankong, StarKey::Jiekong, StarKey::Xunkong])
        });
        let v = natal(&a);
        let hits = liang_chong_hua_gai(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("kong_yao"));
        assert!(matches!(
            hits[0].stars[2].star,
            StarKey::Tiankong | StarKey::Jiekong | StarKey::Xunkong
        ));
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
        // 限命宫三方四正又见七杀时以 qisha 标注并多记一颗证据
        let qisha = v.find_in_surround(v.soul(), StarKey::QishaMaj).is_some();
        assert_eq!(hit.variant, qisha.then_some("qisha"));
        assert_eq!(hit.stars.len(), if qisha { 5 } else { 4 });
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

    /// 限命宫三方四正见七杀：命中带 `variant = "qisha"`，七杀记入证据末位。
    #[test]
    fn lu_shuai_ma_kun_marks_qisha_variant() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            v.find_in_surround(v.soul(), StarKey::QishaMaj).is_some()
                && !lu_shuai_ma_kun(&v).is_empty()
        });
        let h = horo(&a);
        let v = decadal(&h);
        let hits = lu_shuai_ma_kun(&v);
        assert_eq!(hits[0].variant, Some("qisha"));
        assert_eq!(hits[0].stars.len(), 5);
        assert_eq!(hits[0].stars[4].star, StarKey::QishaMaj);
    }

    /// 二限组合的宽（三方会照）与严（本宫同宫）在 `variant` 上的承载：
    /// `None`/`"same_palace"` 为大限 + 小限，`"yearly"`/`"yearly_same_palace"` 为大限 + 流年。
    fn is_age_combo(h: &PatternHit) -> bool {
        !h.variant.is_some_and(|s| s.starts_with("yearly"))
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
            .find(|x| is_age_combo(x))
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
            .find(|x| !is_age_combo(x))
            .expect("decadal + yearly");
        assert_eq!(hit.stars.len(), 2);
        let y = ChartView::at(&a, h.data(), Scope::Yearly, &PatternConfig::default());
        assert!(y.surround(y.soul()).contains(&hit.stars[1].palace));
    }

    /// 两限命宫皆本宫坐禄马：同宫严口径成立，`variant` 升为 `same_palace`。
    #[test]
    fn feng_yun_ji_hui_same_palace_variant() {
        let a = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            meets_in_palace(&v, v.soul()) && meets_in_palace(&v, h.age.index)
        });
        let h = horo(&a);
        let v = decadal(&h);
        let hits = feng_yun_ji_hui(&v);
        let hit = hits
            .iter()
            .find(|x| is_age_combo(x))
            .expect("decadal + age");
        assert_eq!(hit.variant, Some("same_palace"));

        // 仅三方会照（至少一限本宫不坐禄马）：variant 保持 None
        let b = find_chart(|a| {
            let Ok(h) = a.horoscope("2026-8-19", 3) else {
                return false;
            };
            let v = decadal(&h);
            meets_lu_ma(&v, v.soul()).is_some()
                && meets_lu_ma(&v, h.age.index).is_some()
                && !(meets_in_palace(&v, v.soul()) && meets_in_palace(&v, h.age.index))
        });
        let hb = horo(&b);
        let hits = feng_yun_ji_hui(&decadal(&hb));
        assert_eq!(
            hits.iter().find(|x| is_age_combo(x)).expect("hit").variant,
            None
        );
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

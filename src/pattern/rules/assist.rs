//! 辅弼、昌曲、魁钺、羊陀、空劫系格局（《格局》页 #47-#63）。

use crate::data::stars::StarKey;
use crate::data::types::{EarthlyBranch, Mutagen};
use crate::models::star::Star;
use crate::pattern::view::{BRIGHT, KONG_WANG};
use crate::pattern::{ChartView, PatternHit, PatternKey, Rule};

/// 本组规则，按《格局》页顺序。
pub static RULES: &[Rule] = &[
    Rule {
        key: PatternKey::YangTuoJiaMing,
        detect: yang_tuo_jia_ming,
    },
    Rule {
        key: PatternKey::MaTouDaiJian,
        detect: ma_tou_dai_jian,
    },
    Rule {
        key: PatternKey::ZuoYouTongGong,
        detect: zuo_you_tong_gong,
    },
    Rule {
        key: PatternKey::ZuoYouJiaMing,
        detect: zuo_you_jia_ming,
    },
    Rule {
        key: PatternKey::FuBiGongZhu,
        detect: fu_bi_gong_zhu,
    },
    Rule {
        key: PatternKey::KuiYueJiaMing,
        detect: kui_yue_jia_ming,
    },
    Rule {
        key: PatternKey::ZuoGuiXiangGui,
        detect: zuo_gui_xiang_gui,
    },
    Rule {
        key: PatternKey::JieKongJiaMing,
        detect: jie_kong_jia_ming,
    },
    Rule {
        key: PatternKey::LuFengLiangSha,
        detect: lu_feng_liang_sha,
    },
    Rule {
        key: PatternKey::WenGuiWenHua,
        detect: wen_gui_wen_hua,
    },
    Rule {
        key: PatternKey::WenXingChaoMing,
        detect: wen_xing_chao_ming,
    },
    Rule {
        key: PatternKey::ChangQuJiaMing,
        detect: chang_qu_jia_ming,
    },
    Rule {
        key: PatternKey::WenXingAnGong,
        detect: wen_xing_an_gong,
    },
    Rule {
        key: PatternKey::QuanLuShengFeng,
        detect: quan_lu_sheng_feng,
    },
    Rule {
        key: PatternKey::KeMingAnLu,
        detect: ke_ming_an_lu,
    },
    Rule {
        key: PatternKey::KeQuanLuJia,
        detect: ke_quan_lu_jia,
    },
    Rule {
        key: PatternKey::JiaDiDengYong,
        detect: jia_di_deng_yong,
    },
];

/// 一颗星与它的落宫，即规则证据的最小单元。
type StarPos<'a> = (usize, &'a Star);

/// 「A 星与 B 星夹宫 `i`」，返回 `((A 落宫, A 星), (B 落宫, B 星))`；两星在前后哪一侧不限。
fn jia_pair<'a>(
    v: &ChartView<'a>,
    i: usize,
    a: StarKey,
    b: StarKey,
) -> Option<(StarPos<'a>, StarPos<'a>)> {
    let (pa, pb) = v.jia(i, &[a], &[b])?;
    Some(((pa, v.find(pa, a)?), (pb, v.find(pb, b)?)))
}

/// 天同与太阴同守某宫，返回 `(所在宫, 天同, 太阴)`；本宫无主星且开启借宫时取对宫的一对。
fn tong_yin<'a>(v: &ChartView<'a>, i: usize) -> Option<(usize, &'a Star, &'a Star)> {
    let at = |p: usize| {
        Some((
            p,
            v.find(p, StarKey::TiantongMaj)?,
            v.find(p, StarKey::TaiyinMaj)?,
        ))
    };
    at(i).or_else(|| {
        (v.config().borrow && v.is_empty(i))
            .then(|| at(v.opposite(i)))
            .flatten()
    })
}

/// 羊陀夹命：擎羊在命宫后一宫、陀罗在命宫前一宫。
///
/// 擎羊恒在禄存顺行一位、陀罗恒在禄存逆行一位，故被夹的命宫必坐禄存，禄存一并记入证据。
///
/// 来源：作者说明（全书无引文），并注明被夹命宫必坐禄存。
pub fn yang_tuo_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (p, n) = (v.prev(s), v.next(s));
    let (Some(tuo), Some(yang)) = (
        v.find(p, StarKey::TuoluoMin),
        v.find(n, StarKey::QingyangMin),
    ) else {
        return vec![];
    };
    let mut stars = vec![(p, tuo), (n, yang)];
    if let Some(lu) = v.find(s, StarKey::LucunMin) {
        stars.insert(0, (s, lu));
    }
    vec![v.hit(PatternKey::YangTuoJiaMing, s, stars)]
}

/// 马头带箭：命宫在午，擎羊同宫，且天同、太阴同守命宫（命宫无主星时借对宫的同阴）。
///
/// 旁格「午宫贪狼化禄与擎羊同宫」以 `variant = "tanlang_lu"` 报出；两形式互斥（同阴与贪狼不同宫）。
/// 擎羊落午只出现于丙、戊年，故引文的「丙戊镇御边疆」是既成事实而非附加条件。
///
/// 来源：「马头带剑，谓马有刃是也」「天同贪羊陀居午位，丙戊镇御边疆，为马头带箭富且贵」。
pub fn ma_tou_dai_jian(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    if v.branch(s) != EarthlyBranch::Wu {
        return vec![];
    }
    let Some(yang) = v.find(s, StarKey::QingyangMin) else {
        return vec![];
    };
    if let Some((p, tong, yin)) = tong_yin(v, s) {
        return vec![v.hit(
            PatternKey::MaTouDaiJian,
            s,
            vec![(s, yang), (p, tong), (p, yin)],
        )];
    }
    match v.find(s, StarKey::TanlangMaj) {
        Some(tan) if v.mutagen_of(tan) == Some(Mutagen::Lu) => {
            let mut hit = v.hit(PatternKey::MaTouDaiJian, s, vec![(s, yang), (s, tan)]);
            hit.variant = Some("tanlang_lu");
            vec![hit]
        }
        _ => vec![],
    }
}

/// 左右同宫：左辅、右弼同守命宫或身宫；仅三方会照不算。命宫身宫各自成格各报一次。
///
/// 来源：「左右同宫披罗衣紫」。
pub fn zuo_you_tong_gong(v: &ChartView) -> Vec<PatternHit> {
    v.soul_and_body()
        .into_iter()
        .filter_map(|i| {
            let (zuo, you) = (v.find(i, StarKey::ZuofuMin)?, v.find(i, StarKey::YoubiMin)?);
            Some(v.hit(PatternKey::ZuoYouTongGong, i, vec![(i, zuo), (i, you)]))
        })
        .collect()
}

/// 左右夹命：左辅、右弼分居命宫前后两宫。
///
/// 来源：「左右夹命为贵格，如安命在丑宫，左辅在子宫，右弼在寅宫」。
pub fn zuo_you_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match jia_pair(v, s, StarKey::ZuofuMin, StarKey::YoubiMin) {
        Some((zuo, you)) => vec![v.hit(PatternKey::ZuoYouJiaMing, s, vec![zuo, you])],
        None => vec![],
    }
}

/// 辅弼拱主：紫微守命，左辅、右弼来拱或来夹。
///
/// `variant = "surround"` 为两星皆落命宫三方四正（含与紫微同宫），`variant = "jia"` 为两星分居命宫前后。
/// 两形式互斥：三方四正不含前后邻宫，而左辅右弼各只有一颗。只有一颗不成格。
///
/// 来源：「辅弼拱主，紫微守命二星来拱是也，夹之亦然」。
pub fn fu_bi_gong_zhu(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let Some(zi) = v.find(s, StarKey::ZiweiMaj) else {
        return vec![];
    };
    let found = match (
        v.find_in_surround(s, StarKey::ZuofuMin),
        v.find_in_surround(s, StarKey::YoubiMin),
    ) {
        (Some(zuo), Some(you)) => Some(("surround", zuo, you)),
        _ => {
            jia_pair(v, s, StarKey::ZuofuMin, StarKey::YoubiMin).map(|(zuo, you)| ("jia", zuo, you))
        }
    };
    let Some((variant, zuo, you)) = found else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::FuBiGongZhu, s, vec![(s, zi), zuo, you]);
    hit.variant = Some(variant);
    vec![hit]
}

/// 魁钺夹命：天魁、天钺分居命宫前后两宫；同宫或三方会照不算此格。
///
/// 来源：「魁钺夹命为奇格，如命安在辰宫，魁在卯，钺在巳宫是也」。
pub fn kui_yue_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match jia_pair(v, s, StarKey::TiankuiMin, StarKey::TianyueMin) {
        Some((kui, yue)) => vec![v.hit(PatternKey::KuiYueJiaMing, s, vec![kui, yue])],
        None => vec![],
    }
}

/// 坐贵向贵：天魁、天钺分坐命宫与迁移宫（命迁线相对）。
///
/// 来源：「坐贵向贵，谓魁钺在命迭相坐拱是也」。
pub fn zuo_gui_xiang_gui(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let o = v.opposite(s);
    for (a, b) in [
        (StarKey::TiankuiMin, StarKey::TianyueMin),
        (StarKey::TianyueMin, StarKey::TiankuiMin),
    ] {
        if let (Some(x), Some(y)) = (v.find(s, a), v.find(o, b)) {
            return vec![v.hit(PatternKey::ZuoGuiXiangGui, s, vec![(s, x), (o, y)])];
        }
    }
    vec![]
}

/// 劫空夹命：地劫、地空分居命宫前后两宫。运限视角下即限命宫被夹（「岁限行到亦凶」）。
///
/// 来源：「劫空夹命为败局，假如命安在亥宫，劫在子宫，空在戌宫是也。岁限行到亦凶，夹忌亦凶孤贫刑伤」。
/// 「夹忌」页面未给完整定义，不另立规则。
pub fn jie_kong_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    match jia_pair(v, s, StarKey::DijieMin, StarKey::DikongMin) {
        Some((jie, kong)) => vec![v.hit(PatternKey::JieKongJiaMing, s, vec![jie, kong])],
        None => vec![],
    }
}

/// 禄逢两杀：禄存与空亡星（旬空、空亡、截路、截空）同守命宫，且命宫三方四正见地空或地劫。
///
/// 「冲会」取三方四正；空亡星集合取杂耀四颗，天空与地空不计入（地空是本格另一项条件）。
///
/// 来源：「禄逢两杀，禄坐空亡又逢空劫杀星是也」。
pub fn lu_feng_liang_sha(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let Some(lu) = v.find(s, StarKey::LucunMin) else {
        return vec![];
    };
    let Some(kong) = KONG_WANG.iter().find_map(|k| v.find(s, *k)) else {
        return vec![];
    };
    let Some(sha) = [StarKey::DikongMin, StarKey::DijieMin]
        .into_iter()
        .find_map(|k| v.find_in_surround(s, k))
    else {
        return vec![];
    };
    vec![v.hit(PatternKey::LuFengLiangSha, s, vec![(s, lu), (s, kong), sha])]
}

/// 文贵文华：文昌、文曲同守命宫、身宫或命宫三方四正之一宫，`palace` 为该宫。
///
/// 来源：作者说明全书无此四字，引「昌曲禄存犹为奇特」，取用范围为命宫、身宫或命宫三方四正。
pub fn wen_gui_wen_hua(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let mut candidates = v.soul_and_body();
    for p in v.surround(s) {
        if !candidates.contains(&p) {
            candidates.push(p);
        }
    }
    candidates
        .into_iter()
        .filter_map(|i| {
            let (chang, qu) = (
                v.find(i, StarKey::WenchangMin)?,
                v.find(i, StarKey::WenquMin)?,
            );
            Some(v.hit(PatternKey::WenGuiWenHua, i, vec![(i, chang), (i, qu)]))
        })
        .collect()
}

/// 文星朝命：文昌、文曲皆见于命宫三方四正（含与命宫同宫）。
///
/// 作者的「命宫及三方没有重煞、化忌冲破」不作硬条件：「重煞」无定义，改以 `broken` 标记三方四正见煞忌。
///
/// 来源：「文科拱照，贾谊年少登科」。
pub fn wen_xing_chao_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(chang), Some(qu)) = (
        v.find_in_surround(s, StarKey::WenchangMin),
        v.find_in_surround(s, StarKey::WenquMin),
    ) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::WenXingChaoMing, s, vec![chang, qu]);
    hit.broken = !v.no_sha(s);
    vec![hit]
}

/// 昌曲夹命：文昌、文曲分居命宫前后两宫。
///
/// 作者的「命宫宜有吉星或主星得地，并避免重煞冲破」是「宜」不是条件；三方四正见煞忌以 `broken` 标记。
///
/// 来源：「昌曲夹命最为奇，假若命在丑宫，文昌在寅，文曲在子是也」。
pub fn chang_qu_jia_ming(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let Some((chang, qu)) = jia_pair(v, s, StarKey::WenchangMin, StarKey::WenquMin) else {
        return vec![];
    };
    let mut hit = v.hit(PatternKey::ChangQuJiaMing, s, vec![chang, qu]);
    hit.broken = !v.no_sha(s);
    vec![hit]
}

/// 文星暗拱：文昌、文曲以「夹、迁移正照、三方会照」三种口径之一拱照命宫，逐口径独立判定、命中即报。
///
/// - `variant = "jia"`：两星分居命宫前后两宫——页面主口径，与昌曲夹命同形，两格同盘并报；
/// - `variant = "opposite"`：两星同坐迁移宫正照命宫；
/// - `variant = "surround"`：两星皆见于命宫三方四正（含命宫本宫）——与文星朝命同形；
///   迁移宫属三方四正，故 `opposite` 命中时 `surround` 必同时命中。
///
/// 来源：「文星暗拱，贾谊允矣登科」；作者：「名称和口径并不完全一致，实务上应先写清
/// 星曜是夹、对拱还是三合会照」——三种口径并报，取舍留给调用方。
pub fn wen_xing_an_gong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let mut hits = Vec::new();
    let mut push = |variant, stars| {
        let mut hit = v.hit(PatternKey::WenXingAnGong, s, stars);
        hit.variant = Some(variant);
        hits.push(hit);
    };
    if let Some((chang, qu)) = jia_pair(v, s, StarKey::WenchangMin, StarKey::WenquMin) {
        push("jia", vec![chang, qu]);
    }
    let o = v.opposite(s);
    if let (Some(chang), Some(qu)) = (
        v.find(o, StarKey::WenchangMin),
        v.find(o, StarKey::WenquMin),
    ) {
        push("opposite", vec![(o, chang), (o, qu)]);
    }
    if let (Some(chang), Some(qu)) = (
        v.find_in_surround(s, StarKey::WenchangMin),
        v.find_in_surround(s, StarKey::WenquMin),
    ) {
        push("surround", vec![chang, qu]);
    }
    hits
}

/// 权禄生逢：化权星与化禄星同守命宫，且两星皆庙旺。
///
/// 亮度取庙旺两级（引文「庙旺是也，陷不是」，中间的得、利、平不取）。
///
/// 来源：「权禄生逢，二星守命庙旺是也，陷不是」。
pub fn quan_lu_sheng_feng(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (Some(quan), Some(lu)) = (
        v.mutagen_star(s, Mutagen::Quan),
        v.mutagen_star(s, Mutagen::Lu),
    ) else {
        return vec![];
    };
    if !quan.with_brightness(&BRIGHT) || !lu.with_brightness(&BRIGHT) {
        return vec![];
    }
    vec![v.hit(PatternKey::QuanLuShengFeng, s, vec![(s, quan), (s, lu)])]
}

/// 科明暗禄：化科守命宫，命宫的暗合宫（六合位）有禄存（`variant = None`）；
/// 暗合宫为化禄时按部分流派口径以 `variant = "hua_lu"` 报出。
/// 两口径独立判定、证据各记各的禄，暗合宫禄存与化禄俱在则并报。
///
/// 暗合宫化禄不由明禄暗禄覆盖——明禄暗禄要求命宫坐禄存或化禄，命宫只有化科时它不成格。
///
/// 来源：「科明暗禄…即化科守命宫，命宫之暗合宫有禄存是也」；化禄口径为「部分流派亦可」之说。
pub fn ke_ming_an_lu(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let h = v.hidden(s);
    let Some(ke) = v.mutagen_star(s, Mutagen::Ke) else {
        return vec![];
    };
    let mut hits = Vec::new();
    if let Some(lu) = v.find(h, StarKey::LucunMin) {
        hits.push(v.hit(PatternKey::KeMingAnLu, s, vec![(s, ke), (h, lu)]));
    }
    if let Some(hua) = v.mutagen_star(h, Mutagen::Lu) {
        let mut hit = v.hit(PatternKey::KeMingAnLu, s, vec![(s, ke), (h, hua)]);
        hit.variant = Some("hua_lu");
        hits.push(hit);
    }
    hits
}

/// 科权禄夹：化禄、化权、化科中的两化分居命宫前后两宫；三化都在同一侧不算。
///
/// 来源：「科权禄夹为贵格，如命安在子宫，禄在亥宫，权在丑宫，为夹贵」。
pub fn ke_quan_lu_jia(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let (p, n) = (v.prev(s), v.next(s));
    let trio = [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke];
    for a in trio {
        for b in trio {
            if a == b {
                continue;
            }
            if let (Some(x), Some(y)) = (v.mutagen_star(p, a), v.mutagen_star(n, b)) {
                return vec![v.hit(PatternKey::KeQuanLuJia, s, vec![(p, x), (n, y)])];
            }
        }
    }
    vec![]
}

/// 甲第登庸：化科守命宫，化权在迁移宫或两三合宫朝拱命宫；化权同在命宫不算「朝」。
///
/// 再会化禄或禄存时以 `variant = "complete"` 标出（引文「或权或禄全更佳」），该星一并记入证据。
///
/// 来源：「科命权朝登庸甲第，或权或禄全更佳」。
pub fn jia_di_deng_yong(v: &ChartView) -> Vec<PatternHit> {
    let s = v.soul();
    let Some(ke) = v.mutagen_star(s, Mutagen::Ke) else {
        return vec![];
    };
    let [t1, t2] = v.trine(s);
    let Some(quan) = [v.opposite(s), t1, t2]
        .into_iter()
        .find_map(|p| v.mutagen_star(p, Mutagen::Quan).map(|star| (p, star)))
    else {
        return vec![];
    };
    let lu = v.find_mutagen_in_surround(s, Mutagen::Lu).or_else(|| {
        v.surround(s)
            .into_iter()
            .find_map(|p| v.find(p, StarKey::LucunMin).map(|star| (p, star)))
    });
    let mut stars = vec![(s, ke), quan];
    stars.extend(lu);
    let mut hit = v.hit(PatternKey::JiaDiDengYong, s, stars);
    if lu.is_some() {
        hit.variant = Some("complete");
    }
    vec![hit]
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

    /// 命中的证据里是否含「某星落在某宫」。
    fn evidence(hit: &PatternHit, star: StarKey, palace: usize) -> bool {
        hit.stars
            .iter()
            .any(|s| s.star == star && s.palace == palace)
    }

    /// 命宫内的化权星与化禄星（两者俱在时才有值）。
    fn quan_lu_pair<'a>(v: &ChartView<'a>) -> Option<(&'a Star, &'a Star)> {
        let s = v.soul();
        v.mutagen_star(s, Mutagen::Quan)
            .zip(v.mutagen_star(s, Mutagen::Lu))
    }

    /// 宫内带禄、权、科三化的个数。
    fn trio_count(v: &ChartView, i: usize) -> usize {
        [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke]
            .into_iter()
            .filter(|m| v.has_mutagen(i, *m))
            .count()
    }

    #[test]
    fn yang_tuo_jia_ming_follows_lucun_in_soul() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::LucunMin)
        });
        let v = view(&a);
        let s = v.soul();
        let hits = yang_tuo_jia_ming(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, s);
        assert!(evidence(hit, StarKey::LucunMin, s));
        assert!(evidence(hit, StarKey::TuoluoMin, v.prev(s)));
        assert!(evidence(hit, StarKey::QingyangMin, v.next(s)));

        // 命宫不坐禄存则羊陀必不在前后两宫
        let b = find_chart(|a| {
            let v = view(a);
            !v.has(v.soul(), StarKey::LucunMin)
        });
        assert!(yang_tuo_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn ma_tou_dai_jian_covers_tong_yin_and_tanlang_lu() {
        let wu_yang = |v: &ChartView| {
            let s = v.soul();
            v.branch(s) == EarthlyBranch::Wu && v.has(s, StarKey::QingyangMin)
        };

        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            wu_yang(&v) && v.has(s, StarKey::TiantongMaj) && v.has(s, StarKey::TaiyinMaj)
        });
        let v = view(&a);
        let hits = ma_tou_dai_jian(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.variant, None);
        assert!(evidence(hit, StarKey::QingyangMin, v.soul()));
        assert!(evidence(hit, StarKey::TiantongMaj, v.soul()));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            wu_yang(&v)
                && v.find(s, StarKey::TanlangMaj)
                    .is_some_and(|t| v.mutagen_of(t) == Some(Mutagen::Lu))
        });
        let v = view(&b);
        let hits = ma_tou_dai_jian(&v);
        assert_eq!(hits.first().expect("hit").variant, Some("tanlang_lu"));

        // 命宫在午、同阴同宫，但非丙戊年故无擎羊
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.branch(s) == EarthlyBranch::Wu
                && v.has(s, StarKey::TiantongMaj)
                && v.has(s, StarKey::TaiyinMaj)
                && !v.has(s, StarKey::QingyangMin)
        });
        assert!(ma_tou_dai_jian(&view(&c)).is_empty());
    }

    #[test]
    fn zuo_you_tong_gong_hits_soul_and_body() {
        let a = find_chart(|a| {
            let v = view(a);
            v.has(v.soul(), StarKey::ZuofuMin) && v.has(v.soul(), StarKey::YoubiMin)
        });
        let v = view(&a);
        let hits = zuo_you_tong_gong(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert!(evidence(hit, StarKey::ZuofuMin, v.soul()));
        assert!(evidence(hit, StarKey::YoubiMin, v.soul()));

        let b = find_chart(|a| {
            let v = view(a);
            matches!(v.body(), Some(b) if b != v.soul()
                && v.has(b, StarKey::ZuofuMin) && v.has(b, StarKey::YoubiMin))
        });
        let v = view(&b);
        let hits = zuo_you_tong_gong(&v);
        assert_eq!(hits.first().expect("hit").palace, v.body().unwrap());

        // 左辅右弼分居两宫：只三方会照不算
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::ZuofuMin) && !v.has(s, StarKey::YoubiMin)
        });
        assert!(zuo_you_tong_gong(&view(&c)).is_empty());
    }

    #[test]
    fn zuo_you_jia_ming_needs_both_neighbours() {
        let a = find_chart(|a| {
            let v = view(a);
            v.jia(v.soul(), &[StarKey::ZuofuMin], &[StarKey::YoubiMin])
                .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hits = zuo_you_jia_ming(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, s);
        let palaces: Vec<_> = hit.stars.iter().map(|x| x.palace).collect();
        assert!(palaces.contains(&v.prev(s)) && palaces.contains(&v.next(s)));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(v.prev(s), StarKey::ZuofuMin)
                && v.jia(s, &[StarKey::ZuofuMin], &[StarKey::YoubiMin])
                    .is_none()
        });
        assert!(zuo_you_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn fu_bi_gong_zhu_reports_surround_and_jia() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::ZiweiMaj)
                && v.in_surround(s, StarKey::ZuofuMin)
                && v.in_surround(s, StarKey::YoubiMin)
        });
        let v = view(&a);
        let hits = fu_bi_gong_zhu(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.variant, Some("surround"));
        assert!(evidence(hit, StarKey::ZiweiMaj, v.soul()));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::ZiweiMaj)
                && v.jia(s, &[StarKey::ZuofuMin], &[StarKey::YoubiMin])
                    .is_some()
        });
        let v = view(&b);
        assert_eq!(
            fu_bi_gong_zhu(&v).first().expect("hit").variant,
            Some("jia")
        );

        // 紫微守命但辅弼只到一颗
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::ZiweiMaj)
                && v.in_surround(s, StarKey::ZuofuMin)
                && !v.in_surround(s, StarKey::YoubiMin)
        });
        assert!(fu_bi_gong_zhu(&view(&c)).is_empty());
    }

    #[test]
    fn kui_yue_jia_ming_needs_adjacent_pair() {
        let a = find_chart(|a| {
            let v = view(a);
            v.jia(v.soul(), &[StarKey::TiankuiMin], &[StarKey::TianyueMin])
                .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hits = kui_yue_jia_ming(&v);
        let hit = hits.first().expect("hit");
        assert_eq!(hit.palace, s);
        let palaces: Vec<_> = hit.stars.iter().map(|x| x.palace).collect();
        assert!(palaces.contains(&v.prev(s)) && palaces.contains(&v.next(s)));

        // 天魁在兄弟宫而天钺不在父母宫：不成夹
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(v.prev(s), StarKey::TiankuiMin) && !v.has(v.next(s), StarKey::TianyueMin)
        });
        assert!(kui_yue_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn zuo_gui_xiang_gui_needs_soul_surface_axis() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TiankuiMin) && v.has(v.opposite(s), StarKey::TianyueMin)
        });
        let v = view(&a);
        let s = v.soul();
        let hit = zuo_gui_xiang_gui(&v).pop().expect("hit");
        assert!(evidence(&hit, StarKey::TiankuiMin, s));
        assert!(evidence(&hit, StarKey::TianyueMin, v.opposite(s)));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::TiankuiMin) && !v.has(v.opposite(s), StarKey::TianyueMin)
        });
        assert!(zuo_gui_xiang_gui(&view(&b)).is_empty());
    }

    #[test]
    fn jie_kong_jia_ming_needs_both_neighbours() {
        let a = find_chart(|a| {
            let v = view(a);
            v.jia(v.soul(), &[StarKey::DijieMin], &[StarKey::DikongMin])
                .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hit = jie_kong_jia_ming(&v).pop().expect("hit");
        assert_eq!(hit.palace, s);
        let palaces: Vec<_> = hit.stars.iter().map(|x| x.palace).collect();
        assert!(palaces.contains(&v.prev(s)) && palaces.contains(&v.next(s)));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has_any(v.prev(s), &[StarKey::DikongMin, StarKey::DijieMin])
                && v.jia(s, &[StarKey::DijieMin], &[StarKey::DikongMin])
                    .is_none()
        });
        assert!(jie_kong_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn lu_feng_liang_sha_needs_kongwang_and_jiekong() {
        let has_kongwang = |v: &ChartView, i: usize| v.has_any(i, &KONG_WANG);
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && has_kongwang(&v, s)
                && v.surround_has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
        });
        let v = view(&a);
        let s = v.soul();
        let hit = lu_feng_liang_sha(&v).pop().expect("hit");
        assert_eq!(hit.palace, s);
        assert!(evidence(&hit, StarKey::LucunMin, s));
        assert_eq!(hit.stars.len(), 3);

        // 禄坐空亡但三方四正不见空劫
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::LucunMin)
                && has_kongwang(&v, s)
                && !v.surround_has_any(s, &[StarKey::DikongMin, StarKey::DijieMin])
        });
        assert!(lu_feng_liang_sha(&view(&b)).is_empty());
    }

    #[test]
    fn wen_gui_wen_hua_covers_soul_body_and_surround() {
        let together = |v: &ChartView| {
            (0..12).find(|i| v.has(*i, StarKey::WenchangMin) && v.has(*i, StarKey::WenquMin))
        };
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            together(&v)
                .is_some_and(|i| v.surround(s).contains(&i) || v.soul_and_body().contains(&i))
        });
        let v = view(&a);
        let hit = wen_gui_wen_hua(&v).pop().expect("hit");
        assert_eq!(Some(hit.palace), together(&v));
        assert!(evidence(&hit, StarKey::WenchangMin, hit.palace));
        assert!(evidence(&hit, StarKey::WenquMin, hit.palace));

        // 昌曲同宫但落在取用范围之外
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            together(&v)
                .is_some_and(|i| !v.surround(s).contains(&i) && !v.soul_and_body().contains(&i))
        });
        assert!(wen_gui_wen_hua(&view(&b)).is_empty());
    }

    #[test]
    fn wen_xing_chao_ming_needs_both_in_surround() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.in_surround(s, StarKey::WenchangMin) && v.in_surround(s, StarKey::WenquMin)
        });
        let v = view(&a);
        let s = v.soul();
        let hit = wen_xing_chao_ming(&v).pop().expect("hit");
        assert_eq!(hit.palace, s);
        assert_eq!(hit.broken, !v.no_sha(s));
        assert_eq!(hit.stars.len(), 2);

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.in_surround(s, StarKey::WenchangMin) && !v.in_surround(s, StarKey::WenquMin)
        });
        assert!(wen_xing_chao_ming(&view(&b)).is_empty());
    }

    #[test]
    fn chang_qu_jia_ming_needs_both_neighbours() {
        let a = find_chart(|a| {
            let v = view(a);
            v.jia(v.soul(), &[StarKey::WenchangMin], &[StarKey::WenquMin])
                .is_some()
        });
        let v = view(&a);
        let s = v.soul();
        let hit = chang_qu_jia_ming(&v).pop().expect("hit");
        assert_eq!(hit.palace, s);
        assert_eq!(hit.broken, !v.no_sha(s));
        // 夹是文星暗拱的主口径，同盘并报
        assert!(
            wen_xing_an_gong(&v)
                .iter()
                .any(|h| h.variant == Some("jia"))
        );

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(v.prev(s), StarKey::WenchangMin)
                && v.jia(s, &[StarKey::WenchangMin], &[StarKey::WenquMin])
                    .is_none()
        });
        assert!(chang_qu_jia_ming(&view(&b)).is_empty());
    }

    #[test]
    fn wen_xing_an_gong_reports_jia_opposite_and_surround() {
        // 昌曲同坐迁移宫：正照与会照两口径同时命中（迁移宫属三方四正）
        let a = find_chart(|a| {
            let v = view(a);
            let o = v.opposite(v.soul());
            v.has(o, StarKey::WenchangMin) && v.has(o, StarKey::WenquMin)
        });
        let v = view(&a);
        let o = v.opposite(v.soul());
        let hits = wen_xing_an_gong(&v);
        let opp = hits
            .iter()
            .find(|h| h.variant == Some("opposite"))
            .expect("opposite");
        assert_eq!(opp.palace, v.soul());
        assert!(opp.stars.iter().all(|x| x.palace == o));
        assert!(hits.iter().any(|h| h.variant == Some("surround")));

        // 昌曲分居两三合宫：只报会照
        let b = find_chart(|a| {
            let v = view(a);
            let [t1, t2] = v.trine(v.soul());
            (v.has(t1, StarKey::WenchangMin) && v.has(t2, StarKey::WenquMin))
                || (v.has(t1, StarKey::WenquMin) && v.has(t2, StarKey::WenchangMin))
        });
        let v = view(&b);
        let [t1, t2] = v.trine(v.soul());
        let hits = wen_xing_an_gong(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("surround"));
        let palaces: Vec<_> = hits[0].stars.iter().map(|x| x.palace).collect();
        assert!(palaces.contains(&t1) && palaces.contains(&t2));

        // 昌曲同宫坐命：会照口径含命宫本宫，与文星朝命同形并报
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has(s, StarKey::WenchangMin) && v.has(s, StarKey::WenquMin)
        });
        let v = view(&c);
        let hits = wen_xing_an_gong(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("surround"));
        assert!(hits[0].stars.iter().all(|x| x.palace == v.soul()));

        // 夹的形式不落三方四正，只报 jia
        let d = find_chart(|a| {
            let v = view(a);
            v.jia(v.soul(), &[StarKey::WenchangMin], &[StarKey::WenquMin])
                .is_some()
        });
        let hits = wen_xing_an_gong(&view(&d));
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("jia"));
    }

    #[test]
    fn quan_lu_sheng_feng_requires_both_bright() {
        let a = find_chart(|a| {
            let v = view(a);
            quan_lu_pair(&v)
                .is_some_and(|(q, l)| q.with_brightness(&BRIGHT) && l.with_brightness(&BRIGHT))
        });
        let v = view(&a);
        let hit = quan_lu_sheng_feng(&v).pop().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.stars.len(), 2);

        // 权禄同守命宫但有一颗不在庙旺
        let b = find_chart(|a| {
            let v = view(a);
            quan_lu_pair(&v)
                .is_some_and(|(q, l)| !(q.with_brightness(&BRIGHT) && l.with_brightness(&BRIGHT)))
        });
        assert!(quan_lu_sheng_feng(&view(&b)).is_empty());
    }

    #[test]
    fn ke_ming_an_lu_uses_hidden_palace_lucun() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has_mutagen(s, Mutagen::Ke) && v.has(v.hidden(s), StarKey::LucunMin)
        });
        let v = view(&a);
        let s = v.soul();
        let hit = ke_ming_an_lu(&v)
            .into_iter()
            .find(|h| h.variant.is_none())
            .expect("hit");
        assert_eq!(hit.palace, s);
        assert!(evidence(&hit, StarKey::LucunMin, v.hidden(s)));

        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has_mutagen(s, Mutagen::Ke)
                && !v.has(v.hidden(s), StarKey::LucunMin)
                && !v.has_mutagen(v.hidden(s), Mutagen::Lu)
        });
        assert!(ke_ming_an_lu(&view(&b)).is_empty());
    }

    /// 暗合宫无禄存但有化禄：以 `variant = "hua_lu"` 报出（命宫无禄时明禄暗禄不成格，
    /// 此口径不被它覆盖）。
    #[test]
    fn ke_ming_an_lu_hua_lu_variant() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            v.has_mutagen(s, Mutagen::Ke)
                && !v.has(v.hidden(s), StarKey::LucunMin)
                && v.has_mutagen(v.hidden(s), Mutagen::Lu)
                && !v.has(s, StarKey::LucunMin)
                && !v.has_mutagen(s, Mutagen::Lu)
        });
        let v = view(&a);
        let s = v.soul();
        let hits = ke_ming_an_lu(&v);
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].variant, Some("hua_lu"));
        assert_eq!(hits[0].stars[1].palace, v.hidden(s));
        assert_eq!(hits[0].stars[1].mutagen, Some(Mutagen::Lu));
        // 命宫无禄存无化禄：明禄暗禄不报，宽口径不与之重叠
        assert!(!a.patterns().iter().any(|h| h.key == PatternKey::MingLuAnLu));
    }

    #[test]
    fn ke_quan_lu_jia_needs_two_sides() {
        let a = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            trio_count(&v, v.prev(s)) > 0 && trio_count(&v, v.next(s)) > 0
        });
        let v = view(&a);
        let s = v.soul();
        let hit = ke_quan_lu_jia(&v).pop().expect("hit");
        assert_eq!(hit.palace, s);
        let palaces: Vec<_> = hit.stars.iter().map(|x| x.palace).collect();
        assert!(palaces.contains(&v.prev(s)) && palaces.contains(&v.next(s)));

        // 三化都在同一侧邻宫不算夹
        let b = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            trio_count(&v, v.prev(s)) >= 2 && trio_count(&v, v.next(s)) == 0
        });
        assert!(ke_quan_lu_jia(&view(&b)).is_empty());
    }

    #[test]
    fn jia_di_deng_yong_marks_complete_when_lu_joins() {
        let quan_chao = |v: &ChartView| {
            let s = v.soul();
            let [t1, t2] = v.trine(s);
            v.has_mutagen(s, Mutagen::Ke)
                && [v.opposite(s), t1, t2]
                    .iter()
                    .any(|p| v.has_mutagen(*p, Mutagen::Quan))
        };
        let with_lu = |v: &ChartView| {
            let s = v.soul();
            v.find_mutagen_in_surround(s, Mutagen::Lu).is_some()
                || v.in_surround(s, StarKey::LucunMin)
        };

        let a = find_chart(|a| {
            let v = view(a);
            quan_chao(&v) && !with_lu(&v)
        });
        let v = view(&a);
        let hit = jia_di_deng_yong(&v).pop().expect("hit");
        assert_eq!(hit.palace, v.soul());
        assert_eq!(hit.variant, None);
        assert_eq!(hit.stars.len(), 2);

        let b = find_chart(|a| {
            let v = view(a);
            quan_chao(&v) && with_lu(&v)
        });
        let hit = jia_di_deng_yong(&view(&b)).pop().expect("hit");
        assert_eq!(hit.variant, Some("complete"));
        assert_eq!(hit.stars.len(), 3);

        // 化科化权同在命宫：不算「朝」
        let c = find_chart(|a| {
            let v = view(a);
            let s = v.soul();
            let [t1, t2] = v.trine(s);
            v.has_mutagen(s, Mutagen::Ke)
                && v.has_mutagen(s, Mutagen::Quan)
                && ![v.opposite(s), t1, t2]
                    .iter()
                    .any(|p| v.has_mutagen(*p, Mutagen::Quan))
        });
        assert!(jia_di_deng_yong(&view(&c)).is_empty());
    }
}

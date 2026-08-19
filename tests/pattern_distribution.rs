//! 格局引擎在 tier1 金标 1,560 张盘上的批量合理性测试。
//!
//! 逐条规则的正反例在 `src/pattern/rules/*` 的单测里，示例盘复现在 `tests/pattern_examples.rs`；
//! 本测试只回答「放到一大批真实盘上跑会不会出事」：本命与两层运限全跑一遍，
//! 检查结果的结构不变量（宫位索引、证据、视角）、地支不变量（只可能落某几个地支的格局），
//! 并统计每个格局的命中盘数——命中数为 0 的格局收在 [`NEVER_HIT`] 里，
//! 新增的 0 命中（规则写死）与不再为 0（口径放宽）都会让断言失败。
//!
//! 盘的来源是 tier1 金标的入参（1984-2043 每年 2 月 15 日 × 13 时辰 × 男女），
//! 与排盘金标同一批，故这里只测格局，不重复测排盘。

use std::collections::BTreeMap;

use serde_json::Value;
use x_iztro::{
    ALL_PATTERNS, Astrolabe, Config, EarthlyBranch, Gender, Language, PatternHit, PatternKey,
    Scope, by_solar,
};

use EarthlyBranch::*;

static TIER1: &str = include_str!("golden/tier1_data.json");

/// 运限目标：固定一个晚于全部生日（最晚 2043-2-15）的日期与时辰，
/// 使 1,560 张盘的大限、流年都落在有效区间内。
///
/// 目标固定即流年地支固定，故流年命宫在每张盘上都是午宫那一格：
/// 打印表里「流年」一列只有落午（及三方四正含午）的格局有数，其余为 0，是这一取样方式使然。
/// 大限命宫随各人年龄走遍十二宫，「大限」一列才是分布面较全的一列。
const TARGET_DATE: &str = "2050-6-15";
const TARGET_TIME_INDEX: u8 = 3;

/// 只可能成格于特定地支的格局：星耀布局或规则本身把成格宫位钉死在这几个地支。
///
/// 其中金舆扶驾、极向离明、极居卯酉、机月同梁、机巨居卯、日照雷门、金灿光辉、
/// 月朗天门、月生沧海、明珠出海、武贪同行、雄宿朝元、石中隐玉、七杀朝斗、
/// 英星入庙、众水朝东、马头带箭是规则里写死的地支条件；紫府同宫、紫府夹命、
/// 日月同宫、巨日同宫、机巨同临、善荫朝纲、日月照璧、日月夹命、日月夹财、
/// 刑囚夹印则是安星规律的必然结果（如紫微天府只在寅申同宫、日月只在丑未同宫），
/// 规则不写地支也只能落在这里——写成断言即可守住这条规律。
const BRANCH_BOUND: &[(PatternKey, &[EarthlyBranch])] = &[
    (PatternKey::ZiFuTongGong, &[Yin, Shen]),
    (PatternKey::JinYuFuJia, &[Chou, Wei]),
    (PatternKey::ZiFuJiaMing, &[Yin, Shen]),
    (PatternKey::JiXiangLiMing, &[Wu]),
    (PatternKey::JiJuMaoYou, &[Mao, You]),
    (PatternKey::JiYueTongLiang, &[Yin, Shen]),
    (PatternKey::ShanYinChaoGang, &[Chen, Xu]),
    (PatternKey::JiJuTongLin, &[Mao, You]),
    (PatternKey::JiJuJuMao, &[Mao]),
    (PatternKey::RiYueTongGong, &[Chou, Wei]),
    (PatternKey::JuRiTongGong, &[Yin, Shen]),
    (PatternKey::RiZhaoLeiMen, &[Mao]),
    (PatternKey::RiYueZhaoBi, &[Chou, Wei]),
    (PatternKey::JinCanGuangHui, &[Wu]),
    (PatternKey::RiYueJiaMing, &[Chou, Wei]),
    (PatternKey::RiYueJiaCai, &[Chou, Wei]),
    (PatternKey::YueLangTianMen, &[Hai]),
    (PatternKey::YueShengCangHai, &[Zi]),
    (PatternKey::MingZhuChuHai, &[Wei]),
    (PatternKey::WuTanTongXing, &[Chou, Wei]),
    (PatternKey::XingQiuJiaYin, &[Zi, Wu]),
    (PatternKey::XiongSuChaoYuan, &[Yin, Shen]),
    (PatternKey::ShiZhongYinYu, &[Zi, Wu]),
    (PatternKey::QiShaChaoDou, &[Zi, Yin, Wu, Shen]),
    (PatternKey::YingXingRuMiao, &[Zi, Wu]),
    (PatternKey::ZhongShuiChaoDong, &[Yin, Mao]),
    (PatternKey::MaTouDaiJian, &[Wu]),
];

/// 在这 1,560 张盘的本命与大限、流年三层上一次都没命中的格局。
///
/// 两条都不是规则写死，而是 tier1 取样面的必然结果：左辅自辰宫按月顺行、右弼自戌宫按月逆行，
/// 二者相隔 `6 + 2(月 - 1)` 宫，故同宫只出现在农历四月与十月、相隔两宫（可夹一宫）
/// 只出现在三、五、九、十一月；tier1 的生日固定为每年 2 月 15 日，农历月份只有正月与腊月
/// （实测这 1,560 张盘的左右间隔只有 4 与 6 两种），两类月份都取不到。
/// 这两格的正例见 `src/pattern/rules/assist.rs` 的单测与 `src/pattern/mod.rs` 的
/// `zuo_you_jia_ming_requires_both_neighbours`（在 1950-2020 全枚举里都能搜到真实盘）。
const NEVER_HIT: &[PatternKey] = &[PatternKey::ZuoYouTongGong, PatternKey::ZuoYouJiaMing];

/// tier1 金标的入参：阳历日期、时辰索引、性别。
struct Case {
    solar_date: String,
    time_index: u8,
    gender: Gender,
}

fn load_cases() -> Vec<Case> {
    let cases: Vec<Value> = serde_json::from_str(TIER1).expect("解析 tier1_data.json");
    cases
        .iter()
        .map(|c| {
            let p = &c["params"];
            Case {
                solar_date: p["solar_date"].as_str().expect("阳历日期").to_string(),
                time_index: p["time_index"].as_u64().expect("时辰索引") as u8,
                gender: match p["gender"].as_str().expect("性别") {
                    "男" => Gender::Male,
                    "女" => Gender::Female,
                    other => panic!("未知性别：{other}"),
                },
            }
        })
        .collect()
}

fn chart(case: &Case) -> Astrolabe {
    by_solar(
        &case.solar_date,
        case.time_index,
        case.gender,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .expect("tier1 入参均合法")
}

/// 一张盘上一层视角的全部命中，逐条检查结构与地支不变量。
fn check_hits(a: &Astrolabe, hits: &[PatternHit], scope: Scope, label: &str) {
    for hit in hits {
        assert!(
            hit.palace < 12,
            "{label} {}：成格宫位索引 {} 越界",
            hit.key.as_key(),
            hit.palace
        );
        assert!(
            !hit.stars.is_empty(),
            "{label} {}：命中证据不应为空",
            hit.key.as_key()
        );
        assert!(
            hit.stars.iter().all(|s| s.palace < 12),
            "{label} {}：证据星的落宫索引越界",
            hit.key.as_key()
        );
        assert_eq!(
            hit.scope,
            scope,
            "{label} {}：命中视角应为 {scope:?}",
            hit.key.as_key()
        );
        if let Some((_, branches)) = BRANCH_BOUND.iter().find(|(k, _)| *k == hit.key) {
            let branch = a.palaces[hit.palace].earthly_branch;
            assert!(
                branches.contains(&branch),
                "{label} {}：成格宫位地支 {branch:?} 不在 {branches:?} 内",
                hit.key.as_key()
            );
        }
        if scope == Scope::Origin {
            assert!(
                !hit.key.is_horoscope_only(),
                "{label} {}：行运格不应出现在本命结果里",
                hit.key.as_key()
            );
        }
    }
}

#[test]
fn tier1_charts_hit_patterns_consistently() {
    let cases = load_cases();
    assert_eq!(cases.len(), 1560, "tier1 用例数");

    // 每个格局在三层视角上各命中过多少张盘
    let mut tally: BTreeMap<&'static str, [usize; 3]> =
        ALL_PATTERNS.iter().map(|k| (k.as_key(), [0; 3])).collect();

    for case in &cases {
        let label = format!("{} 时辰{} ", case.solar_date, case.time_index);
        let a = chart(case);
        let natal = a.patterns();
        check_hits(&a, &natal, Scope::Origin, &label);

        let horoscope = a
            .horoscope(TARGET_DATE, TARGET_TIME_INDEX)
            .expect("运限目标日期合法");
        assert_eq!(
            horoscope.patterns(Scope::Origin),
            natal,
            "{label}：运限的 Origin 视角应与本命一致"
        );

        let decadal = horoscope.patterns(Scope::Decadal);
        check_hits(&a, &decadal, Scope::Decadal, &label);
        let yearly = horoscope.patterns(Scope::Yearly);
        check_hits(&a, &yearly, Scope::Yearly, &label);

        for (column, hits) in [&natal, &decadal, &yearly].into_iter().enumerate() {
            let mut seen: Vec<PatternKey> = Vec::new();
            for hit in hits {
                if !seen.contains(&hit.key) {
                    seen.push(hit.key);
                    tally.get_mut(hit.key.as_key()).expect("格局在全表内")[column] += 1;
                }
            }
        }
    }

    println!(
        "\ntier1 {} 张盘的格局命中盘数（运限目标 {TARGET_DATE} 时辰 {TARGET_TIME_INDEX}）\n\
         {:<24}{:>8}{:>8}{:>8}",
        cases.len(),
        "格局",
        "本命",
        "大限",
        "流年"
    );
    for key in ALL_PATTERNS {
        let [origin, decadal, yearly] = tally[key.as_key()];
        println!("{:<24}{origin:>8}{decadal:>8}{yearly:>8}", key.as_key());
    }

    let never: Vec<PatternKey> = ALL_PATTERNS
        .into_iter()
        .filter(|k| tally[k.as_key()] == [0; 3])
        .collect();
    println!(
        "\n三层都没命中的格局 {} 个：{:?}",
        never.len(),
        never.iter().map(|k| k.as_key()).collect::<Vec<_>>()
    );
    assert_eq!(
        never, NEVER_HIT,
        "0 命中的格局清单变了：新增的要确认不是规则写死，消失的要从 NEVER_HIT 里删掉"
    );
}

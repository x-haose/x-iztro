//! 格局引擎在 tier1 金标 1,560 张盘上的批量合理性测试。
//!
//! 逐条规则的正反例在 `src/pattern/rules/*` 的单测里，示例盘复现在 `tests/pattern_examples.rs`；
//! 本测试只回答「放到一大批真实盘上跑会不会出事」：本命与两层运限全跑一遍，
//! 检查结果的结构不变量（宫位索引、证据、视角）、地支不变量（只可能落某几个地支的格局），
//! 并统计每个格局的命中盘数——整张计数表锁定为金标 [`GOLDEN_TALLY`] 逐格对照，
//! 命中数为 0 的格局另收在 [`NEVER_HIT`] 里注明成因；
//! 新增的 0 命中（规则写死）、不再为 0（口径放宽）与任何计数漂移都会让断言失败。
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
/// 其中金舆扶驾、极向离明、极居卯酉、机巨居卯、日照雷门、金灿光辉、
/// 月朗天门、月生沧海、明珠出海、武贪同行、雄宿朝元、石中隐玉、七杀朝斗、
/// 英星入庙、众水朝东、马头带箭是规则里写死的地支条件；紫府同宫、紫府夹命、
/// 日月同宫、巨日同宫、机巨同临、善荫朝纲、日月照璧、日月夹命、日月夹财、
/// 刑囚夹印则是安星规律的必然结果（如紫微天府只在寅申同宫、日月只在丑未同宫），
/// 规则不写地支也只能落在这里——写成断言即可守住这条规律。
/// 机月同梁不在表内：主口径钉死寅申，但宽口径（四星齐见三方四正，`variant = "surround"`）
/// 的成格宫是命宫本身、地支不限，钉死地支的只是主口径，由规则单测守。
const BRANCH_BOUND: &[(PatternKey, &[EarthlyBranch])] = &[
    (PatternKey::ZiFuTongGong, &[Yin, Shen]),
    (PatternKey::JinYuFuJia, &[Chou, Wei]),
    (PatternKey::ZiFuJiaMing, &[Yin, Shen]),
    (PatternKey::JiXiangLiMing, &[Wu]),
    (PatternKey::JiJuMaoYou, &[Mao, You]),
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

/// 全表金标：每个格局在本命、大限、流年三层各命中的盘数（顺序同 [`ALL_PATTERNS`]）。
///
/// 数值锁定当前口径在这 1,560 张盘 × 固定运限目标上的实跑结果，任何一格变动都判失败：
/// 无意的口径漂移（含大面积误报这类「永真」回归）由此可红。有意改动格局口径后，
/// 从本测试打印的表抄新数值更新（与 `UPDATE_PATTERN_SNAPSHOTS=1` 重建快照同一流程）。
const GOLDEN_TALLY: &[(&str, [usize; 3])] = &[
    ("jun_chen_qing_hui", [4, 0, 0]),
    ("zi_fu_tong_gong", [30, 22, 0]),
    ("jin_yu_fu_jia", [8, 17, 0]),
    ("zi_fu_jia_ming", [26, 12, 0]),
    ("ji_xiang_li_ming", [0, 0, 12]),
    ("ji_ju_mao_you", [16, 15, 0]),
    ("ji_yue_tong_liang", [402, 376, 530]),
    ("shan_yin_chao_gang", [44, 15, 0]),
    ("ji_ju_tong_lin", [26, 29, 0]),
    ("ji_ju_ju_mao", [16, 17, 0]),
    ("ri_yue_tong_gong", [32, 29, 0]),
    ("ju_ri_tong_gong", [34, 21, 0]),
    ("ri_zhao_lei_men", [20, 25, 0]),
    ("ri_yue_bing_ming", [76, 80, 180]),
    ("ri_yue_fan_bei", [80, 79, 0]),
    ("ri_yue_zhao_bi", [24, 33, 0]),
    ("jin_can_guang_hui", [4, 6, 104]),
    ("ri_yue_cang_hui", [30, 28, 0]),
    ("dan_chi_gui_chi", [40, 42, 0]),
    ("ri_yue_jia_ming", [6, 12, 0]),
    ("ri_yue_jia_cai", [30, 10, 0]),
    ("yue_lang_tian_men", [10, 11, 0]),
    ("yue_sheng_cang_hai", [34, 29, 0]),
    ("ming_zhu_chu_hai", [12, 9, 0]),
    ("wu_tan_tong_xing", [44, 18, 0]),
    ("ling_chang_tuo_wu", [64, 89, 49]),
    ("xing_qiu_jia_yin", [4, 6, 39]),
    ("sheng_bu_feng_shi", [62, 53, 74]),
    ("xiong_su_chao_yuan", [34, 23, 0]),
    ("fu_xiang_chao_yuan", [138, 131, 136]),
    ("huo_tan", [26, 40, 28]),
    ("ling_tan", [70, 55, 18]),
    ("shi_zhong_yin_yu", [46, 28, 140]),
    ("liang_ma_piao_dang", [18, 9, 0]),
    ("yang_liang_chang_lu", [94, 81, 82]),
    ("sha_po_lang", [550, 390, 408]),
    ("qi_sha_chao_dou", [38, 44, 208]),
    ("lu_shuai_ma_kun", [0, 107, 145]),
    ("ying_xing_ru_miao", [22, 31, 130]),
    ("zhong_shui_chao_dong", [8, 1, 0]),
    ("san_qi_jia_hui", [110, 41, 0]),
    ("lu_ma_jiao_chi", [276, 816, 1560]),
    ("lu_he_yuan_yang", [46, 56, 97]),
    ("ming_lu_an_lu", [18, 43, 33]),
    ("lu_ma_pei_yin", [4, 25, 91]),
    ("liang_chong_hua_gai", [4, 5, 10]),
    ("feng_yun_ji_hui", [0, 1244, 0]),
    ("yang_tuo_jia_ming", [130, 233, 563]),
    ("ma_tou_dai_jian", [6, 9, 91]),
    ("zuo_you_tong_gong", [0, 0, 0]),
    ("zuo_you_jia_ming", [0, 0, 0]),
    ("fu_bi_gong_zhu", [22, 18, 0]),
    ("kui_yue_jia_ming", [50, 125, 0]),
    ("zuo_gui_xiang_gui", [78, 120, 0]),
    ("jie_kong_jia_ming", [0, 38, 0]),
    ("lu_feng_liang_sha", [4, 22, 50]),
    ("wen_gui_wen_hua", [204, 154, 58]),
    ("wen_xing_chao_ming", [816, 723, 594]),
    ("chang_qu_jia_ming", [0, 104, 35]),
    ("wen_xing_an_gong", [816, 802, 629]),
    ("quan_lu_sheng_feng", [4, 2, 0]),
    ("ke_ming_an_lu", [14, 39, 0]),
    ("ke_quan_lu_jia", [66, 26, 0]),
    ("jia_di_deng_yong", [24, 32, 0]),
];

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

    // 逐格对照全表金标：每个格局都必须在金标里且计数完全一致
    let golden: BTreeMap<&str, [usize; 3]> = GOLDEN_TALLY.iter().copied().collect();
    assert_eq!(
        golden.len(),
        ALL_PATTERNS.len(),
        "GOLDEN_TALLY 行数应与格局总数一致"
    );
    let diffs: Vec<String> = ALL_PATTERNS
        .into_iter()
        .filter_map(|k| {
            let key = k.as_key();
            let want = golden
                .get(key)
                .unwrap_or_else(|| panic!("GOLDEN_TALLY 缺 {key}"));
            let got = tally[key];
            (got != *want).then(|| format!("  {key}: 实测 {got:?} ≠ 金标 {want:?}"))
        })
        .collect();
    assert!(
        diffs.is_empty(),
        "格局命中盘数与金标不一致（有意改口径时从上方打印的表更新 GOLDEN_TALLY）：\n{}",
        diffs.join("\n")
    );
}

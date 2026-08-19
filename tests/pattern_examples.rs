//! 《格局》页 32 张示例盘的真实盘复现。
//!
//! 页面示例只画了「十二宫地支 + 宫名 + 主星（少数含辅星）」，没有生日、性别、四化、
//! 煞星与杂耀，不能直接当金标。本测试改为反查：按每张示例的「命宫地支 + 命宫主星 +
//! 关键星位」在 1950-2020 逐日（每月 1-28 日）逐时辰、男女两性的枚举里搜出一张真实盘，
//! 再在这张盘上断言对应格局命中、成格宫位正确、多口径格局的 `variant` 正确。
//!
//! 与规则实现的独立性：搜盘条件只用 [`PalaceData`] 的原始星耀列表与地支表达，
//! 不经 `ChartView`，也不以「该格命中」为搜盘条件，故断言不是自证。
//! 页面示例未给煞星信息，而《君臣庆会》《极向离明》两格以「命宫三方四正无煞忌」为前提，
//! 故这几张的搜盘条件额外自行判定无煞（同样只读原始星列表）。
//!
//! 三张示例的预期与其余不同，页面本身即如此，注在各自的测试上：
//! #3 一侧夹宫是空宫，只有借宫口径成立；#9/#10 的太阴在酉，iztro 亮度表判「不」，
//! 默认口径不成格，位置法口径才成格；#15/#16 页面没给吉星与空亡信息，
//! 只能验证「日月夹命宫」这一形态成立，成格与否两种结果都允许并打印。

use std::sync::LazyLock;
use std::thread;

use x_iztro::PalaceData;
use x_iztro::data::types::StarType;
use x_iztro::{
    Astrolabe, BrightnessSource, Config, EarthlyBranch, Gender, Language, Mutagen, Palace,
    PatternConfig, PatternHit, PatternKey, Scope, StarKey, by_solar,
};

use EarthlyBranch::*;
use StarKey::*;

/// 搜盘年份范围：覆盖完整的六十甲子，年系诸星取值齐全。
const SCAN_YEARS: std::ops::Range<i32> = 1950..2021;

/// 每轮并发搜的年数：一轮不够（仍有示例没搜到）再搜下一轮，
/// 结果按年份先后合并，与线程调度无关。
const WAVE: usize = 10;

/// 煞星六曜：《格局》页善荫朝纲一节列举的「煞星」去掉化忌。
const SHA6: [StarKey; 6] = [
    HuoxingMin,
    LingxingMin,
    QingyangMin,
    TuoluoMin,
    DikongMin,
    DijieMin,
];

/// 某地支宫内必须在场的星。
struct At(EarthlyBranch, &'static [StarKey]);

/// 一张页面示例盘的搜盘条件与出处。
struct Example {
    /// 页面示例编号（《格局》页示例盘统计表的 #）
    id: usize,
    /// 所属格局的中文名，失败信息里用
    label: &'static str,
    /// 命宫地支
    soul: EarthlyBranch,
    /// 各宫必须在场的星（含命宫本身）
    stars: &'static [At],
    /// 页面标「空」的宫：必须无主星
    empty: &'static [EarthlyBranch],
    /// 是否要求命宫三方四正无煞忌（该格的成格前提，页面示例未给煞星信息）
    no_sha: bool,
}

/// 32 张示例盘，编号与《格局》页示例盘统计表一致。
static EXAMPLES: &[Example] = &[
    Example {
        id: 1,
        label: "君臣庆会 A",
        soul: Wei,
        stars: &[
            At(Wei, &[ZiweiMaj, PojunMaj]),
            At(Wu, &[TianjiMaj, ZuofuMin]),
            At(Shen, &[YoubiMin]),
        ],
        empty: &[],
        no_sha: true,
    },
    Example {
        id: 2,
        label: "君臣庆会 B",
        soul: Xu,
        stars: &[
            At(Xu, &[ZiweiMaj, TianxiangMaj, WenchangMin]),
            At(Chen, &[PojunMaj, WenquMin]),
        ],
        empty: &[],
        no_sha: true,
    },
    Example {
        id: 3,
        label: "君臣庆会 C",
        soul: Hai,
        stars: &[
            At(Hai, &[TianfuMaj]),
            At(Zi, &[TiantongMaj, TaiyinMaj]),
            At(Chen, &[TianjiMaj, TianliangMaj]),
        ],
        empty: &[Xu],
        no_sha: true,
    },
    Example {
        id: 4,
        label: "紫府同宫",
        soul: Yin,
        stars: &[At(Yin, &[ZiweiMaj, TianfuMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 5,
        label: "金舆扶驾",
        soul: Wei,
        stars: &[
            At(Wei, &[TianfuMaj]),
            At(Wu, &[TaiyangMaj]),
            At(Shen, &[TianjiMaj, TaiyinMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 6,
        label: "紫府夹命",
        soul: Yin,
        stars: &[
            At(Yin, &[TianjiMaj, TaiyinMaj]),
            At(Mao, &[ZiweiMaj, TanlangMaj]),
            At(Chou, &[TianfuMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 7,
        label: "极向离明",
        soul: Wu,
        stars: &[At(Wu, &[ZiweiMaj])],
        empty: &[],
        no_sha: true,
    },
    Example {
        id: 8,
        label: "极居卯酉",
        soul: Mao,
        stars: &[At(Mao, &[ZiweiMaj, TanlangMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 9,
        label: "日月并明",
        soul: Chou,
        stars: &[
            At(Chou, &[TianliangMaj]),
            At(EarthlyBranch::Si, &[TaiyangMaj]),
            At(You, &[TaiyinMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 10,
        label: "丹墀桂墀",
        soul: EarthlyBranch::Si,
        stars: &[At(EarthlyBranch::Si, &[TaiyangMaj]), At(You, &[TaiyinMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 11,
        label: "日月反背",
        soul: Wei,
        stars: &[
            At(Wei, &[TianliangMaj]),
            At(Mao, &[TaiyinMaj]),
            At(Hai, &[TaiyangMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 12,
        label: "日月照璧",
        soul: Xu,
        stars: &[At(Xu, &[PojunMaj]), At(Chou, &[TaiyangMaj, TaiyinMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 13,
        label: "金灿光辉",
        soul: Wu,
        stars: &[At(Wu, &[TaiyangMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 14,
        label: "日月藏辉",
        soul: Hai,
        stars: &[
            At(Hai, &[TaiyangMaj]),
            At(Mao, &[TaiyinMaj]),
            At(EarthlyBranch::Si, &[JumenMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 15,
        label: "日月夹命 ①",
        soul: Chou,
        stars: &[
            At(Chou, &[WuquMaj, TanlangMaj]),
            At(Zi, &[TiantongMaj, TaiyinMaj]),
            At(Yin, &[TaiyangMaj, JumenMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 16,
        label: "日月夹命 ②",
        soul: Wei,
        stars: &[
            At(Wei, &[TianfuMaj]),
            At(Wu, &[TaiyangMaj]),
            At(Shen, &[TianjiMaj, TaiyinMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 17,
        label: "月朗天门",
        soul: Hai,
        stars: &[At(Hai, &[TaiyinMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 18,
        label: "月生沧海（水澄桂萼）",
        soul: Zi,
        stars: &[At(Zi, &[TiantongMaj, TaiyinMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 19,
        label: "月生沧海",
        soul: You,
        stars: &[
            At(You, &[LianzhenMaj, PojunMaj]),
            At(Zi, &[TiantongMaj, TaiyinMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 20,
        label: "明珠出海",
        soul: Wei,
        stars: &[
            At(Chou, &[TiantongMaj, JumenMaj]),
            At(Mao, &[TaiyangMaj, TianliangMaj]),
            At(Hai, &[TaiyinMaj]),
        ],
        empty: &[Wei],
        no_sha: false,
    },
    Example {
        id: 21,
        label: "武贪同行",
        soul: Wei,
        stars: &[At(Wei, &[WuquMaj, TanlangMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 22,
        label: "雄宿朝元",
        soul: Shen,
        stars: &[At(Shen, &[LianzhenMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 23,
        label: "府相朝垣 ①",
        soul: Chen,
        stars: &[
            At(Chen, &[WuquMaj]),
            At(Shen, &[ZiweiMaj, TianfuMaj]),
            At(Zi, &[LianzhenMaj, TianxiangMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 24,
        label: "府相朝垣 ②",
        soul: Shen,
        stars: &[
            At(Shen, &[LianzhenMaj]),
            At(Zi, &[WuquMaj, TianfuMaj]),
            At(Chen, &[ZiweiMaj, TianxiangMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 25,
        label: "府相朝垣 ③",
        soul: You,
        stars: &[
            At(Chou, &[TianfuMaj]),
            At(EarthlyBranch::Si, &[TianxiangMaj]),
            At(Mao, &[ZiweiMaj, TanlangMaj]),
        ],
        empty: &[You],
        no_sha: false,
    },
    Example {
        id: 26,
        label: "府相朝垣 ④",
        soul: Wu,
        stars: &[
            At(Wu, &[ZiweiMaj]),
            At(Xu, &[LianzhenMaj, TianfuMaj]),
            At(Yin, &[WuquMaj, TianxiangMaj]),
        ],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 27,
        label: "石中隐玉",
        soul: Zi,
        stars: &[At(Zi, &[JumenMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 28,
        label: "七杀朝斗 ①",
        soul: Yin,
        stars: &[At(Yin, &[QishaMaj]), At(Shen, &[ZiweiMaj, TianfuMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 29,
        label: "七杀朝斗 ②",
        soul: Wu,
        stars: &[At(Wu, &[QishaMaj]), At(Zi, &[WuquMaj, TianfuMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 30,
        label: "英星入庙",
        soul: Wu,
        stars: &[At(Wu, &[PojunMaj])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 31,
        label: "众水朝东 ①",
        soul: Yin,
        stars: &[At(Yin, &[PojunMaj, WenquMin])],
        empty: &[],
        no_sha: false,
    },
    Example {
        id: 32,
        label: "众水朝东 ②",
        soul: Mao,
        stars: &[At(Mao, &[LianzhenMaj, PojunMaj, WenquMin])],
        empty: &[],
        no_sha: false,
    },
];

// ---------- 搜盘 ----------

/// 按地支取宫。
fn at(a: &Astrolabe, branch: EarthlyBranch) -> &PalaceData {
    a.palaces
        .iter()
        .find(|p| p.earthly_branch == branch)
        .expect("十二宫地支齐全")
}

/// 命宫。
fn soul(a: &Astrolabe) -> &PalaceData {
    a.palaces
        .iter()
        .find(|p| p.name == Palace::Soul)
        .expect("命宫存在")
}

/// 宫内有主星（`StarType::Major`）。
fn has_major(p: &PalaceData) -> bool {
    p.major_stars.iter().any(|s| s.star_type == StarType::Major)
}

/// 命宫三方四正无煞星、无生年化忌 —— 不经 `ChartView`，只读宫内原始星列表。
fn no_sha(a: &Astrolabe) -> bool {
    let s = soul(a).index;
    [s, (s + 4) % 12, (s + 6) % 12, (s + 8) % 12]
        .into_iter()
        .all(|i| {
            let p = &a.palaces[i];
            !p.has_one_of(&SHA6)
                && !p
                    .major_stars
                    .iter()
                    .chain(&p.minor_stars)
                    .chain(&p.adjective_stars)
                    .any(|s| s.mutagen == Some(Mutagen::Ji))
        })
}

/// 该盘是否符合示例的星位描述。
fn matches(a: &Astrolabe, e: &Example) -> bool {
    soul(a).earthly_branch == e.soul
        && e.stars.iter().all(|At(b, stars)| at(a, *b).has(stars))
        && e.empty.iter().all(|b| !has_major(at(a, *b)))
        && (!e.no_sha || no_sha(a))
}

/// 在一年内逐日逐时辰、男女枚举，为每个示例记下第一张符合的盘；本年内全部找齐即提前结束。
fn scan_year(year: i32) -> Vec<Option<Astrolabe>> {
    let mut found: Vec<Option<Astrolabe>> = EXAMPLES.iter().map(|_| None).collect();
    let mut remaining = EXAMPLES.len();
    for month in 1..=12 {
        for day in 1..=28 {
            for time_index in 0..12u8 {
                for gender in [Gender::Male, Gender::Female] {
                    let a = by_solar(
                        &format!("{year}-{month}-{day}"),
                        time_index,
                        gender,
                        true,
                        Language::ZhCN,
                        Config::default(),
                    )
                    .expect("枚举内的日期均合法");
                    for (slot, e) in found.iter_mut().zip(EXAMPLES) {
                        if slot.is_none() && matches(&a, e) {
                            *slot = Some(a.clone());
                            remaining -= 1;
                        }
                    }
                    if remaining == 0 {
                        return found;
                    }
                }
            }
        }
    }
    found
}

/// 全部示例的搜盘结果，下标与 [`EXAMPLES`] 对齐；搜不到为 `None`。
///
/// 按年分派线程并发搜，逐轮合并：同一个示例取年份最早的那一张，
/// 故结果与线程调度无关，可复现。
static CHARTS: LazyLock<Vec<Option<Astrolabe>>> = LazyLock::new(|| {
    let years: Vec<i32> = SCAN_YEARS.collect();
    let mut found: Vec<Option<Astrolabe>> = EXAMPLES.iter().map(|_| None).collect();
    for wave in years.chunks(WAVE) {
        let results: Vec<Vec<Option<Astrolabe>>> = thread::scope(|scope| {
            let handles: Vec<_> = wave
                .iter()
                .map(|&y| scope.spawn(move || scan_year(y)))
                .collect();
            handles.into_iter().map(|h| h.join().unwrap()).collect()
        });
        for year_result in results {
            for (slot, chart) in found.iter_mut().zip(year_result) {
                if slot.is_none() {
                    *slot = chart;
                }
            }
        }
        if found.iter().all(Option::is_some) {
            break;
        }
    }
    found
});

/// 取第 `id` 张示例盘；搜不到时打印原因并返回 `None`（由调用方跳过该张）。
///
/// 跳过前先确认该格局本身能命中——在小范围枚举里找一张命中盘，找不到即判失败：
/// 「示例盘搜不到」只允许是星位组合稀有，不允许是规则永不命中。
fn example(id: usize, key: PatternKey) -> Option<&'static Astrolabe> {
    let idx = EXAMPLES
        .iter()
        .position(|e| e.id == id)
        .expect("示例编号存在");
    match &CHARTS[idx] {
        Some(a) => Some(a),
        None => {
            let label = EXAMPLES[idx].label;
            assert!(
                rule_is_alive(key),
                "示例 #{id}（{label}）没搜到，且 {} 在 1950-1959 的枚举里一次都不命中：\
                 应先排查规则，而非当作稀有星位跳过",
                key.as_key()
            );
            eprintln!(
                "示例 #{id}（{label}）在 {}-{} 的枚举里没有符合星位的真实盘，跳过；\
                 {} 本身可命中（已验证）",
                SCAN_YEARS.start,
                SCAN_YEARS.end - 1,
                key.as_key()
            );
            None
        }
    }
}

/// 该格局在 1950-1959 的枚举里是否至少命中过一次。
fn rule_is_alive(key: PatternKey) -> bool {
    (1950..1960).any(|year| {
        (1..=12).any(|month| {
            (1..=28).any(|day| {
                (0..12u8).any(|time_index| {
                    by_solar(
                        &format!("{year}-{month}-{day}"),
                        time_index,
                        Gender::Male,
                        true,
                        Language::ZhCN,
                        Config::default(),
                    )
                    .expect("枚举内的日期均合法")
                    .patterns()
                    .iter()
                    .any(|h| h.key == key)
                })
            })
        })
    })
}

// ---------- 断言辅助 ----------

/// 该盘上某格局的全部命中（默认口径）。
fn hits(a: &Astrolabe, key: PatternKey) -> Vec<PatternHit> {
    a.patterns().into_iter().filter(|h| h.key == key).collect()
}

/// 指定口径下某格局的全部命中。
fn hits_with(a: &Astrolabe, key: PatternKey, config: &PatternConfig) -> Vec<PatternHit> {
    a.patterns_with(config)
        .into_iter()
        .filter(|h| h.key == key)
        .collect()
}

/// 断言命中唯一一次，返回该命中；同时核对是本命视角。
fn one(a: &Astrolabe, key: PatternKey, id: usize) -> PatternHit {
    let mut h = hits(a, key);
    assert_eq!(
        h.len(),
        1,
        "示例 #{id}：{} 应恰好命中一次，实际 {} 次",
        key.as_key(),
        h.len()
    );
    let hit = h.pop().unwrap();
    assert_eq!(hit.scope, Scope::Origin);
    assert!(!hit.stars.is_empty(), "示例 #{id}：命中证据不应为空");
    hit
}

/// 断言命中落在命宫。
fn in_soul(a: &Astrolabe, hit: &PatternHit, id: usize) {
    assert_eq!(
        hit.palace,
        soul(a).index,
        "示例 #{id}：{} 应成格于命宫",
        hit.key.as_key()
    );
}

/// 一张示例盘的一句话标识，失败与跳过信息里用。
fn describe(a: &Astrolabe) -> String {
    format!(
        "{} 时辰{} {}",
        a.solar_date,
        a.time_index,
        match a.gender {
            Gender::Male => "男",
            Gender::Female => "女",
        }
    )
}

/// 最常见的一类断言：命宫成格、无 variant 或指定 variant。
fn assert_soul_pattern(id: usize, key: PatternKey, variant: Option<&str>) {
    let Some(a) = example(id, key) else { return };
    let hit = one(a, key, id);
    in_soul(a, &hit, id);
    assert_eq!(
        hit.variant,
        variant,
        "示例 #{id}（{}）：variant 不符",
        describe(a)
    );
}

// ---------- 逐张示例 ----------

#[test]
fn example_1_jun_chen_qing_hui_zi_po() {
    assert_soul_pattern(1, PatternKey::JunChenQingHui, Some("zi_po_zuo_you_jia"));
}

#[test]
fn example_2_jun_chen_qing_hui_zi_xiang() {
    let key = PatternKey::JunChenQingHui;
    let Some(a) = example(2, key) else { return };
    let hit = hits(a, key)
        .into_iter()
        .find(|h| h.variant == Some("zi_xiang_chang_qu_axis"))
        .unwrap_or_else(|| panic!("示例 #2（{}）：紫相昌曲形式应命中", describe(a)));
    in_soul(a, &hit, 2);
    // 文昌在命宫、文曲在迁移宫，命迁线上各一颗
    let axis = [soul(a).index, (soul(a).index + 6) % 12];
    for star in [WenchangMin, WenquMin] {
        let at = hit
            .stars
            .iter()
            .find(|s| s.star == star)
            .expect("证据含昌曲");
        assert!(axis.contains(&at.palace), "示例 #2：{star:?} 应在命迁线上");
    }
}

/// #3 的兄弟宫是空宫，只有借对宫的机梁才够成「机阴、同阴相夹天府」，
/// 故默认（借宫开）成格、关掉借宫即不成格。
#[test]
fn example_3_jun_chen_qing_hui_tian_fu_borrows_opposite() {
    let key = PatternKey::JunChenQingHui;
    let Some(a) = example(3, key) else { return };
    let variant = Some("tian_fu_ji_yin_tong_liang_jia");
    let hit = hits(a, key)
        .into_iter()
        .find(|h| h.variant == variant)
        .unwrap_or_else(|| panic!("示例 #3（{}）：天府夹形式应命中", describe(a)));
    in_soul(a, &hit, 3);

    let no_borrow = PatternConfig {
        borrow: false,
        ..PatternConfig::default()
    };
    assert!(
        !hits_with(a, key, &no_borrow)
            .iter()
            .any(|h| h.variant == variant),
        "示例 #3：关掉借宫后天府夹形式不应成立（一侧夹宫是空宫）"
    );
}

#[test]
fn example_4_zi_fu_tong_gong() {
    assert_soul_pattern(4, PatternKey::ZiFuTongGong, None);
}

#[test]
fn example_5_jin_yu_fu_jia() {
    assert_soul_pattern(5, PatternKey::JinYuFuJia, None);
}

#[test]
fn example_6_zi_fu_jia_ming() {
    assert_soul_pattern(6, PatternKey::ZiFuJiaMing, None);
}

#[test]
fn example_7_ji_xiang_li_ming() {
    assert_soul_pattern(7, PatternKey::JiXiangLiMing, None);
}

#[test]
fn example_8_ji_ju_mao_you() {
    assert_soul_pattern(8, PatternKey::JiJuMaoYou, None);
}

/// #9 的太阴在酉：iztro 亮度表判「不」，默认口径不成格；位置法（酉至丑为月明）才成格。
#[test]
fn example_9_ri_yue_bing_ming_needs_positional_brightness() {
    let key = PatternKey::RiYueBingMing;
    let Some(a) = example(9, key) else { return };
    assert!(
        hits(a, key).is_empty(),
        "示例 #9（{}）：默认亮度表口径下太阴酉为「不」，不应成格",
        describe(a)
    );
    let positional = PatternConfig {
        brightness_source: BrightnessSource::Positional,
        ..PatternConfig::default()
    };
    let hit = hits_with(a, key, &positional)
        .pop()
        .unwrap_or_else(|| panic!("示例 #9（{}）：位置法口径下应成格", describe(a)));
    in_soul(a, &hit, 9);
    let moon = hit
        .stars
        .iter()
        .find(|s| s.star == TaiyinMaj)
        .expect("证据含太阴");
    assert_eq!(a.palaces[moon.palace].earthly_branch, You);
}

/// #10 同 #9：太阴在酉，默认口径不成格，位置法下丹墀（命宫太阳明）成立。
#[test]
fn example_10_dan_chi_gui_chi_needs_positional_brightness() {
    let key = PatternKey::DanChiGuiChi;
    let Some(a) = example(10, key) else { return };
    assert!(
        hits(a, key).is_empty(),
        "示例 #10（{}）：默认亮度表口径下不应成格",
        describe(a)
    );
    let positional = PatternConfig {
        brightness_source: BrightnessSource::Positional,
        ..PatternConfig::default()
    };
    let hit = hits_with(a, key, &positional)
        .pop()
        .unwrap_or_else(|| panic!("示例 #10（{}）：位置法口径下应成格", describe(a)));
    in_soul(a, &hit, 10);
}

#[test]
fn example_11_ri_yue_fan_bei() {
    assert_soul_pattern(11, PatternKey::RiYueFanBei, None);
}

/// #12 的日月在田宅宫（丑），成格宫位是田宅而非命宫。
#[test]
fn example_12_ri_yue_zhao_bi_in_property_palace() {
    let key = PatternKey::RiYueZhaoBi;
    let Some(a) = example(12, key) else { return };
    let hit = one(a, key, 12);
    let property = a
        .palaces
        .iter()
        .find(|p| p.name == Palace::Property)
        .expect("田宅宫存在");
    assert_eq!(hit.palace, property.index);
    assert_eq!(property.earthly_branch, Chou);
}

#[test]
fn example_13_jin_can_guang_hui() {
    let key = PatternKey::JinCanGuangHui;
    let Some(a) = example(13, key) else { return };
    let hit = one(a, key, 13);
    in_soul(a, &hit, 13);
    // 页面称三方四正见煞忌为「破格」，成格照报，此处只打印实际取值
    eprintln!("示例 #13（{}）：broken = {}", describe(a), hit.broken);
}

#[test]
fn example_14_ri_yue_cang_hui() {
    let key = PatternKey::RiYueCangHui;
    let Some(a) = example(14, key) else { return };
    let hit = one(a, key, 14);
    in_soul(a, &hit, 14);
    assert!(
        hit.stars.iter().any(|s| s.star == JumenMaj),
        "示例 #14：证据应含巨门"
    );
}

/// #15、#16 页面只画了主星，没给命宫的吉星与空亡信息，而《日月夹命》要求
/// 「不坐空亡 + 本宫有吉星」，故只验证「日月分居命宫前后邻宫」这一形态成立，
/// 成格与否两种结果都允许，实际取值打印出来。
#[test]
fn example_15_and_16_ri_yue_jia_ming_shape() {
    for id in [15, 16] {
        let key = PatternKey::RiYueJiaMing;
        let Some(a) = example(id, key) else { continue };
        let s = soul(a).index;
        let (prev, next) = (&a.palaces[(s + 11) % 12], &a.palaces[(s + 1) % 12]);
        assert!(
            (prev.has(&[TaiyangMaj]) && next.has(&[TaiyinMaj]))
                || (prev.has(&[TaiyinMaj]) && next.has(&[TaiyangMaj])),
            "示例 #{id}（{}）：日月应分居命宫前后邻宫",
            describe(a)
        );
        let hit = hits(a, key).pop();
        if let Some(hit) = &hit {
            in_soul(a, hit, id);
        }
        eprintln!(
            "示例 #{id}（{}）：日月夹命形态成立，吉星/空亡条件{}",
            describe(a),
            if hit.is_some() {
                "满足，成格"
            } else {
                "不满足，未成格（页面未给这两项信息）"
            }
        );
    }
}

#[test]
fn example_17_yue_lang_tian_men() {
    assert_soul_pattern(17, PatternKey::YueLangTianMen, None);
}

#[test]
fn example_18_yue_sheng_cang_hai_in_soul() {
    assert_soul_pattern(18, PatternKey::YueShengCangHai, Some("soul"));
}

/// #19 的同阴在子宫且落田宅，是全书原文的「月在子宫守田宅」形态。
#[test]
fn example_19_yue_sheng_cang_hai_in_property() {
    let key = PatternKey::YueShengCangHai;
    let Some(a) = example(19, key) else { return };
    let hit = hits(a, key)
        .into_iter()
        .find(|h| h.variant == Some("property"))
        .unwrap_or_else(|| panic!("示例 #19（{}）：田宅形态应命中", describe(a)));
    let property = a
        .palaces
        .iter()
        .find(|p| p.name == Palace::Property)
        .expect("田宅宫存在");
    assert_eq!(hit.palace, property.index);
    assert_eq!(property.earthly_branch, Zi);
}

#[test]
fn example_20_ming_zhu_chu_hai() {
    let key = PatternKey::MingZhuChuHai;
    let Some(a) = example(20, key) else { return };
    let hit = one(a, key, 20);
    in_soul(a, &hit, 20);
    // 证据含迁移宫的同巨与三方四正的日月
    for star in [TiantongMaj, JumenMaj, TaiyangMaj, TaiyinMaj] {
        assert!(
            hit.stars.iter().any(|s| s.star == star),
            "示例 #20：证据应含 {star:?}"
        );
    }
}

#[test]
fn example_21_wu_tan_tong_xing() {
    let key = PatternKey::WuTanTongXing;
    let Some(a) = example(21, key) else { return };
    // 武贪同守命宫；身宫若也在此宫，规则按命身各报一次
    let hits = hits(a, key);
    assert!(
        hits.iter().any(|h| h.palace == soul(a).index),
        "示例 #21（{}）：应在命宫成格",
        describe(a)
    );
}

#[test]
fn example_22_xiong_su_chao_yuan() {
    let key = PatternKey::XiongSuChaoYuan;
    let Some(a) = example(22, key) else { return };
    let hit = one(a, key, 22);
    in_soul(a, &hit, 22);
    eprintln!("示例 #22（{}）：broken = {}", describe(a), hit.broken);
}

#[test]
fn example_23_fu_xiang_chao_yuan() {
    assert_soul_pattern(23, PatternKey::FuXiangChaoYuan, None);
}

#[test]
fn example_24_fu_xiang_chao_yuan() {
    assert_soul_pattern(24, PatternKey::FuXiangChaoYuan, None);
}

/// #25 的命宫是空宫（页面「命无主星」形态），规则以 `variant = "soul_empty"` 标出。
#[test]
fn example_25_fu_xiang_chao_yuan_soul_empty() {
    let key = PatternKey::FuXiangChaoYuan;
    let Some(a) = example(25, key) else { return };
    let hit = one(a, key, 25);
    in_soul(a, &hit, 25);
    assert_eq!(hit.variant, Some("soul_empty"));
    assert!(!has_major(soul(a)), "示例 #25：命宫应为空宫");
}

#[test]
fn example_26_fu_xiang_chao_yuan() {
    assert_soul_pattern(26, PatternKey::FuXiangChaoYuan, None);
}

#[test]
fn example_27_shi_zhong_yin_yu() {
    let key = PatternKey::ShiZhongYinYu;
    let Some(a) = example(27, key) else { return };
    assert!(
        hits(a, key).iter().any(|h| h.palace == soul(a).index),
        "示例 #27（{}）：应在命宫成格",
        describe(a)
    );
}

#[test]
fn example_28_qi_sha_chao_dou_yang_dou() {
    assert_soul_pattern(28, PatternKey::QiShaChaoDou, Some("yang_dou"));
}

#[test]
fn example_29_qi_sha_chao_dou_chao_dou() {
    assert_soul_pattern(29, PatternKey::QiShaChaoDou, Some("chao_dou"));
}

#[test]
fn example_30_ying_xing_ru_miao() {
    assert_soul_pattern(30, PatternKey::YingXingRuMiao, None);
}

#[test]
fn example_31_zhong_shui_chao_dong() {
    assert_soul_pattern(31, PatternKey::ZhongShuiChaoDong, None);
}

#[test]
fn example_32_zhong_shui_chao_dong() {
    assert_soul_pattern(32, PatternKey::ZhongShuiChaoDong, None);
}

/// 搜盘结果总览：哪几张搜到、用的是哪张真实盘、哪几张没搜到。
#[test]
fn example_charts_are_reported() {
    let mut missing = Vec::new();
    for (e, chart) in EXAMPLES.iter().zip(CHARTS.iter()) {
        match chart {
            Some(a) => eprintln!("#{:<2} {:<20} {}", e.id, e.label, describe(a)),
            None => {
                eprintln!("#{:<2} {:<20} 未搜到", e.id, e.label);
                missing.push(e.id);
            }
        }
    }
    eprintln!(
        "示例盘 {}/{} 搜到，未搜到：{missing:?}",
        EXAMPLES.len() - missing.len(),
        EXAMPLES.len()
    );
}

//! 反推的往返一致性与入参校验。
//!
//! 反推没有外部金标，正确性由「与正排零分歧」定义，往返测试即金标：
//! 任取一盘，其四柱反查的候选必含原生辰、其特征反查的候选必含原生辰，
//! 且每个候选正排后必须真的得出目标四柱 / 满足全部特征。

use x_iztro::utils::get_mutagens_by_heavenly_stem;
use x_iztro::*;

/// 覆盖边界形态的一组生辰：普通时辰、早晚子、闰月（2004-03-21 落农历闰二月）、
/// 立春与正月初一之间（2001-02-01）、大小月与世纪边界。
const CASES: &[(&str, u8)] = &[
    ("2000-8-16", 2),
    ("2000-8-16", 0),
    ("2000-8-16", 12),
    ("2004-3-21", 5),
    ("2001-2-1", 3),
    ("1900-3-1", 7),
    ("1999-12-31", 12),
    ("2023-6-19", 11),
];

fn chart(date: &str, t: u8, config: &Config) -> Astrolabe {
    by_solar(
        date,
        t,
        Gender::Female,
        true,
        Language::ZhCN,
        config.clone(),
    )
    .unwrap()
}

#[test]
fn bazi_roundtrip_default_config() {
    let cfg = Config::default();
    for (date, t) in CASES {
        let a = chart(date, *t, &cfg);
        let p = a.raw_dates.chinese_date;
        let got = solar_dates_by_bazi(p.yearly, p.monthly, p.daily, p.hourly, (1890, 2110), &cfg)
            .unwrap();
        assert!(
            got.iter()
                .any(|c| c.solar_date == *date && c.time_index == *t),
            "{date} t{t}: {got:?}"
        );
        // 每个候选正排后的四柱必须与目标完全一致
        for c in &got {
            let b = chart(&c.solar_date, c.time_index, &cfg);
            let q = b.raw_dates.chinese_date;
            assert_eq!(
                (q.yearly, q.monthly, q.daily, q.hourly),
                (p.yearly, p.monthly, p.daily, p.hourly),
                "candidate {c:?} disagrees"
            );
        }
    }
}

#[test]
fn bazi_roundtrip_exact_divide() {
    // 节气分界口径下（立春换年、节气换月），立春前生日的四柱与默认口径不同，
    // 反推按同一 config 仍须闭环。1985-2-10 与 2015-2-18 落在立春之后、春节之前
    //（春节晚于立春的年份），年柱已是下一年干支。
    let cfg = Config {
        year_divide: YearDivide::Exact,
        horoscope_divide: HoroscopeDivide::Exact,
        ..Config::default()
    };
    for (date, t) in [
        ("2001-2-1", 3),
        ("2000-8-16", 12),
        ("2004-3-21", 5),
        ("1985-2-10", 2),
        ("2015-2-18", 12),
    ] {
        let a = chart(date, t, &cfg);
        let p = a.raw_dates.chinese_date;
        let got = solar_dates_by_bazi(p.yearly, p.monthly, p.daily, p.hourly, (1980, 2020), &cfg)
            .unwrap();
        assert!(
            got.iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {got:?}"
        );
    }
}

#[test]
fn bazi_roundtrip_current_day_divide() {
    // 晚子归当天口径：正排把 t>=12 归一为早子，(D,0) 与 (D,12) 四柱完全相同，
    // 子时四柱反查须同时给出两个时辰，且必含原生辰。
    let cfg = Config {
        day_divide: DayDivide::Current,
        ..Config::default()
    };
    for (date, t) in [
        ("2000-8-16", 12),
        ("2000-8-16", 0),
        ("2015-2-18", 12),
        ("1999-12-31", 12),
    ] {
        let a = chart(date, t, &cfg);
        let p = a.raw_dates.chinese_date;
        let got = solar_dates_by_bazi(p.yearly, p.monthly, p.daily, p.hourly, (1998, 2016), &cfg)
            .unwrap();
        assert!(
            got.iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {got:?}"
        );
        // 同日早晚子四柱相同：两个时辰都在候选中
        let twin = if t == 12 { 0 } else { 12 };
        assert!(
            got.iter()
                .any(|c| c.solar_date == date && c.time_index == twin),
            "{date} t{twin}: {got:?}"
        );
        for c in &got {
            let b = chart(&c.solar_date, c.time_index, &cfg);
            let q = b.raw_dates.chinese_date;
            assert_eq!(
                (q.yearly, q.monthly, q.daily, q.hourly),
                (p.yearly, p.monthly, p.daily, p.hourly),
                "candidate {c:?} disagrees"
            );
        }
    }
}

#[test]
fn bazi_solutions_repeat_by_sexagenary_cycle() {
    let cfg = Config::default();
    let a = chart("2000-8-16", 2, &cfg);
    let p = a.raw_dates.chinese_date;
    let got =
        solar_dates_by_bazi(p.yearly, p.monthly, p.daily, p.hourly, (1900, 2100), &cfg).unwrap();
    assert_eq!(got.len(), 3, "{got:?}");
    assert!(got.iter().any(|c| c.solar_date == "1940-8-31"));
    assert!(got.iter().any(|c| c.solar_date == "2000-8-16"));
    assert!(got.iter().any(|c| c.solar_date == "2060-8-1"));
}

#[test]
fn bazi_rejects_bad_input() {
    let cfg = Config::default();
    let jia_zi = (HeavenlyStem::Jia, EarthlyBranch::Zi);
    // 阴阳不配：甲丑
    let err = solar_dates_by_bazi(
        (HeavenlyStem::Jia, EarthlyBranch::Chou),
        jia_zi,
        jia_zi,
        jia_zi,
        (1900, 2100),
        &cfg,
    )
    .unwrap_err();
    assert_eq!(err.code(), "invalid_argument");
    // 范围颠倒 / 越界
    for range in [(2100, 1900), (1000, 2000), (2000, 10000)] {
        let err = solar_dates_by_bazi(jia_zi, jia_zi, jia_zi, jia_zi, range, &cfg).unwrap_err();
        assert_eq!(err.code(), "invalid_argument");
    }
}

/// 从一张盘提取一组特征条件（命身宫、五行局、四化、四颗跨系星耀落宫）。
fn criteria_of(a: &Astrolabe, config: &Config, year_range: (i64, i64)) -> ReverseCriteria {
    let branch_of = |k: StarKey| a.star(k).unwrap().palace().earthly_branch;
    let mutagens = get_mutagens_by_heavenly_stem(a.raw_dates.chinese_date.yearly.0, config);
    ReverseCriteria {
        soul_branch: Some(a.earthly_branch_of_soul_palace),
        body_branch: Some(a.earthly_branch_of_body_palace),
        five_elements_class: Some(a.five_elements_class),
        stars: vec![
            StarPosition {
                star: StarKey::ZiweiMaj,
                branch: branch_of(StarKey::ZiweiMaj),
            },
            StarPosition {
                star: StarKey::QishaMaj,
                branch: branch_of(StarKey::QishaMaj),
            },
            StarPosition {
                star: StarKey::LucunMin,
                branch: branch_of(StarKey::LucunMin),
            },
            StarPosition {
                star: StarKey::ZuofuMin,
                branch: branch_of(StarKey::ZuofuMin),
            },
            StarPosition {
                star: StarKey::WenquMin,
                branch: branch_of(StarKey::WenquMin),
            },
            StarPosition {
                star: StarKey::HuoxingMin,
                branch: branch_of(StarKey::HuoxingMin),
            },
        ],
        mutagens: [Some(mutagens[0]), None, None, Some(mutagens[3])],
        year_range,
        ..Default::default()
    }
}

#[test]
fn reverse_chart_roundtrip() {
    let cfg = Config::default();
    for (date, t) in CASES {
        let a = chart(date, *t, &cfg);
        let year: i64 = date.split('-').next().unwrap().parse().unwrap();
        let crit = criteria_of(&a, &cfg, (year - 1, year + 1));
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == *date && c.time_index == *t),
            "{date} t{t}: {} candidates, truncated={}",
            r.candidates.len(),
            r.truncated
        );
        // 每个候选正排后必须满足全部条件（命身宫、五行局、生年四化、星耀落宫逐维核对）
        for c in &r.candidates {
            let b = chart(&c.solar_date, c.time_index, &cfg);
            assert_eq!(
                b.earthly_branch_of_soul_palace,
                a.earthly_branch_of_soul_palace
            );
            assert_eq!(
                b.earthly_branch_of_body_palace, a.earthly_branch_of_body_palace,
                "candidate {c:?} body palace"
            );
            assert_eq!(b.five_elements_class, a.five_elements_class);
            let bm = get_mutagens_by_heavenly_stem(b.raw_dates.chinese_date.yearly.0, &cfg);
            for (want, got) in crit.mutagens.iter().zip(bm) {
                if let Some(w) = want {
                    assert_eq!(*w, got, "candidate {c:?} mutagen");
                }
            }
            for p in &crit.stars {
                assert_eq!(
                    b.star(p.star).unwrap().palace().earthly_branch,
                    p.branch,
                    "candidate {c:?} star {:?}",
                    p.star
                );
            }
        }
    }
}

#[test]
fn reverse_chart_roundtrip_zhongzhou() {
    // 中州派 + 节气分界：条件判定全程随 config
    let cfg = Config {
        algorithm: Algorithm::Zhongzhou,
        year_divide: YearDivide::Exact,
        ..Config::default()
    };
    let a = chart("2001-2-1", 3, &cfg);
    let crit = criteria_of(&a, &cfg, (2000, 2002));
    let r = reverse_chart(&crit, &cfg).unwrap();
    assert!(
        r.candidates
            .iter()
            .any(|c| c.solar_date == "2001-2-1" && c.time_index == 3),
        "{:?}",
        r.candidates
    );
}

/// 凡是会改变盘面几何的 `Config` 开关，剪枝层都必须跟着走：地盘与人盘在排盘末尾整体重排，
/// 条件按重排后的盘给出，而剪枝几何按天盘安星表算——两端脱钩时真解在终验之前就被剪掉，
/// 且结果看着合理（每条候选单独都满足条件，唯独原生辰不在其中）。
#[test]
fn reverse_chart_roundtrip_across_geometry_configs() {
    let (date, t) = ("2000-8-16", 2);
    let variants: [(&str, Config); 4] = [
        ("天盘", Config::default()),
        (
            "地盘",
            Config {
                astro_type: AstroType::Earth,
                ..Config::default()
            },
        ),
        (
            "人盘",
            Config {
                astro_type: AstroType::Human,
                ..Config::default()
            },
        ),
        (
            "人盘+中州派+节气分界",
            Config {
                astro_type: AstroType::Human,
                algorithm: Algorithm::Zhongzhou,
                year_divide: YearDivide::Exact,
                ..Config::default()
            },
        ),
    ];
    for (name, cfg) in &variants {
        let a = chart(date, t, cfg);
        let crit = criteria_of(&a, cfg, (2000, 2000));
        let r = reverse_chart(&crit, cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{name}：反查未含原生辰，候选 {} 个",
            r.candidates.len()
        );
        // 只给命宫地支这一个条件时同样不许错杀——剪枝对命宫的处理与星耀落宫是两条臂
        let soul_only = ReverseCriteria {
            soul_branch: Some(a.earthly_branch_of_soul_palace),
            year_range: (2000, 2000),
            limit: 4096,
            ..Default::default()
        };
        let r = reverse_chart(&soul_only, cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{name}：只给命宫地支时反查未含原生辰，候选 {} 个",
            r.candidates.len()
        );
    }
}

#[test]
fn reverse_chart_roundtrip_current_late_zi() {
    // 晚子归当天口径 + 日敏感星耀条件（紫微起宫、日系杂耀）：
    // 剪枝须按生效时辰（晚子归一为早子）判定，t=12 的原生辰不得被日层剪枝错杀。
    let cfg = Config {
        day_divide: DayDivide::Current,
        ..Config::default()
    };
    for (date, t) in [("2000-8-16", 12), ("2015-2-18", 12)] {
        let a = chart(date, t, &cfg);
        let year: i64 = date.split('-').next().unwrap().parse().unwrap();
        let pos = |k: StarKey| a.star(k).unwrap().palace().earthly_branch;
        let crit = ReverseCriteria {
            stars: vec![
                StarPosition {
                    star: StarKey::ZiweiMaj,
                    branch: pos(StarKey::ZiweiMaj),
                },
                StarPosition {
                    star: StarKey::Santai,
                    branch: pos(StarKey::Santai),
                },
            ],
            year_range: (year, year),
            ..Default::default()
        };
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {} candidates, truncated={}",
            r.candidates.len(),
            r.truncated
        );
    }
}

#[test]
fn reverse_chart_exact_year_divide_covers_lichun_to_new_year_gap() {
    // 立春分界：春节晚于立春的年份，农历上年腊月里立春之后的日子已用下一年干支
    //（如 1985-02-04 立春至 02-19 除夕段年柱为乙丑）。年干系条件（禄存落宫、生年化禄）
    // 反查须包含这段日子里的原生辰。
    let cfg = Config {
        year_divide: YearDivide::Exact,
        ..Config::default()
    };
    for (date, t) in [("1985-2-10", 2), ("2015-2-18", 12)] {
        let a = chart(date, t, &cfg);
        let year: i64 = date.split('-').next().unwrap().parse().unwrap();
        let mutagens = get_mutagens_by_heavenly_stem(a.raw_dates.chinese_date.yearly.0, &cfg);
        let crit = ReverseCriteria {
            stars: vec![StarPosition {
                star: StarKey::LucunMin,
                branch: a.star(StarKey::LucunMin).unwrap().palace().earthly_branch,
            }],
            mutagens: [Some(mutagens[0]), None, None, None],
            year_range: (year, year),
            ..Default::default()
        };
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {} candidates, truncated={}",
            r.candidates.len(),
            r.truncated
        );
    }
}

#[test]
fn reverse_chart_year_range_is_solar_closed_interval() {
    // year_range 按候选公历日期的年份取闭区间：公历年初（元旦到春节前）的日子
    // 属上一个农历年，也必须可达；候选严格落在区间内。1583 为支持范围首年，
    // 其年初属农历 1582 年，一并守住枚举域首端扩一年的边界。
    let cfg = Config::default();
    for (date, t) in [("2001-1-10", 4), ("1583-1-5", 2)] {
        let a = chart(date, t, &cfg);
        let year: i64 = date.split('-').next().unwrap().parse().unwrap();
        let crit = criteria_of(&a, &cfg, (year, year));
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {:?}",
            r.candidates
        );
        let prefix = format!("{year}-");
        for c in &r.candidates {
            assert!(
                c.solar_date.starts_with(&prefix),
                "candidate {c:?} outside solar year range"
            );
        }
    }
}

#[test]
fn reverse_chart_roundtrip_lunar_1602_leap_month_window() {
    // lunar_rust 1.0.1 把农历 1602 年闰二月首日算晚一天（lunar_table 模块修正）：
    // 公历 1602-03-24（闰二月初一）至 04-21（闰二月廿九）窗口内，枚举的农历日标签
    // 必须与正排同源，日敏感剪枝（紫微起宫、日系杂耀）才不会错杀窗口内的真解。
    let cfg = Config::default();
    for (date, t) in [("1602-3-24", 2), ("1602-4-2", 12), ("1602-4-21", 7)] {
        let a = chart(date, t, &cfg);
        let pos = |k: StarKey| a.star(k).unwrap().palace().earthly_branch;
        let crit = ReverseCriteria {
            soul_branch: Some(a.earthly_branch_of_soul_palace),
            stars: vec![
                StarPosition {
                    star: StarKey::ZiweiMaj,
                    branch: pos(StarKey::ZiweiMaj),
                },
                StarPosition {
                    star: StarKey::Santai,
                    branch: pos(StarKey::Santai),
                },
                StarPosition {
                    star: StarKey::Bazuo,
                    branch: pos(StarKey::Bazuo),
                },
                StarPosition {
                    star: StarKey::Engguang,
                    branch: pos(StarKey::Engguang),
                },
                StarPosition {
                    star: StarKey::Tiangui,
                    branch: pos(StarKey::Tiangui),
                },
            ],
            year_range: (1602, 1602),
            ..Default::default()
        };
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t}: {:?}",
            r.candidates
        );
    }
}

#[test]
fn reverse_chart_limit_truncates() {
    let cfg = Config::default();
    let crit = ReverseCriteria {
        stars: vec![StarPosition {
            star: StarKey::HuoxingMin,
            branch: EarthlyBranch::Yin,
        }],
        year_range: (1900, 2100),
        limit: 50,
        ..Default::default()
    };
    let r = reverse_chart(&crit, &cfg).unwrap();
    assert_eq!(r.candidates.len(), 50);
    assert!(r.truncated);
}

#[test]
fn reverse_chart_rejects_bad_criteria() {
    let cfg = Config::default();
    // 空条件
    let err = reverse_chart(&ReverseCriteria::default(), &cfg).unwrap_err();
    assert_eq!(err.code(), "invalid_argument");
    // 流耀：五个层级（运/流/月/日/时）都必须拒绝——它们不出现在本命盘上，
    // 放行会静默得到空结果，看起来像真的无解
    for star in [
        StarKey::Yunlu,
        StarKey::Liulu,
        StarKey::Yuechang,
        StarKey::Richang,
        StarKey::Shima,
    ] {
        let err = reverse_chart(
            &ReverseCriteria {
                stars: vec![StarPosition {
                    star,
                    branch: EarthlyBranch::Zi,
                }],
                ..Default::default()
            },
            &cfg,
        )
        .unwrap_err();
        assert_eq!(err.code(), "invalid_argument", "{star:?} 应被拒绝");
    }
    // 四组十二神：以每宫单值字段存在、不进入星耀列表，落宫条件恒不可满足，须显式拒绝
    for star in [
        StarKey::Changsheng,
        StarKey::Boshi,
        StarKey::Suijian,
        StarKey::Jiangxing,
    ] {
        let err = reverse_chart(
            &ReverseCriteria {
                stars: vec![StarPosition {
                    star,
                    branch: EarthlyBranch::Zi,
                }],
                ..Default::default()
            },
            &cfg,
        )
        .unwrap_err();
        assert_eq!(err.code(), "invalid_argument", "{star:?}");
    }
    // 范围非法
    let err = reverse_chart(
        &ReverseCriteria {
            soul_branch: Some(EarthlyBranch::Zi),
            year_range: (2100, 1900),
            ..Default::default()
        },
        &cfg,
    )
    .unwrap_err();
    assert_eq!(err.code(), "invalid_argument");
}

/// 反推剪枝有专用分派臂的全部星耀：14 主星（归一化为紫微落宫走日层）、
/// 8 年系（干系禄存/擎羊/陀罗/天魁/天钺与支系天马/红鸾/天喜）、
/// 8 月系与时系（左辅/右弼/文昌/文曲/地空/地劫/火星/铃星）、4 日系杂耀（三台/八座/恩光/天贵）。
const PREFILTER_ARM_STARS: [StarKey; 34] = [
    StarKey::ZiweiMaj,
    StarKey::TianjiMaj,
    StarKey::TaiyangMaj,
    StarKey::WuquMaj,
    StarKey::TiantongMaj,
    StarKey::LianzhenMaj,
    StarKey::TianfuMaj,
    StarKey::TaiyinMaj,
    StarKey::TanlangMaj,
    StarKey::JumenMaj,
    StarKey::TianxiangMaj,
    StarKey::TianliangMaj,
    StarKey::QishaMaj,
    StarKey::PojunMaj,
    StarKey::LucunMin,
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
    StarKey::TianmaMin,
    StarKey::TiankuiMin,
    StarKey::TianyueMin,
    StarKey::Hongluan,
    StarKey::Tianxi,
    StarKey::ZuofuMin,
    StarKey::YoubiMin,
    StarKey::WenchangMin,
    StarKey::WenquMin,
    StarKey::DikongMin,
    StarKey::DijieMin,
    StarKey::HuoxingMin,
    StarKey::LingxingMin,
    StarKey::Santai,
    StarKey::Bazuo,
    StarKey::Engguang,
    StarKey::Tiangui,
];

/// 对一张盘按 [`PREFILTER_ARM_STARS`] 逐颗做单星落宫条件反查，断言原生辰总在候选中：
/// 任何一条剪枝臂都不许错杀真解。盘须取正月出生——年系星条件下整个匹配年皆是解，
/// 候选按 月→时辰→日 枚举序给出，正月出生的原生辰位次至多约 400，
/// limit 取 2000 既留足余量又避免整年全量枚举拖慢测试。
fn assert_single_star_ring(date: &str, t: u8) {
    let cfg = Config::default();
    let a = chart(date, t, &cfg);
    let year: i64 = date.split('-').next().unwrap().parse().unwrap();
    for key in PREFILTER_ARM_STARS {
        let branch = a.star(key).unwrap().palace().earthly_branch;
        let crit = ReverseCriteria {
            stars: vec![StarPosition { star: key, branch }],
            year_range: (year, year),
            limit: 2000,
            ..Default::default()
        };
        let r = reverse_chart(&crit, &cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{date} t{t} star {key:?}: {} candidates, truncated={}",
            r.candidates.len(),
            r.truncated
        );
    }
}

#[test]
fn reverse_single_star_ring_first_month_chart() {
    // 2000-02-10 = 农历 2000 年正月初六
    assert_single_star_ring("2000-2-10", 2);
}

#[test]
fn reverse_single_star_ring_late_zi_chart() {
    // 1988-03-01 = 农历 1988 年正月十四，晚子时
    assert_single_star_ring("1988-3-1", 12);
}

#[test]
fn reverse_single_star_ring_early_zi_chart() {
    // 2024-03-01 = 农历 2024 年正月廿一，早子时
    assert_single_star_ring("2024-3-1", 0);
}

/// 与宫内杂耀共用 key 的五颗十二神不许被校验层错杀：咸池/华盖/天德每盘安放、
/// 龙德/大耗中州派盘安放，均可按杂耀落宫作反推条件，原生辰必在候选中。
/// 纯十二神 key（不兼杂耀者）的拒绝由入参校验测试另行覆盖。
#[test]
fn reverse_accepts_twelve_gods_keys_shared_with_adjective_stars() {
    // 2000-02-10 = 农历 2000 年正月初六（正月盘保证枚举位次落在 limit 内）
    let (date, t) = ("2000-2-10", 2);
    let default_cfg = Config::default();
    let zhongzhou_cfg = Config {
        algorithm: Algorithm::Zhongzhou,
        ..Config::default()
    };
    let cases: [(StarKey, &Config); 5] = [
        (StarKey::Xianchi, &default_cfg),
        (StarKey::Huagai, &default_cfg),
        (StarKey::Tiande, &default_cfg),
        (StarKey::Longde, &zhongzhou_cfg),
        (StarKey::Dahao, &zhongzhou_cfg),
    ];
    for (key, cfg) in cases {
        let a = chart(date, t, cfg);
        let branch = a.star(key).unwrap().palace().earthly_branch;
        let crit = ReverseCriteria {
            stars: vec![StarPosition { star: key, branch }],
            year_range: (2000, 2000),
            limit: 2000,
            ..Default::default()
        };
        let r = reverse_chart(&crit, cfg).unwrap();
        assert!(
            r.candidates
                .iter()
                .any(|c| c.solar_date == date && c.time_index == t),
            "{key:?}: {} candidates, truncated={}",
            r.candidates.len(),
            r.truncated
        );
    }
}

// ============================================================
// 绑定入口（统一查询 kind）
// ============================================================

mod ffi_kinds {
    use std::ffi::{CStr, CString};

    use serde_json::{Value, json};
    use x_iztro::ffi::{iztro_free_string, iztro_query};

    fn query(payload: Value) -> Result<Value, String> {
        let input = CString::new(payload.to_string()).unwrap();
        let ptr = unsafe { iztro_query(input.as_ptr()) };
        let out = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap().to_string();
        unsafe { iztro_free_string(ptr) };
        let out: Value = serde_json::from_str(&out).unwrap();
        match out.get("error") {
            Some(e) => Err(e.as_str().unwrap_or_default().to_string()),
            None => Ok(out["value"].clone()),
        }
    }

    #[test]
    fn solar_dates_by_bazi_kind() {
        let got = query(json!({
            "kind": "solarDatesByBazi",
            "pillars": [["gengHeavenly", "chenEarthly"], ["jiaHeavenly", "shenEarthly"], ["bingHeavenly", "wuEarthly"], ["gengHeavenly", "yinEarthly"]],
            "startYear": 1900,
            "endYear": 2100,
        }))
        .unwrap();
        let arr = got.as_array().unwrap();
        assert_eq!(arr.len(), 3);
        assert!(
            arr.iter()
                .any(|c| c["solarDate"] == "2000-8-16" && c["timeIndex"] == 2)
        );
        let err = query(json!({
            "kind": "solarDatesByBazi",
            "pillars": [["jiaHeavenly", "chouEarthly"], ["jiaHeavenly", "ziEarthly"], ["jiaHeavenly", "ziEarthly"], ["jiaHeavenly", "ziEarthly"]],
            "startYear": 1900,
            "endYear": 2100,
        }))
        .unwrap_err();
        assert!(err.contains("polarity"), "{err}");
    }

    #[test]
    fn reverse_chart_kind() {
        let got = query(json!({
            "kind": "reverseChart",
            "reverseCriteria": {
                "soulBranch": "wuEarthly",
                "fiveElementsClass": "wood3rd",
                "stars": [{"star": "ziweiMaj", "branch": "wuEarthly"}],
                "mutagens": ["taiyangMaj", null, null, null],
                "yearRange": [1998, 2002],
            },
        }))
        .unwrap();
        assert!(
            got["candidates"]
                .as_array()
                .unwrap()
                .iter()
                .any(|c| c["solarDate"] == "2000-8-16" && c["timeIndex"] == 2),
            "{got}"
        );
        assert_eq!(got["truncated"], false);
        let err =
            query(json!({"kind": "reverseChart", "reverseCriteria": {"nope": 1}})).unwrap_err();
        assert!(err.contains("unknown reverseCriteria key"), "{err}");
        let err = query(json!({"kind": "reverseChart"})).unwrap_err();
        assert!(err.contains("required"), "{err}");
    }
}

#[test]
fn reverse_chart_contradictory_majors_short_circuit() {
    // 紫微在子与天府在丑几何上互斥（天府位恒为 12-紫微位）：
    // 主星条件归一化后应立即判空，而不是枚举完整个年份范围。
    let cfg = Config::default();
    let crit = ReverseCriteria {
        stars: vec![
            StarPosition {
                star: StarKey::ZiweiMaj,
                branch: EarthlyBranch::Zi,
            },
            StarPosition {
                star: StarKey::TianfuMaj,
                branch: EarthlyBranch::Chou,
            },
        ],
        year_range: (1583, 9999),
        ..Default::default()
    };
    let t = std::time::Instant::now();
    let r = reverse_chart(&crit, &cfg).unwrap();
    assert!(r.candidates.is_empty() && !r.truncated);
    assert!(
        t.elapsed().as_millis() < 100,
        "should short-circuit, took {:?}",
        t.elapsed()
    );
}

#[test]
fn reverse_chart_daily_adjective_stars_roundtrip() {
    // 三台/八座/恩光/天贵（日系杂耀）走日层查表通道，须能与其它条件闭环
    let cfg = Config::default();
    let a = by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        Language::ZhCN,
        cfg.clone(),
    )
    .unwrap();
    let pos = |k: StarKey| a.star(k).unwrap().palace().earthly_branch;
    let crit = ReverseCriteria {
        five_elements_class: Some(a.five_elements_class),
        mutagens: [Some(StarKey::TaiyangMaj), None, None, None],
        stars: vec![
            StarPosition {
                star: StarKey::ZiweiMaj,
                branch: pos(StarKey::ZiweiMaj),
            },
            StarPosition {
                star: StarKey::ZuofuMin,
                branch: pos(StarKey::ZuofuMin),
            },
            StarPosition {
                star: StarKey::WenquMin,
                branch: pos(StarKey::WenquMin),
            },
            StarPosition {
                star: StarKey::Santai,
                branch: pos(StarKey::Santai),
            },
            StarPosition {
                star: StarKey::Bazuo,
                branch: pos(StarKey::Bazuo),
            },
            StarPosition {
                star: StarKey::Engguang,
                branch: pos(StarKey::Engguang),
            },
            StarPosition {
                star: StarKey::Tiangui,
                branch: pos(StarKey::Tiangui),
            },
        ],
        year_range: (1907, 2026),
        ..Default::default()
    };
    let r = reverse_chart(&crit, &cfg).unwrap();
    assert!(
        r.candidates
            .iter()
            .any(|c| c.solar_date == "2000-8-16" && c.time_index == 2),
        "{:?}",
        r.candidates
    );
}

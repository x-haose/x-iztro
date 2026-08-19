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
    // 反推按同一 config 仍须闭环。
    let cfg = Config {
        year_divide: YearDivide::Exact,
        horoscope_divide: HoroscopeDivide::Exact,
        ..Config::default()
    };
    for (date, t) in [("2001-2-1", 3), ("2000-8-16", 12), ("2004-3-21", 5)] {
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
        // 每个候选正排后必须满足全部条件
        for c in &r.candidates {
            let b = chart(&c.solar_date, c.time_index, &cfg);
            assert_eq!(
                b.earthly_branch_of_soul_palace,
                a.earthly_branch_of_soul_palace
            );
            assert_eq!(b.five_elements_class, a.five_elements_class);
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
    // 流耀
    let err = reverse_chart(
        &ReverseCriteria {
            stars: vec![StarPosition {
                star: StarKey::Liulu,
                branch: EarthlyBranch::Zi,
            }],
            ..Default::default()
        },
        &cfg,
    )
    .unwrap_err();
    assert_eq!(err.code(), "invalid_argument");
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

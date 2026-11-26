//! 流派口径开关的行为测试：每个开关在真实盘上按出处给出的规则核对，默认值的输出由各层金标守。
//!
//! 规则出处（页码为研读报告所引 PDF 页号）：晚子时跨月跨年——李铁笔《紫微斗数命运学》p105–106；
//! 闰月整月作下月——《紫微斗数讲义》p33–34、《紫微斗数精成》p11；月马——李 p20；
//! 长生男顺女逆——精成 p22、李 p29，一律顺行——讲义 p64；天使仆役、天伤疾厄——李 p105、p135；
//! 天巫巳申亥寅——精成 p20、p26；岁前将前只作流年星——《紫微斗数一本通》p44、李 p115；
//! 辛壬年两个来因宫——许铨仁《紫微斗数命理学正解（二）》p12。

use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::{Astrolabe, by_lunar, by_solar};

fn solar(date: &str, t: u8, gender: Gender, config: Config) -> Astrolabe {
    by_solar(date, t, gender, true, Language::ZhCN, config).unwrap()
}

fn lunar(date: &str, gender: Gender, config: Config) -> Astrolabe {
    by_lunar(date, 3, gender, LeapMonth::NotLeap, Language::ZhCN, config).unwrap()
}

fn branch_of(chart: &Astrolabe, star: StarKey) -> EarthlyBranch {
    chart.star(star).unwrap().palace().earthly_branch
}

/// 各宫的星耀标识（主星、辅星、杂耀按安放顺序）与四组十二神，用于比对两张盘的星耀布局。
fn layout(chart: &Astrolabe) -> Vec<(Vec<StarKey>, [Option<StarKey>; 4])> {
    chart
        .palaces
        .iter()
        .map(|p| {
            let stars = p
                .major_stars
                .iter()
                .chain(&p.minor_stars)
                .chain(&p.adjective_stars)
                .map(|s| s.key)
                .collect();
            let gods = [
                Some(p.changsheng12),
                Some(p.boshi12),
                p.suiqian12,
                p.jiangqian12,
            ];
            (stars, gods)
        })
        .collect()
}

fn carry() -> Config {
    Config {
        day_divide: DayDivide::ForwardCarry,
        ..Config::default()
    }
}

/// 晚子时整个日期归次日：除夕晚子进下一年。2024-2-9 是癸卯年除夕（腊月廿九），次日甲辰正月初一。
#[test]
fn forward_carry_moves_new_years_eve_into_next_year() {
    let forward = solar("2024-2-9", 12, Gender::Male, Config::default());
    assert_eq!(
        forward.raw_dates.chinese_date.yearly,
        (HeavenlyStem::Gui, EarthlyBranch::Mao)
    );

    let carried = solar("2024-2-9", 12, Gender::Male, carry());
    let next_day = solar("2024-2-10", 0, Gender::Male, Config::default());
    assert_eq!(
        carried.raw_dates.chinese_date.yearly,
        (HeavenlyStem::Jia, EarthlyBranch::Chen)
    );
    assert_eq!(carried.zodiac_key, "dragon");
    let d = carried.raw_dates.lunar_date;
    assert_eq!(
        (d.lunar_year, d.lunar_month, d.lunar_day, d.is_leap),
        (2024, 1, 1, false)
    );
    // 星耀布局与次日早子时完全相同；入参与时辰照原样回显
    assert_eq!(layout(&carried), layout(&next_day));
    assert_eq!(carried.solar_date, "2024-2-9");
    assert_eq!(carried.time_index, 12);
    assert_eq!(carried.sign_key, forward.sign_key);

    // 运限目标同样按次日早子时：流年干支取甲辰；运限的公历日期仍是入参当天
    let h = carried.horoscope("2024-2-9", 12).unwrap();
    assert_eq!(h.yearly.base.heavenly_stem, HeavenlyStem::Jia);
    assert_eq!(h.data().solar_date, "2024-2-9");
    assert_eq!(h.data().lunar_date, "二〇二四年正月初一");
    let h = forward.horoscope("2024-2-9", 12).unwrap();
    assert_eq!(h.yearly.base.heavenly_stem, HeavenlyStem::Gui);
}

/// 晚子时整个日期归次日：月底晚子进下月，左辅右弼等按生月安的星随之移一宫。
#[test]
fn forward_carry_moves_month_end_into_next_month() {
    // 2024 年里找一个非闰月月底、次日是下个月初一的日子
    let (date, next) = (1..=12)
        .flat_map(|m| (1..=28).map(move |d| (m, d)))
        .map(|(m, d)| (format!("2024-{m}-{d}"), format!("2024-{m}-{}", d + 1)))
        .find(|(date, next)| {
            let a = solar(date, 0, Gender::Male, Config::default())
                .raw_dates
                .lunar_date;
            let b = solar(next, 0, Gender::Male, Config::default())
                .raw_dates
                .lunar_date;
            b.lunar_day == 1 && !a.is_leap && !b.is_leap && a.lunar_month != 12
        })
        .expect("2024 年有月底在公历月内的农历月");

    let forward = solar(&date, 12, Gender::Female, Config::default());
    let carried = solar(&date, 12, Gender::Female, carry());
    let next_day = solar(&next, 0, Gender::Female, Config::default());
    let (f, c) = (forward.raw_dates.lunar_date, carried.raw_dates.lunar_date);
    assert_eq!(c.lunar_month, f.lunar_month + 1);
    assert_eq!(c.lunar_day, 1);
    assert_eq!(layout(&carried), layout(&next_day));
    // 默认口径只回卷起紫微的日数，左辅仍按本月；进位后按下月，左辅顺移一宫
    assert_eq!(
        branch_of(&carried, StarKey::ZuofuMin).index(),
        (branch_of(&forward, StarKey::ZuofuMin).index() + 1) % 12
    );
}

/// 闰月整月作下月：2020-5-25 是闰四月初三，默认按四月、整月作下月时按五月；`fix_leap` 不再起作用。
#[test]
fn leap_next_month_treats_whole_leap_month_as_next() {
    let next_month = Config {
        leap_divide: LeapDivide::NextMonth,
        ..Config::default()
    };
    let default = solar("2020-5-25", 3, Gender::Male, Config::default());
    let d = default.raw_dates.lunar_date;
    assert_eq!((d.lunar_month, d.lunar_day, d.is_leap), (4, 3, true));
    // 左辅：辰宫起正月顺数到生月
    assert_eq!(branch_of(&default, StarKey::ZuofuMin), EarthlyBranch::Wei);
    for fix_leap in [true, false] {
        let chart = by_solar(
            "2020-5-25",
            3,
            Gender::Male,
            fix_leap,
            Language::ZhCN,
            next_month.clone(),
        )
        .unwrap();
        assert_eq!(branch_of(&chart, StarKey::ZuofuMin), EarthlyBranch::Shen);
        assert_eq!(branch_of(&chart, StarKey::YoubiMin), EarthlyBranch::Wu);
    }

    // 斗君的生月同按五月：流月命宫比默认口径逆退一宫
    let chart = solar("2020-5-25", 3, Gender::Male, next_month);
    let a = default.horoscope("2024-8-1", 3).unwrap();
    let b = chart.horoscope("2024-8-1", 3).unwrap();
    assert_eq!(b.monthly.index, (a.monthly.index + 11) % 12);
}

/// 天马三种安法：年马按年支、月马按生月（正五九申、二六十巳、三七十一寅、四八十二亥）。
#[test]
fn tianma_placements_follow_year_or_month() {
    use EarthlyBranch::*;
    let month_horse = [Shen, Si, Yin, Hai];
    let monthly = Config {
        tianma_placement: TianmaPlacement::Monthly,
        ..Config::default()
    };
    let both = Config {
        tianma_placement: TianmaPlacement::Both,
        ..Config::default()
    };
    // 2001 辛巳年：巳酉丑年马在亥
    for m in 1..=12usize {
        let date = format!("2001-{m}-10");
        let yearly = lunar(&date, Gender::Male, Config::default());
        assert_eq!(branch_of(&yearly, StarKey::TianmaMin), Hai);
        assert!(yearly.star(StarKey::YuemaAdj).is_none());

        let chart = lunar(&date, Gender::Male, monthly.clone());
        assert_eq!(
            branch_of(&chart, StarKey::TianmaMin),
            month_horse[(m - 1) % 4]
        );
        assert!(chart.star(StarKey::YuemaAdj).is_none());

        let chart = lunar(&date, Gender::Male, both.clone());
        assert_eq!(branch_of(&chart, StarKey::TianmaMin), Hai);
        let yuema = chart.star(StarKey::YuemaAdj).unwrap();
        assert_eq!(yuema.palace().earthly_branch, month_horse[(m - 1) % 4]);
        assert_eq!(yuema.star_type, StarType::Tianma);
    }
}

/// 长生十二神方向：默认阳男阴女顺，`Gender` 男顺女逆，`Clockwise` 一律顺行。
#[test]
fn changsheng_directions() {
    // 顺行即沐浴在长生的下一宫（宫位索引加一）
    let clockwise = |chart: &Astrolabe| {
        let at = |k: StarKey| {
            chart
                .palaces
                .iter()
                .position(|p| p.changsheng12 == k)
                .unwrap()
        };
        (at(StarKey::Changsheng) + 1) % 12 == at(StarKey::Muyu)
    };
    let with = |direction| Config {
        changsheng_direction: direction,
        ..Config::default()
    };
    // 2001 辛巳为阴年、2000 庚辰为阳年
    for (date, gender, yin_yang, by_gender) in [
        ("2001-5-1", Gender::Female, true, false),
        ("2001-5-1", Gender::Male, false, true),
        ("2000-5-1", Gender::Female, false, false),
        ("2000-5-1", Gender::Male, true, true),
    ] {
        assert_eq!(clockwise(&lunar(date, gender, Config::default())), yin_yang);
        assert_eq!(
            clockwise(&lunar(date, gender, with(ChangshengDirection::Gender))),
            by_gender
        );
        assert!(clockwise(&lunar(
            date,
            gender,
            with(ChangshengDirection::Clockwise)
        )));
    }
}

/// `tianshiFriends`：天使在仆役、天伤在疾厄，不分男女与派别。
#[test]
fn tianshi_in_friends_palace() {
    for algorithm in [Algorithm::Default, Algorithm::Zhongzhou] {
        for gender in [Gender::Male, Gender::Female] {
            let chart = lunar(
                "2001-5-1",
                gender,
                Config {
                    algorithm,
                    tianshang_tianshi: TianshangTianshi::TianshiFriends,
                    ..Config::default()
                },
            );
            let palace_of = |k: StarKey| chart.star(k).unwrap().palace().name;
            assert_eq!(palace_of(StarKey::Tianshi), Palace::Friends);
            assert_eq!(palace_of(StarKey::Tianshang), Palace::Health);
        }
    }
}

/// 天巫巳申亥寅：三、七、十一月在亥，四、八、十二月在寅；默认三月寅、四月亥。
#[test]
fn tianwu_si_shen_hai_yin() {
    use EarthlyBranch::*;
    let variant = Config {
        tianwu_placement: TianwuPlacement::SiShenHaiYin,
        ..Config::default()
    };
    for (m, default, jingcheng) in [
        (1, Si, Si),
        (2, Shen, Shen),
        (3, Yin, Hai),
        (4, Hai, Yin),
        (11, Yin, Hai),
        (12, Hai, Yin),
    ] {
        let date = format!("2001-{m}-10");
        assert_eq!(
            branch_of(
                &lunar(&date, Gender::Male, Config::default()),
                StarKey::Tianwu
            ),
            default
        );
        assert_eq!(
            branch_of(
                &lunar(&date, Gender::Male, variant.clone()),
                StarKey::Tianwu
            ),
            jingcheng
        );
    }
}

/// 岁前、将前只在流年安：本命各宫不安，DTO 两组写空串；其余星耀与流年层不变。
#[test]
fn yearly12_only_on_yearly_layer() {
    let yearly_only = Config {
        yearly12_placement: Yearly12Placement::YearlyOnly,
        ..Config::default()
    };
    let default = solar("2000-8-16", 2, Gender::Female, Config::default());
    let chart = solar("2000-8-16", 2, Gender::Female, yearly_only);
    for (p, q) in chart.palaces.iter().zip(&default.palaces) {
        assert_eq!((p.suiqian12, p.jiangqian12), (None, None));
        assert!(q.suiqian12.is_some() && q.jiangqian12.is_some());
    }
    let strip =
        |c: &Astrolabe| -> Vec<Vec<StarKey>> { layout(c).into_iter().map(|(s, _)| s).collect() };
    assert_eq!(strip(&chart), strip(&default));
    let dto = serde_json::to_value(chart.to_dto()).unwrap();
    for p in dto["palaces"].as_array().unwrap() {
        for key in ["suiqian12", "suiqian12Key", "jiangqian12", "jiangqian12Key"] {
            assert_eq!(p[key], "", "{key}");
        }
    }
    assert_eq!(dto["config"]["yearly12Placement"], "yearlyOnly");
    let text = x_iztro::astrolabe_to_text(&chart, Language::ZhCN);
    assert!(text.contains("长生·") && !text.contains("岁前·") && !text.contains("将前·"));
    let a = chart.horoscope("2024-8-1", 3).unwrap();
    let b = default.horoscope("2024-8-1", 3).unwrap();
    assert_eq!(
        a.yearly.yearly_dec_star.suiqian12,
        b.yearly.yearly_dec_star.suiqian12
    );
    assert_eq!(
        a.yearly.yearly_dec_star.jiangqian12,
        b.yearly.yearly_dec_star.jiangqian12
    );
}

/// 辛年卯、丑，壬年寅、子：寅卯为来因宫，子丑标辅来因宫；其余年份没有辅来因宫。
#[test]
fn secondary_original_palace_in_xin_and_ren_years() {
    use EarthlyBranch::*;
    let with_secondary = Config {
        original_palace: OriginalPalaceMarking::WithSecondary,
        ..Config::default()
    };
    let marks = |chart: &Astrolabe| {
        let find = |pick: fn(&x_iztro::models::palace::PalaceData) -> bool| {
            chart
                .palaces
                .iter()
                .filter(|p| pick(p))
                .map(|p| p.earthly_branch)
                .collect::<Vec<_>>()
        };
        (
            find(|p| p.is_original_palace),
            find(|p| p.is_secondary_original_palace),
        )
    };
    for (date, original, secondary) in [
        ("2001-5-1", Mao, Some(Chou)),
        ("2002-5-1", Yin, Some(Zi)),
        ("2000-5-1", Chen, None),
    ] {
        let default = lunar(date, Gender::Male, Config::default());
        assert_eq!(marks(&default), (vec![original], vec![]));
        let chart = lunar(date, Gender::Male, with_secondary.clone());
        assert_eq!(
            marks(&chart),
            (vec![original], secondary.into_iter().collect())
        );
        // 宫位寻址的 originalPalace 仍指寅卯那一宫
        assert_eq!(
            chart
                .palace(x_iztro::PalaceTarget::Original)
                .unwrap()
                .earthly_branch,
            original
        );
        let text = x_iztro::astrolabe_to_text(&chart, Language::ZhCN);
        assert_eq!(text.contains("辅来因宫"), secondary.is_some());
        let dto = serde_json::to_value(chart.to_dto()).unwrap();
        let flagged = dto["palaces"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|p| p.get("isSecondaryOriginalPalace").is_some())
            .count();
        assert_eq!(flagged, usize::from(secondary.is_some()));
    }
}

/// 斗君生月随本命：闰四月十六晚子时（2020-6-7）本命按四月，iztro 的斗君按五月；`natal` 时斗君也按四月。
#[test]
fn dou_jun_month_follows_natal_rule() {
    let default = solar("2020-6-7", 12, Gender::Male, Config::default());
    let d = default.raw_dates.lunar_date;
    assert_eq!((d.lunar_month, d.lunar_day, d.is_leap), (4, 16, true));
    // 本命盘：晚子时不把闰月下半月作下月，左辅在四月之位
    assert_eq!(branch_of(&default, StarKey::ZuofuMin), EarthlyBranch::Wei);
    let natal = solar(
        "2020-6-7",
        12,
        Gender::Male,
        Config {
            dou_jun_month: DouJunMonth::Natal,
            ..Config::default()
        },
    );
    assert_eq!(layout(&natal), layout(&default));
    let a = default.horoscope("2024-8-1", 3).unwrap();
    let b = natal.horoscope("2024-8-1", 3).unwrap();
    // 斗君 = 流年宫逆数生月：生月早一个月，流月命宫顺进一宫
    assert_eq!(b.monthly.index, (a.monthly.index + 1) % 12);

    // 非晚子时两种取法相同
    let early = solar("2020-6-7", 3, Gender::Male, Config::default());
    let early_natal = solar(
        "2020-6-7",
        3,
        Gender::Male,
        Config {
            dou_jun_month: DouJunMonth::Natal,
            ..Config::default()
        },
    );
    assert_eq!(
        early.horoscope("2024-8-1", 3).unwrap().monthly.index,
        early_natal.horoscope("2024-8-1", 3).unwrap().monthly.index
    );
}

/// 新开关随 config 的线协议往返；DTO 的 config 只写非默认的开关。
#[test]
fn new_switches_roundtrip_and_echo_only_when_set() {
    let config: Config = serde_json::from_value(serde_json::json!({
        "dayDivide": "forwardCarry",
        "leapDivide": "nextMonth",
        "tianmaPlacement": "both",
        "changshengDirection": "gender",
        "tianshangTianshi": "tianshiFriends",
        "tianwuPlacement": "siShenHaiYin",
        "yearly12Placement": "yearlyOnly",
        "originalPalace": "withSecondary",
        "douJunMonth": "natal",
    }))
    .unwrap();
    let back: Config = serde_json::from_value(serde_json::to_value(&config).unwrap()).unwrap();
    assert_eq!(back, config);
    let dto = serde_json::to_value(solar("2001-5-1", 2, Gender::Male, config).to_dto()).unwrap();
    assert_eq!(dto["config"]["leapDivide"], "nextMonth");
    assert_eq!(dto["config"]["douJunMonth"], "natal");
    assert_eq!(dto["config"]["dayDivide"], "forwardCarry");

    let dto = serde_json::to_value(solar("2001-5-1", 2, Gender::Male, Config::default()).to_dto())
        .unwrap();
    let keys: Vec<&String> = dto["config"].as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        [
            "ageDivide",
            "algorithm",
            "astroType",
            "dayDivide",
            "horoscopeDivide",
            "yearDivide"
        ]
    );

    let err = serde_json::from_str::<Config>(r#"{"tianmaPlacement":"daily"}"#).unwrap_err();
    assert!(
        err.to_string()
            .starts_with("invalid tianmaPlacement 'daily': expected 'yearly', 'monthly' or 'both'"),
        "{err}"
    );
}

/// 晚子时进位跨过改历空缺与支持范围末日：1582-10-4 的次日是 1582-10-15，9999-12-31 的次日 10000-1-1
/// 仍在农历 9999 年内，两处都照常排盘。
#[test]
fn forward_carry_across_reform_gap_and_last_day() {
    let carried = solar("1582-10-4", 12, Gender::Male, carry());
    let next_day = solar("1582-10-15", 0, Gender::Male, Config::default());
    assert_eq!(layout(&carried), layout(&next_day));
    let last = solar("9999-12-31", 12, Gender::Male, carry());
    assert_eq!(last.raw_dates.lunar_date.lunar_year, 9999);
    last.horoscope("9999-12-31", 12).unwrap();
}

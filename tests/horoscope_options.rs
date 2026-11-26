//! 运限流派口径开关的行为测试：每个开关在真实盘上按出处给出的规则或命例核对，默认值的输出由各层金标守。
//!
//! 规则出处（页码为 PDF 页号；蔡明宏《秘仪》两页并排扫描，写作「书页 X / PDF pY」）：流月按节气换干、年仍按初一——令东来《紫微斗数命理学》p160–161；
//! 大限天干从命宫干连续排——《紫微斗数精成》p398（蒋介石命例）；童限以夫妻宫为太极——依婷系统班 02 讲 p14–17；
//! 第二大限前都以夫妻宫立太极——周星飞《梁若瑜飞星派紫微斗数课程》p19；
//! 小限不起四化——精成 p433；流年命宫取小限——《天纪紫微斗数笔记》p13、p35、p59；
//! 运限四化取该层命宫的宫干——《课程》p21（2009 己丑年用丑宫辛干）、
//! 方外人《河洛紫微斗数》p28、蔡明宏《华山钦天四化紫微斗数飞星秘仪》书页 50 / PDF p37（甲法）；
//! 流月正月固定在寅宫——蔡明宏书页 51–52 / PDF p37–38（乙法）；流日初一落在流月命宫的下一宫——《课程》p264；
//! 闰月流日接本月连续数——王亭之《安星法及推断实例》p85；流时按时支落宫——许铨仁《学习紫微斗数的第一本书》p52–53。

use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::{
    Astrolabe, HoroscopeData, HoroscopeItem, KnowledgePack, by_lunar, by_solar, lunar_months,
    lunar_to_solar,
};

fn solar(date: &str, t: u8, gender: Gender, config: Config) -> Astrolabe {
    by_solar(date, t, gender, true, Language::ZhCN, config).unwrap()
}

fn lunar(date: &str, t: u8, gender: Gender, config: Config) -> Astrolabe {
    by_lunar(date, t, gender, LeapMonth::NotLeap, Language::ZhCN, config).unwrap()
}

/// 盘上某地支所在宫的索引（寅宫为 0）
fn at(branch: EarthlyBranch) -> usize {
    (branch.index() + 10) % 12
}

fn natal_name(chart: &Astrolabe, index: usize) -> Palace {
    chart.palaces[index].name
}

/// 农历日期（`leap` 为闰月）对应的公历日期串
fn solar_of(year: i64, month: u32, day: u32, leap: bool) -> String {
    let d = lunar_to_solar(year, month, day, leap).unwrap();
    format!("{}-{}-{}", d.solar_year, d.solar_month, d.solar_day)
}

/// 运限某一层
fn layer(h: &HoroscopeData, scope: Scope) -> &HoroscopeItem {
    match scope {
        Scope::Decadal => &h.decadal,
        Scope::Yearly => &h.yearly.base,
        Scope::Monthly => &h.monthly,
        Scope::Daily => &h.daily,
        Scope::Hourly => &h.hourly,
        other => panic!("{other:?} 不是运限层"),
    }
}

/// 年按正月初一、月按节气：2025-1-31 是乙巳年正月初三，立春（2-3）未到。初一分界的流月是乙巳年正月戊寅；
/// 月柱按节气换时，流月一览在交节处拆行：各行首尾相接覆盖全年，行内每天的流月干支都等于该行的干支，
/// 每个农历月至多拆成三行。2015 年二月十七清明（令东来例），二月拆成己卯、庚辰两行。
#[test]
fn monthly_list_splits_at_jie() {
    let chart = solar(
        "1990-5-12",
        6,
        Gender::Male,
        Config {
            horoscope_divide: HoroscopeDivide::ExactMonth,
            ..Config::default()
        },
    );
    let list = chart.monthly_list(2015, true).unwrap();
    assert!(list.len() > 12, "2015 年各月几乎都交节");
    let mut prev: Option<(u32, bool, u32)> = None;
    for m in &list {
        match prev {
            Some((month, leap, end)) if month == m.month && leap == m.is_leap_month => {
                assert_eq!(m.day_range.0, end + 1, "同月各行首尾相接")
            }
            _ => assert_eq!(m.day_range.0, 1, "新的一月从初一起"),
        }
        prev = Some((m.month, m.is_leap_month, m.day_range.1));
        for day in m.day_range.0..=m.day_range.1 {
            let h = chart
                .horoscope(&solar_of(2015, m.month, day, m.is_leap_month), 6)
                .unwrap();
            assert_eq!(
                (
                    h.monthly.heavenly_stem,
                    h.monthly.earthly_branch,
                    h.monthly.index
                ),
                (m.heavenly_stem, m.earthly_branch, m.index),
                "{}月{day}日",
                m.month
            );
        }
    }
    let feb: Vec<_> = list.iter().filter(|m| m.month == 2).collect();
    assert_eq!(feb.len(), 2);
    assert_eq!(feb[0].day_range.1, 16);
    assert_eq!(
        (feb[0].heavenly_stem, feb[1].heavenly_stem),
        (HeavenlyStem::Ji, HeavenlyStem::Geng)
    );
    for month in 1..=12 {
        assert!(list.iter().filter(|m| m.month == month).count() <= 3);
    }
    // 初一分界不拆
    let normal = solar("1990-5-12", 6, Gender::Male, Config::default());
    assert_eq!(normal.monthly_list(2015, true).unwrap().len(), 12);
}

/// 节气分界连流年也还在甲辰，流月是甲辰年丑月丁丑；exactMonth 流年乙巳、流月丁丑，流月命宫与初一分界相同。
#[test]
fn exact_month_splits_month_stem_from_year() {
    let config = |horoscope_divide| Config {
        horoscope_divide,
        ..Config::default()
    };
    let run = |divide| {
        let chart = solar("1990-5-12", 6, Gender::Male, config(divide));
        let h = chart.horoscope("2025-1-31", 6).unwrap().into_data();
        (
            (h.yearly.heavenly_stem, h.yearly.earthly_branch),
            (h.monthly.heavenly_stem, h.monthly.earthly_branch),
            h.yearly.index,
            h.monthly.index,
        )
    };
    let normal = run(HoroscopeDivide::Normal);
    let exact = run(HoroscopeDivide::Exact);
    let split = run(HoroscopeDivide::ExactMonth);
    assert_eq!(normal.0, (HeavenlyStem::Yi, EarthlyBranch::Si));
    assert_eq!(normal.1, (HeavenlyStem::Wu, EarthlyBranch::Yin));
    assert_eq!(exact.0, (HeavenlyStem::Jia, EarthlyBranch::Chen));
    assert_eq!(exact.1, (HeavenlyStem::Ding, EarthlyBranch::Chou));
    assert_eq!(split.0, normal.0);
    assert_eq!(split.1, exact.1);
    assert_eq!((split.2, split.3), (normal.2, normal.3));

    // 令东来的例子：2015-4-5 清明，农历二月十七，流月仍在二月，四化已用庚干（庚辰月）
    let chart = solar(
        "1990-5-12",
        6,
        Gender::Male,
        config(HoroscopeDivide::ExactMonth),
    );
    let normal_chart = solar("1990-5-12", 6, Gender::Male, Config::default());
    let h = chart.horoscope("2015-4-5", 6).unwrap();
    assert_eq!(h.monthly.heavenly_stem, HeavenlyStem::Geng);
    assert_eq!(
        h.monthly.index,
        normal_chart.horoscope("2015-4-5", 6).unwrap().monthly.index
    );
    assert_eq!(
        h.monthly.mutagen,
        [
            StarKey::TaiyangMaj,
            StarKey::WuquMaj,
            StarKey::TaiyinMaj,
            StarKey::TiantongMaj
        ]
    );
    // 本命四柱的月柱同样按节气，年柱不变
    assert_eq!(
        chart.raw_dates.chinese_date.monthly,
        solar("1990-5-12", 6, Gender::Male, config(HoroscopeDivide::Exact))
            .raw_dates
            .chinese_date
            .monthly
    );
}

/// 精成 p398 的蒋介石命盘（光绪十三年九月十五午时，丁亥阴男，大限逆行）：命宫甲辰，逆上卯宫癸、寅宫壬，
/// 再逆上丑宫按连续排为辛（宫干为癸）、子宫为庚（宫干为壬）。
#[test]
fn continuous_decadal_stems_follow_jingcheng_example() {
    let config = Config {
        decadal_stem: DecadalStem::Continuous,
        ..Config::default()
    };
    let chart = lunar("1887-9-15", 6, Gender::Male, config.clone());
    let default = lunar("1887-9-15", 6, Gender::Male, Config::default());
    let soul = chart.palace(Palace::Soul).unwrap();
    assert_eq!(
        (soul.heavenly_stem, soul.earthly_branch),
        (HeavenlyStem::Jia, EarthlyBranch::Chen)
    );
    let stem_at = |c: &Astrolabe, b| c.palaces[at(b)].decadal.heavenly_stem;
    for (branch, want, palace_stem) in [
        (EarthlyBranch::Chen, HeavenlyStem::Jia, HeavenlyStem::Jia),
        (EarthlyBranch::Mao, HeavenlyStem::Gui, HeavenlyStem::Gui),
        (EarthlyBranch::Yin, HeavenlyStem::Ren, HeavenlyStem::Ren),
        (EarthlyBranch::Chou, HeavenlyStem::Xin, HeavenlyStem::Gui),
        (EarthlyBranch::Zi, HeavenlyStem::Geng, HeavenlyStem::Ren),
    ] {
        assert_eq!(stem_at(&chart, branch), want, "{branch:?}");
        assert_eq!(stem_at(&default, branch), palace_stem, "{branch:?}");
    }
    // 第四大限（丑宫）的运限四化与运曜按大限干辛
    let start = chart.palaces[at(EarthlyBranch::Chou)].decadal.range.0;
    let h = chart
        .horoscope(&format!("{}-6-1", 1887 + start - 1), 0)
        .unwrap();
    assert_eq!(h.decadal.index, at(EarthlyBranch::Chou));
    assert_eq!(h.decadal.heavenly_stem, HeavenlyStem::Xin);
    assert_eq!(h.decadal.mutagen_stem, Some(HeavenlyStem::Xin));
    assert_eq!(h.decadal.mutagen[0], StarKey::JumenMaj);
    let list = chart.decadal_list();
    assert_eq!(list[3].heavenly_stem, HeavenlyStem::Xin);
    assert_eq!(list[3].mutagen, h.decadal.mutagen);
}

/// 依婷 02 讲：起运前的童限以夫妻宫为太极、用夫妻宫干起四化。土五局五岁起运，虚岁一至四岁都在夫妻宫。
#[test]
fn childhood_on_spouse_palace() {
    let spouse = Config {
        childhood_palace: ChildhoodPalace::Spouse,
        ..Config::default()
    };
    let chart = solar("1990-5-12", 6, Gender::Male, spouse);
    assert_eq!(chart.five_elements_class, FiveElementsClass::Earth5th);
    let spouse_index = chart.palace(Palace::Spouse).unwrap().index;
    let spouse_stem = chart.palaces[spouse_index].heavenly_stem;
    for age in 1..5 {
        let h = chart.horoscope_of_year(1990 + age - 1).unwrap();
        assert_eq!(h.decadal.name_key, HoroscopeName::Childhood);
        assert_eq!(h.decadal.index, spouse_index, "{age}");
        assert_eq!(h.decadal.palace_names[spouse_index], Palace::Soul);
        assert_eq!(h.decadal.heavenly_stem, spouse_stem);
        assert_eq!(h.decadal.mutagen_stem, Some(spouse_stem));
    }
    // 默认童限按命、财、疾、夫依次落宫：虚岁一岁在命宫
    let default = solar("1990-5-12", 6, Gender::Male, Config::default());
    let first = default.horoscope_of_year(1990).unwrap();
    assert_eq!(natal_name(&default, first.decadal.index), Palace::Soul);
    // 起运后照常按大限
    let h = chart.horoscope_of_year(1994).unwrap();
    assert_eq!(h.decadal.name_key, HoroscopeName::Decadal);
    assert_eq!(natal_name(&chart, h.decadal.index), Palace::Soul);
}

/// 精成 p433「小限宫干不飞四化」：小限层四化为空，DTO 以空串标出不起四化，文本不写小限的四化行。
#[test]
fn age_layer_without_mutagens() {
    let chart = solar(
        "1990-5-12",
        6,
        Gender::Male,
        Config {
            age_mutagens: AgeMutagens::None,
            ..Config::default()
        },
    );
    let h = chart.horoscope("2024-8-1", 3).unwrap();
    assert_eq!(h.age.mutagen_stem, None);
    assert!(h.age.mutagen.is_empty() && h.age.mutagens().next().is_none());
    let dto = serde_json::to_value(h.to_dto(Language::ZhCN)).unwrap();
    assert_eq!(dto["age"]["mutagen"], serde_json::json!([]));
    assert_eq!(dto["age"]["mutagenStemKey"], "");
    assert!(dto["yearly"].get("mutagenStemKey").is_none());

    let text = h.to_text();
    let age_section = text.split("\n## ").find(|s| s.starts_with("小限")).unwrap();
    assert!(!age_section.contains("- 四化"), "{age_section}");
    let default = solar("1990-5-12", 6, Gender::Male, Config::default());
    let default_text = default.horoscope("2024-8-1", 3).unwrap().to_text();
    assert!(
        default_text
            .split("\n## ")
            .any(|s| s.starts_with("小限") && s.contains("- 四化"))
    );
}

/// 天纪以小限为流年：例三民国 41 年壬辰生女、命宫在申，56 岁小限从戌逆行落卯宫，卯宫是她的疾厄；
/// 例二民国 37 年戊子生男，44 岁小限从戌顺行落巳宫。流年干支、流耀与岁前将前仍按流年年支，斗君仍从太岁宫起。
#[test]
fn yearly_palace_on_age_palace_matches_tianji_examples() {
    let age_mode = Config {
        yearly_palace: YearlyPalace::Age,
        ..Config::default()
    };
    // 农历七月子时：命宫在申
    let woman = lunar("1952-7-10", 0, Gender::Female, age_mode.clone());
    assert_eq!(
        woman.palace(Palace::Soul).unwrap().earthly_branch,
        EarthlyBranch::Shen
    );
    let h = woman.horoscope_of_year(2007).unwrap();
    assert_eq!(h.age.nominal_age, 56);
    assert_eq!(h.yearly.index, at(EarthlyBranch::Mao));
    assert_eq!(natal_name(&woman, h.yearly.index), Palace::Health);
    assert_eq!(h.yearly.palace_names[at(EarthlyBranch::Mao)], Palace::Soul);
    assert_eq!(
        (h.yearly.heavenly_stem, h.yearly.earthly_branch),
        (HeavenlyStem::Ding, EarthlyBranch::Hai)
    );

    let default = lunar("1952-7-10", 0, Gender::Female, Config::default());
    let d = default.horoscope_of_year(2007).unwrap();
    assert_eq!(d.yearly.index, at(EarthlyBranch::Hai));
    let flow_keys = |item: &HoroscopeItem| -> Vec<Vec<StarKey>> {
        item.stars
            .iter()
            .flatten()
            .map(|g| g.iter().map(|s| s.key).collect())
            .collect()
    };
    assert_eq!(flow_keys(&h.yearly), flow_keys(&d.yearly));
    assert_eq!(
        h.yearly.yearly_dec_star.suiqian12,
        d.yearly.yearly_dec_star.suiqian12
    );
    assert_eq!(h.yearly.mutagen, d.yearly.mutagen);
    let june = woman.horoscope_of_month(2007, 6).unwrap();
    assert_eq!(
        june.monthly.index,
        default.horoscope_of_month(2007, 6).unwrap().monthly.index
    );

    let man = lunar("1948-3-10", 0, Gender::Male, age_mode);
    let h = man.horoscope_of_year(1991).unwrap();
    assert_eq!(h.age.nominal_age, 44);
    assert_eq!(h.yearly.index, at(EarthlyBranch::Si));
}

/// 《课程》p21：2009 己丑年的流年命宫在丑，丑宫宫干是辛，流年四化用辛（巨门禄、太阳权、文曲科、文昌忌）。
/// 丙、辛年生人的丑宫宫干为辛。
fn feixing_chart(config: Config) -> Astrolabe {
    solar("1986-3-20", 4, Gender::Female, config)
}

#[test]
fn scope_soul_palace_stems_follow_feixing_example() {
    let config = Config {
        scope_mutagen_stem: ScopeMutagenStem::ScopeSoulPalace,
        ..Config::default()
    };
    let chart = feixing_chart(config);
    assert_eq!(
        chart.palaces[at(EarthlyBranch::Chou)].heavenly_stem,
        HeavenlyStem::Xin
    );
    let h = chart.horoscope("2009-8-1", 3).unwrap();
    assert_eq!(h.yearly.index, at(EarthlyBranch::Chou));
    assert_eq!(h.yearly.heavenly_stem, HeavenlyStem::Ji);
    assert_eq!(h.yearly.mutagen_stem, Some(HeavenlyStem::Xin));
    assert_eq!(
        h.yearly.mutagen,
        [
            StarKey::JumenMaj,
            StarKey::TaiyangMaj,
            StarKey::WenquMin,
            StarKey::WenchangMin
        ]
    );
    // 其余各层同样取该层命宫的宫干
    for item in [&h.decadal, &h.monthly, &h.daily, &h.hourly] {
        assert_eq!(
            item.mutagen_stem,
            Some(chart.palaces[item.index].heavenly_stem)
        );
    }
    // DTO 只在四化干与层干不同时写出
    let dto = serde_json::to_value(h.to_dto(Language::ZhCN)).unwrap();
    assert_eq!(dto["yearly"]["mutagenStemKey"], "xinHeavenly");
    assert_eq!(dto["yearly"]["mutagenStem"], "辛");
    assert!(dto["decadal"].get("mutagenStemKey").is_none());
    // 文本的四化行注明所用天干
    let text = h.to_text();
    assert!(text.contains("- 四化 (辛干): 巨门化禄"), "{text}");
    // 主题选宫标明取法，逐条的天干即该层命宫的宫干
    let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
    let selection = h.topic_palaces(pack, "career", Scope::Yearly).unwrap();
    assert_eq!(
        selection.scope_mutagen_stem_source.as_key(),
        "scopeSoulPalace"
    );
    for m in &selection.scope_mutagens {
        let item = layer(h.data(), m.scope);
        assert_eq!(m.stem, chart.palaces[item.index].heavenly_stem);
    }
    // 运限格局按这一组四化判定：化禄星在流年视角读出的四化就是禄
    let default = feixing_chart(Config::default());
    let d = default.horoscope("2009-8-1", 3).unwrap();
    assert_eq!(d.yearly.mutagen_stem, Some(HeavenlyStem::Ji));
    assert_ne!(h.yearly.mutagen, d.yearly.mutagen);
    h.patterns(Scope::Yearly).unwrap();
}

/// 蔡明宏乙法：流月不用斗君，正月在寅宫按五虎遁起月干。乙丑年（1985）正月戊寅在寅宫、二月己卯在卯宫。
#[test]
fn month_branch_palace_starts_first_month_at_yin() {
    let chart = solar(
        "1960-2-10",
        8,
        Gender::Male,
        Config {
            monthly_palace: MonthlyPalace::MonthBranch,
            ..Config::default()
        },
    );
    for (month, branch, stem) in [
        (1, EarthlyBranch::Yin, HeavenlyStem::Wu),
        (2, EarthlyBranch::Mao, HeavenlyStem::Ji),
        (12, EarthlyBranch::Chou, HeavenlyStem::Ji),
    ] {
        let h = chart.horoscope_of_month(1985, month).unwrap();
        assert_eq!(h.monthly.index, at(branch), "{month}");
        assert_eq!(h.monthly.heavenly_stem, stem, "{month}");
    }
}

/// 《课程》p263–264 的命例（戊午年十月戌时生，命宫在丑）：2010 年八月流月在戌宫（子女），
/// 流日初一落在流月宫的下一宫，八月初四踏寅宫（父母）、初五踏卯宫（福德）。
#[test]
fn daily_palace_after_monthly_matches_feixing_example() {
    let config = Config {
        daily_palace: DailyPalace::AfterMonthly,
        ..Config::default()
    };
    let chart = lunar("1978-10-6", 10, Gender::Male, config);
    assert_eq!(
        chart.palace(Palace::Soul).unwrap().earthly_branch,
        EarthlyBranch::Chou
    );
    let day = |d: u32| chart.horoscope(&solar_of(2010, 8, d, false), 0).unwrap();
    let fourth = day(4);
    assert_eq!(fourth.monthly.index, at(EarthlyBranch::Xu));
    assert_eq!(natal_name(&chart, fourth.monthly.index), Palace::Children);
    assert_eq!(fourth.daily.index, at(EarthlyBranch::Yin));
    assert_eq!(natal_name(&chart, fourth.daily.index), Palace::Parents);
    let fifth = day(5);
    assert_eq!(fifth.daily.index, at(EarthlyBranch::Mao));
    assert_eq!(natal_name(&chart, fifth.daily.index), Palace::Spirit);
    // 默认初一在流月宫：初五在寅宫
    let default = lunar("1978-10-6", 10, Gender::Male, Config::default());
    assert_eq!(
        default
            .horoscope(&solar_of(2010, 8, 5, false), 0)
            .unwrap()
            .daily
            .index,
        at(EarthlyBranch::Yin)
    );
}

/// 王安 p85：三月有闰月时，三月顺数完本月日数后接闰月前半月连续顺推。1993 年闰三月：闰三月初一的流日
/// 落在三月流月宫后第「三月日数」位，闰三月十五同理；十六日起与默认相同。
#[test]
fn leap_month_days_continue_from_regular_month() {
    let continuous = Config {
        leap_daily: LeapDaily::Continuous,
        ..Config::default()
    };
    let chart = solar("1970-6-6", 4, Gender::Female, continuous);
    let default = solar("1970-6-6", 4, Gender::Female, Config::default());
    // 1993 年三月（非闰）的天数
    let regular_days = lunar_months(1993)
        .unwrap()
        .iter()
        .find(|m| m.month == 3 && !m.is_leap)
        .unwrap()
        .day_count as usize;
    let last = chart
        .horoscope(&solar_of(1993, 3, regular_days as u32, false), 0)
        .unwrap();
    let month_palace = last.monthly.index;
    assert_eq!(last.daily.index, (month_palace + regular_days - 1) % 12);
    for day in [1usize, 15] {
        let date = solar_of(1993, 3, day as u32, true);
        let h = chart.horoscope(&date, 0).unwrap();
        assert_eq!(h.monthly.index, month_palace);
        assert_eq!(
            h.daily.index,
            (month_palace + regular_days + day - 1) % 12,
            "{day}"
        );
        let d = default.horoscope(&date, 0).unwrap();
        assert_eq!(d.daily.index, (month_palace + day - 1) % 12, "{day}");
    }
    let date = solar_of(1993, 3, 16, true);
    assert_eq!(
        chart.horoscope(&date, 0).unwrap().daily.index,
        default.horoscope(&date, 0).unwrap().daily.index
    );
}

/// 许铨仁《第一本书》p52–53、王亭之师传（《紫微斗数讲义》p79）：流时按时支落宫，子时在子宫、丑时在丑宫。
#[test]
fn hourly_palace_by_branch() {
    let chart = solar(
        "1990-5-12",
        6,
        Gender::Male,
        Config {
            hourly_palace: HourlyPalace::Branch,
            ..Config::default()
        },
    );
    for (time_index, branch) in [
        (0, EarthlyBranch::Zi),
        (1, EarthlyBranch::Chou),
        (6, EarthlyBranch::Wu),
        (12, EarthlyBranch::Zi),
    ] {
        let h = chart.horoscope("2024-8-1", time_index).unwrap();
        assert_eq!(h.hourly.index, at(branch), "{time_index}");
        assert_eq!(h.hourly.palace_names[at(branch)], Palace::Soul);
    }
}

/// 新开关随 config 的线协议往返；DTO 的 config 只写非默认的开关，`horoscopeDivide` 照写。
#[test]
fn horoscope_switches_roundtrip_and_echo_only_when_set() {
    let config: Config = serde_json::from_value(serde_json::json!({
        "horoscopeDivide": "exactMonth",
        "decadalStem": "continuous",
        "childhoodPalace": "spouse",
        "ageMutagens": "none",
        "yearlyPalace": "age",
        "scopeMutagenStem": "scopeSoulPalace",
        "monthlyPalace": "monthBranch",
        "dailyPalace": "afterMonthly",
        "leapDaily": "continuous",
        "hourlyPalace": "branch",
    }))
    .unwrap();
    let back: Config = serde_json::from_value(serde_json::to_value(&config).unwrap()).unwrap();
    assert_eq!(back, config);
    let dto = serde_json::to_value(solar("2001-5-1", 2, Gender::Male, config).to_dto()).unwrap();
    for (key, value) in [
        ("horoscopeDivide", "exactMonth"),
        ("decadalStem", "continuous"),
        ("childhoodPalace", "spouse"),
        ("ageMutagens", "none"),
        ("yearlyPalace", "age"),
        ("scopeMutagenStem", "scopeSoulPalace"),
        ("monthlyPalace", "monthBranch"),
        ("dailyPalace", "afterMonthly"),
        ("leapDaily", "continuous"),
        ("hourlyPalace", "branch"),
    ] {
        assert_eq!(dto["config"][key], value, "{key}");
    }
    let err = serde_json::from_str::<Config>(r#"{"hourlyPalace":"daily"}"#).unwrap_err();
    assert!(
        err.to_string()
            .starts_with("invalid hourlyPalace 'daily': expected 'fromDaily' or 'branch'"),
        "{err}"
    );
}

/// 亮度「地」照录王亭之 p101–102：自定义亮度表可用，随 config 往返，DTO 写标识 `di`、文本写「地」。
#[test]
fn di_brightness_in_custom_table() {
    let mut table = [None; 12];
    // 贪狼卯宫为地（寅宫为 0，卯宫为 1）
    table[1] = Some(Brightness::Di);
    let config = Config::default().with_brightness(StarKey::TanlangMaj, table);
    let json = serde_json::to_value(&config).unwrap();
    assert_eq!(json["brightness"]["tanlangMaj"][1], "di");
    let back: Config = serde_json::from_value(json).unwrap();
    assert_eq!(back, config);
    let chart = (1..=12)
        .flat_map(|m| (0..12).map(move |t| (m, t)))
        .map(|(m, t)| lunar(&format!("1990-{m}-10"), t, Gender::Male, config.clone()))
        .find(|c| {
            c.star(StarKey::TanlangMaj).unwrap().palace().earthly_branch == EarthlyBranch::Mao
        })
        .unwrap();
    let star = chart.star(StarKey::TanlangMaj).unwrap();
    assert_eq!(star.brightness, Some(Brightness::Di));
    assert_eq!(
        x_iztro::translate_brightness(Brightness::Di, Language::ZhCN),
        "地"
    );
}

/// 三张一览表的四化格在所用天干不是该层自身的干时注明所用的干：运限四化取该层命宫宫干、大限干连续排。
#[test]
fn overview_tables_name_the_mutagen_stem() {
    let config = Config {
        scope_mutagen_stem: ScopeMutagenStem::ScopeSoulPalace,
        decadal_stem: DecadalStem::Continuous,
        ..Config::default()
    };
    let chart = solar("1986-3-20", 4, Gender::Female, config);
    let row = |text: &str, prefix: &str| -> String {
        text.lines()
            .find(|l| l.starts_with(prefix))
            .unwrap_or_else(|| panic!("没有以 {prefix} 起头的行：\n{text}"))
            .to_string()
    };
    let yearly = chart.yearly_overview_to_text(2009, 2009).unwrap();
    assert!(row(&yearly, "| 2009 |").contains("| (辛干) "), "{yearly}");
    let decadals = chart.decadal_list_to_text();
    assert!(row(&decadals, "| 11 |").contains("| (辛干) "), "{decadals}");
    assert!(row(&decadals, "| 12 |").contains("| (庚干) "), "{decadals}");
    let monthlies = chart.monthly_list_to_text(2009).unwrap();
    assert!(
        row(&monthlies, "| 1 |").contains("| (壬干) "),
        "{monthlies}"
    );
    // 默认口径下四化即该层的干，不注明
    let plain = solar("1986-3-20", 4, Gender::Female, Config::default());
    assert!(
        !plain
            .yearly_overview_to_text(2009, 2009)
            .unwrap()
            .contains("干) ")
    );
}

/// 飞星派「命宫无大限，第二大限前都以夫妻宫立太极」（《课程》p19 例盘：
/// 1987-5-9 辰时男，木三局，第一大限 3–12 在命宫，第二大限 13–22 在兄弟宫）。
#[test]
fn spouse_until_second_decade_covers_first_decade() {
    let config = |childhood_palace| Config {
        childhood_palace,
        ..Config::default()
    };
    let chart = solar(
        "1987-5-9",
        4,
        Gender::Male,
        config(ChildhoodPalace::SpouseUntilSecondDecade),
    );
    let soul = chart.palace(Palace::Soul).unwrap();
    let spouse = chart.palace(Palace::Spouse).unwrap();
    let siblings = chart.palace(Palace::Siblings).unwrap();
    assert_eq!(soul.decadal.range, (3, 12), "例盘第一大限在命宫");
    assert_eq!(siblings.decadal.range, (13, 22), "例盘第二大限在兄弟宫");

    // 虚岁 1（起运前）、虚岁 8（第一大限内）都以夫妻宫立太极，干支取夫妻宫的宫干支
    for (date, name) in [
        ("1987-6-1", HoroscopeName::Childhood),
        ("1994-6-1", HoroscopeName::Decadal),
    ] {
        let h = chart.horoscope(date, 0).unwrap();
        assert_eq!(h.decadal.index, spouse.index, "{date}");
        assert_eq!(h.decadal.name_key, name, "{date}");
        assert_eq!(
            (h.decadal.heavenly_stem, h.decadal.earthly_branch),
            (spouse.heavenly_stem, spouse.earthly_branch),
            "{date}"
        );
        assert_eq!(h.decadal.palace_names[spouse.index], Palace::Soul, "{date}");
    }
    // 第二大限起照常
    let h = chart.horoscope("2000-6-1", 0).unwrap();
    assert_eq!(h.decadal.index, siblings.index);
    // spouse 只改起运之前：第一大限仍在命宫
    let only_childhood = solar("1987-5-9", 4, Gender::Male, config(ChildhoodPalace::Spouse));
    assert_eq!(
        only_childhood
            .horoscope("1994-6-1", 0)
            .unwrap()
            .decadal
            .index,
        soul.index
    );
}

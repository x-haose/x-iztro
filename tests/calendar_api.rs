//! 历法查询入口：对照 lunar-typescript 的金标、互换往返、与排盘取值一致、非法输入。
//!
//! 金标 `tests/golden/calendar_api.csv` 由 `generate_calendar_api.mjs` 生成，每个农历年一行规范化串哈希；
//! 不一致时用 `node generate_calendar_api.mjs --inspect <year>` 打印 JS 侧的规范化串对照。

use sha2::{Digest, Sha256};
use x_iztro::{
    Config, EarthlyBranch, Gender, HeavenlyStem, IztroError, Language, RawLunarDate, RawSolarDate,
    YearDivide, by_solar, lunar_months, lunar_to_solar, lunar_year_pillar, solar_to_lunar,
};

fn ymd(d: RawSolarDate) -> String {
    format!(
        "{:04}-{:02}-{:02}",
        d.solar_year, d.solar_month, d.solar_day
    )
}

/// 农历 `y` 年的规范化串，逐行与 generate_calendar_api.mjs 的 `yearLines` 同构；
/// 同时检查每一天 `solar_to_lunar` 换回原日期（公历 10000 年不在入参范围内，跳过）。
fn year_lines(y: i64) -> Vec<String> {
    let months = lunar_months(y).expect("1-9999 年");
    let mut lines: Vec<String> = months
        .iter()
        .map(|m| {
            format!(
                "M {},{},{},{}",
                m.month,
                u8::from(m.is_leap),
                m.day_count,
                ymd(m.first_day)
            )
        })
        .collect();
    let (stem, branch) = lunar_year_pillar(y).expect("1-9999 年");
    lines.push(format!("P {},{}", stem.index(), branch.index()));
    for m in &months {
        for d in 1..=m.day_count {
            let solar = lunar_to_solar(y, m.month, d, m.is_leap).expect("月份表里的日期");
            if d == 1 {
                assert_eq!(solar, m.first_day, "{y} 年 {} 月初一", m.month);
            }
            if solar.solar_year <= 9999 {
                let back = solar_to_lunar(solar.solar_year, solar.solar_month, solar.solar_day)
                    .expect("公历 1-9999 年");
                let want = RawLunarDate {
                    lunar_year: y,
                    lunar_month: m.month,
                    lunar_day: d,
                    is_leap: m.is_leap,
                };
                assert_eq!(back, want, "{} 换回农历", ymd(solar));
            }
            lines.push(format!(
                "D {},{},{},{}",
                m.month,
                u8::from(m.is_leap),
                d,
                ymd(solar)
            ));
        }
    }
    lines
}

fn hash(lines: &[String]) -> String {
    hex::encode(Sha256::digest(lines.join("\n").as_bytes()))[..32].to_string()
}

/// 逐年核对 `calendar_api.csv` 中 `keep` 选中的年份，返回 (核对年数, 不一致的年份)。
fn check(keep: impl Fn(i64) -> bool) -> (usize, Vec<i64>) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/calendar_api.csv");
    let data = std::fs::read_to_string(path)
        .expect("读取 calendar_api.csv（由 generate_calendar_api.mjs 生成）");
    let mut mismatches = Vec::new();
    let mut years = 0;
    for line in data.lines() {
        let (y, expected) = line.split_once(',').expect("year,hash");
        let y: i64 = y.parse().expect("首列为年份");
        if !keep(y) {
            continue;
        }
        if hash(&year_lines(y)) != expected {
            mismatches.push(y);
        }
        years += 1;
    }
    (years, mismatches)
}

/// 历法分支与边界：首尾年份、历史闰月特例（37、75）、景初与武周改月名（237-239、689-701）、
/// 改历（1582、1583）、公历 10000 年的尾月（9999）、闰月年份（2023、2025）；其余每 97 年取一年。
const SAMPLE_YEARS: &[i64] = &[
    1, 2, 37, 75, 237, 238, 239, 689, 697, 700, 701, 1582, 1583, 1645, 2023, 2024, 2025, 2057, 9999,
];

#[test]
fn calendar_api_matches_lunar_typescript_on_sample_years() {
    let keep = |y: i64| SAMPLE_YEARS.contains(&y) || y % 97 == 0;
    let (years, mismatches) = check(keep);
    assert_eq!(years, (1..=9999).filter(|&y| keep(y)).count());
    assert!(
        mismatches.is_empty(),
        "与 lunar-typescript 不一致：{mismatches:?}"
    );
}

#[test]
#[ignore = "全量 9999 个农历年逐日，release 约 2 秒、debug 约半分钟；make check-rust 以 release 跑"]
fn calendar_api_matches_lunar_typescript_for_every_year() {
    let (years, mismatches) = check(|_| true);
    assert_eq!(years, 9999, "calendar_api.csv 应覆盖 1-9999 年");
    assert!(
        mismatches.is_empty(),
        "{} 处与 lunar-typescript 不一致（前 20 处）：{:?}",
        mismatches.len(),
        &mismatches[..mismatches.len().min(20)]
    );
}

#[test]
fn leap_month_round_trip() {
    let months = lunar_months(2023).unwrap();
    assert_eq!(months.len(), 13);
    assert!(months[2].is_leap && months[2].month == 2);
    assert_eq!(
        lunar_to_solar(2023, 2, 1, true).unwrap(),
        RawSolarDate {
            solar_year: 2023,
            solar_month: 3,
            solar_day: 22
        }
    );
    assert_eq!(
        solar_to_lunar(2023, 3, 22).unwrap(),
        RawLunarDate {
            lunar_year: 2023,
            lunar_month: 2,
            lunar_day: 1,
            is_leap: true
        }
    );
}

#[test]
fn solar_year_one_starts_in_lunar_year_zero() {
    assert_eq!(
        solar_to_lunar(1, 1, 1).unwrap(),
        RawLunarDate {
            lunar_year: 0,
            lunar_month: 11,
            lunar_day: 18,
            is_leap: false
        }
    );
}

#[test]
fn agrees_with_chart_lunar_date_and_year_pillar() {
    // 2024-02-04 立春、2024-02-10 正月初一：其间的日子正月初一分界属癸卯，立春分界属甲辰
    for (date, ymd) in [
        ("2024-2-3", (2024, 2, 3)),
        ("2024-2-4", (2024, 2, 4)),
        ("2024-2-9", (2024, 2, 9)),
        ("2024-2-10", (2024, 2, 10)),
    ] {
        let lunar = solar_to_lunar(ymd.0, ymd.1, ymd.2).unwrap();
        for divide in [YearDivide::Normal, YearDivide::Exact] {
            let config = Config {
                year_divide: divide,
                ..Config::default()
            };
            let chart = by_solar(date, 6, Gender::Female, true, Language::ZhCN, config).unwrap();
            assert_eq!(chart.raw_dates.lunar_date, lunar, "{date}");
            let pillar_year = match divide {
                YearDivide::Normal => lunar.lunar_year,
                YearDivide::Exact => {
                    if ymd >= (2024, 2, 4) {
                        2024
                    } else {
                        2023
                    }
                }
            };
            assert_eq!(
                chart.raw_dates.chinese_date.yearly,
                lunar_year_pillar(pillar_year).unwrap(),
                "{date} {divide:?}"
            );
        }
    }
    assert_eq!(
        lunar_year_pillar(2024).unwrap(),
        (HeavenlyStem::Jia, EarthlyBranch::Chen)
    );
}

fn is_invalid_date(r: Result<impl std::fmt::Debug, IztroError>) -> bool {
    matches!(r, Err(IztroError::InvalidDate(_)))
}

fn is_invalid_argument(r: Result<impl std::fmt::Debug, IztroError>) -> bool {
    matches!(r, Err(IztroError::InvalidArgument(_)))
}

#[test]
fn rejects_dates_that_do_not_exist() {
    assert!(is_invalid_date(solar_to_lunar(0, 1, 1)));
    assert!(is_invalid_date(solar_to_lunar(10000, 1, 1)));
    assert!(is_invalid_date(solar_to_lunar(2024, 13, 1)));
    assert!(is_invalid_date(solar_to_lunar(2023, 2, 29)));
    assert!(is_invalid_date(solar_to_lunar(1582, 10, 10)));
    assert!(solar_to_lunar(1582, 10, 4).is_ok() && solar_to_lunar(1582, 10, 15).is_ok());

    assert!(is_invalid_date(lunar_to_solar(0, 1, 1, false)));
    assert!(is_invalid_date(lunar_to_solar(2024, 0, 1, false)));
    // 2023 年闰二月、2024 年无闰月：不退回常规月
    assert!(is_invalid_date(lunar_to_solar(2023, 3, 1, true)));
    assert!(is_invalid_date(lunar_to_solar(2024, 2, 1, true)));
    // 小月没有三十
    let small = lunar_months(2024)
        .unwrap()
        .into_iter()
        .find(|m| m.day_count == 29)
        .unwrap();
    assert!(is_invalid_date(lunar_to_solar(
        2024,
        small.month,
        30,
        false
    )));
    // 农历 237 年没有三月
    assert!(is_invalid_date(lunar_to_solar(237, 3, 1, false)));

    assert!(is_invalid_argument(lunar_months(0)));
    assert!(is_invalid_argument(lunar_months(10000)));
    assert!(is_invalid_argument(lunar_year_pillar(0)));
    assert!(is_invalid_argument(lunar_year_pillar(10000)));
}

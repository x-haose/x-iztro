//! 历法全量金标：公历 1-9999 年逐年比对 `tests/golden/calendar.csv`（lunar-typescript 生成）。
//!
//! 每年三层哈希，规范化串的构造与 `tests/golden/generate_calendar.mjs` 逐行同构；不一致时用
//! `node generate_calendar.mjs --inspect <year> <year|day|jieqi>` 打印 JS 侧的规范化串对照。

use sha2::{Digest, Sha256};

use super::lunar::{self, Pillar};
use super::{LunarYear, SolarTime, days_in_month, is_reform_gap, lunar_year, sign_index};

/// 排盘时刻的小时：时辰 0-12 对应 max(2t-1, 0)。
const CHART_HOURS: [i64; 13] = [0, 1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23];

/// 规范化串（逐行以换行连接）SHA-256 的前 32 个 hex 字符。
fn hash(lines: &[String]) -> String {
    hex::encode(Sha256::digest(lines.join("\n").as_bytes()))[..32].to_string()
}

/// 定宽日期 `YYYY-MM-DD`，与 JS 的 `toYmd` 同格式。
fn ymd((y, m, d): (i64, i64, i64)) -> String {
    format!("{y:04}-{m:02}-{d:02}")
}

/// 一柱写成「干序,支序」。
fn gz((stem, branch): Pillar) -> String {
    format!("{},{}", stem.index(), branch.index())
}

/// year 层：农历 `y` 年的月表、本年各月首末日的公历日期、闰月与节气时刻。
fn year_lines(y: i64) -> Vec<String> {
    let ly = LunarYear::compute(y);
    let mut lines: Vec<String> = ly
        .months
        .iter()
        .map(|m| {
            format!(
                "M {},{},{},{}",
                m.year, m.month, m.day_count, m.first_julian_day
            )
        })
        .collect();
    for m in ly.months.iter().filter(|m| m.year == y) {
        let first = lunar::solar_date(&ly, m.month, 1).expect("本年的月必有初一");
        let last = lunar::solar_date(&ly, m.month, m.day_count).expect("本年的月必有末日");
        lines.push(format!("S {},{},{}", m.month, ymd(first), ymd(last)));
    }
    lines.push(format!("L {}", ly.leap_month().unwrap_or(0)));
    lines.extend(ly.jie_qi().keys.iter().map(|k| format!("Q {k}")));
    lines
}

/// 公历 `y` 年的全部 (月, 日)，跳过改历空缺。
fn solar_days(y: i64) -> impl Iterator<Item = (i64, i64)> {
    (1..=12).flat_map(move |m| {
        (1..=days_in_month(y, m))
            .filter(move |&d| !is_reform_gap(y, m, d))
            .map(move |d| (m, d))
    })
}

/// day 层：公历 `y` 年逐日 12:30 的农历日期、两种年柱、精确月柱、日柱与星座序号。
fn day_lines(y: i64) -> Vec<String> {
    let ly = lunar_year(y);
    solar_days(y)
        .map(|(m, d)| {
            let t = SolarTime::new(y, m, d, 12, 30);
            let l = lunar::lunar(&ly, &t);
            format!(
                "{m}-{d} {},{},{} {} {} {} {} {}",
                l.date.year,
                l.date.month,
                l.date.day,
                gz(l.year),
                gz(lunar::year_by_li_chun(&ly, &t, l.date.year)),
                gz(lunar::month_exact(&ly, &t, l.date.year)),
                gz(l.day),
                sign_index(m, d)
            )
        })
        .collect()
}

/// jieqi 层：公历 `y` 年每个「节」所在日 13 个排盘时刻的精确月柱、exact 日柱与时柱。
fn jie_qi_lines(y: i64) -> Vec<String> {
    let ly = lunar_year(y);
    let mut days: Vec<(i64, i64)> = ly
        .jie_qi()
        .times
        .iter()
        .step_by(2)
        .filter(|t| t.year == y)
        .map(|t| (t.month, t.day))
        .collect();
    days.sort_unstable();
    days.dedup();
    days.into_iter()
        .map(|(m, d)| {
            let cells: Vec<String> = CHART_HOURS
                .iter()
                .map(|&h| {
                    let t = SolarTime::new(y, m, d, h, 30);
                    let l = lunar::lunar(&ly, &t);
                    let month = lunar::month_exact(&ly, &t, l.date.year);
                    format!("{},{},{}", gz(month), gz(l.day_exact), gz(l.time))
                })
                .collect();
            format!("{m}-{d} {}", cells.join(" "))
        })
        .collect()
}

/// 逐年核对 `calendar.csv` 中 `keep` 选中的年份，返回 (核对年数, 不一致的「年份 层」)。
fn check(keep: impl Fn(i64) -> bool) -> (usize, Vec<String>) {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/calendar.csv");
    let data =
        std::fs::read_to_string(path).expect("读取 calendar.csv（由 generate_calendar.mjs 生成）");
    let mut mismatches = Vec::new();
    let mut years = 0;
    for line in data.lines() {
        let cols: Vec<&str> = line.split(',').collect();
        let y: i64 = cols[0].parse().expect("首列为年份");
        if !keep(y) {
            continue;
        }
        for (layer, actual, expected) in [
            ("year", hash(&year_lines(y)), cols[1]),
            ("day", hash(&day_lines(y)), cols[2]),
            ("jieqi", hash(&jie_qi_lines(y)), cols[3]),
        ] {
            if actual != expected {
                mismatches.push(format!("{y} {layer}"));
            }
        }
        years += 1;
    }
    (years, mismatches)
}

/// 抽样层覆盖每条算法分支与易错年份：儒略历闰世纪与改历（100、1582、1583）、古历平朔平气 / 低精度加修正 /
/// 高精度三段及其分界（619、1645、1959、1960）、历史闰月特例（37 闰十二月、75 闰十一月）、景初与武周改月名
/// （237-239、689-701）、节气落在儒略闰日附近（900、1000）、年份 10000 的比较键（9999），
/// 以及逐年比对 lunar_rust 时出过差异的年份；其余每 97 年取一年。
const SAMPLE_YEARS: &[i64] = &[
    1, 4, 37, 75, 100, 237, 238, 239, 619, 620, 641, 689, 690, 700, 701, 900, 1000, 1001, 1582,
    1583, 1600, 1602, 1622, 1645, 1650, 1651, 1790, 1791, 1925, 1959, 1960, 2034, 2040, 2043, 2047,
    2057, 3241, 5255, 9999,
];

#[test]
fn calendar_matches_lunar_typescript_on_sample_years() {
    let keep = |y: i64| SAMPLE_YEARS.contains(&y) || y % 97 == 0;
    let (years, mismatches) = check(keep);
    assert_eq!(
        years,
        (1..=9999).filter(|&y| keep(y)).count(),
        "抽样年份应全部出现在 calendar.csv 中"
    );
    assert!(
        mismatches.is_empty(),
        "与 lunar-typescript 不一致：{mismatches:?}"
    );
}

#[test]
#[ignore = "全量 9999 年，release 约 10 秒、debug 约 6 分钟"]
fn calendar_matches_lunar_typescript_for_every_year() {
    let (years, mismatches) = check(|_| true);
    assert_eq!(years, 9999, "calendar.csv 应覆盖 1-9999 年");
    assert!(
        mismatches.is_empty(),
        "{} 处与 lunar-typescript 不一致（前 20 处）：{:?}",
        mismatches.len(),
        &mismatches[..mismatches.len().min(20)]
    );
}

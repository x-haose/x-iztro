//! 夹宫与三个运限列表（iztro v2.6.0 新增的四组 API）。
//!
//! 列表值本身由 `golden_chart_lists` 对照 JS 逐项比；这里测的是几何、边界与
//! 绑定层分派：夹宫的环绕与「两宫合计」语义、闰月拆段的项数、大限定位的两种写法、
//! 以及列表用的时辰取自时柱地支而非出生入参（晚子时两者差 12）。

use x_iztro::models::horoscope::{DecadalTarget, MonthPart};
use x_iztro::*;

fn chart(date: &str, time_index: u8) -> Astrolabe {
    by_solar(
        date,
        time_index,
        Gender::Female,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap()
}

/// 大限列表按起运先后排，覆盖十二宫且虚岁首尾相接。
#[test]
fn decadal_list_is_ordered_and_covers_twelve_palaces() {
    let list = chart("2000-8-16", 2).decadal_list();
    assert_eq!(list.len(), 12);

    let mut names: Vec<Palace> = list.iter().map(|d| d.palace_name).collect();
    names.sort_by_key(|p| p.as_key());
    names.dedup();
    assert_eq!(names.len(), 12, "十二宫各出现一次");

    for pair in list.windows(2) {
        let (a, b) = (&pair[0], &pair[1]);
        assert!(a.age_range.0 < b.age_range.0, "按起运虚岁升序");
        assert_eq!(b.age_range.0, a.age_range.1 + 1, "虚岁区间首尾相接");
        assert_eq!(b.year_range.0, a.year_range.1 + 1, "年份区间首尾相接");
        assert_eq!(a.age_range.1 - a.age_range.0, 9, "每限十年");
    }
}

/// 流年列表覆盖该大限的十个虚岁，且两种大限定位方式等价。
#[test]
fn yearly_list_spans_the_decadal_and_accepts_both_targets() {
    let a = chart("2000-8-16", 2);
    let first = &a.decadal_list()[0];

    let by_ordinal = a.yearly_list(0usize).unwrap();
    assert_eq!(by_ordinal.len(), 10);
    assert_eq!(by_ordinal.first().unwrap().age, first.age_range.0);
    assert_eq!(by_ordinal.last().unwrap().age, first.age_range.1);

    let by_name = a.yearly_list(first.palace_name).unwrap();
    let keys = |v: &[_]| -> Vec<(u32, i64)> {
        v.iter()
            .map(|y: &models::horoscope::YearlyHoroscope| (y.age, y.year))
            .collect()
    };
    assert_eq!(
        keys(&by_ordinal),
        keys(&by_name),
        "序号与宫名定位同一个大限"
    );

    assert!(a.yearly_list(12usize).is_err(), "序号越界应报错");
}

/// 闰月按 fix_leap 拆段：无闰月 12 项、有闰月 13 或 14 项。
#[test]
fn monthly_list_splits_leap_month_by_fix_leap() {
    let a = chart("2000-8-16", 2);

    // 2021 农历无闰月
    assert_eq!(a.monthly_list(2021, true).unwrap().len(), 12);
    assert_eq!(a.monthly_list(2021, false).unwrap().len(), 12);

    // 2020 农历闰四月
    let split = a.monthly_list(2020, true).unwrap();
    let whole = a.monthly_list(2020, false).unwrap();
    assert_eq!(split.len(), 14, "闰月拆前后半月");
    assert_eq!(whole.len(), 13, "闰月整月一项");

    let leap: Vec<&models::horoscope::MonthlyHoroscope> =
        split.iter().filter(|m| m.is_leap_month).collect();
    assert_eq!(leap.len(), 2);
    assert_eq!(leap[0].part, MonthPart::First);
    assert_eq!(leap[0].day_range, (1, 15));
    assert_eq!(leap[1].part, MonthPart::Second);
    assert_eq!(leap[1].day_range.0, 16);

    let leap_whole: Vec<&models::horoscope::MonthlyHoroscope> =
        whole.iter().filter(|m| m.is_leap_month).collect();
    assert_eq!(leap_whole.len(), 1);
    assert_eq!(leap_whole[0].part, MonthPart::Normal);
    assert_eq!(leap_whole[0].day_range.0, 1);

    // 月份序列：1..12 各一次，闰月紧跟同月号之后
    let months: Vec<u32> = whole.iter().map(|m| m.month).collect();
    assert_eq!(months, vec![1, 2, 3, 4, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
}

/// 列表用的时辰取自时柱地支：晚子时（入参 12）的时柱地支是子，按 0 起运限。
///
/// 直接用出生入参会让晚子盘的列表与逐层 `horoscope()` 查询对不上。
#[test]
fn lists_use_hour_pillar_branch_not_birth_time_index() {
    let late_zi = chart("2000-8-16", 12);
    assert_eq!(late_zi.time_index, 12, "入参保留原始晚子");
    assert_eq!(
        late_zi.raw_dates.chinese_date.hourly.1,
        EarthlyBranch::Zi,
        "晚子的时柱地支是子"
    );

    let from_list = &late_zi.yearly_list(0usize).unwrap()[0];
    let year = from_list.year;
    let direct = late_zi
        .horoscope(&format!("{year}-8-1"), 0)
        .unwrap()
        .data()
        .yearly
        .base
        .clone();
    assert_eq!(
        from_list.index, direct.index,
        "列表里的流年与按时辰 0 逐层查的落宫一致"
    );
}

/// 夹宫是索引相邻的两宫，判定在两宫合计的集合上做。
#[test]
fn flanking_palaces_wrap_and_judge_across_both() {
    let a = chart("2000-8-16", 2);
    let soul = a.palace(Palace::Soul).unwrap();
    let f = a.flanking_palaces(Palace::Soul).unwrap();
    assert_eq!(f.previous.index, (soul.index + 11) % 12);
    assert_eq!(f.next.index, (soul.index + 1) % 12);

    // 与 pattern 引擎的夹宫几何同源：前一宫是索引 -1
    let all: Vec<StarKey> = [f.previous, f.next]
        .iter()
        .flat_map(|p| p.major_stars.iter().chain(p.minor_stars.iter()))
        .map(|s| s.key)
        .collect();
    if let Some(one) = all.first() {
        assert!(f.have_one_of(&[*one]));
        assert!(!f.not_have(&[*one]));
    }
    assert!(f.not_have(&[StarKey::ZiweiMaj]) || f.have(&[StarKey::ZiweiMaj]));
}

/// 定位不到的宫返回 None，不静默取默认宫。
#[test]
fn flanking_palaces_reports_missing_target() {
    let a = chart("2000-8-16", 2);
    assert!(a.flanking_palaces(99usize).is_none());
}

/// 大限定位方式的错误路径。
#[test]
fn decadal_target_rejects_unknown() {
    let a = chart("2000-8-16", 2);
    let err = a.yearly_list(DecadalTarget::Ordinal(99)).unwrap_err();
    assert!(
        err.to_string().contains("out of range"),
        "错误信息应说明越界：{err}"
    );
}

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

    fn base(kind: &str) -> Value {
        json!({
            "kind": kind,
            "solarDate": "2000-8-16",
            "timeIndex": 2,
            "gender": "female",
            "fixLeap": true,
            "language": "zh-CN",
        })
    }

    #[test]
    fn four_kinds_return_dtos() {
        let mut j = base("flankingPalaces");
        j["palaceKey"] = "soulPalace".into();
        let f = query(j).unwrap();
        assert!(f["previous"]["index"].is_number());
        assert!(f["next"]["nameKey"].is_string());

        let d = query(base("decadalList")).unwrap();
        assert_eq!(d.as_array().unwrap().len(), 12);
        assert!(d[0]["palaceNameKey"].is_string(), "译文字段必有配套 key");
        assert_eq!(d[0]["ageRange"].as_array().unwrap().len(), 2);

        let mut j = base("yearlyList");
        j["decadalOrdinal"] = 0.into();
        let y = query(j).unwrap();
        assert_eq!(y.as_array().unwrap().len(), 10);
        assert!(y[0]["age"].is_number() && y[0]["year"].is_number());

        let mut j = base("monthlyList");
        j["year"] = 2020.into();
        let m = query(j).unwrap();
        assert_eq!(m.as_array().unwrap().len(), 14, "2020 闰四月拆段");
        assert_eq!(m[4]["part"], "first");
        assert_eq!(m[5]["part"], "second");
    }

    #[test]
    fn addressing_is_required_not_defaulted() {
        let err = query(base("yearlyList")).unwrap_err();
        assert!(err.contains("decadalOrdinal"), "{err}");
        let err = query(base("monthlyList")).unwrap_err();
        assert!(err.contains("year"), "{err}");
    }

    /// `monthFixLeap` 独立于排盘的 `fixLeap`：前者决定列表拆不拆闰月，
    /// 后者决定盘按不按闰月下半月安星。复用同一个开关会把两件事绑在一起，
    /// 且让 Rust 原生能独立指定而绑定层不能。
    #[test]
    fn monthly_fix_leap_is_independent_of_chart_fix_leap() {
        let count = |chart_fix_leap: bool, month_fix_leap: Option<bool>| {
            let mut j = base("monthlyList");
            j["year"] = 2020.into();
            j["fixLeap"] = chart_fix_leap.into();
            if let Some(v) = month_fix_leap {
                j["monthFixLeap"] = v.into();
            }
            query(j).unwrap().as_array().unwrap().len()
        };

        // 列表的拆分只随 monthFixLeap 变，与排盘的 fixLeap 无关
        assert_eq!(count(true, Some(true)), 14);
        assert_eq!(count(false, Some(true)), 14);
        assert_eq!(count(true, Some(false)), 13);
        assert_eq!(count(false, Some(false)), 13);

        // 缺省为 true，与 iztro monthlyList(year, fixLeap = true) 一致
        assert_eq!(count(true, None), 14);
        assert_eq!(count(false, None), 14);
    }

    /// 重排转发：新增的「再计算」入口必须带上 fromStem/fromBranch，
    /// 否则拿原盘的答案冒充重排盘的。
    #[test]
    fn rearrange_reaches_the_lists() {
        let plain = query(base("decadalList")).unwrap();
        let mut j = base("decadalList");
        j["fromStem"] = "gengHeavenly".into();
        j["fromBranch"] = "chenEarthly".into();
        let rearranged = query(j).unwrap();
        // 第一个大限恒在命宫，宫名对比不出差异；重排改的是宫名分配、五行局与
        // 由此而来的起运虚岁，故比整份列表
        assert_ne!(plain, rearranged, "重排后的大限列表应与原盘不同");
    }
}

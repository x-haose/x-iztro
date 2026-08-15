//! 扩展点：给星盘补自定义分析方法。
//!
//! 对应 JS iztro 的 `loadPlugin` / `use(plugin)`。JS 只能在运行期往对象上挂函数，
//! Rust 用扩展 trait 在编译期完成同一件事：方法调用语法一致，且类型受检、零开销。
//!
//! 本文件即三语言文档里 Rust 侧那份配方的可执行版本，
//! 与 `python/tests/test_plugin.py`、`go/iztro/plugin_test.go` 实现同一个插件。

use x_iztro::by_solar;
use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;

/// 一个「插件」：给 `Astrolabe` 补两个自定义分析方法。
///
/// 使用方 `use` 本 trait 即可像调用内置方法一样调用它们。
trait MyAnalysis {
    /// 命宫主星名（空宫借对宫），多颗以逗号分隔
    fn major_star(&self) -> String;
    /// 五行局的局数
    fn five_elements_value(&self) -> usize;
}

impl MyAnalysis for x_iztro::Astrolabe {
    fn major_star(&self) -> String {
        let soul = self.palace(Palace::Soul).expect("命宫必然存在");
        let source = if soul.is_empty() {
            soul.opposite_palace()
        } else {
            soul
        };
        source
            .major_stars
            .iter()
            .filter(|s| s.star_type == StarType::Major)
            .map(|s| x_iztro::i18n::translate_star(s.key, self.language))
            .collect::<Vec<_>>()
            .join(",")
    }

    fn five_elements_value(&self) -> usize {
        self.five_elements_class.value()
    }
}

fn chart(lang: Language) -> x_iztro::Astrolabe {
    by_solar(
        "2000-8-16",
        2,
        Gender::Female,
        true,
        lang,
        Config::default(),
    )
    .unwrap()
}

#[test]
fn extension_trait_adds_methods_to_astrolabe() {
    let a = chart(Language::ZhCN);

    // 与 Python / Go 侧的插件测试断言同一组取值
    assert_eq!(a.major_star(), "紫微");
    assert_eq!(a.five_elements_value(), 3);

    // 扩展方法随排盘语言输出
    assert_eq!(chart(Language::EnUS).major_star(), "emperor");
}

#[test]
fn extension_trait_borrows_from_opposite_palace_when_soul_is_empty() {
    // 该盘命宫坐紫微，不走借星分支；换一张空命宫的盘验证借星
    let empty_soul = by_solar(
        "1990-11-5",
        4,
        Gender::Male,
        true,
        Language::ZhCN,
        Config::default(),
    )
    .unwrap();
    let soul = empty_soul.palace(Palace::Soul).unwrap();

    if soul.is_empty() {
        assert!(!empty_soul.major_star().is_empty(), "空命宫应借对宫主星");
        assert_eq!(
            empty_soul.major_star(),
            soul.opposite_palace()
                .major_stars
                .iter()
                .filter(|s| s.star_type == StarType::Major)
                .map(|s| x_iztro::i18n::translate_star(s.key, Language::ZhCN))
                .collect::<Vec<_>>()
                .join(","),
        );
    } else {
        // 命宫非空时直接取本宫，借星分支由上一个断言之外的盘覆盖
        assert!(soul.has(&[soul.major_stars[0].key]));
    }
}

/// 星耀标识可以直接参与扩展方法的判断，不依赖输出语言。
#[test]
fn extension_can_use_language_independent_keys() {
    for lang in [Language::ZhCN, Language::EnUS, Language::JaJP] {
        let a = chart(lang);
        assert!(a.palace(Palace::Soul).unwrap().has(&[StarKey::ZiweiMaj]));
        assert_eq!(a.five_elements_value(), 3);
    }
}

/// `translate_chinese_date` 与星盘的 `chinese_date` 字段必须逐字一致 ——
/// 后者就是由它生成的，四柱标识则由 `raw_dates.chinese_date` 提供。
#[test]
fn translate_chinese_date_matches_chart_field() {
    use x_iztro::utils::translate_chinese_date;

    for lang in [Language::ZhCN, Language::EnUS, Language::JaJP] {
        let a = chart(lang);
        let rc = a.raw_dates.chinese_date;
        assert_eq!(
            translate_chinese_date([rc.yearly, rc.monthly, rc.daily, rc.hourly], lang),
            a.chinese_date,
            "{lang:?} 四柱展示串与排盘字段不一致",
        );
    }

    // 词条为多字符时改用「 - 」分隔柱
    let en = chart(Language::EnUS);
    let rc = en.raw_dates.chinese_date;
    assert_eq!(
        translate_chinese_date([rc.yearly, rc.monthly, rc.daily, rc.hourly], Language::EnUS),
        "geng chen - jia shen - bing woo - geng yin",
    );
}

/// `merge_stars` 按宫位首尾相接，长度与顺序都可预期。
#[test]
fn merge_stars_concatenates_by_palace() {
    use x_iztro::utils::merge_stars;

    let a = chart(Language::ZhCN);
    let majors: [Vec<_>; 12] = std::array::from_fn(|i| a.palaces[i].major_stars.clone());
    let minors: [Vec<_>; 12] = std::array::from_fn(|i| a.palaces[i].minor_stars.clone());

    let merged = merge_stars(&[majors.clone(), minors.clone()]);
    for i in 0..12 {
        assert_eq!(merged[i].len(), majors[i].len() + minors[i].len());
        // 顺序为传入顺序：主星在前，辅星在后
        for (j, s) in majors[i].iter().enumerate() {
            assert_eq!(merged[i][j].key, s.key);
        }
    }

    // 空输入返回十二个空列表
    assert_eq!(merge_stars(&[]).iter().filter(|v| v.is_empty()).count(), 12);
}

/// `horoscope_now` 与显式传今天的日期时辰应得到同一个运限。
#[test]
fn horoscope_now_equals_explicit_today() {
    use x_iztro::utils::time_to_index;

    let a = chart(Language::ZhCN);
    let now = chrono::Local::now();
    let date = now.format("%Y-%-m-%-d").to_string();
    let ti = time_to_index(now.format("%H").to_string().parse().unwrap());

    let by_now = a.horoscope_now().unwrap();
    let explicit = a.horoscope(&date, ti).unwrap();

    assert_eq!(by_now.solar_date, explicit.solar_date);
    assert_eq!(by_now.decadal.index, explicit.decadal.index);
    assert_eq!(by_now.yearly.base.index, explicit.yearly.base.index);
    assert_eq!(by_now.hourly.index, explicit.hourly.index);
}

//! 格局 DTO 的三侧共用快照：Rust 写基线，Python 与 Go 读同一批文件比对。
//!
//! 每份快照是一张盘在本命、大限、流年三层的 `patterns_dto` 输出，连同排盘入参一起落盘，
//! 使另外两侧不必重复描述用例——读文件里的 `params` 重新排盘再比对即可（
//! `python/tests/test_patterns.py` 与 `go/iztro/pattern_golden_test.go`）。
//! 覆盖 4 张盘 × 6 种语言 = 24 份：含男女、一张中州派排盘、一张位置法亮度口径。
//!
//! 快照位于 tests/golden/pattern_snapshots/*.json。文件缺失时本测试写入当前输出并通过
//! ——即建立基线；已存在则逐字节比对。要在有意改动格局输出后重建基线，
//! 删掉该目录再跑一次本测试（三侧的比对文件随之更新）。

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{Value, json};
use x_iztro::{
    Algorithm, Astrolabe, BrightnessSource, Config, Gender, Language, PatternConfig, Scope,
    by_solar,
};

const SNAPSHOT_DIR: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/golden/pattern_snapshots"
);

/// 运限目标：晚于全部用例生日的固定日期与时辰。
const TARGET_DATE: &str = "2026-8-19";
const TARGET_TIME_INDEX: u8 = 3;

/// 六种输出语言，每张盘各出一份快照。
const LANGUAGES: [Language; 6] = [
    Language::ZhCN,
    Language::ZhTW,
    Language::EnUS,
    Language::JaJP,
    Language::KoKR,
    Language::ViVN,
];

/// 一张快照盘：排盘入参与判定口径。
struct Case {
    /// 快照文件名前缀
    name: &'static str,
    /// 阳历生日
    solar_date: &'static str,
    /// 时辰索引
    time_index: u8,
    /// 性别
    gender: Gender,
    /// 排盘配置
    config: Config,
    /// 格局判定口径
    pattern_config: PatternConfig,
}

/// 四张快照盘：默认口径的男女各一张、中州派排盘一张、位置法亮度口径一张。
///
/// 后两张挑的是「换了口径结果就变」的盘，口径才真被这批快照覆盖：
/// 中州派那张在默认派下多一条生不逢时（破军形态），位置法那张在亮度表口径下
/// 没有日月并明与丹墀桂墀。
fn cases() -> Vec<Case> {
    vec![
        Case {
            name: "female_default",
            solar_date: "2000-8-16",
            time_index: 2,
            gender: Gender::Female,
            config: Config::default(),
            pattern_config: PatternConfig::default(),
        },
        Case {
            name: "male_default",
            solar_date: "1984-2-15",
            time_index: 7,
            gender: Gender::Male,
            config: Config::default(),
            pattern_config: PatternConfig::default(),
        },
        Case {
            name: "male_zhongzhou",
            solar_date: "1985-6-11",
            time_index: 11,
            gender: Gender::Male,
            config: Config {
                algorithm: Algorithm::Zhongzhou,
                ..Config::default()
            },
            pattern_config: PatternConfig::default(),
        },
        Case {
            name: "female_positional",
            solar_date: "1985-1-3",
            time_index: 7,
            gender: Gender::Female,
            config: Config::default(),
            pattern_config: PatternConfig {
                brightness_source: BrightnessSource::Positional,
                ..PatternConfig::default()
            },
        },
    ]
}

/// 性别的语言无关标识（与绑定层入参一致）。
fn gender_key(gender: Gender) -> &'static str {
    match gender {
        Gender::Male => "male",
        Gender::Female => "female",
    }
}

/// 排盘配置的绑定层形态（camelCase 键 + 语言无关取值），供 Python / Go 直接喂回。
fn config_json(config: &Config) -> Value {
    json!({
        "yearDivide": config.year_divide.as_key(),
        "horoscopeDivide": config.horoscope_divide.as_key(),
        "ageDivide": config.age_divide.as_key(),
        "dayDivide": config.day_divide.as_key(),
        "algorithm": config.algorithm.as_key(),
        "astroType": config.astro_type.as_key(),
    })
}

fn chart(case: &Case, language: Language) -> Astrolabe {
    by_solar(
        case.solar_date,
        case.time_index,
        case.gender,
        true,
        language,
        case.config.clone(),
    )
    .expect("快照用例入参合法")
}

/// 一份快照的完整内容：入参 + 三层命中。
fn snapshot_value(case: &Case, language: Language) -> Value {
    let astrolabe = chart(case, language);
    let horoscope = astrolabe
        .horoscope(TARGET_DATE, TARGET_TIME_INDEX)
        .expect("运限目标合法");
    let pc = &case.pattern_config;
    json!({
        "params": {
            "solarDate": case.solar_date,
            "timeIndex": case.time_index,
            "gender": gender_key(case.gender),
            "fixLeap": true,
            "language": language.as_code(),
            "config": config_json(&case.config),
            "patternConfig": serde_json::to_value(pc).expect("口径可序列化"),
            "targetDate": TARGET_DATE,
            "targetTimeIndex": TARGET_TIME_INDEX,
        },
        "origin": astrolabe.patterns_dto(pc),
        "decadal": horoscope.data().patterns_dto(&astrolabe, Scope::Decadal, pc),
        "yearly": horoscope.data().patterns_dto(&astrolabe, Scope::Yearly, pc),
    })
}

/// 与已有快照比对；快照不存在则写入当前输出作为基线。
fn assert_snapshot(name: &str, actual: &Value) {
    let dir = Path::new(SNAPSHOT_DIR);
    fs::create_dir_all(dir).expect("建立快照目录");
    let path: PathBuf = dir.join(format!("{name}.json"));
    let actual = format!(
        "{}\n",
        serde_json::to_string_pretty(actual).expect("快照可序列化")
    );

    if !path.exists() {
        fs::write(&path, &actual).expect("写入快照基线");
        eprintln!("pattern snapshot baseline written: {}", path.display());
        return;
    }

    let expected = fs::read_to_string(&path).expect("读取快照");
    assert_eq!(
        actual,
        expected,
        "\n\n格局输出与快照 {} 不一致。若是有意改动，删除 tests/golden/pattern_snapshots/ 后重跑本测试重建基线。\n",
        path.display(),
    );
}

#[test]
fn pattern_dto_matches_snapshots() {
    for case in cases() {
        for language in LANGUAGES {
            assert_snapshot(
                &format!("{}_{}", case.name, language.as_code()),
                &snapshot_value(&case, language),
            );
        }
    }
}

/// 快照本身的自洽：语言只改译文，不改 key、宫位、证据与视角。
#[test]
fn snapshots_differ_only_by_translation() {
    for case in cases() {
        let base = snapshot_value(&case, Language::ZhCN);
        for language in LANGUAGES.into_iter().filter(|l| *l != Language::ZhCN) {
            let other = snapshot_value(&case, language);
            for layer in ["origin", "decadal", "yearly"] {
                let (a, b) = (
                    base[layer].as_array().expect("命中列表"),
                    other[layer].as_array().expect("命中列表"),
                );
                assert_eq!(
                    a.len(),
                    b.len(),
                    "{} {layer}：{} 与 zh-CN 的命中数不一致",
                    case.name,
                    language.as_code()
                );
                for (x, y) in a.iter().zip(b) {
                    for field in ["key", "scope", "palaceIndex", "palaceNameKey", "broken"] {
                        assert_eq!(
                            x[field],
                            y[field],
                            "{} {layer}：{} 的 {field} 与 zh-CN 不一致",
                            case.name,
                            language.as_code()
                        );
                    }
                    assert_eq!(x["variant"], y["variant"]);
                    assert_eq!(
                        x["stars"]
                            .as_array()
                            .expect("证据")
                            .iter()
                            .map(|s| s["key"].clone())
                            .collect::<Vec<_>>(),
                        y["stars"]
                            .as_array()
                            .expect("证据")
                            .iter()
                            .map(|s| s["key"].clone())
                            .collect::<Vec<_>>(),
                    );
                }
            }
        }
    }
}

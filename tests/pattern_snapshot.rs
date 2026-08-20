//! 格局 DTO 的三侧共用快照：Rust 写基线，Python 与 Go 读同一批文件比对。
//!
//! 每份快照是一张盘在本命、大限、流年、流月、流日、流时六层的 `patterns_dto` 输出，
//! 连同排盘入参一起落盘，使另外两侧不必重复描述用例——读文件里的 `params` 重新排盘
//! 再比对即可（`python/tests/test_patterns.py` 与 `go/iztro/pattern_golden_test.go`）。
//! 覆盖 4 张盘 × 6 种语言 = 24 份：含男女、一张中州派排盘、一张位置法亮度口径。
//!
//! 快照位于 tests/golden/pattern_snapshots/*.json。文件缺失或内容不一致都是失败；
//! 只有显式设 `UPDATE_PATTERN_SNAPSHOTS=1` 跑本测试才写入当前输出作为基线
//! （全部文件覆盖重写，三侧的比对文件随之更新）。目录里的文件集必须恰为
//! 用例 × 语言，多余文件同样报错——防止改名或删用例后留下孤儿快照。

use std::collections::BTreeSet;
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

/// 重建基线的显式开关：设为 `1` 时本测试覆盖写全部快照而非比对。
const UPDATE_ENV: &str = "UPDATE_PATTERN_SNAPSHOTS";

/// 运限目标：晚于全部用例生日的固定日期与时辰。
const TARGET_DATE: &str = "2026-8-19";
const TARGET_TIME_INDEX: u8 = 3;

/// 快照里的六个视角层，键名即 `Scope::as_key`。
const LAYERS: [(&str, Scope); 6] = [
    ("origin", Scope::Origin),
    ("decadal", Scope::Decadal),
    ("yearly", Scope::Yearly),
    ("monthly", Scope::Monthly),
    ("daily", Scope::Daily),
    ("hourly", Scope::Hourly),
];

/// 六种输出语言，每张盘各出一份快照。
const LANGUAGES: [Language; 6] = [
    Language::ZhCN,
    Language::ZhTW,
    Language::EnUS,
    Language::JaJP,
    Language::KoKR,
    Language::ViVN,
];

fn update_mode() -> bool {
    std::env::var_os(UPDATE_ENV).is_some_and(|v| v == "1")
}

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

/// 一份快照的完整内容：入参 + 六层命中。
fn snapshot_value(case: &Case, language: Language) -> Value {
    let astrolabe = chart(case, language);
    let horoscope = astrolabe
        .horoscope(TARGET_DATE, TARGET_TIME_INDEX)
        .expect("运限目标合法");
    let pc = &case.pattern_config;
    let mut v = json!({
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
    });
    for (key, scope) in LAYERS.into_iter().filter(|(_, s)| *s != Scope::Origin) {
        v[key] = serde_json::to_value(horoscope.data().patterns_dto(&astrolabe, scope, pc))
            .expect("命中可序列化");
    }
    v
}

/// 与已有快照逐字节比对；`UPDATE_PATTERN_SNAPSHOTS=1` 时改为覆盖写入基线。
fn assert_snapshot(name: &str, actual: &Value) {
    let dir = Path::new(SNAPSHOT_DIR);
    let path: PathBuf = dir.join(format!("{name}.json"));
    let actual = format!(
        "{}\n",
        serde_json::to_string_pretty(actual).expect("快照可序列化")
    );

    if update_mode() {
        fs::create_dir_all(dir).expect("建立快照目录");
        fs::write(&path, &actual).expect("写入快照基线");
        eprintln!("pattern snapshot baseline written: {}", path.display());
        return;
    }

    assert!(
        path.exists(),
        "快照缺失：{}。基线只能显式重建：{UPDATE_ENV}=1 cargo test --test pattern_snapshot",
        path.display(),
    );
    let expected = fs::read_to_string(&path).expect("读取快照");
    assert_eq!(
        actual,
        expected,
        "\n\n格局输出与快照 {} 不一致。若是有意改动，用 {UPDATE_ENV}=1 重跑本测试重建基线（三侧读同一批文件）。\n",
        path.display(),
    );
}

/// 用例 × 语言展开出的全部快照文件名。
fn expected_files() -> BTreeSet<String> {
    cases()
        .iter()
        .flat_map(|case| {
            LANGUAGES
                .iter()
                .map(|l| format!("{}_{}.json", case.name, l.as_code()))
        })
        .collect()
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

    // 目录文件集必须恰为用例 × 语言：缺失在上面逐份报，孤儿文件在这里报。
    let actual: BTreeSet<String> = fs::read_dir(SNAPSHOT_DIR)
        .expect("快照目录存在")
        .map(|e| {
            e.expect("读目录项")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        actual,
        expected_files(),
        "快照目录文件集与用例 × 语言不一致：多余的手工删除，缺失的用 {UPDATE_ENV}=1 重建",
    );
}

/// hourly 层必须对 targetTimeIndex 敏感：至少一张用例盘在两个目标时辰下流时层不同。
/// 这保证绑定层若把 targetTimeIndex 接断（恒取 0），快照比对有差异可红。
#[test]
fn hourly_layer_depends_on_target_time_index() {
    let differs = cases().iter().any(|case| {
        let astrolabe = chart(case, Language::ZhCN);
        let pc = &case.pattern_config;
        let at = |t: u8| {
            let h = astrolabe.horoscope(TARGET_DATE, t).expect("运限目标合法");
            serde_json::to_value(h.data().patterns_dto(&astrolabe, Scope::Hourly, pc)).unwrap()
        };
        at(TARGET_TIME_INDEX) != at(0)
    });
    assert!(
        differs,
        "全部用例盘的流时层在目标时辰 {TARGET_TIME_INDEX} 与 0 下相同：\
         该层没把 targetTimeIndex 变成承重入参，应换目标日期或时辰"
    );
}

/// 快照本身的自洽：语言只改译文，不改 key、宫位、证据与视角。
#[test]
fn snapshots_differ_only_by_translation() {
    for case in cases() {
        let base = snapshot_value(&case, Language::ZhCN);
        for language in LANGUAGES.into_iter().filter(|l| *l != Language::ZhCN) {
            let other = snapshot_value(&case, language);
            for (layer, _) in LAYERS {
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

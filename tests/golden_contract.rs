//! Golden contract 测试：绑定层 DTO JSON 与 JS iztro 的 JSON 输出逐键逐值对照。
//!
//! 数据由 tests/golden/generate_contract.mjs 生成（多语言、双算法、晚子时、
//! 闰月命盘及各自的运限对象）。Rust 侧 `to_dto()` 序列化后与 JS 对象做深度
//! 对比：JS 的每个键值必须一致；Rust 侧仅允许在声明的路径上多出声明的扩展键
//! （排盘上下文与语言无关标识，见 extension_keys）。

mod common;

use serde_json::Value;
use std::fs;
use x_iztro::data::types::*;
use x_iztro::{by_solar, get_horoscope};

const DATA_PATH: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/golden/contract_data.json"
);

/// DTO 相对 JS 输出允许多出的扩展键，按其所在对象的路径限定。
///
/// `path` 为规范化 DTO 路径：根为 `astrolabe` / `horoscope`，数组下标一律
/// 记作 `[]`。扩展键分两类——排盘上下文（genderKey/timeIndex/fixLeap/
/// language/config）只挂在命盘根上，语言无关标识（各 `*key`/`*Key(s)`）
/// 只挂在其对应的宿主对象上。路径不匹配的多余键即为契约偏离。
fn extension_keys(path: &str) -> &'static [&'static str] {
    /// 星耀对象（主星/辅星/杂耀/流耀）的标识字段
    const STAR: &[&str] = &["key", "brightnessKey", "mutagenKey"];
    /// 运限层级对象的标识字段
    const SCOPE: &[&str] = &[
        "heavenlyStemKey",
        "earthlyBranchKey",
        "palaceNameKeys",
        "mutagenKeys",
    ];

    match path {
        "astrolabe" => &[
            "genderKey",
            "timeIndex",
            "fixLeap",
            "language",
            "config",
            "earthlyBranchOfSoulPalaceKey",
            "earthlyBranchOfBodyPalaceKey",
            "soulKey",
            "bodyKey",
            "fiveElementsClassKey",
        ],
        "astrolabe.rawDates.chineseDate" => {
            &["yearlyKeys", "monthlyKeys", "dailyKeys", "hourlyKeys"]
        }
        "astrolabe.palaces[]" => &[
            "nameKey",
            "heavenlyStemKey",
            "earthlyBranchKey",
            "changsheng12Key",
            "boshi12Key",
            "jiangqian12Key",
            "suiqian12Key",
            "mutagenStarKeys",
        ],
        "astrolabe.palaces[].decadal" => &["heavenlyStemKey", "earthlyBranchKey"],
        "astrolabe.palaces[].majorStars[]"
        | "astrolabe.palaces[].minorStars[]"
        | "astrolabe.palaces[].adjectiveStars[]" => STAR,
        "horoscope.decadal" | "horoscope.age" | "horoscope.yearly" | "horoscope.monthly"
        | "horoscope.daily" | "horoscope.hourly" => SCOPE,
        "horoscope.decadal.stars[][]"
        | "horoscope.age.stars[][]"
        | "horoscope.yearly.stars[][]"
        | "horoscope.monthly.stars[][]"
        | "horoscope.daily.stars[][]"
        | "horoscope.hourly.stars[][]" => STAR,
        "horoscope.yearly.yearlyDecStar" => &["suiqian12Keys", "jiangqian12Keys"],
        _ => &[],
    }
}

fn parse_lang(s: &str) -> Language {
    match s {
        "zh-CN" => Language::ZhCN,
        "zh-TW" => Language::ZhTW,
        "en-US" => Language::EnUS,
        "ja-JP" => Language::JaJP,
        "ko-KR" => Language::KoKR,
        "vi-VN" => Language::ViVN,
        _ => panic!("Unknown language: {s}"),
    }
}

/// 深度对比：JS 值的每个键/元素在 Rust 值中必须存在且相等；
/// 对象键集必须一致，Rust 侧仅允许在 extension_keys 声明的路径上多出声明的键。
///
/// `path` 为规范化路径（数组下标记作 `[]`），既用于扩展键查表也用于失败定位；
/// `at` 是带下标的具体位置，只出现在失败信息里。
fn deep_compare(path: &str, at: &str, js: &Value, rust: &Value, failures: &mut Vec<String>) {
    match (js, rust) {
        (Value::Object(jm), Value::Object(rm)) => {
            for (k, jv) in jm {
                match rm.get(k) {
                    Some(rv) => deep_compare(
                        &format!("{path}.{k}"),
                        &format!("{at}.{k}"),
                        jv,
                        rv,
                        failures,
                    ),
                    None => failures.push(format!("{at}.{k}: missing in rust output")),
                }
            }
            for k in rm.keys() {
                if !jm.contains_key(k) && !extension_keys(path).contains(&k.as_str()) {
                    failures.push(format!(
                        "{at}.{k}: unexpected extra key in rust output (path {path})"
                    ));
                }
            }
        }
        (Value::Array(ja), Value::Array(ra)) => {
            if ja.len() != ra.len() {
                failures.push(format!("{at}: array len js={} rust={}", ja.len(), ra.len()));
                return;
            }
            for (i, (jv, rv)) in ja.iter().zip(ra.iter()).enumerate() {
                deep_compare(
                    &format!("{path}[]"),
                    &format!("{at}[{i}]"),
                    jv,
                    rv,
                    failures,
                );
            }
        }
        _ => {
            if js != rust {
                failures.push(format!("{at}: js={js} rust={rust}"));
            }
        }
    }
}

#[test]
fn golden_contract_json() {
    let data = fs::read_to_string(DATA_PATH)
        .expect("contract_data.json missing — run `node tests/golden/generate_contract.mjs`");
    let cases: Vec<Value> = serde_json::from_str(&data).unwrap();

    let mut failures: Vec<String> = Vec::new();

    for case in &cases {
        let p = &case["p"];
        let date = p["d"].as_str().unwrap();
        let ti = p["t"].as_u64().unwrap() as u8;
        let gender = if p["g"].as_u64().unwrap() == 0 {
            Gender::Male
        } else {
            Gender::Female
        };
        let lang = parse_lang(p["lang"].as_str().unwrap());
        let algorithm = match p["algorithm"].as_str().unwrap() {
            "zhongzhou" => Algorithm::Zhongzhou,
            _ => Algorithm::Default,
        };
        let label = format!(
            "[{date} t{ti} {} {}]",
            p["lang"].as_str().unwrap(),
            p["algorithm"].as_str().unwrap()
        );

        let astrolabe = by_solar(
            date,
            ti,
            gender,
            true,
            lang,
            Config {
                algorithm,
                ..Config::default()
            },
        )
        .unwrap();
        let rust_astrolabe = serde_json::to_value(astrolabe.to_dto()).unwrap();
        deep_compare(
            "astrolabe",
            &format!("{label} astrolabe"),
            &case["astrolabe"],
            &rust_astrolabe,
            &mut failures,
        );

        let target_date = p["td"].as_str().unwrap();
        let horoscope = get_horoscope(&astrolabe, target_date, 3, lang).unwrap();
        let rust_horoscope = serde_json::to_value(horoscope.to_dto(lang)).unwrap();
        deep_compare(
            "horoscope",
            &format!("{label} horoscope"),
            &case["horoscope"],
            &rust_horoscope,
            &mut failures,
        );

        if failures.len() >= 40 {
            break;
        }
    }

    assert!(
        failures.is_empty(),
        "\n\nGolden contract FAILED: {} difference(s):\n  {}\n",
        failures.len(),
        failures.join("\n  "),
    );
    eprintln!(
        "Golden contract: all {} cases match the JS JSON!",
        cases.len()
    );
}

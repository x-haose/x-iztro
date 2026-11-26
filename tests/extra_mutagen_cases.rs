//! 附加四化星的四侧共用用例 `tests/data/extra_mutagen_cases.json`：逐宫核对 DTO 的四槽、各类四化之星、
//! 四槽落宫与自化。期望缺失或与当前输出不同都是失败；有意改动后用 `UPDATE_EXTRA_MUTAGEN_CASES=1` 重跑写入。

use serde_json::{Value, json};
use x_iztro::data::stars::MUTAGEN;
use x_iztro::{Config, Gender, Language, by_solar};

const CASES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/data/extra_mutagen_cases.json"
);

#[test]
fn shared_extra_mutagen_cases() {
    let mut doc: Value = serde_json::from_str(&std::fs::read_to_string(CASES).unwrap()).unwrap();
    let update = std::env::var_os("UPDATE_EXTRA_MUTAGEN_CASES").is_some();
    for case in doc["cases"].as_array_mut().unwrap() {
        let c = &case["chart"];
        let config: Config = serde_json::from_value(case["config"].clone()).unwrap();
        let gender = match c["gender"].as_str().unwrap() {
            "male" => Gender::Male,
            _ => Gender::Female,
        };
        let chart = by_solar(
            c["solarDate"].as_str().unwrap(),
            c["timeIndex"].as_u64().unwrap() as u8,
            gender,
            true,
            Language::ZhCN,
            config,
        )
        .unwrap();
        let dto = serde_json::to_value(chart.to_dto()).unwrap();
        let palaces: Vec<Value> = chart
            .palaces
            .iter()
            .map(|p| {
                let table = p.stem_mutagens();
                let primary: Vec<&str> = table.primary.iter().map(|s| s.as_key()).collect();
                assert_eq!(
                    dto["palaces"][p.index]["mutagenStarKeys"],
                    json!(primary),
                    "{}: DTO 四槽即四化表的四槽",
                    case["name"]
                );
                let stars: serde_json::Map<String, Value> = MUTAGEN
                    .iter()
                    .map(|m| {
                        let keys: Vec<&str> =
                            p.mutagen_stars(&[*m]).iter().map(|s| s.as_key()).collect();
                        (m.as_key().to_string(), json!(keys))
                    })
                    .collect();
                let selfs: serde_json::Map<String, Value> = MUTAGEN
                    .iter()
                    .map(|m| (m.as_key().to_string(), json!(p.self_mutaged(&[*m]))))
                    .collect();
                json!({
                    "index": p.index,
                    "mutagenStarKeys": primary,
                    "mutagenStars": stars,
                    "mutagedPlaces": p.mutaged_places(&chart.palaces),
                    "selfMutaged": selfs,
                })
            })
            .collect();
        if update {
            case["palaces"] = json!(palaces);
        } else {
            assert_eq!(json!(palaces), case["palaces"], "{}", case["name"]);
        }
    }
    if update {
        std::fs::write(CASES, serde_json::to_string_pretty(&doc).unwrap() + "\n").unwrap();
    }
}

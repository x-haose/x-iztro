//! Golden tier-3 self-check: hash verification of the full 60-year parameter space (~285K cases).
//!
//! First run: generates all hashes and saves to `tests/golden/selfcheck_baseline.csv`
//! Subsequent runs: generates hashes and compares against the saved baseline.
//! If any hash changes, the test fails (regression detected).
//!
//! Marked `#[ignore]` — only runs manually via `cargo test --test golden_tier3 -- --ignored`

use rs_iztro::data::types::*;
use rs_iztro::by_solar;
use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

const BASELINE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/selfcheck_baseline.csv");

fn hash_astrolabe(astrolabe: &rs_iztro::Astrolabe) -> String {
    let json = serde_json::to_string(astrolabe).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    hex::encode(hasher.finalize())
}

fn days_in_month(year: i32, month: u32) -> u32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    }
}

#[test]
#[ignore]
fn test_tier3_selfcheck() {
    let lang = Language::ZhCN;
    let baseline_exists = Path::new(BASELINE_PATH).exists();

    let mut baseline: HashMap<String, String> = HashMap::new();

    if baseline_exists {
        let content = fs::read_to_string(BASELINE_PATH).unwrap();
        for line in content.lines().skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() == 3 {
                let key = format!("{},{}", fields[0], fields[1]);
                baseline.insert(key, fields[2].to_string());
            }
        }
        eprintln!("Loaded {} baseline hashes", baseline.len());
    } else {
        eprintln!(
            "No baseline found. Generating baseline to {}",
            BASELINE_PATH
        );
    }

    let mut output = String::from("solar_date,time_index,hash\n");
    let mut total = 0usize;
    let mut mismatches = 0usize;
    let mut first_mismatches: Vec<String> = Vec::new();

    for year in 1984..=2043i32 {
        for month in 1..=12u32 {
            let max_day = days_in_month(year, month);
            for day in 1..=max_day {
                let solar_date = format!("{}-{}-{}", year, month, day);
                for t in 0..=12u8 {
                    let astrolabe = by_solar(
                        &solar_date,
                        t,
                        Gender::Male,
                        true,
                        lang,
                        Algorithm::Default,
                    );
                    let hash = hash_astrolabe(&astrolabe);

                    let key = format!("{},{}", solar_date, t);
                    if baseline_exists
                        && let Some(expected) = baseline.get(&key)
                            && *expected != hash {
                                mismatches += 1;
                                if first_mismatches.len() < 20 {
                                    first_mismatches.push(format!(
                                        "{}: expected {}..., got {}...",
                                        key,
                                        &expected[..16.min(expected.len())],
                                        &hash[..16.min(hash.len())]
                                    ));
                                }
                            }

                    output.push_str(&format!("{},{},{}\n", solar_date, t, hash));
                    total += 1;
                }
            }
        }
        eprint!("\r  Year {}/2043 ({} cases)", year, total);
    }

    // Always write the new baseline (or first baseline)
    if !baseline_exists {
        fs::write(BASELINE_PATH, &output).unwrap();
        eprintln!(
            "\nBaseline generated: {} hashes -> {}",
            total, BASELINE_PATH
        );
    } else if mismatches > 0 {
        // Write updated file for comparison
        let new_path = format!("{}.new", BASELINE_PATH);
        fs::write(&new_path, &output).unwrap();
        panic!(
            "\n{} hash mismatches out of {} cases! New hashes written to {}\nFirst mismatches:\n{}",
            mismatches,
            total,
            new_path,
            first_mismatches.join("\n")
        );
    } else {
        eprintln!("\nAll {} hashes match baseline.", total);
    }
}

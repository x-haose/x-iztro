//! Golden tier-2 tests: compare x-iztro output against ~37,440 compact JS-generated cases.
//!
//! Each year file contains ~624 cases in compact JSON format.
//! Fields compared: soul/body branches, five elements class, soul/body stars,
//! palace names, major star names (as sets), minor star names (as sets), decadal ranges.
//! All cases (including time_index=12) must match the JS output exactly.

use x_iztro::by_solar;
use x_iztro::data::types::*;
use x_iztro::i18n::{
    translate_earthly_branch, translate_five_elements_class, translate_palace, translate_star,
};
use serde::Deserialize;
use std::collections::BTreeSet;

const TIER2_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier2");
const LANG: Language = Language::ZhCN;
const MAX_FAILURES: usize = 100;

#[derive(Deserialize)]
struct Tier2Case {
    d: String,            // solar_date e.g. "1984-2-1"
    t: u8,                // time_index (0-12)
    g: u8,                // gender: 0=男 1=女
    sb: String,           // earthlyBranchOfSoulPalace
    bb: String,           // earthlyBranchOfBodyPalace
    fc: String,           // fiveElementsClass
    ss: String,           // soul star
    bs: String,           // body star
    pn: Vec<String>,      // 12 palace names
    ms: Vec<Vec<String>>, // 12 arrays of major star names
    ns: Vec<Vec<String>>, // 12 arrays of minor star names
    dr: Vec<[i64; 2]>,    // 12 decadal ranges [start, end]
}

#[test]
fn golden_tier2_compact() {
    let mut entries: Vec<_> = std::fs::read_dir(TIER2_DIR)
        .expect("Failed to read tier2 directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name();
            let name = name.to_string_lossy();
            name.starts_with("year_") && name.ends_with(".json")
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    assert!(
        !entries.is_empty(),
        "No tier2 year files found in {}",
        TIER2_DIR
    );

    let mut failures: Vec<String> = Vec::new();
    let mut total_cases = 0usize;
    let mut files_processed = 0usize;

    'outer: for entry in &entries {
        let path = entry.path();
        let file_name = path.file_name().unwrap().to_string_lossy().to_string();
        let data = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", path.display(), e));
        let cases: Vec<Tier2Case> = serde_json::from_str(&data)
            .unwrap_or_else(|e| panic!("Failed to parse {}: {}", path.display(), e));

        files_processed += 1;
        total_cases += cases.len();

        for (ci, case) in cases.iter().enumerate() {
            let gender = if case.g == 0 {
                Gender::Male
            } else {
                Gender::Female
            };
            let case_label = format!(
                "{} [{}] d={} t={} g={}",
                file_name, ci, case.d, case.t, case.g
            );

            let astrolabe =
                by_solar(&case.d, case.t, gender, true, LANG, Config::default()).unwrap();

            // Soul branch
            let act_sb =
                translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, LANG).to_string();
            if act_sb != case.sb {
                failures.push(format!(
                    "{}: sb expected={} actual={}",
                    case_label, case.sb, act_sb
                ));
            }

            // Body branch
            let act_bb =
                translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, LANG).to_string();
            if act_bb != case.bb {
                failures.push(format!(
                    "{}: bb expected={} actual={}",
                    case_label, case.bb, act_bb
                ));
            }

            // Five elements class
            let act_fc =
                translate_five_elements_class(astrolabe.five_elements_class, LANG).to_string();
            if act_fc != case.fc {
                failures.push(format!(
                    "{}: fc expected={} actual={}",
                    case_label, case.fc, act_fc
                ));
            }

            // Soul star
            let act_ss = translate_star(astrolabe.soul, LANG).to_string();
            if act_ss != case.ss {
                failures.push(format!(
                    "{}: ss expected={} actual={}",
                    case_label, case.ss, act_ss
                ));
            }

            // Body star
            let act_bs = translate_star(astrolabe.body, LANG).to_string();
            if act_bs != case.bs {
                failures.push(format!(
                    "{}: bs expected={} actual={}",
                    case_label, case.bs, act_bs
                ));
            }

            // Palaces
            if astrolabe.palaces.len() != 12 {
                failures.push(format!(
                    "{}: palace count={}",
                    case_label,
                    astrolabe.palaces.len()
                ));
                if failures.len() >= MAX_FAILURES {
                    break 'outer;
                }
                continue;
            }

            for pi in 0..12 {
                let act_palace = &astrolabe.palaces[pi];

                // Palace name
                let act_name = translate_palace(act_palace.name, LANG);
                let exp_name = &case.pn[pi];
                if act_name != exp_name {
                    failures.push(format!(
                        "{} p[{}]: name expected={} actual={}",
                        case_label, pi, exp_name, act_name
                    ));
                }

                // Major stars (as sets to handle order differences)
                let act_majors: BTreeSet<String> = act_palace
                    .major_stars
                    .iter()
                    .map(|s| translate_star(s.key, LANG).to_string())
                    .collect();
                let exp_majors: BTreeSet<String> = case.ms[pi].iter().cloned().collect();
                if act_majors != exp_majors {
                    failures.push(format!(
                        "{} p[{}]: majors expected={:?} actual={:?}",
                        case_label, pi, exp_majors, act_majors
                    ));
                }

                // Minor stars (as sets)
                let act_minors: BTreeSet<String> = act_palace
                    .minor_stars
                    .iter()
                    .map(|s| translate_star(s.key, LANG).to_string())
                    .collect();
                let exp_minors: BTreeSet<String> = case.ns[pi].iter().cloned().collect();
                if act_minors != exp_minors {
                    failures.push(format!(
                        "{} p[{}]: minors expected={:?} actual={:?}",
                        case_label, pi, exp_minors, act_minors
                    ));
                }

                // Decadal range
                let exp_range = (case.dr[pi][0] as u32, case.dr[pi][1] as u32);
                let act_range = act_palace.decadal.range;
                if act_range != exp_range {
                    failures.push(format!(
                        "{} p[{}]: decadal expected={:?} actual={:?}",
                        case_label, pi, exp_range, act_range
                    ));
                }
            }

            if failures.len() >= MAX_FAILURES {
                break 'outer;
            }
        }
    }

    if !failures.is_empty() {
        let shown = failures.len().min(MAX_FAILURES);
        let mut msg = format!(
            "\n\nGolden tier-2 FAILED: {} failure(s) (showing first {}):\n\n",
            failures.len(),
            shown,
        );
        for (i, f) in failures.iter().take(shown).enumerate() {
            msg.push_str(&format!("  {}. {}\n", i + 1, f));
        }
        panic!("{}", msg);
    }

    eprintln!(
        "Golden tier-2: all {} cases in {} files passed!",
        total_cases, files_processed
    );
}

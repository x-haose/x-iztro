//! Prompt 输出的快照测试：固定命盘在 zh-CN 与 en-US 下的完整文本逐字节比对。
//!
//! 三侧的 Prompt 测试只做 `contains` 断言，措辞、字段顺序、缺失段落的改动
//! 都察觉不到。此处把整份输出落成快照文件，任何一处变化都会显式暴露，
//! 由人判断是有意改动（更新快照）还是回归。
//!
//! 快照位于 tests/golden/prompt_snapshots/*.txt。文件缺失或内容不一致都是失败；
//! 只有显式设 `UPDATE_PROMPT_SNAPSHOTS=1` 跑本测试才写入当前输出作为基线。

use std::fs;
use std::path::{Path, PathBuf};
use x_iztro::data::types::*;
use x_iztro::prompt::{astrolabe_to_prompt, horoscope_to_prompt};
use x_iztro::{by_solar, get_horoscope};

const SNAPSHOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/prompt_snapshots");

/// 重建基线的显式开关：设为 `1` 时本测试覆盖写快照而非比对。
const UPDATE_ENV: &str = "UPDATE_PROMPT_SNAPSHOTS";

fn update_mode() -> bool {
    std::env::var_os(UPDATE_ENV).is_some_and(|v| v == "1")
}

/// 固定盘：2000-8-16 时辰 2 女命；运限目标 2025-1-1 时辰 0。
const BIRTH_DATE: &str = "2000-8-16";
const BIRTH_TIME_INDEX: u8 = 2;
const TARGET_DATE: &str = "2025-1-1";
const TARGET_TIME_INDEX: u8 = 0;

/// 与已有快照逐字节比对；`UPDATE_PROMPT_SNAPSHOTS=1` 时改为覆盖写入基线。
fn assert_snapshot(name: &str, actual: &str) {
    let dir = Path::new(SNAPSHOT_DIR);
    let path: PathBuf = dir.join(format!("{name}.txt"));

    if update_mode() {
        fs::create_dir_all(dir).expect("create snapshot dir");
        fs::write(&path, actual).expect("write snapshot baseline");
        eprintln!("prompt snapshot baseline written: {}", path.display());
        return;
    }

    assert!(
        path.exists(),
        "快照缺失：{}。基线只能显式重建：{UPDATE_ENV}=1 cargo test --test prompt_snapshot",
        path.display(),
    );
    let expected = fs::read_to_string(&path).expect("read snapshot");
    assert_eq!(
        actual,
        expected,
        "\n\nPrompt 输出与快照 {} 不一致。若是有意改动，用 {UPDATE_ENV}=1 重跑本测试重建基线。\n",
        path.display(),
    );
}

/// 快照目录的文件集必须恰为两类输出 × 两种语言，孤儿文件同样报错。
#[test]
fn snapshot_dir_matches_expected_set() {
    let expected: std::collections::BTreeSet<String> = ["astrolabe", "horoscope"]
        .iter()
        .flat_map(|kind| {
            ["zh-CN", "en-US"]
                .iter()
                .map(move |tag| format!("{kind}_{tag}.txt"))
        })
        .collect();
    let actual: std::collections::BTreeSet<String> = fs::read_dir(SNAPSHOT_DIR)
        .expect("快照目录存在")
        .map(|e| {
            e.expect("读目录项")
                .file_name()
                .to_string_lossy()
                .into_owned()
        })
        .collect();
    assert_eq!(
        actual, expected,
        "快照目录文件集与用例不一致：多余的手工删除，缺失的用 {UPDATE_ENV}=1 重建",
    );
}

fn chart(lang: Language) -> x_iztro::Astrolabe {
    by_solar(
        BIRTH_DATE,
        BIRTH_TIME_INDEX,
        Gender::Female,
        true,
        lang,
        Config::default(),
    )
    .unwrap()
}

#[test]
fn astrolabe_prompt_matches_snapshot() {
    for (lang, tag) in [(Language::ZhCN, "zh-CN"), (Language::EnUS, "en-US")] {
        let astrolabe = chart(lang);
        assert_snapshot(
            &format!("astrolabe_{tag}"),
            &astrolabe_to_prompt(&astrolabe, lang),
        );
    }
}

#[test]
fn horoscope_prompt_matches_snapshot() {
    for (lang, tag) in [(Language::ZhCN, "zh-CN"), (Language::EnUS, "en-US")] {
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, TARGET_DATE, TARGET_TIME_INDEX, lang).unwrap();
        assert_snapshot(
            &format!("horoscope_{tag}"),
            &horoscope_to_prompt(&astrolabe, &horoscope, lang),
        );
    }
}

//! Prompt 输出的快照测试：固定命盘在 zh-CN 与 en-US 下的完整文本逐字节比对。
//!
//! 三侧的 Prompt 测试只做 `contains` 断言，措辞、字段顺序、缺失段落的改动
//! 都察觉不到。此处把整份输出落成快照文件，任何一处变化都会显式暴露，
//! 由人判断是有意改动（更新快照）还是回归。
//!
//! 快照位于 tests/golden/prompt_snapshots/*.txt。文件缺失时本测试会写入当前
//! 输出并通过——即建立基线；已存在则严格比对。要在有意改动 Prompt 后重建
//! 基线，删除对应快照文件再跑一次本测试。

use std::fs;
use std::path::{Path, PathBuf};
use x_iztro::data::types::*;
use x_iztro::prompt::{astrolabe_to_prompt, horoscope_to_prompt};
use x_iztro::{by_solar, get_horoscope};

const SNAPSHOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/prompt_snapshots");

/// 固定盘：2000-8-16 时辰 2 女命；运限目标 2025-1-1 时辰 0。
const BIRTH_DATE: &str = "2000-8-16";
const BIRTH_TIME_INDEX: u8 = 2;
const TARGET_DATE: &str = "2025-1-1";
const TARGET_TIME_INDEX: u8 = 0;

/// 与已有快照比对；快照不存在则写入当前输出作为基线。
fn assert_snapshot(name: &str, actual: &str) {
    let dir = Path::new(SNAPSHOT_DIR);
    fs::create_dir_all(dir).expect("create snapshot dir");
    let path: PathBuf = dir.join(format!("{name}.txt"));

    if !path.exists() {
        fs::write(&path, actual).expect("write snapshot baseline");
        eprintln!("prompt snapshot baseline written: {}", path.display());
        return;
    }

    let expected = fs::read_to_string(&path).expect("read snapshot");
    assert_eq!(
        actual,
        expected,
        "\n\nPrompt 输出与快照 {} 不一致。若是有意改动，删除该文件后重跑本测试重建基线。\n",
        path.display(),
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

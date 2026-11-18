//! 语义化文本（to_text）的快照测试：固定命盘在 zh-CN 与 en-US 下的完整输出逐字节比对。
//!
//! 三侧的文本测试只做 `contains` 断言，措辞、字段顺序、缺失段落的改动
//! 都察觉不到。此处把整份输出落成快照文件，任何一处变化都会显式暴露，
//! 由人判断是有意改动（更新快照）还是回归。
//!
//! 快照位于 tests/golden/text_snapshots/*.txt。文件缺失或内容不一致都是失败；
//! 只有显式设 `UPDATE_TEXT_SNAPSHOTS=1` 跑本测试才写入当前输出作为基线。
//!
//! 命名：`{类}_{语言}` 为完整事实；`{类}_overview_{语言}` 为总览层；`{类}_knowledge_zh-CN`
//! 与 `{类}_summary_zh-CN` 为完整事实配内嵌包的全文与摘要层释义。主题文本（`topic`，取婚恋主题）
//! 必须带知识包，只有后两种。

use std::fs;
use std::path::{Path, PathBuf};
use x_iztro::data::types::*;
use x_iztro::text::{
    astrolabe_to_text, astrolabe_to_text_with, decadal_list_to_text, decadal_list_to_text_with,
    flanking_palaces_to_text, flanking_palaces_to_text_with, horoscope_to_text,
    horoscope_to_text_with, monthly_list_to_text, monthly_list_to_text_with, palace_to_text,
    palace_to_text_with, patterns_to_text, patterns_to_text_with, surrounded_palaces_to_text,
    surrounded_palaces_to_text_with, yearly_overview_to_text, yearly_overview_to_text_with,
};
use x_iztro::{KnowledgePack, TextDetail, TextOptions, by_solar, get_horoscope};

const SNAPSHOT_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/text_snapshots");

/// 重建基线的显式开关：设为 `1` 时本测试覆盖写快照而非比对。
const UPDATE_ENV: &str = "UPDATE_TEXT_SNAPSHOTS";

fn update_mode() -> bool {
    std::env::var_os(UPDATE_ENV).is_some_and(|v| v == "1")
}

/// 固定盘：2000-8-16 时辰 2 女命；运限目标 2025-1-1 时辰 0。
const BIRTH_DATE: &str = "2000-8-16";
const BIRTH_TIME_INDEX: u8 = 2;
const TARGET_DATE: &str = "2025-1-1";
const TARGET_TIME_INDEX: u8 = 0;
/// 流年一览取的农历年区间：前两年未起运（童限），其后跨两个大限
const YEARLY_RANGE: (i64, i64) = (2001, 2012);
/// 流月一览取的农历年：2025 年有闰六月，覆盖闰月拆成前后半月的两行
const MONTHLY_YEAR: i64 = 2025;
/// 主题文本取的主题：夫妻宫连三方四正，另有命宫、身宫与子女宫，覆盖两组宫位与去重
const TOPIC: &str = "romance";

/// 与已有快照逐字节比对；`UPDATE_TEXT_SNAPSHOTS=1` 时改为覆盖写入基线。
fn assert_snapshot(name: &str, actual: &str) {
    let dir = Path::new(SNAPSHOT_DIR);
    let path: PathBuf = dir.join(format!("{name}.txt"));

    if update_mode() {
        fs::create_dir_all(dir).expect("create snapshot dir");
        fs::write(&path, actual).expect("write snapshot baseline");
        eprintln!("text snapshot baseline written: {}", path.display());
        return;
    }

    assert!(
        path.exists(),
        "快照缺失：{}。基线只能显式重建：{UPDATE_ENV}=1 cargo test --test text_snapshot",
        path.display(),
    );
    let expected = fs::read_to_string(&path).expect("read snapshot");
    assert_eq!(
        actual,
        expected,
        "\n\n文本输出与快照 {} 不一致。若是有意改动，用 {UPDATE_ENV}=1 重跑本测试重建基线。\n",
        path.display(),
    );
}

/// 九类输出的名称；带释义形态在名称后加 `_knowledge`
const KINDS: [&str; 9] = [
    "astrolabe",
    "horoscope",
    "patterns",
    "palace",
    "surrounded",
    "flanking",
    "decadals",
    "yearlies",
    "monthlies",
];

/// 受事实详略影响的五类输出（格局、大限一览、流年一览与流月一览本身就是总览）
const DETAIL_KINDS: [&str; 5] = ["astrolabe", "horoscope", "palace", "surrounded", "flanking"];

/// 快照目录的文件集必须恰为：九类输出 × 两种语言、五类总览输出 × 两种语言、
/// 九类输出加主题文本的全文释义与摘要层释义 × zh-CN（内嵌知识包只有 zh-CN），孤儿文件同样报错。
#[test]
fn snapshot_dir_matches_expected_set() {
    let expected: std::collections::BTreeSet<String> = ["zh-CN", "en-US"]
        .iter()
        .flat_map(|tag| {
            KINDS
                .iter()
                .map(move |kind| format!("{kind}_{tag}.txt"))
                .chain(
                    DETAIL_KINDS
                        .iter()
                        .map(move |kind| format!("{kind}_overview_{tag}.txt")),
                )
        })
        .chain(KINDS.iter().chain(&["topic"]).flat_map(|kind| {
            ["knowledge", "summary"]
                .iter()
                .map(move |depth| format!("{kind}_{depth}_zh-CN.txt"))
        }))
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

const LANGS: [(Language, &str); 2] = [(Language::ZhCN, "zh-CN"), (Language::EnUS, "en-US")];

#[test]
fn astrolabe_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        assert_snapshot(
            &format!("astrolabe_{tag}"),
            &astrolabe_to_text(&astrolabe, lang),
        );
    }
}

#[test]
fn horoscope_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, TARGET_DATE, TARGET_TIME_INDEX, lang).unwrap();
        assert_snapshot(
            &format!("horoscope_{tag}"),
            &horoscope_to_text(&astrolabe, &horoscope, lang),
        );
    }
}

#[test]
fn patterns_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let hits = astrolabe.patterns();
        let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
        assert_snapshot(
            &format!("patterns_{tag}"),
            &patterns_to_text(&hits, &names, lang),
        );
    }
}

#[test]
fn palace_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let palace = astrolabe.palace(Palace::Soul).unwrap();
        assert_snapshot(&format!("palace_{tag}"), &palace_to_text(&palace, lang));
    }
}

#[test]
fn surrounded_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();
        assert_snapshot(
            &format!("surrounded_{tag}"),
            &surrounded_palaces_to_text(&sp, lang),
        );
    }
}

#[test]
fn flanking_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let f = astrolabe.flanking_palaces(Palace::Soul).unwrap();
        assert_snapshot(
            &format!("flanking_{tag}"),
            &flanking_palaces_to_text(&f, lang),
        );
    }
}

#[test]
fn decadal_list_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        assert_snapshot(
            &format!("decadals_{tag}"),
            &decadal_list_to_text(&astrolabe, lang),
        );
    }
}

#[test]
fn yearly_overview_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        assert_snapshot(
            &format!("yearlies_{tag}"),
            &yearly_overview_to_text(&astrolabe, YEARLY_RANGE.0, YEARLY_RANGE.1, lang).unwrap(),
        );
    }
}

#[test]
fn monthly_list_text_matches_snapshot() {
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        assert_snapshot(
            &format!("monthlies_{tag}"),
            &monthly_list_to_text(&astrolabe, MONTHLY_YEAR, lang).unwrap(),
        );
    }
}

/// 带释义的七类输出：同一固定盘按内嵌 zh-CN 知识包取材，逐字节比对。
#[test]
fn text_with_knowledge_matches_snapshot() {
    let lang = Language::ZhCN;
    let pack = KnowledgePack::builtin(lang).unwrap();
    let opts = TextOptions::new().knowledge(pack);
    let astrolabe = chart(lang);
    let horoscope = get_horoscope(&astrolabe, TARGET_DATE, TARGET_TIME_INDEX, lang).unwrap();
    let hits = astrolabe.patterns();
    let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
    let palace = astrolabe.palace(Palace::Soul).unwrap();
    let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();

    assert_snapshot(
        "astrolabe_knowledge_zh-CN",
        &astrolabe_to_text_with(&astrolabe, &opts, lang),
    );
    assert_snapshot(
        "horoscope_knowledge_zh-CN",
        &horoscope_to_text_with(&astrolabe, &horoscope, &opts, lang),
    );
    assert_snapshot(
        "patterns_knowledge_zh-CN",
        &patterns_to_text_with(&hits, &names, &opts, lang),
    );
    assert_snapshot(
        "palace_knowledge_zh-CN",
        &palace_to_text_with(&palace, &opts, lang),
    );
    assert_snapshot(
        "surrounded_knowledge_zh-CN",
        &surrounded_palaces_to_text_with(&sp, &opts, lang),
    );
    let f = astrolabe.flanking_palaces(Palace::Soul).unwrap();
    assert_snapshot(
        "flanking_knowledge_zh-CN",
        &flanking_palaces_to_text_with(&f, &opts, lang),
    );
    assert_snapshot(
        "topic_knowledge_zh-CN",
        &astrolabe.topic_to_text(TOPIC, &opts).unwrap(),
    );
    assert_snapshot(
        "decadals_knowledge_zh-CN",
        &decadal_list_to_text_with(&astrolabe, &opts, lang),
    );
    assert_snapshot(
        "yearlies_knowledge_zh-CN",
        &yearly_overview_to_text_with(&astrolabe, YEARLY_RANGE.0, YEARLY_RANGE.1, &opts, lang)
            .unwrap(),
    );
    assert_snapshot(
        "monthlies_knowledge_zh-CN",
        &monthly_list_to_text_with(&astrolabe, MONTHLY_YEAR, &opts, lang).unwrap(),
    );
}

/// 总览层的五类输出 × 两种语言（不带释义）。
#[test]
fn overview_text_matches_snapshot() {
    let opts = TextOptions::new().detail(TextDetail::Overview);
    for (lang, tag) in LANGS {
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, TARGET_DATE, TARGET_TIME_INDEX, lang).unwrap();
        let palace = astrolabe.palace(Palace::Soul).unwrap();
        let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();
        let f = astrolabe.flanking_palaces(Palace::Soul).unwrap();
        for (kind, text) in [
            ("astrolabe", astrolabe_to_text_with(&astrolabe, &opts, lang)),
            (
                "horoscope",
                horoscope_to_text_with(&astrolabe, &horoscope, &opts, lang),
            ),
            ("palace", palace_to_text_with(&palace, &opts, lang)),
            (
                "surrounded",
                surrounded_palaces_to_text_with(&sp, &opts, lang),
            ),
            ("flanking", flanking_palaces_to_text_with(&f, &opts, lang)),
        ] {
            assert_snapshot(&format!("{kind}_overview_{tag}"), &text);
        }
    }
}

/// 摘要层释义的七类输出（完整事实，zh-CN 内嵌包）。
#[test]
fn text_with_summary_knowledge_matches_snapshot() {
    let lang = Language::ZhCN;
    let pack = KnowledgePack::builtin(lang).unwrap();
    let opts = TextOptions::new().knowledge_summary(pack).unwrap();
    let astrolabe = chart(lang);
    let horoscope = get_horoscope(&astrolabe, TARGET_DATE, TARGET_TIME_INDEX, lang).unwrap();
    let hits = astrolabe.patterns();
    let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
    let palace = astrolabe.palace(Palace::Soul).unwrap();
    let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();
    let f = astrolabe.flanking_palaces(Palace::Soul).unwrap();
    for (kind, text) in [
        ("astrolabe", astrolabe_to_text_with(&astrolabe, &opts, lang)),
        (
            "horoscope",
            horoscope_to_text_with(&astrolabe, &horoscope, &opts, lang),
        ),
        (
            "patterns",
            patterns_to_text_with(&hits, &names, &opts, lang),
        ),
        ("palace", palace_to_text_with(&palace, &opts, lang)),
        (
            "surrounded",
            surrounded_palaces_to_text_with(&sp, &opts, lang),
        ),
        ("flanking", flanking_palaces_to_text_with(&f, &opts, lang)),
        (
            "decadals",
            decadal_list_to_text_with(&astrolabe, &opts, lang),
        ),
        (
            "yearlies",
            yearly_overview_to_text_with(&astrolabe, YEARLY_RANGE.0, YEARLY_RANGE.1, &opts, lang)
                .unwrap(),
        ),
        (
            "monthlies",
            monthly_list_to_text_with(&astrolabe, MONTHLY_YEAR, &opts, lang).unwrap(),
        ),
        ("topic", astrolabe.topic_to_text(TOPIC, &opts).unwrap()),
    ] {
        assert_snapshot(&format!("{kind}_summary_zh-CN"), &text);
    }
}

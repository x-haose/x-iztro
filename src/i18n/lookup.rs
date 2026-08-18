//! 标识与译名的双向查找
//!
//! 同层的 `translate_*` 按类目分开收强类型枚举；本模块把十二类标识合成一张表，
//! 收字符串标识出译名（对应 iztro 的 `t`），或收任意语言的译名反查标识
//! （对应 iztro 的 `kot`）。绑定层与不知道标识属于哪一类的调用方走这里。

use std::sync::LazyLock;

use crate::data::constants::{
    CHINESE_TIME, EARTHLY_BRANCHES, FIVE_ELEMENTS_CLASSES, HEAVENLY_STEMS, SIGNS, ZODIAC,
};
use crate::data::stars::{ALL_STARS, MUTAGEN, StarKey};
use crate::data::types::*;

use super::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_horoscope_name, translate_mutagen,
    translate_palace, translate_sign, translate_star, translate_time, translate_zodiac,
};

/// 六种语言的固定顺序，供本模块的字面量表按序取值
const LANGS: [Language; 6] = [
    Language::ZhCN,
    Language::ZhTW,
    Language::EnUS,
    Language::JaJP,
    Language::KoKR,
    Language::ViVN,
];

fn lang_index(lang: Language) -> usize {
    LANGS.iter().position(|l| *l == lang).unwrap_or(0)
}

/// 身宫与来因宫的标识
///
/// 这两个不是十二宫之一，而是落在某一宫上的标记，因此不进 [`Palace`] 枚举，
/// 译名在此单列。取值顺序同 [`LANGS`]。
const MARKER_PALACES: [(&str, [&str; 6]); 2] = [
    (
        "bodyPalace",
        ["身宫", "身宮", "body", "身宮", "신궁", "Thân"],
    ),
    (
        "originalPalace",
        ["来因", "来因", "origin", "来因", "라인", "Lai Nhân"],
    ),
];

/// 性别标识
const GENDERS: [(&str, Gender); 2] = [("male", Gender::Male), ("female", Gender::Female)];

/// 运限层级标识
///
/// `turn` 是 iztro 对小限的标识写法。顺序同 iztro 的 `common.json`，
/// 因为 [`all_keys`] 要照它排。
const HOROSCOPE_NAMES: [(&str, HoroscopeName); 7] = [
    ("decadal", HoroscopeName::Decadal),
    ("childhood", HoroscopeName::Childhood),
    ("yearly", HoroscopeName::Yearly),
    ("monthly", HoroscopeName::Monthly),
    ("daily", HoroscopeName::Daily),
    ("hourly", HoroscopeName::Hourly),
    ("turn", HoroscopeName::Age),
];

/// 十二宫与身宫、来因宫合排的标识顺序
///
/// iztro 的 `palace` 翻译文件把身宫排在命宫之后、来因宫排在末尾，十二宫本身
/// 按顺行排列，与 [`PALACES`] 的逆行布局顺序不同。[`all_keys`] 照这里排，
/// 反查结果才与 iztro 的 `kot` 一致。
const LOOKUP_PALACE_KEYS: [&str; 14] = [
    "soulPalace",
    "bodyPalace",
    "siblingsPalace",
    "spousePalace",
    "childrenPalace",
    "wealthPalace",
    "healthPalace",
    "surfacePalace",
    "friendsPalace",
    "careerPalace",
    "propertyPalace",
    "spiritPalace",
    "parentsPalace",
    "originalPalace",
];

/// 亮度标识
const BRIGHTNESSES: [Brightness; 7] = [
    Brightness::Miao,
    Brightness::Wang,
    Brightness::De,
    Brightness::Li,
    Brightness::Ping,
    Brightness::Bu,
    Brightness::Xian,
];

/// 标识译成指定语言的文本
///
/// 覆盖星耀、宫位（含身宫来因宫）、天干、地支、亮度、四化、五行局、性别、
/// 生肖、时辰、星座、运限层级十二类。未知标识返回 `None`。
pub fn translate_key(key: &str, lang: Language) -> Option<&'static str> {
    if let Some(star) = StarKey::from_key(key) {
        return Some(translate_star(star, lang));
    }
    if let Some(palace) = Palace::from_key(key) {
        return Some(translate_palace(palace, lang));
    }
    if let Some((_, names)) = MARKER_PALACES.iter().find(|(k, _)| *k == key) {
        return Some(names[lang_index(lang)]);
    }
    if let Some(stem) = HeavenlyStem::from_key(key) {
        return Some(translate_heavenly_stem(stem, lang));
    }
    if let Some(branch) = EarthlyBranch::from_key(key) {
        return Some(translate_earthly_branch(branch, lang));
    }
    if let Some(brightness) = Brightness::from_key(key) {
        return Some(translate_brightness(brightness, lang));
    }
    if let Some(mutagen) = Mutagen::from_key(key) {
        return Some(translate_mutagen(mutagen, lang));
    }
    if let Some(class) = FiveElementsClass::from_key(key) {
        return Some(translate_five_elements_class(class, lang));
    }
    if let Some((_, gender)) = GENDERS.iter().find(|(k, _)| *k == key) {
        return Some(translate_gender(*gender, lang));
    }
    if let Some(i) = ZODIAC.iter().position(|z| *z == key) {
        return Some(translate_zodiac(EARTHLY_BRANCHES[i], lang));
    }
    if let Some(i) = CHINESE_TIME.iter().position(|t| *t == key) {
        return Some(translate_time(i as u8, lang));
    }
    if let Some(i) = SIGNS.iter().position(|s| *s == key) {
        return Some(translate_sign(i, lang));
    }
    if let Some((_, name)) = HOROSCOPE_NAMES.iter().find(|(k, _)| *k == key) {
        return Some(translate_horoscope_name(*name, lang));
    }
    None
}

/// 全部可翻译标识
///
/// 顺序复刻 iztro 各语言翻译文件的合并次序（`common.json`、五行局、天干、地支、
/// 亮度、四化、星耀、宫位、性别），[`key_of`] 据此决定同形译名取哪一个标识。
pub fn all_keys() -> Vec<&'static str> {
    ALL_KEYS.clone()
}

/// [`all_keys`] 的结果，首次反查时构建一次。
///
/// 反查要对六种语言各扫一遍整表，每次现算会把这张两百多项的表重复分配。
static ALL_KEYS: LazyLock<Vec<&'static str>> = LazyLock::new(build_all_keys);

fn build_all_keys() -> Vec<&'static str> {
    let mut keys: Vec<&'static str> = Vec::with_capacity(260);

    // common.json：运限层级、生肖、时辰、星座
    keys.extend(HOROSCOPE_NAMES.iter().map(|(k, _)| *k));
    keys.extend(ZODIAC);
    keys.extend(CHINESE_TIME);
    keys.extend(SIGNS);

    keys.extend(FIVE_ELEMENTS_CLASSES.iter().map(|c| c.as_key()));
    keys.extend(HEAVENLY_STEMS.iter().map(|s| s.as_key()));
    keys.extend(EARTHLY_BRANCHES.iter().map(|b| b.as_key()));
    keys.extend(BRIGHTNESSES.iter().map(|b| b.as_key()));
    keys.extend(MUTAGEN.iter().map(|m| m.as_key()));
    keys.extend(ALL_STARS.iter().map(|s| s.as_key()));
    keys.extend(LOOKUP_PALACE_KEYS);
    keys.extend(GENDERS.iter().map(|(k, _)| *k));

    keys
}

/// 反查时的语言遍历顺序
///
/// iztro 的 `kot` 按其 i18n 资源对象的字面量顺序逐语言扫描，先扫完一种语言的
/// 全部标识才进下一种。同形译名落到哪个标识由此决定，故顺序必须一致。
const LOOKUP_LANGS: [Language; 6] = [
    Language::EnUS,
    Language::JaJP,
    Language::KoKR,
    Language::ZhCN,
    Language::ZhTW,
    Language::ViVN,
];

/// 由任意语言的译名反查标识
///
/// 逐语言、每种语言内逐标识比对，取先命中者：语言按 en-US、ja-JP、ko-KR、
/// zh-CN、zh-TW、vi-VN 扫描，语言内的标识顺序见 [`all_keys`]，与 iztro 的
/// `kot` 逐例一致。找不到返回 `None`。
///
/// 同一文本在不同类目下同形时（如 en-US 的 `horse` 既是生肖马也是天马），
/// 用 [`key_of_in`] 按标识名限定类目。
pub fn key_of(text: &str) -> Option<&'static str> {
    key_of_matching(text, |_| true)
}

/// 限定标识名后反查标识
///
/// 只在标识名含 `key_filter` 子串的那些标识里找，对应 iztro `kot` 的第二个参数。
/// 类目在标识名上有共同后缀，据此可消歧：`"Maj"` 只看十四主星、`"Min"` 只看辅星、
/// `"Heavenly"` / `"Earthly"` 只看干支、`"Palace"` 只看宫位、`"Hour"` 只看时辰。
pub fn key_of_in(text: &str, key_filter: &str) -> Option<&'static str> {
    key_of_matching(text, |key| key.contains(key_filter))
}

fn key_of_matching(text: &str, accept: impl Fn(&str) -> bool + Copy) -> Option<&'static str> {
    LOOKUP_LANGS.iter().find_map(|lang| {
        ALL_KEYS
            .iter()
            .copied()
            .find(|key| accept(key) && translate_key(key, *lang) == Some(text))
    })
}

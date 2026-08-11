//! 金标测试共享工具。

use rs_iztro::data::types::*;
use rs_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use rs_iztro::models::astrolabe::Astrolabe;
use rs_iztro::models::star::Star;

const LANG: Language = Language::ZhCN;

/// 星耀条目：`名:亮度:四化`，亮度/四化无则为空串。
fn star_entry(s: &Star) -> String {
    format!(
        "{}:{}:{}",
        s.name,
        s.brightness.map(|b| translate_brightness(b, LANG)).unwrap_or(""),
        s.mutagen.map(|m| translate_mutagen(m, LANG)).unwrap_or(""),
    )
}

/// 排盘结果的规范化字符串。
///
/// 与 JS 侧 tests/golden/canonical.mjs 的 canonicalAstrolabe() 严格同构：
/// 逐字节一致的字符串经 SHA-256 后即为 tier3 金标哈希。
///
/// 格式：顶层字段 '|' 连接，之后每宫一段，段间 '#' 连接；
/// 辅星与杂耀条目按字符串排序，主星保持安放顺序。
pub fn canonical_astrolabe(a: &Astrolabe) -> String {
    let top = [
        translate_gender(a.gender, LANG).to_string(),
        a.solar_date.clone(),
        a.lunar_date.clone(),
        a.chinese_date.clone(),
        a.time.clone(),
        a.time_range.clone(),
        a.sign.clone(),
        a.zodiac.clone(),
        translate_earthly_branch(a.earthly_branch_of_soul_palace, LANG).to_string(),
        translate_earthly_branch(a.earthly_branch_of_body_palace, LANG).to_string(),
        translate_star(a.soul, LANG).to_string(),
        translate_star(a.body, LANG).to_string(),
        translate_five_elements_class(a.five_elements_class, LANG).to_string(),
    ]
    .join("|");

    let palaces: Vec<String> = a
        .palaces
        .iter()
        .map(|p| {
            let majors: Vec<String> = p.major_stars.iter().map(star_entry).collect();
            let mut minors: Vec<String> = p.minor_stars.iter().map(star_entry).collect();
            minors.sort();
            let mut adjs: Vec<String> =
                p.adjective_stars.iter().map(|s| s.name.clone()).collect();
            adjs.sort();
            let ages: Vec<String> = p.ages.iter().map(|x| x.to_string()).collect();
            [
                translate_palace(p.name, LANG).to_string(),
                translate_heavenly_stem(p.heavenly_stem, LANG).to_string(),
                translate_earthly_branch(p.earthly_branch, LANG).to_string(),
                if p.is_body_palace { "1" } else { "0" }.to_string(),
                if p.is_original_palace { "1" } else { "0" }.to_string(),
                majors.join(";"),
                minors.join(";"),
                adjs.join(";"),
                translate_star(p.changsheng12, LANG).to_string(),
                translate_star(p.boshi12, LANG).to_string(),
                translate_star(p.jiangqian12, LANG).to_string(),
                translate_star(p.suiqian12, LANG).to_string(),
                format!("{}-{}", p.decadal.range.0, p.decadal.range.1),
                ages.join(","),
            ]
            .join("|")
        })
        .collect();

    format!("{}#{}", top, palaces.join("#"))
}

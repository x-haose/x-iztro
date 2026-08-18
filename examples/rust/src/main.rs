//! x-iztro Rust 示例
//!
//! 排盘 → 命盘信息 → 流年运限 → AI 提示词
//!
//! 运行方式：
//!   cd examples/rust
//!   cargo run

use x_iztro::data::stars::StarKey;
use x_iztro::data::types::*;
use x_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use x_iztro::{IztroError, astrolabe_to_prompt, by_solar, horoscope_to_prompt};

fn main() -> Result<(), IztroError> {
    let lang = Language::ZhCN;

    // ============================================================
    // 1. 排盘
    // ============================================================
    println!("===== 排盘 =====\n");

    let astrolabe = by_solar(
        // "1997-03-04",
        "2001-07-16",
        12,
        // 1,
        Gender::Male,
        true,
        lang,
        Config::default(),
    )?;

    println!("阳历：{}", astrolabe.solar_date);
    println!("农历：{}", astrolabe.lunar_date);
    println!("干支：{}", astrolabe.chinese_date);
    println!("时辰：{} ({})", astrolabe.time, astrolabe.time_range);
    println!("星座：{}", astrolabe.sign);
    println!("生肖：{}", astrolabe.zodiac);

    // ============================================================
    // 2. 命盘信息
    // ============================================================
    println!("\n===== 命盘信息 =====\n");

    println!(
        "命宫：{}    身宫：{}",
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang),
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang),
    );
    println!(
        "命主：{}    身主：{}",
        translate_star(astrolabe.soul, lang),
        translate_star(astrolabe.body, lang),
    );
    println!(
        "五行局：{}",
        translate_five_elements_class(astrolabe.five_elements_class, lang),
    );

    println!("\n— 十二宫 —\n");
    for i in 0..12 {
        let p = astrolabe.palace(i).expect("索引 0-11 必然命中");
        let major: Vec<String> = p
            .major_stars
            .iter()
            .map(|s| {
                let mut name = s.name.clone();
                if let Some(b) = s.brightness {
                    name.push_str(&format!("({})", translate_brightness(b, lang)));
                }
                if let Some(m) = s.mutagen {
                    name.push_str(&format!("[{}]", translate_mutagen(m, lang)));
                }
                name
            })
            .collect();
        let body_tag = if p.is_body_palace { " [身]" } else { "" };
        println!(
            "  {}{} ({}{}) | {}",
            translate_palace(p.name, lang),
            body_tag,
            translate_heavenly_stem(p.heavenly_stem, lang),
            translate_earthly_branch(p.earthly_branch, lang),
            if major.is_empty() {
                "空宫".to_string()
            } else {
                major.join(" ")
            },
        );
    }

    // 关键星耀位置
    println!("\n— 关键星耀 —\n");
    let key_stars = [
        StarKey::ZiweiMaj,
        StarKey::TianjiMaj,
        StarKey::TaiyangMaj,
        StarKey::TaiyinMaj,
        StarKey::TianfuMaj,
        StarKey::LucunMin,
        StarKey::TianmaMin,
    ];
    for key in &key_stars {
        if let Some(star) = astrolabe.star(*key) {
            let mut info = format!(
                "  {} → {}",
                translate_star(*key, lang),
                translate_palace(star.palace().name, lang),
            );
            if let Some(b) = star.brightness {
                info.push_str(&format!(" ({})", translate_brightness(b, lang)));
            }
            if let Some(m) = star.mutagen {
                info.push_str(&format!(" [{}]", translate_mutagen(m, lang)));
            }
            println!("{info}");
        }
    }

    // ============================================================
    // 3. 流年运限
    // ============================================================
    println!("\n===== 流年运限 (2026-10-01) =====\n");

    let horoscope = astrolabe.horoscope("2027-10-1", 0)?;

    println!("日期：{} / {}", horoscope.solar_date, horoscope.lunar_date);
    println!(
        "大限：{} ({}{})",
        translate_palace(astrolabe.palace(horoscope.decadal.index).unwrap().name, lang),
        translate_heavenly_stem(horoscope.decadal.heavenly_stem, lang),
        translate_earthly_branch(horoscope.decadal.earthly_branch, lang),
    );
    println!(
        "小限：{} (虚岁 {})",
        translate_palace(astrolabe.palace(horoscope.age.index).unwrap().name, lang),
        horoscope.age.nominal_age,
    );
    println!(
        "流年：{}{}",
        translate_heavenly_stem(horoscope.yearly.heavenly_stem, lang),
        translate_earthly_branch(horoscope.yearly.earthly_branch, lang),
    );
    println!(
        "流月：{}{}",
        translate_heavenly_stem(horoscope.monthly.heavenly_stem, lang),
        translate_earthly_branch(horoscope.monthly.earthly_branch, lang),
    );
    println!(
        "流日：{}{}",
        translate_heavenly_stem(horoscope.daily.heavenly_stem, lang),
        translate_earthly_branch(horoscope.daily.earthly_branch, lang),
    );
    println!(
        "流时：{}{}",
        translate_heavenly_stem(horoscope.hourly.heavenly_stem, lang),
        translate_earthly_branch(horoscope.hourly.earthly_branch, lang),
    );

    // 流年四化
    let yearly_mutagen_names: Vec<String> = horoscope
        .yearly
        .base
        .mutagen
        .iter()
        .map(|k| translate_star(*k, lang).to_string())
        .collect();
    println!("\n流年四化：{}", yearly_mutagen_names.join("、"));

    // ============================================================
    // 4. AI 提示词（本命 + 流年）
    // ============================================================
    println!("\n===== AI 提示词 =====\n");

    let natal_prompt = astrolabe_to_prompt(&astrolabe, lang);
    let fortune_prompt = horoscope_to_prompt(&astrolabe, &horoscope, lang);
    let full_prompt = format!("{}\n{}", natal_prompt, fortune_prompt);

    println!("{full_prompt}");
    println!("（Prompt 共 {} 字符）", full_prompt.chars().count());

    println!("\n===== 完毕 =====");

    Ok(())
}

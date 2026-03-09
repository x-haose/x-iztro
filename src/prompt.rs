// 提示信息模块

use crate::data::types::Language;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace,
    translate_star,
};
use crate::models::astrolabe::Astrolabe;
use crate::models::star::Star;

/// Format a star with its brightness and mutagen annotations.
fn format_star(star: &Star, lang: Language) -> String {
    let mut s = star.name.clone();
    if let Some(b) = star.brightness {
        s.push_str(&format!("({})", translate_brightness(b, lang)));
    }
    if let Some(m) = star.mutagen {
        s.push_str(&format!("[{}]", translate_mutagen(m, lang)));
    }
    s
}

/// Generate a structured text representation of an astrology chart suitable for AI analysis.
pub fn astrolabe_to_prompt(astrolabe: &Astrolabe, lang: Language) -> String {
    let mut out = String::new();

    // Basic info section
    out.push_str("=== Basic Info ===\n");
    out.push_str(&format!(
        "Gender: {}\n",
        translate_gender(astrolabe.gender, lang)
    ));
    out.push_str(&format!("Solar Date: {}\n", astrolabe.solar_date));
    out.push_str(&format!("Lunar Date: {}\n", astrolabe.lunar_date));
    out.push_str(&format!("Chinese Date: {}\n", astrolabe.chinese_date));
    out.push_str(&format!("Time: {} ({})\n", astrolabe.time, astrolabe.time_range));
    out.push_str(&format!("Zodiac Sign: {}\n", astrolabe.sign));
    out.push_str(&format!("Zodiac Animal: {}\n", astrolabe.zodiac));
    out.push_str(&format!(
        "Soul Palace Branch: {}\n",
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang)
    ));
    out.push_str(&format!(
        "Body Palace Branch: {}\n",
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang)
    ));
    out.push_str(&format!(
        "Soul Star: {}\n",
        translate_star(astrolabe.soul, lang)
    ));
    out.push_str(&format!(
        "Body Star: {}\n",
        translate_star(astrolabe.body, lang)
    ));
    out.push_str(&format!(
        "Five Elements Class: {}\n",
        translate_five_elements_class(astrolabe.five_elements_class, lang)
    ));

    // Palace info section
    out.push_str("\n=== Palaces ===\n");
    for palace in &astrolabe.palaces {
        out.push_str(&format!(
            "\n--- {} {} ---\n",
            translate_palace(palace.name, lang),
            if palace.is_body_palace { "[Body Palace]" } else { "" }
        ));
        out.push_str(&format!(
            "Stem-Branch: {}{}\n",
            translate_heavenly_stem(palace.heavenly_stem, lang),
            translate_earthly_branch(palace.earthly_branch, lang)
        ));
        out.push_str(&format!(
            "Decadal: {}-{}\n",
            palace.decadal.range.0, palace.decadal.range.1
        ));

        if !palace.major_stars.is_empty() {
            let stars: Vec<String> = palace
                .major_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("Major Stars: {}\n", stars.join(", ")));
        }

        if !palace.minor_stars.is_empty() {
            let stars: Vec<String> = palace
                .minor_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("Minor Stars: {}\n", stars.join(", ")));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::types::*;

    #[test]
    fn test_astrolabe_to_prompt_non_empty() {
        let astrolabe = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Algorithm::Default,
        );
        let prompt = astrolabe_to_prompt(&astrolabe, Language::ZhCN);

        assert!(!prompt.is_empty());
        assert!(prompt.contains("=== Basic Info ==="));
        assert!(prompt.contains("=== Palaces ==="));
        assert!(prompt.contains("Gender:"));
        assert!(prompt.contains("Solar Date:"));
        assert!(prompt.contains("Major Stars:"));
        assert!(prompt.contains("Stem-Branch:"));
        assert!(prompt.contains("Decadal:"));
    }

    #[test]
    fn test_astrolabe_to_prompt_en() {
        let astrolabe = by_solar(
            "2000-8-16",
            2,
            Gender::Male,
            true,
            Language::EnUS,
            Algorithm::Default,
        );
        let prompt = astrolabe_to_prompt(&astrolabe, Language::EnUS);

        assert!(!prompt.is_empty());
        assert!(prompt.contains("=== Palaces ==="));
    }
}

// 提示信息模块

use crate::data::stars::StarKey;
use crate::data::types::Language;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::HoroscopeData;
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

/// 根据语言返回标签文本
struct Labels {
    sec_basic: &'static str,
    sec_palaces: &'static str,
    gender: &'static str,
    solar_date: &'static str,
    lunar_date: &'static str,
    chinese_date: &'static str,
    time: &'static str,
    zodiac_sign: &'static str,
    zodiac_animal: &'static str,
    soul_palace_branch: &'static str,
    body_palace_branch: &'static str,
    soul_star: &'static str,
    body_star: &'static str,
    five_elements_class: &'static str,
    body_palace: &'static str,
    stem_branch: &'static str,
    decadal: &'static str,
    major_stars: &'static str,
    minor_stars: &'static str,
    adjective_stars: &'static str,
    // horoscope labels
    sec_horoscope: &'static str,
    target_date: &'static str,
    decadal_fortune: &'static str,
    age_fortune: &'static str,
    nominal_age: &'static str,
    yearly: &'static str,
    monthly: &'static str,
    daily: &'static str,
    hourly: &'static str,
    mutagen_fly: &'static str,
    scope_stars: &'static str,
}

fn labels(lang: Language) -> Labels {
    match lang {
        Language::ZhCN => Labels {
            sec_basic: "=== 基本信息 ===",
            sec_palaces: "=== 十二宫 ===",
            gender: "性别",
            solar_date: "阳历",
            lunar_date: "农历",
            chinese_date: "干支",
            time: "时辰",
            zodiac_sign: "星座",
            zodiac_animal: "生肖",
            soul_palace_branch: "命宫地支",
            body_palace_branch: "身宫地支",
            soul_star: "命主",
            body_star: "身主",
            five_elements_class: "五行局",
            body_palace: "[身宫]",
            stem_branch: "天干地支",
            decadal: "大限",
            major_stars: "主星",
            minor_stars: "辅星",
            adjective_stars: "杂耀",
            sec_horoscope: "=== 运限 ===",
            target_date: "目标日期",
            decadal_fortune: "大限",
            age_fortune: "小限",
            nominal_age: "虚岁",
            yearly: "流年",
            monthly: "流月",
            daily: "流日",
            hourly: "流时",
            mutagen_fly: "四化",
            scope_stars: "流耀",
        },
        Language::ZhTW => Labels {
            sec_basic: "=== 基本資訊 ===",
            sec_palaces: "=== 十二宮 ===",
            gender: "性別",
            solar_date: "陽曆",
            lunar_date: "農曆",
            chinese_date: "干支",
            time: "時辰",
            zodiac_sign: "星座",
            zodiac_animal: "生肖",
            soul_palace_branch: "命宮地支",
            body_palace_branch: "身宮地支",
            soul_star: "命主",
            body_star: "身主",
            five_elements_class: "五行局",
            body_palace: "[身宮]",
            stem_branch: "天干地支",
            decadal: "大限",
            major_stars: "主星",
            minor_stars: "輔星",
            adjective_stars: "雜曜",
            sec_horoscope: "=== 運限 ===",
            target_date: "目標日期",
            decadal_fortune: "大限",
            age_fortune: "小限",
            nominal_age: "虛歲",
            yearly: "流年",
            monthly: "流月",
            daily: "流日",
            hourly: "流時",
            mutagen_fly: "四化",
            scope_stars: "流曜",
        },
        _ => Labels {
            sec_basic: "=== Basic Info ===",
            sec_palaces: "=== Palaces ===",
            gender: "Gender",
            solar_date: "Solar Date",
            lunar_date: "Lunar Date",
            chinese_date: "Chinese Date",
            time: "Time",
            zodiac_sign: "Zodiac Sign",
            zodiac_animal: "Zodiac Animal",
            soul_palace_branch: "Soul Palace Branch",
            body_palace_branch: "Body Palace Branch",
            soul_star: "Soul Star",
            body_star: "Body Star",
            five_elements_class: "Five Elements Class",
            body_palace: "[Body Palace]",
            stem_branch: "Stem-Branch",
            decadal: "Decadal",
            major_stars: "Major Stars",
            minor_stars: "Minor Stars",
            adjective_stars: "Adjective Stars",
            sec_horoscope: "=== Horoscope ===",
            target_date: "Target Date",
            decadal_fortune: "Decadal Fortune",
            age_fortune: "Age Fortune",
            nominal_age: "Nominal Age",
            yearly: "Yearly",
            monthly: "Monthly",
            daily: "Daily",
            hourly: "Hourly",
            mutagen_fly: "Mutagen",
            scope_stars: "Scope Stars",
        },
    }
}

/// Generate a structured text prompt of the natal chart for AI analysis.
pub fn astrolabe_to_prompt(astrolabe: &Astrolabe, lang: Language) -> String {
    let l = labels(lang);
    let mut out = String::new();

    // Basic info section
    out.push_str(&format!("{}\n", l.sec_basic));
    out.push_str(&format!(
        "{}: {}\n",
        l.gender,
        translate_gender(astrolabe.gender, lang)
    ));
    out.push_str(&format!("{}: {}\n", l.solar_date, astrolabe.solar_date));
    out.push_str(&format!("{}: {}\n", l.lunar_date, astrolabe.lunar_date));
    out.push_str(&format!("{}: {}\n", l.chinese_date, astrolabe.chinese_date));
    out.push_str(&format!(
        "{}: {} ({})\n",
        l.time, astrolabe.time, astrolabe.time_range
    ));
    out.push_str(&format!("{}: {}\n", l.zodiac_sign, astrolabe.sign));
    out.push_str(&format!("{}: {}\n", l.zodiac_animal, astrolabe.zodiac));
    out.push_str(&format!(
        "{}: {}\n",
        l.soul_palace_branch,
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang)
    ));
    out.push_str(&format!(
        "{}: {}\n",
        l.body_palace_branch,
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang)
    ));
    out.push_str(&format!(
        "{}: {}\n",
        l.soul_star,
        translate_star(astrolabe.soul, lang)
    ));
    out.push_str(&format!(
        "{}: {}\n",
        l.body_star,
        translate_star(astrolabe.body, lang)
    ));
    out.push_str(&format!(
        "{}: {}\n",
        l.five_elements_class,
        translate_five_elements_class(astrolabe.five_elements_class, lang)
    ));

    // Palace info section
    out.push_str(&format!("\n{}\n", l.sec_palaces));
    for palace in &astrolabe.palaces {
        out.push_str(&format!(
            "\n--- {} {} ---\n",
            translate_palace(palace.name, lang),
            if palace.is_body_palace {
                l.body_palace
            } else {
                ""
            }
        ));
        out.push_str(&format!(
            "{}: {}{}\n",
            l.stem_branch,
            translate_heavenly_stem(palace.heavenly_stem, lang),
            translate_earthly_branch(palace.earthly_branch, lang)
        ));
        out.push_str(&format!(
            "{}: {}-{}\n",
            l.decadal, palace.decadal.range.0, palace.decadal.range.1
        ));

        if !palace.major_stars.is_empty() {
            let stars: Vec<String> = palace
                .major_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("{}: {}\n", l.major_stars, stars.join(", ")));
        }

        if !palace.minor_stars.is_empty() {
            let stars: Vec<String> = palace
                .minor_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("{}: {}\n", l.minor_stars, stars.join(", ")));
        }

        if !palace.adjective_stars.is_empty() {
            let stars: Vec<String> = palace
                .adjective_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("{}: {}\n", l.adjective_stars, stars.join(", ")));
        }
    }

    out
}

/// 输出某运限层级的四化信息
fn format_mutagen_line(
    label: &str,
    mutagen_label: &str,
    keys: &[StarKey],
    lang: Language,
) -> String {
    let names: Vec<String> = keys
        .iter()
        .map(|k| translate_star(*k, lang).to_string())
        .collect();
    format!("  {}{}: {}\n", label, mutagen_label, names.join(", "))
}

/// 输出某运限层级的十二宫星耀分布（本命主星/辅星 + 运限流耀）
fn format_scope_palaces(
    item: &crate::models::horoscope::HoroscopeItem,
    astrolabe: &Astrolabe,
    lang: Language,
    l: &Labels,
) -> String {
    let mut out = String::new();
    let scope_stars = item.stars.as_ref();
    for (i, palace) in astrolabe.palaces.iter().enumerate() {
        let scope_palace_name = if i < item.palace_names.len() {
            translate_palace(item.palace_names[i], lang)
        } else {
            translate_palace(palace.name, lang)
        };
        let natal_palace_name = translate_palace(palace.name, lang);

        // 宫位标题
        if scope_palace_name == natal_palace_name {
            out.push_str(&format!("  {}:\n", scope_palace_name));
        } else {
            out.push_str(&format!(
                "  {} ({}):\n",
                scope_palace_name, natal_palace_name
            ));
        }

        // 本命主星
        if !palace.major_stars.is_empty() {
            let stars: Vec<String> = palace
                .major_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("    {}: {}\n", l.major_stars, stars.join(", ")));
        }

        // 本命辅星
        if !palace.minor_stars.is_empty() {
            let stars: Vec<String> = palace
                .minor_stars
                .iter()
                .map(|s| format_star(s, lang))
                .collect();
            out.push_str(&format!("    {}: {}\n", l.minor_stars, stars.join(", ")));
        }

        // 运限流耀
        if let Some(all_stars) = scope_stars {
            let scope_star_list = all_stars.get(i).cloned().unwrap_or_default();
            if !scope_star_list.is_empty() {
                let stars: Vec<String> = scope_star_list
                    .iter()
                    .map(|s| format_star(s, lang))
                    .collect();
                out.push_str(&format!("    {}: {}\n", l.scope_stars, stars.join(", ")));
            }
        }
    }
    out
}

/// Generate a structured text prompt of horoscope (fortune) data for AI analysis.
pub fn horoscope_to_prompt(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    lang: Language,
) -> String {
    let l = labels(lang);
    let mut out = String::new();

    out.push_str(&format!("{}\n", l.sec_horoscope));
    out.push_str(&format!(
        "{}: {} / {}\n\n",
        l.target_date, horoscope.solar_date, horoscope.lunar_date
    ));

    // 大限
    out.push_str(&format!("--- {} ---\n", l.decadal_fortune,));
    out.push_str(&format!(
        "{}: {} ({}{})\n",
        l.decadal_fortune,
        translate_palace(astrolabe.palaces[horoscope.decadal.index].name, lang),
        translate_heavenly_stem(horoscope.decadal.heavenly_stem, lang),
        translate_earthly_branch(horoscope.decadal.earthly_branch, lang),
    ));
    out.push_str(&format_mutagen_line(
        l.decadal_fortune,
        l.mutagen_fly,
        &horoscope.decadal.mutagen,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.decadal,
        astrolabe,
        lang,
        &l,
    ));

    // 小限
    out.push_str(&format!(
        "\n{}: {} ({} {})\n",
        l.age_fortune,
        translate_palace(astrolabe.palaces[horoscope.age.base.index].name, lang),
        l.nominal_age,
        horoscope.age.nominal_age,
    ));

    // 流年
    out.push_str(&format!("\n--- {} ---\n", l.yearly,));
    out.push_str(&format!(
        "{}: {}{}\n",
        l.yearly,
        translate_heavenly_stem(horoscope.yearly.base.heavenly_stem, lang),
        translate_earthly_branch(horoscope.yearly.base.earthly_branch, lang),
    ));
    out.push_str(&format_mutagen_line(
        l.yearly,
        l.mutagen_fly,
        &horoscope.yearly.base.mutagen,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.yearly.base,
        astrolabe,
        lang,
        &l,
    ));

    // 流月/流日/流时 — 只输出四化，不展开十二宫
    let format_brief = |label: &str, item: &crate::models::horoscope::HoroscopeItem| -> String {
        let mut s = format!(
            "\n{}: {}{}\n",
            label,
            translate_heavenly_stem(item.heavenly_stem, lang),
            translate_earthly_branch(item.earthly_branch, lang),
        );
        s.push_str(&format_mutagen_line(
            label,
            l.mutagen_fly,
            &item.mutagen,
            lang,
        ));
        s
    };

    out.push_str(&format_brief(l.monthly, &horoscope.monthly));
    out.push_str(&format_brief(l.daily, &horoscope.daily));
    out.push_str(&format_brief(l.hourly, &horoscope.hourly));

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::astro::horoscope::get_horoscope;
    use crate::data::types::*;

    #[test]
    fn test_astrolabe_to_prompt_zh_cn() {
        let astrolabe = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let prompt = astrolabe_to_prompt(&astrolabe, Language::ZhCN);

        assert!(!prompt.is_empty());
        assert!(prompt.contains("=== 基本信息 ==="));
        assert!(prompt.contains("=== 十二宫 ==="));
        assert!(prompt.contains("性别:"));
        assert!(prompt.contains("阳历:"));
        assert!(prompt.contains("主星:"));
        assert!(prompt.contains("天干地支:"));
        assert!(prompt.contains("大限:"));
    }

    #[test]
    fn test_astrolabe_to_prompt_en() {
        let astrolabe = by_solar(
            "2000-8-16",
            2,
            Gender::Male,
            true,
            Language::EnUS,
            Config::default(),
        )
        .unwrap();
        let prompt = astrolabe_to_prompt(&astrolabe, Language::EnUS);

        assert!(!prompt.is_empty());
        assert!(prompt.contains("=== Palaces ==="));
        assert!(prompt.contains("Gender:"));
        assert!(prompt.contains("Major Stars:"));
    }

    #[test]
    fn test_horoscope_to_prompt_zh_cn() {
        let lang = Language::ZhCN;
        let astrolabe = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            lang,
            Config::default(),
        )
        .unwrap();
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let prompt = horoscope_to_prompt(&astrolabe, &horoscope, lang);

        assert!(!prompt.is_empty());
        assert!(prompt.contains("=== 运限 ==="));
        assert!(prompt.contains("大限:"));
        assert!(prompt.contains("流年:"));
        assert!(prompt.contains("流月:"));
        assert!(prompt.contains("流年四化:"));
    }
}

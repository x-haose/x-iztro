//! AI 分析 Prompt 生成
//!
//! 把星盘与运限落成结构化纯文本，供大语言模型直接阅读：每行一个「标签: 取值」，
//! 宫位以 `--- 宫名 ---` 分段。星耀、宫名、干支等取值一律按传入语言翻译，
//! 结构标签则取本模块的六语言标签表——三个语言绑定的 Prompt 都出自这里，
//! 因此同一张盘在三侧的输出逐字节一致。

use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::{HeavenlyStem, Language};
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace, translate_star,
};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::{HoroscopeData, HoroscopeItem, YearlyDecStar};
use crate::models::palace::PalaceData;
use crate::models::star::Star;

/// 列表型取值的分隔符：星耀列表、十二神、四化都用它连接
const LIST_SEP: &str = ", ";

/// 一颗星的展示写法：`名称(亮度)[四化]`，无亮度或无四化则省略对应部分
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

/// 星耀列表的展示写法；列表为空时返回 `None`，调用方据此整行省略
fn format_stars(stars: &[Star], lang: Language) -> Option<String> {
    if stars.is_empty() {
        return None;
    }
    Some(
        stars
            .iter()
            .map(|s| format_star(s, lang))
            .collect::<Vec<_>>()
            .join(LIST_SEP),
    )
}

/// 星耀标识列表译名，用 [`LIST_SEP`] 连接
fn format_star_keys(keys: &[StarKey], lang: Language) -> String {
    keys.iter()
        .map(|k| translate_star(*k, lang).to_string())
        .collect::<Vec<_>>()
        .join(LIST_SEP)
}

/// 定义结构标签表并生成按语言取值的 `labels`。
///
/// 每个标签一行，六列依次为 zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN，
/// 六种语言的写法并排可见，增删标签不会漏掉某一种语言。
/// 表内只接受 `//` 行注释，不接受字段上的 `///` 文档注释。
macro_rules! labels_table {
    ($($field:ident: [$zh_cn:literal, $zh_tw:literal, $en_us:literal, $ja_jp:literal, $ko_kr:literal, $vi_vn:literal]),+ $(,)?) => {
        /// 一种语言下的全部结构标签
        struct Labels {
            $($field: &'static str,)+
        }

        /// 取指定语言的结构标签
        fn labels(lang: Language) -> Labels {
            let i = match lang {
                Language::ZhCN => 0,
                Language::ZhTW => 1,
                Language::EnUS => 2,
                Language::JaJP => 3,
                Language::KoKR => 4,
                Language::ViVN => 5,
            };
            Labels {
                $($field: [$zh_cn, $zh_tw, $en_us, $ja_jp, $ko_kr, $vi_vn][i],)+
            }
        }
    };
}

labels_table! {
    // ---- 本命盘 ----
    sec_basic: ["=== 基本信息 ===", "=== 基本資訊 ===", "=== Basic Info ===", "=== 基本情報 ===", "=== 기본 정보 ===", "=== Thông tin cơ bản ==="],
    sec_palaces: ["=== 十二宫 ===", "=== 十二宮 ===", "=== Palaces ===", "=== 十二宮 ===", "=== 십이궁 ===", "=== Mười hai cung ==="],
    gender: ["性别", "性別", "Gender", "性別", "성별", "Giới tính"],
    solar_date: ["阳历", "陽曆", "Solar Date", "新暦", "양력", "Dương lịch"],
    lunar_date: ["农历", "農曆", "Lunar Date", "旧暦", "음력", "Âm lịch"],
    chinese_date: ["干支", "干支", "Chinese Date", "干支", "간지", "Can chi"],
    time: ["时辰", "時辰", "Time", "時辰", "시진", "Giờ"],
    zodiac_sign: ["星座", "星座", "Zodiac Sign", "星座", "별자리", "Cung hoàng đạo"],
    zodiac_animal: ["生肖", "生肖", "Zodiac Animal", "干支動物", "띠", "Con giáp"],
    soul_palace_branch: ["命宫地支", "命宮地支", "Soul Palace Branch", "命宮地支", "명궁 지지", "Địa chi cung Mệnh"],
    body_palace_branch: ["身宫地支", "身宮地支", "Body Palace Branch", "身宮地支", "신궁 지지", "Địa chi cung Thân"],
    soul_star: ["命主", "命主", "Soul Star", "命主", "명주", "Mệnh chủ"],
    body_star: ["身主", "身主", "Body Star", "身主", "신주", "Thân chủ"],
    five_elements_class: ["五行局", "五行局", "Five Elements Class", "五行局", "오행국", "Cục ngũ hành"],
    birth_mutagen: ["生年四化", "生年四化", "Birth-Year Mutagen", "生年四化", "생년사화", "Tứ hóa sinh niên"],
    body_palace: ["[身宫]", "[身宮]", "[Body Palace]", "[身宮]", "[신궁]", "[Thân]"],
    original_palace: ["[来因]", "[來因]", "[Original Palace]", "[来因]", "[라인]", "[Lai Nhân]"],
    stem_branch: ["天干地支", "天干地支", "Stem-Branch", "天干地支", "천간지지", "Can chi"],
    decadal: ["大限", "大限", "Decadal", "大限", "대한", "Đại hạn"],
    ages: ["小限虚岁", "小限虛歲", "Age Fortune Years", "小限数え年", "소한 나이", "Tuổi tiểu hạn"],
    twelve_gods: ["十二神", "十二神", "Twelve Gods", "十二神", "십이신", "Mười hai thần"],
    major_stars: ["主星", "主星", "Major Stars", "主星", "주성", "Chính tinh"],
    minor_stars: ["辅星", "輔星", "Minor Stars", "輔星", "보성", "Phụ tinh"],
    adjective_stars: ["杂耀", "雜曜", "Adjective Stars", "雑曜", "잡요", "Tạp diệu"],

    // ---- 运限 ----
    sec_horoscope: ["=== 运限 ===", "=== 運限 ===", "=== Horoscope ===", "=== 運限 ===", "=== 운한 ===", "=== Vận hạn ==="],
    target_date: ["目标日期", "目標日期", "Target Date", "対象日", "대상 날짜", "Ngày mục tiêu"],
    decadal_fortune: ["大限", "大限", "Decadal Fortune", "大限", "대한", "Đại hạn"],
    age_fortune: ["小限", "小限", "Age Fortune", "小限", "소한", "Tiểu hạn"],
    nominal_age: ["虚岁", "虛歲", "Nominal Age", "数え年", "세는나이", "Tuổi âm"],
    yearly: ["流年", "流年", "Yearly", "流年", "유년", "Lưu niên"],
    monthly: ["流月", "流月", "Monthly", "流月", "유월", "Lưu nguyệt"],
    daily: ["流日", "流日", "Daily", "流日", "유일", "Lưu nhật"],
    hourly: ["流时", "流時", "Hourly", "流時", "유시", "Lưu thời"],
    mutagen_fly: ["四化", "四化", "Mutagen", "四化", "사화", "Tứ hóa"],
    scope_stars: ["流耀", "流曜", "Scope Stars", "流曜", "유요", "Lưu diệu"],
    soul_palace: ["命宫", "命宮", "Soul Palace", "命宮", "명궁", "cung Mệnh"],
    natal: ["本命", "本命", "Natal", "本命", "본명", "bản mệnh"],
    palace_names: ["宫名", "宮名", "Palace Names", "宮名", "궁명", "Tên cung"],

    // 复合标签的分隔符：中文与日文的汉字词直接相连（「大限四化」），
    // 拉丁字母与韩文须以空格分词（Decadal Fortune Mutagen、「대한 사화」）
    label_sep: ["", "", " ", "", " ", " "],
}

impl Labels {
    /// 运限层级名与从属标签拼成的复合标签，如「大限四化」`Decadal Fortune Mutagen`
    fn compound(&self, scope: &str, suffix: &str) -> String {
        format!("{scope}{}{suffix}", self.label_sep)
    }
}

/// 生年四化：本命年干化出的四颗星，按禄权科忌顺序写成「星名+四化名」
fn format_birth_mutagen(astrolabe: &Astrolabe, stem: HeavenlyStem, lang: Language) -> String {
    let stars = astrolabe.config.mutagens_of(stem);
    stars
        .iter()
        .zip(MUTAGEN)
        .map(|(star, mutagen)| {
            format!(
                "{}{}",
                translate_star(*star, lang),
                translate_mutagen(mutagen, lang)
            )
        })
        .collect::<Vec<_>>()
        .join(LIST_SEP)
}

/// 宫位标题上的标记：身宫、来因宫各占一个方括号，都不是则为空串
fn palace_markers(palace: &PalaceData, l: &Labels) -> String {
    let mut markers = String::new();
    if palace.is_body_palace {
        markers.push(' ');
        markers.push_str(l.body_palace);
    }
    if palace.is_original_palace {
        markers.push(' ');
        markers.push_str(l.original_palace);
    }
    markers
}

/// 本命盘的结构化文本
pub fn astrolabe_to_prompt(astrolabe: &Astrolabe, lang: Language) -> String {
    let l = labels(lang);
    let mut out = String::new();

    // ---- 基本信息 ----
    out.push_str(&format!("{}\n", l.sec_basic));
    let mut line = |label: &str, value: &str| out.push_str(&format!("{label}: {value}\n"));
    line(l.gender, translate_gender(astrolabe.gender, lang));
    line(l.solar_date, &astrolabe.solar_date);
    line(l.lunar_date, &astrolabe.lunar_date);
    line(l.chinese_date, &astrolabe.chinese_date);
    line(
        l.time,
        &format!("{} ({})", astrolabe.time, astrolabe.time_range),
    );
    line(l.zodiac_sign, &astrolabe.sign);
    line(l.zodiac_animal, &astrolabe.zodiac);
    line(
        l.soul_palace_branch,
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang),
    );
    line(
        l.body_palace_branch,
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang),
    );
    line(l.soul_star, translate_star(astrolabe.soul, lang));
    line(l.body_star, translate_star(astrolabe.body, lang));
    line(
        l.five_elements_class,
        translate_five_elements_class(astrolabe.five_elements_class, lang),
    );
    // 生年四化由本命年干决定，年干即四柱年柱的天干
    line(
        l.birth_mutagen,
        &format_birth_mutagen(astrolabe, astrolabe.raw_dates.chinese_date.yearly.0, lang),
    );

    // ---- 十二宫 ----
    out.push_str(&format!("\n{}\n", l.sec_palaces));
    for palace in &astrolabe.palaces {
        out.push_str(&format!(
            "\n--- {}{} ---\n",
            translate_palace(palace.name, lang),
            palace_markers(palace, &l)
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
        out.push_str(&format!(
            "{}: {}\n",
            l.ages,
            palace
                .ages
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(LIST_SEP)
        ));
        // 长生、博士、岁前、将前四组十二神各取本宫的一位
        out.push_str(&format!(
            "{}: {}\n",
            l.twelve_gods,
            format_star_keys(
                &[
                    palace.changsheng12,
                    palace.boshi12,
                    palace.suiqian12,
                    palace.jiangqian12,
                ],
                lang
            )
        ));

        for (label, stars) in [
            (l.major_stars, &palace.major_stars),
            (l.minor_stars, &palace.minor_stars),
            (l.adjective_stars, &palace.adjective_stars),
        ] {
            if let Some(text) = format_stars(stars, lang) {
                out.push_str(&format!("{label}: {text}\n"));
            }
        }
    }

    out
}

/// 某运限层级的四化行，如「大限四化: 天梁, 紫微, 左辅, 武曲」
fn format_mutagen_line(scope: &str, l: &Labels, keys: &[StarKey], lang: Language) -> String {
    format!(
        "  {}: {}\n",
        l.compound(scope, l.mutagen_fly),
        format_star_keys(keys, lang)
    )
}

/// 某运限层级的落宫行，如「大限命宫: 本命夫妻 (壬辰)」
///
/// 运限的命宫落在本命的哪一宫，是该层级十二宫重排的起点，也是解盘的入口。
fn format_scope_head(
    scope: &str,
    l: &Labels,
    item: &HoroscopeItem,
    astrolabe: &Astrolabe,
    lang: Language,
) -> String {
    format!(
        "{}: {}{}{} ({}{})\n",
        l.compound(scope, l.soul_palace),
        l.natal,
        l.label_sep,
        translate_palace(astrolabe.palaces[item.index].name, lang),
        translate_heavenly_stem(item.heavenly_stem, lang),
        translate_earthly_branch(item.earthly_branch, lang),
    )
}

/// 某运限层级重排后的十二宫名，按本命宫位索引顺序排列
fn format_palace_names_line(
    scope: &str,
    l: &Labels,
    item: &HoroscopeItem,
    lang: Language,
) -> String {
    format!(
        "  {}: {}\n",
        l.compound(scope, l.palace_names),
        item.palace_names
            .iter()
            .map(|p| translate_palace(*p, lang).to_string())
            .collect::<Vec<_>>()
            .join(LIST_SEP)
    )
}

/// 一宫之内的本命主星、辅星与杂耀，按给定缩进逐行输出
fn format_palace_stars(palace: &PalaceData, indent: &str, l: &Labels, lang: Language) -> String {
    let mut out = String::new();
    for (label, stars) in [
        (l.major_stars, &palace.major_stars),
        (l.minor_stars, &palace.minor_stars),
        (l.adjective_stars, &palace.adjective_stars),
    ] {
        if let Some(text) = format_stars(stars, lang) {
            out.push_str(&format!("{indent}{label}: {text}\n"));
        }
    }
    out
}

/// 某运限层级的十二宫展开：宫名（运限名 + 本命名）、本命主辅星、该层级流耀
///
/// `dec_star` 非空时（流年）额外输出每宫的岁前、将前十二神。
fn format_scope_palaces(
    item: &HoroscopeItem,
    dec_star: Option<&YearlyDecStar>,
    astrolabe: &Astrolabe,
    lang: Language,
    l: &Labels,
) -> String {
    let mut out = String::new();
    for (i, palace) in astrolabe.palaces.iter().enumerate() {
        let scope_palace_name = item.palace_names.get(i).map_or_else(
            || translate_palace(palace.name, lang),
            |p| translate_palace(*p, lang),
        );
        let natal_palace_name = translate_palace(palace.name, lang);

        if scope_palace_name == natal_palace_name {
            out.push_str(&format!("  {scope_palace_name}:\n"));
        } else {
            out.push_str(&format!("  {scope_palace_name} ({natal_palace_name}):\n"));
        }

        // 本命主星与辅星（杂耀在运限视角下噪声大，此处不展开）
        for (label, stars) in [
            (l.major_stars, &palace.major_stars),
            (l.minor_stars, &palace.minor_stars),
        ] {
            if let Some(text) = format_stars(stars, lang) {
                out.push_str(&format!("    {label}: {text}\n"));
            }
        }

        if let Some(stars) = item.stars.as_ref().and_then(|groups| groups.get(i))
            && let Some(text) = format_stars(stars, lang)
        {
            out.push_str(&format!("    {}: {text}\n", l.scope_stars));
        }

        if let Some(dec) = dec_star {
            out.push_str(&format!(
                "    {}: {}\n",
                l.twelve_gods,
                format_star_keys(&[dec.suiqian12[i], dec.jiangqian12[i]], lang)
            ));
        }
    }
    out
}

/// 运限的结构化文本
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

    // ---- 大限：落宫、四化、十二宫展开 ----
    out.push_str(&format!("--- {} ---\n", l.decadal_fortune));
    out.push_str(&format_scope_head(
        l.decadal_fortune,
        &l,
        &horoscope.decadal,
        astrolabe,
        lang,
    ));
    out.push_str(&format_mutagen_line(
        l.decadal_fortune,
        &l,
        &horoscope.decadal.mutagen,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.decadal,
        None,
        astrolabe,
        lang,
        &l,
    ));

    // ---- 小限：落宫、虚岁与该宫星耀 ----
    let age_palace = &astrolabe.palaces[horoscope.age.index];
    out.push_str(&format!(
        "\n{}: {}{}{} ({} {})\n",
        l.compound(l.age_fortune, l.soul_palace),
        l.natal,
        l.label_sep,
        translate_palace(age_palace.name, lang),
        l.nominal_age,
        horoscope.age.nominal_age,
    ));
    out.push_str(&format_palace_stars(age_palace, "  ", &l, lang));

    // ---- 流年：落宫、四化、十二宫展开（含岁前/将前十二神）----
    out.push_str(&format!("\n--- {} ---\n", l.yearly));
    out.push_str(&format_scope_head(
        l.yearly,
        &l,
        &horoscope.yearly.base,
        astrolabe,
        lang,
    ));
    out.push_str(&format_mutagen_line(
        l.yearly,
        &l,
        &horoscope.yearly.mutagen,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.yearly.base,
        Some(&horoscope.yearly.yearly_dec_star),
        astrolabe,
        lang,
        &l,
    ));

    // ---- 流月/流日/流时：只列落宫、重排宫名与四化，不逐宫展开星耀 ----
    for (label, item) in [
        (l.monthly, &horoscope.monthly),
        (l.daily, &horoscope.daily),
        (l.hourly, &horoscope.hourly),
    ] {
        out.push('\n');
        out.push_str(&format_scope_head(label, &l, item, astrolabe, lang));
        out.push_str(&format_palace_names_line(label, &l, item, lang));
        out.push_str(&format_mutagen_line(label, &l, &item.mutagen, lang));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::astro::horoscope::get_horoscope;
    use crate::data::types::*;

    fn chart(lang: Language) -> Astrolabe {
        by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            lang,
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn astrolabe_prompt_zh_cn_has_all_sections() {
        let prompt = astrolabe_to_prompt(&chart(Language::ZhCN), Language::ZhCN);

        assert!(prompt.contains("=== 基本信息 ==="));
        assert!(prompt.contains("=== 十二宫 ==="));
        assert!(prompt.contains("性别: "));
        assert!(prompt.contains("阳历: 2000-8-16"));
        assert!(prompt.contains("天干地支: "));
        assert!(prompt.contains("主星: "));
        assert!(prompt.contains("生年四化: "));
        assert!(prompt.contains("小限虚岁: "));
        assert!(prompt.contains("十二神: "));
        assert!(prompt.contains("[身宫]"));
        assert!(prompt.contains("[来因]"));
        // 宫位标题不留多余空格
        assert!(!prompt.contains("  ---"));
    }

    #[test]
    fn astrolabe_prompt_en_us_separates_compound_labels() {
        let prompt = astrolabe_to_prompt(&chart(Language::EnUS), Language::EnUS);

        assert!(prompt.contains("=== Palaces ==="));
        assert!(prompt.contains("Gender: "));
        assert!(prompt.contains("Major Stars: "));
        assert!(prompt.contains("Birth-Year Mutagen: "));
        assert!(prompt.contains("Twelve Gods: "));
    }

    #[test]
    fn horoscope_prompt_zh_cn_covers_every_scope() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let prompt = horoscope_to_prompt(&astrolabe, &horoscope, lang);

        assert!(prompt.contains("=== 运限 ==="));
        assert!(prompt.contains("大限命宫: 本命"));
        assert!(prompt.contains("大限四化: "));
        assert!(prompt.contains("小限命宫: 本命"));
        assert!(prompt.contains("流年命宫: 本命"));
        assert!(prompt.contains("流年四化: "));
        assert!(prompt.contains("流月命宫: 本命"));
        assert!(prompt.contains("流月宫名: "));
        assert!(prompt.contains("流日四化: "));
        assert!(prompt.contains("流时四化: "));
        // 流年逐宫带岁前/将前十二神
        assert!(prompt.contains("    十二神: "));
    }

    /// 拉丁字母语言的复合标签必须靠 `label_sep` 分词，
    /// 否则会写出 `Decadal FortuneMutagen` 这样的粘连标签。
    #[test]
    fn horoscope_prompt_en_us_separates_compound_labels() {
        let lang = Language::EnUS;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let prompt = horoscope_to_prompt(&astrolabe, &horoscope, lang);

        assert!(prompt.contains("=== Horoscope ==="));
        assert!(prompt.contains("Decadal Fortune Soul Palace: Natal "));
        assert!(prompt.contains("Decadal Fortune Mutagen: "));
        assert!(prompt.contains("Age Fortune Soul Palace: Natal "));
        assert!(prompt.contains("Yearly Mutagen: "));
        assert!(prompt.contains("Monthly Palace Names: "));
        assert!(!prompt.contains("FortuneMutagen"));
    }

    /// 六种语言都要有自己的结构标签，任一语言漏配都会退化成别的语言的文本。
    #[test]
    fn every_language_has_its_own_labels() {
        let cases = [
            (Language::ZhCN, "=== 基本信息 ===", "五行局: "),
            (Language::ZhTW, "=== 基本資訊 ===", "五行局: "),
            (
                Language::EnUS,
                "=== Basic Info ===",
                "Five Elements Class: ",
            ),
            (Language::JaJP, "=== 基本情報 ===", "五行局: "),
            (Language::KoKR, "=== 기본 정보 ===", "오행국: "),
            (Language::ViVN, "=== Thông tin cơ bản ===", "Cục ngũ hành: "),
        ];
        for (lang, header, five_elements) in cases {
            let prompt = astrolabe_to_prompt(&chart(lang), lang);
            assert!(prompt.contains(header), "{lang:?} 缺少基本信息标题");
            assert!(prompt.contains(five_elements), "{lang:?} 缺少五行局标签");
        }
    }

    /// 中日两种语言的复合标签直接相连，韩越与英文分词。
    #[test]
    fn label_sep_matches_language_convention() {
        for (lang, expected) in [
            (Language::ZhTW, "大限四化: "),
            (Language::JaJP, "大限四化: "),
            (Language::KoKR, "대한 사화: "),
            (Language::ViVN, "Đại hạn Tứ hóa: "),
        ] {
            let astrolabe = chart(lang);
            let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
            let prompt = horoscope_to_prompt(&astrolabe, &horoscope, lang);
            assert!(prompt.contains(expected), "{lang:?} 期望含 {expected:?}");
        }
    }
}

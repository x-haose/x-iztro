//! 语义化文本投影（to_text）
//!
//! 与 `to_json`（机器格式）相对：同一份盘面事实的自然语言形态，供大语言模型
//! 与人直接阅读。每行一个「标签: 取值」，宫位以 `--- 宫名 ---` 分段。
//! 星耀、宫名、干支等取值一律按传入语言翻译，结构标签取本模块的六语言标签表——
//! 三个语言绑定的文本都出自这里，同一张盘在三侧的输出逐字节一致。
//!
//! 覆盖对象：本命盘（[`astrolabe_to_text`]）、运限（[`horoscope_to_text`]）、
//! 格局命中（[`patterns_to_text`]）、单宫（[`palace_to_text`]）、
//! 三方四正（[`surrounded_palaces_to_text`]）。各对象另有同名 `to_text`
//! 便捷方法（[`Astrolabe::to_text`]、[`HoroscopeRef::to_text`] 等）。

use crate::data::constants::CHINESE_TIME;
use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::{HeavenlyStem, Language, Palace, Scope};
use crate::i18n::lookup::translate_key;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_mutagen, translate_palace,
    translate_pattern, translate_star,
};
use crate::models::astrolabe::{Astrolabe, PalaceRef};
use crate::models::horoscope::{HoroscopeData, HoroscopeItem, HoroscopeRef, YearlyDecStar};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::models::surpalaces::SurroundedPalaces;
use crate::pattern::{PatternConfig, PatternHit, patterns_at};

/// 列表型取值的分隔符：星耀列表、十二神、四化都用它连接
const LIST_SEP: &str = ", ";

/// 一颗星的展示写法：`名称(亮度)[四化]`，无亮度或无四化则省略对应部分。
///
/// 星名按 `key` 用传入语言重翻而非取 `star.name`——后者在排盘时已按排盘语言
/// 固化，直接用会在「文本语言 ≠ 排盘语言」时输出混排文本。
fn format_star(star: &Star, lang: Language) -> String {
    let mut s = translate_star(star.key, lang).to_string();
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

    // ---- 格局 ----
    sec_patterns: ["=== 格局 ===", "=== 格局 ===", "=== Patterns ===", "=== 格局 ===", "=== 격국 ===", "=== Cách cục ==="],
    patterns_word: ["格局", "格局", "Patterns", "格局", "격국", "Cách cục"],
    broken_mark: ["[破格]", "[破格]", "[Broken]", "[破格]", "[파격]", "[Phá cách]"],

    // ---- 三方四正 ----
    target_palace: ["本宫", "本宮", "Target Palace", "本宮", "본궁", "Cung gốc"],
    opposite_palace: ["对宫", "對宮", "Opposite Palace", "対宮", "대궁", "Cung đối"],
    wealth_palace: ["财帛位", "財帛位", "Wealth Palace", "財帛位", "재백위", "Vị Tài bạch"],
    career_palace: ["官禄位", "官祿位", "Career Palace", "官禄位", "관록위", "Vị Quan lộc"],

    // ---- 运限 ----
    sec_horoscope: ["=== 运限 ===", "=== 運限 ===", "=== Horoscope ===", "=== 運限 ===", "=== 운한 ===", "=== Vận hạn ==="],
    target_date: ["目标日期", "目標日期", "Target Date", "対象日", "대상 날짜", "Ngày mục tiêu"],
    decadal_fortune: ["大限", "大限", "Decadal Fortune", "大限", "대한", "Đại hạn"],
    childhood_fortune: ["童限", "童限", "Childhood Fortune", "童限", "동한", "Đồng hạn"],
    yearly: ["流年", "流年", "Yearly", "流年", "유년", "Lưu niên"],
    monthly: ["流月", "流月", "Monthly", "流月", "유월", "Lưu nguyệt"],
    daily: ["流日", "流日", "Daily", "流日", "유일", "Lưu nhật"],
    hourly: ["流时", "流時", "Hourly", "流時", "유시", "Lưu thời"],
    age_fortune: ["小限", "小限", "Age Fortune", "小限", "소한", "Tiểu hạn"],
    nominal_age: ["虚岁", "虛歲", "Nominal Age", "数え年", "세는나이", "Tuổi âm"],
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

    /// 运限层级的结构标签，按语言无关标识取；
    /// 未起运的大限层（童限）与大限是不同的解盘语义，标签随之不同
    fn scope_label(&self, name: crate::data::types::HoroscopeName) -> &'static str {
        use crate::data::types::HoroscopeName;
        match name {
            HoroscopeName::Decadal => self.decadal_fortune,
            HoroscopeName::Childhood => self.childhood_fortune,
            HoroscopeName::Age => self.age_fortune,
            HoroscopeName::Yearly => self.yearly,
            HoroscopeName::Monthly => self.monthly,
            HoroscopeName::Daily => self.daily,
            HoroscopeName::Hourly => self.hourly,
        }
    }
}

/// 四化星列表写成「星名+四化名」：入参按禄权科忌顺序（`mutagens_of` 与
/// 运限层级 `mutagen` 字段的固定顺序），逐星标注化名，读文本无须记顺序约定
fn format_mutagen_stars(stars: &[StarKey], lang: Language) -> String {
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

/// 生年四化：本命年干化出的四颗星，按禄权科忌顺序写成「星名+四化名」
fn format_birth_mutagen(astrolabe: &Astrolabe, stem: HeavenlyStem, lang: Language) -> String {
    format_mutagen_stars(&astrolabe.config.mutagens_of(stem), lang)
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

/// 单个宫位的完整文本块：标题、干支、大限区间、小限虚岁、十二神、三类星耀
fn format_palace_block(palace: &PalaceData, l: &Labels, lang: Language) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "--- {}{} ---\n",
        translate_palace(palace.name, lang),
        palace_markers(palace, l)
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
    out
}

/// 一条格局命中的展示写法：`- 名称(宫名) [破格]: 参与成格的星`，
/// 与运限层级格局行的「名称(宫名) [破格]」词序一致。
///
/// `palace_names` 是该判定视角下按宫位索引排列的十二宫名
/// （本命为 `Astrolabe.palaces[i].name`，运限为 `HoroscopeItem.palace_names`）；
/// 索引超出列表时省略宫名标注而不错标——错标的宫名比缺标注更误导解读。
fn format_pattern_hit(
    hit: &PatternHit,
    palace_names: &[Palace],
    l: &Labels,
    lang: Language,
) -> String {
    debug_assert_eq!(palace_names.len(), 12, "palace_names 应为十二宫全表");
    let palace_note = palace_names
        .get(hit.palace)
        .map(|p| format!("({})", translate_palace(*p, lang)))
        .unwrap_or_default();
    let broken = if hit.broken {
        format!(" {}", l.broken_mark)
    } else {
        String::new()
    };
    let stars = hit
        .stars
        .iter()
        .map(|s| {
            let mut t = translate_star(s.star, lang).to_string();
            if let Some(b) = s.brightness {
                t.push_str(&format!("({})", translate_brightness(b, lang)));
            }
            if let Some(m) = s.mutagen {
                t.push_str(&format!("[{}]", translate_mutagen(m, lang)));
            }
            t
        })
        .collect::<Vec<_>>()
        .join(LIST_SEP);
    format!(
        "- {}{}{}: {}\n",
        translate_pattern(hit.key, lang),
        palace_note,
        broken,
        stars,
    )
}

/// 格局命中列表的文本：每条一行；列表为空时返回空串。
///
/// `palace_names` 是判定视角下按宫位索引排列的十二宫名，
/// 与 [`crate::pattern::PatternHit::to_dto`] 的同名入参语义一致。
pub fn patterns_to_text(hits: &[PatternHit], palace_names: &[Palace], lang: Language) -> String {
    let l = labels(lang);
    hits.iter()
        .map(|h| format_pattern_hit(h, palace_names, &l, lang))
        .collect()
}

/// 单个宫位的文本：与本命盘文本中该宫的段落一致
pub fn palace_to_text(palace: &PalaceData, lang: Language) -> String {
    format_palace_block(palace, &labels(lang), lang)
}

/// 三方四正的文本：本宫、对宫、财帛位、官禄位各一段，列出宫名、干支与星耀
pub fn surrounded_palaces_to_text(sp: &SurroundedPalaces, lang: Language) -> String {
    let l = labels(lang);
    let mut out = String::new();
    for (role, palace) in [
        (l.target_palace, sp.target),
        (l.opposite_palace, sp.opposite),
        (l.wealth_palace, sp.wealth),
        (l.career_palace, sp.career),
    ] {
        out.push_str(&format!(
            "{}: {} ({}{})\n",
            role,
            translate_palace(palace.name, lang),
            translate_heavenly_stem(palace.heavenly_stem, lang),
            translate_earthly_branch(palace.earthly_branch, lang)
        ));
        out.push_str(&format_palace_stars(palace, "  ", &l, lang));
    }
    out
}

/// 本命盘的语义化文本
pub fn astrolabe_to_text(astrolabe: &Astrolabe, lang: Language) -> String {
    let l = labels(lang);
    let mut out = String::new();

    // ---- 基本信息 ----
    out.push_str(&format!("{}\n", l.sec_basic));
    let mut line = |label: &str, value: &str| out.push_str(&format!("{label}: {value}\n"));
    line(l.gender, translate_gender(astrolabe.gender, lang));
    line(l.solar_date, &astrolabe.solar_date);
    line(l.lunar_date, &astrolabe.lunar_date);
    // 干支四柱由枚举按传入语言现翻（模型上的展示串是排盘语言的固化值）
    let cd = &astrolabe.raw_dates.chinese_date;
    line(
        l.chinese_date,
        &crate::utils::translate_chinese_date([cd.yearly, cd.monthly, cd.daily, cd.hourly], lang),
    );
    // 时辰/星座/生肖按语义 key 用传入语言重翻（模型上的译文是排盘语言的固化值，
    // 直接用会在「文本语言 ≠ 排盘语言」时混排）；key 必在词表内，查不到即库内缺陷
    let by_key = |key: &str| {
        translate_key(key, lang)
            .map(str::to_string)
            .unwrap_or_else(|| key.to_string())
    };
    line(
        l.time,
        &format!(
            "{} ({})",
            by_key(CHINESE_TIME[astrolabe.time_index as usize]),
            astrolabe.time_range
        ),
    );
    line(l.zodiac_sign, &by_key(&astrolabe.sign_key));
    line(l.zodiac_animal, &by_key(&astrolabe.zodiac_key));
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
        out.push('\n');
        out.push_str(&format_palace_block(palace, &l, lang));
    }

    // ---- 格局（默认口径的本命命中；无命中时整节省略） ----
    let hits = astrolabe.patterns();
    if !hits.is_empty() {
        let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
        out.push_str(&format!("\n{}\n", l.sec_patterns));
        out.push_str(&patterns_to_text(&hits, &names, lang));
    }

    out
}

/// 某运限层级的四化行，如「大限四化: 天梁禄, 紫微权, 左辅科, 武曲忌」
fn format_mutagen_line(scope: &str, l: &Labels, keys: &[StarKey], lang: Language) -> String {
    format!(
        "  {}: {}\n",
        l.compound(scope, l.mutagen_fly),
        format_mutagen_stars(keys, lang)
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

/// 某运限层级的流耀行：每颗流耀标注它落在该层级的哪一宫，如「流月流耀: 流魁(命宫)」；
/// 该层级无流耀时返回空串
fn format_scope_stars_line(
    scope: &str,
    l: &Labels,
    item: &HoroscopeItem,
    lang: Language,
) -> String {
    let Some(groups) = item.stars.as_ref() else {
        return String::new();
    };
    let entries: Vec<String> = groups
        .iter()
        .enumerate()
        .flat_map(|(i, stars)| {
            let palace = item.palace_names.get(i).copied();
            stars.iter().map(move |s| {
                let mut t = translate_star(s.key, lang).to_string();
                if let Some(p) = palace {
                    t.push_str(&format!("({})", translate_palace(p, lang)));
                }
                t
            })
        })
        .collect();
    if entries.is_empty() {
        return String::new();
    }
    format!(
        "  {}: {}\n",
        l.compound(scope, l.scope_stars),
        entries.join(LIST_SEP)
    )
}

/// 某运限层级的格局行：该视角命中的格局名，标注成格宫位（该视角宫名）、
/// 破格带标记；无命中时返回空串。
///
/// 同一格局的多个口径命中（`variant` 不同、名称与宫位相同）在概览行里
/// 渲染结果相同，此处去重——口径明细走 DTO 或 [`patterns_to_text`] 查看。
fn format_scope_patterns_line(
    scope_label: &str,
    l: &Labels,
    hits: &[PatternHit],
    palace_names: &[Palace],
    lang: Language,
) -> String {
    if hits.is_empty() {
        return String::new();
    }
    let mut entries: Vec<String> = Vec::new();
    for h in hits {
        let mut t = translate_pattern(h.key, lang).to_string();
        if let Some(p) = palace_names.get(h.palace) {
            t.push_str(&format!("({})", translate_palace(*p, lang)));
        }
        if h.broken {
            t.push_str(&format!(" {}", l.broken_mark));
        }
        if !entries.contains(&t) {
            entries.push(t);
        }
    }
    format!(
        "  {}: {}\n",
        l.compound(scope_label, l.patterns_word),
        entries.join(LIST_SEP)
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

/// 运限的语义化文本
pub fn horoscope_to_text(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    lang: Language,
) -> String {
    let l = labels(lang);
    let config = PatternConfig::default();
    // 各层级的格局行按该层视角判定（小限不在 Scope 之列，无格局行）
    let scope_patterns = |scope: Scope| patterns_at(astrolabe, horoscope, scope, &config);
    let mut out = String::new();

    out.push_str(&format!("{}\n", l.sec_horoscope));
    out.push_str(&format!(
        "{}: {} / {}\n\n",
        l.target_date, horoscope.solar_date, horoscope.lunar_date
    ));

    // ---- 大限：落宫、四化、格局、十二宫展开 ----
    //      层级标签按 name_key 取：未起运时即「童限」，与大限是不同的解盘语义
    let decadal_label = l.scope_label(horoscope.decadal.name_key);
    out.push_str(&format!("--- {decadal_label} ---\n"));
    out.push_str(&format_scope_head(
        decadal_label,
        &l,
        &horoscope.decadal,
        astrolabe,
        lang,
    ));
    out.push_str(&format_mutagen_line(
        decadal_label,
        &l,
        &horoscope.decadal.mutagen,
        lang,
    ));
    out.push_str(&format_scope_patterns_line(
        decadal_label,
        &l,
        &scope_patterns(Scope::Decadal),
        &horoscope.decadal.palace_names,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.decadal,
        None,
        astrolabe,
        lang,
        &l,
    ));

    // ---- 小限：落宫、虚岁、宫名、四化与该宫星耀 ----
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
    out.push_str(&format_palace_names_line(
        l.age_fortune,
        &l,
        &horoscope.age.base,
        lang,
    ));
    out.push_str(&format_mutagen_line(
        l.age_fortune,
        &l,
        &horoscope.age.mutagen,
        lang,
    ));
    out.push_str(&format_palace_stars(age_palace, "  ", &l, lang));

    // ---- 流年：落宫、四化、格局、十二宫展开（含岁前/将前十二神）----
    let yearly_label = l.scope_label(horoscope.yearly.name_key);
    out.push_str(&format!("\n--- {yearly_label} ---\n"));
    out.push_str(&format_scope_head(
        yearly_label,
        &l,
        &horoscope.yearly.base,
        astrolabe,
        lang,
    ));
    out.push_str(&format_mutagen_line(
        yearly_label,
        &l,
        &horoscope.yearly.mutagen,
        lang,
    ));
    out.push_str(&format_scope_patterns_line(
        yearly_label,
        &l,
        &scope_patterns(Scope::Yearly),
        &horoscope.yearly.palace_names,
        lang,
    ));
    out.push_str(&format_scope_palaces(
        &horoscope.yearly.base,
        Some(&horoscope.yearly.yearly_dec_star),
        astrolabe,
        lang,
        &l,
    ));

    // ---- 流月/流日/流时：落宫、重排宫名、四化、流耀与格局，不逐宫展开星耀 ----
    for (item, scope) in [
        (&horoscope.monthly, Scope::Monthly),
        (&horoscope.daily, Scope::Daily),
        (&horoscope.hourly, Scope::Hourly),
    ] {
        let label = l.scope_label(item.name_key);
        out.push('\n');
        out.push_str(&format_scope_head(label, &l, item, astrolabe, lang));
        out.push_str(&format_palace_names_line(label, &l, item, lang));
        out.push_str(&format_mutagen_line(label, &l, &item.mutagen, lang));
        out.push_str(&format_scope_stars_line(label, &l, item, lang));
        out.push_str(&format_scope_patterns_line(
            label,
            &l,
            &scope_patterns(scope),
            &item.palace_names,
            lang,
        ));
    }

    out
}

impl Astrolabe {
    /// 本命盘的语义化文本（按排盘语言）；[`astrolabe_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        astrolabe_to_text(self, self.language)
    }
}

impl HoroscopeRef<'_> {
    /// 运限的语义化文本（按星盘排盘语言）；[`horoscope_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        horoscope_to_text(self.astrolabe(), self.data(), self.astrolabe().language)
    }
}

impl PalaceRef<'_> {
    /// 本宫的语义化文本（按星盘排盘语言）；[`palace_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        palace_to_text(self, self.astrolabe().language)
    }
}

impl SurroundedPalaces<'_> {
    /// 三方四正的语义化文本；[`surrounded_palaces_to_text`] 的便捷形态
    pub fn to_text(&self, lang: Language) -> String {
        surrounded_palaces_to_text(self, lang)
    }
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
    fn astrolabe_text_zh_cn_has_all_sections() {
        let text = chart(Language::ZhCN).to_text();

        assert!(text.contains("=== 基本信息 ==="));
        assert!(text.contains("=== 十二宫 ==="));
        assert!(text.contains("性别: "));
        assert!(text.contains("阳历: 2000-8-16"));
        assert!(text.contains("天干地支: "));
        assert!(text.contains("主星: "));
        assert!(text.contains("生年四化: "));
        assert!(text.contains("小限虚岁: "));
        assert!(text.contains("十二神: "));
        assert!(text.contains("[身宫]"));
        assert!(text.contains("[来因]"));
        // 宫位标题不留多余空格
        assert!(!text.contains("  ---"));
    }

    #[test]
    fn astrolabe_text_en_us_separates_compound_labels() {
        let text = chart(Language::EnUS).to_text();

        assert!(text.contains("=== Palaces ==="));
        assert!(text.contains("Gender: "));
        assert!(text.contains("Major Stars: "));
        assert!(text.contains("Birth-Year Mutagen: "));
        assert!(text.contains("Twelve Gods: "));
    }

    /// 本命文本带格局节：有命中的盘列出格局，格局行含所在宫名
    #[test]
    fn astrolabe_text_includes_patterns_section() {
        let astrolabe = chart(Language::ZhCN);
        let hits = astrolabe.patterns();
        let text = astrolabe.to_text();
        if hits.is_empty() {
            assert!(!text.contains("=== 格局 ==="));
        } else {
            assert!(text.contains("=== 格局 ==="));
            let first = crate::i18n::translate_pattern(hits[0].key, Language::ZhCN);
            assert!(text.contains(first));
        }
    }

    #[test]
    fn horoscope_text_zh_cn_covers_every_scope() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);

        assert!(text.contains("=== 运限 ==="));
        assert!(text.contains("大限命宫: 本命"));
        assert!(text.contains("大限四化: "));
        assert!(text.contains("小限命宫: 本命"));
        assert!(text.contains("小限宫名: "));
        assert!(text.contains("小限四化: "));
        assert!(text.contains("流年命宫: 本命"));
        assert!(text.contains("流年四化: "));
        assert!(text.contains("流月命宫: 本命"));
        assert!(text.contains("流月宫名: "));
        assert!(text.contains("流日四化: "));
        assert!(text.contains("流时四化: "));
        // 流年逐宫带岁前/将前十二神
        assert!(text.contains("    十二神: "));
        // 流月/流日/流时的流耀带落宫标注
        assert!(text.contains("流月流耀: "));
        assert!(text.contains("流日流耀: "));
        assert!(text.contains("流时流耀: "));
    }

    /// 未起运的盘（童限），大限段的标题与标签全部写「童限」而非「大限」
    #[test]
    fn horoscope_text_uses_childhood_label_before_decadal_start() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        // 2001 年目标日期，命主 2000 年生，尚未起运，大限层为童限
        let horoscope = get_horoscope(&astrolabe, "2001-10-1", 0, lang).unwrap();
        assert_eq!(horoscope.decadal.name_key, HoroscopeName::Childhood);
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);
        assert!(text.contains("--- 童限 ---"));
        assert!(text.contains("童限命宫: 本命"));
        assert!(text.contains("童限四化: "));
        assert!(!text.contains("--- 大限 ---"));
    }

    /// 拉丁字母语言的复合标签必须靠 `label_sep` 分词，
    /// 否则会写出 `Decadal FortuneMutagen` 这样的粘连标签。
    #[test]
    fn horoscope_text_en_us_separates_compound_labels() {
        let lang = Language::EnUS;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);

        assert!(text.contains("=== Horoscope ==="));
        assert!(text.contains("Decadal Fortune Soul Palace: Natal "));
        assert!(text.contains("Decadal Fortune Mutagen: "));
        assert!(text.contains("Age Fortune Soul Palace: Natal "));
        assert!(text.contains("Yearly Mutagen: "));
        assert!(text.contains("Monthly Palace Names: "));
        assert!(!text.contains("FortuneMutagen"));
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
            let text = chart(lang).to_text();
            assert!(text.contains(header), "{lang:?} 缺少基本信息标题");
            assert!(text.contains(five_elements), "{lang:?} 缺少五行局标签");
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
            let text = horoscope_to_text(&astrolabe, &horoscope, lang);
            assert!(text.contains(expected), "{lang:?} 期望含 {expected:?}");
        }
    }

    /// 自由函数传任意语言都输出纯该语言文本：zh-CN 盘按 en-US 渲染，
    /// 与原生 en-US 盘的渲染逐字节一致（星名、时辰、星座、干支、流耀都按 key 重翻，
    /// 不许出现「文本语言 ≠ 排盘语言」时的混排）。
    #[test]
    fn free_functions_render_any_language_without_mixing() {
        let zh = chart(Language::ZhCN);
        let en = chart(Language::EnUS);
        assert_eq!(
            astrolabe_to_text(&zh, Language::EnUS),
            astrolabe_to_text(&en, Language::EnUS)
        );
        let hz = get_horoscope(&zh, "2024-10-1", 0, Language::ZhCN).unwrap();
        let he = get_horoscope(&en, "2024-10-1", 0, Language::EnUS).unwrap();
        assert_eq!(
            horoscope_to_text(&zh, &hz, Language::EnUS),
            horoscope_to_text(&en, &he, Language::EnUS)
        );
        let sp_zh = zh.surrounded_palaces(Palace::Soul).unwrap();
        let sp_en = en.surrounded_palaces(Palace::Soul).unwrap();
        assert_eq!(sp_zh.to_text(Language::EnUS), sp_en.to_text(Language::EnUS));
    }

    /// 单宫文本与本命盘文本中该宫的段落一致
    #[test]
    fn palace_text_matches_astrolabe_section() {
        let astrolabe = chart(Language::ZhCN);
        let palace_text = palace_to_text(&astrolabe.palaces[0], Language::ZhCN);
        assert!(palace_text.starts_with("--- "));
        assert!(astrolabe.to_text().contains(&palace_text));
    }

    /// 三方四正文本列出四个角色宫位
    #[test]
    fn surrounded_palaces_text_lists_four_roles() {
        let astrolabe = chart(Language::ZhCN);
        let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();
        let text = sp.to_text(Language::ZhCN);
        assert!(text.contains("本宫: 命宫"));
        assert!(text.contains("对宫: "));
        assert!(text.contains("财帛位: "));
        assert!(text.contains("官禄位: "));
    }
}

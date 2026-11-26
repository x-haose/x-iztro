//! 语义化文本投影（to_text）
//!
//! 与 `to_json`（机器格式）相对：同一份盘面事实的自然语言形态，供大语言模型
//! 与人直接阅读。输出是可直接阅读的 Markdown 子集——只用 `#` 标题、`- ` 列表、
//! `**粗体**` 与一张窄表，不渲染时源码同样可读。结构标签取本模块的六语言标签表，
//! 星耀、宫名、干支等取值按传入语言现翻；三个语言绑定的文本都出自这里，
//! 同一张盘在三侧的输出逐字节一致。
//!
//! 覆盖对象：本命盘（[`astrolabe_to_text`]）、运限（[`horoscope_to_text`]）、
//! 格局命中（[`patterns_to_text`]）、单宫（[`palace_to_text`]）、
//! 三方四正（[`surrounded_palaces_to_text`]）、夹宫、大限一览、流年一览、流月一览。各对象另有同名 `to_text` 便捷方法。
//! `*_to_text_with` 收 [`TextOptions`]，两个开关互相独立：[`TextDetail`] 管事实写到多细，
//! [`KnowledgeDepth`] 管释义写多长；运限文本另有 [`TextOptions::horoscope_through`] 管写到哪一层。释义随段落而不随表格——宫位段落之后紧跟该宫星耀的释义，
//! 格局列表之后紧跟格局释义，运限各层级段之后紧跟流耀与格局释义，本命文末附四化释义，
//! 事实与解读同处一屏。

use crate::astro::builder::parse_solar_date;
use crate::astro::topic::{
    TopicGroupKind, TopicRole, TopicSelection, layers_through, select as select_topic,
};
use crate::data::constants::CHINESE_TIME;
use crate::data::stars::{MUTAGEN, StarKey};
use crate::data::types::{HOROSCOPE_SCOPES, Language, Mutagen, Palace, Scope};
use crate::error::IztroError;
use crate::i18n::lookup::translate_key;
use crate::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_gender, translate_heavenly_stem, translate_lunar_date_body, translate_mutagen,
    translate_palace, translate_pattern, translate_star,
};
use crate::knowledge::{KnowledgePack, TopicEntry};
use crate::models::astrolabe::{Astrolabe, PalaceRef};
use crate::models::flanking::FlankingPalaces;
use crate::models::horoscope::{HoroscopeData, HoroscopeItem, HoroscopeRef};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::models::surpalaces::SurroundedPalaces;
use crate::pattern::{PatternConfig, PatternHit, PatternKey, patterns_through};
use crate::utils::fix_index;

/// 列表型取值的分隔符：星耀列表、十二神、四化都用它连接
const LIST_SEP: &str = ", ";
/// 同一行内多个字段的分隔符
const FIELD_SEP: &str = " · ";
/// 四化落宫、流耀落宫的指向符
const ARROW: &str = "→";
/// 空列表的占位（表格单元格不能留空）
const NONE_MARK: &str = "—";

/// 事实写到多细。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextDetail {
    /// 总览：本命只写基本信息、十二宫总览表与格局，不写十二宫详解；运限各层（小限除外）
    /// 写宫名对照、四化、流耀与格局，不展开十二宫表；宫位段落（单宫、三方四正、夹宫）
    /// 只写主星、辅星与杂耀。格局与大限一览本身就是总览，不受影响。
    Overview,
    /// 完整：全部事实
    #[default]
    Full,
}

crate::data::keyed::keyed_enum!(TextDetail, "text detail" {
    Overview => "overview",
    Full => "full",
});

/// 释义写多长（带知识包时）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KnowledgeDepth {
    /// 摘要：条目的摘要，或本身不超过 [`crate::knowledge::SUMMARY_FALLBACK_WIDTH`] 的正文；
    /// 格局不附成立条件
    Summary,
    /// 完整：正文全文，格局附成立条件
    #[default]
    Full,
}

crate::data::keyed::keyed_enum!(KnowledgeDepth, "knowledge depth" {
    Summary => "summary",
    Full => "full",
});

/// 文本投影的选项：事实详略、释义材料与深度、格局判定口径；一切影响输出内容的开关都收在这里。
#[derive(Debug, Clone, Copy, Default)]
pub struct TextOptions<'a> {
    /// 事实写到多细
    detail: TextDetail,
    /// 释义材料来源；`None` 即只输出事实
    knowledge: Option<&'a KnowledgePack>,
    /// 释义深度；只有经 [`TextOptions::knowledge_summary`] 校验过的包才会配上 `Summary`
    depth: KnowledgeDepth,
    /// 格局判定口径；`None` 取 [`PatternConfig::default`]
    pattern_config: Option<&'a PatternConfig>,
    /// 运限文本写到哪一层为止；`None` 时自由函数写到流时，[`HoroscopeRef`] 的便捷方法写到查询精度
    through: Option<Scope>,
}

impl<'a> TextOptions<'a> {
    /// 完整事实、不带释义、默认格局口径。
    pub fn new() -> Self {
        Self::default()
    }

    /// 事实写到多细，见 [`TextDetail`]。
    pub fn detail(mut self, detail: TextDetail) -> Self {
        self.detail = detail;
        self
    }

    /// 按盘从 `pack` 取释义全文：宫位段落之后附该宫星耀的释义、格局之后附格局释义与成立条件、
    /// 本命文末附四化释义。
    pub fn knowledge(mut self, pack: &'a KnowledgePack) -> Self {
        self.knowledge = Some(pack);
        self.depth = KnowledgeDepth::Full;
        self
    }

    /// 按盘从 `pack` 取摘要层释义，附着位置同 [`Self::knowledge`]。
    ///
    /// # Errors
    /// 包里有正文超过 [`crate::knowledge::SUMMARY_FALLBACK_WIDTH`] 却没有摘要的条目时返回
    /// [`IztroError::InvalidArgument`]，列出全部缺项（[`KnowledgePack::missing_summaries`]）。
    pub fn knowledge_summary(mut self, pack: &'a KnowledgePack) -> Result<Self, IztroError> {
        let missing = pack.missing_summaries();
        if !missing.is_empty() {
            return Err(IztroError::InvalidArgument(format!(
                "knowledge pack '{}' cannot serve the summary layer: no summary for {}",
                pack.id,
                missing.join(", ")
            )));
        }
        self.knowledge = Some(pack);
        self.depth = KnowledgeDepth::Summary;
        Ok(self)
    }

    /// 格局按 `config` 口径判定；与 [`Astrolabe::patterns_with`] 及按盘取材的子包配同一口径。
    pub fn pattern_config(mut self, config: &'a PatternConfig) -> Self {
        self.pattern_config = Some(config);
        self
    }

    /// 运限文本写到 `scope` 这一层为止，更细的层级不写：大限只写大限；流年写大限、小限、流年；
    /// 流月、流日、流时依次各多一层。只对运限文本起作用。不设时 [`HoroscopeRef::to_text_with`]
    /// 写到查询精度（[`HoroscopeRef::precision`]），自由函数 [`horoscope_to_text_with`] 写到流时。
    ///
    /// 只写到大限时，大限段不列要读小限与流年才判得出的格局（[`PatternKey::needs_finer_layer`]），
    /// 同一大限内各年的大限段因此相同。
    ///
    /// # Errors
    /// `scope` 为 [`Scope::Origin`] 时返回 [`IztroError::InvalidArgument`]：本命不是运限层级。
    pub fn horoscope_through(mut self, scope: Scope) -> Result<Self, IztroError> {
        if scope == Scope::Origin {
            return Err(IztroError::InvalidArgument(
                "horoscope text cannot stop at 'origin': expected decadal, yearly, monthly, daily or hourly"
                    .to_string(),
            ));
        }
        self.through = Some(scope);
        Ok(self)
    }

    /// 运限文本写到的最后一层；不设即流时。
    fn through(&self) -> Scope {
        self.through.unwrap_or(Scope::Hourly)
    }

    /// 运限文本是否写 `scope` 这一层。
    fn writes_scope(&self, scope: Scope) -> bool {
        scope.depth() <= self.through().depth()
    }

    /// 当前的释义材料来源；`None` 即只输出事实。
    pub fn knowledge_pack(&self) -> Option<&'a KnowledgePack> {
        self.knowledge
    }

    /// 生效的格局判定口径。
    fn effective_pattern_config(&self) -> PatternConfig {
        self.pattern_config.cloned().unwrap_or_default()
    }

    /// 带释义时的取材口；不带知识包为 `None`
    fn notes(&self) -> Option<Notes<'a>> {
        self.knowledge.map(|pack| Notes {
            pack,
            depth: self.depth,
        })
    }
}

/// 按释义深度从知识包取各类条目的文本：完整层取正文，摘要层取摘要或不超过上限的正文。
/// 摘要层的包已在 [`TextOptions::knowledge_summary`] 校验过，`None` 只表示条目没有正文。
#[derive(Clone, Copy)]
struct Notes<'p> {
    pack: &'p KnowledgePack,
    depth: KnowledgeDepth,
}

impl<'p> Notes<'p> {
    /// 星耀的解读
    fn star(&self, key: StarKey) -> Option<&'p str> {
        let e = self.pack.star(key)?;
        match self.depth {
            KnowledgeDepth::Full => e.intro.as_deref(),
            KnowledgeDepth::Summary => e.brief(),
        }
    }

    /// 同宫两颗主星的组合解读；包里只记在一方名下也取得到
    fn combination(&self, a: StarKey, b: StarKey) -> Option<&'p str> {
        let one_side = |x: StarKey, y: StarKey| {
            let e = self.pack.star(x)?;
            match self.depth {
                KnowledgeDepth::Full => e.combinations.get(y.as_key()).map(String::as_str),
                KnowledgeDepth::Summary => e.combination_brief(y.as_key()),
            }
        };
        one_side(a, b).or_else(|| one_side(b, a))
    }

    /// 格局的解读与成立条件；成立条件只在完整层给出
    fn pattern(&self, key: PatternKey) -> (Option<&'p str>, Option<&'p str>) {
        let Some(e) = self.pack.pattern(key) else {
            return (None, None);
        };
        match self.depth {
            KnowledgeDepth::Full => (e.intro.as_deref(), e.conditions.as_deref()),
            KnowledgeDepth::Summary => (e.brief(), None),
        }
    }

    /// 四化的含义
    fn mutagen(&self, m: Mutagen) -> Option<&'p str> {
        let e = self.pack.mutagen(m)?;
        match self.depth {
            KnowledgeDepth::Full => e.intro.as_deref(),
            KnowledgeDepth::Summary => e.brief(),
        }
    }
}

/// 定义结构标签表并生成按语言取值的 `labels`。
///
/// 每个标签一行，六列依次为 zh-CN、zh-TW、en-US、ja-JP、ko-KR、vi-VN，
/// 六种语言的写法并排可见，增删标签不会漏掉某一种语言。标签只放词，
/// Markdown 记号与括号一律由代码拼接。带 `{}` 的标签是模板，占位处填译名。
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

        /// 六列全部字面量，供守护测试扫描
        #[cfg(test)]
        fn all_label_literals() -> Vec<&'static str> {
            vec![$($zh_cn, $zh_tw, $en_us, $ja_jp, $ko_kr, $vi_vn,)+]
        }
    };
}

labels_table! {
    // ---- 文档 ----
    doc_chart: ["命盘", "命盤", "Natal Chart", "命盤", "명반", "Lá số"],
    doc_horoscope: ["运限", "運限", "Horoscope", "運限", "운한", "Vận hạn"],
    sec_basic: ["基本信息", "基本資訊", "Basic Info", "基本情報", "기본 정보", "Thông tin cơ bản"],
    sec_overview: ["十二宫总览", "十二宮總覽", "Palace Overview", "十二宮概要", "십이궁 개요", "Tổng quan mười hai cung"],
    sec_palaces: ["十二宫", "十二宮", "Palaces", "十二宮", "십이궁", "Mười hai cung"],
    sec_patterns: ["格局", "格局", "Patterns", "格局", "격국", "Cách cục"],
    sec_mutagen_notes: ["四化释义", "四化釋義", "Mutagen Notes", "四化解説", "사화 해설", "Giải nghĩa tứ hóa"],
    sec_star_notes: ["星耀释义", "星曜釋義", "Star Notes", "星曜解説", "성요 해설", "Giải nghĩa tinh diệu"],

    // ---- 基本信息 ----
    solar_date: ["阳历", "陽曆", "Solar", "新暦", "양력", "Dương lịch"],
    lunar_date: ["农历", "農曆", "Lunar", "旧暦", "음력", "Âm lịch"],
    chinese_date: ["四柱", "四柱", "Pillars", "四柱", "사주", "Tứ trụ"],
    time: ["时辰", "時辰", "Hour", "時辰", "시진", "Giờ"],
    zodiac_sign: ["星座", "星座", "Sign", "星座", "별자리", "Cung hoàng đạo"],
    zodiac_animal: ["生肖", "生肖", "Zodiac", "干支動物", "띠", "Con giáp"],
    soul_star: ["命主", "命主", "Soul Star", "命主", "명주", "Mệnh chủ"],
    body_star: ["身主", "身主", "Body Star", "身主", "신주", "Thân chủ"],
    five_elements_class: ["五行局", "五行局", "Five Elements Class", "五行局", "오행국", "Cục ngũ hành"],
    birth_mutagen: ["生年四化", "生年四化", "Birth-Year Mutagen", "生年四化", "생년사화", "Tứ hóa sinh niên"],
    soul_palace: ["命宫", "命宮", "Soul Palace", "命宮", "명궁", "Cung Mệnh"],
    body_palace: ["身宫", "身宮", "Body Palace", "身宮", "신궁", "Cung Thân"],
    original_palace: ["来因宫", "來因宮", "Original Palace", "来因宮", "래인궁", "Cung Lai Nhân"],
    secondary_original_palace: ["辅来因宫", "輔來因宮", "Secondary Original Palace", "副来因宮", "보조 래인궁", "Cung Lai Nhân phụ"],

    // ---- 宫位 ----
    col_palace: ["宫位", "宮位", "Palace", "宮位", "궁위", "Cung"],
    decadal: ["大限", "大限", "Decadal", "大限", "대한", "Đại hạn"],
    ages: ["小限虚岁", "小限虛歲", "Age Fortune Years", "小限数え年", "소한 나이", "Tuổi tiểu hạn"],
    major_stars: ["主星", "主星", "Major Stars", "主星", "주성", "Chính tinh"],
    minor_stars: ["辅星", "輔星", "Minor Stars", "輔星", "보성", "Phụ tinh"],
    adjective_stars: ["杂耀", "雜曜", "Adjective Stars", "雑曜", "잡요", "Tạp diệu"],
    empty_palace: ["空宫", "空宮", "Empty", "空宮", "공궁", "Cung trống"],
    surround: ["三方四正", "三方四正", "Trine & Opposite", "三方四正", "삼방사정", "Tam phương tứ chính"],
    opposite: ["对宫", "對宮", "Opposite", "対宮", "대궁", "Đối cung"],
    trine: ["三合", "三合", "Trine", "三合", "삼합", "Tam hợp"],
    // 三方四正文本里各宫的角色
    target_palace: ["本宫", "本宮", "Target Palace", "本宮", "본궁", "Cung gốc"],
    opposite_palace: ["对宫", "對宮", "Opposite Palace", "対宮", "대궁", "Cung đối"],
    wealth_palace: ["财帛位", "財帛位", "Wealth Palace", "財帛位", "재백위", "Vị Tài bạch"],
    career_palace: ["官禄位", "官祿位", "Career Palace", "官禄位", "관록위", "Vị Quan lộc"],
    flanking: ["夹宫", "夾宮", "Flanking Palaces", "挟宮", "협궁", "Cung kẹp"],
    // 主题文本里只取本宫的那组宫位
    related_palaces: ["相关宫位", "相關宮位", "Related Palaces", "関連宮位", "관련 궁위", "Cung liên quan"],
    previous_palace: ["前宫", "前宮", "Previous Palace", "前宮", "전궁", "Cung trước"],
    next_palace: ["后宫", "後宮", "Next Palace", "後宮", "후궁", "Cung sau"],
    doc_decadals: ["大限一览", "大限一覽", "Decadal Timeline", "大限一覧", "대한 일람", "Tổng quan Đại hạn"],
    col_decadal_no: ["序", "序", "No", "序", "순", "STT"],
    col_natal_palace: ["本命宫", "本命宮", "Natal Palace", "本命宮", "본명궁", "Cung bản mệnh"],
    col_stem_branch: ["干支", "干支", "Stem-Branch", "干支", "간지", "Can chi"],
    col_age_range: ["虚岁", "虛歲", "Nominal Age", "数え年", "나이", "Tuổi ta"],
    col_year_range: ["年份", "年份", "Years", "年", "연도", "Năm"],
    doc_monthlies: ["流月一览", "流月一覽", "Monthly Timeline", "流月一覧", "유월 일람", "Tổng quan Lưu nguyệt"],
    doc_yearlies: ["流年一览", "流年一覽", "Yearly Timeline", "流年一覧", "유년 일람", "Tổng quan Lưu niên"],
    col_new_year: ["正月初一", "正月初一", "Lunar New Year", "旧正月", "설날", "Tết"],
    col_year: ["年份", "年份", "Year", "年", "연도", "Năm"],
    col_lunar_month: ["农历月", "農曆月", "Lunar Month", "旧暦月", "음력 월", "Tháng âm lịch"],
    leap_month: ["闰", "閏", "leap", "閏", "윤", "nhuận"],
    // 宫干飞化行的标签模板，`{}` 处填宫干
    stem_flying: ["宫干{}飞化", "宮干{}飛化", "Stem {} Flying", "宮干{}飛化", "궁간 {} 비화", "Can cung {} phi hóa"],
    twelve_gods: ["十二神", "十二神", "Twelve Gods", "十二神", "십이신", "Mười hai thần"],
    changsheng: ["长生", "長生", "Changsheng", "長生", "장생", "Trường sinh"],
    boshi: ["博士", "博士", "Boshi", "博士", "박사", "Bác sĩ"],
    suiqian: ["岁前", "歲前", "Suiqian", "歳前", "세전", "Tuế tiền"],
    jiangqian: ["将前", "將前", "Jiangqian", "将前", "장전", "Tướng tiền"],

    // ---- 星耀写法 ----
    // 四化标记模板，`{}` 处填四化译名：中文写「化禄」与禄存区分，英文译名为字母故加方括号
    mutagen_mark: ["化{}", "化{}", "[{}]", "化{}", "화{}", "hóa {}"],

    // ---- 格局 ----
    broken: ["破格", "破格", "broken", "破格", "파격", "phá cách"],
    conditions: ["成立条件", "成立條件", "Conditions", "成立条件", "성립 조건", "Điều kiện"],

    // ---- 释义 ----
    same_palace: ["同宫", "同宮", "same palace", "同宮", "동궁", "đồng cung"],

    // ---- 运限 ----
    decadal_fortune: ["大限", "大限", "Decadal Fortune", "大限", "대한", "Đại hạn"],
    childhood_fortune: ["童限", "童限", "Childhood Fortune", "童限", "동한", "Đồng hạn"],
    yearly: ["流年", "流年", "Yearly", "流年", "유년", "Lưu niên"],
    monthly: ["流月", "流月", "Monthly", "流月", "유월", "Lưu nguyệt"],
    daily: ["流日", "流日", "Daily", "流日", "유일", "Lưu nhật"],
    hourly: ["流时", "流時", "Hourly", "流時", "유시", "Lưu thời"],
    age_fortune: ["小限", "小限", "Age Fortune", "小限", "소한", "Tiểu hạn"],
    nominal_age: ["虚岁", "虛歲", "Nominal Age", "数え年", "세는나이", "Tuổi âm"],
    mutagen_fly: ["四化", "四化", "Mutagen", "四化", "사화", "Tứ hóa"],
    // 四化行注明所用天干的模板，`{}` 处填天干
    mutagen_stem: ["{}干", "{}干", "Stem {}", "{}干", "{}간", "Can {}"],
    scope_stars: ["流耀", "流曜", "Scope Stars", "流曜", "유요", "Lưu diệu"],
    natal: ["本命", "本命", "Natal", "本命", "본명", "Bản mệnh"],
    palace_names: ["宫名", "宮名", "Palace Names", "宮名", "궁명", "Tên cung"],

    // 复合标签的分隔符：中文与日文的汉字词直接相连（「本命夫妻」），
    // 拉丁字母与韩文须以空格分词（Natal Spouse、「본명 부처」）
    label_sep: ["", "", " ", "", " ", " "],
}

impl Labels {
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

// ============================================================
// 基础写法
// ============================================================

/// 四化标记：中文「化禄」、英文「\[A\]」
fn mutagen_mark(l: &Labels, m: Mutagen, lang: Language) -> String {
    l.mutagen_mark.replace("{}", translate_mutagen(m, lang))
}

/// 一颗星的写法：`名称(亮度)化X`，无亮度或无四化则省略对应部分。
///
/// 星名按 `key` 用传入语言现翻而非取 `star.name`——后者在排盘时已按排盘语言固化，
/// 直接用会在「文本语言 ≠ 排盘语言」时输出混排文本。
fn star_token(star: &Star, l: &Labels, lang: Language) -> String {
    let mut s = translate_star(star.key, lang).to_string();
    if let Some(b) = star.brightness {
        s.push_str(&format!("({})", translate_brightness(b, lang)));
    }
    if let Some(m) = star.mutagen {
        s.push_str(l.label_sep);
        s.push_str(&mutagen_mark(l, m, lang));
    }
    s
}

/// 干支写法：中日文相连（癸丑），拉丁字母与韩文分词（Quý Sửu）
fn stem_branch(
    stem: crate::data::types::HeavenlyStem,
    branch: crate::data::types::EarthlyBranch,
    l: &Labels,
    lang: Language,
) -> String {
    format!(
        "{}{}{}",
        translate_heavenly_stem(stem, lang),
        l.label_sep,
        translate_earthly_branch(branch, lang)
    )
}

/// 星耀列表；为空返回 `None`，调用方据此整行省略或写占位
fn star_list(stars: &[Star], l: &Labels, lang: Language) -> Option<String> {
    if stars.is_empty() {
        return None;
    }
    Some(
        stars
            .iter()
            .map(|s| star_token(s, l, lang))
            .collect::<Vec<_>>()
            .join(LIST_SEP),
    )
}

/// 语义 key 的译文；词表必含盘上出现的 key，查不到时原样回落
fn by_key(key: &str, lang: Language) -> String {
    translate_key(key, lang)
        .map(str::to_string)
        .unwrap_or_else(|| key.to_string())
}

/// 宫位名后的标记：身宫、来因宫、辅来因宫各占一个方括号
fn palace_marks(p: &PalaceData, l: &Labels) -> String {
    let mut out = String::new();
    if p.is_body_palace {
        out.push_str(&format!(" [{}]", l.body_palace));
    }
    if p.is_original_palace {
        out.push_str(&format!(" [{}]", l.original_palace));
    }
    if p.is_secondary_original_palace {
        out.push_str(&format!(" [{}]", l.secondary_original_palace));
    }
    out
}

/// 从 `start` 起按宫位索引递减的十二宫顺序（本命盘以命宫起即命兄夫子财疾迁仆官田福父）
fn palace_order(start: usize) -> Vec<usize> {
    (0..12)
        .map(|k: i32| fix_index(start as i32 - k, 12))
        .collect()
}

/// 本命盘的阅读顺序：命宫起。十二宫必含命宫，缺失即星盘不变量已破坏
fn reading_order(astrolabe: &Astrolabe) -> Vec<usize> {
    let soul = astrolabe
        .palaces
        .iter()
        .find(|p| p.name == Palace::Soul)
        .expect("十二宫必含命宫")
        .index;
    palace_order(soul)
}

/// 四化星列表带落宫：`太阴化禄→疾厄`；星不在盘上时不写落宫。
/// `natal_frame` 为真时落宫写作「本命X」——运限层级的段落里宫名默认指该层重排宫名，
/// 四化星落的是本命宫位，须标明参照系
fn mutagen_stars_with_places(
    astrolabe: &Astrolabe,
    stars: impl IntoIterator<Item = (StarKey, Mutagen)>,
    natal_frame: bool,
    l: &Labels,
    lang: Language,
) -> String {
    stars
        .into_iter()
        .map(|(star, m)| {
            let mut t = format!(
                "{}{}{}",
                translate_star(star, lang),
                l.label_sep,
                mutagen_mark(l, m, lang)
            );
            if let Some(place) = astrolabe.star(star).map(|s| s.palace().name) {
                t.push_str(ARROW);
                if natal_frame {
                    t.push_str(l.natal);
                    t.push_str(l.label_sep);
                }
                t.push_str(translate_palace(place, lang));
            }
            t
        })
        .collect::<Vec<_>>()
        .join(LIST_SEP)
}

/// 运限层不展开十二宫表时的宫位映射行：从该层命宫起，逐宫写「层宫名→本命宫名」。
/// 单写一串层宫名会丢掉顺序基准——该串按宫位索引排，既非命宫起也非本命宫序，
/// 读者无从还原对应关系。
fn scope_palace_map(
    astrolabe: &Astrolabe,
    index: usize,
    names: &[Palace],
    l: &Labels,
    lang: Language,
) -> String {
    palace_order(index)
        .into_iter()
        .map(|i| {
            let natal = astrolabe.palaces[i].name;
            let scope = names.get(i).copied().unwrap_or(natal);
            format!(
                "{}{ARROW}{}{}{}",
                translate_palace(scope, lang),
                l.natal,
                l.label_sep,
                translate_palace(natal, lang)
            )
        })
        .collect::<Vec<_>>()
        .join(LIST_SEP)
}

/// 一段释义：`**标题**: 正文`，正文保留知识包的段落结构
fn note(out: &mut String, heading: &str, body: &str) {
    out.push_str(&format!("**{heading}**: {}\n\n", body.trim()));
}

// ============================================================
// 宫位段
// ============================================================

/// 运限层的宫位段落参照：宫名按该层，另写大限至该层各层落在本宫的四化与流耀
struct Frame<'h> {
    /// 所选层的层级标签（大限、童限、流年……）
    label: &'static str,
    /// 所选层按宫位索引排列的十二宫名
    names: &'h [Palace],
    /// 大限至所选层各层，按层级先后
    layers: Vec<&'h HoroscopeItem>,
}

impl Frame<'_> {
    /// 宫位索引在该层的宫名
    fn name(&self, index: usize) -> Palace {
        self.names[index]
    }
}

/// 某宫的三方四正行：对宫与三合两宫（财帛位、官禄位）的宫名，几何取自内核；
/// 有 `frame` 时宫名按该层
fn surround_line(
    astrolabe: &Astrolabe,
    p: &PalaceData,
    frame: Option<&Frame>,
    l: &Labels,
    lang: Language,
) -> String {
    let sp = astrolabe
        .palace(p.index)
        .expect("宫位索引来自本盘")
        .surrounded_palaces();
    let name = |q: &PalaceData| translate_palace(frame.map_or(q.name, |f| f.name(q.index)), lang);
    format!(
        "- {}: {} {}{FIELD_SEP}{} {}{LIST_SEP}{}\n",
        l.surround,
        l.opposite,
        name(sp.opposite),
        l.trine,
        name(sp.wealth),
        name(sp.career)
    )
}

/// 某宫的宫干飞化行：四化星与其落宫；有 `frame` 时落宫标明「本命」参照系
fn flying_line(
    astrolabe: &Astrolabe,
    p: &PalaceData,
    frame: Option<&Frame>,
    l: &Labels,
    lang: Language,
) -> String {
    format!(
        "- {}: {}\n",
        l.stem_flying
            .replace("{}", translate_heavenly_stem(p.heavenly_stem, lang)),
        mutagen_stars_with_places(
            astrolabe,
            p.stem_mutagens().iter(),
            frame.is_some(),
            l,
            lang
        )
    )
}

/// 运限各层落在本宫的四化与流耀：每层至多两行（`- 大限四化: 天机化权`、`- 流年流耀: 流羊`），
/// 该层在本宫没有的不写
fn layer_lines(p: &PalaceData, frame: &Frame, l: &Labels, lang: Language) -> String {
    let mut out = String::new();
    for item in &frame.layers {
        let label = l.scope_label(item.name_key);
        let mutagens: Vec<String> = item
            .mutagens()
            .filter(|(star, _)| p.has(&[*star]))
            .map(|(star, m)| {
                format!(
                    "{}{}{}",
                    translate_star(star, lang),
                    l.label_sep,
                    mutagen_mark(l, m, lang)
                )
            })
            .collect();
        if !mutagens.is_empty() {
            out.push_str(&format!(
                "- {label}{}{}: {}\n",
                l.label_sep,
                l.mutagen_fly,
                mutagens.join(LIST_SEP)
            ));
        }
        if let Some(stars) = item.stars.as_ref().and_then(|g| g.get(p.index))
            && !stars.is_empty()
        {
            let names: Vec<&str> = stars.iter().map(|s| translate_star(s.key, lang)).collect();
            out.push_str(&format!(
                "- {label}{}{}: {}\n",
                l.label_sep,
                l.scope_stars,
                names.join(LIST_SEP)
            ));
        }
    }
    out
}

/// 某宫的十二神行：四组各一位，标组名；本命不安的岁前、将前不写
fn gods_line(p: &PalaceData, l: &Labels, lang: Language) -> String {
    let pairs = [
        (l.changsheng, Some(p.changsheng12)),
        (l.boshi, Some(p.boshi12)),
        (l.suiqian, p.suiqian12),
        (l.jiangqian, p.jiangqian12),
    ];
    format!(
        "- {}: {}\n",
        l.twelve_gods,
        pairs
            .iter()
            .filter_map(|(group, star)| star.map(|s| format!("{group}·{}", translate_star(s, lang))))
            .collect::<Vec<_>>()
            .join(LIST_SEP)
    )
}

/// 一宫的标题行：`### 命宫 (癸丑) · 大限 3-12 [身宫]`；`role` 给出时写在宫名前
/// （三方四正文本里标明本宫/对宫/财帛位/官禄位）。有 `frame` 时宫名写该层的、括号里补本命宫名：
/// `### 本宫 · 流月兄弟 (本命田宅 · 丙子) · 大限 34-43`
fn palace_heading(
    p: &PalaceData,
    role: Option<&str>,
    frame: Option<&Frame>,
    l: &Labels,
    lang: Language,
) -> String {
    let role = role.map(|r| format!("{r}{FIELD_SEP}")).unwrap_or_default();
    let stem_branch = stem_branch(p.heavenly_stem, p.earthly_branch, l, lang);
    let name = match frame {
        None => format!("{} ({stem_branch})", translate_palace(p.name, lang)),
        Some(f) => format!(
            "{}{}{} ({}{}{}{FIELD_SEP}{stem_branch})",
            f.label,
            l.label_sep,
            translate_palace(f.name(p.index), lang),
            l.natal,
            l.label_sep,
            translate_palace(p.name, lang),
        ),
    };
    format!(
        "### {role}{name}{FIELD_SEP}{} {}-{}{}\n",
        l.decadal,
        p.decadal.range.0,
        p.decadal.range.1,
        palace_marks(p, l),
    )
}

/// 一宫的事实行：主星（空宫写明）、辅星、杂耀，有 `frame` 时接运限各层的四化与流耀；
/// 完整层另有三方四正、宫干飞化、十二神、小限
fn palace_facts(
    astrolabe: &Astrolabe,
    p: &PalaceData,
    detail: TextDetail,
    frame: Option<&Frame>,
    l: &Labels,
    lang: Language,
) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "- {}: {}\n",
        l.major_stars,
        star_list(&p.major_stars, l, lang).unwrap_or_else(|| l.empty_palace.to_string())
    ));
    if let Some(s) = star_list(&p.minor_stars, l, lang) {
        out.push_str(&format!("- {}: {s}\n", l.minor_stars));
    }
    if let Some(s) = star_list(&p.adjective_stars, l, lang) {
        out.push_str(&format!("- {}: {s}\n", l.adjective_stars));
    }
    if let Some(f) = frame {
        out.push_str(&layer_lines(p, f, l, lang));
    }
    if detail == TextDetail::Overview {
        return out;
    }
    out.push_str(&surround_line(astrolabe, p, frame, l, lang));
    out.push_str(&flying_line(astrolabe, p, frame, l, lang));
    out.push_str(&gods_line(p, l, lang));
    out.push_str(&format!(
        "- {}: {}\n",
        l.ages,
        p.ages
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(LIST_SEP)
    ));
    out
}

/// 一宫的释义：同宫主星的组合解读在前（每对一次，包里只记在一方名下也查得到），
/// 再按主星、辅星、杂耀顺序逐星释义；十二神不释义。无材料时返回空串
fn palace_notes(p: &PalaceData, notes: Notes, l: &Labels, lang: Language) -> String {
    let mut out = String::new();
    let majors: Vec<StarKey> = p.major_stars.iter().map(|s| s.key).collect();
    for (i, a) in majors.iter().enumerate() {
        for b in &majors[i + 1..] {
            if let Some(text) = notes.combination(*a, *b) {
                let heading = format!(
                    "{} × {} ({})",
                    translate_star(*a, lang),
                    translate_star(*b, lang),
                    l.same_palace
                );
                note(&mut out, &heading, text);
            }
        }
    }
    for star in p
        .major_stars
        .iter()
        .chain(&p.minor_stars)
        .chain(&p.adjective_stars)
    {
        if let Some(text) = notes.star(star.key) {
            note(&mut out, &star_token(star, l, lang), text);
        }
    }
    out
}

/// 一宫的段落：标题（可带角色）、按详略写事实，带包时加释义；`frame` 见 [`palace_heading`]
fn palace_section(
    astrolabe: &Astrolabe,
    p: &PalaceData,
    role: Option<&str>,
    frame: Option<&Frame>,
    opts: &TextOptions,
    l: &Labels,
    lang: Language,
) -> String {
    let mut out = palace_heading(p, role, frame, l, lang);
    out.push_str(&palace_facts(astrolabe, p, opts.detail, frame, l, lang));
    if let Some(notes) = opts.notes() {
        let text = palace_notes(p, notes, l, lang);
        if !text.is_empty() {
            out.push('\n');
            out.push_str(&text);
        }
    }
    out
}

// ============================================================
// 格局
// ============================================================

/// 格局命中列表：`- **名称** (宫名, 破格)：参与成格的星`。
///
/// `palace_names` 是判定视角下按宫位索引排列的十二宫名，与
/// [`crate::pattern::PatternHit::to_dto`] 的同名入参语义一致；索引超出列表时不标宫名。
pub fn patterns_to_text(hits: &[PatternHit], palace_names: &[Palace], lang: Language) -> String {
    patterns_to_text_with(hits, palace_names, &TextOptions::default(), lang)
}

/// 格局命中列表的渲染主体；`palace_names` 语义同 [`patterns_to_text`]
fn pattern_list(
    hits: &[PatternHit],
    palace_names: &[Palace],
    l: &Labels,
    lang: Language,
) -> String {
    debug_assert_eq!(palace_names.len(), 12, "palace_names 应为十二宫全表");
    let mut out = String::new();
    for hit in hits {
        let mut tags: Vec<String> = Vec::new();
        if let Some(p) = palace_names.get(hit.palace) {
            tags.push(translate_palace(*p, lang).to_string());
        }
        if hit.broken {
            tags.push(l.broken.to_string());
        }
        let tag = if tags.is_empty() {
            String::new()
        } else {
            format!(" ({})", tags.join(LIST_SEP))
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
                    t.push_str(l.label_sep);
                    t.push_str(&mutagen_mark(l, m, lang));
                }
                t
            })
            .collect::<Vec<_>>()
            .join(LIST_SEP);
        out.push_str(&format!(
            "- **{}**{tag}: {stars}\n",
            translate_pattern(hit.key, lang)
        ));
    }
    out
}

/// 格局释义：解读与（完整层的）成立条件，同一格局只出一次；`seen` 跨节共享
fn pattern_notes(
    hits: &[PatternHit],
    notes: Notes,
    seen: &mut Vec<PatternKey>,
    l: &Labels,
    lang: Language,
) -> String {
    let mut out = String::new();
    for hit in hits {
        if seen.contains(&hit.key) {
            continue;
        }
        seen.push(hit.key);
        let (reading, conditions) = notes.pattern(hit.key);
        let mut body = reading.unwrap_or_default().to_string();
        if let Some(conditions) = conditions {
            if !body.is_empty() {
                body.push_str("\n\n");
            }
            body.push_str(&format!("{}: {}", l.conditions, conditions.trim()));
        }
        if !body.is_empty() {
            note(&mut out, translate_pattern(hit.key, lang), &body);
        }
    }
    out
}

/// 格局列表 + 释义
pub fn patterns_to_text_with(
    hits: &[PatternHit],
    palace_names: &[Palace],
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    // 独立文档，故带标题：零命中时空串会让调用方分不清「确实没有格局」与「参数没生效」
    let list = pattern_list(hits, palace_names, &l, lang);
    let mut out = format!("# {}\n\n", l.sec_patterns);
    if list.is_empty() {
        out.push_str(NONE_MARK);
        out.push('\n');
    } else {
        out.push_str(&list);
    }
    if let Some(notes) = opts.notes() {
        let text = pattern_notes(hits, notes, &mut Vec::new(), &l, lang);
        if !text.is_empty() {
            out.push('\n');
            out.push_str(&text);
        }
    }
    out
}

/// 四化释义（禄权科忌四条）；无材料返回空串
fn mutagen_notes(notes: Notes, l: &Labels, lang: Language) -> String {
    let mut out = String::new();
    for m in MUTAGEN {
        if let Some(text) = notes.mutagen(m) {
            note(&mut out, translate_mutagen(m, lang), text);
        }
    }
    if out.is_empty() {
        return out;
    }
    format!("## {}\n\n{out}", l.sec_mutagen_notes)
}

// ============================================================
// 本命盘
// ============================================================

/// 本命盘的语义化文本（默认格局口径，只含事实）
pub fn astrolabe_to_text(astrolabe: &Astrolabe, lang: Language) -> String {
    astrolabe_to_text_with(astrolabe, &TextOptions::default(), lang)
}

/// 本命盘的语义化文本：基本信息、十二宫总览表、格局，完整层另有从命宫起的十二宫详解；
/// 带知识包时每宫详解附该宫星耀释义、格局附释义、文末附四化释义
pub fn astrolabe_to_text_with(astrolabe: &Astrolabe, opts: &TextOptions, lang: Language) -> String {
    let l = labels(lang);
    // 各节以换行结尾、节与节之间空一行
    let mut sections: Vec<String> = Vec::new();
    let mut out = String::new();
    let hour = by_key(CHINESE_TIME[astrolabe.time_index as usize], lang);

    // ---- 标题 ----
    out.push_str(&format!(
        "# {} {} {} {}\n\n",
        l.doc_chart,
        astrolabe.solar_date,
        hour,
        translate_gender(astrolabe.gender, lang),
    ));

    // ---- 基本信息 ----
    out.push_str(&format!("## {}\n", l.sec_basic));
    let cd = &astrolabe.raw_dates.chinese_date;
    out.push_str(&format!(
        "- {}: {}{FIELD_SEP}{}: {}{FIELD_SEP}{}: {} ({})\n",
        l.solar_date,
        astrolabe.solar_date,
        l.lunar_date,
        {
            let d = &astrolabe.raw_dates.lunar_date;
            translate_lunar_date_body(d.lunar_year, d.lunar_month, d.lunar_day, d.is_leap, lang)
        },
        l.time,
        hour,
        astrolabe.time_range,
    ));
    out.push_str(&format!(
        "- {}: {}{FIELD_SEP}{}: {}{FIELD_SEP}{}: {}\n",
        l.chinese_date,
        crate::utils::translate_chinese_date([cd.yearly, cd.monthly, cd.daily, cd.hourly], lang),
        l.zodiac_animal,
        by_key(&astrolabe.zodiac_key, lang),
        l.zodiac_sign,
        by_key(&astrolabe.sign_key, lang),
    ));
    out.push_str(&format!(
        "- {}: {}{FIELD_SEP}{}: {}{FIELD_SEP}{}: {}\n",
        l.five_elements_class,
        translate_five_elements_class(astrolabe.five_elements_class, lang),
        l.soul_star,
        translate_star(astrolabe.soul, lang),
        l.body_star,
        translate_star(astrolabe.body, lang),
    ));
    let mut positions = format!(
        "- {}: {}{FIELD_SEP}{}: {}",
        l.soul_palace,
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang),
        l.body_palace,
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang),
    );
    if let Some(b) = astrolabe.palaces.iter().find(|p| p.is_body_palace) {
        positions.push_str(&format!(" ({})", translate_palace(b.name, lang)));
    }
    for (label, marked) in [
        (
            l.original_palace,
            astrolabe.palaces.iter().find(|p| p.is_original_palace),
        ),
        (
            l.secondary_original_palace,
            astrolabe
                .palaces
                .iter()
                .find(|p| p.is_secondary_original_palace),
        ),
    ] {
        if let Some(o) = marked {
            positions.push_str(&format!(
                "{FIELD_SEP}{label}: {} ({})",
                translate_earthly_branch(o.earthly_branch, lang),
                translate_palace(o.name, lang)
            ));
        }
    }
    out.push_str(&positions);
    out.push('\n');
    // 生年四化由本命年干决定，年干即四柱年柱的天干
    let birth_stars = astrolabe.config.stem_mutagens(cd.yearly.0);
    out.push_str(&format!(
        "- {}: {}\n",
        l.birth_mutagen,
        mutagen_stars_with_places(astrolabe, birth_stars.iter(), false, &l, lang)
    ));
    sections.push(std::mem::take(&mut out));

    // ---- 十二宫总览 ----
    let order = reading_order(astrolabe);
    out.push_str(&format!("## {}\n", l.sec_overview));
    out.push_str(&format!(
        "| {} | {} | {} | {} |\n|---|---|---|---|\n",
        l.col_palace, l.major_stars, l.minor_stars, l.decadal
    ));
    for &i in &order {
        let p = &astrolabe.palaces[i];
        let name = translate_palace(p.name, lang);
        let name_cell = if p.name == Palace::Soul {
            format!("**{name}**")
        } else {
            name.to_string()
        };
        out.push_str(&format!(
            "| {name_cell} {}{} | {} | {} | {}-{} |\n",
            translate_earthly_branch(p.earthly_branch, lang),
            palace_marks(p, &l),
            star_list(&p.major_stars, &l, lang).unwrap_or_else(|| NONE_MARK.to_string()),
            star_list(&p.minor_stars, &l, lang).unwrap_or_else(|| NONE_MARK.to_string()),
            p.decadal.range.0,
            p.decadal.range.1,
        ));
    }
    sections.push(std::mem::take(&mut out));

    // ---- 格局 ----
    let hits = astrolabe.patterns_with(&opts.effective_pattern_config());
    if !hits.is_empty() {
        let names: Vec<Palace> = astrolabe.palaces.iter().map(|p| p.name).collect();
        out.push_str(&format!("## {}\n", l.sec_patterns));
        out.push_str(&pattern_list(&hits, &names, &l, lang));
        if let Some(notes) = opts.notes() {
            let text = pattern_notes(&hits, notes, &mut Vec::new(), &l, lang);
            if !text.is_empty() {
                out.push('\n');
                out.push_str(&text);
            }
        }
        sections.push(std::mem::take(&mut out));
    }

    // ---- 十二宫详解 ----
    if opts.detail == TextDetail::Full {
        out.push_str(&format!("## {}\n", l.sec_palaces));
        for &i in &order {
            out.push('\n');
            out.push_str(&palace_section(
                astrolabe,
                &astrolabe.palaces[i],
                None,
                None,
                opts,
                &l,
                lang,
            ));
        }
        sections.push(std::mem::take(&mut out));
    }

    // ---- 附录：四化释义 ----
    if let Some(notes) = opts.notes() {
        let text = mutagen_notes(notes, &l, lang);
        if !text.is_empty() {
            sections.push(text);
        }
    }

    sections.join("\n")
}

// ============================================================
// 单宫与三方四正
// ============================================================

/// 单宫的语义化文本：与本命盘详解里该宫的段落一致
pub fn palace_to_text(palace: &PalaceRef, lang: Language) -> String {
    palace_to_text_with(palace, &TextOptions::default(), lang)
}

/// 单宫文本，带包时附该宫星耀释义
pub fn palace_to_text_with(palace: &PalaceRef, opts: &TextOptions, lang: Language) -> String {
    let l = labels(lang);
    palace_section(palace.astrolabe(), palace, None, None, opts, &l, lang)
}

/// 三方四正的语义化文本：本宫、对宫、财帛位、官禄位各一段
pub fn surrounded_palaces_to_text(sp: &SurroundedPalaces, lang: Language) -> String {
    surrounded_palaces_to_text_with(sp, &TextOptions::default(), lang)
}

/// 三方四正文本，带包时每宫附星耀释义
pub fn surrounded_palaces_to_text_with(
    sp: &SurroundedPalaces,
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    let mut out = format!(
        "## {} {}\n",
        translate_palace(sp.target.name, lang),
        l.surround
    );
    for (role, p) in [
        (l.target_palace, sp.target),
        (l.opposite_palace, sp.opposite),
        (l.wealth_palace, sp.wealth),
        (l.career_palace, sp.career),
    ] {
        out.push('\n');
        out.push_str(&palace_section(
            sp.astrolabe(),
            p,
            Some(role),
            None,
            opts,
            &l,
            lang,
        ));
    }
    out
}

/// 夹宫的语义化文本（默认口径，只含事实）。
pub fn flanking_palaces_to_text(f: &FlankingPalaces, lang: Language) -> String {
    flanking_palaces_to_text_with(f, &TextOptions::default(), lang)
}

/// 夹宫的语义化文本：前后两宫各一段，与三方四正同构，角色写「前宫」「后宫」。
///
/// 判定方法（`have` 等）看的是两宫合计，文本按宫分段是为了读得出各宫自己的星与飞化；
/// 要合看就读两段。
pub fn flanking_palaces_to_text_with(
    f: &FlankingPalaces,
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    let mut out = format!("## {}\n", l.flanking);
    for (role, p) in [(l.previous_palace, f.previous), (l.next_palace, f.next)] {
        out.push('\n');
        out.push_str(&palace_section(
            f.astrolabe(),
            p,
            Some(role),
            None,
            opts,
            &l,
            lang,
        ));
    }
    out
}

/// 大限一览的语义化文本（默认口径）。
pub fn decadal_list_to_text(astrolabe: &Astrolabe, lang: Language) -> String {
    decadal_list_to_text_with(astrolabe, &TextOptions::default(), lang)
}

/// 大限一览的语义化文本：十二个大限一张表，按起运先后排。
///
/// 不展开每限的流年——十二限各十年会撑到一百二十行，一眼看完一生的十二个十年才是这张表
/// 要做的事；某一限的流年用 [`Astrolabe::yearly_list`] 单取。
/// 带知识包时表后附各限四化星的释义（同一颗星只出一次）。
pub fn decadal_list_to_text_with(
    astrolabe: &Astrolabe,
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    let list = astrolabe.decadal_list();
    let mut out = format!("# {}\n\n", l.doc_decadals);

    out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} |\n|---|---|---|---|---|---|\n",
        l.col_decadal_no,
        l.col_natal_palace,
        l.col_age_range,
        l.col_year_range,
        l.col_stem_branch,
        l.mutagen_fly,
    ));
    for (n, d) in list.iter().enumerate() {
        out.push_str(&format!(
            "| {} | {} | {}-{} | {}-{} | {} | {} |\n",
            n + 1,
            translate_palace(d.palace_name, lang),
            d.age_range.0,
            d.age_range.1,
            d.year_range.0,
            d.year_range.1,
            stem_branch(d.heavenly_stem, d.earthly_branch, &l, lang),
            mutagen_cell(astrolabe, &d.base, &l, lang),
        ));
    }

    out.push_str(&mutagen_star_notes(
        list.iter().flat_map(|d| d.mutagens()),
        opts,
        &l,
        lang,
    ));
    out
}

/// 一览表后的四化星释义一节：各行的四化星按出现先后去重；不带知识包或无材料返回空串
fn mutagen_star_notes(
    rows: impl Iterator<Item = (StarKey, Mutagen)>,
    opts: &TextOptions,
    l: &Labels,
    lang: Language,
) -> String {
    let Some(notes) = opts.notes() else {
        return String::new();
    };
    let mut seen: Vec<StarKey> = Vec::new();
    let mut text = String::new();
    for (key, _) in rows {
        if seen.contains(&key) {
            continue;
        }
        seen.push(key);
        if let Some(reading) = notes.star(key) {
            note(&mut text, translate_star(key, lang), reading);
        }
    }
    if text.is_empty() {
        return String::new();
    }
    format!("\n## {}\n\n{text}", l.sec_star_notes)
}

/// 指定农历年的流月一览（只含事实）
///
/// # Errors
/// 同 [`monthly_list_to_text_with`]。
pub fn monthly_list_to_text(
    astrolabe: &Astrolabe,
    year: i64,
    lang: Language,
) -> Result<String, IztroError> {
    monthly_list_to_text_with(astrolabe, year, &TextOptions::default(), lang)
}

/// 指定农历年的流月一览：每个流月一行，写农历月、阳历起止日、流月命宫所在的本命宫、干支与四化落宫。
/// 闰月按前后半月拆成两行（初一分界下前半月的流月同本月、后半月同下月），月号前加「闰」并注明农历日区间。
/// 带知识包时表后附四化星的释义（跨行去重）。各行与 [`Astrolabe::monthly_list`] 一一对应：按节气换月柱时
/// 农历月在交节处拆成两行，不是整月的行在月号后注明农历日区间。
///
/// # Errors
/// 年份不在 1-9999，或某一段的首日落在支持范围外时返回 [`IztroError`]（同
/// [`Astrolabe::monthly_list`]）。
pub fn monthly_list_to_text_with(
    astrolabe: &Astrolabe,
    year: i64,
    opts: &TextOptions,
    lang: Language,
) -> Result<String, IztroError> {
    let l = labels(lang);
    let list = astrolabe.monthly_list(year, true)?;
    let mut out = format!("# {} {year}\n\n", l.doc_monthlies);

    out.push_str(&format!(
        "| {} | {} | {} | {} | {} |\n|---|---|---|---|---|\n",
        l.col_lunar_month, l.solar_date, l.col_natal_palace, l.col_stem_branch, l.mutagen_fly,
    ));
    for m in &list {
        let signed_month = if m.is_leap_month {
            -(m.month as i64)
        } else {
            m.month as i64
        };
        let solar = |day: u32| {
            let (y, mo, d) = crate::calendar::solar_date_of(year, signed_month, day as i64)
                .expect("流月列表项的农历日在该月的天数之内");
            format!("{y}-{mo}-{d}")
        };
        let whole_month = m.day_range
            == (
                1,
                crate::calendar::month_day_count(year, signed_month)
                    .expect("流月列表项的月份在该年之内") as u32,
            );
        let month = match (m.is_leap_month, whole_month) {
            (false, true) => m.month.to_string(),
            (false, false) => format!("{} ({}-{})", m.month, m.day_range.0, m.day_range.1),
            (true, _) => format!(
                "{}{}{} ({}-{})",
                l.leap_month, l.label_sep, m.month, m.day_range.0, m.day_range.1
            ),
        };
        out.push_str(&format!(
            "| {month} | {} ~ {} | {} | {} | {} |\n",
            solar(m.day_range.0),
            solar(m.day_range.1),
            translate_palace(astrolabe.palaces[m.index].name, lang),
            stem_branch(m.heavenly_stem, m.earthly_branch, &l, lang),
            mutagen_cell(astrolabe, &m.base, &l, lang),
        ));
    }

    out.push_str(&mutagen_star_notes(
        list.iter().flat_map(|m| m.mutagens()),
        opts,
        &l,
        lang,
    ));
    Ok(out)
}

// ============================================================
// 运限
// ============================================================

/// 各运限层级视角的格局命中，按 [`HOROSCOPE_SCOPES`] 顺序排列
type ScopeHits = Vec<(Scope, Vec<PatternHit>)>;

/// 文本写到的各运限层级的格局命中，按选项的口径判定、只取到选项写到的层级
/// （小限不在 Scope 之列，无格局）
fn horoscope_scope_hits(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    opts: &TextOptions,
) -> ScopeHits {
    let config = opts.effective_pattern_config();
    HOROSCOPE_SCOPES
        .into_iter()
        .filter(|scope| opts.writes_scope(*scope))
        .map(|scope| {
            let hits = patterns_through(astrolabe, horoscope, scope, opts.through(), &config);
            (scope, hits)
        })
        .collect()
}

/// 某层级的命中；不在列表内的层级视为无命中
fn hits_of(scope_hits: &ScopeHits, scope: Scope) -> &[PatternHit] {
    scope_hits
        .iter()
        .find(|(s, _)| *s == scope)
        .map_or(&[], |(_, hits)| hits.as_slice())
}

/// 运限层级的标题行：`## 大限 · 命宫: 本命夫妻 (庚辰)`
fn scope_heading(
    label: &str,
    item: &HoroscopeItem,
    astrolabe: &Astrolabe,
    l: &Labels,
    lang: Language,
) -> String {
    format!(
        "## {label}{FIELD_SEP}{}: {}{}{} ({})\n",
        l.soul_palace,
        l.natal,
        l.label_sep,
        translate_palace(astrolabe.palaces[item.index].name, lang),
        stem_branch(item.heavenly_stem, item.earthly_branch, l, lang),
    )
}

/// 层级的四化行：四化星带本命落宫，落宫标明「本命」参照系
/// （同一段落里流耀行与十二宫表用的是该层重排宫名）。四化不是由该层干支的天干化出时
/// （`scope_mutagen_stem=scopeSoulPalace`），标签后注明所用的天干（`- 四化 (辛干): …`）；
/// 不起四化（小限 `age_mutagens=none`）时返回空串
fn scope_mutagen_line(
    astrolabe: &Astrolabe,
    item: &HoroscopeItem,
    l: &Labels,
    lang: Language,
) -> String {
    let Some(stem) = item.mutagen_stem else {
        return String::new();
    };
    let label = if stem == item.heavenly_stem {
        l.mutagen_fly.to_string()
    } else {
        format!(
            "{} ({})",
            l.mutagen_fly,
            l.mutagen_stem
                .replace("{}", translate_heavenly_stem(stem, lang))
        )
    };
    format!(
        "- {label}: {}\n",
        mutagen_stars_with_places(astrolabe, item.mutagens(), true, l, lang)
    )
}

/// 一览表里一层的四化格：四化星带本命落宫；四化所用的干不是该层自身的干时，前面注明所用的干
/// （`(辛干) 巨门化禄→本命子女, …`）；该层不起四化写「—」
fn mutagen_cell(astrolabe: &Astrolabe, item: &HoroscopeItem, l: &Labels, lang: Language) -> String {
    let Some(stem) = item.mutagen_stem else {
        return NONE_MARK.to_string();
    };
    let stars = mutagen_stars_with_places(astrolabe, item.mutagens(), true, l, lang);
    if stem == item.heavenly_stem {
        stars
    } else {
        format!(
            "({}) {stars}",
            l.mutagen_stem
                .replace("{}", translate_heavenly_stem(stem, lang))
        )
    }
}

/// 层级的格局行：命中格局带该层宫名，同名同宫去重；无命中返回空串
fn scope_pattern_line(
    hits: &[PatternHit],
    item: &HoroscopeItem,
    l: &Labels,
    lang: Language,
) -> String {
    let mut entries: Vec<String> = Vec::new();
    for h in hits {
        let mut t = translate_pattern(h.key, lang).to_string();
        let mut tags: Vec<&str> = Vec::new();
        if let Some(p) = item.palace_names.get(h.palace) {
            tags.push(translate_palace(*p, lang));
        }
        if h.broken {
            tags.push(l.broken);
        }
        if !tags.is_empty() {
            t.push_str(&format!(" ({})", tags.join(LIST_SEP)));
        }
        if !entries.contains(&t) {
            entries.push(t);
        }
    }
    if entries.is_empty() {
        return String::new();
    }
    format!("- {}: {}\n", l.sec_patterns, entries.join(LIST_SEP))
}

/// 层级的流耀行：每颗流耀指向它落在该层的哪一宫；无流耀返回空串
fn scope_stars_line(item: &HoroscopeItem, l: &Labels, lang: Language) -> String {
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
                    t.push_str(&format!("{ARROW}{}", translate_palace(p, lang)));
                }
                t
            })
        })
        .collect();
    if entries.is_empty() {
        return String::new();
    }
    format!("- {}: {}\n", l.scope_stars, entries.join(LIST_SEP))
}

/// 层级的十二宫表：该层宫名、本命宫名、本命主辅星、该层流耀（流年另加岁前/将前）
fn scope_table(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    item: &HoroscopeItem,
    label: &str,
    yearly_gods: bool,
    l: &Labels,
    lang: Language,
) -> String {
    let mut out = String::new();
    let mut header = format!(
        "| {label} | {} | {} | {} | {} |",
        l.natal, l.major_stars, l.minor_stars, l.scope_stars
    );
    let mut rule = "|---|---|---|---|---|".to_string();
    if yearly_gods {
        header.push_str(&format!(" {} |", l.twelve_gods));
        rule.push_str("---|");
    }
    out.push_str(&format!("{header}\n{rule}\n"));
    for i in palace_order(item.index) {
        let p = &astrolabe.palaces[i];
        let scope_name = item
            .palace_names
            .get(i)
            .map_or(translate_palace(p.name, lang), |n| {
                translate_palace(*n, lang)
            });
        let flow = item
            .stars
            .as_ref()
            .and_then(|g| g.get(i))
            .and_then(|s| star_list(s, l, lang))
            .unwrap_or_else(|| NONE_MARK.to_string());
        out.push_str(&format!(
            "| {scope_name} | {} | {} | {} | {flow} |",
            translate_palace(p.name, lang),
            star_list(&p.major_stars, l, lang).unwrap_or_else(|| NONE_MARK.to_string()),
            star_list(&p.minor_stars, l, lang).unwrap_or_else(|| NONE_MARK.to_string()),
        ));
        if yearly_gods {
            let dec = &horoscope.yearly.yearly_dec_star;
            out.push_str(&format!(
                " {}·{}{LIST_SEP}{}·{} |",
                l.suiqian,
                translate_star(dec.suiqian12[i], lang),
                l.jiangqian,
                translate_star(dec.jiangqian12[i], lang)
            ));
        }
        out.push('\n');
    }
    out
}

/// 运限层级段的事实部分。`table` 给出时（值为是否加岁前/将前列）写四化、格局与该层十二宫表；
/// 不给时写宫名对照、四化、流耀与格局，不逐宫展开
fn scope_facts(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    item: &HoroscopeItem,
    hits: &[PatternHit],
    table: Option<bool>,
    l: &Labels,
    lang: Language,
) -> String {
    let label = l.scope_label(item.name_key);
    let mut out = scope_heading(label, item, astrolabe, l, lang);
    match table {
        Some(yearly_gods) => {
            out.push_str(&scope_mutagen_line(astrolabe, item, l, lang));
            out.push_str(&scope_pattern_line(hits, item, l, lang));
            out.push('\n');
            out.push_str(&scope_table(
                astrolabe,
                horoscope,
                item,
                label,
                yearly_gods,
                l,
                lang,
            ));
        }
        None => {
            out.push_str(&format!(
                "- {}: {}\n",
                l.palace_names,
                scope_palace_map(astrolabe, item.index, &item.palace_names, l, lang)
            ));
            out.push_str(&scope_mutagen_line(astrolabe, item, l, lang));
            out.push_str(&scope_stars_line(item, l, lang));
            out.push_str(&scope_pattern_line(hits, item, l, lang));
        }
    }
    out
}

/// 层级的流耀释义：跨层级按星去重；无材料返回空串
fn flow_notes(
    item: &HoroscopeItem,
    notes: Notes,
    seen: &mut Vec<StarKey>,
    l: &Labels,
    lang: Language,
) -> String {
    let mut out = String::new();
    for star in item.stars.iter().flatten().flatten() {
        if seen.contains(&star.key) {
            continue;
        }
        seen.push(star.key);
        if let Some(text) = notes.star(star.key) {
            note(&mut out, &star_token(star, l, lang), text);
        }
    }
    out
}

/// 运限文本的标题行，只写到文本写到的层级：写到流日或流时为目标日期的阳历与农历
/// （`# 运限 2025-1-1 (二〇二四年腊月初二)`）；写到流月为农历年份与流年干支，再加流月干支
/// （`# 运限 2025 (乙巳) · 流月 (甲申)`）；只写到大限或流年为农历年份与流年干支（`# 运限 2027 (丁未)`）。
/// 按年、按月取的运限，目标日期是内核挑的代表日，写进标题会被当成所问的日子。
fn horoscope_title(
    horoscope: &HoroscopeData,
    through: Scope,
    l: &Labels,
    lang: Language,
) -> String {
    format!(
        "# {} {}\n",
        l.doc_horoscope,
        horoscope_when(horoscope, through, l, lang)
    )
}

/// 运限写到 `through` 层时的时间写法，规则见 [`horoscope_title`]：`2025-1-1 (二〇二四年腊月初二)`、
/// `2025 (乙巳) · 流月 (甲申)`、`2027 (丁未)`
fn horoscope_when(horoscope: &HoroscopeData, through: Scope, l: &Labels, lang: Language) -> String {
    let yearly = &horoscope.yearly;
    let year = format!(
        "{} ({})",
        yearly_layer_year(horoscope),
        stem_branch(yearly.heavenly_stem, yearly.earthly_branch, l, lang)
    );
    match through {
        Scope::Daily | Scope::Hourly => {
            format!("{} ({})", horoscope.solar_date, horoscope.lunar_date)
        }
        Scope::Monthly => format!(
            "{year}{FIELD_SEP}{} ({})",
            l.monthly,
            stem_branch(
                horoscope.monthly.heavenly_stem,
                horoscope.monthly.earthly_branch,
                l,
                lang
            )
        ),
        Scope::Origin | Scope::Decadal | Scope::Yearly => year,
    }
}

/// 流年干支所属的年份：目标日所在农历年及其前后一年里，年干支与流年层干支相同的那一年。
/// 流年按正月初一或立春换年（`horoscope_divide`），立春与正月初一相隔不到一年，三年里恰有一年相合；
/// [`Astrolabe::horoscope_of_year`] 与 [`Astrolabe::horoscope_of_month`] 取的运限即所问的年份。
///
/// # Panics
/// `horoscope` 不是本库算出的运限（阳历日期不是 `y-m-d`，或流年干支与目标日相距一年以上）时。
fn yearly_layer_year(horoscope: &HoroscopeData) -> i64 {
    let (y, m, d) =
        parse_solar_date(&horoscope.solar_date).expect("运限的阳历日期由内核按 y-m-d 写出");
    let lunar_year = crate::calendar::lunar_date_of(y, m, d).year;
    let pillar = (
        horoscope.yearly.heavenly_stem,
        horoscope.yearly.earthly_branch,
    );
    (lunar_year - 1..=lunar_year + 1)
        .find(|year| crate::calendar::year_pillar(*year) == pillar)
        .expect("流年换年点与正月初一相隔不到一年")
}

/// 运限的语义化文本（默认格局口径，只含事实）
///
/// # Panics
/// 同 [`horoscope_to_text_with`]。
pub fn horoscope_to_text(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    lang: Language,
) -> String {
    horoscope_to_text_with(astrolabe, horoscope, &TextOptions::default(), lang)
}

/// 运限的语义化文本：大限、小限、流年、流月、流日、流时各一节，完整层的大限与流年展开
/// 十二宫表、总览层与其余各层同写宫名对照与流耀行；带知识包时各层附流耀与格局释义（跨层去重）。
/// 写到哪一层为止由 [`TextOptions::horoscope_through`] 决定，不设即写到流时，小限随流年一起写或不写；
/// 标题同样只写到这一层：写到流日或流时为目标日期，写到流月为农历年份、流年干支与流月干支，
/// 只写到大限或流年为农历年份与流年干支。
/// `horoscope` 不带查询精度：按年、按月取的运限用 [`HoroscopeRef::to_text_with`]，默认只写到查询精度
///
/// # Panics
/// `horoscope` 不是本库算出的运限（阳历日期不是 `y-m-d`，或流年干支与该日相距一年以上）时。
pub fn horoscope_to_text_with(
    astrolabe: &Astrolabe,
    horoscope: &HoroscopeData,
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    let scope_hits = horoscope_scope_hits(astrolabe, horoscope, opts);
    let mut seen_stars: Vec<StarKey> = Vec::new();
    let mut seen_patterns: Vec<PatternKey> = Vec::new();
    let mut out = String::new();
    let full = opts.detail == TextDetail::Full;

    out.push_str(&horoscope_title(horoscope, opts.through(), &l, lang));

    // 一个层级段：事实，带释义时再接该层的流耀释义与格局释义
    let mut scope_section = |item: &HoroscopeItem, scope: Scope, table: Option<bool>| -> String {
        let hits = hits_of(&scope_hits, scope);
        let mut s = scope_facts(astrolabe, horoscope, item, hits, table, &l, lang);
        if let Some(notes) = opts.notes() {
            let mut text = flow_notes(item, notes, &mut seen_stars, &l, lang);
            text.push_str(&pattern_notes(hits, notes, &mut seen_patterns, &l, lang));
            if !text.is_empty() {
                s.push('\n');
                s.push_str(&text);
            }
        }
        s
    };

    // ---- 大限（未起运时为童限）----
    out.push('\n');
    out.push_str(&scope_section(
        &horoscope.decadal,
        Scope::Decadal,
        full.then_some(false),
    ));

    if !opts.writes_scope(Scope::Yearly) {
        return out;
    }

    // ---- 小限：落宫、虚岁、宫名、四化与该宫星耀 ----
    let age = &horoscope.age;
    let age_palace = &astrolabe.palaces[age.index];
    out.push('\n');
    out.push_str(&format!(
        "## {}{FIELD_SEP}{}: {}{}{}{FIELD_SEP}{} {}\n",
        l.age_fortune,
        l.soul_palace,
        l.natal,
        l.label_sep,
        translate_palace(age_palace.name, lang),
        l.nominal_age,
        age.nominal_age,
    ));
    out.push_str(&format!(
        "- {}: {}\n",
        l.palace_names,
        scope_palace_map(astrolabe, age.index, &age.palace_names, &l, lang)
    ));
    out.push_str(&scope_mutagen_line(astrolabe, &age.base, &l, lang));
    for (label, stars) in [
        (l.major_stars, &age_palace.major_stars),
        (l.minor_stars, &age_palace.minor_stars),
        (l.adjective_stars, &age_palace.adjective_stars),
    ] {
        if let Some(s) = star_list(stars, &l, lang) {
            out.push_str(&format!("- {label}: {s}\n"));
        }
    }

    // ---- 流年（完整层的十二宫表加岁前/将前）----
    out.push('\n');
    out.push_str(&scope_section(
        &horoscope.yearly.base,
        Scope::Yearly,
        full.then_some(true),
    ));

    // ---- 流月/流日/流时：不逐宫展开 ----
    for (item, scope) in [
        (&horoscope.monthly, Scope::Monthly),
        (&horoscope.daily, Scope::Daily),
        (&horoscope.hourly, Scope::Hourly),
    ] {
        if !opts.writes_scope(scope) {
            break;
        }
        out.push('\n');
        out.push_str(&scope_section(item, scope, None));
    }

    out
}

/// 连续若干农历年的流年一览（只含事实）
///
/// # Errors
/// 同 [`yearly_overview_to_text_with`]。
pub fn yearly_overview_to_text(
    astrolabe: &Astrolabe,
    from: i64,
    to: i64,
    lang: Language,
) -> Result<String, IztroError> {
    yearly_overview_to_text_with(astrolabe, from, to, &TextOptions::default(), lang)
}

/// 农历 `from` 年到 `to` 年（含两端）的流年一览：每年一行，写年份、干支、正月初一的阳历日期、虚岁、
/// 大限与小限所在的本命宫、流年命宫所在的本命宫与流年四化落宫。各年取
/// [`Astrolabe::horoscope_of_year`]，与按年取运限同一口径；未起运的年份大限一格写童限。
/// 改历年里没有正月时正月初一一格写「—」。带知识包时表后附四化星释义（跨行去重）。
///
/// # Errors
/// `from` 大于 `to` 时返回 [`IztroError::InvalidArgument`]；某一年不在 1-9999 时同
/// [`Astrolabe::horoscope_of_year`]。
pub fn yearly_overview_to_text_with(
    astrolabe: &Astrolabe,
    from: i64,
    to: i64,
    opts: &TextOptions,
    lang: Language,
) -> Result<String, IztroError> {
    if from > to {
        return Err(IztroError::InvalidArgument(format!(
            "invalid year range {from}-{to}: the first year is after the last"
        )));
    }
    let l = labels(lang);
    let years = (from..=to)
        .map(|year| Ok((year, astrolabe.horoscope_of_year(year)?.into_data())))
        .collect::<Result<Vec<_>, IztroError>>()?;
    let mut out = format!("# {} {from}-{to}\n\n", l.doc_yearlies);

    out.push_str(&format!(
        "| {} | {} | {} | {} | {} | {} | {}{}{} | {} |\n|---|---|---|---|---|---|---|---|\n",
        l.col_year,
        l.col_stem_branch,
        l.col_new_year,
        l.nominal_age,
        l.decadal_fortune,
        l.age_fortune,
        l.yearly,
        l.label_sep,
        l.soul_palace,
        l.mutagen_fly,
    ));
    let palace = |index: usize| translate_palace(astrolabe.palaces[index].name, lang);
    for (year, h) in &years {
        let new_year = crate::calendar::solar_date_of(*year, 1, 1)
            .map_or_else(|| NONE_MARK.to_string(), |(y, m, d)| format!("{y}-{m}-{d}"));
        let decadal = if h.decadal.name_key == crate::data::types::HoroscopeName::Childhood {
            format!(
                "{}{FIELD_SEP}{}",
                l.childhood_fortune,
                palace(h.decadal.index)
            )
        } else {
            palace(h.decadal.index).to_string()
        };
        out.push_str(&format!(
            "| {year} | {} | {new_year} | {} | {decadal} | {} | {} | {} |\n",
            stem_branch(h.yearly.heavenly_stem, h.yearly.earthly_branch, &l, lang),
            h.age.nominal_age,
            palace(h.age.index),
            palace(h.yearly.index),
            mutagen_cell(astrolabe, &h.yearly.base, &l, lang),
        ));
    }

    out.push_str(&mutagen_star_notes(
        years.iter().flat_map(|(_, h)| h.yearly.mutagens()),
        opts,
        &l,
        lang,
    ));
    Ok(out)
}

// ============================================================
// 主题
// ============================================================

/// 主题文本要求带知识包：主题定义在包里
fn topic_notes<'a>(opts: &TextOptions<'a>) -> Result<Notes<'a>, IztroError> {
    opts.notes().ok_or_else(|| {
        IztroError::InvalidArgument(
            "topic text requires a knowledge pack: topics are defined in the pack".to_string(),
        )
    })
}

/// 主题文本的主体，按 `selection` 成段：`surround` 各组连同三方四正各成一节，`palaces` 各宫归入
/// 「相关宫位」一节，同一宫只出一次；主题说明写在标题下（摘要层取摘要），文末附落在选中宫内的格局与
/// 四化释义。`horoscope` 给出时是 `selection.scope` 层的主题文本：标题写运限时间，标题下逐层写大限至
/// 该层的命宫位置与四化，宫位段落按该层宫名并附各层落在本宫的四化与流耀，格局取该层视角的命中。
fn topic_text(
    astrolabe: &Astrolabe,
    horoscope: Option<&HoroscopeData>,
    selection: &TopicSelection,
    entry: &TopicEntry,
    notes: Notes,
    opts: &TextOptions,
    lang: Language,
) -> String {
    let l = labels(lang);
    let layers: Vec<&HoroscopeItem> = horoscope.map_or_else(Vec::new, |h| {
        layers_through(h, selection.scope)
            .into_iter()
            .map(|(_, item)| item)
            .collect()
    });
    let frame = layers.last().map(|item| Frame {
        label: l.scope_label(item.name_key),
        names: &item.palace_names,
        layers: layers.clone(),
    });
    let frame = frame.as_ref();
    let hour = by_key(CHINESE_TIME[astrolabe.time_index as usize], lang);
    // 各节以换行结尾、节与节之间空一行
    let mut sections: Vec<String> = Vec::new();
    let title = entry.title.as_deref().unwrap_or(&selection.topic);
    let chart = format!(
        "{} {} {} {}",
        l.doc_chart,
        astrolabe.solar_date,
        hour,
        translate_gender(astrolabe.gender, lang),
    );
    let mut head = match horoscope {
        None => format!("# {title}{FIELD_SEP}{chart}\n"),
        Some(h) => format!(
            "# {title}{FIELD_SEP}{} {}{FIELD_SEP}{chart}\n",
            l.doc_horoscope,
            horoscope_when(h, selection.scope, &l, lang)
        ),
    };
    let description = match notes.depth {
        KnowledgeDepth::Full => entry.intro.as_deref(),
        KnowledgeDepth::Summary => entry.brief(),
    };
    if let Some(text) = description {
        head.push('\n');
        head.push_str(text.trim());
        head.push('\n');
    }
    sections.push(head);

    for item in &layers {
        let label = l.scope_label(item.name_key);
        let mut s = scope_heading(label, item, astrolabe, &l, lang);
        s.push_str(&scope_mutagen_line(astrolabe, item, &l, lang));
        sections.push(s);
    }

    let mut related = String::new();
    for (g, group) in selection.groups.iter().enumerate() {
        let mut body = String::new();
        for p in selection.palaces.iter().filter(|p| p.group == g) {
            let role = match p.role {
                TopicRole::Target => Some(l.target_palace),
                TopicRole::Opposite => Some(l.opposite_palace),
                TopicRole::Wealth => Some(l.wealth_palace),
                TopicRole::Career => Some(l.career_palace),
                TopicRole::Related => None,
            };
            let section = palace_section(
                astrolabe,
                &astrolabe.palaces[p.index],
                role,
                frame,
                opts,
                &l,
                lang,
            );
            let out = match group.kind {
                TopicGroupKind::Surround => &mut body,
                TopicGroupKind::Only => &mut related,
            };
            out.push('\n');
            out.push_str(&section);
        }
        if body.is_empty() {
            continue;
        }
        let name = match frame {
            None => translate_palace(astrolabe.palaces[group.target].name, lang).to_string(),
            Some(f) => format!(
                "{}{}{}",
                f.label,
                l.label_sep,
                translate_palace(f.name(group.target), lang)
            ),
        };
        sections.push(format!("## {name} {}\n{body}", l.surround));
    }
    if !related.is_empty() {
        sections.push(format!("## {}\n{related}", l.related_palaces));
    }

    let names: Vec<Palace> = match layers.last() {
        Some(item) => item.palace_names.clone(),
        None => astrolabe.palaces.iter().map(|p| p.name).collect(),
    };
    let hits = &selection.patterns;
    if !hits.is_empty() {
        let mut out = format!("## {}\n", l.sec_patterns);
        out.push_str(&pattern_list(hits, &names, &l, lang));
        let text = pattern_notes(hits, notes, &mut Vec::new(), &l, lang);
        if !text.is_empty() {
            out.push('\n');
            out.push_str(&text);
        }
        sections.push(out);
    }
    let text = mutagen_notes(notes, &l, lang);
    if !text.is_empty() {
        sections.push(text);
    }
    sections.join("\n")
}

/// 主题的语义化文本（本命）：按知识包里该主题的定义选宫（[`Astrolabe::topic_palaces`] 同一套选宫）——
/// `surround` 的各宫连同三方四正各成一组，`palaces` 的各宫只取本宫、归入「相关宫位」一节，同一宫只出一次——
/// 每宫一段，详略与释义深度随 `opts`；主题说明写在标题下（摘要层取摘要），文末附落在选中宫内的格局与四化释义。
/// 主题定义与释义都取自 `opts` 的知识包。
///
/// # Errors
/// `opts` 没有知识包、包里没有名为 `topic` 的主题、或主题里有不认识的宫位标识时返回
/// [`IztroError::InvalidArgument`]。
pub fn topic_to_text(
    astrolabe: &Astrolabe,
    topic: &str,
    opts: &TextOptions,
    lang: Language,
) -> Result<String, IztroError> {
    let notes = topic_notes(opts)?;
    let entry = crate::astro::topic::entry(notes.pack, topic)?;
    let selection = select_topic(
        astrolabe,
        None,
        topic,
        entry,
        &opts.effective_pattern_config(),
    )?;
    Ok(topic_text(
        astrolabe, None, &selection, entry, notes, opts, lang,
    ))
}

/// 主题在运限 `scope` 层的语义化文本：选宫同 [`HoroscopeRef::topic_palaces`]——宫名按该层定位、
/// 三方四正以该层的本宫起算，`bodyPalace` / `originalPalace` 落在本命那一宫。标题写运限时间（同运限文本，
/// 写到 `scope` 层）与命盘；标题下逐层写大限至该层的命宫位置与四化；每宫段落的标题写该层宫名并补本命宫名，
/// 宫内星曜照本命，另写大限至该层各层落在本宫的四化与流耀，三方四正行用该层宫名、宫干飞化的落宫标明本命；
/// 格局取该层视角在选中宫内的命中。`Scope::Origin` 与 [`topic_to_text`] 逐字相同。
///
/// # Errors
/// `scope` 比查询精度（[`HoroscopeRef::precision`]）细，或同 [`topic_to_text`]，返回
/// [`IztroError::InvalidArgument`]。
pub fn horoscope_topic_to_text(
    horoscope: &HoroscopeRef,
    topic: &str,
    scope: Scope,
    opts: &TextOptions,
    lang: Language,
) -> Result<String, IztroError> {
    horoscope.check_within_precision(scope, "scope")?;
    let notes = topic_notes(opts)?;
    let entry = crate::astro::topic::entry(notes.pack, topic)?;
    let astrolabe = horoscope.astrolabe();
    let selection = select_topic(
        astrolabe,
        Some((horoscope.data(), scope)),
        topic,
        entry,
        &opts.effective_pattern_config(),
    )?;
    let data = (scope != Scope::Origin).then(|| horoscope.data());
    Ok(topic_text(
        astrolabe, data, &selection, entry, notes, opts, lang,
    ))
}

// ============================================================
// 便捷方法
// ============================================================

impl Astrolabe {
    /// 本命盘的语义化文本（按排盘语言）；[`astrolabe_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        astrolabe_to_text(self, self.language)
    }

    /// 本命盘文本，按选项附释义；[`astrolabe_to_text_with`] 的便捷形态
    pub fn to_text_with(&self, opts: &TextOptions) -> String {
        astrolabe_to_text_with(self, opts, self.language)
    }

    /// 大限一览的语义化文本（按排盘语言）；[`decadal_list_to_text`] 的便捷形态
    pub fn decadal_list_to_text(&self) -> String {
        decadal_list_to_text(self, self.language)
    }

    /// 大限一览文本，按选项附释义；[`decadal_list_to_text_with`] 的便捷形态
    pub fn decadal_list_to_text_with(&self, opts: &TextOptions) -> String {
        decadal_list_to_text_with(self, opts, self.language)
    }

    /// 流月一览的语义化文本（按排盘语言）；[`monthly_list_to_text`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`monthly_list_to_text_with`]。
    pub fn monthly_list_to_text(&self, year: i64) -> Result<String, IztroError> {
        monthly_list_to_text(self, year, self.language)
    }

    /// 流月一览文本，按选项附释义；[`monthly_list_to_text_with`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`monthly_list_to_text_with`]。
    pub fn monthly_list_to_text_with(
        &self,
        year: i64,
        opts: &TextOptions,
    ) -> Result<String, IztroError> {
        monthly_list_to_text_with(self, year, opts, self.language)
    }

    /// 流年一览的语义化文本（按排盘语言）；[`yearly_overview_to_text`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`yearly_overview_to_text_with`]。
    pub fn yearly_overview_to_text(&self, from: i64, to: i64) -> Result<String, IztroError> {
        yearly_overview_to_text(self, from, to, self.language)
    }

    /// 流年一览文本，按选项附释义；[`yearly_overview_to_text_with`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`yearly_overview_to_text_with`]。
    pub fn yearly_overview_to_text_with(
        &self,
        from: i64,
        to: i64,
        opts: &TextOptions,
    ) -> Result<String, IztroError> {
        yearly_overview_to_text_with(self, from, to, opts, self.language)
    }

    /// 主题文本（按排盘语言）；[`topic_to_text`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`topic_to_text`]。
    pub fn topic_to_text(&self, topic: &str, opts: &TextOptions) -> Result<String, IztroError> {
        topic_to_text(self, topic, opts, self.language)
    }
}

impl HoroscopeRef<'_> {
    /// 主题在 `scope` 层的文本（按星盘排盘语言）；[`horoscope_topic_to_text`] 的便捷形态
    ///
    /// # Errors
    /// 同 [`horoscope_topic_to_text`]。
    pub fn topic_to_text(
        &self,
        topic: &str,
        scope: Scope,
        opts: &TextOptions,
    ) -> Result<String, IztroError> {
        horoscope_topic_to_text(self, topic, scope, opts, self.astrolabe().language)
    }

    /// 运限的语义化文本（按星盘排盘语言），写到查询精度（[`Self::precision`]）为止；
    /// [`horoscope_to_text_with`] 的便捷形态
    pub fn to_text(&self) -> String {
        let opts = TextOptions {
            through: Some(self.precision()),
            ..TextOptions::default()
        };
        horoscope_to_text_with(
            self.astrolabe(),
            self.data(),
            &opts,
            self.astrolabe().language,
        )
    }

    /// 运限文本，按选项附释义；未设 [`TextOptions::horoscope_through`] 时写到查询精度为止。
    /// [`horoscope_to_text_with`] 的便捷形态
    ///
    /// # Errors
    /// `opts` 设的层级比查询精度细时返回 [`IztroError::InvalidArgument`]：那几层是内核挑的代表日的，
    /// 不是查询的答案。
    pub fn to_text_with(&self, opts: &TextOptions) -> Result<String, IztroError> {
        let through = match opts.through {
            Some(scope) => {
                self.check_within_precision(scope, "through")?;
                scope
            }
            None => self.precision(),
        };
        let opts = TextOptions {
            through: Some(through),
            ..*opts
        };
        Ok(horoscope_to_text_with(
            self.astrolabe(),
            self.data(),
            &opts,
            self.astrolabe().language,
        ))
    }
}

impl PalaceRef<'_> {
    /// 本宫的语义化文本（按星盘排盘语言）；[`palace_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        palace_to_text(self, self.astrolabe().language)
    }

    /// 本宫文本，按选项附释义；[`palace_to_text_with`] 的便捷形态
    pub fn to_text_with(&self, opts: &TextOptions) -> String {
        palace_to_text_with(self, opts, self.astrolabe().language)
    }
}

impl FlankingPalaces<'_> {
    /// 夹宫的语义化文本（按星盘排盘语言）；[`flanking_palaces_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        flanking_palaces_to_text(self, self.astrolabe().language)
    }

    /// 夹宫文本，按选项附释义；[`flanking_palaces_to_text_with`] 的便捷形态
    pub fn to_text_with(&self, opts: &TextOptions) -> String {
        flanking_palaces_to_text_with(self, opts, self.astrolabe().language)
    }
}

impl SurroundedPalaces<'_> {
    /// 三方四正的语义化文本（按星盘排盘语言）；[`surrounded_palaces_to_text`] 的便捷形态
    pub fn to_text(&self) -> String {
        surrounded_palaces_to_text(self, self.astrolabe().language)
    }

    /// 三方四正文本，按选项附释义；[`surrounded_palaces_to_text_with`] 的便捷形态
    pub fn to_text_with(&self, opts: &TextOptions) -> String {
        surrounded_palaces_to_text_with(self, opts, self.astrolabe().language)
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

    fn with(pack: &KnowledgePack) -> TextOptions<'_> {
        TextOptions::new().knowledge(pack)
    }

    /// 标签只放词：Markdown 结构记号由代码拼接，六语言字面量里不许出现
    /// （英文四化标记 `[{}]` 的方括号不是结构记号，后面不接 `(` 不会成链接）
    #[test]
    fn labels_carry_no_markup() {
        for lit in all_label_literals() {
            for ch in ['#', '*', '|', '=', '\n', ':'] {
                assert!(!lit.contains(ch), "标签 {lit:?} 含结构字符 {ch:?}");
            }
        }
    }

    #[test]
    fn astrolabe_text_zh_cn_has_all_sections() {
        let text = chart(Language::ZhCN).to_text();
        assert!(text.starts_with("# 命盘 2000-8-16 寅时 女\n"), "{text}");
        for needle in [
            "## 基本信息",
            "## 十二宫总览",
            "| 宫位 | 主星 | 辅星 | 大限 |",
            "## 十二宫",
            "### 命宫 (",
            "- 三方四正: 对宫 ",
            "飞化: ",
            "- 十二神: 长生·",
            "- 小限虚岁: ",
            "- 生年四化: ",
            "[身宫]",
            "[来因宫]",
        ] {
            assert!(text.contains(needle), "缺少 {needle:?}\n{text}");
        }
        assert!(!text.contains("**紫微**："));
        assert!(!text.contains("四化释义"));
    }

    /// 十二宫详解从命宫起、按索引递减排列
    #[test]
    fn palaces_are_listed_from_soul_palace() {
        let astrolabe = chart(Language::ZhCN);
        let text = astrolabe.to_text();
        let detail = text.split("## 十二宫\n").nth(1).unwrap();
        let first = detail.lines().find(|l| l.starts_with("### ")).unwrap();
        assert!(first.starts_with("### 命宫 ("), "{first}");
        let order = reading_order(&astrolabe);
        assert_eq!(astrolabe.palaces[order[0]].name, Palace::Soul);
        assert_eq!(astrolabe.palaces[order[1]].name, Palace::Siblings);
        assert_eq!(astrolabe.palaces[order[11]].name, Palace::Parents);
    }

    #[test]
    fn astrolabe_text_en_us_has_english_labels() {
        let text = chart(Language::EnUS).to_text();
        assert!(text.starts_with("# Natal Chart 2000-8-16 "), "{text}");
        assert!(text.contains("## Palaces"));
        assert!(text.contains("- Major Stars: "));
        assert!(text.contains("- Birth-Year Mutagen: "));
        assert!(text.contains("- Twelve Gods: Changsheng·"));
    }

    /// 带释义：格局节后有释义、每宫事实后有该宫星的释义、文末有四化释义，
    /// 且无释义文本的每一行都在带释义文本里（同一渲染器）
    #[test]
    fn astrolabe_text_with_knowledge_inlines_notes() {
        let astrolabe = chart(Language::ZhCN);
        let pack = KnowledgePack::builtin(Language::ZhCN).unwrap();
        let text = astrolabe.to_text_with(&with(pack));
        assert!(text.contains("## 四化释义"));
        assert!(text.contains("**禄**: "));
        let soul = astrolabe.palace(Palace::Soul).unwrap();
        let star = soul.major_stars[0].key;
        let heading = format!("**{}", translate_star(star, Language::ZhCN));
        let soul_section = text.split("### 命宫 (").nth(1).unwrap();
        let next = soul_section.find("\n### ").unwrap_or(soul_section.len());
        assert!(
            soul_section[..next].contains(&heading),
            "{}",
            &soul_section[..next]
        );
        assert_ordered_subsequence(&astrolabe.to_text(), &text);
    }

    /// `needle` 的每一行按原顺序都能在 `haystack` 里找到整行匹配（同一渲染器的子集关系）
    fn assert_ordered_subsequence(needle: &str, haystack: &str) {
        let mut lines = haystack.lines();
        for want in needle.lines() {
            assert!(
                lines.any(|got| got == want),
                "带释义文本缺行或顺序不同：{want:?}"
            );
        }
    }

    /// 复合标签的分词随语言：中日相连，韩越英以空格分词；干支与四化标记同理
    #[test]
    fn label_sep_matches_language_convention() {
        for (lang, heading, mutagen) in [
            (Language::ZhTW, "命宮: 本命", "化祿"),
            (Language::JaJP, "命宮: 本命", "化禄"),
            (Language::KoKR, "명궁: 본명 ", " 화록"),
            (Language::ViVN, "Cung Mệnh: Bản mệnh ", " hóa Lộc"),
            (Language::EnUS, "Soul Palace: Natal ", " [A]"),
        ] {
            let astrolabe = chart(lang);
            let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
            let text = horoscope_to_text(&astrolabe, &horoscope, lang);
            assert!(
                text.contains(heading),
                "{lang:?} 期望含 {heading:?}\n{text}"
            );
            assert!(
                text.contains(mutagen),
                "{lang:?} 期望含 {mutagen:?}\n{text}"
            );
        }
        // 拉丁字母语言的干支分词
        let vi = chart(Language::ViVN).to_text();
        assert!(vi.contains("### Mệnh (Nhâm Ngọ)"), "{vi}");
        let en = chart(Language::EnUS).to_text();
        assert!(en.contains("### soul (ren woo)"), "{en}");
    }

    #[test]
    fn horoscope_text_zh_cn_covers_every_scope() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);
        for needle in [
            "# 运限 2024-10-1 (",
            "## 大限 · 命宫: 本命",
            "| 大限 | 本命 | 主星 | 辅星 | 流耀 |",
            "## 小限 · 命宫: 本命",
            "- 宫名: ",
            "## 流年 · 命宫: 本命",
            "| 流年 | 本命 | 主星 | 辅星 | 流耀 | 十二神 |",
            "岁前·",
            "## 流月 · 命宫: 本命",
            "- 流耀: ",
            "## 流日 · ",
            "## 流时 · ",
            "- 四化: ",
        ] {
            assert!(text.contains(needle), "缺少 {needle:?}\n{text}");
        }
    }

    /// 写到某一层为止：该层及之前的节都在，之后的节都不在；小限随流年。不设与写到流时逐字节相同
    #[test]
    fn horoscope_text_stops_at_the_requested_scope() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let sections = [
            "## 大限 · ",
            "## 小限 · ",
            "## 流年 · ",
            "## 流月 · ",
            "## 流日 · ",
            "## 流时 · ",
        ];
        for (through, kept) in [
            (Scope::Decadal, 1),
            (Scope::Yearly, 3),
            (Scope::Monthly, 4),
            (Scope::Daily, 5),
            (Scope::Hourly, 6),
        ] {
            let opts = TextOptions::new().horoscope_through(through).unwrap();
            let text = horoscope_to_text_with(&astrolabe, &horoscope, &opts, lang);
            for (i, heading) in sections.iter().enumerate() {
                assert_eq!(
                    text.contains(heading),
                    i < kept,
                    "{through:?} {heading}\n{text}"
                );
            }
        }
        let hourly = TextOptions::new().horoscope_through(Scope::Hourly).unwrap();
        assert_eq!(
            horoscope_to_text_with(&astrolabe, &horoscope, &hourly, lang),
            horoscope_to_text(&astrolabe, &horoscope, lang)
        );
        assert!(TextOptions::new().horoscope_through(Scope::Origin).is_err());
    }

    /// 只写到大限时，大限段不列要读小限与流年才判得出的风云际会：同一大限里各年的文本相同。
    /// 写到流年时它照列在大限段
    #[test]
    fn decadal_only_text_is_the_same_across_the_decade() {
        let astrolabe = by_solar(
            "1980-3-10",
            2,
            Gender::Male,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let decadal = TextOptions::new()
            .horoscope_through(Scope::Decadal)
            .unwrap();
        let text = |year: i64, opts: &TextOptions| {
            astrolabe
                .horoscope_of_year(year)
                .unwrap()
                .to_text_with(opts)
                .unwrap()
        };
        let with_age = text(2025, &TextOptions::new());
        assert!(with_age.contains("风云际会 (命宫)"), "{with_age}");
        let decadal_2025 = text(2025, &decadal);
        assert!(!decadal_2025.contains("风云际会"), "{decadal_2025}");
        // 两年同属一个大限，标题行的年份不同，其余逐字节相同
        let body = |t: &str| t.split_once('\n').unwrap().1.to_string();
        assert_eq!(body(&decadal_2025), body(&text(2027, &decadal)));
    }

    /// 按年、按月取的运限，不带选项的文本只写到查询精度
    #[test]
    fn horoscope_ref_text_stops_at_its_precision() {
        let astrolabe = chart(Language::ZhCN);
        let text = astrolabe.horoscope_of_year(2027).unwrap().to_text();
        assert!(
            text.contains("## 流年 · ") && !text.contains("## 流月 · "),
            "{text}"
        );
        let text = astrolabe.horoscope_of_month(2025, 7).unwrap().to_text();
        assert!(
            text.contains("## 流月 · ") && !text.contains("## 流日 · "),
            "{text}"
        );
    }

    /// 按年、按月取的运限，标题不写代表日：按年写农历年份与流年干支，按月再加流月干支。
    /// 节气分界下按正月取的代表日是立春次日，农历 2027 年正月初一在立春之后，那一天还在农历 2026 年，
    /// 标题仍写与流年干支相合的 2027
    #[test]
    fn horoscope_title_names_the_queried_year_and_month_not_the_representative_day() {
        let zh = chart(Language::ZhCN);
        let en = chart(Language::EnUS);
        let title = |text: String| text.lines().next().unwrap().to_string();
        for (h, want) in [
            (zh.horoscope_of_year(2027).unwrap(), "# 运限 2027 (丁未)"),
            (
                zh.horoscope_of_month(2025, 7).unwrap(),
                "# 运限 2025 (乙巳) · 流月 (甲申)",
            ),
            (
                en.horoscope_of_year(2027).unwrap(),
                "# Horoscope 2027 (ding wei)",
            ),
            (
                en.horoscope_of_month(2025, 7).unwrap(),
                "# Horoscope 2025 (yi si) · Monthly (jia shen)",
            ),
        ] {
            assert_eq!(title(h.to_text()), want, "{}", h.solar_date);
        }

        let exact = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config {
                horoscope_divide: HoroscopeDivide::Exact,
                ..Config::default()
            },
        )
        .unwrap();
        let h = exact.horoscope_of_month(2027, 1).unwrap();
        assert!(h.lunar_date.starts_with("二〇二六年"), "{}", h.lunar_date);
        assert_eq!(title(h.to_text()), "# 运限 2027 (丁未) · 流月 (壬寅)");
    }

    /// 未起运的盘（童限），大限段的标题写「童限」而非「大限」
    #[test]
    fn horoscope_text_uses_childhood_label_before_decadal_start() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2001-10-1", 0, lang).unwrap();
        assert_eq!(horoscope.decadal.name_key, HoroscopeName::Childhood);
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);
        assert!(text.contains("## 童限 · 命宫: 本命"));
        assert!(text.contains("| 童限 | 本命 |"));
        assert!(!text.contains("## 大限 · "));
    }

    #[test]
    fn horoscope_text_en_us_separates_compound_labels() {
        let lang = Language::EnUS;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);
        assert!(text.contains("## Decadal Fortune · Soul Palace: Natal "));
        assert!(text.contains("## Age Fortune · Soul Palace: Natal "));
        assert!(text.contains("| Yearly | Natal |"));
    }

    /// 六种语言都要有自己的结构标签，任一语言漏配都会退化成别的语言的文本。
    #[test]
    fn every_language_has_its_own_labels() {
        let cases = [
            (Language::ZhCN, "## 基本信息", "五行局: "),
            (Language::ZhTW, "## 基本資訊", "五行局: "),
            (Language::EnUS, "## Basic Info", "Five Elements Class: "),
            (Language::JaJP, "## 基本情報", "五行局: "),
            (Language::KoKR, "## 기본 정보", "오행국: "),
            (Language::ViVN, "## Thông tin cơ bản", "Cục ngũ hành: "),
        ];
        for (lang, header, five_elements) in cases {
            let text = chart(lang).to_text();
            assert!(text.contains(header), "{lang:?} 缺少基本信息标题");
            assert!(text.contains(five_elements), "{lang:?} 缺少五行局标签");
        }
    }

    /// 自由函数传任意语言都输出纯该语言文本：zh-CN 盘按 en-US 渲染，
    /// 与原生 en-US 盘的渲染逐字节一致。
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
        assert_eq!(
            surrounded_palaces_to_text(&sp_zh, Language::EnUS),
            surrounded_palaces_to_text(&sp_en, Language::EnUS)
        );
    }

    /// 同宫主星的组合解读：包里只记在一方名下也要出，且只出一次、排在单星释义之前
    #[test]
    fn star_notes_find_combinations_from_either_side() {
        let astrolabe = chart(Language::ZhCN);
        let palace = astrolabe
            .palaces
            .iter()
            .find(|p| p.major_stars.len() >= 2)
            .expect("测试盘应有双主星宫");
        let (a, b) = (palace.major_stars[0].key, palace.major_stars[1].key);
        let make = |json: serde_json::Value| KnowledgePack::from_value(&json).unwrap();
        let base = serde_json::json!({"schema": 1, "id": "t", "version": "1", "language": "zh-CN"});
        let palace_ref = astrolabe.palace(palace.index).unwrap();

        let mut only_b = base.clone();
        only_b["stars"] = serde_json::json!({
            b.as_key(): {"intro": "B 的释义", "combinations": {a.as_key(): "AB 同宫解读"}}
        });
        let pack = make(only_b);
        let text = palace_to_text_with(&palace_ref, &with(&pack), Language::ZhCN);
        assert_eq!(text.matches("AB 同宫解读").count(), 1, "{text}");

        let mut both = base.clone();
        both["stars"] = serde_json::json!({
            a.as_key(): {"intro": "A 的释义", "combinations": {b.as_key(): "AB 同宫解读"}},
            b.as_key(): {"intro": "B 的释义", "combinations": {a.as_key(): "AB 同宫解读"}}
        });
        let pack = make(both);
        let text = palace_to_text_with(&palace_ref, &with(&pack), Language::ZhCN);
        assert_eq!(text.matches("AB 同宫解读").count(), 1, "{text}");
        assert!(text.contains("A 的释义") && text.contains("B 的释义"));
        assert!(text.find("AB 同宫解读").unwrap() < text.find("A 的释义").unwrap());
    }

    /// 单宫文本与本命盘详解里该宫的段落一致
    #[test]
    fn palace_text_matches_astrolabe_section() {
        let astrolabe = chart(Language::ZhCN);
        let palace = astrolabe.palace(Palace::Soul).unwrap();
        let palace_text = palace.to_text();
        assert!(palace_text.starts_with("### 命宫 ("));
        assert!(astrolabe.to_text().contains(&palace_text));
    }

    /// 三方四正文本列出四宫段落
    #[test]
    fn surrounded_palaces_text_lists_four_palaces() {
        let astrolabe = chart(Language::ZhCN);
        let sp = astrolabe.surrounded_palaces(Palace::Soul).unwrap();
        let text = sp.to_text();
        assert!(text.starts_with("## 命宫 三方四正\n"));
        assert_eq!(text.matches("\n### ").count(), 4);
        for role in [
            "### 本宫 · 命宫 (",
            "### 对宫 · ",
            "### 财帛位 · ",
            "### 官禄位 · ",
        ] {
            assert!(text.contains(role), "缺少 {role:?}\n{text}");
        }
    }

    /// 运限段落里四化落宫标明本命参照系，流耀落宫用该层宫名
    #[test]
    fn scope_mutagen_places_are_in_natal_frame() {
        let lang = Language::ZhCN;
        let astrolabe = chart(lang);
        let horoscope = get_horoscope(&astrolabe, "2024-10-1", 0, lang).unwrap();
        let text = horoscope_to_text(&astrolabe, &horoscope, lang);
        let line = text.lines().find(|l| l.starts_with("- 四化: ")).unwrap();
        assert!(line.contains("→本命"), "{line}");
        let flow = text.lines().find(|l| l.starts_with("- 流耀: ")).unwrap();
        assert!(!flow.contains("→本命"), "{flow}");
    }

    /// TextOptions 的格局口径同时作用于格局节与带释义时的格局释义
    #[test]
    fn pattern_config_reaches_pattern_section() {
        // 该盘在两个亮度口径下命中数不同（位置法 3、表法 2），两侧都非空，
        // 故格局节与释义节都有可比对的内容
        let astrolabe = by_solar(
            "1990-10-10",
            0,
            Gender::Male,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let positional = PatternConfig {
            brightness_source: crate::pattern::BrightnessSource::Positional,
            ..PatternConfig::default()
        };
        let default_text = astrolabe.to_text();
        let positional_text =
            astrolabe.to_text_with(&TextOptions::new().pattern_config(&positional));
        let hits_pos = astrolabe.patterns_with(&positional);
        let hits_def = astrolabe.patterns();
        assert_ne!(
            hits_pos.len(),
            hits_def.len(),
            "测试盘应在两口径下命中数不同"
        );
        for h in &hits_pos {
            let name = format!("**{}**", translate_pattern(h.key, Language::ZhCN));
            assert!(
                positional_text.contains(&name),
                "{name} 应出现在 positional 文本"
            );
        }
        assert_ne!(default_text, positional_text);
    }
}

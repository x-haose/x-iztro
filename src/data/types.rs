use serde::{Deserialize, Serialize};

/// 阴阳
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YinYang {
    /// 阳
    Yang,
    /// 阴
    Yin,
}

impl YinYang {
    /// 单字写法
    ///
    /// 阴阳在 iztro 中不参与国际化，六种语言下都是这两个汉字。
    pub fn as_str(self) -> &'static str {
        match self {
            YinYang::Yang => "阳",
            YinYang::Yin => "阴",
        }
    }
}

/// 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiveElements {
    /// 木
    Wood,
    /// 金
    Metal,
    /// 水
    Water,
    /// 火
    Fire,
    /// 土
    Earth,
}

impl FiveElements {
    /// 单字写法
    ///
    /// 五行在 iztro 中不参与国际化，六种语言下都是这五个汉字。
    pub fn as_str(self) -> &'static str {
        match self {
            FiveElements::Wood => "木",
            FiveElements::Metal => "金",
            FiveElements::Water => "水",
            FiveElements::Fire => "火",
            FiveElements::Earth => "土",
        }
    }
}

/// 天干
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum HeavenlyStem {
    /// 甲
    Jia,
    /// 乙
    Yi,
    /// 丙
    Bing,
    /// 丁
    Ding,
    /// 戊
    Wu,
    /// 己
    Ji,
    /// 庚
    Geng,
    /// 辛
    Xin,
    /// 壬
    Ren,
    /// 癸
    Gui,
}

/// 地支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum EarthlyBranch {
    /// 子
    Zi,
    /// 丑
    Chou,
    /// 寅
    Yin,
    /// 卯
    Mao,
    /// 辰
    Chen,
    /// 巳
    Si,
    /// 午
    Wu,
    /// 未
    Wei,
    /// 申
    Shen,
    /// 酉
    You,
    /// 戌
    Xu,
    /// 亥
    Hai,
}

/// 十二宫位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(usize)]
pub enum Palace {
    /// 命宫
    Soul,
    /// 父母
    Parents,
    /// 福德
    Spirit,
    /// 田宅
    Property,
    /// 官禄
    Career,
    /// 交友
    Friends,
    /// 迁移
    Surface,
    /// 疾厄
    Health,
    /// 财帛
    Wealth,
    /// 子女
    Children,
    /// 夫妻
    Spouse,
    /// 兄弟
    Siblings,
}

/// 五行局
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum FiveElementsClass {
    /// 水二局
    Water2nd = 2,
    /// 木三局
    Wood3rd = 3,
    /// 金四局
    Metal4th = 4,
    /// 土五局
    Earth5th = 5,
    /// 火六局
    Fire6th = 6,
}

/// 四化
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mutagen {
    /// 禄
    Lu,
    /// 权
    Quan,
    /// 科
    Ke,
    /// 忌
    Ji,
}

/// 星曜亮度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    /// 庙
    Miao,
    /// 旺
    Wang,
    /// 得
    De,
    /// 利
    Li,
    /// 平
    Ping,
    /// 不
    Bu,
    /// 陷
    Xian,
}

/// 星曜类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum StarType {
    /// 主星
    Major,
    /// 吉星
    Soft,
    /// 煞星
    Tough,
    /// 杂耀
    Adjective,
    /// 桃花星
    Flower,
    /// 解神
    Helper,
    /// 禄存
    Lucun,
    /// 天马
    Tianma,
}

impl StarType {
    /// 语言无关标识（与 JS iztro 的 `type` 取值一致）
    pub fn as_key(self) -> &'static str {
        match self {
            StarType::Major => "major",
            StarType::Soft => "soft",
            StarType::Tough => "tough",
            StarType::Adjective => "adjective",
            StarType::Flower => "flower",
            StarType::Helper => "helper",
            StarType::Lucun => "lucun",
            StarType::Tianma => "tianma",
        }
    }
}

/// 运限范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Scope {
    /// 本命
    Origin,
    /// 大限
    Decadal,
    /// 流年
    Yearly,
    /// 流月
    Monthly,
    /// 流日
    Daily,
    /// 流时
    Hourly,
}

impl Scope {
    /// 语言无关标识（与 JS iztro 的 `scope` 取值一致）
    pub fn as_key(self) -> &'static str {
        match self {
            Scope::Origin => "origin",
            Scope::Decadal => "decadal",
            Scope::Yearly => "yearly",
            Scope::Monthly => "monthly",
            Scope::Daily => "daily",
            Scope::Hourly => "hourly",
        }
    }

    /// 由语言无关标识还原；未知标识返回 `None`
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "origin" => Some(Scope::Origin),
            "decadal" => Some(Scope::Decadal),
            "yearly" => Some(Scope::Yearly),
            "monthly" => Some(Scope::Monthly),
            "daily" => Some(Scope::Daily),
            "hourly" => Some(Scope::Hourly),
            _ => None,
        }
    }
}

/// 有流耀与运限四化的五个层级，按大限、流年、流月、流日、流时排列（本命与小限不在此列）。
pub const HOROSCOPE_SCOPES: [Scope; 5] = [
    Scope::Decadal,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

/// 运限层级显示名
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoroscopeName {
    /// 大限
    Decadal,
    /// 童限（未起运时的大限位）
    Childhood,
    /// 小限
    Age,
    /// 流年
    Yearly,
    /// 流月
    Monthly,
    /// 流日
    Daily,
    /// 流时
    Hourly,
}

impl HoroscopeName {
    /// 语言无关标识（iztro i18n key；小限为 `turn`）
    pub fn as_key(self) -> &'static str {
        match self {
            HoroscopeName::Decadal => "decadal",
            HoroscopeName::Childhood => "childhood",
            HoroscopeName::Age => "turn",
            HoroscopeName::Yearly => "yearly",
            HoroscopeName::Monthly => "monthly",
            HoroscopeName::Daily => "daily",
            HoroscopeName::Hourly => "hourly",
        }
    }

    /// 由语言无关标识还原；未知标识返回 `None`
    pub fn from_key(key: &str) -> Option<Self> {
        match key {
            "decadal" => Some(HoroscopeName::Decadal),
            "childhood" => Some(HoroscopeName::Childhood),
            "turn" => Some(HoroscopeName::Age),
            "yearly" => Some(HoroscopeName::Yearly),
            "monthly" => Some(HoroscopeName::Monthly),
            "daily" => Some(HoroscopeName::Daily),
            "hourly" => Some(HoroscopeName::Hourly),
            _ => None,
        }
    }
}

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// 男
    Male,
    /// 女
    Female,
}

impl Gender {
    /// 性别的阴阳：男为阳、女为阴，决定大限与长生十二神的顺逆
    pub fn yin_yang(self) -> YinYang {
        match self {
            Gender::Male => YinYang::Yang,
            Gender::Female => YinYang::Yin,
        }
    }
}

/// 语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    /// 简体中文
    ZhCN,
    /// 繁体中文
    ZhTW,
    /// 英文
    EnUS,
    /// 日文
    JaJP,
    /// 韩文
    KoKR,
    /// 越南文
    ViVN,
}

impl Language {
    /// 语言代码，取值与 iztro 的 `Language` 一致
    pub fn as_code(self) -> &'static str {
        match self {
            Language::ZhCN => "zh-CN",
            Language::ZhTW => "zh-TW",
            Language::EnUS => "en-US",
            Language::JaJP => "ja-JP",
            Language::KoKR => "ko-KR",
            Language::ViVN => "vi-VN",
        }
    }

    /// 由语言代码还原，大小写不敏感，连字符与下划线等价（`zh-CN` / `zh_cn` 都可）；
    /// 未知代码返回 `None`
    pub fn from_code(code: &str) -> Option<Self> {
        match code.to_ascii_lowercase().replace('_', "-").as_str() {
            "zh-cn" => Some(Language::ZhCN),
            "zh-tw" => Some(Language::ZhTW),
            "en-us" => Some(Language::EnUS),
            "ja-jp" => Some(Language::JaJP),
            "ko-kr" => Some(Language::KoKR),
            "vi-vn" => Some(Language::ViVN),
            _ => None,
        }
    }
}

/// 算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Algorithm {
    /// 默认
    Default,
    /// 中州派
    Zhongzhou,
}

/// 排盘视角：中州派把同一组出生数据看作三张盘，差别在于用哪一宫的干支起五行局。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum AstroType {
    /// 天盘：以命宫干支起五行局，即常规排盘结果
    Heaven,
    /// 地盘：以身宫干支起五行局，身宫即为新盘的命宫
    Earth,
    /// 人盘：以福德宫干支起五行局，福德宫即为新盘的命宫
    Human,
}

/// 农历输入的闰月处理方式（`by_lunar` 专用）。
///
/// 把 iztro `byLunar` 的 `isLeapMonth` 与 `fixLeap` 两个布尔合成一个三态值：
/// 两个布尔相邻传参极易写反且不报错，而 `fixLeap` 只在输入是闰月时才有意义，
/// 三态恰好覆盖全部有效组合。阳历排盘的 `fix_leap` 仍是单个布尔（见 [`by_solar`]）。
///
/// [`by_solar`]: crate::by_solar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeapMonth {
    /// 输入的农历月不是闰月
    NotLeap,
    /// 输入的农历月是闰月，按闰月本身排盘
    Leap,
    /// 输入的农历月是闰月，且十五之后视作次月（iztro `fixLeap`）
    LeapFixed,
}

impl LeapMonth {
    /// 由 iztro 风格的两个布尔（`isLeapMonth`、`fixLeap`）合成；非闰月时 `fix_leap` 被忽略
    pub fn from_flags(is_leap_month: bool, fix_leap: bool) -> Self {
        match (is_leap_month, fix_leap) {
            (false, _) => LeapMonth::NotLeap,
            (true, false) => LeapMonth::Leap,
            (true, true) => LeapMonth::LeapFixed,
        }
    }

    /// 输入月是否闰月
    pub fn is_leap_month(self) -> bool {
        !matches!(self, LeapMonth::NotLeap)
    }

    /// 是否按 iztro `fixLeap` 规则把闰月十五之后视作次月
    pub fn fix_leap(self) -> bool {
        matches!(self, LeapMonth::LeapFixed)
    }
}

/// 年分界点：排盘年干支（及其驱动的四化、命主身主等）按哪一天换年
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YearDivide {
    /// 正月初一分界
    Normal,
    /// 立春分界
    Exact,
}

/// 运限分界点：运限干支与干支纪日的月柱按初一还是节气推算
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HoroscopeDivide {
    /// 年按正月初一分界，月按初一以五虎遁推算
    Normal,
    /// 年按立春分界，月按节气分界
    Exact,
}

/// 虚岁分界点
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgeDivide {
    /// 以自然农历年为界，跨年即加一岁
    Normal,
    /// 以生日为界，过了生日才加一岁
    Birthday,
}

/// 晚子时（23:00-00:00）归属
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DayDivide {
    /// 晚子时归次日
    Forward,
    /// 晚子时归当天（按当日早子时排盘）
    Current,
}

/// 为配置开关枚举生成语言无关标识的双向映射。
///
/// 绑定层 config JSON 的取值与这里一一对应，是该映射的唯一来源——
/// 解析入参与回写 `ConfigDto` 都走它，两处不会各抄一份而走样。
macro_rules! config_switch_keys {
    ($ty:ident { $($variant:ident => $key:literal),+ $(,)? }) => {
        impl $ty {
            /// 语言无关标识（与 JS iztro 同名配置项的取值一致）
            pub fn as_key(self) -> &'static str {
                match self {
                    $($ty::$variant => $key),+
                }
            }

            /// 由语言无关标识还原；未知标识返回 `None`
            pub fn from_key(key: &str) -> Option<Self> {
                match key {
                    $($key => Some($ty::$variant),)+
                    _ => None,
                }
            }
        }
    };
}

config_switch_keys!(YearDivide { Normal => "normal", Exact => "exact" });
config_switch_keys!(HoroscopeDivide { Normal => "normal", Exact => "exact" });
config_switch_keys!(AgeDivide { Normal => "normal", Birthday => "birthday" });
config_switch_keys!(DayDivide { Forward => "forward", Current => "current" });
config_switch_keys!(Algorithm { Default => "default", Zhongzhou => "zhongzhou" });
config_switch_keys!(AstroType { Heaven => "heaven", Earth => "earth", Human => "human" });
config_switch_keys!(LeapMonth { NotLeap => "notLeap", Leap => "leap", LeapFixed => "leapFixed" });

/// 自定义四化与亮度表。
///
/// 紫微斗数流派众多，四化与星耀亮度是分歧最集中的两处。这里按 key **整表替换**
/// 默认值：给出某个天干的四化就只改那个天干，未给出的天干仍用默认表；亮度同理。
///
/// 通过 [`Config::with_mutagens`] / [`Config::with_brightness`] 构造。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableOverrides {
    /// 天干 → 该干化出的四颗星，顺序为禄、权、科、忌
    mutagens: std::collections::HashMap<HeavenlyStem, [crate::data::stars::StarKey; 4]>,
    /// 星耀 → 它在十二宫（寅宫为 0）各自的亮度，无亮度的位置为 None
    brightness: std::collections::HashMap<crate::data::stars::StarKey, [Option<Brightness>; 12]>,
}

impl TableOverrides {
    /// 覆盖某个天干的四化表。
    pub fn set_mutagens(&mut self, stem: HeavenlyStem, stars: [crate::data::stars::StarKey; 4]) {
        self.mutagens.insert(stem, stars);
    }

    /// 覆盖某颗星的十二宫亮度表。
    pub fn set_brightness(
        &mut self,
        star: crate::data::stars::StarKey,
        table: [Option<Brightness>; 12],
    ) {
        self.brightness.insert(star, table);
    }

    /// 取该天干被覆盖的四化表；未覆盖时返回 `None`。
    pub fn mutagens_of(&self, stem: HeavenlyStem) -> Option<&[crate::data::stars::StarKey; 4]> {
        self.mutagens.get(&stem)
    }

    /// 取该星被覆盖的亮度表；未覆盖时返回 `None`。
    pub fn brightness_of(
        &self,
        star: crate::data::stars::StarKey,
    ) -> Option<&[Option<Brightness>; 12]> {
        self.brightness.get(&star)
    }

    /// 是否没有任何覆盖。
    pub fn is_empty(&self) -> bool {
        self.mutagens.is_empty() && self.brightness.is_empty()
    }
}

/// 排盘配置：控制分界点、算法派别与自定义表的全部开关
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    /// 年分界点
    pub year_divide: YearDivide,
    /// 运限分界点
    pub horoscope_divide: HoroscopeDivide,
    /// 虚岁分界点
    pub age_divide: AgeDivide,
    /// 晚子时归属
    pub day_divide: DayDivide,
    /// 算法派别
    pub algorithm: Algorithm,
    /// 排盘视角（天盘/地盘/人盘）
    pub astro_type: AstroType,
    /// 自定义四化与亮度表；`None` 表示全部使用默认表。
    ///
    /// 不参与序列化：它是排盘的输入配置，不属于排盘结果，
    /// 加入 DTO 会破坏与 JS iztro 的字段契约。
    #[serde(skip)]
    pub overrides: Option<std::sync::Arc<TableOverrides>>,
}

impl Default for Config {
    /// 与 JS iztro 的默认配置一致
    fn default() -> Self {
        Config {
            year_divide: YearDivide::Normal,
            horoscope_divide: HoroscopeDivide::Normal,
            age_divide: AgeDivide::Normal,
            day_divide: DayDivide::Forward,
            algorithm: Algorithm::Default,
            astro_type: AstroType::Heaven,
            overrides: None,
        }
    }
}

impl Config {
    /// 在当前配置上指定排盘视角。
    pub fn with_astro_type(mut self, astro_type: AstroType) -> Self {
        self.astro_type = astro_type;
        self
    }

    /// 在当前配置上覆盖某个天干的四化表。
    pub fn with_mutagens(
        mut self,
        stem: HeavenlyStem,
        stars: [crate::data::stars::StarKey; 4],
    ) -> Self {
        let mut tables = self
            .overrides
            .take()
            .map_or_else(TableOverrides::default, |arc| {
                std::sync::Arc::try_unwrap(arc).unwrap_or_else(|shared| (*shared).clone())
            });
        tables.set_mutagens(stem, stars);
        self.overrides = Some(std::sync::Arc::new(tables));
        self
    }

    /// 在当前配置上覆盖某颗星的十二宫亮度表。
    pub fn with_brightness(
        mut self,
        star: crate::data::stars::StarKey,
        table: [Option<Brightness>; 12],
    ) -> Self {
        let mut tables = self
            .overrides
            .take()
            .map_or_else(TableOverrides::default, |arc| {
                std::sync::Arc::try_unwrap(arc).unwrap_or_else(|shared| (*shared).clone())
            });
        tables.set_brightness(star, table);
        self.overrides = Some(std::sync::Arc::new(tables));
        self
    }

    /// 该天干实际生效的四化表：有覆盖用覆盖，否则用默认表。
    pub fn mutagens_of(&self, stem: HeavenlyStem) -> [crate::data::stars::StarKey; 4] {
        self.overrides
            .as_ref()
            .and_then(|t| t.mutagens_of(stem))
            .copied()
            .unwrap_or_else(|| crate::data::heavenly_stems::get_heavenly_stem_info(stem).mutagen)
    }

    /// 该星在指定宫位实际生效的亮度：有覆盖用覆盖，否则用默认表。
    ///
    /// `palace_index` 为盘上位置（寅宫为 0），越界会对 12 取模。
    pub fn brightness_of(
        &self,
        star: crate::data::stars::StarKey,
        palace_index: usize,
    ) -> Option<Brightness> {
        let index = palace_index % 12;
        if let Some(table) = self.overrides.as_ref().and_then(|t| t.brightness_of(star)) {
            return table[index];
        }
        crate::data::stars::get_brightness_table(star)?[index]
    }
}

/// 时辰索引
pub type TimeIndex = u8;

impl HeavenlyStem {
    /// 天干序号（甲=0 … 癸=9）
    pub fn index(&self) -> usize {
        *self as usize
    }
    /// 由序号取天干（对 10 取模）
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::HEAVENLY_STEMS[i % 10]
    }
}

impl EarthlyBranch {
    /// 地支序号（子=0 … 亥=11）
    pub fn index(&self) -> usize {
        *self as usize
    }
    /// 由序号取地支（对 12 取模）
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::EARTHLY_BRANCHES[i % 12]
    }
}

impl Palace {
    /// 宫位在 [`crate::data::constants::PALACES`] 中的序号
    pub fn index(&self) -> usize {
        *self as usize
    }
    /// 由序号取宫位名（对 12 取模）
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::PALACES[i % 12]
    }
}

impl FiveElementsClass {
    /// 五行局数值（水二局=2 … 火六局=6）
    pub fn value(&self) -> usize {
        *self as usize
    }
}

impl Palace {
    /// 语言无关的宫位标识（iztro i18n key，如 "soulPalace"）。
    pub fn as_key(&self) -> &'static str {
        match self {
            Palace::Soul => "soulPalace",
            Palace::Parents => "parentsPalace",
            Palace::Spirit => "spiritPalace",
            Palace::Property => "propertyPalace",
            Palace::Career => "careerPalace",
            Palace::Friends => "friendsPalace",
            Palace::Surface => "surfacePalace",
            Palace::Health => "healthPalace",
            Palace::Wealth => "wealthPalace",
            Palace::Children => "childrenPalace",
            Palace::Spouse => "spousePalace",
            Palace::Siblings => "siblingsPalace",
        }
    }
}

impl HeavenlyStem {
    /// 语言无关的天干标识（iztro i18n key，如 "jiaHeavenly"）。
    pub fn as_key(&self) -> &'static str {
        match self {
            HeavenlyStem::Jia => "jiaHeavenly",
            HeavenlyStem::Yi => "yiHeavenly",
            HeavenlyStem::Bing => "bingHeavenly",
            HeavenlyStem::Ding => "dingHeavenly",
            HeavenlyStem::Wu => "wuHeavenly",
            HeavenlyStem::Ji => "jiHeavenly",
            HeavenlyStem::Geng => "gengHeavenly",
            HeavenlyStem::Xin => "xinHeavenly",
            HeavenlyStem::Ren => "renHeavenly",
            HeavenlyStem::Gui => "guiHeavenly",
        }
    }
}

impl EarthlyBranch {
    /// 语言无关的地支标识（iztro i18n key，如 "ziEarthly"）。
    pub fn as_key(&self) -> &'static str {
        match self {
            EarthlyBranch::Zi => "ziEarthly",
            EarthlyBranch::Chou => "chouEarthly",
            EarthlyBranch::Yin => "yinEarthly",
            EarthlyBranch::Mao => "maoEarthly",
            EarthlyBranch::Chen => "chenEarthly",
            EarthlyBranch::Si => "siEarthly",
            EarthlyBranch::Wu => "wuEarthly",
            EarthlyBranch::Wei => "weiEarthly",
            EarthlyBranch::Shen => "shenEarthly",
            EarthlyBranch::You => "youEarthly",
            EarthlyBranch::Xu => "xuEarthly",
            EarthlyBranch::Hai => "haiEarthly",
        }
    }
}

impl Mutagen {
    /// 语言无关的四化标识（iztro i18n key，如 "sihuaLu"）。
    pub fn as_key(&self) -> &'static str {
        match self {
            Mutagen::Lu => "sihuaLu",
            Mutagen::Quan => "sihuaQuan",
            Mutagen::Ke => "sihuaKe",
            Mutagen::Ji => "sihuaJi",
        }
    }
}

impl Brightness {
    /// 语言无关的亮度标识（iztro i18n key，如 "miao"）。
    pub fn as_key(&self) -> &'static str {
        match self {
            Brightness::Miao => "miao",
            Brightness::Wang => "wang",
            Brightness::De => "de",
            Brightness::Li => "li",
            Brightness::Ping => "ping",
            Brightness::Bu => "bu",
            Brightness::Xian => "xian",
        }
    }
}

impl FiveElementsClass {
    /// 语言无关的五行局标识（"water2nd" 等）。
    pub fn as_key(&self) -> &'static str {
        match self {
            FiveElementsClass::Water2nd => "water2nd",
            FiveElementsClass::Wood3rd => "wood3rd",
            FiveElementsClass::Metal4th => "metal4th",
            FiveElementsClass::Earth5th => "earth5th",
            FiveElementsClass::Fire6th => "fire6th",
        }
    }
}

impl Palace {
    /// 由语言无关标识反查宫位名；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<Palace> {
        match key {
            "soulPalace" => Some(Palace::Soul),
            "parentsPalace" => Some(Palace::Parents),
            "spiritPalace" => Some(Palace::Spirit),
            "propertyPalace" => Some(Palace::Property),
            "careerPalace" => Some(Palace::Career),
            "friendsPalace" => Some(Palace::Friends),
            "surfacePalace" => Some(Palace::Surface),
            "healthPalace" => Some(Palace::Health),
            "wealthPalace" => Some(Palace::Wealth),
            "childrenPalace" => Some(Palace::Children),
            "spousePalace" => Some(Palace::Spouse),
            "siblingsPalace" => Some(Palace::Siblings),
            _ => None,
        }
    }
}

impl HeavenlyStem {
    /// 由语言无关标识反查天干；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<HeavenlyStem> {
        match key {
            "jiaHeavenly" => Some(HeavenlyStem::Jia),
            "yiHeavenly" => Some(HeavenlyStem::Yi),
            "bingHeavenly" => Some(HeavenlyStem::Bing),
            "dingHeavenly" => Some(HeavenlyStem::Ding),
            "wuHeavenly" => Some(HeavenlyStem::Wu),
            "jiHeavenly" => Some(HeavenlyStem::Ji),
            "gengHeavenly" => Some(HeavenlyStem::Geng),
            "xinHeavenly" => Some(HeavenlyStem::Xin),
            "renHeavenly" => Some(HeavenlyStem::Ren),
            "guiHeavenly" => Some(HeavenlyStem::Gui),
            _ => None,
        }
    }
}

impl EarthlyBranch {
    /// 由语言无关标识反查地支；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<EarthlyBranch> {
        match key {
            "ziEarthly" => Some(EarthlyBranch::Zi),
            "chouEarthly" => Some(EarthlyBranch::Chou),
            "yinEarthly" => Some(EarthlyBranch::Yin),
            "maoEarthly" => Some(EarthlyBranch::Mao),
            "chenEarthly" => Some(EarthlyBranch::Chen),
            "siEarthly" => Some(EarthlyBranch::Si),
            "wuEarthly" => Some(EarthlyBranch::Wu),
            "weiEarthly" => Some(EarthlyBranch::Wei),
            "shenEarthly" => Some(EarthlyBranch::Shen),
            "youEarthly" => Some(EarthlyBranch::You),
            "xuEarthly" => Some(EarthlyBranch::Xu),
            "haiEarthly" => Some(EarthlyBranch::Hai),
            _ => None,
        }
    }
}

impl Mutagen {
    /// 由语言无关标识反查四化；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<Mutagen> {
        match key {
            "sihuaLu" => Some(Mutagen::Lu),
            "sihuaQuan" => Some(Mutagen::Quan),
            "sihuaKe" => Some(Mutagen::Ke),
            "sihuaJi" => Some(Mutagen::Ji),
            _ => None,
        }
    }
}

impl Brightness {
    /// 由语言无关标识反查亮度；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<Brightness> {
        match key {
            "miao" => Some(Brightness::Miao),
            "wang" => Some(Brightness::Wang),
            "de" => Some(Brightness::De),
            "li" => Some(Brightness::Li),
            "ping" => Some(Brightness::Ping),
            "bu" => Some(Brightness::Bu),
            "xian" => Some(Brightness::Xian),
            _ => None,
        }
    }
}

impl FiveElementsClass {
    /// 由语言无关标识反查五行局；标识未知时返回 `None`。
    pub fn from_key(key: &str) -> Option<FiveElementsClass> {
        match key {
            "water2nd" => Some(FiveElementsClass::Water2nd),
            "wood3rd" => Some(FiveElementsClass::Wood3rd),
            "metal4th" => Some(FiveElementsClass::Metal4th),
            "earth5th" => Some(FiveElementsClass::Earth5th),
            "fire6th" => Some(FiveElementsClass::Fire6th),
            _ => None,
        }
    }
}

#[cfg(test)]
mod key_roundtrip_tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::constants::{EARTHLY_BRANCHES, HEAVENLY_STEMS, PALACES};
    use crate::data::stars::StarKey;

    /// 语言代码大小写与连字符/下划线写法都能还原，`as_code` 与 `from_code` 互逆。
    #[test]
    fn test_language_code_aliases() {
        for lang in [
            Language::ZhCN,
            Language::ZhTW,
            Language::EnUS,
            Language::JaJP,
            Language::KoKR,
            Language::ViVN,
        ] {
            let code = lang.as_code();
            assert_eq!(Language::from_code(code), Some(lang));
            assert_eq!(Language::from_code(&code.to_lowercase()), Some(lang));
            assert_eq!(Language::from_code(&code.replace('-', "_")), Some(lang));
        }
        assert_eq!(Language::from_code("zh_cn"), Some(Language::ZhCN));
        assert_eq!(Language::from_code("klingon"), None);
    }

    /// `as_key` 与 `from_key` 必须互为逆运算，否则绑定层的 key 往返会失真。
    #[test]
    fn test_enum_key_roundtrip() {
        for p in PALACES {
            assert_eq!(Palace::from_key(p.as_key()), Some(p), "{p:?}");
        }
        for s in HEAVENLY_STEMS {
            assert_eq!(HeavenlyStem::from_key(s.as_key()), Some(s), "{s:?}");
        }
        for b in EARTHLY_BRANCHES {
            assert_eq!(EarthlyBranch::from_key(b.as_key()), Some(b), "{b:?}");
        }
        for m in [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            assert_eq!(Mutagen::from_key(m.as_key()), Some(m), "{m:?}");
        }
        for b in [
            Brightness::Miao,
            Brightness::Wang,
            Brightness::De,
            Brightness::Li,
            Brightness::Ping,
            Brightness::Bu,
            Brightness::Xian,
        ] {
            assert_eq!(Brightness::from_key(b.as_key()), Some(b), "{b:?}");
        }
        for c in [
            FiveElementsClass::Water2nd,
            FiveElementsClass::Wood3rd,
            FiveElementsClass::Metal4th,
            FiveElementsClass::Earth5th,
            FiveElementsClass::Fire6th,
        ] {
            assert_eq!(FiveElementsClass::from_key(c.as_key()), Some(c), "{c:?}");
        }

        for v in [YearDivide::Normal, YearDivide::Exact] {
            assert_eq!(YearDivide::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [HoroscopeDivide::Normal, HoroscopeDivide::Exact] {
            assert_eq!(HoroscopeDivide::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [AgeDivide::Normal, AgeDivide::Birthday] {
            assert_eq!(AgeDivide::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [DayDivide::Forward, DayDivide::Current] {
            assert_eq!(DayDivide::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [Algorithm::Default, Algorithm::Zhongzhou] {
            assert_eq!(Algorithm::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [AstroType::Heaven, AstroType::Earth, AstroType::Human] {
            assert_eq!(AstroType::from_key(v.as_key()), Some(v), "{v:?}");
        }
        for v in [LeapMonth::NotLeap, LeapMonth::Leap, LeapMonth::LeapFixed] {
            assert_eq!(LeapMonth::from_key(v.as_key()), Some(v), "{v:?}");
            assert_eq!(
                LeapMonth::from_flags(v.is_leap_month(), v.fix_leap()),
                v,
                "{v:?}"
            );
        }
        assert_eq!(LeapMonth::from_flags(false, true), LeapMonth::NotLeap);

        assert_eq!(Palace::from_key("bodyPalace"), None);
        assert_eq!(StarKey::from_key("nope"), None);
        assert_eq!(Algorithm::from_key("normal"), None);
    }

    /// 盘上出现的每一颗星（含运限流耀）都要能由 key 还原。
    #[test]
    fn test_star_key_roundtrip_over_chart() {
        let chart = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();

        let mut checked = 0;
        for palace in &chart.palaces {
            for star in palace
                .major_stars
                .iter()
                .chain(palace.minor_stars.iter())
                .chain(palace.adjective_stars.iter())
            {
                assert_eq!(
                    StarKey::from_key(star.key.as_key()),
                    Some(star.key),
                    "{:?} 的 key 往返失败",
                    star.key
                );
                checked += 1;
            }
            assert_eq!(
                StarKey::from_key(palace.changsheng12.as_key()),
                Some(palace.changsheng12)
            );
            assert_eq!(
                StarKey::from_key(palace.suiqian12.as_key()),
                Some(palace.suiqian12)
            );
        }
        assert!(checked > 60, "覆盖的星耀太少：{checked}");
    }
}

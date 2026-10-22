use std::collections::BTreeMap;

use crate::data::keyed::keyed_enum;
use crate::data::stars::StarKey;

/// 阴阳
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

keyed_enum!(HeavenlyStem, "heavenly stem" {
    Jia => "jiaHeavenly",
    Yi => "yiHeavenly",
    Bing => "bingHeavenly",
    Ding => "dingHeavenly",
    Wu => "wuHeavenly",
    Ji => "jiHeavenly",
    Geng => "gengHeavenly",
    Xin => "xinHeavenly",
    Ren => "renHeavenly",
    Gui => "guiHeavenly",
});

/// 地支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

keyed_enum!(EarthlyBranch, "earthly branch" {
    Zi => "ziEarthly",
    Chou => "chouEarthly",
    Yin => "yinEarthly",
    Mao => "maoEarthly",
    Chen => "chenEarthly",
    Si => "siEarthly",
    Wu => "wuEarthly",
    Wei => "weiEarthly",
    Shen => "shenEarthly",
    You => "youEarthly",
    Xu => "xuEarthly",
    Hai => "haiEarthly",
});

/// 十二宫位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

keyed_enum!(Palace, "palace" {
    Soul => "soulPalace",
    Parents => "parentsPalace",
    Spirit => "spiritPalace",
    Property => "propertyPalace",
    Career => "careerPalace",
    Friends => "friendsPalace",
    Surface => "surfacePalace",
    Health => "healthPalace",
    Wealth => "wealthPalace",
    Children => "childrenPalace",
    Spouse => "spousePalace",
    Siblings => "siblingsPalace",
});

/// 五行局
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(FiveElementsClass, "five elements class" {
    Water2nd => "water2nd",
    Wood3rd => "wood3rd",
    Metal4th => "metal4th",
    Earth5th => "earth5th",
    Fire6th => "fire6th",
});

/// 四化
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

keyed_enum!(Mutagen, "mutagen" {
    Lu => "sihuaLu",
    Quan => "sihuaQuan",
    Ke => "sihuaKe",
    Ji => "sihuaJi",
});

/// 星曜亮度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(Brightness, "brightness" {
    Miao => "miao",
    Wang => "wang",
    De => "de",
    Li => "li",
    Ping => "ping",
    Bu => "bu",
    Xian => "xian",
});

/// 星曜类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(StarType, "star type" {
    Major => "major",
    Soft => "soft",
    Tough => "tough",
    Adjective => "adjective",
    Flower => "flower",
    Helper => "helper",
    Lucun => "lucun",
    Tianma => "tianma",
});

/// 运限范围
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(Scope, "scope" {
    Origin => "origin",
    Decadal => "decadal",
    Yearly => "yearly",
    Monthly => "monthly",
    Daily => "daily",
    Hourly => "hourly",
});

/// 有流耀与运限四化的五个层级，按大限、流年、流月、流日、流时排列（本命与小限不在此列）。
pub const HOROSCOPE_SCOPES: [Scope; 5] = [
    Scope::Decadal,
    Scope::Yearly,
    Scope::Monthly,
    Scope::Daily,
    Scope::Hourly,
];

/// 运限层级显示名
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(HoroscopeName, "horoscope name" {
    Decadal => "decadal",
    Childhood => "childhood",
    Age => "turn",
    Yearly => "yearly",
    Monthly => "monthly",
    Daily => "daily",
    Hourly => "hourly",
});

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// 男
    Male,
    /// 女
    Female,
}

keyed_enum!(Gender, "gender" {
    Male => "male",
    Female => "female",
});

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

keyed_enum!(Language, "language" {
    ZhCN => "zh-CN",
    ZhTW => "zh-TW",
    EnUS => "en-US",
    JaJP => "ja-JP",
    KoKR => "ko-KR",
    ViVN => "vi-VN",
});

impl Language {
    /// 由人写的语言代码还原：大小写不敏感，连字符与下划线等价（`zh-CN` / `zh_cn` 都可）；
    /// 未知代码返回 `None`。规范写法见 [`Language::KEYS`]，serde 与 [`Language::from_key`] 只认规范写法。
    pub fn from_code(code: &str) -> Option<Self> {
        let normalized = code.replace('_', "-");
        Self::KEYS
            .iter()
            .find(|key| key.eq_ignore_ascii_case(&normalized))
            .and_then(|key| Self::from_key(key))
    }
}

/// 算法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Algorithm {
    /// 默认
    Default,
    /// 中州派
    Zhongzhou,
}

keyed_enum!(Algorithm, "algorithm" {
    Default => "default",
    Zhongzhou => "zhongzhou",
});

/// 排盘视角：中州派把同一组出生数据看作三张盘，差别在于用哪一宫的干支起五行局。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AstroType {
    /// 天盘：以命宫干支起五行局，即常规排盘结果
    Heaven,
    /// 地盘：以身宫干支起五行局，身宫即为新盘的命宫
    Earth,
    /// 人盘：以福德宫干支起五行局，福德宫即为新盘的命宫
    Human,
}

keyed_enum!(AstroType, "astroType" {
    Heaven => "heaven",
    Earth => "earth",
    Human => "human",
});

/// 农历输入的闰月处理方式（`by_lunar` 专用）。
///
/// 把 iztro `byLunar` 的 `isLeapMonth` 与 `fixLeap` 两个布尔合成一个三态值：
/// 两个布尔相邻传参极易写反且不报错，而 `fixLeap` 只在输入是闰月时才有意义，
/// 三态恰好覆盖全部有效组合。阳历排盘的 `fix_leap` 仍是单个布尔（见 [`by_solar`]）。
///
/// [`by_solar`]: crate::by_solar
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LeapMonth {
    /// 输入的农历月不是闰月
    NotLeap,
    /// 输入的农历月是闰月，按闰月本身排盘
    Leap,
    /// 输入的农历月是闰月，且十五之后视作次月（iztro `fixLeap`）
    LeapFixed,
}

keyed_enum!(LeapMonth, "leap month" {
    NotLeap => "notLeap",
    Leap => "leap",
    LeapFixed => "leapFixed",
});

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum YearDivide {
    /// 正月初一分界
    Normal,
    /// 立春分界
    Exact,
}

keyed_enum!(YearDivide, "yearDivide" {
    Normal => "normal",
    Exact => "exact",
});

/// 运限分界点：运限干支与干支纪日的月柱按初一还是节气推算
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoroscopeDivide {
    /// 年按正月初一分界，月按初一以五虎遁推算
    Normal,
    /// 年按立春分界，月按节气分界
    Exact,
}

keyed_enum!(HoroscopeDivide, "horoscopeDivide" {
    Normal => "normal",
    Exact => "exact",
});

/// 虚岁分界点
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgeDivide {
    /// 以自然农历年为界，跨年即加一岁
    Normal,
    /// 以生日为界，过了生日才加一岁
    Birthday,
}

keyed_enum!(AgeDivide, "ageDivide" {
    Normal => "normal",
    Birthday => "birthday",
});

/// 晚子时（23:00-00:00）归属
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DayDivide {
    /// 晚子时归次日
    Forward,
    /// 晚子时归当天（按当日早子时排盘）
    Current,
}

keyed_enum!(DayDivide, "dayDivide" {
    Forward => "forward",
    Current => "current",
});

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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    /// 属于排盘输入：随 `Config` 的 serde（即绑定线协议）往返；星盘 DTO 的 `config` 只记六个开关。
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

/// `Config` 线协议的部分键补丁：缺省键取默认值，未知键报错（拼错的键不会被静默忽略）。
///
/// 六个开关取语言无关标识；`mutagens` 为天干标识 → 四化星标识（禄、权、科、忌四项），
/// `brightness` 为星耀标识 → 十二宫亮度标识（按宫位索引、寅宫为 0，空串表示该宫无亮度）。
#[derive(Default, serde::Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
struct ConfigPatch {
    year_divide: Option<YearDivide>,
    horoscope_divide: Option<HoroscopeDivide>,
    age_divide: Option<AgeDivide>,
    day_divide: Option<DayDivide>,
    algorithm: Option<Algorithm>,
    astro_type: Option<AstroType>,
    mutagens: Option<BTreeMap<String, Vec<String>>>,
    brightness: Option<BTreeMap<String, Vec<String>>>,
}

impl ConfigPatch {
    /// 在默认配置上打补丁；自定义表里任一标识非法或项数不对即报错，报错带出所在的键。
    fn into_config(self) -> Result<Config, String> {
        let defaults = Config::default();
        let mut config = Config {
            year_divide: self.year_divide.unwrap_or(defaults.year_divide),
            horoscope_divide: self.horoscope_divide.unwrap_or(defaults.horoscope_divide),
            age_divide: self.age_divide.unwrap_or(defaults.age_divide),
            day_divide: self.day_divide.unwrap_or(defaults.day_divide),
            algorithm: self.algorithm.unwrap_or(defaults.algorithm),
            astro_type: self.astro_type.unwrap_or(defaults.astro_type),
            overrides: None,
        };
        for (stem_key, star_keys) in self.mutagens.unwrap_or_default() {
            let stem = HeavenlyStem::from_key(&stem_key).ok_or_else(|| {
                format!("invalid mutagens key '{stem_key}': unknown heavenly stem")
            })?;
            let stars: [StarKey; 4] = star_keys
                .iter()
                .map(|key| {
                    StarKey::from_key(key).ok_or_else(|| {
                        format!("invalid mutagens for '{stem_key}': unknown star '{key}'")
                    })
                })
                .collect::<Result<Vec<_>, _>>()?
                .try_into()
                .map_err(|got: Vec<StarKey>| {
                    format!(
                        "invalid mutagens for '{stem_key}': expected 4 stars (lu, quan, ke, ji), got {}",
                        got.len()
                    )
                })?;
            config = config.with_mutagens(stem, stars);
        }
        for (star_key, brightness_keys) in self.brightness.unwrap_or_default() {
            let star = StarKey::from_key(&star_key)
                .ok_or_else(|| format!("invalid brightness key '{star_key}': unknown star"))?;
            if brightness_keys.len() != 12 {
                return Err(format!(
                    "invalid brightness for '{star_key}': expected 12 entries, got {}",
                    brightness_keys.len()
                ));
            }
            let mut table = [None; 12];
            for (slot, key) in table.iter_mut().zip(&brightness_keys) {
                if key.is_empty() {
                    continue;
                }
                *slot = Some(Brightness::from_key(key).ok_or_else(|| {
                    format!("invalid brightness for '{star_key}': unknown brightness '{key}'")
                })?);
            }
            config = config.with_brightness(star, table);
        }
        Ok(config)
    }
}

/// 反序列化即绑定层的 config JSON：部分键补丁，缺省键取默认值，未知键报错；
/// `mutagens` / `brightness` 的写法同序列化输出。
impl<'de> serde::Deserialize<'de> for Config {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ConfigPatch::deserialize(deserializer)?
            .into_config()
            .map_err(serde::de::Error::custom)
    }
}

/// 序列化写出六个开关与已覆盖的自定义表（键按标识排序），可被反序列化原样还原。
impl serde::Serialize for Config {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeMap;
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("yearDivide", &self.year_divide)?;
        map.serialize_entry("horoscopeDivide", &self.horoscope_divide)?;
        map.serialize_entry("ageDivide", &self.age_divide)?;
        map.serialize_entry("dayDivide", &self.day_divide)?;
        map.serialize_entry("algorithm", &self.algorithm)?;
        map.serialize_entry("astroType", &self.astro_type)?;
        if let Some(tables) = self.overrides.as_deref() {
            if !tables.mutagens.is_empty() {
                let mutagens: BTreeMap<&str, &[StarKey; 4]> = tables
                    .mutagens
                    .iter()
                    .map(|(stem, stars)| (stem.as_key(), stars))
                    .collect();
                map.serialize_entry("mutagens", &mutagens)?;
            }
            if !tables.brightness.is_empty() {
                let brightness: BTreeMap<&str, Vec<&str>> = tables
                    .brightness
                    .iter()
                    .map(|(star, table)| {
                        let keys = table
                            .iter()
                            .map(|b| b.map_or("", Brightness::as_key))
                            .collect();
                        (star.as_key(), keys)
                    })
                    .collect();
                map.serialize_entry("brightness", &brightness)?;
            }
        }
        map.end()
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

#[cfg(test)]
mod tests {
    use super::*;

    /// 语言代码大小写与连字符/下划线写法都能还原到规范写法。
    #[test]
    fn language_code_aliases() {
        for key in Language::KEYS {
            let lang = Language::from_key(key).unwrap();
            assert_eq!(Language::from_code(key), Some(lang));
            assert_eq!(Language::from_code(&key.to_lowercase()), Some(lang));
            assert_eq!(Language::from_code(&key.replace('-', "_")), Some(lang));
        }
        assert_eq!(Language::from_code("klingon"), None);
        assert_eq!(Language::from_key("zh_cn"), None);
    }

    /// 两个布尔与三态互转：非闰月时 `fix_leap` 不起作用。
    #[test]
    fn leap_month_flags() {
        for v in [LeapMonth::NotLeap, LeapMonth::Leap, LeapMonth::LeapFixed] {
            assert_eq!(LeapMonth::from_flags(v.is_leap_month(), v.fix_leap()), v);
        }
        assert_eq!(LeapMonth::from_flags(false, true), LeapMonth::NotLeap);
    }

    /// 线协议是部分键补丁：缺省键取默认值。
    #[test]
    fn config_patch_defaults_missing_keys() {
        let c: Config = serde_json::from_str(r#"{"algorithm":"zhongzhou"}"#).unwrap();
        assert_eq!(c.algorithm, Algorithm::Zhongzhou);
        assert_eq!(c.year_divide, Config::default().year_divide);
        assert!(c.overrides.is_none());
    }

    /// 拼错的键报错，不静默按默认配置排盘。
    #[test]
    fn config_rejects_unknown_key() {
        let err = serde_json::from_str::<Config>(r#"{"yearDivde":"exact"}"#).unwrap_err();
        assert!(err.to_string().contains("yearDivde"), "{err}");
    }

    /// 开关取值非法时，报错带字段名与全部合法取值。
    #[test]
    fn config_rejects_unknown_switch_value() {
        let err = serde_json::from_str::<Config>(r#"{"yearDivide":"lichun"}"#).unwrap_err();
        assert!(
            err.to_string()
                .starts_with("invalid yearDivide 'lichun': expected 'normal' or 'exact'"),
            "{err}"
        );
    }

    /// 自定义四化与亮度表随序列化往返，不丢失。
    #[test]
    fn config_roundtrip_keeps_overrides() {
        let mut brightness = [None; 12];
        brightness[0] = Some(Brightness::Miao);
        brightness[11] = Some(Brightness::Xian);
        let config = Config {
            year_divide: YearDivide::Exact,
            ..Config::default()
        }
        .with_mutagens(
            HeavenlyStem::Geng,
            [
                StarKey::TaiyangMaj,
                StarKey::WuquMaj,
                StarKey::TiantongMaj,
                StarKey::TaiyinMaj,
            ],
        )
        .with_brightness(StarKey::ZiweiMaj, brightness);

        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(json["yearDivide"], "exact");
        assert_eq!(
            json["mutagens"]["gengHeavenly"],
            serde_json::json!(["taiyangMaj", "wuquMaj", "tiantongMaj", "taiyinMaj"])
        );
        assert_eq!(json["brightness"]["ziweiMaj"][0], "miao");
        assert_eq!(json["brightness"]["ziweiMaj"][1], "");
        let back: Config = serde_json::from_value(json).unwrap();
        assert_eq!(back, config);
    }

    /// 自定义表的项数或标识不对时，报错指出是哪一个键。
    #[test]
    fn config_rejects_malformed_tables() {
        let err = serde_json::from_str::<Config>(r#"{"mutagens":{"jiaHeavenly":["ziweiMaj"]}}"#)
            .unwrap_err();
        assert!(err.to_string().contains("'jiaHeavenly'"), "{err}");
        let err = serde_json::from_str::<Config>(
            r#"{"brightness":{"ziweiMaj":["miao","","","","","","","","","","","dim"]}}"#,
        )
        .unwrap_err();
        assert!(
            err.to_string().contains("unknown brightness 'dim'"),
            "{err}"
        );
    }
}

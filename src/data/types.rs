use serde::{Deserialize, Serialize};

/// 阴阳
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YinYang {
    /// 阳
    Yang,
    /// 阴
    Yin,
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

/// 运限范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    /// 男
    Male,
    /// 女
    Female,
}

/// 语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    ZhCN,
    ZhTW,
    EnUS,
    JaJP,
    KoKR,
    ViVN,
}

/// 算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    /// 默认
    Default,
    /// 中州派
    Zhongzhou,
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

/// 排盘配置：控制分界点与算法派别的全部开关
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
        }
    }
}

/// 时辰索引
pub type TimeIndex = u8;

impl HeavenlyStem {
    pub fn index(&self) -> usize {
        *self as usize
    }
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::HEAVENLY_STEMS[i % 10]
    }
}

impl EarthlyBranch {
    pub fn index(&self) -> usize {
        *self as usize
    }
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::EARTHLY_BRANCHES[i % 12]
    }
}

impl Palace {
    pub fn index(&self) -> usize {
        *self as usize
    }
    pub fn from_index(i: usize) -> Self {
        crate::data::constants::PALACES[i % 12]
    }
}

impl FiveElementsClass {
    pub fn value(&self) -> usize {
        *self as usize
    }
}

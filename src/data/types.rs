use std::collections::{BTreeMap, HashMap};

use crate::data::keyed::keyed_enum;
use crate::data::stars::{MUTAGEN, StarKey};

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

impl Mutagen {
    /// 在禄、权、科、忌顺序中的位次（禄为 0）；四化表四槽即按此位次存放。
    pub fn index(self) -> usize {
        match self {
            Mutagen::Lu => 0,
            Mutagen::Quan => 1,
            Mutagen::Ke => 2,
            Mutagen::Ji => 3,
        }
    }
}

/// 星曜亮度
///
/// 前七级是 iztro 亮度表的取值；闲与地是 iztro 没有的两级，只出现在自定义亮度表里。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
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
    /// 闲：在王亭之《安星法及推断实例》的六级（庙旺地平闲陷）与《紫微斗数一本通》第七章的七级
    /// （庙旺地利平闲陷）里都排在平之下、陷之上。标识 `xian2` 是带声调数字的拼音（xián），
    /// 与陷（xiàn）的 `xian` 区分
    Xian2,
    /// 地：照录王亭之《安星法及推断实例》p101–102 的庙陷总表（贪狼卯、天梁酉、化禄巳三格），
    /// 该书 p107 的术语只释入庙、坐旺、落陷、平闲，没有给「地」下定义，所以不把它并入任何一级
    Di,
}

keyed_enum!(Brightness, "brightness" {
    Miao => "miao",
    Wang => "wang",
    De => "de",
    Li => "li",
    Ping => "ping",
    Bu => "bu",
    Xian => "xian",
    Xian2 => "xian2",
    Di => "di",
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

impl Scope {
    /// 层级深浅：本命 0、大限 1、流年 2、流月 3、流日 4、流时 5，越大越细；
    /// 运限层级的先后即 [`HOROSCOPE_SCOPES`] 的顺序。
    pub(crate) fn depth(self) -> u8 {
        match self {
            Scope::Origin => 0,
            Scope::Decadal => 1,
            Scope::Yearly => 2,
            Scope::Monthly => 3,
            Scope::Daily => 4,
            Scope::Hourly => 5,
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

/// 运限分界点：运限干支与干支纪日的月柱按初一还是节气推算。
///
/// 年的一半决定流年干支（及按它安的年系杂耀、岁前/将前十二神），月的一半决定流月干支与
/// 干支纪日的月柱；流月命宫始终按农历月排，不随月柱换。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum HoroscopeDivide {
    /// 年按正月初一分界，月按初一以五虎遁推算（闰月十六日起归下月）
    Normal,
    /// 年按立春分界，月按节气分界
    Exact,
    /// 年按正月初一分界，月按节气分界：流月四化在交节时换干，流年仍在正月初一换
    ExactMonth,
}

keyed_enum!(HoroscopeDivide, "horoscopeDivide" {
    Normal => "normal",
    Exact => "exact",
    ExactMonth => "exactMonth",
});

impl HoroscopeDivide {
    /// 流年干支是否按立春换年
    pub fn year_by_li_chun(self) -> bool {
        matches!(self, HoroscopeDivide::Exact)
    }

    /// 月柱（流月干支、干支纪日的月柱）是否按节气换月
    pub fn month_by_jie(self) -> bool {
        matches!(self, HoroscopeDivide::Exact | HoroscopeDivide::ExactMonth)
    }
}

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
#[non_exhaustive]
pub enum DayDivide {
    /// 与 iztro 一致：日柱、起紫微的日数与日系杂耀按次日算（月末的次日回卷到初一），
    /// 生月与年干支仍按当天
    Forward,
    /// 晚子时归当天（按当日早子时排盘）
    Current,
    /// 晚子时整个日期归次日（按次日早子时排盘）：月底的晚子进下月、除夕的晚子进下一年，
    /// 农历生日、生月、年干支与生肖都随之改变；运限目标的晚子时同样按次日早子时算
    ForwardCarry,
}

keyed_enum!(DayDivide, "dayDivide" {
    Forward => "forward",
    Current => "current",
    ForwardCarry => "forwardCarry",
});

/// 闰月出生时安星用哪个月。
///
/// 只管本命盘与斗君的生月；流月的闰月一律十五日分界，与此无关。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum LeapDivide {
    /// 按排盘入参 `fix_leap`：打开时闰月十五日及以前按本月、十六日起按下月，关闭时整月按本月
    #[default]
    Default,
    /// 闰月整月按下月，排盘入参 `fix_leap` 不起作用
    NextMonth,
}

keyed_enum!(LeapDivide, "leapDivide" {
    Default => "default",
    NextMonth => "nextMonth",
});

/// 天马的安法。
///
/// 年马按生年地支（寅午戌申、申子辰寅、巳酉丑亥、亥卯未巳）；月马按生月
/// （正五九申、二六十巳、三七十一寅、四八十二亥），生月与左辅右弼所用的月相同（已按闰月口径修正）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TianmaPlacement {
    /// 天马安年马
    #[default]
    Yearly,
    /// 天马安月马
    Monthly,
    /// 天马安年马，另在月马之位安杂耀月马 `yuema`
    Both,
}

keyed_enum!(TianmaPlacement, "tianmaPlacement" {
    Yearly => "yearly",
    Monthly => "monthly",
    Both => "both",
});

/// 长生十二神的排列方向（起长生的宫位都按五行局）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ChangshengDirection {
    /// 阳男阴女顺行、阴男阳女逆行
    #[default]
    YinYang,
    /// 男命顺行、女命逆行，不看生年阴阳
    Gender,
    /// 一律顺行
    Clockwise,
}

keyed_enum!(ChangshengDirection, "changshengDirection" {
    YinYang => "yinYang",
    Gender => "gender",
    Clockwise => "clockwise",
});

/// 天伤、天使的安法（两星分居迁移宫前后的仆役位与疾厄位）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TianshangTianshi {
    /// 与 iztro 一致：天伤在仆役、天使在疾厄；中州派阴男阳女两者对调
    #[default]
    Default,
    /// 天使在仆役、天伤在疾厄，不分男女与派别
    TianshiFriends,
}

keyed_enum!(TianshangTianshi, "tianshangTianshi" {
    Default => "default",
    TianshiFriends => "tianshiFriends",
});

/// 天巫的安法：按生月（已按闰月口径修正）每四个月一轮。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum TianwuPlacement {
    /// 正五九巳、二六十申、三七十一寅、四八十二亥
    #[default]
    SiShenYinHai,
    /// 正五九巳、二六十申、三七十一亥、四八十二寅
    SiShenHaiYin,
}

keyed_enum!(TianwuPlacement, "tianwuPlacement" {
    SiShenYinHai => "siShenYinHai",
    SiShenHaiYin => "siShenHaiYin",
});

/// 岁前、将前十二神安在哪些盘上。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum Yearly12Placement {
    /// 本命盘按生年地支安，流年按流年地支另安
    #[default]
    NatalAndYearly,
    /// 只在流年安，本命盘各宫不安
    YearlyOnly,
}

keyed_enum!(Yearly12Placement, "yearly12Placement" {
    NatalAndYearly => "natalAndYearly",
    YearlyOnly => "yearlyOnly",
});

/// 来因宫的标法。
///
/// 来因宫是宫干等于生年天干的宫。五虎遁排到子丑两宫与寅卯两宫同干，辛年（卯、丑）、
/// 壬年（寅、子）因此各有两宫合格；其余八干只有一宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum OriginalPalaceMarking {
    /// 只标寅至亥十宫里合格的那一宫，子丑不标
    #[default]
    Primary,
    /// 另把辛年丑宫、壬年子宫标为辅来因宫
    WithSecondary,
}

keyed_enum!(OriginalPalaceMarking, "originalPalace" {
    Primary => "primary",
    WithSecondary => "withSecondary",
});

/// 推斗君（流月起点）时生月的取法。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum DouJunMonth {
    /// 与 iztro 一致：按闰月口径取生月，`fix_leap` 打开时闰月十六日起作下月，晚子时也算
    #[default]
    Default,
    /// 与本命盘同一规则取生月：`fix_leap` 下晚子时（`day_divide=forward`）不把闰月十六日起作下月
    Natal,
}

keyed_enum!(DouJunMonth, "douJunMonth" {
    Default => "default",
    Natal => "natal",
});

/// 大限天干的取法（大限地支恒为所在宫的地支）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum DecadalStem {
    /// 与 iztro 一致：取所在宫的本命宫干
    #[default]
    Palace,
    /// 以命宫宫干为第一大限的干，按天干次序随大限方向连续排：顺行的大限逐限加一干，逆行的逐限减一干
    Continuous,
}

keyed_enum!(DecadalStem, "decadalStem" {
    Palace => "palace",
    Continuous => "continuous",
});

/// 童限（起运之前）落在哪一宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ChildhoodPalace {
    /// 与 iztro 一致：虚岁一岁起依次落命宫、财帛、疾厄、夫妻、福德、官禄
    #[default]
    Sequence,
    /// 一律落夫妻宫：童限以夫妻宫为命宫推排宫名，四化取夫妻宫干
    Spouse,
}

keyed_enum!(ChildhoodPalace, "childhoodPalace" {
    Sequence => "sequence",
    Spouse => "spouse",
});

/// 小限是否起四化。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum AgeMutagens {
    /// 与 iztro 一致：取小限宫的宫干起四化
    #[default]
    PalaceStem,
    /// 小限不起四化：小限层的四化星为空
    None,
}

keyed_enum!(AgeMutagens, "ageMutagens" {
    PalaceStem => "palaceStem",
    None => "none",
});

/// 流年命宫取哪一宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum YearlyPalace {
    /// 与 iztro 一致：太岁宫，即流年年支所在宫
    #[default]
    Taisui,
    /// 当年的小限宫；流年干支、流耀、岁前/将前十二神与斗君仍按流年年支与太岁宫
    Age,
}

keyed_enum!(YearlyPalace, "yearlyPalace" {
    Taisui => "taisui",
    Age => "age",
});

/// 运限各层（大限、童限、流年、流月、流日、流时）四化取哪个干；小限恒取小限宫干。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ScopeMutagenStem {
    /// 与 iztro 一致：该层自身的干，即大限干、童限宫干、流年年干与流月、流日、流时的干
    #[default]
    Scope,
    /// 该层命宫的本命宫干
    ScopeSoulPalace,
}

keyed_enum!(ScopeMutagenStem, "scopeMutagenStem" {
    Scope => "scope",
    ScopeSoulPalace => "scopeSoulPalace",
});

/// 流月命宫的起法；两种都是每月顺行一宫，闰月前半月随本月、后半月随下月。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum MonthlyPalace {
    /// 与 iztro 一致：斗君为正月，即太岁宫逆数生月、顺数生时所到之宫
    #[default]
    DouJun,
    /// 正月固定在寅宫，即每月落在该月月建（正月寅、二月卯……）所在宫
    MonthBranch,
}

keyed_enum!(MonthlyPalace, "monthlyPalace" {
    DouJun => "douJun",
    MonthBranch => "monthBranch",
});

/// 流日命宫的起法：初一落在哪一宫，之后每日顺行一宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum DailyPalace {
    /// 与 iztro 一致：初一在流月命宫
    #[default]
    FromMonthly,
    /// 初一在流月命宫的下一宫（顺行方向）
    AfterMonthly,
}

keyed_enum!(DailyPalace, "dailyPalace" {
    FromMonthly => "fromMonthly",
    AfterMonthly => "afterMonthly",
});

/// 闰月里流日的数法。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum LeapDaily {
    /// 与 iztro 一致：闰月前半月从流月命宫重新起初一，十六日起按下月的流月命宫加（日数 − 1）
    #[default]
    Default,
    /// 闰月前半月接着同号本月的日数连续顺数（本月三十日则闰月初一落在第三十一位）；十六日起同默认
    Continuous,
}

keyed_enum!(LeapDaily, "leapDaily" {
    Default => "default",
    Continuous => "continuous",
});

/// 流时命宫的起法。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum HourlyPalace {
    /// 与 iztro 一致：子时在流日命宫，每时辰顺行一宫
    #[default]
    FromDaily,
    /// 按时支落宫：子时在子宫、丑时在丑宫，与流日命宫无关
    Branch,
}

keyed_enum!(HourlyPalace, "hourlyPalace" {
    FromDaily => "fromDaily",
    Branch => "branch",
});

/// 截空与旬空的安法。
///
/// 截空按生年天干占相邻两宫（甲己申酉、乙庚午未、丙辛辰巳、丁壬寅卯、戊癸子丑），
/// 旬空占生年所在一旬空出的两宫；两者都是阳年以阳宫为正空、阴年以阴宫为正空，另一宫为傍空。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum VoidPlacement {
    /// 与 iztro 一致：默认派把截空的两宫分别安作截路（阳宫）与空亡（阴宫），中州派只在正空安截空；
    /// 旬空只在正空安一颗
    #[default]
    Default,
    /// 截空、旬空各安正空、傍空两宫：正空安截空、旬空，傍空安傍截空、傍旬空，不分派别；
    /// 默认派不再安截路、空亡
    ZhengPang,
}

keyed_enum!(VoidPlacement, "voidPlacement" {
    Default => "default",
    ZhengPang => "zhengPang",
});

/// 一个天干化出的四化星。
///
/// 禄、权、科、忌四槽各一颗星，即 iztro 的四化表；有的流派一干化出多于四颗星
/// （如庚干天同、天相同化忌），四槽之外同化一类的星记作附加星。
/// 同一天干里一颗星至多化一类（[`Config::check`] 校验）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StemMutagens {
    /// 四槽，顺序为禄、权、科、忌
    pub primary: [StarKey; 4],
    /// 四槽之外的附加星，按禄、权、科、忌分组（下标即 [`Mutagen::index`]）；默认四化表没有附加星
    pub extra: [Vec<StarKey>; 4],
}

impl StemMutagens {
    /// 全部（星, 四化）：按禄、权、科、忌，每类先四槽之星、后附加星。
    pub fn iter(&self) -> impl Iterator<Item = (StarKey, Mutagen)> + '_ {
        mutagen_pairs(&self.primary, &self.extra)
    }

    /// 化出指定四化的星：四槽之星在前，附加星在后。
    pub fn stars_of(&self, mutagen: Mutagen) -> impl Iterator<Item = StarKey> + '_ {
        std::iter::once(self.primary[mutagen.index()])
            .chain(self.extra[mutagen.index()].iter().copied())
    }

    /// 该星在本干化哪一类；不在本干四化里返回 `None`。
    pub fn mutagen_of(&self, star: StarKey) -> Option<Mutagen> {
        self.iter().find(|(s, _)| *s == star).map(|(_, m)| m)
    }
}

/// 四槽与附加星合成的（星, 四化）序列，顺序同 [`StemMutagens::iter`]。
///
/// `primary` 按禄、权、科、忌排列（运限层的四化星即这四项）。
pub(crate) fn mutagen_pairs<'a>(
    primary: &'a [StarKey],
    extra: &'a [Vec<StarKey>; 4],
) -> impl Iterator<Item = (StarKey, Mutagen)> + 'a {
    MUTAGEN.iter().enumerate().flat_map(move |(i, &m)| {
        primary
            .get(i)
            .into_iter()
            .chain(extra[i].iter())
            .map(move |&s| (s, m))
    })
}

/// 没有附加星时的四组空表。
static NO_EXTRA_MUTAGENS: [Vec<StarKey>; 4] = [Vec::new(), Vec::new(), Vec::new(), Vec::new()];

/// 自定义四化与亮度表。
///
/// 紫微斗数流派众多，四化与星耀亮度是分歧最集中的两处。四化四槽与亮度按 key **整表替换**
/// 默认值：给出某个天干的四化就只改那个天干，未给出的天干仍用默认表；亮度同理。
/// 附加四化星加在该天干生效的四槽（覆盖表或默认表）之外。
///
/// 通过 [`Config::with_mutagens`] / [`Config::with_extra_mutagens`] / [`Config::with_brightness`] 构造。
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TableOverrides {
    /// 天干 → 该干化出的四颗星，顺序为禄、权、科、忌
    mutagens: HashMap<HeavenlyStem, [StarKey; 4]>,
    /// 天干 → 四槽之外的附加四化星，按禄、权、科、忌分组
    extra_mutagens: HashMap<HeavenlyStem, [Vec<StarKey>; 4]>,
    /// 星耀 → 它在十二宫（寅宫为 0）各自的亮度，无亮度的位置为 None
    brightness: HashMap<StarKey, [Option<Brightness>; 12]>,
}

impl TableOverrides {
    /// 覆盖某个天干的四化表。
    pub fn set_mutagens(&mut self, stem: HeavenlyStem, stars: [StarKey; 4]) {
        self.mutagens.insert(stem, stars);
    }

    /// 设定某个天干在某一类四化上的附加星，替换该类原有的附加星；空列表即去掉该类附加星。
    pub fn set_extra_mutagens(
        &mut self,
        stem: HeavenlyStem,
        mutagen: Mutagen,
        stars: Vec<StarKey>,
    ) {
        let groups = self.extra_mutagens.entry(stem).or_default();
        groups[mutagen.index()] = stars;
        if groups.iter().all(Vec::is_empty) {
            self.extra_mutagens.remove(&stem);
        }
    }

    /// 覆盖某颗星的十二宫亮度表。
    pub fn set_brightness(&mut self, star: StarKey, table: [Option<Brightness>; 12]) {
        self.brightness.insert(star, table);
    }

    /// 取该天干被覆盖的四化表；未覆盖时返回 `None`。
    pub fn mutagens_of(&self, stem: HeavenlyStem) -> Option<&[StarKey; 4]> {
        self.mutagens.get(&stem)
    }

    /// 取该天干的附加四化星（按禄、权、科、忌分组）；没有附加星时返回 `None`。
    pub fn extra_mutagens_of(&self, stem: HeavenlyStem) -> Option<&[Vec<StarKey>; 4]> {
        self.extra_mutagens.get(&stem)
    }

    /// 取该星被覆盖的亮度表；未覆盖时返回 `None`。
    pub fn brightness_of(&self, star: StarKey) -> Option<&[Option<Brightness>; 12]> {
        self.brightness.get(&star)
    }

    /// 是否没有任何覆盖。
    pub fn is_empty(&self) -> bool {
        self.mutagens.is_empty() && self.extra_mutagens.is_empty() && self.brightness.is_empty()
    }

    /// 校验四化表：改动过的每个天干里，四槽与附加星合起来一颗星至多出现一次。
    ///
    /// 星耀的四化标记只有一个取值，同一天干让一颗星化两类（或同类重复）时无从标记。
    /// 报错正文带出天干与星的标识。
    pub fn check(&self) -> Result<(), String> {
        let mut stems: Vec<HeavenlyStem> = self
            .mutagens
            .keys()
            .chain(self.extra_mutagens.keys())
            .copied()
            .collect();
        stems.sort_by_key(|s| s.index());
        stems.dedup();
        for stem in stems {
            let table = stem_mutagens(Some(self), stem);
            let mut seen: Vec<StarKey> = Vec::new();
            for (star, _) in table.iter() {
                if seen.contains(&star) {
                    return Err(format!(
                        "invalid mutagens for '{}': star '{}' appears more than once",
                        stem.as_key(),
                        star.as_key()
                    ));
                }
                seen.push(star);
            }
        }
        Ok(())
    }
}

/// 该天干生效的四化：四槽取覆盖表或默认表，附加星取覆盖表。
pub(crate) fn stem_mutagens(
    overrides: Option<&TableOverrides>,
    stem: HeavenlyStem,
) -> StemMutagens {
    StemMutagens {
        primary: primary_mutagens(overrides, stem),
        extra: overrides
            .and_then(|t| t.extra_mutagens_of(stem))
            .unwrap_or(&NO_EXTRA_MUTAGENS)
            .clone(),
    }
}

/// 该天干生效的四槽：有覆盖用覆盖，否则用默认表。
fn primary_mutagens(overrides: Option<&TableOverrides>, stem: HeavenlyStem) -> [StarKey; 4] {
    overrides
        .and_then(|t| t.mutagens_of(stem))
        .copied()
        .unwrap_or_else(|| crate::data::heavenly_stems::get_heavenly_stem_info(stem).mutagen)
}

/// 排盘配置：控制分界点、算法派别、各项流派口径与自定义表的全部开关
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
    /// 截空与旬空的安法
    pub void_placement: VoidPlacement,
    /// 闰月出生时安星用哪个月
    pub leap_divide: LeapDivide,
    /// 天马的安法
    pub tianma_placement: TianmaPlacement,
    /// 长生十二神的排列方向
    pub changsheng_direction: ChangshengDirection,
    /// 天伤、天使的安法
    pub tianshang_tianshi: TianshangTianshi,
    /// 天巫的安法
    pub tianwu_placement: TianwuPlacement,
    /// 岁前、将前十二神安在哪些盘上
    pub yearly12_placement: Yearly12Placement,
    /// 来因宫的标法
    pub original_palace: OriginalPalaceMarking,
    /// 推斗君时生月的取法
    pub dou_jun_month: DouJunMonth,
    /// 大限天干的取法
    pub decadal_stem: DecadalStem,
    /// 童限落在哪一宫
    pub childhood_palace: ChildhoodPalace,
    /// 小限是否起四化
    pub age_mutagens: AgeMutagens,
    /// 流年命宫取哪一宫
    pub yearly_palace: YearlyPalace,
    /// 运限各层四化取哪个干
    pub scope_mutagen_stem: ScopeMutagenStem,
    /// 流月命宫的起法
    pub monthly_palace: MonthlyPalace,
    /// 流日命宫的起法
    pub daily_palace: DailyPalace,
    /// 闰月里流日的数法
    pub leap_daily: LeapDaily,
    /// 流时命宫的起法
    pub hourly_palace: HourlyPalace,
    /// 自定义四化与亮度表；`None` 表示全部使用默认表。
    ///
    /// 属于排盘输入：随 `Config` 的 serde（即绑定线协议）往返；星盘 DTO 的 `config` 只记开关。
    /// 排盘入口先经 [`Config::check`] 校验。
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
            void_placement: VoidPlacement::Default,
            leap_divide: LeapDivide::Default,
            tianma_placement: TianmaPlacement::Yearly,
            changsheng_direction: ChangshengDirection::YinYang,
            tianshang_tianshi: TianshangTianshi::Default,
            tianwu_placement: TianwuPlacement::SiShenYinHai,
            yearly12_placement: Yearly12Placement::NatalAndYearly,
            original_palace: OriginalPalaceMarking::Primary,
            dou_jun_month: DouJunMonth::Default,
            decadal_stem: DecadalStem::Palace,
            childhood_palace: ChildhoodPalace::Sequence,
            age_mutagens: AgeMutagens::PalaceStem,
            yearly_palace: YearlyPalace::Taisui,
            scope_mutagen_stem: ScopeMutagenStem::Scope,
            monthly_palace: MonthlyPalace::DouJun,
            daily_palace: DailyPalace::FromMonthly,
            leap_daily: LeapDaily::Default,
            hourly_palace: HourlyPalace::FromDaily,
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

    /// 在当前配置上指定截空与旬空的安法。
    pub fn with_void_placement(mut self, void_placement: VoidPlacement) -> Self {
        self.void_placement = void_placement;
        self
    }

    /// 取出自定义表改写后放回；共享中的表先复制一份。
    fn edit_tables(mut self, edit: impl FnOnce(&mut TableOverrides)) -> Self {
        let mut tables = self
            .overrides
            .take()
            .map_or_else(TableOverrides::default, |arc| {
                std::sync::Arc::try_unwrap(arc).unwrap_or_else(|shared| (*shared).clone())
            });
        edit(&mut tables);
        self.overrides = Some(std::sync::Arc::new(tables));
        self
    }

    /// 在当前配置上覆盖某个天干的四化表。
    pub fn with_mutagens(self, stem: HeavenlyStem, stars: [StarKey; 4]) -> Self {
        self.edit_tables(|t| t.set_mutagens(stem, stars))
    }

    /// 在当前配置上设定某个天干在某一类四化上的附加星（替换该类原有的附加星）。
    ///
    /// 附加星与四槽之星同化一类，例如庚干的化忌除四槽的天同外再加天相。
    pub fn with_extra_mutagens(
        self,
        stem: HeavenlyStem,
        mutagen: Mutagen,
        stars: impl IntoIterator<Item = StarKey>,
    ) -> Self {
        let stars: Vec<StarKey> = stars.into_iter().collect();
        self.edit_tables(|t| t.set_extra_mutagens(stem, mutagen, stars))
    }

    /// 在当前配置上覆盖某颗星的十二宫亮度表。
    pub fn with_brightness(self, star: StarKey, table: [Option<Brightness>; 12]) -> Self {
        self.edit_tables(|t| t.set_brightness(star, table))
    }

    /// 校验自定义表，规则见 [`TableOverrides::check`]；没有自定义表时恒通过。
    ///
    /// # Errors
    /// 某个天干里一颗星出现不止一次时返回报错正文。
    pub fn check(&self) -> Result<(), String> {
        self.overrides
            .as_deref()
            .map_or(Ok(()), TableOverrides::check)
    }

    /// 该天干实际生效的四槽：有覆盖用覆盖，否则用默认表。附加星见 [`Config::stem_mutagens`]。
    pub fn mutagens_of(&self, stem: HeavenlyStem) -> [StarKey; 4] {
        primary_mutagens(self.overrides.as_deref(), stem)
    }

    /// 该天干实际生效的附加四化星，按禄、权、科、忌分组；没有附加星时四组皆空。
    pub fn extra_mutagens_of(&self, stem: HeavenlyStem) -> &[Vec<StarKey>; 4] {
        self.overrides
            .as_deref()
            .and_then(|t| t.extra_mutagens_of(stem))
            .unwrap_or(&NO_EXTRA_MUTAGENS)
    }

    /// 该天干实际生效的全部四化星：四槽加附加星。
    pub fn stem_mutagens(&self, stem: HeavenlyStem) -> StemMutagens {
        stem_mutagens(self.overrides.as_deref(), stem)
    }

    /// 指定天干下某颗星化哪一类（含附加星）；不在该干四化里返回 `None`。
    pub fn mutagen_of(&self, stem: HeavenlyStem, star: StarKey) -> Option<Mutagen> {
        mutagen_pairs(&self.mutagens_of(stem), self.extra_mutagens_of(stem))
            .find(|(s, _)| *s == star)
            .map(|(_, m)| m)
    }

    /// 该星在指定宫位实际生效的亮度：有覆盖用覆盖，否则用默认表。
    ///
    /// `palace_index` 为盘上位置（寅宫为 0），越界会对 12 取模。
    pub fn brightness_of(&self, star: StarKey, palace_index: usize) -> Option<Brightness> {
        let index = palace_index % 12;
        if let Some(table) = self.overrides.as_ref().and_then(|t| t.brightness_of(star)) {
            return table[index];
        }
        crate::data::stars::get_brightness_table(star)?[index]
    }
}

/// `Config` 线协议的部分键补丁：缺省键取默认值，未知键报错（拼错的键不会被静默忽略）。
///
/// 开关取语言无关标识；`mutagens` 为天干标识 → 四化星标识（禄、权、科、忌四项），
/// `extraMutagens` 为天干标识 → 四化标识 → 附加星标识列表，
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
    void_placement: Option<VoidPlacement>,
    leap_divide: Option<LeapDivide>,
    tianma_placement: Option<TianmaPlacement>,
    changsheng_direction: Option<ChangshengDirection>,
    tianshang_tianshi: Option<TianshangTianshi>,
    tianwu_placement: Option<TianwuPlacement>,
    yearly12_placement: Option<Yearly12Placement>,
    original_palace: Option<OriginalPalaceMarking>,
    dou_jun_month: Option<DouJunMonth>,
    decadal_stem: Option<DecadalStem>,
    childhood_palace: Option<ChildhoodPalace>,
    age_mutagens: Option<AgeMutagens>,
    yearly_palace: Option<YearlyPalace>,
    scope_mutagen_stem: Option<ScopeMutagenStem>,
    monthly_palace: Option<MonthlyPalace>,
    daily_palace: Option<DailyPalace>,
    leap_daily: Option<LeapDaily>,
    hourly_palace: Option<HourlyPalace>,
    mutagens: Option<BTreeMap<String, Vec<String>>>,
    extra_mutagens: Option<BTreeMap<String, BTreeMap<String, Vec<String>>>>,
    brightness: Option<BTreeMap<String, Vec<String>>>,
}

impl ConfigPatch {
    /// 在默认配置上打补丁；自定义表里任一标识非法、项数不对或同一天干里星有重复即报错，报错带出所在的键。
    fn into_config(self) -> Result<Config, String> {
        let defaults = Config::default();
        let mut config = Config {
            year_divide: self.year_divide.unwrap_or(defaults.year_divide),
            horoscope_divide: self.horoscope_divide.unwrap_or(defaults.horoscope_divide),
            age_divide: self.age_divide.unwrap_or(defaults.age_divide),
            day_divide: self.day_divide.unwrap_or(defaults.day_divide),
            algorithm: self.algorithm.unwrap_or(defaults.algorithm),
            astro_type: self.astro_type.unwrap_or(defaults.astro_type),
            void_placement: self.void_placement.unwrap_or(defaults.void_placement),
            leap_divide: self.leap_divide.unwrap_or(defaults.leap_divide),
            tianma_placement: self.tianma_placement.unwrap_or(defaults.tianma_placement),
            changsheng_direction: self
                .changsheng_direction
                .unwrap_or(defaults.changsheng_direction),
            tianshang_tianshi: self.tianshang_tianshi.unwrap_or(defaults.tianshang_tianshi),
            tianwu_placement: self.tianwu_placement.unwrap_or(defaults.tianwu_placement),
            yearly12_placement: self
                .yearly12_placement
                .unwrap_or(defaults.yearly12_placement),
            original_palace: self.original_palace.unwrap_or(defaults.original_palace),
            dou_jun_month: self.dou_jun_month.unwrap_or(defaults.dou_jun_month),
            decadal_stem: self.decadal_stem.unwrap_or(defaults.decadal_stem),
            childhood_palace: self.childhood_palace.unwrap_or(defaults.childhood_palace),
            age_mutagens: self.age_mutagens.unwrap_or(defaults.age_mutagens),
            yearly_palace: self.yearly_palace.unwrap_or(defaults.yearly_palace),
            scope_mutagen_stem: self
                .scope_mutagen_stem
                .unwrap_or(defaults.scope_mutagen_stem),
            monthly_palace: self.monthly_palace.unwrap_or(defaults.monthly_palace),
            daily_palace: self.daily_palace.unwrap_or(defaults.daily_palace),
            leap_daily: self.leap_daily.unwrap_or(defaults.leap_daily),
            hourly_palace: self.hourly_palace.unwrap_or(defaults.hourly_palace),
            overrides: None,
        };
        let parse_stem = |table: &str, stem_key: &str| {
            HeavenlyStem::from_key(stem_key)
                .ok_or_else(|| format!("invalid {table} key '{stem_key}': unknown heavenly stem"))
        };
        let parse_stars = |table: &str, stem_key: &str, star_keys: &[String]| {
            star_keys
                .iter()
                .map(|key| {
                    StarKey::from_key(key).ok_or_else(|| {
                        format!("invalid {table} for '{stem_key}': unknown star '{key}'")
                    })
                })
                .collect::<Result<Vec<_>, _>>()
        };
        for (stem_key, star_keys) in self.mutagens.unwrap_or_default() {
            let stem = parse_stem("mutagens", &stem_key)?;
            let stars: [StarKey; 4] = parse_stars("mutagens", &stem_key, &star_keys)?
                .try_into()
                .map_err(|got: Vec<StarKey>| {
                    format!(
                        "invalid mutagens for '{stem_key}': expected 4 stars (lu, quan, ke, ji), got {}",
                        got.len()
                    )
                })?;
            config = config.with_mutagens(stem, stars);
        }
        for (stem_key, groups) in self.extra_mutagens.unwrap_or_default() {
            let stem = parse_stem("extraMutagens", &stem_key)?;
            for (mutagen_key, star_keys) in groups {
                let mutagen = Mutagen::from_key(&mutagen_key).ok_or_else(|| {
                    format!(
                        "invalid extraMutagens for '{stem_key}': {}",
                        crate::data::keyed::unknown_key_message(
                            "mutagen",
                            &mutagen_key,
                            Mutagen::KEYS
                        )
                    )
                })?;
                let stars = parse_stars("extraMutagens", &stem_key, &star_keys)?;
                config = config.with_extra_mutagens(stem, mutagen, stars);
            }
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
        config.check()?;
        Ok(config)
    }
}

/// 反序列化即绑定层的 config JSON：部分键补丁，缺省键取默认值，未知键报错；
/// `mutagens` / `extraMutagens` / `brightness` 的写法同序列化输出。
impl<'de> serde::Deserialize<'de> for Config {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        ConfigPatch::deserialize(deserializer)?
            .into_config()
            .map_err(serde::de::Error::custom)
    }
}

/// 序列化写出全部开关与已覆盖的自定义表（键按标识排序），可被反序列化原样还原。
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
        map.serialize_entry("voidPlacement", &self.void_placement)?;
        map.serialize_entry("leapDivide", &self.leap_divide)?;
        map.serialize_entry("tianmaPlacement", &self.tianma_placement)?;
        map.serialize_entry("changshengDirection", &self.changsheng_direction)?;
        map.serialize_entry("tianshangTianshi", &self.tianshang_tianshi)?;
        map.serialize_entry("tianwuPlacement", &self.tianwu_placement)?;
        map.serialize_entry("yearly12Placement", &self.yearly12_placement)?;
        map.serialize_entry("originalPalace", &self.original_palace)?;
        map.serialize_entry("douJunMonth", &self.dou_jun_month)?;
        map.serialize_entry("decadalStem", &self.decadal_stem)?;
        map.serialize_entry("childhoodPalace", &self.childhood_palace)?;
        map.serialize_entry("ageMutagens", &self.age_mutagens)?;
        map.serialize_entry("yearlyPalace", &self.yearly_palace)?;
        map.serialize_entry("scopeMutagenStem", &self.scope_mutagen_stem)?;
        map.serialize_entry("monthlyPalace", &self.monthly_palace)?;
        map.serialize_entry("dailyPalace", &self.daily_palace)?;
        map.serialize_entry("leapDaily", &self.leap_daily)?;
        map.serialize_entry("hourlyPalace", &self.hourly_palace)?;
        if let Some(tables) = self.overrides.as_deref() {
            if !tables.mutagens.is_empty() {
                let mutagens: BTreeMap<&str, &[StarKey; 4]> = tables
                    .mutagens
                    .iter()
                    .map(|(stem, stars)| (stem.as_key(), stars))
                    .collect();
                map.serialize_entry("mutagens", &mutagens)?;
            }
            if !tables.extra_mutagens.is_empty() {
                let extra: BTreeMap<&str, BTreeMap<&str, &Vec<StarKey>>> = tables
                    .extra_mutagens
                    .iter()
                    .map(|(stem, groups)| {
                        let groups = MUTAGEN
                            .iter()
                            .zip(groups)
                            .filter(|(_, stars)| !stars.is_empty())
                            .map(|(m, stars)| (m.as_key(), stars))
                            .collect();
                        (stem.as_key(), groups)
                    })
                    .collect();
                map.serialize_entry("extraMutagens", &extra)?;
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

    /// 空曜安法、附加四化星与闲级亮度随序列化往返；附加星按四化标识分组写出，空组不写。
    #[test]
    fn config_roundtrip_keeps_void_placement_and_extra_mutagens() {
        let mut brightness = [None; 12];
        brightness[3] = Some(Brightness::Xian2);
        let config = Config::default()
            .with_void_placement(VoidPlacement::ZhengPang)
            .with_extra_mutagens(HeavenlyStem::Geng, Mutagen::Ji, [StarKey::TianxiangMaj])
            .with_extra_mutagens(HeavenlyStem::Geng, Mutagen::Lu, [])
            .with_brightness(StarKey::Hongluan, brightness);
        let json = serde_json::to_value(&config).unwrap();
        assert_eq!(json["voidPlacement"], "zhengPang");
        assert_eq!(
            json["extraMutagens"],
            serde_json::json!({"gengHeavenly": {"sihuaJi": ["tianxiangMaj"]}})
        );
        assert_eq!(json["brightness"]["hongluan"][3], "xian2");
        let back: Config = serde_json::from_value(json).unwrap();
        assert_eq!(back, config);
        assert_eq!(
            back.stem_mutagens(HeavenlyStem::Geng)
                .stars_of(Mutagen::Ji)
                .collect::<Vec<_>>(),
            [StarKey::TiantongMaj, StarKey::TianxiangMaj]
        );
    }

    /// 附加四化星的天干、四化标识非法，或同一天干里星有重复时，报错指出所在的键。
    #[test]
    fn config_rejects_malformed_extra_mutagens() {
        let err = serde_json::from_str::<Config>(
            r#"{"extraMutagens":{"gengHeavenly":{"sihuaXx":["tianxiangMaj"]}}}"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("sihuaXx"), "{err}");
        let err = serde_json::from_str::<Config>(
            r#"{"extraMutagens":{"geng":{"sihuaJi":["tianxiangMaj"]}}}"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("'geng'"), "{err}");
        // 天同已在庚干四槽化忌
        let err = serde_json::from_str::<Config>(
            r#"{"extraMutagens":{"gengHeavenly":{"sihuaKe":["tiantongMaj"]}}}"#,
        )
        .unwrap_err();
        assert!(
            err.to_string()
                .contains("'gengHeavenly': star 'tiantongMaj' appears more than once"),
            "{err}"
        );
        // 附加星互相重复同样拒绝
        let err = serde_json::from_str::<Config>(
            r#"{"extraMutagens":{"gengHeavenly":{"sihuaJi":["tianxiangMaj","tianxiangMaj"]}}}"#,
        )
        .unwrap_err();
        assert!(err.to_string().contains("tianxiangMaj"), "{err}");
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

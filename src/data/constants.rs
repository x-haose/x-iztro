use crate::data::types::*;

/// 支持的语言代码，顺序与 iztro `data.LANGUAGES` 一致
pub const LANGUAGES: [&str; 6] = ["en-US", "ja-JP", "ko-KR", "zh-CN", "zh-TW", "vi-VN"];

/// 星座标识，按黄道顺序（白羊起）
pub const SIGNS: [&str; 12] = [
    "aries",
    "taurus",
    "gemini",
    "cancer",
    "leo",
    "virgo",
    "libra",
    "scorpio",
    "sagittarius",
    "capricorn",
    "aquarius",
    "pisces",
];

/// 生肖标识，按地支顺序（子=鼠 … 亥=猪）
pub const ZODIAC: [&str; 12] = [
    "rat", "ox", "tiger", "rabbit", "dragon", "snake", "horse", "sheep", "monkey", "rooster",
    "dog", "pig",
];

/// 时辰标识，索引 0-12（早子时起，晚子时止）
pub const CHINESE_TIME: [&str; 13] = [
    "earlyRatHour",
    "oxHour",
    "tigerHour",
    "rabbitHour",
    "dragonHour",
    "snakeHour",
    "horseHour",
    "goatHour",
    "monkeyHour",
    "roosterHour",
    "dogHour",
    "pigHour",
    "lateRatHour",
];

/// 天干顺序：甲乙丙丁戊己庚辛壬癸
pub const HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,
    HeavenlyStem::Yi,
    HeavenlyStem::Bing,
    HeavenlyStem::Ding,
    HeavenlyStem::Wu,
    HeavenlyStem::Ji,
    HeavenlyStem::Geng,
    HeavenlyStem::Xin,
    HeavenlyStem::Ren,
    HeavenlyStem::Gui,
];

/// 地支顺序：子丑寅卯辰巳午未申酉戌亥
pub const EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi,
    EarthlyBranch::Chou,
    EarthlyBranch::Yin,
    EarthlyBranch::Mao,
    EarthlyBranch::Chen,
    EarthlyBranch::Si,
    EarthlyBranch::Wu,
    EarthlyBranch::Wei,
    EarthlyBranch::Shen,
    EarthlyBranch::You,
    EarthlyBranch::Xu,
    EarthlyBranch::Hai,
];

/// 十二宫位顺序
pub const PALACES: [Palace; 12] = [
    Palace::Soul,
    Palace::Parents,
    Palace::Spirit,
    Palace::Property,
    Palace::Career,
    Palace::Friends,
    Palace::Surface,
    Palace::Health,
    Palace::Wealth,
    Palace::Children,
    Palace::Spouse,
    Palace::Siblings,
];

/// 时辰对应的时间范围
pub const TIME_RANGES: [&str; 13] = [
    "00:00~01:00",
    "01:00~03:00",
    "03:00~05:00",
    "05:00~07:00",
    "07:00~09:00",
    "09:00~11:00",
    "11:00~13:00",
    "13:00~15:00",
    "15:00~17:00",
    "17:00~19:00",
    "19:00~21:00",
    "21:00~23:00",
    "23:00~00:00",
];

/// 五虎遁：年干 → 正月月干
/// 索引对应天干序号（甲=0, 乙=1, ...）
pub const TIGER_RULE: [HeavenlyStem; 10] = [
    HeavenlyStem::Bing, // 甲 → 丙
    HeavenlyStem::Wu,   // 乙 → 戊
    HeavenlyStem::Geng, // 丙 → 庚
    HeavenlyStem::Ren,  // 丁 → 壬
    HeavenlyStem::Jia,  // 戊 → 甲
    HeavenlyStem::Bing, // 己 → 丙
    HeavenlyStem::Wu,   // 庚 → 戊
    HeavenlyStem::Geng, // 辛 → 庚
    HeavenlyStem::Ren,  // 壬 → 壬
    HeavenlyStem::Jia,  // 癸 → 甲
];

/// 五鼠遁：日干 → 子时时干
/// 索引对应天干序号（甲=0, 乙=1, ...）
pub const RAT_RULE: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,  // 甲 → 甲
    HeavenlyStem::Bing, // 乙 → 丙
    HeavenlyStem::Wu,   // 丙 → 戊
    HeavenlyStem::Geng, // 丁 → 庚
    HeavenlyStem::Ren,  // 戊 → 壬
    HeavenlyStem::Jia,  // 己 → 甲
    HeavenlyStem::Bing, // 庚 → 丙
    HeavenlyStem::Wu,   // 辛 → 戊
    HeavenlyStem::Geng, // 壬 → 庚
    HeavenlyStem::Ren,  // 癸 → 壬
];

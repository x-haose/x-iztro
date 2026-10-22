use crate::data::stars::StarKey;
use crate::data::types::*;

/// 星耀名称
pub fn star_name(key: StarKey) -> &'static str {
    match key {
        StarKey::ZiweiMaj => "紫微",
        StarKey::TianjiMaj => "天機",
        StarKey::TaiyangMaj => "太陽",
        StarKey::WuquMaj => "武曲",
        StarKey::TiantongMaj => "天同",
        StarKey::LianzhenMaj => "廉貞",
        StarKey::TianfuMaj => "天府",
        StarKey::TaiyinMaj => "太陰",
        StarKey::TanlangMaj => "貪狼",
        StarKey::JumenMaj => "巨門",
        StarKey::TianxiangMaj => "天相",
        StarKey::TianliangMaj => "天梁",
        StarKey::QishaMaj => "七殺",
        StarKey::PojunMaj => "破軍",
        StarKey::ZuofuMin => "左輔",
        StarKey::YoubiMin => "右弼",
        StarKey::WenchangMin => "文昌",
        StarKey::WenquMin => "文曲",
        StarKey::LucunMin => "禄存",
        StarKey::TianmaMin => "天馬",
        StarKey::QingyangMin => "擎羊",
        StarKey::TuoluoMin => "陀羅",
        StarKey::HuoxingMin => "火星",
        StarKey::LingxingMin => "鈴星",
        StarKey::TiankuiMin => "天魁",
        StarKey::TianyueMin => "天鉞",
        StarKey::DikongMin => "地空",
        StarKey::DijieMin => "地劫",
        StarKey::JieshaAdj => "劫殺",
        StarKey::Tiankong => "天空",
        StarKey::Tianxing => "天刑",
        StarKey::Tianyao => "天姚",
        StarKey::Jieshen => "解神",
        StarKey::Yinsha => "陰煞",
        StarKey::Tianxi => "天喜",
        StarKey::Tianguan => "天官",
        StarKey::Tianfu => "天福",
        StarKey::Tianku => "天哭",
        StarKey::Tianxu => "天虚",
        StarKey::Longchi => "竜池",
        StarKey::Fengge => "鳳閣",
        StarKey::Hongluan => "紅鸞",
        StarKey::Guchen => "孤辰",
        StarKey::Guasu => "寡宿",
        StarKey::Feilian => "蜚廉",
        StarKey::Posui => "破碎",
        StarKey::Taifu => "台輔",
        StarKey::Fenggao => "封誥",
        StarKey::Tianwu => "天巫",
        StarKey::Tianyue2 => "天月",
        StarKey::Santai => "三台",
        StarKey::Bazuo => "八座",
        StarKey::Engguang => "恩光",
        StarKey::Tiangui => "天貴",
        StarKey::Tiancai => "天才",
        StarKey::Tianshou => "天寿",
        StarKey::Jiekong => "截空",
        StarKey::Xunzhong => "旬中",
        StarKey::Xunkong => "旬空",
        StarKey::Kongwang => "空亡",
        StarKey::Jielu => "截路",
        StarKey::Yuede => "月徳",
        StarKey::Tianshang => "天傷",
        StarKey::Tianshi => "天使",
        StarKey::Tianchu => "天厨",
        StarKey::Changsheng => "長生",
        StarKey::Muyu => "沐浴",
        StarKey::Guandai => "冠帯",
        StarKey::Linguan => "臨官",
        StarKey::Diwang => "帝旺",
        StarKey::Shuai => "衰",
        StarKey::Bing => "病",
        StarKey::Si => "死",
        StarKey::Mu => "墓",
        StarKey::Jue => "絶",
        StarKey::Tai => "胎",
        StarKey::Yang => "養",
        StarKey::Boshi => "博士",
        StarKey::Lishi => "力士",
        StarKey::Qinglong => "青竜",
        StarKey::Xiaohao => "小耗",
        StarKey::Jiangjun => "将軍",
        StarKey::Zhoushu => "奏書",
        StarKey::Faylian => "飛廉",
        StarKey::Xishen => "喜神",
        StarKey::Bingfu => "病符",
        StarKey::Dahao => "大耗",
        StarKey::Suipo => "歳破",
        StarKey::Fubing => "伏兵",
        StarKey::Guanfu => "官府",
        StarKey::Suijian => "歳建",
        StarKey::Huiqi => "晦気",
        StarKey::Sangmen => "喪門",
        StarKey::Guansuo => "貫索",
        StarKey::Gwanfu => "官符",
        StarKey::Longde => "竜徳",
        StarKey::Baihu => "白虎",
        StarKey::Tiande => "天徳",
        StarKey::Diaoke => "弔客",
        StarKey::Jiangxing => "将星",
        StarKey::Panan => "攀鞍",
        StarKey::Suiyi => "歳駅",
        StarKey::Xiishen => "息神",
        StarKey::Huagai => "華蓋",
        StarKey::Jiesha => "劫煞",
        StarKey::Zhaisha => "災煞",
        StarKey::Tiansha => "天煞",
        StarKey::Zhibei => "指背",
        StarKey::Xianchi => "咸池",
        StarKey::Yuesha => "月煞",
        StarKey::Wangshen => "亡神",
        StarKey::Yunkui => "限の魁",
        StarKey::Yunyue => "限の鉞",
        StarKey::Yunchang => "限の昌",
        StarKey::Yunqu => "限の曲",
        StarKey::Yunluan => "限の鸞",
        StarKey::Yunxi => "限の喜",
        StarKey::Yunlu => "限の禄",
        StarKey::Yunyang => "限の羊",
        StarKey::Yuntuo => "限の陀",
        StarKey::Yunma => "限の馬",
        StarKey::Liukui => "年の魁",
        StarKey::Liuyue => "年の鉞",
        StarKey::Liuchang => "年の昌",
        StarKey::Liuqu => "年の曲",
        StarKey::Liuluan => "年の鸞",
        StarKey::Liuxi => "年の喜",
        StarKey::Liulu => "年の禄",
        StarKey::Liuyang => "年の羊",
        StarKey::Liutuo => "年の陀",
        StarKey::Liuma => "年の馬",
        StarKey::Nianjie => "年の解",
        StarKey::Yuekui => "月の魁",
        StarKey::Yueyue => "月の鉞",
        StarKey::Yuechang => "月の昌",
        StarKey::Yuequ => "月の曲",
        StarKey::Yueluan => "月の鸞",
        StarKey::Yuexi => "月の喜",
        StarKey::Yuelu => "月の禄",
        StarKey::Yueyang => "月の羊",
        StarKey::Yuetuo => "月の陀",
        StarKey::Yuema => "月の馬",
        StarKey::Rikui => "日の魁",
        StarKey::Riyue => "日の鉞",
        StarKey::Richang => "日の昌",
        StarKey::Riqu => "日の曲",
        StarKey::Riluan => "日の鸞",
        StarKey::Rixi => "日の喜",
        StarKey::Rilu => "日の禄",
        StarKey::Riyang => "日の羊",
        StarKey::Rituo => "日の陀",
        StarKey::Rima => "日の馬",
        StarKey::Shikui => "時の魁",
        StarKey::Shiyue => "時の鉞",
        StarKey::Shichang => "時の昌",
        StarKey::Shiqu => "時の曲",
        StarKey::Shiluan => "時の鸞",
        StarKey::Shixi => "時の喜",
        StarKey::Shilu => "時の禄",
        StarKey::Shiyang => "時の羊",
        StarKey::Shituo => "時の陀",
        StarKey::Shima => "時の馬",
    }
}

/// 宫位名称
pub fn palace_name(palace: Palace) -> &'static str {
    match palace {
        Palace::Soul => "命宮",
        Palace::Parents => "父母",
        Palace::Spirit => "福徳",
        Palace::Property => "田宅",
        Palace::Career => "官禄",
        Palace::Friends => "奴僕",
        Palace::Surface => "遷移",
        Palace::Health => "疾厄",
        Palace::Wealth => "財帛",
        Palace::Children => "子女",
        Palace::Spouse => "夫妻",
        Palace::Siblings => "兄弟",
    }
}

/// 天干名称
pub fn heavenly_stem_name(stem: HeavenlyStem) -> &'static str {
    match stem {
        HeavenlyStem::Jia => "甲",
        HeavenlyStem::Yi => "乙",
        HeavenlyStem::Bing => "丙",
        HeavenlyStem::Ding => "丁",
        HeavenlyStem::Wu => "戊",
        HeavenlyStem::Ji => "己",
        HeavenlyStem::Geng => "庚",
        HeavenlyStem::Xin => "辛",
        HeavenlyStem::Ren => "壬",
        HeavenlyStem::Gui => "癸",
    }
}

/// 地支名称
pub fn earthly_branch_name(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "子",
        EarthlyBranch::Chou => "丑",
        EarthlyBranch::Yin => "寅",
        EarthlyBranch::Mao => "卯",
        EarthlyBranch::Chen => "辰",
        EarthlyBranch::Si => "巳",
        EarthlyBranch::Wu => "午",
        EarthlyBranch::Wei => "未",
        EarthlyBranch::Shen => "申",
        EarthlyBranch::You => "酉",
        EarthlyBranch::Xu => "戌",
        EarthlyBranch::Hai => "亥",
    }
}

/// 亮度名称
pub fn brightness_name(b: Brightness) -> &'static str {
    match b {
        Brightness::Miao => "廟",
        Brightness::Wang => "旺",
        Brightness::De => "得",
        Brightness::Li => "利",
        Brightness::Ping => "平",
        Brightness::Bu => "不",
        Brightness::Xian => "陥",
    }
}

/// 四化名称
pub fn mutagen_name(m: Mutagen) -> &'static str {
    match m {
        Mutagen::Lu => "禄",
        Mutagen::Quan => "権",
        Mutagen::Ke => "科",
        Mutagen::Ji => "忌",
    }
}

/// 五行局名称
pub fn five_elements_class_name(c: FiveElementsClass) -> &'static str {
    match c {
        FiveElementsClass::Water2nd => "水の二局",
        FiveElementsClass::Wood3rd => "木の三局",
        FiveElementsClass::Metal4th => "金の四局",
        FiveElementsClass::Earth5th => "土の五局",
        FiveElementsClass::Fire6th => "火の六局",
    }
}

/// 性别名称
pub fn gender_name(g: Gender) -> &'static str {
    match g {
        Gender::Male => "男",
        Gender::Female => "女",
    }
}

/// 時辰名表（索引 0-12：早子時、丑時…亥時、晚子時）
const TIME_NAMES: [&str; 13] = [
    "早子時",
    "丑時",
    "寅時",
    "卯時",
    "辰時",
    "巳時",
    "午時",
    "未時",
    "申時",
    "酉時",
    "戌時",
    "亥時",
    "晩子時",
];

/// 星座名表（索引 0-11：おひつじ座起、黄道順）
const SIGN_NAMES: [&str; 12] = [
    "おひつじ座",
    "おうし座",
    "ふたご座",
    "かに座",
    "しし座",
    "おとめ座",
    "てんびん座",
    "さそり座",
    "いて座",
    "やぎ座",
    "みずがめ座",
    "うお座",
];

/// 干支動物名表（地支索引順：子鼠…亥豚）
const ZODIAC_NAMES: [&str; 12] = [
    "鼠", "牛", "虎", "兎", "竜", "蛇", "馬", "羊", "猿", "鶏", "犬", "豚",
];

/// 时辰名称（索引 0-12）
pub fn time_name(time_index: u8) -> &'static str {
    TIME_NAMES[time_index as usize]
}

/// 星座名称（索引 0-11，白羊座起）
pub fn sign_name(sign_index: usize) -> &'static str {
    SIGN_NAMES[sign_index]
}

/// 生肖名称（按地支）
pub fn zodiac_name(branch: EarthlyBranch) -> &'static str {
    ZODIAC_NAMES[branch.index()]
}

/// 运限层级名称
pub fn horoscope_name(n: HoroscopeName) -> &'static str {
    match n {
        HoroscopeName::Decadal => "大限",
        HoroscopeName::Childhood => "子供",
        HoroscopeName::Age => "小限",
        HoroscopeName::Yearly => "流年",
        HoroscopeName::Monthly => "流月",
        HoroscopeName::Daily => "流日",
        HoroscopeName::Hourly => "流時",
    }
}

/// 星盘的农历日期写法：「农历」一词加 [`lunar_date_body`]（对应 iztro 的 `lunarDate` 模板）
pub fn lunar_date(year: i64, month: u32, day: u32, is_leap: bool) -> String {
    format!("旧暦{}", lunar_date_body(year, month, day, is_leap))
}

/// 农历日期去掉「农历」一词的部分：数字年月日，闰月带前缀；用于已有标签的场合
pub fn lunar_date_body(year: i64, month: u32, day: u32, is_leap: bool) -> String {
    let leap = if is_leap { "閏" } else { "" };
    format!("{year}年{leap}{month}月{day}日")
}

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
        StarKey::LucunMin => "祿存",
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
        StarKey::Tianxu => "天虛",
        StarKey::Longchi => "龍池",
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
        StarKey::Tianshou => "天壽",
        StarKey::Jiekong => "截空",
        StarKey::Xunzhong => "旬中",
        StarKey::Xunkong => "旬空",
        StarKey::Kongwang => "空亡",
        StarKey::Jielu => "截路",
        StarKey::Yuede => "月德",
        StarKey::Tianshang => "天傷",
        StarKey::Tianshi => "天使",
        StarKey::Tianchu => "天廚",
        StarKey::Changsheng => "長生",
        StarKey::Muyu => "沐浴",
        StarKey::Guandai => "冠帶",
        StarKey::Linguan => "臨官",
        StarKey::Diwang => "帝旺",
        StarKey::Shuai => "衰",
        StarKey::Bing => "病",
        StarKey::Si => "死",
        StarKey::Mu => "墓",
        StarKey::Jue => "絕",
        StarKey::Tai => "胎",
        StarKey::Yang => "養",
        StarKey::Boshi => "博士",
        StarKey::Lishi => "力士",
        StarKey::Qinglong => "青龍",
        StarKey::Xiaohao => "小耗",
        StarKey::Jiangjun => "將軍",
        StarKey::Zhoushu => "奏書",
        StarKey::Faylian => "飛廉",
        StarKey::Xishen => "喜神",
        StarKey::Bingfu => "病符",
        StarKey::Dahao => "大耗",
        StarKey::Suipo => "歲破",
        StarKey::Fubing => "伏兵",
        StarKey::Guanfu => "官府",
        StarKey::Suijian => "歲建",
        StarKey::Huiqi => "晦氣",
        StarKey::Sangmen => "喪門",
        StarKey::Guansuo => "貫索",
        StarKey::Gwanfu => "官符",
        StarKey::Longde => "龍德",
        StarKey::Baihu => "白虎",
        StarKey::Tiande => "天德",
        StarKey::Diaoke => "弔客",
        StarKey::Jiangxing => "將星",
        StarKey::Panan => "攀鞍",
        StarKey::Suiyi => "歲驛",
        StarKey::Xiishen => "息神",
        StarKey::Huagai => "華蓋",
        StarKey::Jiesha => "劫煞",
        StarKey::Zhaisha => "災煞",
        StarKey::Tiansha => "天煞",
        StarKey::Zhibei => "指背",
        StarKey::Xianchi => "咸池",
        StarKey::Yuesha => "月煞",
        StarKey::Wangshen => "亡神",
        StarKey::Yunkui => "運魁",
        StarKey::Yunyue => "運鉞",
        StarKey::Yunchang => "運昌",
        StarKey::Yunqu => "運曲",
        StarKey::Yunluan => "運鸞",
        StarKey::Yunxi => "運喜",
        StarKey::Yunlu => "運祿",
        StarKey::Yunyang => "運羊",
        StarKey::Yuntuo => "運陀",
        StarKey::Yunma => "運馬",
        StarKey::Liukui => "流魁",
        StarKey::Liuyue => "流鉞",
        StarKey::Liuchang => "流昌",
        StarKey::Liuqu => "流曲",
        StarKey::Liuluan => "流鸞",
        StarKey::Liuxi => "流喜",
        StarKey::Liulu => "流祿",
        StarKey::Liuyang => "流羊",
        StarKey::Liutuo => "流陀",
        StarKey::Liuma => "流馬",
        StarKey::Nianjie => "年解",
        StarKey::Yuekui => "月魁",
        StarKey::Yueyue => "月鉞",
        StarKey::Yuechang => "月昌",
        StarKey::Yuequ => "月曲",
        StarKey::Yueluan => "月鸞",
        StarKey::Yuexi => "月喜",
        StarKey::Yuelu => "月祿",
        StarKey::Yueyang => "月羊",
        StarKey::Yuetuo => "月陀",
        StarKey::Yuema => "月馬",
        StarKey::Rikui => "日魁",
        StarKey::Riyue => "日鉞",
        StarKey::Richang => "日昌",
        StarKey::Riqu => "日曲",
        StarKey::Riluan => "日鸞",
        StarKey::Rixi => "日喜",
        StarKey::Rilu => "日祿",
        StarKey::Riyang => "日羊",
        StarKey::Rituo => "日陀",
        StarKey::Rima => "日馬",
        StarKey::Shikui => "時魁",
        StarKey::Shiyue => "時鉞",
        StarKey::Shichang => "時昌",
        StarKey::Shiqu => "時曲",
        StarKey::Shiluan => "時鸞",
        StarKey::Shixi => "時喜",
        StarKey::Shilu => "時祿",
        StarKey::Shiyang => "時羊",
        StarKey::Shituo => "時陀",
        StarKey::Shima => "時馬",
    }
}

/// 宫位名称
pub fn palace_name(palace: Palace) -> &'static str {
    match palace {
        Palace::Soul => "命宮",
        Palace::Parents => "父母",
        Palace::Spirit => "福德",
        Palace::Property => "田宅",
        Palace::Career => "官祿",
        Palace::Friends => "僕役",
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
        Brightness::Xian => "陷",
    }
}

/// 四化名称
pub fn mutagen_name(m: Mutagen) -> &'static str {
    match m {
        Mutagen::Lu => "祿",
        Mutagen::Quan => "權",
        Mutagen::Ke => "科",
        Mutagen::Ji => "忌",
    }
}

/// 五行局名称
pub fn five_elements_class_name(c: FiveElementsClass) -> &'static str {
    match c {
        FiveElementsClass::Water2nd => "水二局",
        FiveElementsClass::Wood3rd => "木三局",
        FiveElementsClass::Metal4th => "金四局",
        FiveElementsClass::Earth5th => "土五局",
        FiveElementsClass::Fire6th => "火六局",
    }
}

/// 性别名称
pub fn gender_name(g: Gender) -> &'static str {
    match g {
        Gender::Male => "男",
        Gender::Female => "女",
    }
}

/// 時辰名稱表（索引 0-12：早子時、丑時…亥時、晚子時）
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
    "晚子時",
];

/// 星座名稱表（索引 0-11：白羊座起，按黃道順序）
const SIGN_NAMES: [&str; 12] = [
    "白羊座",
    "金牛座",
    "雙子座",
    "巨蟹座",
    "獅子座",
    "處女座",
    "天秤座",
    "天蠍座",
    "射手座",
    "摩羯座",
    "水瓶座",
    "雙魚座",
];

/// 生肖名稱表（按地支索引：子鼠…亥豬）
const ZODIAC_NAMES: [&str; 12] = [
    "鼠", "牛", "虎", "兔", "龍", "蛇", "馬", "羊", "猴", "雞", "狗", "豬",
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
        HoroscopeName::Childhood => "童限",
        HoroscopeName::Age => "小限",
        HoroscopeName::Yearly => "流年",
        HoroscopeName::Monthly => "流月",
        HoroscopeName::Daily => "流日",
        HoroscopeName::Hourly => "流時",
    }
}

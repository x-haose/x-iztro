use crate::data::stars::StarKey;
use crate::data::types::*;

pub fn star_name(key: StarKey) -> &'static str {
    match key {
        StarKey::ZiweiMaj => "紫微",
        StarKey::TianjiMaj => "天机",
        StarKey::TaiyangMaj => "太阳",
        StarKey::WuquMaj => "武曲",
        StarKey::TiantongMaj => "天同",
        StarKey::LianzhenMaj => "廉贞",
        StarKey::TianfuMaj => "天府",
        StarKey::TaiyinMaj => "太阴",
        StarKey::TanlangMaj => "贪狼",
        StarKey::JumenMaj => "巨门",
        StarKey::TianxiangMaj => "天相",
        StarKey::TianliangMaj => "天梁",
        StarKey::QishaMaj => "七杀",
        StarKey::PojunMaj => "破军",
        StarKey::ZuofuMin => "左辅",
        StarKey::YoubiMin => "右弼",
        StarKey::WenchangMin => "文昌",
        StarKey::WenquMin => "文曲",
        StarKey::LucunMin => "禄存",
        StarKey::TianmaMin => "天马",
        StarKey::QingyangMin => "擎羊",
        StarKey::TuoluoMin => "陀罗",
        StarKey::HuoxingMin => "火星",
        StarKey::LingxingMin => "铃星",
        StarKey::TiankuiMin => "天魁",
        StarKey::TianyueMin => "天钺",
        StarKey::DikongMin => "地空",
        StarKey::DijieMin => "地劫",
        StarKey::JieshaAdj => "劫杀",
        StarKey::Tiankong => "天空",
        StarKey::Tianxing => "天刑",
        StarKey::Tianyao => "天姚",
        StarKey::Jieshen => "解神",
        StarKey::Yinsha => "阴煞",
        StarKey::Tianxi => "天喜",
        StarKey::Tianguan => "天官",
        StarKey::Tianfu => "天福",
        StarKey::Tianku => "天哭",
        StarKey::Tianxu => "天虚",
        StarKey::Longchi => "龙池",
        StarKey::Fengge => "凤阁",
        StarKey::Hongluan => "红鸾",
        StarKey::Guchen => "孤辰",
        StarKey::Guasu => "寡宿",
        StarKey::Feilian => "蜚廉",
        StarKey::Posui => "破碎",
        StarKey::Taifu => "台辅",
        StarKey::Fenggao => "封诰",
        StarKey::Tianwu => "天巫",
        StarKey::Tianyue2 => "天月",
        StarKey::Santai => "三台",
        StarKey::Bazuo => "八座",
        StarKey::Engguang => "恩光",
        StarKey::Tiangui => "天贵",
        StarKey::Tiancai => "天才",
        StarKey::Tianshou => "天寿",
        StarKey::Jiekong => "截空",
        StarKey::Xunzhong => "旬中",
        StarKey::Xunkong => "旬空",
        StarKey::Kongwang => "空亡",
        StarKey::Jielu => "截路",
        StarKey::Yuede => "月德",
        StarKey::Tianshang => "天伤",
        StarKey::Tianshi => "天使",
        StarKey::Tianchu => "天厨",
        StarKey::Changsheng => "长生",
        StarKey::Muyu => "沐浴",
        StarKey::Guandai => "冠带",
        StarKey::Linguan => "临官",
        StarKey::Diwang => "帝旺",
        StarKey::Shuai => "衰",
        StarKey::Bing => "病",
        StarKey::Si => "死",
        StarKey::Mu => "墓",
        StarKey::Jue => "绝",
        StarKey::Tai => "胎",
        StarKey::Yang => "养",
        StarKey::Boshi => "博士",
        StarKey::Lishi => "力士",
        StarKey::Qinglong => "青龙",
        StarKey::Xiaohao => "小耗",
        StarKey::Jiangjun => "将军",
        StarKey::Zhoushu => "奏书",
        StarKey::Faylian => "飞廉",
        StarKey::Xishen => "喜神",
        StarKey::Bingfu => "病符",
        StarKey::Dahao => "大耗",
        StarKey::Suipo => "岁破",
        StarKey::Fubing => "伏兵",
        StarKey::Guanfu => "官府",
        StarKey::Suijian => "岁建",
        StarKey::Huiqi => "晦气",
        StarKey::Sangmen => "丧门",
        StarKey::Guansuo => "贯索",
        StarKey::Gwanfu => "官符",
        StarKey::Longde => "龙德",
        StarKey::Baihu => "白虎",
        StarKey::Tiande => "天德",
        StarKey::Diaoke => "吊客",
        StarKey::Jiangxing => "将星",
        StarKey::Panan => "攀鞍",
        StarKey::Suiyi => "岁驿",
        StarKey::Xiishen => "息神",
        StarKey::Huagai => "华盖",
        StarKey::Jiesha => "劫煞",
        StarKey::Zhaisha => "灾煞",
        StarKey::Tiansha => "天煞",
        StarKey::Zhibei => "指背",
        StarKey::Xianchi => "咸池",
        StarKey::Yuesha => "月煞",
        StarKey::Wangshen => "亡神",
        StarKey::Yunkui => "运魁",
        StarKey::Yunyue => "运钺",
        StarKey::Yunchang => "运昌",
        StarKey::Yunqu => "运曲",
        StarKey::Yunluan => "运鸾",
        StarKey::Yunxi => "运喜",
        StarKey::Yunlu => "运禄",
        StarKey::Yunyang => "运羊",
        StarKey::Yuntuo => "运陀",
        StarKey::Yunma => "运马",
        StarKey::Liukui => "流魁",
        StarKey::Liuyue => "流钺",
        StarKey::Liuchang => "流昌",
        StarKey::Liuqu => "流曲",
        StarKey::Liuluan => "流鸾",
        StarKey::Liuxi => "流喜",
        StarKey::Liulu => "流禄",
        StarKey::Liuyang => "流羊",
        StarKey::Liutuo => "流陀",
        StarKey::Liuma => "流马",
        StarKey::Nianjie => "年解",
        StarKey::Yuekui => "月魁",
        StarKey::Yueyue => "月钺",
        StarKey::Yuechang => "月昌",
        StarKey::Yuequ => "月曲",
        StarKey::Yueluan => "月鸾",
        StarKey::Yuexi => "月喜",
        StarKey::Yuelu => "月禄",
        StarKey::Yueyang => "月羊",
        StarKey::Yuetuo => "月陀",
        StarKey::Yuema => "月马",
        StarKey::Rikui => "日魁",
        StarKey::Riyue => "日钺",
        StarKey::Richang => "日昌",
        StarKey::Riqu => "日曲",
        StarKey::Riluan => "日鸾",
        StarKey::Rixi => "日喜",
        StarKey::Rilu => "日禄",
        StarKey::Riyang => "日羊",
        StarKey::Rituo => "日陀",
        StarKey::Rima => "日马",
        StarKey::Shikui => "时魁",
        StarKey::Shiyue => "时钺",
        StarKey::Shichang => "时昌",
        StarKey::Shiqu => "时曲",
        StarKey::Shiluan => "时鸾",
        StarKey::Shixi => "时喜",
        StarKey::Shilu => "时禄",
        StarKey::Shiyang => "时羊",
        StarKey::Shituo => "时陀",
        StarKey::Shima => "时马",
    }
}

pub fn palace_name(palace: Palace) -> &'static str {
    match palace {
        Palace::Soul => "命宫",
        Palace::Parents => "父母",
        Palace::Spirit => "福德",
        Palace::Property => "田宅",
        Palace::Career => "官禄",
        Palace::Friends => "仆役",
        Palace::Surface => "迁移",
        Palace::Health => "疾厄",
        Palace::Wealth => "财帛",
        Palace::Children => "子女",
        Palace::Spouse => "夫妻",
        Palace::Siblings => "兄弟",
    }
}

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

pub fn brightness_name(b: Brightness) -> &'static str {
    match b {
        Brightness::Miao => "庙",
        Brightness::Wang => "旺",
        Brightness::De => "得",
        Brightness::Li => "利",
        Brightness::Ping => "平",
        Brightness::Bu => "不",
        Brightness::Xian => "陷",
    }
}

pub fn mutagen_name(m: Mutagen) -> &'static str {
    match m {
        Mutagen::Lu => "禄",
        Mutagen::Quan => "权",
        Mutagen::Ke => "科",
        Mutagen::Ji => "忌",
    }
}

pub fn five_elements_class_name(c: FiveElementsClass) -> &'static str {
    match c {
        FiveElementsClass::Water2nd => "水二局",
        FiveElementsClass::Wood3rd => "木三局",
        FiveElementsClass::Metal4th => "金四局",
        FiveElementsClass::Earth5th => "土五局",
        FiveElementsClass::Fire6th => "火六局",
    }
}

pub fn gender_name(g: Gender) -> &'static str {
    match g {
        Gender::Male => "男",
        Gender::Female => "女",
    }
}

/// 时辰名称表（索引 0-12：早子时、丑时…亥时、晚子时）
const TIME_NAMES: [&str; 13] = [
    "早子时", "丑时", "寅时", "卯时", "辰时", "巳时", "午时",
    "未时", "申时", "酉时", "戌时", "亥时", "晚子时",
];

/// 星座名称表（索引 0-11：白羊座起，按黄道顺序）
const SIGN_NAMES: [&str; 12] = [
    "白羊座", "金牛座", "双子座", "巨蟹座", "狮子座", "处女座",
    "天秤座", "天蝎座", "射手座", "摩羯座", "水瓶座", "双鱼座",
];

/// 生肖名称表（按地支索引：子鼠…亥猪）
const ZODIAC_NAMES: [&str; 12] = [
    "鼠", "牛", "虎", "兔", "龙", "蛇", "马", "羊", "猴", "鸡", "狗", "猪",
];

pub fn time_name(time_index: u8) -> &'static str {
    TIME_NAMES[time_index as usize]
}

pub fn sign_name(sign_index: usize) -> &'static str {
    SIGN_NAMES[sign_index]
}

pub fn zodiac_name(branch: EarthlyBranch) -> &'static str {
    ZODIAC_NAMES[branch.index()]
}

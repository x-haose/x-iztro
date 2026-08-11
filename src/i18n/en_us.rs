use crate::data::stars::StarKey;
use crate::data::types::*;

pub fn star_name(key: StarKey) -> &'static str {
    match key {
        StarKey::ZiweiMaj => "emperor",
        StarKey::TianjiMaj => "advisor",
        StarKey::TaiyangMaj => "sun",
        StarKey::WuquMaj => "general",
        StarKey::TiantongMaj => "fortunate",
        StarKey::LianzhenMaj => "judge",
        StarKey::TianfuMaj => "empress",
        StarKey::TaiyinMaj => "moon",
        StarKey::TanlangMaj => "wolf",
        StarKey::JumenMaj => "advocator",
        StarKey::TianxiangMaj => "minister",
        StarKey::TianliangMaj => "sage",
        StarKey::QishaMaj => "marshal",
        StarKey::PojunMaj => "rebel",
        StarKey::ZuofuMin => "officer",
        StarKey::YoubiMin => "helper",
        StarKey::WenchangMin => "scholar",
        StarKey::WenquMin => "artist",
        StarKey::LucunMin => "money",
        StarKey::TianmaMin => "horse",
        StarKey::QingyangMin => "driven",
        StarKey::TuoluoMin => "tangled",
        StarKey::HuoxingMin => "impulsive",
        StarKey::LingxingMin => "spark",
        StarKey::TiankuiMin => "assistant",
        StarKey::TianyueMin => "aide",
        StarKey::DikongMin => "ideologue",
        StarKey::DijieMin => "fickle",
        StarKey::JieshaAdj => "murder",
        StarKey::Tiankong => "utopian",
        StarKey::Tianxing => "serious",
        StarKey::Tianyao => "social",
        StarKey::Jieshen => "considery",
        StarKey::Yinsha => "gloomy",
        StarKey::Tianxi => "cheerful",
        StarKey::Tianguan => "solemn",
        StarKey::Tianfu => "lucky",
        StarKey::Tianku => "upset",
        StarKey::Tianxu => "frail",
        StarKey::Longchi => "talented",
        StarKey::Fengge => "refined",
        StarKey::Hongluan => "attractive",
        StarKey::Guchen => "alone",
        StarKey::Guasu => "lonely",
        StarKey::Feilian => "instigated",
        StarKey::Posui => "broken",
        StarKey::Taifu => "honorable",
        StarKey::Fenggao => "awarded",
        StarKey::Tianwu => "psychic",
        StarKey::Tianyue2 => "sickly",
        StarKey::Santai => "senior",
        StarKey::Bazuo => "dignified",
        StarKey::Engguang => "grateful",
        StarKey::Tiangui => "noble",
        StarKey::Tiancai => "gifted",
        StarKey::Tianshou => "ageless",
        StarKey::Jiekong => "interrupted",
        StarKey::Xunzhong => "meditative",
        StarKey::Xunkong => "fancied",
        StarKey::Kongwang => "bottomless",
        StarKey::Jielu => "intercepted",
        StarKey::Yuede => "peaceful",
        StarKey::Tianshang => "wounded",
        StarKey::Tianshi => "heaven",
        StarKey::Tianchu => "gourmet",
        StarKey::Changsheng => "born",
        StarKey::Muyu => "infancy",
        StarKey::Guandai => "adolescence",
        StarKey::Linguan => "adulthood",
        StarKey::Diwang => "prime",
        StarKey::Shuai => "weak",
        StarKey::Bing => "sick",
        StarKey::Si => "dead",
        StarKey::Mu => "buried",
        StarKey::Jue => "dissipated",
        StarKey::Tai => "embryo",
        StarKey::Yang => "molding",
        StarKey::Boshi => "doctor",
        StarKey::Lishi => "sumo",
        StarKey::Qinglong => "dragon",
        StarKey::Xiaohao => "consumer",
        StarKey::Jiangjun => "general",
        StarKey::Zhoushu => "book",
        StarKey::Faylian => "gossip",
        StarKey::Xishen => "happiness",
        StarKey::Bingfu => "illness",
        StarKey::Dahao => "wastrel",
        StarKey::Suipo => "wastrel",
        StarKey::Fubing => "ambush",
        StarKey::Guanfu => "government",
        StarKey::Suijian => "initial",
        StarKey::Huiqi => "unlucky",
        StarKey::Sangmen => "downcast",
        StarKey::Guansuo => "tied",
        StarKey::Gwanfu => "official",
        StarKey::Longde => "virtuous",
        StarKey::Baihu => "sinister",
        StarKey::Tiande => "blessed",
        StarKey::Diaoke => "sorrowing",
        StarKey::Jiangxing => "capable",
        StarKey::Panan => "admired",
        StarKey::Suiyi => "varied",
        StarKey::Xiishen => "listless",
        StarKey::Huagai => "religious",
        StarKey::Jiesha => "robbed",
        StarKey::Zhaisha => "disastery",
        StarKey::Tiansha => "condemned",
        StarKey::Zhibei => "insidious",
        StarKey::Xianchi => "passionate",
        StarKey::Yuesha => "hapless",
        StarKey::Wangshen => "perished",
        StarKey::Yunkui => "assistant(D)",
        StarKey::Yunyue => "aide(D)",
        StarKey::Yunchang => "scholar(D)",
        StarKey::Yunqu => "artist(D)",
        StarKey::Yunluan => "attractive(D)",
        StarKey::Yunxi => "cheerful(D)",
        StarKey::Yunlu => "money(D)",
        StarKey::Yunyang => "driven(D)",
        StarKey::Yuntuo => "tangled(D)",
        StarKey::Yunma => "horse(D)",
        StarKey::Liukui => "assistant(Y)",
        StarKey::Liuyue => "aide(Y)",
        StarKey::Liuchang => "scholar(Y)",
        StarKey::Liuqu => "artist(Y)",
        StarKey::Liuluan => "attractive(Y)",
        StarKey::Liuxi => "cheerful(Y)",
        StarKey::Liulu => "money(Y)",
        StarKey::Liuyang => "driven(Y)",
        StarKey::Liutuo => "tangled(Y)",
        StarKey::Liuma => "horse(Y)",
        StarKey::Nianjie => "considery(Y)",
        StarKey::Yuekui => "assistant(M)",
        StarKey::Yueyue => "aide(M)",
        StarKey::Yuechang => "scholar(M)",
        StarKey::Yuequ => "artist(M)",
        StarKey::Yueluan => "attractive(M)",
        StarKey::Yuexi => "cheerful(M)",
        StarKey::Yuelu => "money(M)",
        StarKey::Yueyang => "driven(M)",
        StarKey::Yuetuo => "tangled(M)",
        StarKey::Yuema => "horse(M)",
        StarKey::Rikui => "assistant(d)",
        StarKey::Riyue => "aide(d)",
        StarKey::Richang => "scholar(d)",
        StarKey::Riqu => "artist(d)",
        StarKey::Riluan => "attractive(d)",
        StarKey::Rixi => "cheerful(d)",
        StarKey::Rilu => "money(d)",
        StarKey::Riyang => "driven(d)",
        StarKey::Rituo => "tangled(d)",
        StarKey::Rima => "horse(d)",
        StarKey::Shikui => "assistant(H)",
        StarKey::Shiyue => "aide(H)",
        StarKey::Shichang => "scholar(H)",
        StarKey::Shiqu => "artist(H)",
        StarKey::Shiluan => "attractive(H)",
        StarKey::Shixi => "cheerful(H)",
        StarKey::Shilu => "money(H)",
        StarKey::Shiyang => "driven(H)",
        StarKey::Shituo => "tangled(H)",
        StarKey::Shima => "horse(H)",
    }
}

pub fn palace_name(palace: Palace) -> &'static str {
    match palace {
        Palace::Soul => "soul",
        Palace::Parents => "parents",
        Palace::Spirit => "spirit",
        Palace::Property => "property",
        Palace::Career => "career",
        Palace::Friends => "friends",
        Palace::Surface => "surface",
        Palace::Health => "health",
        Palace::Wealth => "wealth",
        Palace::Children => "children",
        Palace::Spouse => "spouse",
        Palace::Siblings => "siblings",
    }
}

pub fn heavenly_stem_name(stem: HeavenlyStem) -> &'static str {
    match stem {
        HeavenlyStem::Jia => "jia",
        HeavenlyStem::Yi => "yi",
        HeavenlyStem::Bing => "bing",
        HeavenlyStem::Ding => "ding",
        HeavenlyStem::Wu => "wu",
        HeavenlyStem::Ji => "ji",
        HeavenlyStem::Geng => "geng",
        HeavenlyStem::Xin => "xin",
        HeavenlyStem::Ren => "ren",
        HeavenlyStem::Gui => "gui",
    }
}

pub fn earthly_branch_name(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "zi",
        EarthlyBranch::Chou => "chou",
        EarthlyBranch::Yin => "yin",
        EarthlyBranch::Mao => "mao",
        EarthlyBranch::Chen => "chen",
        EarthlyBranch::Si => "si",
        EarthlyBranch::Wu => "woo",
        EarthlyBranch::Wei => "wei",
        EarthlyBranch::Shen => "shen",
        EarthlyBranch::You => "you",
        EarthlyBranch::Xu => "xu",
        EarthlyBranch::Hai => "hai",
    }
}

pub fn brightness_name(b: Brightness) -> &'static str {
    match b {
        Brightness::Miao => "[+3]",
        Brightness::Wang => "[+2]",
        Brightness::De => "[+1]",
        Brightness::Li => "[0]",
        Brightness::Ping => "[-1]",
        Brightness::Bu => "[-2]",
        Brightness::Xian => "[-3]",
    }
}

pub fn mutagen_name(m: Mutagen) -> &'static str {
    match m {
        Mutagen::Lu => "A",
        Mutagen::Quan => "B",
        Mutagen::Ke => "C",
        Mutagen::Ji => "D",
    }
}

pub fn five_elements_class_name(c: FiveElementsClass) -> &'static str {
    match c {
        FiveElementsClass::Water2nd => "water 2nd",
        FiveElementsClass::Wood3rd => "wood 3rd",
        FiveElementsClass::Metal4th => "metal 4th",
        FiveElementsClass::Earth5th => "earth 5th",
        FiveElementsClass::Fire6th => "fire 6th",
    }
}

pub fn gender_name(g: Gender) -> &'static str {
    match g {
        Gender::Male => "male",
        Gender::Female => "female",
    }
}

/// Hour-of-birth names (index 0-12: early Rat hour ... late Rat hour).
const TIME_NAMES: [&str; 13] = [
    "early Rat hour", "Ox hour", "Tiger hour", "Rabbit hour", "Dragon hour",
    "Snake hour", "Horse hour", "Goat hour", "Monkey hour", "Rooster hour",
    "Dog hour", "Pig hour", "late Rat hour",
];

/// Zodiac sign names (index 0-11: Aries onward, in ecliptic order).
const SIGN_NAMES: [&str; 12] = [
    "aries", "taurus", "gemini", "cancer", "leo", "virgo",
    "libra", "scorpio", "sagittarius", "capricorn", "aquarius", "pisces",
];

/// Chinese zodiac animal names (indexed by earthly branch: Zi=rat ... Hai=pig).
const ZODIAC_NAMES: [&str; 12] = [
    "rat", "ox", "tiger", "rabbit", "dragon", "snake",
    "horse", "sheep", "monkey", "rooster", "dog", "pig",
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

pub fn horoscope_name(n: HoroscopeName) -> &'static str {
    match n {
        HoroscopeName::Decadal => "decadal",
        HoroscopeName::Childhood => "childhood",
        HoroscopeName::Age => "age",
        HoroscopeName::Yearly => "yearly",
        HoroscopeName::Monthly => "monthly",
        HoroscopeName::Daily => "daily",
        HoroscopeName::Hourly => "hourly",
    }
}

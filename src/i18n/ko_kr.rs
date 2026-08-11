use crate::data::stars::StarKey;
use crate::data::types::*;

pub fn star_name(key: StarKey) -> &'static str {
    match key {
        StarKey::ZiweiMaj => "자미",
        StarKey::TianjiMaj => "천기",
        StarKey::TaiyangMaj => "태양",
        StarKey::WuquMaj => "무곡",
        StarKey::TiantongMaj => "천동",
        StarKey::LianzhenMaj => "염정",
        StarKey::TianfuMaj => "천부",
        StarKey::TaiyinMaj => "태음",
        StarKey::TanlangMaj => "탐랑",
        StarKey::JumenMaj => "거문",
        StarKey::TianxiangMaj => "천상",
        StarKey::TianliangMaj => "천량",
        StarKey::QishaMaj => "칠살",
        StarKey::PojunMaj => "파군",
        StarKey::ZuofuMin => "좌보",
        StarKey::YoubiMin => "우필",
        StarKey::WenchangMin => "문창",
        StarKey::WenquMin => "문곡",
        StarKey::LucunMin => "록존",
        StarKey::TianmaMin => "천마",
        StarKey::QingyangMin => "경양",
        StarKey::TuoluoMin => "타라",
        StarKey::HuoxingMin => "화성",
        StarKey::LingxingMin => "령성",
        StarKey::TiankuiMin => "천괴",
        StarKey::TianyueMin => "천월",
        StarKey::DikongMin => "지공",
        StarKey::DijieMin => "지겁",
        StarKey::JieshaAdj => "겁살",
        StarKey::Tiankong => "천공",
        StarKey::Tianxing => "천형",
        StarKey::Tianyao => "천요",
        StarKey::Jieshen => "해신",
        StarKey::Yinsha => "음살",
        StarKey::Tianxi => "천희",
        StarKey::Tianguan => "천관",
        StarKey::Tianfu => "천복",
        StarKey::Tianku => "천곡",
        StarKey::Tianxu => "천허",
        StarKey::Longchi => "용지",
        StarKey::Fengge => "봉각",
        StarKey::Hongluan => "홍란",
        StarKey::Guchen => "고진",
        StarKey::Guasu => "과숙",
        StarKey::Feilian => "비렴",
        StarKey::Posui => "파쇄",
        StarKey::Taifu => "태보",
        StarKey::Fenggao => "봉고",
        StarKey::Tianwu => "천무",
        StarKey::Tianyue2 => "천월",
        StarKey::Santai => "삼태",
        StarKey::Bazuo => "팔좌",
        StarKey::Engguang => "은광",
        StarKey::Tiangui => "천귀",
        StarKey::Tiancai => "천재",
        StarKey::Tianshou => "천수",
        StarKey::Jiekong => "절중",
        StarKey::Xunzhong => "순중",
        StarKey::Xunkong => "순공",
        StarKey::Kongwang => "공망",
        StarKey::Jielu => "절로",
        StarKey::Yuede => "월덕",
        StarKey::Tianshang => "천상",
        StarKey::Tianshi => "천사",
        StarKey::Tianchu => "천주",
        StarKey::Changsheng => "장생",
        StarKey::Muyu => "목욕",
        StarKey::Guandai => "관대",
        StarKey::Linguan => "임관",
        StarKey::Diwang => "제왕",
        StarKey::Shuai => "쇠",
        StarKey::Bing => "병",
        StarKey::Si => "사",
        StarKey::Mu => "묘",
        StarKey::Jue => "절",
        StarKey::Tai => "태",
        StarKey::Yang => "양",
        StarKey::Boshi => "박사",
        StarKey::Lishi => "역사",
        StarKey::Qinglong => "청룡",
        StarKey::Xiaohao => "소모",
        StarKey::Jiangjun => "장군",
        StarKey::Zhoushu => "주서",
        StarKey::Faylian => "비렴",
        StarKey::Xishen => "희신",
        StarKey::Bingfu => "병부",
        StarKey::Dahao => "대모",
        StarKey::Suipo => "태파",
        StarKey::Fubing => "복병",
        StarKey::Guanfu => "관부",
        StarKey::Suijian => "태세",
        StarKey::Huiqi => "회기",
        StarKey::Sangmen => "상문",
        StarKey::Guansuo => "관색",
        StarKey::Gwanfu => "관부",
        StarKey::Longde => "용덕",
        StarKey::Baihu => "백호",
        StarKey::Tiande => "복덕",
        StarKey::Diaoke => "조객",
        StarKey::Jiangxing => "장성",
        StarKey::Panan => "반안",
        StarKey::Suiyi => "세역",
        StarKey::Xiishen => "식신",
        StarKey::Huagai => "화개",
        StarKey::Jiesha => "겁살",
        StarKey::Zhaisha => "재살",
        StarKey::Tiansha => "천살",
        StarKey::Zhibei => "지배",
        StarKey::Xianchi => "함지",
        StarKey::Yuesha => "월살",
        StarKey::Wangshen => "망신",
        StarKey::Yunkui => "천괴(십년)",
        StarKey::Yunyue => "천월(십년)",
        StarKey::Yunchang => "문창(십년)",
        StarKey::Yunqu => "문곡(십년)",
        StarKey::Yunluan => "홍란(십년)",
        StarKey::Yunxi => "천희(십년)",
        StarKey::Yunlu => "록존(십년)",
        StarKey::Yunyang => "경양(십년)",
        StarKey::Yuntuo => "타라(십년)",
        StarKey::Yunma => "천마(십년)",
        StarKey::Liukui => "천괴(년)",
        StarKey::Liuyue => "천월(년)",
        StarKey::Liuchang => "문창(년)",
        StarKey::Liuqu => "문곡(년)",
        StarKey::Liuluan => "홍란(년)",
        StarKey::Liuxi => "천희(년)",
        StarKey::Liulu => "록존(년)",
        StarKey::Liuyang => "경양(년)",
        StarKey::Liutuo => "타라(년)",
        StarKey::Liuma => "천마(년)",
        StarKey::Nianjie => "해신(년)",
        StarKey::Yuekui => "천괴(월)",
        StarKey::Yueyue => "천월(월)",
        StarKey::Yuechang => "문창(월)",
        StarKey::Yuequ => "문곡(월)",
        StarKey::Yueluan => "홍란(월)",
        StarKey::Yuexi => "천희(월)",
        StarKey::Yuelu => "록존(월)",
        StarKey::Yueyang => "경양(월)",
        StarKey::Yuetuo => "타라(월)",
        StarKey::Yuema => "천마(월)",
        StarKey::Rikui => "천괴(일)",
        StarKey::Riyue => "천월(일)",
        StarKey::Richang => "문창(일)",
        StarKey::Riqu => "문곡(일)",
        StarKey::Riluan => "홍란(일)",
        StarKey::Rixi => "천희(일)",
        StarKey::Rilu => "록존(일)",
        StarKey::Riyang => "경양(일)",
        StarKey::Rituo => "타라(일)",
        StarKey::Rima => "천마(일)",
        StarKey::Shikui => "천괴(시)",
        StarKey::Shiyue => "천월(시)",
        StarKey::Shichang => "문창(시)",
        StarKey::Shiqu => "문곡(시)",
        StarKey::Shiluan => "홍란(시)",
        StarKey::Shixi => "천희(시)",
        StarKey::Shilu => "록존(시)",
        StarKey::Shiyang => "경양(시)",
        StarKey::Shituo => "타라(시)",
        StarKey::Shima => "천마(시)",
    }
}

pub fn palace_name(palace: Palace) -> &'static str {
    match palace {
        Palace::Soul => "명궁",
        Palace::Parents => "부모",
        Palace::Spirit => "복덕",
        Palace::Property => "전택",
        Palace::Career => "관록",
        Palace::Friends => "노복",
        Palace::Surface => "천이",
        Palace::Health => "질액",
        Palace::Wealth => "재백",
        Palace::Children => "자녀",
        Palace::Spouse => "부처",
        Palace::Siblings => "형제",
    }
}

pub fn heavenly_stem_name(stem: HeavenlyStem) -> &'static str {
    match stem {
        HeavenlyStem::Jia => "갑",
        HeavenlyStem::Yi => "을",
        HeavenlyStem::Bing => "병",
        HeavenlyStem::Ding => "정",
        HeavenlyStem::Wu => "무",
        HeavenlyStem::Ji => "기",
        HeavenlyStem::Geng => "경",
        HeavenlyStem::Xin => "신",
        HeavenlyStem::Ren => "임",
        HeavenlyStem::Gui => "계",
    }
}

pub fn earthly_branch_name(branch: EarthlyBranch) -> &'static str {
    match branch {
        EarthlyBranch::Zi => "자",
        EarthlyBranch::Chou => "축",
        EarthlyBranch::Yin => "인",
        EarthlyBranch::Mao => "묘",
        EarthlyBranch::Chen => "진",
        EarthlyBranch::Si => "사",
        EarthlyBranch::Wu => "오",
        EarthlyBranch::Wei => "미",
        EarthlyBranch::Shen => "신",
        EarthlyBranch::You => "유",
        EarthlyBranch::Xu => "술",
        EarthlyBranch::Hai => "해",
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
        Mutagen::Lu => "록",
        Mutagen::Quan => "권",
        Mutagen::Ke => "과",
        Mutagen::Ji => "기",
    }
}

pub fn five_elements_class_name(c: FiveElementsClass) -> &'static str {
    match c {
        FiveElementsClass::Water2nd => "수이국",
        FiveElementsClass::Wood3rd => "목삼국",
        FiveElementsClass::Metal4th => "금사국",
        FiveElementsClass::Earth5th => "토오국",
        FiveElementsClass::Fire6th => "화육국",
    }
}

pub fn gender_name(g: Gender) -> &'static str {
    match g {
        Gender::Male => "남성",
        Gender::Female => "여자",
    }
}

/// 시진 이름표（색인 0-12: 아침 자시…밤에 자시）
const TIME_NAMES: [&str; 13] = [
    "아침 자시", "축시", "인시", "묘시", "진시", "사시", "오시",
    "미시", "신시", "유시", "술시", "해시", "밤에 자시",
];

/// 별자리 이름표（색인 0-11: 백양궁부터 황도 순서）
const SIGN_NAMES: [&str; 12] = [
    "백양궁", "금우궁", "쌍아궁", "거해궁", "사자궁", "처녀궁",
    "천칭궁", "천갈궁", "인마궁", "마갈궁", "보병궁", "쌍어궁",
];

/// 띠 동물 이름표（지지 색인 순: 자=쥐…해=돼지）
const ZODIAC_NAMES: [&str; 12] = [
    "쥐", "소", "호랑이", "토끼", "용", "뱀",
    "말", "양", "원숭이", "닭", "개", "돼지",
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
        HoroscopeName::Decadal => "대한",
        HoroscopeName::Childhood => "어린",
        HoroscopeName::Age => "소한",
        HoroscopeName::Yearly => "유년",
        HoroscopeName::Monthly => "유월",
        HoroscopeName::Daily => "유일",
        HoroscopeName::Hourly => "유시",
    }
}

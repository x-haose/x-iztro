use super::types::{Brightness, FiveElements, Mutagen, YinYang};

/// 星耀键值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StarKey {
    // ========== 14 主星 ==========
    /// 紫微
    ZiweiMaj,
    /// 天机
    TianjiMaj,
    /// 太阳
    TaiyangMaj,
    /// 武曲
    WuquMaj,
    /// 天同
    TiantongMaj,
    /// 廉贞
    LianzhenMaj,
    /// 天府
    TianfuMaj,
    /// 太阴
    TaiyinMaj,
    /// 贪狼
    TanlangMaj,
    /// 巨门
    JumenMaj,
    /// 天相
    TianxiangMaj,
    /// 天梁
    TianliangMaj,
    /// 七杀
    QishaMaj,
    /// 破军
    PojunMaj,

    // ========== 14 辅星 ==========
    /// 左辅
    ZuofuMin,
    /// 右弼
    YoubiMin,
    /// 文昌
    WenchangMin,
    /// 文曲
    WenquMin,
    /// 禄存
    LucunMin,
    /// 天马
    TianmaMin,
    /// 擎羊
    QingyangMin,
    /// 陀罗
    TuoluoMin,
    /// 火星
    HuoxingMin,
    /// 铃星
    LingxingMin,
    /// 天魁
    TiankuiMin,
    /// 天钺
    TianyueMin,
    /// 地空
    DikongMin,
    /// 地劫
    DijieMin,

    // ========== 杂耀 ==========
    /// 劫煞
    JieshaAdj,
    /// 天空
    Tiankong,
    /// 天刑
    Tianxing,
    /// 天姚
    Tianyao,
    /// 解神
    Jieshen,
    /// 阴煞
    Yinsha,
    /// 天喜
    Tianxi,
    /// 天官
    Tianguan,
    /// 天福
    Tianfu,
    /// 天哭
    Tianku,
    /// 天虚
    Tianxu,
    /// 龙池
    Longchi,
    /// 凤阁
    Fengge,
    /// 红鸾
    Hongluan,
    /// 孤辰
    Guchen,
    /// 寡宿
    Guasu,
    /// 飞廉
    Feilian,
    /// 破碎
    Posui,
    /// 台辅
    Taifu,
    /// 封诰
    Fenggao,
    /// 天巫
    Tianwu,
    /// 天月
    Tianyue2,
    /// 三台
    Santai,
    /// 八座
    Bazuo,
    /// 恩光
    Engguang,
    /// 天贵
    Tiangui,
    /// 天才
    Tiancai,
    /// 天寿
    Tianshou,
    /// 截空
    Jiekong,
    /// 旬中
    Xunzhong,
    /// 旬空
    Xunkong,
    /// 空亡
    Kongwang,
    /// 截路
    Jielu,
    /// 傍截空：截空两宫中的傍空一宫，见 [`crate::data::types::VoidPlacement::ZhengPang`]
    PangJiekong,
    /// 傍旬空：旬空两宫中的傍空一宫，见 [`crate::data::types::VoidPlacement::ZhengPang`]
    PangXunkong,
    /// 月天马：按生月安的天马（书中称月马），天马兼安年马、月马时作杂耀安放
    /// （[`crate::data::types::TianmaPlacement::Both`]）。中文译名避开流月天马 [`StarKey::Yuema`] 的「月马」，
    /// 按译名反查标识时两者各回各的。
    YuemaAdj,
    /// 月德
    Yuede,
    /// 天伤
    Tianshang,
    /// 天使
    Tianshi,
    /// 天厨
    Tianchu,

    // ========== 长生12神 ==========
    /// 长生
    Changsheng,
    /// 沐浴
    Muyu,
    /// 冠带
    Guandai,
    /// 临官
    Linguan,
    /// 帝旺
    Diwang,
    /// 衰
    Shuai,
    /// 病
    Bing,
    /// 死
    Si,
    /// 墓
    Mu,
    /// 绝
    Jue,
    /// 胎
    Tai,
    /// 养
    Yang,

    // ========== 博士12神 ==========
    /// 博士
    Boshi,
    /// 力士
    Lishi,
    /// 青龙
    Qinglong,
    /// 小耗
    Xiaohao,
    /// 将军
    Jiangjun,
    /// 奏书
    Zhoushu,
    /// 飞廉(博士)
    Faylian,
    /// 喜神
    Xishen,
    /// 病符
    Bingfu,
    /// 大耗
    Dahao,
    /// 岁破
    Suipo,
    /// 伏兵
    Fubing,
    /// 官府
    Guanfu,

    // ========== 岁前12神 ==========
    /// 岁建
    Suijian,
    /// 晦气
    Huiqi,
    /// 丧门
    Sangmen,
    /// 贯索
    Guansuo,
    /// 官符
    Gwanfu,
    /// 龙德
    Longde,
    /// 白虎
    Baihu,
    /// 天德
    Tiande,
    /// 吊客
    Diaoke,

    // ========== 将前12神 ==========
    /// 将星
    Jiangxing,
    /// 攀鞍
    Panan,
    /// 岁驿
    Suiyi,
    /// 息神
    Xiishen,
    /// 华盖
    Huagai,
    /// 劫煞(将前)
    Jiesha,
    /// 灾煞
    Zhaisha,
    /// 天煞
    Tiansha,
    /// 指背
    Zhibei,
    /// 咸池
    Xianchi,
    /// 月煞
    Yuesha,
    /// 亡神
    Wangshen,

    // ========== 运限流耀 ==========
    /// 运魁
    Yunkui,
    /// 运钺
    Yunyue,
    /// 运昌
    Yunchang,
    /// 运曲
    Yunqu,
    /// 运鸾
    Yunluan,
    /// 运喜
    Yunxi,
    /// 运禄
    Yunlu,
    /// 运羊
    Yunyang,
    /// 运陀
    Yuntuo,
    /// 运马
    Yunma,

    /// 流魁
    Liukui,
    /// 流钺
    Liuyue,
    /// 流昌
    Liuchang,
    /// 流曲
    Liuqu,
    /// 流鸾
    Liuluan,
    /// 流喜
    Liuxi,
    /// 流禄
    Liulu,
    /// 流羊
    Liuyang,
    /// 流陀
    Liutuo,
    /// 流马
    Liuma,

    /// 年解
    Nianjie,

    /// 月魁
    Yuekui,
    /// 月钺
    Yueyue,
    /// 月昌
    Yuechang,
    /// 月曲
    Yuequ,
    /// 月鸾
    Yueluan,
    /// 月喜
    Yuexi,
    /// 月禄
    Yuelu,
    /// 月羊
    Yueyang,
    /// 月陀
    Yuetuo,
    /// 月马
    Yuema,

    /// 日魁
    Rikui,
    /// 日钺
    Riyue,
    /// 日昌
    Richang,
    /// 日曲
    Riqu,
    /// 日鸾
    Riluan,
    /// 日喜
    Rixi,
    /// 日禄
    Rilu,
    /// 日羊
    Riyang,
    /// 日陀
    Rituo,
    /// 日马
    Rima,

    /// 时魁
    Shikui,
    /// 时钺
    Shiyue,
    /// 时昌
    Shichang,
    /// 时曲
    Shiqu,
    /// 时鸾
    Shiluan,
    /// 时喜
    Shixi,
    /// 时禄
    Shilu,
    /// 时羊
    Shiyang,
    /// 时陀
    Shituo,
    /// 时马
    Shima,
}

crate::data::keyed::keyed_enum!(StarKey, "star" {
    ZiweiMaj => "ziweiMaj",
    TianjiMaj => "tianjiMaj",
    TaiyangMaj => "taiyangMaj",
    WuquMaj => "wuquMaj",
    TiantongMaj => "tiantongMaj",
    LianzhenMaj => "lianzhenMaj",
    TianfuMaj => "tianfuMaj",
    TaiyinMaj => "taiyinMaj",
    TanlangMaj => "tanlangMaj",
    JumenMaj => "jumenMaj",
    TianxiangMaj => "tianxiangMaj",
    TianliangMaj => "tianliangMaj",
    QishaMaj => "qishaMaj",
    PojunMaj => "pojunMaj",
    ZuofuMin => "zuofuMin",
    YoubiMin => "youbiMin",
    WenchangMin => "wenchangMin",
    WenquMin => "wenquMin",
    LucunMin => "lucunMin",
    TianmaMin => "tianmaMin",
    QingyangMin => "qingyangMin",
    TuoluoMin => "tuoluoMin",
    HuoxingMin => "huoxingMin",
    LingxingMin => "lingxingMin",
    TiankuiMin => "tiankuiMin",
    TianyueMin => "tianyueMin",
    DikongMin => "dikongMin",
    DijieMin => "dijieMin",
    JieshaAdj => "jieshaAdj",
    Tiankong => "tiankong",
    Tianxing => "tianxing",
    Tianyao => "tianyao",
    Jieshen => "jieshen",
    Yinsha => "yinsha",
    Tianxi => "tianxi",
    Tianguan => "tianguan",
    Tianfu => "tianfu",
    Tianku => "tianku",
    Tianxu => "tianxu",
    Longchi => "longchi",
    Fengge => "fengge",
    Hongluan => "hongluan",
    Guchen => "guchen",
    Guasu => "guasu",
    Feilian => "feilian",
    Posui => "posui",
    Taifu => "taifu",
    Fenggao => "fenggao",
    Tianwu => "tianwu",
    Tianyue2 => "tianyue",
    Santai => "santai",
    Bazuo => "bazuo",
    Engguang => "engguang",
    Tiangui => "tiangui",
    Tiancai => "tiancai",
    Tianshou => "tianshou",
    Jiekong => "jiekong",
    Xunzhong => "xunzhong",
    Xunkong => "xunkong",
    Kongwang => "kongwang",
    Jielu => "jielu",
    PangJiekong => "pangJiekong",
    PangXunkong => "pangXunkong",
    YuemaAdj => "yuemaAdj",
    Yuede => "yuede",
    Tianshang => "tianshang",
    Tianshi => "tianshi",
    Tianchu => "tianchu",
    Changsheng => "changsheng",
    Muyu => "muyu",
    Guandai => "guandai",
    Linguan => "linguan",
    Diwang => "diwang",
    Shuai => "shuai",
    Bing => "bing",
    Si => "si",
    Mu => "mu",
    Jue => "jue",
    Tai => "tai",
    Yang => "yang",
    Boshi => "boshi",
    Lishi => "lishi",
    Qinglong => "qinglong",
    Xiaohao => "xiaohao",
    Jiangjun => "jiangjun",
    Zhoushu => "zhoushu",
    Faylian => "faylian",
    Xishen => "xishen",
    Bingfu => "bingfu",
    Dahao => "dahao",
    Suipo => "suipo",
    Fubing => "fubing",
    Guanfu => "guanfu",
    Suijian => "suijian",
    Huiqi => "huiqi",
    Sangmen => "sangmen",
    Guansuo => "guansuo",
    Gwanfu => "gwanfu",
    Longde => "longde",
    Baihu => "baihu",
    Tiande => "tiande",
    Diaoke => "diaoke",
    Jiangxing => "jiangxing",
    Panan => "panan",
    Suiyi => "suiyi",
    Xiishen => "xiishen",
    Huagai => "huagai",
    Jiesha => "jiesha",
    Zhaisha => "zhaisha",
    Tiansha => "tiansha",
    Zhibei => "zhibei",
    Xianchi => "xianchi",
    Yuesha => "yuesha",
    Wangshen => "wangshen",
    Yunkui => "yunkui",
    Yunyue => "yunyue",
    Yunchang => "yunchang",
    Yunqu => "yunqu",
    Yunluan => "yunluan",
    Yunxi => "yunxi",
    Yunlu => "yunlu",
    Yunyang => "yunyang",
    Yuntuo => "yuntuo",
    Yunma => "yunma",
    Liukui => "liukui",
    Liuyue => "liuyue",
    Liuchang => "liuchang",
    Liuqu => "liuqu",
    Liuluan => "liuluan",
    Liuxi => "liuxi",
    Liulu => "liulu",
    Liuyang => "liuyang",
    Liutuo => "liutuo",
    Liuma => "liuma",
    Nianjie => "nianjie",
    Yuekui => "yuekui",
    Yueyue => "yueyue",
    Yuechang => "yuechang",
    Yuequ => "yuequ",
    Yueluan => "yueluan",
    Yuexi => "yuexi",
    Yuelu => "yuelu",
    Yueyang => "yueyang",
    Yuetuo => "yuetuo",
    Yuema => "yuema",
    Rikui => "rikui",
    Riyue => "riyue",
    Richang => "richang",
    Riqu => "riqu",
    Riluan => "riluan",
    Rixi => "rixi",
    Rilu => "rilu",
    Riyang => "riyang",
    Rituo => "rituo",
    Rima => "rima",
    Shikui => "shikui",
    Shiyue => "shiyue",
    Shichang => "shichang",
    Shiqu => "shiqu",
    Shiluan => "shiluan",
    Shixi => "shixi",
    Shilu => "shilu",
    Shiyang => "shiyang",
    Shituo => "shituo",
    Shima => "shima",
});

/// 四化的固定顺序：禄、权、科、忌
///
/// 天干四化表按这个顺序存放，据此可把「第几位」与四化名互转。
pub const MUTAGEN: [Mutagen; 4] = [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji];

/// 全部星耀标识，顺序与枚举声明一致
///
/// 供译名反查标识（遍历各类目找出属于星耀的那一个）与数据表导出使用。
pub const ALL_STARS: [StarKey; 165] = [
    StarKey::ZiweiMaj,
    StarKey::TianjiMaj,
    StarKey::TaiyangMaj,
    StarKey::WuquMaj,
    StarKey::TiantongMaj,
    StarKey::LianzhenMaj,
    StarKey::TianfuMaj,
    StarKey::TaiyinMaj,
    StarKey::TanlangMaj,
    StarKey::JumenMaj,
    StarKey::TianxiangMaj,
    StarKey::TianliangMaj,
    StarKey::QishaMaj,
    StarKey::PojunMaj,
    StarKey::ZuofuMin,
    StarKey::YoubiMin,
    StarKey::WenchangMin,
    StarKey::WenquMin,
    StarKey::LucunMin,
    StarKey::TianmaMin,
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
    StarKey::HuoxingMin,
    StarKey::LingxingMin,
    StarKey::TiankuiMin,
    StarKey::TianyueMin,
    StarKey::DikongMin,
    StarKey::DijieMin,
    StarKey::JieshaAdj,
    StarKey::Tiankong,
    StarKey::Tianxing,
    StarKey::Tianyao,
    StarKey::Jieshen,
    StarKey::Yinsha,
    StarKey::Tianxi,
    StarKey::Tianguan,
    StarKey::Tianfu,
    StarKey::Tianku,
    StarKey::Tianxu,
    StarKey::Longchi,
    StarKey::Fengge,
    StarKey::Hongluan,
    StarKey::Guchen,
    StarKey::Guasu,
    StarKey::Feilian,
    StarKey::Posui,
    StarKey::Taifu,
    StarKey::Fenggao,
    StarKey::Tianwu,
    StarKey::Tianyue2,
    StarKey::Santai,
    StarKey::Bazuo,
    StarKey::Engguang,
    StarKey::Tiangui,
    StarKey::Tiancai,
    StarKey::Tianshou,
    StarKey::Jiekong,
    StarKey::Xunzhong,
    StarKey::Xunkong,
    StarKey::Kongwang,
    StarKey::Jielu,
    StarKey::PangJiekong,
    StarKey::PangXunkong,
    StarKey::YuemaAdj,
    StarKey::Yuede,
    StarKey::Tianshang,
    StarKey::Tianshi,
    StarKey::Tianchu,
    StarKey::Changsheng,
    StarKey::Muyu,
    StarKey::Guandai,
    StarKey::Linguan,
    StarKey::Diwang,
    StarKey::Shuai,
    StarKey::Bing,
    StarKey::Si,
    StarKey::Mu,
    StarKey::Jue,
    StarKey::Tai,
    StarKey::Yang,
    StarKey::Boshi,
    StarKey::Lishi,
    StarKey::Qinglong,
    StarKey::Xiaohao,
    StarKey::Jiangjun,
    StarKey::Zhoushu,
    StarKey::Faylian,
    StarKey::Xishen,
    StarKey::Bingfu,
    StarKey::Dahao,
    StarKey::Suipo,
    StarKey::Fubing,
    StarKey::Guanfu,
    StarKey::Suijian,
    StarKey::Huiqi,
    StarKey::Sangmen,
    StarKey::Guansuo,
    StarKey::Gwanfu,
    StarKey::Longde,
    StarKey::Baihu,
    StarKey::Tiande,
    StarKey::Diaoke,
    StarKey::Jiangxing,
    StarKey::Panan,
    StarKey::Suiyi,
    StarKey::Xiishen,
    StarKey::Huagai,
    StarKey::Jiesha,
    StarKey::Zhaisha,
    StarKey::Tiansha,
    StarKey::Zhibei,
    StarKey::Xianchi,
    StarKey::Yuesha,
    StarKey::Wangshen,
    StarKey::Yunkui,
    StarKey::Yunyue,
    StarKey::Yunchang,
    StarKey::Yunqu,
    StarKey::Yunluan,
    StarKey::Yunxi,
    StarKey::Yunlu,
    StarKey::Yunyang,
    StarKey::Yuntuo,
    StarKey::Yunma,
    StarKey::Liukui,
    StarKey::Liuyue,
    StarKey::Liuchang,
    StarKey::Liuqu,
    StarKey::Liuluan,
    StarKey::Liuxi,
    StarKey::Liulu,
    StarKey::Liuyang,
    StarKey::Liutuo,
    StarKey::Liuma,
    StarKey::Nianjie,
    StarKey::Yuekui,
    StarKey::Yueyue,
    StarKey::Yuechang,
    StarKey::Yuequ,
    StarKey::Yueluan,
    StarKey::Yuexi,
    StarKey::Yuelu,
    StarKey::Yueyang,
    StarKey::Yuetuo,
    StarKey::Yuema,
    StarKey::Rikui,
    StarKey::Riyue,
    StarKey::Richang,
    StarKey::Riqu,
    StarKey::Riluan,
    StarKey::Rixi,
    StarKey::Rilu,
    StarKey::Riyang,
    StarKey::Rituo,
    StarKey::Rima,
    StarKey::Shikui,
    StarKey::Shiyue,
    StarKey::Shichang,
    StarKey::Shiqu,
    StarKey::Shiluan,
    StarKey::Shixi,
    StarKey::Shilu,
    StarKey::Shiyang,
    StarKey::Shituo,
    StarKey::Shima,
];

/// 有 [`StarInfo`] 记录的星耀：14 主星与文昌、文曲、火星、铃星、擎羊、陀罗
pub const STARS_WITH_INFO: [StarKey; 20] = [
    StarKey::ZiweiMaj,
    StarKey::TianjiMaj,
    StarKey::TaiyangMaj,
    StarKey::WuquMaj,
    StarKey::TiantongMaj,
    StarKey::LianzhenMaj,
    StarKey::TianfuMaj,
    StarKey::TaiyinMaj,
    StarKey::TanlangMaj,
    StarKey::JumenMaj,
    StarKey::TianxiangMaj,
    StarKey::TianliangMaj,
    StarKey::QishaMaj,
    StarKey::PojunMaj,
    StarKey::WenchangMin,
    StarKey::WenquMin,
    StarKey::HuoxingMin,
    StarKey::LingxingMin,
    StarKey::QingyangMin,
    StarKey::TuoluoMin,
];

/// 星耀基础信息
///
/// 对应 iztro `data.STARS_INFO` 的一条记录：14 主星与文昌、文曲、火星、
/// 铃星、擎羊、陀罗六颗辅星共 20 颗有此记录，其余星耀没有。
pub struct StarInfo {
    /// 十二宫亮度，索引 0=寅, 1=卯, ..., 11=丑；该宫无亮度则为 `None`
    pub brightness: [Option<Brightness>; 12],
    /// 五行；太阳、七杀与六颗辅星在表中未填，为 `None`
    pub five_elements: Option<FiveElements>,
    /// 阴阳；太阳、贪狼、天相、天梁、七杀、破军与六颗辅星在表中未填，为 `None`
    pub yin_yang: Option<YinYang>,
}

/// 获取星耀基础信息
///
/// 无记录的星耀返回 `None`。
pub fn get_star_info(key: StarKey) -> Option<StarInfo> {
    let (five_elements, yin_yang) = star_five_elements_and_yin_yang(key);

    Some(StarInfo {
        brightness: get_brightness_table(key)?,
        five_elements,
        yin_yang,
    })
}

/// 星耀的五行与阴阳
///
/// 与亮度表分列：亮度是十二格的表，五行阴阳是单值，合并会把单值埋进长表里。
/// 两者共同构成 [`StarInfo`]。
fn star_five_elements_and_yin_yang(key: StarKey) -> (Option<FiveElements>, Option<YinYang>) {
    use FiveElements::*;
    use YinYang::*;

    match key {
        StarKey::ZiweiMaj => (Some(Earth), Some(Yin)),
        StarKey::TianjiMaj => (Some(Wood), Some(Yin)),
        StarKey::WuquMaj => (Some(Metal), Some(Yin)),
        StarKey::TiantongMaj => (Some(Water), Some(Yang)),
        StarKey::LianzhenMaj => (Some(Fire), Some(Yin)),
        StarKey::TianfuMaj => (Some(Earth), Some(Yang)),
        StarKey::TaiyinMaj => (Some(Water), Some(Yin)),
        StarKey::TanlangMaj => (Some(Water), None),
        StarKey::JumenMaj => (Some(Earth), Some(Yin)),
        StarKey::TianxiangMaj => (Some(Water), None),
        StarKey::TianliangMaj => (Some(Earth), None),
        StarKey::PojunMaj => (Some(Water), None),
        _ => (None, None),
    }
}

/// 获取星耀亮度表
///
/// 返回长度为12的数组，索引 0=寅, 1=卯, ..., 11=丑。
/// 若该星耀没有亮度表则返回 `None`。
pub fn get_brightness_table(key: StarKey) -> Option<[Option<Brightness>; 12]> {
    use Brightness::*;

    let s = |b: Brightness| Some(b);
    let n: Option<Brightness> = None;

    match key {
        StarKey::ZiweiMaj => Some([
            s(Wang),
            s(Wang),
            s(De),
            s(Wang),
            s(Miao),
            s(Miao),
            s(Wang),
            s(Wang),
            s(De),
            s(Wang),
            s(Ping),
            s(Miao),
        ]),
        StarKey::TianjiMaj => Some([
            s(De),
            s(Wang),
            s(Li),
            s(Ping),
            s(Miao),
            s(Xian),
            s(De),
            s(Wang),
            s(Li),
            s(Ping),
            s(Miao),
            s(Xian),
        ]),
        StarKey::TaiyangMaj => Some([
            s(Wang),
            s(Miao),
            s(Wang),
            s(Wang),
            s(Wang),
            s(De),
            s(De),
            s(Ping),
            s(Bu),
            s(Xian),
            s(Xian),
            s(Bu),
        ]),
        StarKey::WuquMaj => Some([
            s(De),
            s(Li),
            s(Miao),
            s(Ping),
            s(Wang),
            s(Miao),
            s(De),
            s(Li),
            s(Miao),
            s(Ping),
            s(Wang),
            s(Miao),
        ]),
        StarKey::TiantongMaj => Some([
            s(Li),
            s(Ping),
            s(Ping),
            s(Miao),
            s(Xian),
            s(Bu),
            s(Wang),
            s(Ping),
            s(Ping),
            s(Miao),
            s(Wang),
            s(Bu),
        ]),
        StarKey::LianzhenMaj => Some([
            s(Miao),
            s(Ping),
            s(Li),
            s(Xian),
            s(Ping),
            s(Li),
            s(Miao),
            s(Ping),
            s(Li),
            s(Xian),
            s(Ping),
            s(Li),
        ]),
        StarKey::TianfuMaj => Some([
            s(Miao),
            s(De),
            s(Miao),
            s(De),
            s(Wang),
            s(Miao),
            s(De),
            s(Wang),
            s(Miao),
            s(De),
            s(Miao),
            s(Miao),
        ]),
        StarKey::TaiyinMaj => Some([
            s(Wang),
            s(Xian),
            s(Xian),
            s(Xian),
            s(Bu),
            s(Bu),
            s(Li),
            s(Wang),
            s(Wang),
            s(Miao),
            s(Miao),
            s(Miao),
        ]),
        StarKey::TanlangMaj => Some([
            s(Ping),
            s(Li),
            s(Miao),
            s(Xian),
            s(Wang),
            s(Miao),
            s(Ping),
            s(Li),
            s(Miao),
            s(Xian),
            s(Wang),
            s(Miao),
        ]),
        StarKey::JumenMaj => Some([
            s(Miao),
            s(Miao),
            s(Xian),
            s(Wang),
            s(Wang),
            s(Bu),
            s(Miao),
            s(Miao),
            s(Xian),
            s(Wang),
            s(Wang),
            s(Bu),
        ]),
        StarKey::TianxiangMaj => Some([
            s(Miao),
            s(Xian),
            s(De),
            s(De),
            s(Miao),
            s(De),
            s(Miao),
            s(Xian),
            s(De),
            s(De),
            s(Miao),
            s(Miao),
        ]),
        StarKey::TianliangMaj => Some([
            s(Miao),
            s(Miao),
            s(Miao),
            s(Xian),
            s(Miao),
            s(Wang),
            s(Xian),
            s(De),
            s(Miao),
            s(Xian),
            s(Miao),
            s(Wang),
        ]),
        StarKey::QishaMaj => Some([
            s(Miao),
            s(Wang),
            s(Miao),
            s(Ping),
            s(Wang),
            s(Miao),
            s(Miao),
            s(Wang),
            s(Miao),
            s(Ping),
            s(Wang),
            s(Miao),
        ]),
        StarKey::PojunMaj => Some([
            s(De),
            s(Xian),
            s(Wang),
            s(Ping),
            s(Miao),
            s(Wang),
            s(De),
            s(Xian),
            s(Wang),
            s(Ping),
            s(Miao),
            s(Wang),
        ]),
        StarKey::WenchangMin => Some([
            s(Xian),
            s(Li),
            s(De),
            s(Miao),
            s(Xian),
            s(Li),
            s(De),
            s(Miao),
            s(Xian),
            s(Li),
            s(De),
            s(Miao),
        ]),
        StarKey::WenquMin => Some([
            s(Ping),
            s(Wang),
            s(De),
            s(Miao),
            s(Xian),
            s(Wang),
            s(De),
            s(Miao),
            s(Xian),
            s(Wang),
            s(De),
            s(Miao),
        ]),
        StarKey::HuoxingMin => Some([
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
        ]),
        StarKey::LingxingMin => Some([
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
            s(Miao),
            s(Li),
            s(Xian),
            s(De),
        ]),
        StarKey::QingyangMin => Some([
            n,
            s(Xian),
            s(Miao),
            n,
            s(Xian),
            s(Miao),
            n,
            s(Xian),
            s(Miao),
            n,
            s(Xian),
            s(Miao),
        ]),
        StarKey::TuoluoMin => Some([
            s(Xian),
            n,
            s(Miao),
            s(Xian),
            n,
            s(Miao),
            s(Xian),
            n,
            s(Miao),
            s(Xian),
            n,
            s(Miao),
        ]),
        _ => None,
    }
}

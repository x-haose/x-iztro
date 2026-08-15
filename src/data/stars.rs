use serde::{Deserialize, Serialize};

use super::types::{Brightness, FiveElements, Mutagen, YinYang};

/// 星耀键值
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

/// 四化的固定顺序：禄、权、科、忌
///
/// 天干四化表按这个顺序存放，据此可把「第几位」与四化名互转。
pub const MUTAGEN: [Mutagen; 4] = [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji];

/// 全部星耀标识，顺序与枚举声明一致
///
/// 供译名反查标识（遍历各类目找出属于星耀的那一个）与数据表导出使用。
pub const ALL_STARS: [StarKey; 162] = [
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
            s(Xian),
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
            s(Bu),
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
            s(Miao),
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

impl StarKey {
    /// 语言无关的星耀标识（iztro i18n key，如 "ziweiMaj"），
    /// 供跨语言绑定做身份判断。
    pub fn as_key(&self) -> &'static str {
        match self {
            StarKey::ZiweiMaj => "ziweiMaj",
            StarKey::TianjiMaj => "tianjiMaj",
            StarKey::TaiyangMaj => "taiyangMaj",
            StarKey::WuquMaj => "wuquMaj",
            StarKey::TiantongMaj => "tiantongMaj",
            StarKey::LianzhenMaj => "lianzhenMaj",
            StarKey::TianfuMaj => "tianfuMaj",
            StarKey::TaiyinMaj => "taiyinMaj",
            StarKey::TanlangMaj => "tanlangMaj",
            StarKey::JumenMaj => "jumenMaj",
            StarKey::TianxiangMaj => "tianxiangMaj",
            StarKey::TianliangMaj => "tianliangMaj",
            StarKey::QishaMaj => "qishaMaj",
            StarKey::PojunMaj => "pojunMaj",
            StarKey::ZuofuMin => "zuofuMin",
            StarKey::YoubiMin => "youbiMin",
            StarKey::WenchangMin => "wenchangMin",
            StarKey::WenquMin => "wenquMin",
            StarKey::LucunMin => "lucunMin",
            StarKey::TianmaMin => "tianmaMin",
            StarKey::QingyangMin => "qingyangMin",
            StarKey::TuoluoMin => "tuoluoMin",
            StarKey::HuoxingMin => "huoxingMin",
            StarKey::LingxingMin => "lingxingMin",
            StarKey::TiankuiMin => "tiankuiMin",
            StarKey::TianyueMin => "tianyueMin",
            StarKey::DikongMin => "dikongMin",
            StarKey::DijieMin => "dijieMin",
            StarKey::JieshaAdj => "jieshaAdj",
            StarKey::Tiankong => "tiankong",
            StarKey::Tianxing => "tianxing",
            StarKey::Tianyao => "tianyao",
            StarKey::Jieshen => "jieshen",
            StarKey::Yinsha => "yinsha",
            StarKey::Tianxi => "tianxi",
            StarKey::Tianguan => "tianguan",
            StarKey::Tianfu => "tianfu",
            StarKey::Tianku => "tianku",
            StarKey::Tianxu => "tianxu",
            StarKey::Longchi => "longchi",
            StarKey::Fengge => "fengge",
            StarKey::Hongluan => "hongluan",
            StarKey::Guchen => "guchen",
            StarKey::Guasu => "guasu",
            StarKey::Feilian => "feilian",
            StarKey::Posui => "posui",
            StarKey::Taifu => "taifu",
            StarKey::Fenggao => "fenggao",
            StarKey::Tianwu => "tianwu",
            StarKey::Tianyue2 => "tianyue",
            StarKey::Santai => "santai",
            StarKey::Bazuo => "bazuo",
            StarKey::Engguang => "engguang",
            StarKey::Tiangui => "tiangui",
            StarKey::Tiancai => "tiancai",
            StarKey::Tianshou => "tianshou",
            StarKey::Jiekong => "jiekong",
            StarKey::Xunzhong => "xunzhong",
            StarKey::Xunkong => "xunkong",
            StarKey::Kongwang => "kongwang",
            StarKey::Jielu => "jielu",
            StarKey::Yuede => "yuede",
            StarKey::Tianshang => "tianshang",
            StarKey::Tianshi => "tianshi",
            StarKey::Tianchu => "tianchu",
            StarKey::Changsheng => "changsheng",
            StarKey::Muyu => "muyu",
            StarKey::Guandai => "guandai",
            StarKey::Linguan => "linguan",
            StarKey::Diwang => "diwang",
            StarKey::Shuai => "shuai",
            StarKey::Bing => "bing",
            StarKey::Si => "si",
            StarKey::Mu => "mu",
            StarKey::Jue => "jue",
            StarKey::Tai => "tai",
            StarKey::Yang => "yang",
            StarKey::Boshi => "boshi",
            StarKey::Lishi => "lishi",
            StarKey::Qinglong => "qinglong",
            StarKey::Xiaohao => "xiaohao",
            StarKey::Jiangjun => "jiangjun",
            StarKey::Zhoushu => "zhoushu",
            StarKey::Faylian => "faylian",
            StarKey::Xishen => "xishen",
            StarKey::Bingfu => "bingfu",
            StarKey::Dahao => "dahao",
            StarKey::Suipo => "suipo",
            StarKey::Fubing => "fubing",
            StarKey::Guanfu => "guanfu",
            StarKey::Suijian => "suijian",
            StarKey::Huiqi => "huiqi",
            StarKey::Sangmen => "sangmen",
            StarKey::Guansuo => "guansuo",
            StarKey::Gwanfu => "gwanfu",
            StarKey::Longde => "longde",
            StarKey::Baihu => "baihu",
            StarKey::Tiande => "tiande",
            StarKey::Diaoke => "diaoke",
            StarKey::Jiangxing => "jiangxing",
            StarKey::Panan => "panan",
            StarKey::Suiyi => "suiyi",
            StarKey::Xiishen => "xiishen",
            StarKey::Huagai => "huagai",
            StarKey::Jiesha => "jiesha",
            StarKey::Zhaisha => "zhaisha",
            StarKey::Tiansha => "tiansha",
            StarKey::Zhibei => "zhibei",
            StarKey::Xianchi => "xianchi",
            StarKey::Yuesha => "yuesha",
            StarKey::Wangshen => "wangshen",
            StarKey::Yunkui => "yunkui",
            StarKey::Yunyue => "yunyue",
            StarKey::Yunchang => "yunchang",
            StarKey::Yunqu => "yunqu",
            StarKey::Yunluan => "yunluan",
            StarKey::Yunxi => "yunxi",
            StarKey::Yunlu => "yunlu",
            StarKey::Yunyang => "yunyang",
            StarKey::Yuntuo => "yuntuo",
            StarKey::Yunma => "yunma",
            StarKey::Liukui => "liukui",
            StarKey::Liuyue => "liuyue",
            StarKey::Liuchang => "liuchang",
            StarKey::Liuqu => "liuqu",
            StarKey::Liuluan => "liuluan",
            StarKey::Liuxi => "liuxi",
            StarKey::Liulu => "liulu",
            StarKey::Liuyang => "liuyang",
            StarKey::Liutuo => "liutuo",
            StarKey::Liuma => "liuma",
            StarKey::Nianjie => "nianjie",
            StarKey::Yuekui => "yuekui",
            StarKey::Yueyue => "yueyue",
            StarKey::Yuechang => "yuechang",
            StarKey::Yuequ => "yuequ",
            StarKey::Yueluan => "yueluan",
            StarKey::Yuexi => "yuexi",
            StarKey::Yuelu => "yuelu",
            StarKey::Yueyang => "yueyang",
            StarKey::Yuetuo => "yuetuo",
            StarKey::Yuema => "yuema",
            StarKey::Rikui => "rikui",
            StarKey::Riyue => "riyue",
            StarKey::Richang => "richang",
            StarKey::Riqu => "riqu",
            StarKey::Riluan => "riluan",
            StarKey::Rixi => "rixi",
            StarKey::Rilu => "rilu",
            StarKey::Riyang => "riyang",
            StarKey::Rituo => "rituo",
            StarKey::Rima => "rima",
            StarKey::Shikui => "shikui",
            StarKey::Shiyue => "shiyue",
            StarKey::Shichang => "shichang",
            StarKey::Shiqu => "shiqu",
            StarKey::Shiluan => "shiluan",
            StarKey::Shixi => "shixi",
            StarKey::Shilu => "shilu",
            StarKey::Shiyang => "shiyang",
            StarKey::Shituo => "shituo",
            StarKey::Shima => "shima",
        }
    }
}

impl StarKey {
    /// 由语言无关标识反查星耀；标识未知时返回 `None`。
    ///
    /// 与 [`StarKey::as_key`] 互为逆运算，供绑定层把外部传入的 key 字符串还原为枚举。
    pub fn from_key(key: &str) -> Option<StarKey> {
        match key {
            "ziweiMaj" => Some(StarKey::ZiweiMaj),
            "tianjiMaj" => Some(StarKey::TianjiMaj),
            "taiyangMaj" => Some(StarKey::TaiyangMaj),
            "wuquMaj" => Some(StarKey::WuquMaj),
            "tiantongMaj" => Some(StarKey::TiantongMaj),
            "lianzhenMaj" => Some(StarKey::LianzhenMaj),
            "tianfuMaj" => Some(StarKey::TianfuMaj),
            "taiyinMaj" => Some(StarKey::TaiyinMaj),
            "tanlangMaj" => Some(StarKey::TanlangMaj),
            "jumenMaj" => Some(StarKey::JumenMaj),
            "tianxiangMaj" => Some(StarKey::TianxiangMaj),
            "tianliangMaj" => Some(StarKey::TianliangMaj),
            "qishaMaj" => Some(StarKey::QishaMaj),
            "pojunMaj" => Some(StarKey::PojunMaj),
            "zuofuMin" => Some(StarKey::ZuofuMin),
            "youbiMin" => Some(StarKey::YoubiMin),
            "wenchangMin" => Some(StarKey::WenchangMin),
            "wenquMin" => Some(StarKey::WenquMin),
            "lucunMin" => Some(StarKey::LucunMin),
            "tianmaMin" => Some(StarKey::TianmaMin),
            "qingyangMin" => Some(StarKey::QingyangMin),
            "tuoluoMin" => Some(StarKey::TuoluoMin),
            "huoxingMin" => Some(StarKey::HuoxingMin),
            "lingxingMin" => Some(StarKey::LingxingMin),
            "tiankuiMin" => Some(StarKey::TiankuiMin),
            "tianyueMin" => Some(StarKey::TianyueMin),
            "dikongMin" => Some(StarKey::DikongMin),
            "dijieMin" => Some(StarKey::DijieMin),
            "jieshaAdj" => Some(StarKey::JieshaAdj),
            "tiankong" => Some(StarKey::Tiankong),
            "tianxing" => Some(StarKey::Tianxing),
            "tianyao" => Some(StarKey::Tianyao),
            "jieshen" => Some(StarKey::Jieshen),
            "yinsha" => Some(StarKey::Yinsha),
            "tianxi" => Some(StarKey::Tianxi),
            "tianguan" => Some(StarKey::Tianguan),
            "tianfu" => Some(StarKey::Tianfu),
            "tianku" => Some(StarKey::Tianku),
            "tianxu" => Some(StarKey::Tianxu),
            "longchi" => Some(StarKey::Longchi),
            "fengge" => Some(StarKey::Fengge),
            "hongluan" => Some(StarKey::Hongluan),
            "guchen" => Some(StarKey::Guchen),
            "guasu" => Some(StarKey::Guasu),
            "feilian" => Some(StarKey::Feilian),
            "posui" => Some(StarKey::Posui),
            "taifu" => Some(StarKey::Taifu),
            "fenggao" => Some(StarKey::Fenggao),
            "tianwu" => Some(StarKey::Tianwu),
            "tianyue" => Some(StarKey::Tianyue2),
            "santai" => Some(StarKey::Santai),
            "bazuo" => Some(StarKey::Bazuo),
            "engguang" => Some(StarKey::Engguang),
            "tiangui" => Some(StarKey::Tiangui),
            "tiancai" => Some(StarKey::Tiancai),
            "tianshou" => Some(StarKey::Tianshou),
            "jiekong" => Some(StarKey::Jiekong),
            "xunzhong" => Some(StarKey::Xunzhong),
            "xunkong" => Some(StarKey::Xunkong),
            "kongwang" => Some(StarKey::Kongwang),
            "jielu" => Some(StarKey::Jielu),
            "yuede" => Some(StarKey::Yuede),
            "tianshang" => Some(StarKey::Tianshang),
            "tianshi" => Some(StarKey::Tianshi),
            "tianchu" => Some(StarKey::Tianchu),
            "changsheng" => Some(StarKey::Changsheng),
            "muyu" => Some(StarKey::Muyu),
            "guandai" => Some(StarKey::Guandai),
            "linguan" => Some(StarKey::Linguan),
            "diwang" => Some(StarKey::Diwang),
            "shuai" => Some(StarKey::Shuai),
            "bing" => Some(StarKey::Bing),
            "si" => Some(StarKey::Si),
            "mu" => Some(StarKey::Mu),
            "jue" => Some(StarKey::Jue),
            "tai" => Some(StarKey::Tai),
            "yang" => Some(StarKey::Yang),
            "boshi" => Some(StarKey::Boshi),
            "lishi" => Some(StarKey::Lishi),
            "qinglong" => Some(StarKey::Qinglong),
            "xiaohao" => Some(StarKey::Xiaohao),
            "jiangjun" => Some(StarKey::Jiangjun),
            "zhoushu" => Some(StarKey::Zhoushu),
            "faylian" => Some(StarKey::Faylian),
            "xishen" => Some(StarKey::Xishen),
            "bingfu" => Some(StarKey::Bingfu),
            "dahao" => Some(StarKey::Dahao),
            "suipo" => Some(StarKey::Suipo),
            "fubing" => Some(StarKey::Fubing),
            "guanfu" => Some(StarKey::Guanfu),
            "suijian" => Some(StarKey::Suijian),
            "huiqi" => Some(StarKey::Huiqi),
            "sangmen" => Some(StarKey::Sangmen),
            "guansuo" => Some(StarKey::Guansuo),
            "gwanfu" => Some(StarKey::Gwanfu),
            "longde" => Some(StarKey::Longde),
            "baihu" => Some(StarKey::Baihu),
            "tiande" => Some(StarKey::Tiande),
            "diaoke" => Some(StarKey::Diaoke),
            "jiangxing" => Some(StarKey::Jiangxing),
            "panan" => Some(StarKey::Panan),
            "suiyi" => Some(StarKey::Suiyi),
            "xiishen" => Some(StarKey::Xiishen),
            "huagai" => Some(StarKey::Huagai),
            "jiesha" => Some(StarKey::Jiesha),
            "zhaisha" => Some(StarKey::Zhaisha),
            "tiansha" => Some(StarKey::Tiansha),
            "zhibei" => Some(StarKey::Zhibei),
            "xianchi" => Some(StarKey::Xianchi),
            "yuesha" => Some(StarKey::Yuesha),
            "wangshen" => Some(StarKey::Wangshen),
            "yunkui" => Some(StarKey::Yunkui),
            "yunyue" => Some(StarKey::Yunyue),
            "yunchang" => Some(StarKey::Yunchang),
            "yunqu" => Some(StarKey::Yunqu),
            "yunluan" => Some(StarKey::Yunluan),
            "yunxi" => Some(StarKey::Yunxi),
            "yunlu" => Some(StarKey::Yunlu),
            "yunyang" => Some(StarKey::Yunyang),
            "yuntuo" => Some(StarKey::Yuntuo),
            "yunma" => Some(StarKey::Yunma),
            "liukui" => Some(StarKey::Liukui),
            "liuyue" => Some(StarKey::Liuyue),
            "liuchang" => Some(StarKey::Liuchang),
            "liuqu" => Some(StarKey::Liuqu),
            "liuluan" => Some(StarKey::Liuluan),
            "liuxi" => Some(StarKey::Liuxi),
            "liulu" => Some(StarKey::Liulu),
            "liuyang" => Some(StarKey::Liuyang),
            "liutuo" => Some(StarKey::Liutuo),
            "liuma" => Some(StarKey::Liuma),
            "nianjie" => Some(StarKey::Nianjie),
            "yuekui" => Some(StarKey::Yuekui),
            "yueyue" => Some(StarKey::Yueyue),
            "yuechang" => Some(StarKey::Yuechang),
            "yuequ" => Some(StarKey::Yuequ),
            "yueluan" => Some(StarKey::Yueluan),
            "yuexi" => Some(StarKey::Yuexi),
            "yuelu" => Some(StarKey::Yuelu),
            "yueyang" => Some(StarKey::Yueyang),
            "yuetuo" => Some(StarKey::Yuetuo),
            "yuema" => Some(StarKey::Yuema),
            "rikui" => Some(StarKey::Rikui),
            "riyue" => Some(StarKey::Riyue),
            "richang" => Some(StarKey::Richang),
            "riqu" => Some(StarKey::Riqu),
            "riluan" => Some(StarKey::Riluan),
            "rixi" => Some(StarKey::Rixi),
            "rilu" => Some(StarKey::Rilu),
            "riyang" => Some(StarKey::Riyang),
            "rituo" => Some(StarKey::Rituo),
            "rima" => Some(StarKey::Rima),
            "shikui" => Some(StarKey::Shikui),
            "shiyue" => Some(StarKey::Shiyue),
            "shichang" => Some(StarKey::Shichang),
            "shiqu" => Some(StarKey::Shiqu),
            "shiluan" => Some(StarKey::Shiluan),
            "shixi" => Some(StarKey::Shixi),
            "shilu" => Some(StarKey::Shilu),
            "shiyang" => Some(StarKey::Shiyang),
            "shituo" => Some(StarKey::Shituo),
            "shima" => Some(StarKey::Shima),
            _ => None,
        }
    }
}

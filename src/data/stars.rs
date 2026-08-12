use serde::{Deserialize, Serialize};

use super::types::Brightness;

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

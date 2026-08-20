//! 格局标识：语言无关的 key、全表与反查。

use serde::{Deserialize, Serialize};

/// 格局标识。变体名与 iztro-docs《格局》页的条目一一对应；
/// `as_key` 给出 snake_case 的语言无关字符串，是绑定层与知识包引用格局的唯一锚点。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PatternKey {
    /// 君臣庆会
    JunChenQingHui,
    /// 紫府同宫
    ZiFuTongGong,
    /// 金舆扶驾
    JinYuFuJia,
    /// 紫府夹命
    ZiFuJiaMing,
    /// 极向离明
    JiXiangLiMing,
    /// 极居卯酉
    JiJuMaoYou,
    /// 机月同梁
    JiYueTongLiang,
    /// 善荫朝纲
    ShanYinChaoGang,
    /// 机巨同临
    JiJuTongLin,
    /// 机巨居卯
    JiJuJuMao,
    /// 日月同宫
    RiYueTongGong,
    /// 巨日同宫
    JuRiTongGong,
    /// 日照雷门
    RiZhaoLeiMen,
    /// 日月并明
    RiYueBingMing,
    /// 日月反背
    RiYueFanBei,
    /// 日月照璧
    RiYueZhaoBi,
    /// 金灿光辉
    JinCanGuangHui,
    /// 日月藏辉
    RiYueCangHui,
    /// 丹墀桂墀
    DanChiGuiChi,
    /// 日月夹命
    RiYueJiaMing,
    /// 日月夹财
    RiYueJiaCai,
    /// 月朗天门
    YueLangTianMen,
    /// 月生沧海
    YueShengCangHai,
    /// 明珠出海
    MingZhuChuHai,
    /// 武贪同行
    WuTanTongXing,
    /// 铃昌陀武
    LingChangTuoWu,
    /// 刑囚夹印
    XingQiuJiaYin,
    /// 生不逢时
    ShengBuFengShi,
    /// 雄宿朝元
    XiongSuChaoYuan,
    /// 府相朝垣
    FuXiangChaoYuan,
    /// 火贪
    HuoTan,
    /// 铃贪
    LingTan,
    /// 石中隐玉
    ShiZhongYinYu,
    /// 梁马飘荡
    LiangMaPiaoDang,
    /// 阳梁昌禄
    YangLiangChangLu,
    /// 杀破狼
    ShaPoLang,
    /// 七杀朝斗
    QiShaChaoDou,
    /// 禄衰马困（行运格）
    LuShuaiMaKun,
    /// 英星入庙
    YingXingRuMiao,
    /// 众水朝东
    ZhongShuiChaoDong,
    /// 三奇加会
    SanQiJiaHui,
    /// 禄马交驰
    LuMaJiaoChi,
    /// 禄合鸳鸯
    LuHeYuanYang,
    /// 明禄暗禄
    MingLuAnLu,
    /// 禄马佩印
    LuMaPeiYin,
    /// 两重华盖
    LiangChongHuaGai,
    /// 风云际会（行运格）
    FengYunJiHui,
    /// 羊陀夹命
    YangTuoJiaMing,
    /// 马头带箭
    MaTouDaiJian,
    /// 左右同宫
    ZuoYouTongGong,
    /// 左右夹命
    ZuoYouJiaMing,
    /// 辅弼拱主
    FuBiGongZhu,
    /// 魁钺夹命
    KuiYueJiaMing,
    /// 坐贵向贵
    ZuoGuiXiangGui,
    /// 劫空夹命
    JieKongJiaMing,
    /// 禄逢两杀
    LuFengLiangSha,
    /// 文贵文华
    WenGuiWenHua,
    /// 文星朝命
    WenXingChaoMing,
    /// 昌曲夹命
    ChangQuJiaMing,
    /// 文星暗拱
    WenXingAnGong,
    /// 权禄生逢
    QuanLuShengFeng,
    /// 科明暗禄
    KeMingAnLu,
    /// 科权禄夹
    KeQuanLuJia,
    /// 甲第登庸
    JiaDiDengYong,
}

/// 全部格局，按 iztro-docs《格局》页的条目顺序。
pub const ALL_PATTERNS: [PatternKey; 64] = [
    PatternKey::JunChenQingHui,
    PatternKey::ZiFuTongGong,
    PatternKey::JinYuFuJia,
    PatternKey::ZiFuJiaMing,
    PatternKey::JiXiangLiMing,
    PatternKey::JiJuMaoYou,
    PatternKey::JiYueTongLiang,
    PatternKey::ShanYinChaoGang,
    PatternKey::JiJuTongLin,
    PatternKey::JiJuJuMao,
    PatternKey::RiYueTongGong,
    PatternKey::JuRiTongGong,
    PatternKey::RiZhaoLeiMen,
    PatternKey::RiYueBingMing,
    PatternKey::RiYueFanBei,
    PatternKey::RiYueZhaoBi,
    PatternKey::JinCanGuangHui,
    PatternKey::RiYueCangHui,
    PatternKey::DanChiGuiChi,
    PatternKey::RiYueJiaMing,
    PatternKey::RiYueJiaCai,
    PatternKey::YueLangTianMen,
    PatternKey::YueShengCangHai,
    PatternKey::MingZhuChuHai,
    PatternKey::WuTanTongXing,
    PatternKey::LingChangTuoWu,
    PatternKey::XingQiuJiaYin,
    PatternKey::ShengBuFengShi,
    PatternKey::XiongSuChaoYuan,
    PatternKey::FuXiangChaoYuan,
    PatternKey::HuoTan,
    PatternKey::LingTan,
    PatternKey::ShiZhongYinYu,
    PatternKey::LiangMaPiaoDang,
    PatternKey::YangLiangChangLu,
    PatternKey::ShaPoLang,
    PatternKey::QiShaChaoDou,
    PatternKey::LuShuaiMaKun,
    PatternKey::YingXingRuMiao,
    PatternKey::ZhongShuiChaoDong,
    PatternKey::SanQiJiaHui,
    PatternKey::LuMaJiaoChi,
    PatternKey::LuHeYuanYang,
    PatternKey::MingLuAnLu,
    PatternKey::LuMaPeiYin,
    PatternKey::LiangChongHuaGai,
    PatternKey::FengYunJiHui,
    PatternKey::YangTuoJiaMing,
    PatternKey::MaTouDaiJian,
    PatternKey::ZuoYouTongGong,
    PatternKey::ZuoYouJiaMing,
    PatternKey::FuBiGongZhu,
    PatternKey::KuiYueJiaMing,
    PatternKey::ZuoGuiXiangGui,
    PatternKey::JieKongJiaMing,
    PatternKey::LuFengLiangSha,
    PatternKey::WenGuiWenHua,
    PatternKey::WenXingChaoMing,
    PatternKey::ChangQuJiaMing,
    PatternKey::WenXingAnGong,
    PatternKey::QuanLuShengFeng,
    PatternKey::KeMingAnLu,
    PatternKey::KeQuanLuJia,
    PatternKey::JiaDiDengYong,
];

impl PatternKey {
    /// 语言无关标识（snake_case 拼音）。
    pub fn as_key(self) -> &'static str {
        match self {
            PatternKey::JunChenQingHui => "jun_chen_qing_hui",
            PatternKey::ZiFuTongGong => "zi_fu_tong_gong",
            PatternKey::JinYuFuJia => "jin_yu_fu_jia",
            PatternKey::ZiFuJiaMing => "zi_fu_jia_ming",
            PatternKey::JiXiangLiMing => "ji_xiang_li_ming",
            PatternKey::JiJuMaoYou => "ji_ju_mao_you",
            PatternKey::JiYueTongLiang => "ji_yue_tong_liang",
            PatternKey::ShanYinChaoGang => "shan_yin_chao_gang",
            PatternKey::JiJuTongLin => "ji_ju_tong_lin",
            PatternKey::JiJuJuMao => "ji_ju_ju_mao",
            PatternKey::RiYueTongGong => "ri_yue_tong_gong",
            PatternKey::JuRiTongGong => "ju_ri_tong_gong",
            PatternKey::RiZhaoLeiMen => "ri_zhao_lei_men",
            PatternKey::RiYueBingMing => "ri_yue_bing_ming",
            PatternKey::RiYueFanBei => "ri_yue_fan_bei",
            PatternKey::RiYueZhaoBi => "ri_yue_zhao_bi",
            PatternKey::JinCanGuangHui => "jin_can_guang_hui",
            PatternKey::RiYueCangHui => "ri_yue_cang_hui",
            PatternKey::DanChiGuiChi => "dan_chi_gui_chi",
            PatternKey::RiYueJiaMing => "ri_yue_jia_ming",
            PatternKey::RiYueJiaCai => "ri_yue_jia_cai",
            PatternKey::YueLangTianMen => "yue_lang_tian_men",
            PatternKey::YueShengCangHai => "yue_sheng_cang_hai",
            PatternKey::MingZhuChuHai => "ming_zhu_chu_hai",
            PatternKey::WuTanTongXing => "wu_tan_tong_xing",
            PatternKey::LingChangTuoWu => "ling_chang_tuo_wu",
            PatternKey::XingQiuJiaYin => "xing_qiu_jia_yin",
            PatternKey::ShengBuFengShi => "sheng_bu_feng_shi",
            PatternKey::XiongSuChaoYuan => "xiong_su_chao_yuan",
            PatternKey::FuXiangChaoYuan => "fu_xiang_chao_yuan",
            PatternKey::HuoTan => "huo_tan",
            PatternKey::LingTan => "ling_tan",
            PatternKey::ShiZhongYinYu => "shi_zhong_yin_yu",
            PatternKey::LiangMaPiaoDang => "liang_ma_piao_dang",
            PatternKey::YangLiangChangLu => "yang_liang_chang_lu",
            PatternKey::ShaPoLang => "sha_po_lang",
            PatternKey::QiShaChaoDou => "qi_sha_chao_dou",
            PatternKey::LuShuaiMaKun => "lu_shuai_ma_kun",
            PatternKey::YingXingRuMiao => "ying_xing_ru_miao",
            PatternKey::ZhongShuiChaoDong => "zhong_shui_chao_dong",
            PatternKey::SanQiJiaHui => "san_qi_jia_hui",
            PatternKey::LuMaJiaoChi => "lu_ma_jiao_chi",
            PatternKey::LuHeYuanYang => "lu_he_yuan_yang",
            PatternKey::MingLuAnLu => "ming_lu_an_lu",
            PatternKey::LuMaPeiYin => "lu_ma_pei_yin",
            PatternKey::LiangChongHuaGai => "liang_chong_hua_gai",
            PatternKey::FengYunJiHui => "feng_yun_ji_hui",
            PatternKey::YangTuoJiaMing => "yang_tuo_jia_ming",
            PatternKey::MaTouDaiJian => "ma_tou_dai_jian",
            PatternKey::ZuoYouTongGong => "zuo_you_tong_gong",
            PatternKey::ZuoYouJiaMing => "zuo_you_jia_ming",
            PatternKey::FuBiGongZhu => "fu_bi_gong_zhu",
            PatternKey::KuiYueJiaMing => "kui_yue_jia_ming",
            PatternKey::ZuoGuiXiangGui => "zuo_gui_xiang_gui",
            PatternKey::JieKongJiaMing => "jie_kong_jia_ming",
            PatternKey::LuFengLiangSha => "lu_feng_liang_sha",
            PatternKey::WenGuiWenHua => "wen_gui_wen_hua",
            PatternKey::WenXingChaoMing => "wen_xing_chao_ming",
            PatternKey::ChangQuJiaMing => "chang_qu_jia_ming",
            PatternKey::WenXingAnGong => "wen_xing_an_gong",
            PatternKey::QuanLuShengFeng => "quan_lu_sheng_feng",
            PatternKey::KeMingAnLu => "ke_ming_an_lu",
            PatternKey::KeQuanLuJia => "ke_quan_lu_jia",
            PatternKey::JiaDiDengYong => "jia_di_deng_yong",
        }
    }

    /// 由语言无关标识反查；未知字符串返回 `None`。
    pub fn from_key(key: &str) -> Option<Self> {
        ALL_PATTERNS.iter().copied().find(|p| p.as_key() == key)
    }

    /// 是否行运格：只在运限视角下判定，本命盘不报。
    pub fn is_horoscope_only(self) -> bool {
        matches!(self, PatternKey::LuShuaiMaKun | PatternKey::FengYunJiHui)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keys_round_trip_and_unique() {
        let mut seen = std::collections::HashSet::new();
        for p in ALL_PATTERNS {
            assert_eq!(PatternKey::from_key(p.as_key()), Some(p));
            assert!(seen.insert(p.as_key()), "duplicate key {}", p.as_key());
        }
        assert_eq!(PatternKey::from_key("nope"), None);
    }
}

use super::stars::StarKey;
use super::types::{FiveElements, HeavenlyStem, YinYang};

/// 天干信息
pub struct HeavenlyStemInfo {
    /// 阴阳
    pub yin_yang: YinYang,
    /// 五行
    pub five_elements: FiveElements,
    /// 对冲天干
    pub crash: Option<HeavenlyStem>,
    /// 四化 [禄, 权, 科, 忌]
    pub mutagen: [StarKey; 4],
}

/// 获取天干信息
pub fn get_heavenly_stem_info(stem: HeavenlyStem) -> HeavenlyStemInfo {
    match stem {
        HeavenlyStem::Jia => HeavenlyStemInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Wood,
            crash: Some(HeavenlyStem::Geng),
            mutagen: [
                StarKey::LianzhenMaj,
                StarKey::PojunMaj,
                StarKey::WuquMaj,
                StarKey::TaiyangMaj,
            ],
        },
        HeavenlyStem::Yi => HeavenlyStemInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Wood,
            crash: Some(HeavenlyStem::Xin),
            mutagen: [
                StarKey::TianjiMaj,
                StarKey::TianliangMaj,
                StarKey::ZiweiMaj,
                StarKey::TaiyinMaj,
            ],
        },
        HeavenlyStem::Bing => HeavenlyStemInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Fire,
            crash: Some(HeavenlyStem::Ren),
            mutagen: [
                StarKey::TiantongMaj,
                StarKey::TianjiMaj,
                StarKey::WenchangMin,
                StarKey::LianzhenMaj,
            ],
        },
        HeavenlyStem::Ding => HeavenlyStemInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Fire,
            crash: Some(HeavenlyStem::Gui),
            mutagen: [
                StarKey::TaiyinMaj,
                StarKey::TiantongMaj,
                StarKey::TianjiMaj,
                StarKey::JumenMaj,
            ],
        },
        HeavenlyStem::Wu => HeavenlyStemInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Earth,
            crash: None,
            mutagen: [
                StarKey::TanlangMaj,
                StarKey::TaiyinMaj,
                StarKey::YoubiMin,
                StarKey::TianjiMaj,
            ],
        },
        HeavenlyStem::Ji => HeavenlyStemInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Earth,
            crash: None,
            mutagen: [
                StarKey::WuquMaj,
                StarKey::TanlangMaj,
                StarKey::TianliangMaj,
                StarKey::WenquMin,
            ],
        },
        HeavenlyStem::Geng => HeavenlyStemInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Metal,
            crash: Some(HeavenlyStem::Jia),
            mutagen: [
                StarKey::TaiyangMaj,
                StarKey::WuquMaj,
                StarKey::TaiyinMaj,
                StarKey::TiantongMaj,
            ],
        },
        HeavenlyStem::Xin => HeavenlyStemInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Metal,
            crash: Some(HeavenlyStem::Yi),
            mutagen: [
                StarKey::JumenMaj,
                StarKey::TaiyangMaj,
                StarKey::WenquMin,
                StarKey::WenchangMin,
            ],
        },
        HeavenlyStem::Ren => HeavenlyStemInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Water,
            crash: Some(HeavenlyStem::Bing),
            mutagen: [
                StarKey::TianliangMaj,
                StarKey::ZiweiMaj,
                StarKey::ZuofuMin,
                StarKey::WuquMaj,
            ],
        },
        HeavenlyStem::Gui => HeavenlyStemInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Water,
            crash: Some(HeavenlyStem::Ding),
            mutagen: [
                StarKey::PojunMaj,
                StarKey::JumenMaj,
                StarKey::TaiyinMaj,
                StarKey::TanlangMaj,
            ],
        },
    }
}

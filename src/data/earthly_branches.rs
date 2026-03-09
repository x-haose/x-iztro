use super::stars::StarKey;
use super::types::{EarthlyBranch, FiveElements, YinYang};

/// 地支信息
pub struct EarthlyBranchInfo {
    /// 阴阳
    pub yin_yang: YinYang,
    /// 五行
    pub five_elements: FiveElements,
    /// 对冲地支
    pub crash: EarthlyBranch,
    /// 命主星
    pub soul: StarKey,
    /// 身主星
    pub body: StarKey,
}

/// 获取地支信息
pub fn get_earthly_branch_info(branch: EarthlyBranch) -> EarthlyBranchInfo {
    match branch {
        EarthlyBranch::Zi => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Water,
            crash: EarthlyBranch::Wu,
            soul: StarKey::TanlangMaj,
            body: StarKey::HuoxingMin,
        },
        EarthlyBranch::Chou => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Wei,
            soul: StarKey::JumenMaj,
            body: StarKey::TianxiangMaj,
        },
        EarthlyBranch::Yin => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Wood,
            crash: EarthlyBranch::Shen,
            soul: StarKey::LucunMin,
            body: StarKey::TianliangMaj,
        },
        EarthlyBranch::Mao => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Wood,
            crash: EarthlyBranch::You,
            soul: StarKey::WenquMin,
            body: StarKey::TiantongMaj,
        },
        EarthlyBranch::Chen => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Xu,
            soul: StarKey::LianzhenMaj,
            body: StarKey::WenchangMin,
        },
        EarthlyBranch::Si => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Fire,
            crash: EarthlyBranch::Hai,
            soul: StarKey::WuquMaj,
            body: StarKey::TianjiMaj,
        },
        EarthlyBranch::Wu => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Fire,
            crash: EarthlyBranch::Zi,
            soul: StarKey::PojunMaj,
            body: StarKey::HuoxingMin,
        },
        EarthlyBranch::Wei => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Chou,
            soul: StarKey::WuquMaj,
            body: StarKey::TianxiangMaj,
        },
        EarthlyBranch::Shen => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Metal,
            crash: EarthlyBranch::Yin,
            soul: StarKey::LianzhenMaj,
            body: StarKey::TianliangMaj,
        },
        EarthlyBranch::You => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Metal,
            crash: EarthlyBranch::Mao,
            soul: StarKey::WenquMin,
            body: StarKey::TiantongMaj,
        },
        EarthlyBranch::Xu => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Chen,
            soul: StarKey::LucunMin,
            body: StarKey::WenchangMin,
        },
        EarthlyBranch::Hai => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Water,
            crash: EarthlyBranch::Si,
            soul: StarKey::JumenMaj,
            body: StarKey::TianjiMaj,
        },
    }
}

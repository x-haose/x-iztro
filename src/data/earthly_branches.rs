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
    /// 对应脏腑
    pub inside: &'static str,
    /// 对应身体部位
    pub outside: &'static str,
    /// 健康提示
    pub health_tip: &'static str,
}

/// 获取地支信息
///
/// `inside` / `outside` / `health_tip` 三项在 iztro 中只有中文一种写法，
/// 不参与国际化，因此原样返回。
pub fn get_earthly_branch_info(branch: EarthlyBranch) -> EarthlyBranchInfo {
    match branch {
        EarthlyBranch::Zi => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Water,
            crash: EarthlyBranch::Wu,
            soul: StarKey::TanlangMaj,
            body: StarKey::HuoxingMin,
            inside: "胆",
            outside: "下体",
            health_tip: "生殖系统、膀胱、尿道之疾病，听觉障碍",
        },
        EarthlyBranch::Chou => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Wei,
            soul: StarKey::JumenMaj,
            body: StarKey::TianxiangMaj,
            inside: "肝",
            outside: "小腿、脚（右）",
            health_tip: "胸部、肋膜炎、胃病、脚部",
        },
        EarthlyBranch::Yin => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Wood,
            crash: EarthlyBranch::Shen,
            soul: StarKey::LucunMin,
            body: StarKey::TianliangMaj,
            inside: "肺",
            outside: "大腿（右）",
            health_tip: "胆囊、关节、胫部、神经痛、风湿",
        },
        EarthlyBranch::Mao => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Wood,
            crash: EarthlyBranch::You,
            soul: StarKey::WenquMin,
            body: StarKey::TiantongMaj,
            inside: "大肠",
            outside: "腰（右）、背",
            health_tip: "肝病、颜面神经、失眠、神经衰弱",
        },
        EarthlyBranch::Chen => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Xu,
            soul: StarKey::LianzhenMaj,
            body: StarKey::WenchangMin,
            inside: "胃",
            outside: "胸、胳膊（右）",
            health_tip: "消化系统、脊椎、皮肤疾病",
        },
        EarthlyBranch::Si => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Fire,
            crash: EarthlyBranch::Hai,
            soul: StarKey::WuquMaj,
            body: StarKey::TianjiMaj,
            inside: "脾",
            outside: "左肩",
            health_tip: "喉头、牙病、感冒",
        },
        EarthlyBranch::Wu => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Fire,
            crash: EarthlyBranch::Zi,
            soul: StarKey::PojunMaj,
            body: StarKey::HuoxingMin,
            inside: "心",
            outside: "头",
            health_tip: "心脏、视觉、味觉障碍、火难",
        },
        EarthlyBranch::Wei => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Chou,
            soul: StarKey::WuquMaj,
            body: StarKey::TianxiangMaj,
            inside: "小肠",
            outside: "脸",
            health_tip: "消化系统、胰脏、健忘症、疲倦、手腕、嘴唇",
        },
        EarthlyBranch::Shen => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Metal,
            crash: EarthlyBranch::Yin,
            soul: StarKey::LianzhenMaj,
            body: StarKey::TianliangMaj,
            inside: "膀胱",
            outside: "胸、胳膊（左）",
            health_tip: "呼吸系统、肺部、消化系统、大肠",
        },
        EarthlyBranch::You => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Metal,
            crash: EarthlyBranch::Mao,
            soul: StarKey::WenquMin,
            body: StarKey::TiantongMaj,
            inside: "肾",
            outside: "腰（左）、腹",
            health_tip: "吐血、痢血、小肠之疾、脑出血、头腕部",
        },
        EarthlyBranch::Xu => EarthlyBranchInfo {
            yin_yang: YinYang::Yang,
            five_elements: FiveElements::Earth,
            crash: EarthlyBranch::Chen,
            soul: StarKey::LucunMin,
            body: StarKey::WenchangMin,
            inside: "心包",
            outside: "大腿（左）",
            health_tip: "下半身之疾、子宫、痔疮、脚部",
        },
        EarthlyBranch::Hai => EarthlyBranchInfo {
            yin_yang: YinYang::Yin,
            five_elements: FiveElements::Water,
            crash: EarthlyBranch::Si,
            soul: StarKey::JumenMaj,
            body: StarKey::TianjiMaj,
            inside: "三焦",
            outside: "小腿、脚（左）",
            health_tip: "排泄机能障碍、肾脏、尿道、偏头痛",
        },
    }
}

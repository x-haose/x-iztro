//! 格局名称六语言词表。
//!
//! iztro 本身没有格局 API，这些译名由 x-iztro 给出：中文按《紫微斗数全书》原名，
//! 日文用汉字，韩文用汉字音（首音法则），越南文用汉越音，英文为意译。

use crate::data::types::Language;
use crate::pattern::PatternKey;

macro_rules! pattern_names {
    ($($key:ident: [$zh_cn:literal, $zh_tw:literal, $en_us:literal, $ja_jp:literal, $ko_kr:literal, $vi_vn:literal]),+ $(,)?) => {
        /// 格局名称（按语言）
        pub fn pattern_name(key: PatternKey, lang: Language) -> &'static str {
            let i = match lang {
                Language::ZhCN => 0,
                Language::ZhTW => 1,
                Language::EnUS => 2,
                Language::JaJP => 3,
                Language::KoKR => 4,
                Language::ViVN => 5,
            };
            match key {
                $(PatternKey::$key => [$zh_cn, $zh_tw, $en_us, $ja_jp, $ko_kr, $vi_vn][i],)+
            }
        }
    };
}

pattern_names! {
    JunChenQingHui: ["君臣庆会", "君臣慶會", "Sovereign and Ministers Assembly", "君臣慶会", "군신경회", "Quân Thần Khánh Hội"],
    ZiFuTongGong: ["紫府同宫", "紫府同宮", "Emperor and Empress in One Palace", "紫府同宮", "자부동궁", "Tử Phủ Đồng Cung"],
    JinYuFuJia: ["金舆扶驾", "金輿扶駕", "Golden Carriage Escort", "金輿扶駕", "금여부가", "Kim Dư Phù Giá"],
    ZiFuJiaMing: ["紫府夹命", "紫府夾命", "Emperor and Empress Flanking Life", "紫府夾命", "자부협명", "Tử Phủ Giáp Mệnh"],
    JiXiangLiMing: ["极向离明", "極向離明", "Emperor Facing Southern Light", "極向離明", "극향리명", "Cực Hướng Ly Minh"],
    JiJuMaoYou: ["极居卯酉", "極居卯酉", "Emperor at Mao or You", "極居卯酉", "극거묘유", "Cực Cư Mão Dậu"],
    JiYueTongLiang: ["机月同梁", "機月同梁", "Advisor, Moon, Fortunate and Sage", "機月同梁", "기월동량", "Cơ Nguyệt Đồng Lương"],
    ShanYinChaoGang: ["善荫朝纲", "善蔭朝綱", "Benevolent Shelter of the Court", "善蔭朝綱", "선음조강", "Thiện Ấm Triều Cương"],
    JiJuTongLin: ["机巨同临", "機巨同臨", "Advisor and Advocator Together", "機巨同臨", "기거동림", "Cơ Cự Đồng Lâm"],
    JiJuJuMao: ["机巨居卯", "機巨居卯", "Advisor and Advocator at Mao", "機巨居卯", "기거거묘", "Cơ Cự Cư Mão"],
    RiYueTongGong: ["日月同宫", "日月同宮", "Sun and Moon in One Palace", "日月同宮", "일월동궁", "Nhật Nguyệt Đồng Cung"],
    JuRiTongGong: ["巨日同宫", "巨日同宮", "Advocator and Sun in One Palace", "巨日同宮", "거일동궁", "Cự Nhật Đồng Cung"],
    RiZhaoLeiMen: ["日照雷门", "日照雷門", "Sun Shining on Thunder Gate", "日照雷門", "일조뢰문", "Nhật Chiếu Lôi Môn"],
    RiYueBingMing: ["日月并明", "日月並明", "Sun and Moon Both Bright", "日月並明", "일월병명", "Nhật Nguyệt Tịnh Minh"],
    RiYueFanBei: ["日月反背", "日月反背", "Sun and Moon Both Dim", "日月反背", "일월반배", "Nhật Nguyệt Phản Bối"],
    RiYueZhaoBi: ["日月照璧", "日月照璧", "Sun and Moon Lighting the Wall", "日月照璧", "일월조벽", "Nhật Nguyệt Chiếu Bích"],
    JinCanGuangHui: ["金灿光辉", "金燦光輝", "Golden Radiance", "金燦光輝", "금찬광휘", "Kim Xán Quang Huy"],
    RiYueCangHui: ["日月藏辉", "日月藏輝", "Sun and Moon Hiding Their Light", "日月藏輝", "일월장휘", "Nhật Nguyệt Tàng Huy"],
    DanChiGuiChi: ["丹墀桂墀", "丹墀桂墀", "Cinnabar and Cassia Steps", "丹墀桂墀", "단지계지", "Đan Trì Quế Trì"],
    RiYueJiaMing: ["日月夹命", "日月夾命", "Sun and Moon Flanking Life", "日月夾命", "일월협명", "Nhật Nguyệt Giáp Mệnh"],
    RiYueJiaCai: ["日月夹财", "日月夾財", "Sun and Moon Flanking Wealth", "日月夾財", "일월협재", "Nhật Nguyệt Giáp Tài"],
    YueLangTianMen: ["月朗天门", "月朗天門", "Bright Moon at Heaven's Gate", "月朗天門", "월랑천문", "Nguyệt Lãng Thiên Môn"],
    YueShengCangHai: ["月生沧海", "月生滄海", "Moon Rising over the Sea", "月生滄海", "월생창해", "Nguyệt Sinh Thương Hải"],
    MingZhuChuHai: ["明珠出海", "明珠出海", "Pearl Emerging from the Sea", "明珠出海", "명주출해", "Minh Châu Xuất Hải"],
    WuTanTongXing: ["武贪同行", "武貪同行", "General and Wolf Together", "武貪同行", "무탐동행", "Vũ Tham Đồng Hành"],
    LingChangTuoWu: ["铃昌陀武", "鈴昌陀武", "Bell, Scholar, Tuoluo and General", "鈴昌陀武", "영창타무", "Linh Xương Đà Vũ"],
    XingQiuJiaYin: ["刑囚夹印", "刑囚夾印", "Punishment and Prisoner Flanking the Seal", "刑囚夾印", "형수협인", "Hình Tù Giáp Ấn"],
    ShengBuFengShi: ["生不逢时", "生不逢時", "Born at the Wrong Time", "生不逢時", "생불봉시", "Sinh Bất Phùng Thời"],
    XiongSuChaoYuan: ["雄宿朝元", "雄宿朝元", "Heroic Star Facing the Origin", "雄宿朝元", "웅수조원", "Hùng Tú Triều Nguyên"],
    FuXiangChaoYuan: ["府相朝垣", "府相朝垣", "Empress and Minister Facing the Palace", "府相朝垣", "부상조원", "Phủ Tướng Triều Viên"],
    HuoTan: ["火贪", "火貪", "Fire and Wolf", "火貪", "화탐", "Hỏa Tham"],
    LingTan: ["铃贪", "鈴貪", "Bell and Wolf", "鈴貪", "영탐", "Linh Tham"],
    ShiZhongYinYu: ["石中隐玉", "石中隱玉", "Jade Hidden in Stone", "石中隱玉", "석중은옥", "Thạch Trung Ẩn Ngọc"],
    LiangMaPiaoDang: ["梁马飘荡", "梁馬飄蕩", "Sage and Horse Drifting", "梁馬飄蕩", "양마표탕", "Lương Mã Phiêu Đãng"],
    YangLiangChangLu: ["阳梁昌禄", "陽梁昌祿", "Sun, Sage, Scholar and Money", "陽梁昌祿", "양량창록", "Dương Lương Xương Lộc"],
    ShaPoLang: ["杀破狼", "殺破狼", "Marshal, Rebel and Wolf", "殺破狼", "살파랑", "Sát Phá Lang"],
    QiShaChaoDou: ["七杀朝斗", "七殺朝斗", "Marshal Facing the Dipper", "七殺朝斗", "칠살조두", "Thất Sát Triều Đẩu"],
    LuShuaiMaKun: ["禄衰马困", "祿衰馬困", "Money Waning, Horse Trapped", "祿衰馬困", "녹쇠마곤", "Lộc Suy Mã Khốn"],
    YingXingRuMiao: ["英星入庙", "英星入廟", "Heroic Star Enthroned", "英星入廟", "영성입묘", "Anh Tinh Nhập Miếu"],
    ZhongShuiChaoDong: ["众水朝东", "眾水朝東", "Waters Flowing East", "衆水朝東", "중수조동", "Chúng Thủy Triều Đông"],
    SanQiJiaHui: ["三奇加会", "三奇加會", "Three Wonders Assembly", "三奇加会", "삼기가회", "Tam Kỳ Gia Hội"],
    LuMaJiaoChi: ["禄马交驰", "祿馬交馳", "Money and Horse Galloping Together", "祿馬交馳", "녹마교치", "Lộc Mã Giao Trì"],
    LuHeYuanYang: ["禄合鸳鸯", "祿合鴛鴦", "Mandarin Ducks of Fortune", "祿合鴛鴦", "녹합원앙", "Lộc Hợp Uyên Ương"],
    MingLuAnLu: ["明禄暗禄", "明祿暗祿", "Open and Hidden Fortune", "明祿暗祿", "명록암록", "Minh Lộc Ám Lộc"],
    LuMaPeiYin: ["禄马佩印", "祿馬佩印", "Money and Horse Bearing the Seal", "祿馬佩印", "녹마패인", "Lộc Mã Bội Ấn"],
    LiangChongHuaGai: ["两重华盖", "兩重華蓋", "Double Canopy", "兩重華蓋", "양중화개", "Lưỡng Trùng Hoa Cái"],
    FengYunJiHui: ["风云际会", "風雲際會", "Meeting of Wind and Cloud", "風雲際会", "풍운제회", "Phong Vân Tế Hội"],
    YangTuoJiaMing: ["羊陀夹命", "羊陀夾命", "Qingyang and Tuoluo Flanking Life", "羊陀夾命", "양타협명", "Dương Đà Giáp Mệnh"],
    MaTouDaiJian: ["马头带箭", "馬頭帶箭", "Arrow at the Horse's Head", "馬頭帶箭", "마두대전", "Mã Đầu Đới Tiễn"],
    ZuoYouTongGong: ["左右同宫", "左右同宮", "Officer and Helper in One Palace", "左右同宮", "좌우동궁", "Tả Hữu Đồng Cung"],
    ZuoYouJiaMing: ["左右夹命", "左右夾命", "Officer and Helper Flanking Life", "左右夾命", "좌우협명", "Tả Hữu Giáp Mệnh"],
    FuBiGongZhu: ["辅弼拱主", "輔弼拱主", "Officer and Helper Attending the Emperor", "輔弼拱主", "보필공주", "Phụ Bật Củng Chủ"],
    KuiYueJiaMing: ["魁钺夹命", "魁鉞夾命", "Kui and Yue Flanking Life", "魁鉞夾命", "괴월협명", "Khôi Việt Giáp Mệnh"],
    ZuoGuiXiangGui: ["坐贵向贵", "坐貴向貴", "Sitting on and Facing Nobility", "坐貴向貴", "좌귀향귀", "Tọa Quý Hướng Quý"],
    JieKongJiaMing: ["劫空夹命", "劫空夾命", "Void and Robbery Flanking Life", "劫空夾命", "겁공협명", "Kiếp Không Giáp Mệnh"],
    LuFengLiangSha: ["禄逢两杀", "祿逢兩殺", "Fortune Meeting Two Killers", "祿逢兩殺", "녹봉량살", "Lộc Phùng Lưỡng Sát"],
    WenGuiWenHua: ["文贵文华", "文貴文華", "Literary Nobility and Brilliance", "文貴文華", "문귀문화", "Văn Quý Văn Hoa"],
    WenXingChaoMing: ["文星朝命", "文星朝命", "Literary Stars Facing Life", "文星朝命", "문성조명", "Văn Tinh Triều Mệnh"],
    ChangQuJiaMing: ["昌曲夹命", "昌曲夾命", "Scholar and Artist Flanking Life", "昌曲夾命", "창곡협명", "Xương Khúc Giáp Mệnh"],
    WenXingAnGong: ["文星暗拱", "文星暗拱", "Literary Stars in Hidden Support", "文星暗拱", "문성암공", "Văn Tinh Ám Củng"],
    QuanLuShengFeng: ["权禄生逢", "權祿生逢", "Power and Fortune at Birth", "權祿生逢", "권록생봉", "Quyền Lộc Sinh Phùng"],
    KeMingAnLu: ["科明暗禄", "科明暗祿", "Fame Open, Fortune Hidden", "科明暗祿", "과명암록", "Khoa Minh Ám Lộc"],
    KeQuanLuJia: ["科权禄夹", "科權祿夾", "Fame, Power and Fortune Flanking", "科權祿夾", "과권록협", "Khoa Quyền Lộc Giáp"],
    JiaDiDengYong: ["甲第登庸", "甲第登庸", "Top Graduate Appointed", "甲第登庸", "갑제등용", "Giáp Đệ Đăng Dung"],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pattern::ALL_PATTERNS;

    #[test]
    fn every_pattern_has_a_name_in_every_language() {
        for p in ALL_PATTERNS {
            for lang in [
                Language::ZhCN,
                Language::ZhTW,
                Language::EnUS,
                Language::JaJP,
                Language::KoKR,
                Language::ViVN,
            ] {
                assert!(!pattern_name(p, lang).is_empty());
            }
        }
        assert_eq!(
            pattern_name(PatternKey::ZiFuTongGong, Language::ZhCN),
            "紫府同宫"
        );
    }
}

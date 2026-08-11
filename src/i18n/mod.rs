pub mod zh_cn;
pub mod zh_tw;
pub mod en_us;
pub mod ja_jp;
pub mod ko_kr;
pub mod vi_vn;

use crate::data::stars::StarKey;
use crate::data::types::*;

pub fn translate_star(key: StarKey, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::star_name(key),
        Language::ZhTW => zh_tw::star_name(key),
        Language::EnUS => en_us::star_name(key),
        Language::JaJP => ja_jp::star_name(key),
        Language::KoKR => ko_kr::star_name(key),
        Language::ViVN => vi_vn::star_name(key),
    }
}

pub fn translate_palace(palace: Palace, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::palace_name(palace),
        Language::ZhTW => zh_tw::palace_name(palace),
        Language::EnUS => en_us::palace_name(palace),
        Language::JaJP => ja_jp::palace_name(palace),
        Language::KoKR => ko_kr::palace_name(palace),
        Language::ViVN => vi_vn::palace_name(palace),
    }
}

pub fn translate_heavenly_stem(stem: HeavenlyStem, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::heavenly_stem_name(stem),
        Language::ZhTW => zh_tw::heavenly_stem_name(stem),
        Language::EnUS => en_us::heavenly_stem_name(stem),
        Language::JaJP => ja_jp::heavenly_stem_name(stem),
        Language::KoKR => ko_kr::heavenly_stem_name(stem),
        Language::ViVN => vi_vn::heavenly_stem_name(stem),
    }
}

pub fn translate_earthly_branch(branch: EarthlyBranch, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::earthly_branch_name(branch),
        Language::ZhTW => zh_tw::earthly_branch_name(branch),
        Language::EnUS => en_us::earthly_branch_name(branch),
        Language::JaJP => ja_jp::earthly_branch_name(branch),
        Language::KoKR => ko_kr::earthly_branch_name(branch),
        Language::ViVN => vi_vn::earthly_branch_name(branch),
    }
}

pub fn translate_brightness(b: Brightness, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::brightness_name(b),
        Language::ZhTW => zh_tw::brightness_name(b),
        Language::EnUS => en_us::brightness_name(b),
        Language::JaJP => ja_jp::brightness_name(b),
        Language::KoKR => ko_kr::brightness_name(b),
        Language::ViVN => vi_vn::brightness_name(b),
    }
}

pub fn translate_mutagen(m: Mutagen, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::mutagen_name(m),
        Language::ZhTW => zh_tw::mutagen_name(m),
        Language::EnUS => en_us::mutagen_name(m),
        Language::JaJP => ja_jp::mutagen_name(m),
        Language::KoKR => ko_kr::mutagen_name(m),
        Language::ViVN => vi_vn::mutagen_name(m),
    }
}

pub fn translate_five_elements_class(c: FiveElementsClass, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::five_elements_class_name(c),
        Language::ZhTW => zh_tw::five_elements_class_name(c),
        Language::EnUS => en_us::five_elements_class_name(c),
        Language::JaJP => ja_jp::five_elements_class_name(c),
        Language::KoKR => ko_kr::five_elements_class_name(c),
        Language::ViVN => vi_vn::five_elements_class_name(c),
    }
}

pub fn translate_gender(g: Gender, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::gender_name(g),
        Language::ZhTW => zh_tw::gender_name(g),
        Language::EnUS => en_us::gender_name(g),
        Language::JaJP => ja_jp::gender_name(g),
        Language::KoKR => ko_kr::gender_name(g),
        Language::ViVN => vi_vn::gender_name(g),
    }
}

/// 时辰名称（time_index 0-12：早子时…晚子时）
pub fn translate_time(time_index: u8, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::time_name(time_index),
        Language::ZhTW => zh_tw::time_name(time_index),
        Language::EnUS => en_us::time_name(time_index),
        Language::JaJP => ja_jp::time_name(time_index),
        Language::KoKR => ko_kr::time_name(time_index),
        Language::ViVN => vi_vn::time_name(time_index),
    }
}

/// 星座名称（sign_index 0-11：白羊座起，按黄道顺序）
pub fn translate_sign(sign_index: usize, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::sign_name(sign_index),
        Language::ZhTW => zh_tw::sign_name(sign_index),
        Language::EnUS => en_us::sign_name(sign_index),
        Language::JaJP => ja_jp::sign_name(sign_index),
        Language::KoKR => ko_kr::sign_name(sign_index),
        Language::ViVN => vi_vn::sign_name(sign_index),
    }
}

/// 生肖名称（按年支）
pub fn translate_zodiac(branch: EarthlyBranch, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::zodiac_name(branch),
        Language::ZhTW => zh_tw::zodiac_name(branch),
        Language::EnUS => en_us::zodiac_name(branch),
        Language::JaJP => ja_jp::zodiac_name(branch),
        Language::KoKR => ko_kr::zodiac_name(branch),
        Language::ViVN => vi_vn::zodiac_name(branch),
    }
}

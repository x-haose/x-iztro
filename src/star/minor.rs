//! 辅星安放
//!
//! 将14颗辅星安放到12宫中。

use crate::data::heavenly_stems::get_heavenly_stem_info;
use crate::data::stars::{StarKey, get_brightness_table};
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::star::Star;

/// 创建一颗辅星
fn make_star(
    key: StarKey,
    star_type: StarType,
    scope: Scope,
    palace_index: usize,
    yearly_stem: Option<HeavenlyStem>,
    lang: Language,
) -> Star {
    let brightness = get_brightness_table(key).and_then(|table| table[palace_index]);
    let mutagen = yearly_stem.and_then(|stem| {
        let info = get_heavenly_stem_info(stem);
        info.mutagen
            .iter()
            .position(|&k| k == key)
            .map(|i| [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji][i])
    });
    Star {
        key,
        name: translate_star(key, lang).to_string(),
        star_type,
        scope,
        brightness,
        mutagen,
    }
}

/// 获取14颗辅星的安放结果。
/// 安放顺序（决定宫内星耀排列）：左辅、右弼、文昌、文曲、天魁、天钺、
/// 禄存、天马、地空、地劫、火星、铃星、擎羊、陀罗。
#[allow(clippy::too_many_arguments)]
pub fn get_minor_stars(
    zuo_index: usize,
    you_index: usize,
    chang_index: usize,
    qu_index: usize,
    kui_index: usize,
    yue_index: usize,
    lu_index: usize,
    yang_index: usize,
    tuo_index: usize,
    ma_index: usize,
    kong_index: usize,
    jie_index: usize,
    huo_index: usize,
    ling_index: usize,
    yearly_stem: HeavenlyStem,
    lang: Language,
) -> [Vec<Star>; 12] {
    let mut result: [Vec<Star>; 12] = Default::default();

    // (宫位索引, 星耀, 类型, 是否参与四化)
    let placements: [(usize, StarKey, StarType, bool); 14] = [
        (zuo_index, StarKey::ZuofuMin, StarType::Soft, true),
        (you_index, StarKey::YoubiMin, StarType::Soft, true),
        (chang_index, StarKey::WenchangMin, StarType::Soft, true),
        (qu_index, StarKey::WenquMin, StarType::Soft, true),
        (kui_index, StarKey::TiankuiMin, StarType::Soft, false),
        (yue_index, StarKey::TianyueMin, StarType::Soft, false),
        (lu_index, StarKey::LucunMin, StarType::Lucun, false),
        (ma_index, StarKey::TianmaMin, StarType::Tianma, false),
        (kong_index, StarKey::DikongMin, StarType::Tough, false),
        (jie_index, StarKey::DijieMin, StarType::Tough, false),
        (huo_index, StarKey::HuoxingMin, StarType::Tough, false),
        (ling_index, StarKey::LingxingMin, StarType::Tough, false),
        (yang_index, StarKey::QingyangMin, StarType::Tough, false),
        (tuo_index, StarKey::TuoluoMin, StarType::Tough, false),
    ];

    for (idx, key, star_type, with_mutagen) in placements {
        let stem = if with_mutagen {
            Some(yearly_stem)
        } else {
            None
        };
        result[idx].push(make_star(key, star_type, Scope::Origin, idx, stem, lang));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minor_stars_count() {
        let result = get_minor_stars(
            0,
            1,
            2,
            3,
            4,
            5,
            6,
            7,
            8,
            9,
            10,
            11,
            0,
            1,
            HeavenlyStem::Jia,
            Language::ZhCN,
        );
        let total: usize = result.iter().map(|v| v.len()).sum();
        assert_eq!(total, 14);
    }
}

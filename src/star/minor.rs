//! 辅星安放
//!
//! 将14颗辅星安放到12宫中。

use crate::data::heavenly_stems::get_heavenly_stem_info;
use crate::data::stars::{get_brightness_table, StarKey};
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

/// 获取14颗辅星的安放结果
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

    // 吉星（Soft）— 有四化检查
    let soft_with_mutagen: [(usize, StarKey); 4] = [
        (zuo_index, StarKey::ZuofuMin),
        (you_index, StarKey::YoubiMin),
        (chang_index, StarKey::WenchangMin),
        (qu_index, StarKey::WenquMin),
    ];

    for (idx, key) in soft_with_mutagen {
        let star = make_star(key, StarType::Soft, Scope::Origin, idx, Some(yearly_stem), lang);
        result[idx].push(star);
    }

    // 吉星（Soft）— 无四化
    let soft_no_mutagen: [(usize, StarKey); 2] = [
        (kui_index, StarKey::TiankuiMin),
        (yue_index, StarKey::TianyueMin),
    ];

    for (idx, key) in soft_no_mutagen {
        let star = make_star(key, StarType::Soft, Scope::Origin, idx, None, lang);
        result[idx].push(star);
    }

    // 禄存、天马 — 特殊类型，无四化
    {
        let star = make_star(StarKey::LucunMin, StarType::Lucun, Scope::Origin, lu_index, None, lang);
        result[lu_index].push(star);
    }
    {
        let star = make_star(StarKey::TianmaMin, StarType::Tianma, Scope::Origin, ma_index, None, lang);
        result[ma_index].push(star);
    }

    // 煞星（Tough）— 有四化检查
    let tough_with_mutagen: [(usize, StarKey); 2] = [
        (yang_index, StarKey::QingyangMin),
        (tuo_index, StarKey::TuoluoMin),
    ];

    for (idx, key) in tough_with_mutagen {
        let star = make_star(key, StarType::Tough, Scope::Origin, idx, Some(yearly_stem), lang);
        result[idx].push(star);
    }

    // 煞星（Tough）— 无四化
    let tough_no_mutagen: [(usize, StarKey); 4] = [
        (kong_index, StarKey::DikongMin),
        (jie_index, StarKey::DijieMin),
        (huo_index, StarKey::HuoxingMin),
        (ling_index, StarKey::LingxingMin),
    ];

    for (idx, key) in tough_no_mutagen {
        let star = make_star(key, StarType::Tough, Scope::Origin, idx, None, lang);
        result[idx].push(star);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minor_stars_count() {
        let result = get_minor_stars(
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1,
            HeavenlyStem::Jia,
            Language::ZhCN,
        );
        let total: usize = result.iter().map(|v| v.len()).sum();
        assert_eq!(total, 14);
    }
}

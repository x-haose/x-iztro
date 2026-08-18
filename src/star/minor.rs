//! 辅星安放
//!
//! 将14颗辅星安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;

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
    config: &Config,
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
        result[idx].push(crate::star::make_star(
            key,
            star_type,
            Scope::Origin,
            idx,
            stem,
            lang,
            config,
        ));
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
            &Config::default(),
        );
        let total: usize = result.iter().map(|v| v.len()).sum();
        assert_eq!(total, 14);
    }
}

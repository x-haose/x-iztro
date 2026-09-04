//! 主星安放
//!
//! 根据紫微和天府的起始宫位索引，将14颗主星安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;
use crate::utils::fix_index;

/// 紫微系主星与其相对紫微的逆行位次：安星宫位 = 紫微位 − 位次。
///
/// 反推的主星落宫剪枝（`astro::reverse`）也从本表取几何，剪枝与正排永不脱节。
pub(crate) const ZIWEI_GROUP: [(usize, StarKey); 6] = [
    (0, StarKey::ZiweiMaj),
    (1, StarKey::TianjiMaj),
    (3, StarKey::TaiyangMaj),
    (4, StarKey::WuquMaj),
    (5, StarKey::TiantongMaj),
    (8, StarKey::LianzhenMaj),
];

/// 天府系主星与其相对天府的顺行位次：安星宫位 = 天府位 + 位次。
///
/// 与 [`ZIWEI_GROUP`] 同为反推剪枝的几何来源。
pub(crate) const TIANFU_GROUP: [(usize, StarKey); 8] = [
    (0, StarKey::TianfuMaj),
    (1, StarKey::TaiyinMaj),
    (2, StarKey::TanlangMaj),
    (3, StarKey::JumenMaj),
    (4, StarKey::TianxiangMaj),
    (5, StarKey::TianliangMaj),
    (6, StarKey::QishaMaj),
    (10, StarKey::PojunMaj),
];

/// 获取14颗主星的安放结果
///
/// 紫微系6星（逆时针安放）+ 天府系8星（顺时针安放）
pub fn get_major_stars(
    ziwei_index: usize,
    tianfu_index: usize,
    yearly_stem: HeavenlyStem,
    lang: Language,
    config: &Config,
) -> [Vec<Star>; 12] {
    let mut result: [Vec<Star>; 12] = Default::default();

    // 紫微系（逆时针，从 ziwei_index 减位次）
    for (offset, key) in ZIWEI_GROUP {
        let idx = fix_index(ziwei_index as i32 - offset as i32, 12);
        let star = crate::star::make_star(
            key,
            StarType::Major,
            Scope::Origin,
            idx,
            Some(yearly_stem),
            lang,
            config,
        );
        result[idx].push(star);
    }

    // 天府系（顺时针，从 tianfu_index 加位次）
    for (offset, key) in TIANFU_GROUP {
        let idx = fix_index(tianfu_index as i32 + offset as i32, 12);
        let star = crate::star::make_star(
            key,
            StarType::Major,
            Scope::Origin,
            idx,
            Some(yearly_stem),
            lang,
            config,
        );
        result[idx].push(star);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_major_stars_count() {
        let result = get_major_stars(0, 0, HeavenlyStem::Jia, Language::ZhCN, &Config::default());
        let total: usize = result.iter().map(|v| v.len()).sum();
        // 14 major stars total
        assert_eq!(total, 14);
    }

    #[test]
    fn test_all_major_type() {
        let result = get_major_stars(3, 9, HeavenlyStem::Yi, Language::ZhCN, &Config::default());
        for palace in &result {
            for star in palace {
                assert_eq!(star.star_type, StarType::Major);
                assert_eq!(star.scope, Scope::Origin);
            }
        }
    }
}

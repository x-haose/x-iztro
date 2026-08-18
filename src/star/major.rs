//! 主星安放
//!
//! 根据紫微和天府的起始宫位索引，将14颗主星安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;
use crate::utils::fix_index;

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

    // 紫微系（逆时针，从 ziwei_index 减 i）
    // i=0: 紫微, i=1: 天机, i=2: SKIP, i=3: 太阳, i=4: 武曲, i=5: 天同,
    // i=6: SKIP, i=7: SKIP, i=8: 廉贞
    let ziwei_group: [(usize, StarKey); 6] = [
        (0, StarKey::ZiweiMaj),
        (1, StarKey::TianjiMaj),
        (3, StarKey::TaiyangMaj),
        (4, StarKey::WuquMaj),
        (5, StarKey::TiantongMaj),
        (8, StarKey::LianzhenMaj),
    ];

    for (offset, key) in ziwei_group {
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

    // 天府系（顺时针，从 tianfu_index 加 i）
    // i=0: 天府, i=1: 太阴, i=2: 贪狼, i=3: 巨门, i=4: 天相, i=5: 天梁,
    // i=6: 七杀, i=7: SKIP, i=8: SKIP, i=9: SKIP, i=10: 破军
    let tianfu_group: [(usize, StarKey); 8] = [
        (0, StarKey::TianfuMaj),
        (1, StarKey::TaiyinMaj),
        (2, StarKey::TanlangMaj),
        (3, StarKey::JumenMaj),
        (4, StarKey::TianxiangMaj),
        (5, StarKey::TianliangMaj),
        (6, StarKey::QishaMaj),
        (10, StarKey::PojunMaj),
    ];

    for (offset, key) in tianfu_group {
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

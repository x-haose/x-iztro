//! 流耀安放
//!
//! 长生12神、博士12神、岁前12神、将前12神

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::utils::{earthly_branch_to_palace_index as eb2pi, fix_index};

/// 判断性别阴阳与年支阴阳是否同性
///
/// Male=Yang, Female=Yin; 偶数地支索引=Yang, 奇数=Yin
fn same_yinyang(gender: Gender, yearly_branch: EarthlyBranch) -> bool {
    let gender_is_yang = gender == Gender::Male;
    let branch_is_yang = yearly_branch.index().is_multiple_of(2);
    gender_is_yang == branch_is_yang
}

/// 长生12神
///
/// 根据五行局确定起始宫位，根据性别阴阳与年支阴阳决定顺逆。
pub fn get_changsheng12(
    five_elements_class: FiveElementsClass,
    gender: Gender,
    yearly_branch: EarthlyBranch,
) -> [StarKey; 12] {
    use EarthlyBranch::*;

    // 起始宫位（由五行局决定）
    let start_branch = match five_elements_class {
        FiveElementsClass::Water2nd => Shen,
        FiveElementsClass::Wood3rd => Hai,
        FiveElementsClass::Metal4th => Si,
        FiveElementsClass::Earth5th => Shen,
        FiveElementsClass::Fire6th => Yin,
    };
    let start = eb2pi(start_branch);

    // 顺逆：同阴阳则顺时针（+i），异阴阳则逆时针（-i）
    let clockwise = same_yinyang(gender, yearly_branch);

    let stars = [
        StarKey::Changsheng,
        StarKey::Muyu,
        StarKey::Guandai,
        StarKey::Linguan,
        StarKey::Diwang,
        StarKey::Shuai,
        StarKey::Bing,
        StarKey::Si,
        StarKey::Mu,
        StarKey::Jue,
        StarKey::Tai,
        StarKey::Yang,
    ];

    let mut result = [StarKey::Changsheng; 12]; // placeholder init
    for (i, &key) in stars.iter().enumerate() {
        let idx = if clockwise {
            fix_index(start as i32 + i as i32, 12)
        } else {
            fix_index(start as i32 - i as i32, 12)
        };
        result[idx] = key;
    }

    result
}

/// 博士12神
///
/// 从禄存位置开始，根据性别阴阳与年支阴阳决定顺逆。
pub fn get_boshi12(lu_index: usize, gender: Gender, yearly_branch: EarthlyBranch) -> [StarKey; 12] {
    let clockwise = same_yinyang(gender, yearly_branch);

    let stars = [
        StarKey::Boshi,
        StarKey::Lishi,
        StarKey::Qinglong,
        StarKey::Xiaohao,
        StarKey::Jiangjun,
        StarKey::Zhoushu,
        StarKey::Faylian,
        StarKey::Xishen,
        StarKey::Bingfu,
        StarKey::Dahao,
        StarKey::Fubing,
        StarKey::Guanfu,
    ];

    let mut result = [StarKey::Boshi; 12]; // placeholder init
    for (i, &key) in stars.iter().enumerate() {
        let idx = if clockwise {
            fix_index(lu_index as i32 + i as i32, 12)
        } else {
            fix_index(lu_index as i32 - i as i32, 12)
        };
        result[idx] = key;
    }

    result
}

/// 岁前12神 + 将前12神
///
/// 返回 (suiqian12, jiangqian12)
pub fn get_yearly12(
    yearly_branch: EarthlyBranch,
    algorithm: Algorithm,
) -> ([StarKey; 12], [StarKey; 12]) {
    use EarthlyBranch::*;

    // ===== 岁前12神 =====
    // 从年支宫位开始顺时针安放
    let start_sui = eb2pi(yearly_branch);

    let suiqian_stars: [StarKey; 12] = if algorithm == Algorithm::Zhongzhou {
        [
            StarKey::Suijian,
            StarKey::Huiqi,
            StarKey::Sangmen,
            StarKey::Guansuo,
            StarKey::Gwanfu,
            StarKey::Xiaohao,
            StarKey::Suipo, // 中州派用岁破替代大耗
            StarKey::Longde,
            StarKey::Baihu,
            StarKey::Tiande,
            StarKey::Diaoke,
            StarKey::Bingfu,
        ]
    } else {
        [
            StarKey::Suijian,
            StarKey::Huiqi,
            StarKey::Sangmen,
            StarKey::Guansuo,
            StarKey::Gwanfu,
            StarKey::Xiaohao,
            StarKey::Dahao,
            StarKey::Longde,
            StarKey::Baihu,
            StarKey::Tiande,
            StarKey::Diaoke,
            StarKey::Bingfu,
        ]
    };

    let mut suiqian12 = [StarKey::Suijian; 12];
    for (i, &key) in suiqian_stars.iter().enumerate() {
        let idx = fix_index(start_sui as i32 + i as i32, 12);
        suiqian12[idx] = key;
    }

    // ===== 将前12神 =====
    // 起始地支按年支分组
    let jiang_start_branch = match yearly_branch {
        Yin | Wu | Xu => Wu,
        Shen | Zi | Chen => Zi,
        Si | You | Chou => You,
        Hai | Mao | Wei => Mao,
    };
    let start_jiang = eb2pi(jiang_start_branch);

    let jiangqian_stars = [
        StarKey::Jiangxing,
        StarKey::Panan,
        StarKey::Suiyi,
        StarKey::Xiishen,
        StarKey::Huagai,
        StarKey::Jiesha,
        StarKey::Zhaisha,
        StarKey::Tiansha,
        StarKey::Zhibei,
        StarKey::Xianchi,
        StarKey::Yuesha,
        StarKey::Wangshen,
    ];

    let mut jiangqian12 = [StarKey::Jiangxing; 12];
    for (i, &key) in jiangqian_stars.iter().enumerate() {
        let idx = fix_index(start_jiang as i32 + i as i32, 12);
        jiangqian12[idx] = key;
    }

    (suiqian12, jiangqian12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_changsheng12_all_unique() {
        let result = get_changsheng12(FiveElementsClass::Water2nd, Gender::Male, EarthlyBranch::Zi);
        // All 12 should be distinct
        let mut seen = std::collections::HashSet::new();
        for key in &result {
            assert!(seen.insert(key));
        }
    }

    #[test]
    fn test_boshi12_all_unique() {
        let result = get_boshi12(0, Gender::Male, EarthlyBranch::Zi);
        let mut seen = std::collections::HashSet::new();
        for key in &result {
            assert!(seen.insert(key));
        }
    }

    #[test]
    fn test_yearly12_all_unique() {
        let (sui, jiang) = get_yearly12(EarthlyBranch::Zi, Algorithm::Default);
        let mut seen_sui = std::collections::HashSet::new();
        for key in &sui {
            assert!(seen_sui.insert(key));
        }
        let mut seen_jiang = std::collections::HashSet::new();
        for key in &jiang {
            assert!(seen_jiang.insert(key));
        }
    }

    #[test]
    fn test_yearly12_zhongzhou_has_suipo() {
        let (sui, _) = get_yearly12(EarthlyBranch::Zi, Algorithm::Zhongzhou);
        assert!(sui.contains(&StarKey::Suipo));
        assert!(!sui.contains(&StarKey::Dahao));
    }

    #[test]
    fn test_yearly12_default_has_dahao() {
        let (sui, _) = get_yearly12(EarthlyBranch::Zi, Algorithm::Default);
        assert!(sui.contains(&StarKey::Dahao));
        assert!(!sui.contains(&StarKey::Suipo));
    }
}

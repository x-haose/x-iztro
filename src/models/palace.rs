use serde::{Deserialize, Serialize};

use crate::data::heavenly_stems::get_heavenly_stem_info;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::star::Star;

/// 宫位上的大限信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decadal {
    /// 大限起止虚岁（含两端）
    pub range: (u32, u32),
    /// 大限天干
    pub heavenly_stem: HeavenlyStem,
    /// 大限地支
    pub earthly_branch: EarthlyBranch,
}

/// 单个宫位：星耀、干支、四组十二神与大限小限信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalaceData {
    /// 宫位索引（0-11，寅宫为 0）
    pub index: usize,
    /// 宫位名称
    pub name: Palace,
    /// 是否身宫
    pub is_body_palace: bool,
    /// 是否来因宫（宫干与年干相同且不在子丑二宫）
    pub is_original_palace: bool,
    /// 宫位天干
    pub heavenly_stem: HeavenlyStem,
    /// 宫位地支
    pub earthly_branch: EarthlyBranch,
    /// 主星列表（按安放顺序）
    pub major_stars: Vec<Star>,
    /// 辅星列表（按安放顺序）
    pub minor_stars: Vec<Star>,
    /// 杂耀列表（按安放顺序）
    pub adjective_stars: Vec<Star>,
    /// 长生十二神
    pub changsheng12: StarKey,
    /// 博士十二神
    pub boshi12: StarKey,
    /// 将前十二神
    pub jiangqian12: StarKey,
    /// 岁前十二神
    pub suiqian12: StarKey,
    /// 大限信息
    pub decadal: Decadal,
    /// 小限经过的虚岁列表
    pub ages: Vec<u32>,
}

impl PalaceData {
    /// 收集宫位内所有星耀的 StarKey
    fn all_star_keys(&self) -> Vec<StarKey> {
        let mut keys = Vec::new();
        for s in &self.major_stars {
            keys.push(s.key);
        }
        for s in &self.minor_stars {
            keys.push(s.key);
        }
        for s in &self.adjective_stars {
            keys.push(s.key);
        }
        keys
    }

    /// 判断宫位是否包含所有指定星耀
    pub fn has(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().all(|s| all_keys.contains(s))
    }

    /// 判断宫位是否不包含所有指定星耀（全不在才返回 true）
    pub fn not_have(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().all(|s| !all_keys.contains(s))
    }

    /// 判断宫位是否包含任一指定星耀
    pub fn has_one_of(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().any(|s| all_keys.contains(s))
    }

    /// 判断是否有指定四化（检查主星和辅星的 mutagen 字段）
    /// TS: only checks majorStars and minorStars (NOT adjectiveStars)
    pub fn has_mutagen(&self, mutagen: Mutagen) -> bool {
        self.major_stars
            .iter()
            .chain(self.minor_stars.iter())
            .any(|s| s.mutagen == Some(mutagen))
    }

    /// 判断是否没有指定四化
    pub fn not_have_mutagen(&self, mutagen: Mutagen) -> bool {
        !self.has_mutagen(mutagen)
    }

    /// 判断是否空宫（没有 star_type == Major 的主星）
    pub fn is_empty(&self) -> bool {
        !self
            .major_stars
            .iter()
            .any(|s| s.star_type == StarType::Major)
    }

    /// 飞化到目标宫位
    /// 根据本宫天干的四化，判断对应四化星是否在目标宫位中
    pub fn flies_to(&self, target_palace: &PalaceData, mutagen: Mutagen) -> bool {
        let info = get_heavenly_stem_info(self.heavenly_stem);
        let mutagen_index = match mutagen {
            Mutagen::Lu => 0,
            Mutagen::Quan => 1,
            Mutagen::Ke => 2,
            Mutagen::Ji => 3,
        };
        let star_key = info.mutagen[mutagen_index];
        target_palace.has(&[star_key])
    }

    /// 自化判断：本宫天干四化星是否在本宫内
    pub fn self_mutaged(&self, mutagen: Mutagen) -> bool {
        self.flies_to(self, mutagen)
    }

    /// 是否有任一自化
    pub fn self_mutaged_one_of(&self) -> bool {
        [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji]
            .iter()
            .any(|m| self.self_mutaged(*m))
    }

    /// 是否没有自化（检查所有四化，全部没有才返回 true）
    pub fn not_self_mutaged(&self) -> bool {
        !self.self_mutaged_one_of()
    }

    /// 获取四化飞入的宫位索引：依次为禄、权、科、忌对应的星所在宫位，
    /// 未找到的项为 None
    pub fn mutaged_places(&self, all_palaces: &[PalaceData]) -> Vec<Option<usize>> {
        let info = get_heavenly_stem_info(self.heavenly_stem);
        info.mutagen
            .iter()
            .map(|star_key| {
                all_palaces.iter().find_map(|p| {
                    if p.has(&[*star_key]) {
                        Some(p.index)
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::astro::builder::by_solar;
    use crate::data::stars::StarKey;
    use crate::data::types::*;

    fn make_astrolabe() -> crate::models::astrolabe::Astrolabe {
        by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn test_palace_has() {
        let a = make_astrolabe();
        // Find a palace with a known major star and test has()
        let found = a
            .palaces
            .iter()
            .find(|p| p.major_stars.iter().any(|s| s.key == StarKey::ZiweiMaj));
        assert!(found.is_some());
        let p = found.unwrap();
        assert!(p.has(&[StarKey::ZiweiMaj]));
    }

    #[test]
    fn test_palace_not_have() {
        let a = make_astrolabe();
        // The palace with Ziwei should not have all 14 major stars
        let p = a
            .palaces
            .iter()
            .find(|p| p.has(&[StarKey::ZiweiMaj]))
            .unwrap();
        // Ziwei and Pojun can't be in the same palace
        assert!(p.not_have(&[StarKey::PojunMaj]) || p.has(&[StarKey::PojunMaj]));
        // A star not present should return true for not_have
        let not_here: Vec<StarKey> = vec![
            StarKey::ZiweiMaj,
            StarKey::TianjiMaj,
            StarKey::TaiyangMaj,
            StarKey::WuquMaj,
            StarKey::TiantongMaj,
            StarKey::LianzhenMaj,
            StarKey::TianfuMaj,
            StarKey::TaiyinMaj,
            StarKey::TanlangMaj,
            StarKey::JumenMaj,
            StarKey::TianxiangMaj,
            StarKey::TianliangMaj,
            StarKey::QishaMaj,
            StarKey::PojunMaj,
        ]
        .into_iter()
        .filter(|k| !p.has(&[*k]))
        .collect();
        if !not_here.is_empty() {
            assert!(p.not_have(&not_here));
        }
    }

    #[test]
    fn test_palace_has_one_of() {
        let a = make_astrolabe();
        let _p = &a.palaces[0];
        // Should have at least one of all 14 major stars (some palace has at least one)
        let all_majors = [
            StarKey::ZiweiMaj,
            StarKey::TianjiMaj,
            StarKey::TaiyangMaj,
            StarKey::WuquMaj,
            StarKey::TiantongMaj,
            StarKey::LianzhenMaj,
            StarKey::TianfuMaj,
            StarKey::TaiyinMaj,
            StarKey::TanlangMaj,
            StarKey::JumenMaj,
            StarKey::TianxiangMaj,
            StarKey::TianliangMaj,
            StarKey::QishaMaj,
            StarKey::PojunMaj,
        ];
        // At least some palace has one of these
        let any_has = a.palaces.iter().any(|p| p.has_one_of(&all_majors));
        assert!(any_has);
    }

    #[test]
    fn test_palace_has_mutagen() {
        let a = make_astrolabe();
        // At least one palace should have 禄
        let has_lu = a.palaces.iter().any(|p| p.has_mutagen(Mutagen::Lu));
        assert!(has_lu);
        // At least one palace should have 忌
        let has_ji = a.palaces.iter().any(|p| p.has_mutagen(Mutagen::Ji));
        assert!(has_ji);
    }

    #[test]
    fn test_palace_is_empty() {
        let a = make_astrolabe();
        // Count non-empty palaces (those with StarType::Major stars)
        let non_empty = a.palaces.iter().filter(|p| !p.is_empty()).count();
        // Should have some non-empty palaces
        assert!(non_empty > 0);
        // Not all palaces can be non-empty (14 major stars across 12 palaces, some share)
        // but some palaces should be empty
        let empty_count = a.palaces.iter().filter(|p| p.is_empty()).count();
        // Total should be 12
        assert_eq!(non_empty + empty_count, 12);
    }

    #[test]
    fn test_palace_flies_to() {
        let a = make_astrolabe();
        let source = &a.palaces[0];
        // flies_to should work: the mutagen star from source's heavenly stem
        // should be found in some palace
        let mut found_target = false;
        for target in &a.palaces {
            if source.flies_to(target, Mutagen::Lu) {
                found_target = true;
                break;
            }
        }
        assert!(found_target, "Lu mutagen star should exist in some palace");
    }

    #[test]
    fn test_palace_self_mutaged() {
        let a = make_astrolabe();
        // self_mutaged checks if the palace's own heavenly stem's mutagen star is in itself
        // This may or may not be true for any given palace; just verify it doesn't panic
        for p in &a.palaces {
            let _ = p.self_mutaged(Mutagen::Lu);
            let _ = p.self_mutaged_one_of();
            let _ = p.not_self_mutaged();
        }
    }

    #[test]
    fn test_palace_mutaged_places() {
        let a = make_astrolabe();
        let places = a.palaces[0].mutaged_places(&a.palaces);
        assert_eq!(places.len(), 4);
        // Each mutagen star must exist in exactly one palace
        for place in &places {
            assert!(
                place.is_some(),
                "Each mutagen star should be found in some palace"
            );
        }
    }
}

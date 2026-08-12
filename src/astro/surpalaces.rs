use crate::data::stars::StarKey;
use crate::data::types::Mutagen;
use crate::models::palace::PalaceData;

/// 三方四正宫位
pub struct SurroundedPalaces<'a> {
    /// 本宫
    pub target: &'a PalaceData,
    /// 对宫 (+6)
    pub opposite: &'a PalaceData,
    /// 财帛位 (+8, modular)
    pub wealth: &'a PalaceData,
    /// 官禄位 (+4)
    pub career: &'a PalaceData,
}

impl<'a> SurroundedPalaces<'a> {
    /// 三方四正是否包含所有指定星耀
    pub fn have(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().all(|s| all_keys.contains(s))
    }

    /// 三方四正是否不含所有指定星耀
    pub fn not_have(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().all(|s| !all_keys.contains(s))
    }

    /// 三方四正是否包含任一指定星耀
    pub fn have_one_of(&self, stars: &[StarKey]) -> bool {
        let all_keys = self.all_star_keys();
        stars.iter().any(|s| all_keys.contains(s))
    }

    /// 三方四正是否有指定四化
    pub fn have_mutagen(&self, mutagen: Mutagen) -> bool {
        self.target.has_mutagen(mutagen)
            || self.opposite.has_mutagen(mutagen)
            || self.wealth.has_mutagen(mutagen)
            || self.career.has_mutagen(mutagen)
    }

    /// 三方四正是否没有指定四化
    pub fn not_have_mutagen(&self, mutagen: Mutagen) -> bool {
        !self.have_mutagen(mutagen)
    }

    /// 收集三方四正所有星耀的 StarKey
    fn all_star_keys(&self) -> Vec<StarKey> {
        let palaces = [self.target, self.opposite, self.wealth, self.career];
        let mut keys = Vec::new();
        for p in &palaces {
            for s in &p.major_stars {
                keys.push(s.key);
            }
            for s in &p.minor_stars {
                keys.push(s.key);
            }
            for s in &p.adjective_stars {
                keys.push(s.key);
            }
        }
        keys
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
    fn test_surrounded_palaces_structure() {
        let a = make_astrolabe();
        let sp = a.surrounded_palaces(0);
        // target is palace 0
        assert_eq!(sp.target.index, 0);
        // opposite is palace 6
        assert_eq!(sp.opposite.index, 6);
        // career is palace 4
        assert_eq!(sp.career.index, 4);
        // wealth is palace 8
        assert_eq!(sp.wealth.index, 8);
    }

    #[test]
    fn test_surrounded_palaces_wrapping() {
        let a = make_astrolabe();
        let sp = a.surrounded_palaces(10);
        assert_eq!(sp.target.index, 10);
        // opposite: (10+6) % 12 = 4
        assert_eq!(sp.opposite.index, 4);
        // career: (10+4) % 12 = 2
        assert_eq!(sp.career.index, 2);
        // wealth: (10+8) % 12 = 6
        assert_eq!(sp.wealth.index, 6);
    }

    #[test]
    fn test_surrounded_palaces_have() {
        let a = make_astrolabe();
        // The surrounded palaces of palace 0 cover 4 palaces,
        // so they should have many stars combined
        let sp = a.surrounded_palaces(0);
        // At least one major star should be present across 4 palaces
        assert!(sp.have_one_of(&[
            StarKey::ZiweiMaj,
            StarKey::TianjiMaj,
            StarKey::TaiyangMaj,
            StarKey::TianfuMaj,
        ]));
    }

    #[test]
    fn test_surrounded_palaces_not_have() {
        let a = make_astrolabe();
        let sp = a.surrounded_palaces(0);
        // Collect all star keys in the surrounded palaces
        // Then pick a star NOT present to test not_have
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
        let missing: Vec<StarKey> = all_majors
            .iter()
            .filter(|k| !sp.have(&[**k]))
            .copied()
            .collect();
        if !missing.is_empty() {
            assert!(sp.not_have(&missing));
        }
    }

    #[test]
    fn test_surrounded_palaces_mutagen() {
        let a = make_astrolabe();
        // Check across all 12 positions — at least one should have Lu mutagen
        let any_has_lu = (0..12).any(|i| a.surrounded_palaces(i).have_mutagen(Mutagen::Lu));
        assert!(any_has_lu);
        let any_has_ji = (0..12).any(|i| a.surrounded_palaces(i).have_mutagen(Mutagen::Ji));
        assert!(any_has_ji);
    }

    #[test]
    fn test_surrounded_palaces_not_have_mutagen() {
        let a = make_astrolabe();
        let sp = a.surrounded_palaces(0);
        // not_have_mutagen is the inverse of have_mutagen
        for m in &[Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            assert_eq!(sp.not_have_mutagen(*m), !sp.have_mutagen(*m));
        }
    }
}

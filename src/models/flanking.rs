use crate::data::stars::StarKey;
use crate::data::types::Mutagen;
use crate::models::astrolabe::Astrolabe;
use crate::models::palace::PalaceData;

/// 夹宫：目标宫前后相邻的两宫。
///
/// 十二宫首尾相连，故首宫的前一宫是末宫、末宫的后一宫是首宫。
/// 星耀与四化的判定一律在**两宫合计**的集合上做：`have` 要求两宫合起来含全部目标星，
/// 不要求同在一宫。
pub struct FlankingPalaces<'a> {
    /// 前一宫（索引 -1）
    pub previous: &'a PalaceData,
    /// 后一宫（索引 +1）
    pub next: &'a PalaceData,
    /// 所属星盘：文本投影要查星落宫与宫干飞化，须回指全盘
    pub(crate) astrolabe: &'a Astrolabe,
}

impl<'a> FlankingPalaces<'a> {
    /// 所属星盘。
    pub fn astrolabe(&self) -> &'a Astrolabe {
        self.astrolabe
    }

    /// 两个夹宫合计是否包含全部指定星耀。
    pub fn have(&self, stars: &[StarKey]) -> bool {
        let keys = self.all_star_keys();
        stars.iter().all(|s| keys.contains(s))
    }

    /// 两个夹宫是否都不含任何指定星耀。
    pub fn not_have(&self, stars: &[StarKey]) -> bool {
        let keys = self.all_star_keys();
        stars.iter().all(|s| !keys.contains(s))
    }

    /// 两个夹宫合计是否包含任一指定星耀。
    pub fn have_one_of(&self, stars: &[StarKey]) -> bool {
        let keys = self.all_star_keys();
        stars.iter().any(|s| keys.contains(s))
    }

    /// 任一夹宫是否有指定四化。
    pub fn have_mutagen(&self, mutagen: Mutagen) -> bool {
        self.previous.has_mutagen(mutagen) || self.next.has_mutagen(mutagen)
    }

    /// 两个夹宫是否都没有指定四化。
    pub fn not_have_mutagen(&self, mutagen: Mutagen) -> bool {
        !self.have_mutagen(mutagen)
    }

    /// 两宫全部星耀的标识。
    fn all_star_keys(&self) -> Vec<StarKey> {
        let mut keys = Vec::new();
        for p in [self.previous, self.next] {
            for group in [&p.major_stars, &p.minor_stars, &p.adjective_stars] {
                keys.extend(group.iter().map(|s| s.key));
            }
        }
        keys
    }
}

#[cfg(test)]
mod tests {
    use crate::astro::builder::by_solar;
    use crate::data::types::*;
    use crate::models::astrolabe::PalaceTarget;

    fn chart() -> crate::models::astrolabe::Astrolabe {
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

    /// 夹宫取的是索引相邻的两宫，且十二宫首尾相连。
    #[test]
    fn flanking_wraps_around_twelve_palaces() {
        let a = chart();
        for i in 0..12usize {
            let f = a
                .flanking_palaces(PalaceTarget::Index(i))
                .expect("宫位存在");
            assert_eq!(f.previous.index, (i + 11) % 12, "第 {i} 宫的前一宫");
            assert_eq!(f.next.index, (i + 1) % 12, "第 {i} 宫的后一宫");
        }
    }

    /// 星耀判定在两宫合计的集合上做：分处两宫的两颗星，`have` 也成立。
    #[test]
    fn have_spans_both_palaces() {
        let a = chart();
        let f = a
            .flanking_palaces(PalaceTarget::Name(Palace::Soul))
            .expect("命宫存在");
        let (Some(from_prev), Some(from_next)) = (
            f.previous.major_stars.first().map(|s| s.key),
            f.next.major_stars.first().map(|s| s.key),
        ) else {
            return;
        };
        assert!(f.have(&[from_prev, from_next]), "分处两宫的星应算作被夹");
        assert!(f.have_one_of(&[from_prev]));
        assert!(!f.not_have(&[from_prev]));
    }
}

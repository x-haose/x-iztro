use serde::{Deserialize, Serialize};

use crate::astro::surpalaces::SurroundedPalaces;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::utils::fix_index;

/// 数字化农历日期
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RawLunarDate {
    /// 农历年
    pub lunar_year: i64,
    /// 农历月（1-12，闰月与否见 is_leap）
    pub lunar_month: u32,
    /// 农历日（1-30）
    pub lunar_day: u32,
    /// 是否闰月
    pub is_leap: bool,
}

/// 四柱干支（枚举形式）
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RawChineseDate {
    /// 年柱 [天干, 地支]
    pub yearly: (HeavenlyStem, EarthlyBranch),
    /// 月柱 [天干, 地支]
    pub monthly: (HeavenlyStem, EarthlyBranch),
    /// 日柱 [天干, 地支]
    pub daily: (HeavenlyStem, EarthlyBranch),
    /// 时柱 [天干, 地支]
    pub hourly: (HeavenlyStem, EarthlyBranch),
}

/// 结构化的出生日期信息（lunar_date / chinese_date 展示串的数据形式）
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RawDates {
    /// 数字化农历生日
    pub lunar_date: RawLunarDate,
    /// 四柱干支
    pub chinese_date: RawChineseDate,
}

/// 完整星盘：排盘入口的返回值，承载十二宫与全部命盘信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Astrolabe {
    /// 性别
    pub gender: Gender,
    /// 阳历日期（与排盘入参一致，"YYYY-M-D"）
    pub solar_date: String,
    /// 农历日期中文表示（如「二〇〇〇年七月十七」）
    pub lunar_date: String,
    /// 干支纪日四柱展示串（按排盘语言）
    pub chinese_date: String,
    /// 时辰名称（按排盘语言，如「寅时」）
    pub time: String,
    /// 时辰对应的时间段（如「03:00~05:00」）
    pub time_range: String,
    /// 星座（按排盘语言，如「狮子座」）
    pub sign: String,
    /// 生肖（按排盘语言，按年支）
    pub zodiac: String,
    /// 命宫地支
    pub earthly_branch_of_soul_palace: EarthlyBranch,
    /// 身宫地支
    pub earthly_branch_of_body_palace: EarthlyBranch,
    /// 命主星
    pub soul: StarKey,
    /// 身主星
    pub body: StarKey,
    /// 五行局
    pub five_elements_class: FiveElementsClass,
    /// 十二宫数据（索引 0 为寅宫）
    pub palaces: Vec<PalaceData>,
    /// 结构化的农历生日与四柱干支
    pub raw_dates: RawDates,
    /// 出生时辰索引 (0=早子, 1=丑, ..., 12=晚子)，晚子归当天的配置下仍保留原始值
    pub time_index: u8,
    /// 是否修正闰月（闰月下半月按下月安星）
    pub fix_leap: bool,
    /// 排盘输出语言
    pub language: Language,
    /// 排盘配置（运限的虚岁分界、干支分界与派别差异依赖它）
    pub config: Config,
}

/// 宫位定位方式。
///
/// 除索引与宫名外，还支持按身宫、来因宫这两个标记定位 —— 它们不是固定的某一宫，
/// 而是排盘算出来落在哪一宫。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PalaceTarget {
    /// 盘上位置索引（0-11，寅宫为 0）
    Index(usize),
    /// 宫名
    Name(Palace),
    /// 身宫所在的那一宫
    Body,
    /// 来因宫所在的那一宫（可能不存在）
    Original,
}

impl From<usize> for PalaceTarget {
    fn from(index: usize) -> Self {
        PalaceTarget::Index(index)
    }
}

impl From<Palace> for PalaceTarget {
    fn from(name: Palace) -> Self {
        PalaceTarget::Name(name)
    }
}

/// 一个宫位连同它所属的整张星盘。
///
/// 由 [`Astrolabe::palace`] 等入口返回。解引用即得到 [`PalaceData`] 本身，
/// 另外提供需要星盘上下文才能回答的查询：飞星的目标宫可以直接写索引或宫名，
/// 四化落宫不必再把十二宫传进来。
#[derive(Clone, Copy)]
pub struct PalaceRef<'a> {
    palace: &'a PalaceData,
    astrolabe: &'a Astrolabe,
}

impl<'a> std::ops::Deref for PalaceRef<'a> {
    type Target = PalaceData;

    fn deref(&self) -> &Self::Target {
        self.palace
    }
}

impl<'a> PalaceRef<'a> {
    /// 宫位数据本身，生命周期与星盘一致。
    pub fn data(&self) -> &'a PalaceData {
        self.palace
    }

    /// 宫位所属的星盘。
    pub fn astrolabe(&self) -> &'a Astrolabe {
        self.astrolabe
    }

    /// 本宫的对宫。
    pub fn opposite_palace(&self) -> PalaceRef<'a> {
        self.astrolabe
            .palace_at(fix_index(self.palace.index as i32 + 6, 12))
    }

    /// 本宫的三方四正。
    pub fn surrounded_palaces(&self) -> SurroundedPalaces<'a> {
        self.astrolabe.surrounded_at(self.palace.index)
    }

    /// 飞化到目标宫：本宫天干的指定四化星是否**全部**落在目标宫内。
    ///
    /// 目标宫可写索引、宫名、身宫或来因宫；定位不到或四化列表为空时返回 false。
    pub fn flies_to(&self, target: impl Into<PalaceTarget>, mutagens: &[Mutagen]) -> bool {
        match self.astrolabe.palace(target) {
            Some(to) => self.palace.flies_to(&to, mutagens),
            None => false,
        }
    }

    /// 飞化到目标宫：本宫天干的指定四化星是否有**任一颗**落在目标宫内。
    ///
    /// 目标宫定位不到时返回 false；定位得到而四化列表为空时返回 true。
    pub fn flies_one_of_to(&self, target: impl Into<PalaceTarget>, mutagens: &[Mutagen]) -> bool {
        match self.astrolabe.palace(target) {
            Some(to) => self.palace.flies_one_of_to(&to, mutagens),
            None => false,
        }
    }

    /// 未飞化到目标宫：本宫天干的指定四化星是否**一颗都不**落在目标宫内。
    ///
    /// 目标宫定位不到时返回 false；定位得到而四化列表为空时返回 true。
    pub fn not_fly_to(&self, target: impl Into<PalaceTarget>, mutagens: &[Mutagen]) -> bool {
        match self.astrolabe.palace(target) {
            Some(to) => self.palace.not_fly_to(&to, mutagens),
            None => false,
        }
    }

    /// 本宫天干四化分别飞入的宫位，依次为禄、权、科、忌；
    /// 该四化星不在盘上时对应项为 `None`。
    pub fn mutaged_places(&self) -> Vec<Option<PalaceRef<'a>>> {
        self.palace
            .mutaged_places(&self.astrolabe.palaces)
            .into_iter()
            .map(|idx| idx.map(|i| self.astrolabe.palace_at(i)))
            .collect()
    }
}

/// 一颗星耀连同它所在的宫位与整张星盘。
///
/// 由 [`Astrolabe::star`] 返回。解引用即得到 [`Star`] 本身，
/// 另外提供需要星盘上下文才能回答的关系查询。
#[derive(Clone, Copy)]
pub struct StarRef<'a> {
    star: &'a Star,
    palace: PalaceRef<'a>,
}

impl<'a> std::ops::Deref for StarRef<'a> {
    type Target = Star;

    fn deref(&self) -> &Self::Target {
        self.star
    }
}

impl<'a> StarRef<'a> {
    /// 星耀所在的宫位。
    pub fn palace(&self) -> PalaceRef<'a> {
        self.palace
    }

    /// 星耀所在宫位的对宫。
    pub fn opposite_palace(&self) -> PalaceRef<'a> {
        self.palace.opposite_palace()
    }

    /// 星耀所在宫位的三方四正。
    pub fn surrounded_palaces(&self) -> SurroundedPalaces<'a> {
        self.palace.surrounded_palaces()
    }
}

impl Astrolabe {
    /// 按索引、宫名、身宫或来因宫定位一个宫位；定位不到返回 `None`。
    ///
    /// ```ignore
    /// astrolabe.palace(0);                     // 寅宫
    /// astrolabe.palace(Palace::Soul);          // 命宫
    /// astrolabe.palace(PalaceTarget::Body);    // 身宫
    /// ```
    pub fn palace(&self, target: impl Into<PalaceTarget>) -> Option<PalaceRef<'_>> {
        let palace = match target.into() {
            PalaceTarget::Index(index) => self.palaces.get(index),
            PalaceTarget::Name(name) => self.palaces.iter().find(|p| p.name == name),
            PalaceTarget::Body => self.palaces.iter().find(|p| p.is_body_palace),
            PalaceTarget::Original => self.palaces.iter().find(|p| p.is_original_palace),
        }?;
        Some(PalaceRef {
            palace,
            astrolabe: self,
        })
    }

    /// 按索引取宫位视图，索引越界时对 12 取模。
    pub(crate) fn palace_at(&self, index: usize) -> PalaceRef<'_> {
        PalaceRef {
            palace: &self.palaces[fix_index(index as i32, 12)],
            astrolabe: self,
        }
    }

    /// 按索引取三方四正，索引越界时对 12 取模。
    fn surrounded_at(&self, index: usize) -> SurroundedPalaces<'_> {
        SurroundedPalaces {
            target: &self.palaces[fix_index(index as i32, 12)],
            opposite: &self.palaces[fix_index(index as i32 + 6, 12)],
            career: &self.palaces[fix_index(index as i32 + 4, 12)],
            wealth: &self.palaces[fix_index(index as i32 + 8, 12)],
        }
    }

    /// 取指定宫位的三方四正：本宫、对宫、官禄位、财帛位；定位不到返回 `None`。
    pub fn surrounded_palaces(
        &self,
        target: impl Into<PalaceTarget>,
    ) -> Option<SurroundedPalaces<'_>> {
        let palace = self.palace(target)?;
        Some(self.surrounded_at(palace.index))
    }

    /// 指定宫位的三方四正是否**全部**包含列出的星耀。
    pub fn is_surrounded(&self, target: impl Into<PalaceTarget>, stars: &[StarKey]) -> bool {
        self.surrounded_palaces(target)
            .is_some_and(|sp| sp.have(stars))
    }

    /// 指定宫位的三方四正是否包含列出星耀中的**任意一颗**。
    pub fn is_surrounded_one_of(&self, target: impl Into<PalaceTarget>, stars: &[StarKey]) -> bool {
        self.surrounded_palaces(target)
            .is_some_and(|sp| sp.have_one_of(stars))
    }

    /// 指定宫位的三方四正是否**一颗都不**包含列出的星耀。
    pub fn not_surrounded(&self, target: impl Into<PalaceTarget>, stars: &[StarKey]) -> bool {
        self.surrounded_palaces(target)
            .is_some_and(|sp| sp.not_have(stars))
    }

    /// 查找星耀，返回它连同所在宫位与星盘的引用；不在盘上返回 `None`。
    pub fn star(&self, key: StarKey) -> Option<StarRef<'_>> {
        for p in &self.palaces {
            for s in p
                .major_stars
                .iter()
                .chain(p.minor_stars.iter())
                .chain(p.adjective_stars.iter())
            {
                if s.key == key {
                    return Some(StarRef {
                        star: s,
                        palace: self.palace_at(p.index),
                    });
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::stars::StarKey;

    fn make_astrolabe() -> super::Astrolabe {
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
    fn test_palace_by_index() {
        let a = make_astrolabe();
        for i in 0..12 {
            assert_eq!(a.palace(i).unwrap().index, i);
        }
        assert!(a.palace(12).is_none());
    }

    #[test]
    fn test_palace_by_body_and_original() {
        let a = make_astrolabe();

        let body = a.palace(PalaceTarget::Body).expect("每张盘必有身宫");
        assert!(body.is_body_palace);

        // 来因宫可能不存在，存在时其标记必须为真
        if let Some(original) = a.palace(PalaceTarget::Original) {
            assert!(original.is_original_palace);
        }
    }

    #[test]
    fn test_palace_by_name() {
        let a = make_astrolabe();
        let soul = a.palace(Palace::Soul);
        assert!(soul.is_some());
        assert_eq!(soul.unwrap().name, Palace::Soul);

        // All 12 palace names should be findable
        let names = [
            Palace::Soul,
            Palace::Parents,
            Palace::Spirit,
            Palace::Property,
            Palace::Career,
            Palace::Friends,
            Palace::Surface,
            Palace::Health,
            Palace::Wealth,
            Palace::Children,
            Palace::Spouse,
            Palace::Siblings,
        ];
        for name in &names {
            assert!(a.palace(*name).is_some());
        }
    }

    #[test]
    fn test_star_lookup() {
        let a = make_astrolabe();
        // Every major star should be findable
        let major_keys = [
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
        for key in &major_keys {
            let star = a.star(*key);
            assert!(star.is_some(), "Star {:?} should be found", key);
            let star = star.unwrap();
            assert_eq!(star.key, *key);
            // The palace should contain this star
            assert!(star.palace().has(&[*key]));
            // 对宫恒为本宫 +6
            assert_eq!(
                star.opposite_palace().index,
                (star.palace().index + 6) % 12,
                "Star {:?} opposite palace mismatch",
                key
            );
            // 三方四正的本宫即星耀所在宫
            assert_eq!(star.surrounded_palaces().target.index, star.palace().index);
        }
    }

    #[test]
    fn test_palace_ref_back_reference() {
        let a = make_astrolabe();
        let soul = a.palace(Palace::Soul).unwrap();

        // 回引拿到的是同一张盘
        assert_eq!(soul.astrolabe().solar_date, a.solar_date);
        // 解引用后即宫位数据本身
        assert_eq!(soul.name, Palace::Soul);
        assert_eq!(soul.opposite_palace().index, (soul.index + 6) % 12);
        assert_eq!(soul.surrounded_palaces().target.index, soul.index);
    }

    #[test]
    fn test_palace_ref_flies_to_accepts_target() {
        let a = make_astrolabe();
        let soul = a.palace(Palace::Soul).unwrap();
        let career = a.palace(Palace::Career).unwrap();

        // 索引、宫名两种写法与直接传宫位数据结果一致
        for m in [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            let expected = soul.data().flies_to(&career, &[m]);
            assert_eq!(soul.flies_to(career.index, &[m]), expected);
            assert_eq!(soul.flies_to(Palace::Career, &[m]), expected);

            let expected_one = soul.data().flies_one_of_to(&career, &[m]);
            assert_eq!(soul.flies_one_of_to(Palace::Career, &[m]), expected_one);
            assert_eq!(
                soul.not_fly_to(Palace::Career, &[m]),
                soul.data().not_fly_to(&career, &[m])
            );
        }

        // 定位不到目标宫时一律为 false
        assert!(!soul.flies_to(99, &[Mutagen::Lu]));
        assert!(!soul.flies_one_of_to(99, &[Mutagen::Lu]));
        assert!(!soul.not_fly_to(99, &[Mutagen::Lu]));
    }

    #[test]
    fn test_palace_ref_mutaged_places() {
        let a = make_astrolabe();
        let soul = a.palace(Palace::Soul).unwrap();

        let places = soul.mutaged_places();
        assert_eq!(places.len(), 4);
        // 与显式传十二宫的底层结果逐项一致
        let indices = soul.data().mutaged_places(&a.palaces);
        for (place, index) in places.iter().zip(indices.iter()) {
            assert_eq!(place.map(|p| p.index), *index);
        }
        // 四化星必然都在盘上
        assert!(places.iter().all(|p| p.is_some()));
    }

    #[test]
    fn test_surrounded_palaces_indices() {
        let a = make_astrolabe();
        // Test palace 3
        let sp = a.surrounded_palaces(3).unwrap();
        assert_eq!(sp.target.index, 3);
        assert_eq!(sp.opposite.index, 9); // 3+6
        assert_eq!(sp.career.index, 7); // 3+4
        assert_eq!(sp.wealth.index, 11); // 3+8

        // 按宫名定位得到同一组三方四正
        let soul = a.palace(Palace::Soul).unwrap();
        let by_name = a.surrounded_palaces(Palace::Soul).unwrap();
        assert_eq!(by_name.target.index, soul.index);
    }
}

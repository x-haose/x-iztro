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

impl Astrolabe {
    /// 通过索引获取宫位
    pub fn palace(&self, index: usize) -> &PalaceData {
        &self.palaces[index]
    }

    /// 通过宫位名称获取宫位
    pub fn palace_by_name(&self, name: Palace) -> Option<&PalaceData> {
        self.palaces.iter().find(|p| p.name == name)
    }

    /// 获取三方四正
    pub fn surrounded_palaces(&self, index: usize) -> SurroundedPalaces<'_> {
        let opposite = fix_index(index as i32 + 6, 12);
        let career = fix_index(index as i32 + 4, 12);
        let wealth = fix_index(index as i32 + 8, 12);
        SurroundedPalaces {
            target: &self.palaces[index],
            opposite: &self.palaces[opposite],
            career: &self.palaces[career],
            wealth: &self.palaces[wealth],
        }
    }

    /// 查找星耀所在宫位
    pub fn star(&self, key: StarKey) -> Option<(&Star, &PalaceData)> {
        for p in &self.palaces {
            for s in p
                .major_stars
                .iter()
                .chain(p.minor_stars.iter())
                .chain(p.adjective_stars.iter())
            {
                if s.key == key {
                    return Some((s, p));
                }
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::astro::builder::by_solar;
    use crate::data::stars::StarKey;
    use crate::data::types::*;

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
            assert_eq!(a.palace(i).index, i);
        }
    }

    #[test]
    fn test_palace_by_name() {
        let a = make_astrolabe();
        let soul = a.palace_by_name(Palace::Soul);
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
            assert!(a.palace_by_name(*name).is_some());
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
            let result = a.star(*key);
            assert!(result.is_some(), "Star {:?} should be found", key);
            let (star, palace) = result.unwrap();
            assert_eq!(star.key, *key);
            // The palace should contain this star
            assert!(palace.has(&[*key]));
        }
    }

    #[test]
    fn test_surrounded_palaces_indices() {
        let a = make_astrolabe();
        // Test palace 3
        let sp = a.surrounded_palaces(3);
        assert_eq!(sp.target.index, 3);
        assert_eq!(sp.opposite.index, 9); // 3+6
        assert_eq!(sp.career.index, 7); // 3+4
        assert_eq!(sp.wealth.index, 11); // 3+8
    }
}

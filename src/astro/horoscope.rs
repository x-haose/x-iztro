//! 运限计算
//!
//! 根据出生星盘和目标日期，计算大限、小限、流年、流月、流日、流时。

use lunar_rust::lunar::LunarRefHelper;
use lunar_rust::{lunar, solar};

use crate::astro::builder::{parse_earthly_branch, parse_heavenly_stem};
use crate::astro::palace::get_palace_names;
use crate::astro::surpalaces::SurroundedPalaces;
use crate::data::constants::TIGER_RULE;
use crate::data::heavenly_stems::get_heavenly_stem_info;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::i18n::{translate_horoscope_name, translate_star};
use crate::models::astrolabe::Astrolabe;
use crate::models::horoscope::{AgeItem, HoroscopeData, HoroscopeItem, YearlyDecStar, YearlyItem};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::star::decorative::get_yearly12;
use crate::star::location::{
    get_chang_qu_index_by_stem, get_kui_yue_index, get_luan_xi_index, get_lu_yang_tuo_ma_index,
    get_nianjie_index,
};
use crate::utils::{earthly_branch_to_palace_index, fix_index};

// ============================================================
// 童限宫位映射
// ============================================================

/// 童限宫位名称：在大限未覆盖的童年时期使用
/// 对应 TS: [命宫, 财帛, 疾厄, 夫妻, 福德, 官禄]
const CHILDHOOD_PALACES: [Palace; 6] = [
    Palace::Soul,
    Palace::Wealth,
    Palace::Health,
    Palace::Spouse,
    Palace::Spirit,
    Palace::Career,
];

// ============================================================
// 流耀计算
// ============================================================

/// 获取运限流耀
///
/// 根据给定范围的天干地支，计算流耀在12宫中的分布。
/// 返回12个宫位的星耀列表。
pub fn get_horoscope_stars(
    stem: HeavenlyStem,
    branch: EarthlyBranch,
    scope: Scope,
    lang: Language,
) -> [Vec<Star>; 12] {
    let kui_yue = get_kui_yue_index(stem);
    let chang_qu = get_chang_qu_index_by_stem(stem);
    let lu_yang_tuo_ma = get_lu_yang_tuo_ma_index(stem, branch);
    let luan_xi = get_luan_xi_index(branch);

    let star_keys: (
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
        StarKey,
    ) = match scope {
        Scope::Decadal => (
            StarKey::Yunkui,
            StarKey::Yunyue,
            StarKey::Yunchang,
            StarKey::Yunqu,
            StarKey::Yunlu,
            StarKey::Yunyang,
            StarKey::Yuntuo,
            StarKey::Yunma,
            StarKey::Yunluan,
            StarKey::Yunxi,
        ),
        Scope::Yearly => (
            StarKey::Liukui,
            StarKey::Liuyue,
            StarKey::Liuchang,
            StarKey::Liuqu,
            StarKey::Liulu,
            StarKey::Liuyang,
            StarKey::Liutuo,
            StarKey::Liuma,
            StarKey::Liuluan,
            StarKey::Liuxi,
        ),
        Scope::Monthly => (
            StarKey::Yuekui,
            StarKey::Yueyue,
            StarKey::Yuechang,
            StarKey::Yuequ,
            StarKey::Yuelu,
            StarKey::Yueyang,
            StarKey::Yuetuo,
            StarKey::Yuema,
            StarKey::Yueluan,
            StarKey::Yuexi,
        ),
        Scope::Daily => (
            StarKey::Rikui,
            StarKey::Riyue,
            StarKey::Richang,
            StarKey::Riqu,
            StarKey::Rilu,
            StarKey::Riyang,
            StarKey::Rituo,
            StarKey::Rima,
            StarKey::Riluan,
            StarKey::Rixi,
        ),
        Scope::Hourly => (
            StarKey::Shikui,
            StarKey::Shiyue,
            StarKey::Shichang,
            StarKey::Shiqu,
            StarKey::Shilu,
            StarKey::Shiyang,
            StarKey::Shituo,
            StarKey::Shima,
            StarKey::Shiluan,
            StarKey::Shixi,
        ),
        _ => unreachable!("get_horoscope_stars does not support Origin scope"),
    };

    let mut stars: [Vec<Star>; 12] = std::array::from_fn(|_| Vec::new());

    let placements = [
        (kui_yue.kui, star_keys.0, StarType::Soft),
        (kui_yue.yue, star_keys.1, StarType::Soft),
        (chang_qu.chang, star_keys.2, StarType::Soft),
        (chang_qu.qu, star_keys.3, StarType::Soft),
        (lu_yang_tuo_ma.lu, star_keys.4, StarType::Lucun),
        (lu_yang_tuo_ma.yang, star_keys.5, StarType::Tough),
        (lu_yang_tuo_ma.tuo, star_keys.6, StarType::Tough),
        (lu_yang_tuo_ma.ma, star_keys.7, StarType::Tianma),
        (luan_xi.hongluan, star_keys.8, StarType::Flower),
        (luan_xi.tianxi, star_keys.9, StarType::Flower),
    ];

    for (idx, key, star_type) in placements {
        stars[idx].push(Star {
            key,
            name: translate_star(key, lang).to_string(),
            star_type,
            scope,
            brightness: None,
            mutagen: None,
        });
    }

    // 流年范围额外安年解（按流年年支）
    if scope == Scope::Yearly {
        let nianjie_idx = get_nianjie_index(branch);
        stars[nianjie_idx].push(Star {
            key: StarKey::Nianjie,
            name: translate_star(StarKey::Nianjie, lang).to_string(),
            star_type: StarType::Helper,
            scope: Scope::Yearly,
            brightness: None,
            mutagen: None,
        });
    }

    stars
}

// ============================================================
// 主函数
// ============================================================

/// 时辰索引转小时数（用于 lunar_rust 日期创建）
fn time_index_to_hour(time_index: u8) -> i64 {
    match time_index {
        0 => 0,
        12 => 23,
        i => (i as i64) * 2 - 1,
    }
}

/// 根据出生星盘和目标阳历日期计算运限
///
/// # 参数
/// - `astrolabe`: 出生星盘
/// - `solar_date`: 目标阳历日期，格式 "YYYY-M-D"
/// - `time_index`: 目标时辰索引 (0=早子, 1=丑, ..., 12=晚子)
/// - `language`: 语言
pub fn get_horoscope(
    astrolabe: &Astrolabe,
    solar_date: &str,
    time_index: u8,
    language: Language,
) -> HoroscopeData {
    assert!(
        time_index <= 12,
        "time_index must be 0-12, got {time_index}"
    );

    // ---- 1. 解析出生日期的农历信息 ----
    let birth_parts: Vec<&str> = astrolabe.solar_date.split('-').collect();
    let birth_year: i64 = birth_parts[0].parse().expect("Invalid birth year");
    let birth_month: i64 = birth_parts[1].parse().expect("Invalid birth month");
    let birth_day: i64 = birth_parts[2].parse().expect("Invalid birth day");

    let birth_hour = time_index_to_hour(astrolabe.time_index);
    let birth_solar = solar::from_ymdhms(birth_year, birth_month, birth_day, birth_hour, 0, 0);
    let birth_lunar = lunar::from_solar(&birth_solar);

    let birthday_lunar_year = birth_lunar.get_year();
    let birthday_lunar_month = birth_lunar.get_month().unsigned_abs() as i32;
    let birthday_lunar_day = birth_lunar.get_day() as u32;
    let birthday_is_leap = birth_lunar.get_month() < 0;

    // ---- 2. 解析目标日期的农历信息 ----
    let parts: Vec<&str> = solar_date.split('-').collect();
    assert!(parts.len() == 3, "Invalid solar date format: {solar_date}");
    let target_year: i64 = parts[0].parse().expect("Invalid year");
    let target_month: i64 = parts[1].parse().expect("Invalid month");
    let target_day: i64 = parts[2].parse().expect("Invalid day");

    let target_hour = time_index_to_hour(time_index);
    let target_solar = solar::from_ymdhms(target_year, target_month, target_day, target_hour, 0, 0);
    let target_lunar = lunar::from_solar(&target_solar);

    let target_lunar_year = target_lunar.get_year();
    let target_lunar_month_raw = target_lunar.get_month();
    let target_is_leap = target_lunar_month_raw < 0;
    let target_lunar_month = target_lunar_month_raw.unsigned_abs() as i32;
    let target_lunar_day = target_lunar.get_day() as u32;

    // ---- 3. 计算虚岁 ----
    let nominal_age = (target_lunar_year - birthday_lunar_year + 1).max(1) as u32;

    // ---- 4. 目标日期干支四柱 ----
    //     年柱按正月初一分界；月柱按初一分界以五虎遁推算，闰月下半月归下月；
    //     日柱晚子时归次日；时柱天干随日柱推算
    let target_year_gan_str = target_lunar.get_year_gan();
    let target_year_zhi_str = target_lunar.get_year_zhi();
    let target_day_gan_str = target_lunar.get_day_gan_exact();
    let target_day_zhi_str = target_lunar.get_day_zhi_exact();
    let target_time_gan_str = target_lunar.get_time_gan();
    let target_time_zhi_str = target_lunar.get_time_zhi();

    let target_year_stem =
        parse_heavenly_stem(&target_year_gan_str).expect("Unknown target year stem");
    let target_year_branch =
        parse_earthly_branch(&target_year_zhi_str).expect("Unknown target year branch");

    let target_month_fix: i32 = if target_is_leap && target_lunar_day > 15 { 1 } else { 0 };
    let target_month_stem = HeavenlyStem::from_index(fix_index(
        TIGER_RULE[target_year_stem.index()].index() as i32 + target_lunar_month - 1
            + target_month_fix,
        10,
    ));
    let target_month_branch =
        EarthlyBranch::from_index(fix_index(2 + target_lunar_month - 1 + target_month_fix, 12));

    let target_day_stem =
        parse_heavenly_stem(&target_day_gan_str).expect("Unknown target day stem");
    let target_day_branch =
        parse_earthly_branch(&target_day_zhi_str).expect("Unknown target day branch");
    let target_time_stem =
        parse_heavenly_stem(&target_time_gan_str).expect("Unknown target time stem");
    let target_time_branch =
        parse_earthly_branch(&target_time_zhi_str).expect("Unknown target time branch");

    // ---- 5. 大限 ----
    //     虚岁落在某宫大限区间内取该宫；尚未起运时为童限，
    //     按虚岁依次落在 命宫/财帛/疾厄/夫妻/福德/官禄，取本命宫位干支
    let (decadal_palace_idx, is_childhood) = find_decadal_palace(astrolabe, nominal_age);
    let (decadal_stem, decadal_branch) = if is_childhood {
        let p = &astrolabe.palaces[decadal_palace_idx];
        (p.heavenly_stem, p.earthly_branch)
    } else {
        let d = &astrolabe.palaces[decadal_palace_idx].decadal;
        (d.heavenly_stem, d.earthly_branch)
    };
    let decadal_mutagen = get_heavenly_stem_info(decadal_stem).mutagen.to_vec();
    let decadal_palace_names = get_palace_names(decadal_palace_idx).to_vec();
    let decadal_stars = get_horoscope_stars(decadal_stem, decadal_branch, Scope::Decadal, language);

    let decadal = HoroscopeItem {
        index: decadal_palace_idx,
        name: translate_horoscope_name(
            if is_childhood { HoroscopeName::Childhood } else { HoroscopeName::Decadal },
            language,
        )
        .to_string(),
        heavenly_stem: decadal_stem,
        earthly_branch: decadal_branch,
        palace_names: decadal_palace_names,
        mutagen: decadal_mutagen,
        stars: Some(decadal_stars.to_vec()),
    };

    // ---- 6. 小限 ----
    let age_palace_idx = find_age_palace(astrolabe, nominal_age);
    let age_stem = astrolabe.palaces[age_palace_idx].heavenly_stem;
    let age_branch = astrolabe.palaces[age_palace_idx].earthly_branch;
    let age_mutagen = get_heavenly_stem_info(age_stem).mutagen.to_vec();
    let age_palace_names = get_palace_names(age_palace_idx).to_vec();

    let age = AgeItem {
        base: HoroscopeItem {
            index: age_palace_idx,
            name: translate_horoscope_name(HoroscopeName::Age, language).to_string(),
            heavenly_stem: age_stem,
            earthly_branch: age_branch,
            palace_names: age_palace_names,
            mutagen: age_mutagen,
            stars: None,
        },
        nominal_age,
    };

    // ---- 7. 流年（含按目标年支起的岁前/将前十二神） ----
    let yearly_index = earthly_branch_to_palace_index(target_year_branch);
    let yearly_mutagen = get_heavenly_stem_info(target_year_stem).mutagen.to_vec();
    let yearly_palace_names = get_palace_names(yearly_index).to_vec();
    let yearly_stars =
        get_horoscope_stars(target_year_stem, target_year_branch, Scope::Yearly, language);
    let (yearly_suiqian12, yearly_jiangqian12) =
        get_yearly12(target_year_branch, astrolabe.algorithm);

    let yearly = YearlyItem {
        base: HoroscopeItem {
            index: yearly_index,
            name: translate_horoscope_name(HoroscopeName::Yearly, language).to_string(),
            heavenly_stem: target_year_stem,
            earthly_branch: target_year_branch,
            palace_names: yearly_palace_names,
            mutagen: yearly_mutagen,
            stars: Some(yearly_stars.to_vec()),
        },
        yearly_dec_star: YearlyDecStar {
            jiangqian12: yearly_jiangqian12.to_vec(),
            suiqian12: yearly_suiqian12.to_vec(),
        },
    };

    // ---- 8. 流月 ----
    let leap_addition = if birthday_is_leap && birthday_lunar_day > 15 {
        1
    } else {
        0
    };
    let date_leap_addition = if target_is_leap && target_lunar_day > 15 {
        1
    } else {
        0
    };
    // 出生时辰的地支原始索引 (0=子, 1=丑, ...)
    let birth_time_branch_raw_index = fix_index(astrolabe.time_index as i32, 12) as i32;

    let monthly_index = fix_index(
        yearly_index as i32 - (birthday_lunar_month + leap_addition)
            + birth_time_branch_raw_index
            + (target_lunar_month + date_leap_addition),
        12,
    );
    let monthly_mutagen = get_heavenly_stem_info(target_month_stem).mutagen.to_vec();
    let monthly_palace_names = get_palace_names(monthly_index).to_vec();
    let monthly_stars = get_horoscope_stars(
        target_month_stem,
        target_month_branch,
        Scope::Monthly,
        language,
    );

    let monthly = HoroscopeItem {
        index: monthly_index,
        name: translate_horoscope_name(HoroscopeName::Monthly, language).to_string(),
        heavenly_stem: target_month_stem,
        earthly_branch: target_month_branch,
        palace_names: monthly_palace_names,
        mutagen: monthly_mutagen,
        stars: Some(monthly_stars.to_vec()),
    };

    // ---- 9. 流日 ----
    let daily_index = fix_index(monthly_index as i32 + target_lunar_day as i32 - 1, 12);
    let daily_mutagen = get_heavenly_stem_info(target_day_stem).mutagen.to_vec();
    let daily_palace_names = get_palace_names(daily_index).to_vec();
    let daily_stars =
        get_horoscope_stars(target_day_stem, target_day_branch, Scope::Daily, language);

    let daily = HoroscopeItem {
        index: daily_index,
        name: translate_horoscope_name(HoroscopeName::Daily, language).to_string(),
        heavenly_stem: target_day_stem,
        earthly_branch: target_day_branch,
        palace_names: daily_palace_names,
        mutagen: daily_mutagen,
        stars: Some(daily_stars.to_vec()),
    };

    // ---- 10. 流时 ----
    let target_hour_branch_index = fix_index(time_index as i32, 12) as i32;
    let hourly_index = fix_index(daily_index as i32 + target_hour_branch_index, 12);
    let hourly_mutagen = get_heavenly_stem_info(target_time_stem).mutagen.to_vec();
    let hourly_palace_names = get_palace_names(hourly_index).to_vec();
    let hourly_stars = get_horoscope_stars(
        target_time_stem,
        target_time_branch,
        Scope::Hourly,
        language,
    );

    let hourly = HoroscopeItem {
        index: hourly_index,
        name: translate_horoscope_name(HoroscopeName::Hourly, language).to_string(),
        heavenly_stem: target_time_stem,
        earthly_branch: target_time_branch,
        palace_names: hourly_palace_names,
        mutagen: hourly_mutagen,
        stars: Some(hourly_stars.to_vec()),
    };

    // ---- 11. 农历日期字符串（lunar_rust 的月份中文名对闰月自带「闰」前缀） ----
    let lunar_date_str = format!(
        "{}年{}月{}",
        target_lunar.get_year_in_chinese(),
        target_lunar.get_month_in_chinese(),
        target_lunar.get_day_in_chinese(),
    );

    HoroscopeData {
        solar_date: solar_date.to_string(),
        lunar_date: lunar_date_str,
        decadal,
        age,
        yearly,
        monthly,
        daily,
        hourly,
    }
}

// ============================================================
// 辅助函数
// ============================================================

/// 查找大限宫位索引，返回 (宫位索引, 是否童限)。
///
/// 虚岁落在某宫大限区间内取该宫；尚未起运时为童限，
/// 按虚岁依次映射到 [命宫, 财帛, 疾厄, 夫妻, 福德, 官禄]。
fn find_decadal_palace(astrolabe: &Astrolabe, nominal_age: u32) -> (usize, bool) {
    for p in &astrolabe.palaces {
        let (start, end) = p.decadal.range;
        if nominal_age >= start && nominal_age <= end {
            return (p.index, false);
        }
    }

    let childhood_idx = ((nominal_age as usize).saturating_sub(1)) % CHILDHOOD_PALACES.len();
    let target_palace = CHILDHOOD_PALACES[childhood_idx];

    let idx = astrolabe
        .palaces
        .iter()
        .find(|p| p.name == target_palace)
        .map(|p| p.index)
        .unwrap_or(0);
    (idx, true)
}

/// 查找小限宫位索引
///
/// 根据虚岁查找对应的小限宫位。
fn find_age_palace(astrolabe: &Astrolabe, nominal_age: u32) -> usize {
    for p in &astrolabe.palaces {
        if p.ages.contains(&nominal_age) {
            return p.index;
        }
    }
    0
}

// ============================================================
// HoroscopeData 查询方法
// ============================================================

impl HoroscopeItem {
    /// 将名称解析为 Palace 枚举（通过 palace_names 反查）
    fn palace_index_by_name(&self, name: Palace) -> Option<usize> {
        self.palace_names.iter().position(|p| *p == name)
    }
}

impl HoroscopeData {
    /// 获取小限宫位
    pub fn age_palace<'a>(&self, astrolabe: &'a Astrolabe) -> &'a PalaceData {
        &astrolabe.palaces[self.age.base.index]
    }

    /// 根据宫位名称和运限范围获取宫位
    ///
    /// - Origin: 直接从星盘中按宫位名称查找
    /// - 其他: 从该运限的宫位名称列表中查找索引，返回星盘中对应位置的宫位
    pub fn palace<'a>(
        &self,
        name: Palace,
        scope: Scope,
        astrolabe: &'a Astrolabe,
    ) -> Option<&'a PalaceData> {
        if scope == Scope::Origin {
            astrolabe.palace_by_name(name)
        } else {
            let scope_item = self.scope_item(scope)?;
            let idx = scope_item.palace_index_by_name(name)?;
            Some(&astrolabe.palaces[idx])
        }
    }

    /// 获取指定宫位的三方四正
    pub fn surround_palaces<'a>(
        &self,
        name: Palace,
        scope: Scope,
        astrolabe: &'a Astrolabe,
    ) -> Option<SurroundedPalaces<'a>> {
        let palace = self.palace(name, scope, astrolabe)?;
        Some(astrolabe.surrounded_palaces(palace.index))
    }

    /// 检查指定运限的四化星是否在指定宫位中
    ///
    /// 检查该运限天干的四化对应星耀是否出现在目标宫位的主星或辅星中。
    pub fn has_horoscope_mutagen(
        &self,
        name: Palace,
        scope: Scope,
        mutagen: Mutagen,
        astrolabe: &Astrolabe,
    ) -> bool {
        if scope == Scope::Origin {
            return false;
        }
        let scope_item = match self.scope_item(scope) {
            Some(i) => i,
            None => return false,
        };
        let palace = match self.palace(name, scope, astrolabe) {
            Some(p) => p,
            None => return false,
        };
        let mutagen_index = match mutagen {
            Mutagen::Lu => 0,
            Mutagen::Quan => 1,
            Mutagen::Ke => 2,
            Mutagen::Ji => 3,
        };
        let star_key = scope_item.mutagen[mutagen_index];
        palace
            .major_stars
            .iter()
            .chain(palace.minor_stars.iter())
            .any(|s| s.key == star_key)
    }

    /// 检查大限和流年的流耀是否包含指定星耀
    ///
    /// 合并大限和流年的流耀，检查指定宫位中是否包含所有指定星耀。
    pub fn has_horoscope_stars(
        &self,
        name: Palace,
        scope: Scope,
        stars: &[StarKey],
        astrolabe: &Astrolabe,
    ) -> bool {
        let palace_idx = match self.palace(name, scope, astrolabe) {
            Some(p) => p.index,
            None => return false,
        };
        let all_keys = self.collect_horoscope_star_keys(palace_idx);
        stars.iter().all(|s| all_keys.contains(s))
    }

    /// 检查大限和流年的流耀是否不包含指定星耀
    pub fn not_have_horoscope_stars(
        &self,
        name: Palace,
        scope: Scope,
        stars: &[StarKey],
        astrolabe: &Astrolabe,
    ) -> bool {
        let palace_idx = match self.palace(name, scope, astrolabe) {
            Some(p) => p.index,
            None => return false,
        };
        let all_keys = self.collect_horoscope_star_keys(palace_idx);
        stars.iter().all(|s| !all_keys.contains(s))
    }

    /// 收集指定宫位中大限和流年的所有流耀 StarKey
    fn collect_horoscope_star_keys(&self, palace_idx: usize) -> Vec<StarKey> {
        let mut keys = Vec::new();
        if let Some(stars) = self.decadal.stars.as_ref().and_then(|s| s.get(palace_idx)) {
            keys.extend(stars.iter().map(|s| s.key));
        }
        if let Some(stars) = self.yearly.base.stars.as_ref().and_then(|s| s.get(palace_idx)) {
            keys.extend(stars.iter().map(|s| s.key));
        }
        keys
    }

    /// 获取指定运限范围的 HoroscopeItem
    fn scope_item(&self, scope: Scope) -> Option<&HoroscopeItem> {
        match scope {
            Scope::Decadal => Some(&self.decadal),
            Scope::Yearly => Some(&self.yearly.base),
            Scope::Monthly => Some(&self.monthly),
            Scope::Daily => Some(&self.daily),
            Scope::Hourly => Some(&self.hourly),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;

    fn make_astrolabe() -> Astrolabe {
        by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Algorithm::Default,
        )
    }

    #[test]
    fn test_get_horoscope_basic() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);

        // Basic structure checks
        assert_eq!(horoscope.solar_date, "2023-10-15");
        assert!(!horoscope.lunar_date.is_empty());

        // Decadal should have palace names and stars
        assert_eq!(horoscope.decadal.palace_names.len(), 12);
        assert!(horoscope.decadal.stars.is_some());
        let dec_stars = horoscope.decadal.stars.as_ref().unwrap();
        assert_eq!(dec_stars.len(), 12);

        // Yearly should have stars including nianjie
        assert!(horoscope.yearly.base.stars.is_some());
        let yr_stars = horoscope.yearly.base.stars.as_ref().unwrap();
        assert_eq!(yr_stars.len(), 12);

        // Monthly, daily, hourly should have stars
        assert!(horoscope.monthly.stars.is_some());
        assert!(horoscope.daily.stars.is_some());
        assert!(horoscope.hourly.stars.is_some());

        // Nominal age should be reasonable
        assert!(horoscope.age.nominal_age > 0);

        // Mutagen should have 4 entries
        assert_eq!(horoscope.decadal.mutagen.len(), 4);
        assert_eq!(horoscope.yearly.base.mutagen.len(), 4);
        assert_eq!(horoscope.monthly.mutagen.len(), 4);
        assert_eq!(horoscope.daily.mutagen.len(), 4);
        assert_eq!(horoscope.hourly.mutagen.len(), 4);
    }

    #[test]
    fn test_get_horoscope_nominal_age() {
        let astrolabe = make_astrolabe();
        // 2000年出生，2023年查询
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);
        // 虚岁 = 目标农历年 - 出生农历年 + 1
        // 2000年出生 → 2023年大约24虚岁
        assert!(horoscope.age.nominal_age >= 23 && horoscope.age.nominal_age <= 25);
    }

    #[test]
    fn test_get_horoscope_stars_decadal() {
        let stars = get_horoscope_stars(
            HeavenlyStem::Jia,
            EarthlyBranch::Yin,
            Scope::Decadal,
            Language::ZhCN,
        );
        // Should place 10 stars across 12 palaces
        let total: usize = stars.iter().map(|v| v.len()).sum();
        assert_eq!(total, 10);
        // All stars should have Decadal scope
        for palace_stars in &stars {
            for s in palace_stars {
                assert_eq!(s.scope, Scope::Decadal);
            }
        }
    }

    #[test]
    fn test_get_horoscope_stars_yearly_has_nianjie() {
        let stars = get_horoscope_stars(
            HeavenlyStem::Jia,
            EarthlyBranch::Yin,
            Scope::Yearly,
            Language::ZhCN,
        );
        // 流耀 10 颗（魁钺昌曲禄羊陀马鸾喜）+ 流年额外的年解 = 11
        let total: usize = stars.iter().map(|v| v.len()).sum();
        assert_eq!(total, 11);
        let has_nianjie = stars
            .iter()
            .flat_map(|v| v.iter())
            .any(|s| s.key == StarKey::Nianjie);
        assert!(has_nianjie);
    }

    #[test]
    fn test_horoscope_palace_query() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);

        // Origin scope should find palace
        let soul = horoscope.palace(Palace::Soul, Scope::Origin, &astrolabe);
        assert!(soul.is_some());
        assert_eq!(soul.unwrap().name, Palace::Soul);

        // Decadal scope should find palace
        let dec_soul = horoscope.palace(Palace::Soul, Scope::Decadal, &astrolabe);
        assert!(dec_soul.is_some());
    }

    #[test]
    fn test_horoscope_age_palace() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);
        let age_palace = horoscope.age_palace(&astrolabe);
        assert!(age_palace.index < 12);
    }

    #[test]
    fn test_horoscope_surround_palaces() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);
        let sp = horoscope.surround_palaces(Palace::Soul, Scope::Origin, &astrolabe);
        assert!(sp.is_some());
        let sp = sp.unwrap();
        assert_eq!(sp.target.name, Palace::Soul);
    }

    #[test]
    fn test_childhood_decadal() {
        // Test with a very young age (birth year = target year)
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2000-12-1", 4, Language::ZhCN);
        // Should be age 1, which may be in childhood range
        assert!(horoscope.age.nominal_age <= 2);
        // Decadal index should be valid
        assert!(horoscope.decadal.index < 12);
    }

    #[test]
    fn test_has_horoscope_mutagen() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);

        // The decadal's Lu mutagen star should exist in some palace
        let mutagen_star = horoscope.decadal.mutagen[0]; // Lu
        // Find which palace has this star
        let mut found = false;
        for palace_name in &[
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
        ] {
            if horoscope.has_horoscope_mutagen(*palace_name, Scope::Decadal, Mutagen::Lu, &astrolabe)
            {
                found = true;
                break;
            }
        }
        // The mutagen star should be found in at least one palace
        assert!(found, "Lu mutagen star {:?} should be in some palace", mutagen_star);
    }

    #[test]
    fn test_has_horoscope_stars_and_not() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN);

        // Find a palace that has horoscope stars
        let dec_stars = horoscope.decadal.stars.as_ref().unwrap();
        // Find first non-empty palace in decadal stars
        let mut test_palace_idx = None;
        let mut test_star_key = None;
        for (i, stars) in dec_stars.iter().enumerate() {
            if !stars.is_empty() {
                test_palace_idx = Some(i);
                test_star_key = Some(stars[0].key);
                break;
            }
        }

        if let (Some(idx), Some(key)) = (test_palace_idx, test_star_key) {
            let palace_name = astrolabe.palaces[idx].name;
            // Use Origin scope so the palace is looked up by its native name
            assert!(horoscope.has_horoscope_stars(palace_name, Scope::Origin, &[key], &astrolabe));
            // not_have should return false for a star that exists
            assert!(!horoscope.not_have_horoscope_stars(palace_name, Scope::Origin, &[key], &astrolabe));
        }
    }
}

use crate::data::stars::StarKey;
use crate::data::types::{Brightness, Config, EarthlyBranch, HeavenlyStem, Language, Mutagen};
use crate::i18n::{translate_earthly_branch, translate_heavenly_stem};

/// 将索引约束在 0..max 范围内（循环取模）
/// 对应 TS: fixIndex(index, max=12)
pub fn fix_index(index: i32, max: i32) -> usize {
    let result = ((index % max) + max) % max;
    result as usize
}

/// 地支索引转宫位索引（寅宫为 0）
/// 宫位从寅开始，所以需要减去寅的地支索引(2)
pub fn earthly_branch_to_palace_index(branch: EarthlyBranch) -> usize {
    fix_index(
        branch.index() as i32 - EarthlyBranch::Yin.index() as i32,
        12,
    )
}

/// 小时转时辰索引
/// 0点=0(早子), 1-2=1(丑), ..., 23=12(晚子)
pub fn time_to_index(hour: u8) -> u8 {
    match hour {
        0 => 0,
        23 => 12,
        h => h.div_ceil(2),
    }
}

/// 获取小限起始宫位索引
/// 寅午戌→辰, 申子辰→戌, 巳酉丑→未, 亥卯未→丑
pub fn get_age_index(branch: EarthlyBranch) -> usize {
    use EarthlyBranch::*;
    let start = match branch {
        Yin | Wu | Xu => Chen,
        Shen | Zi | Chen => Xu,
        Si | You | Chou => Wei,
        Hai | Mao | Wei => Chou,
    };
    earthly_branch_to_palace_index(start)
}

/// 星耀落在指定宫位时的亮度；该星没有亮度表时返回 `None`。
///
/// `palace_index` 为盘上位置（0 是寅宫），越界会对 12 取模。
/// `config` 里若有该星的自定义亮度表则以它为准。
pub fn get_brightness(star: StarKey, palace_index: i32, config: &Config) -> Option<Brightness> {
    config.brightness_of(star, fix_index(palace_index, 12))
}

/// 指定天干下，某颗星化什么；该星不在这个天干的四化表里时返回 `None`。
///
/// `config` 里若有该天干的自定义四化表则以它为准。
pub fn get_mutagen(star: StarKey, stem: HeavenlyStem, config: &Config) -> Option<Mutagen> {
    let position = config.mutagens_of(stem).iter().position(|s| *s == star)?;

    Some(match position {
        0 => Mutagen::Lu,
        1 => Mutagen::Quan,
        2 => Mutagen::Ke,
        _ => Mutagen::Ji,
    })
}

/// 指定天干化出的四颗星，顺序为禄、权、科、忌。
///
/// `config` 里若有该天干的自定义四化表则以它为准。
pub fn get_mutagens_by_heavenly_stem(stem: HeavenlyStem, config: &Config) -> [StarKey; 4] {
    config.mutagens_of(stem)
}

/// 按语言拼接四柱干支 [年柱, 月柱, 日柱, 时柱]。
///
/// 星盘的 `chinese_date` 字段即由本函数生成；结构化的四柱在 `raw_dates.chinese_date`。
/// 词条均为单字符时柱内紧凑相连、柱间空格（如「庚辰 甲申 丁未 庚子」）；
/// 任一词条为多字符时柱内空格、柱间「 - 」（如「geng chen - jia shen - …」）
pub fn translate_chinese_date(
    pillars: [(HeavenlyStem, EarthlyBranch); 4],
    lang: Language,
) -> String {
    let translated: Vec<(&str, &str)> = pillars
        .iter()
        .map(|(s, b)| {
            (
                translate_heavenly_stem(*s, lang),
                translate_earthly_branch(*b, lang),
            )
        })
        .collect();
    let compact = translated
        .iter()
        .all(|(s, b)| s.chars().count() == 1 && b.chars().count() == 1);
    if compact {
        translated
            .iter()
            .map(|(s, b)| format!("{s}{b}"))
            .collect::<Vec<_>>()
            .join(" ")
    } else {
        translated
            .iter()
            .map(|(s, b)| format!("{s} {b}"))
            .collect::<Vec<_>>()
            .join(" - ")
    }
}

/// 把多组「十二宫星耀」按宫位合并成一组。
///
/// 安星是分批进行的（主星、辅星、杂耀各出一组十二宫列表），
/// 本函数按宫位索引把它们首尾相接，顺序为传入顺序。
pub fn merge_stars(
    groups: &[[Vec<crate::models::star::Star>; 12]],
) -> [Vec<crate::models::star::Star>; 12] {
    let mut merged: [Vec<crate::models::star::Star>; 12] = std::array::from_fn(|_| Vec::new());
    for group in groups {
        for (i, stars) in group.iter().enumerate() {
            merged[i].extend(stars.iter().cloned());
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::astro::builder::by_solar;
    use crate::data::types::{Config, Gender, Language};

    /// 亮度与四化的查表结果必须与排盘时写入星耀的字段一致。
    #[test]
    fn test_brightness_and_mutagen_match_astrolabe() {
        let chart = by_solar(
            "2000-8-16",
            2,
            Gender::Female,
            true,
            Language::ZhCN,
            Config::default(),
        )
        .unwrap();
        let year_stem = chart.raw_dates.chinese_date.yearly.0;

        for palace in &chart.palaces {
            for star in palace.major_stars.iter().chain(palace.minor_stars.iter()) {
                assert_eq!(
                    get_brightness(star.key, palace.index as i32, &chart.config),
                    star.brightness,
                    "{:?} 在宫位 {} 的亮度不一致",
                    star.key,
                    palace.index
                );
                assert_eq!(
                    get_mutagen(star.key, year_stem, &chart.config),
                    star.mutagen,
                    "{:?} 在年干 {:?} 下的四化不一致",
                    star.key,
                    year_stem
                );
            }
        }
    }

    #[test]
    fn test_mutagens_by_heavenly_stem() {
        // 壬干四化：天梁禄、紫微权、左辅科、武曲忌
        let stars = get_mutagens_by_heavenly_stem(HeavenlyStem::Ren, &Config::default());
        assert_eq!(stars[0], StarKey::TianliangMaj);
        assert_eq!(stars[1], StarKey::ZiweiMaj);
        assert_eq!(stars[2], StarKey::ZuofuMin);
        assert_eq!(stars[3], StarKey::WuquMaj);

        // get_mutagen 是它的反查
        assert_eq!(
            get_mutagen(StarKey::ZiweiMaj, HeavenlyStem::Ren, &Config::default()),
            Some(Mutagen::Quan)
        );
        assert_eq!(
            get_mutagen(StarKey::TianfuMaj, HeavenlyStem::Ren, &Config::default()),
            None
        );
    }

    #[test]
    fn test_fix_index() {
        assert_eq!(fix_index(0, 12), 0);
        assert_eq!(fix_index(12, 12), 0);
        assert_eq!(fix_index(-1, 12), 11);
        assert_eq!(fix_index(-12, 12), 0);
        assert_eq!(fix_index(5, 12), 5);
    }

    #[test]
    fn test_earthly_branch_to_palace_index() {
        // 寅=2, palace_index = 2-2 = 0
        assert_eq!(earthly_branch_to_palace_index(EarthlyBranch::Yin), 0);
        // 子=0, palace_index = 0-2 = -2 → fix_index(-2,12) = 10
        assert_eq!(earthly_branch_to_palace_index(EarthlyBranch::Zi), 10);
        // 卯=3, palace_index = 3-2 = 1
        assert_eq!(earthly_branch_to_palace_index(EarthlyBranch::Mao), 1);
    }

    #[test]
    fn test_time_to_index() {
        assert_eq!(time_to_index(0), 0);
        assert_eq!(time_to_index(1), 1);
        assert_eq!(time_to_index(2), 1);
        assert_eq!(time_to_index(3), 2);
        assert_eq!(time_to_index(23), 12);
    }

    #[test]
    fn test_get_age_index() {
        // 寅午戌→辰, 辰=4, palace_index = 4-2 = 2
        assert_eq!(get_age_index(EarthlyBranch::Yin), 2);
        assert_eq!(get_age_index(EarthlyBranch::Wu), 2);
        // 申子辰→戌, 戌=10, palace_index = 10-2 = 8
        assert_eq!(get_age_index(EarthlyBranch::Zi), 8);
    }
}

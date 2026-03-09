use crate::data::constants::{PALACES, TIGER_RULE};
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::types::*;
use crate::models::palace::Decadal;
use crate::utils::{fix_index, get_age_index};

/// 命宫身宫计算结果
#[derive(Debug, Clone)]
pub struct SoulAndBody {
    /// 命宫宫位索引 (0-11, 寅宫为0)
    pub soul_index: usize,
    /// 身宫宫位索引
    pub body_index: usize,
    /// 命宫天干
    pub heavenly_stem_of_soul: HeavenlyStem,
    /// 命宫地支
    pub earthly_branch_of_soul: EarthlyBranch,
}

/// 计算命宫和身宫
///
/// - `month_index`: fixLunarMonthIndex 结果 (0-based)
/// - `time_index`: 时辰索引 (0-12)
/// - `yearly_stem`: 年干
pub fn get_soul_and_body(
    month_index: usize,
    time_index: u8,
    yearly_stem: HeavenlyStem,
) -> SoulAndBody {
    // 时辰对应地支索引: time_index 0=子(0), 1=丑(1), ..., 12=晚子(0)
    let time_earthly_branch_index = fix_index(time_index as i32, 12);

    // 命宫索引 = 月 - 时辰地支
    let soul_index = fix_index(
        month_index as i32 - time_earthly_branch_index as i32,
        12,
    );

    // 身宫索引 = 月 + 时辰地支
    let body_index = fix_index(
        month_index as i32 + time_earthly_branch_index as i32,
        12,
    );

    // 五虎遁得寅宫天干
    let start_stem = TIGER_RULE[yearly_stem.index()];

    // 命宫天干
    let heavenly_stem_of_soul_index = fix_index(
        start_stem.index() as i32 + soul_index as i32,
        10,
    );
    let heavenly_stem_of_soul = HeavenlyStem::from_index(heavenly_stem_of_soul_index);

    // 命宫地支: 宫位索引0=寅(地支索引2), 所以 +2
    let earthly_branch_of_soul = EarthlyBranch::from_index(fix_index(
        soul_index as i32 + 2,
        12,
    ));

    SoulAndBody {
        soul_index,
        body_index,
        heavenly_stem_of_soul,
        earthly_branch_of_soul,
    }
}

/// 计算五行局
pub fn get_five_elements_class(
    stem: HeavenlyStem,
    branch: EarthlyBranch,
) -> FiveElementsClass {
    // 天干编号: 甲乙→1, 丙丁→2, 戊己→3, 庚辛→4, 壬癸→5
    let stem_number = stem.index() / 2 + 1;

    // 地支编号: 对地支索引取模6后 / 2 + 1
    // 子(0)午(6)→0→1, 丑(1)未(7)→1→1, 寅(2)申(8)→2→2, 卯(3)酉(9)→3→2, 辰(4)戌(10)→4→3, 巳(5)亥(11)→5→3
    let branch_number = fix_index(branch.index() as i32, 6) / 2 + 1;

    let mut sum = stem_number + branch_number;
    while sum > 5 {
        sum -= 5;
    }

    match sum {
        1 => FiveElementsClass::Wood3rd,
        2 => FiveElementsClass::Metal4th,
        3 => FiveElementsClass::Water2nd,
        4 => FiveElementsClass::Fire6th,
        5 => FiveElementsClass::Earth5th,
        _ => unreachable!(),
    }
}

/// 获取十二宫位名称
///
/// 根据命宫索引，返回每个宫位对应的宫位名称
pub fn get_palace_names(soul_index: usize) -> [Palace; 12] {
    let mut names = [Palace::Soul; 12];
    for i in 0..12 {
        names[i] = PALACES[fix_index(i as i32 - soul_index as i32, 12)];
    }
    names
}

/// 计算大限和小限
pub fn get_decadals_and_ages(
    soul_index: usize,
    five_elements_class: FiveElementsClass,
    gender: Gender,
    yearly_stem: HeavenlyStem,
    yearly_branch: EarthlyBranch,
) -> ([Decadal; 12], [Vec<u32>; 12]) {
    // 判断顺逆行
    let gender_yinyang = match gender {
        Gender::Male => YinYang::Yang,
        Gender::Female => YinYang::Yin,
    };
    let branch_yinyang = get_earthly_branch_info(yearly_branch).yin_yang;
    let forward = gender_yinyang == branch_yinyang;

    // 寅宫天干
    let start_stem = TIGER_RULE[yearly_stem.index()];

    // 小限
    let age_idx = get_age_index(yearly_branch);

    // Re-arrange ages: for each i, compute idx and place ages there
    let mut final_ages: [Vec<u32>; 12] = std::array::from_fn(|_| Vec::new());
    for i in 0..12 {
        let ages_for_palace: Vec<u32> = (0..10).map(|j| (12 * j + i + 1) as u32).collect();
        let idx = match gender {
            Gender::Male => fix_index(age_idx as i32 + i as i32, 12),
            Gender::Female => fix_index(age_idx as i32 - i as i32, 12),
        };
        final_ages[idx] = ages_for_palace;
    }

    // Re-arrange decadals into proper array indexed by palace position
    let mut final_decadals: [Option<Decadal>; 12] = std::array::from_fn(|_| None);
    for i in 0..12 {
        let idx = if forward {
            fix_index(soul_index as i32 + i as i32, 12)
        } else {
            fix_index(soul_index as i32 - i as i32, 12)
        };

        let start_age = five_elements_class.value() + 10 * i;
        let end_age = start_age + 9;

        let stem_index = fix_index(start_stem.index() as i32 + idx as i32, 10);
        let branch_index = fix_index(2 + idx as i32, 12);

        final_decadals[idx] = Some(Decadal {
            range: (start_age as u32, end_age as u32),
            heavenly_stem: HeavenlyStem::from_index(stem_index),
            earthly_branch: EarthlyBranch::from_index(branch_index),
        });
    }

    let final_decadals: [Decadal; 12] =
        final_decadals.map(|d| d.expect("all decadals should be filled"));

    (final_decadals, final_ages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_soul_and_body() {
        // 甲年，正月(month_index=0)，子时(time_index=0)
        let result = get_soul_and_body(0, 0, HeavenlyStem::Jia);
        // soul_index = fix_index(0-0, 12) = 0 (寅宫)
        assert_eq!(result.soul_index, 0);
        // body_index = fix_index(0+0, 12) = 0
        assert_eq!(result.body_index, 0);
        // 甲年寅宫天干=丙
        assert_eq!(result.heavenly_stem_of_soul, HeavenlyStem::Bing);
        // 宫位索引0 → 地支索引2 → 寅
        assert_eq!(result.earthly_branch_of_soul, EarthlyBranch::Yin);
    }

    #[test]
    fn test_get_soul_and_body_with_offset() {
        // 甲年，三月(month_index=2)，卯时(time_index=3)
        let result = get_soul_and_body(2, 3, HeavenlyStem::Jia);
        // soul_index = fix_index(2-3, 12) = 11
        assert_eq!(result.soul_index, 11);
        // body_index = fix_index(2+3, 12) = 5
        assert_eq!(result.body_index, 5);
    }

    #[test]
    fn test_get_five_elements_class() {
        // 丙寅 → stem_number = 2/2+1 = 2, branch_number = 2/2+1 = 2, sum = 4 → Fire6th
        let result = get_five_elements_class(HeavenlyStem::Bing, EarthlyBranch::Yin);
        assert_eq!(result, FiveElementsClass::Fire6th);

        // 甲子 → stem_number = 0/2+1 = 1, branch_number = 0/2+1 = 1, sum = 2 → Metal4th
        let result = get_five_elements_class(HeavenlyStem::Jia, EarthlyBranch::Zi);
        assert_eq!(result, FiveElementsClass::Metal4th);
    }

    #[test]
    fn test_get_palace_names() {
        // soul_index=0 → names[0]=PALACES[0]=Soul, names[1]=PALACES[1]=Parents, ...
        let names = get_palace_names(0);
        assert_eq!(names[0], Palace::Soul);
        assert_eq!(names[1], Palace::Parents);
        assert_eq!(names[11], Palace::Siblings);

        // soul_index=1 → names[0]=PALACES[-1 mod 12]=PALACES[11]=Siblings
        let names = get_palace_names(1);
        assert_eq!(names[0], Palace::Siblings);
        assert_eq!(names[1], Palace::Soul);
    }

    #[test]
    fn test_get_decadals_and_ages() {
        // Basic smoke test
        let (decadals, ages) = get_decadals_and_ages(
            0,
            FiveElementsClass::Metal4th,
            Gender::Male,
            HeavenlyStem::Jia,
            EarthlyBranch::Zi, // Yang branch
        );
        // Male + Yang branch → forward
        // First decadal at soul_index=0, start_age=4
        assert_eq!(decadals[0].range, (4, 13));
        // All age vectors should have 10 elements
        for age_vec in &ages {
            assert_eq!(age_vec.len(), 10);
        }
    }
}

use crate::data::types::EarthlyBranch;

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

#[cfg(test)]
mod tests {
    use super::*;

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

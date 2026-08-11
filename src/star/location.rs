//! 星耀位置计算
//!
//! 所有函数均为纯函数，接受预计算的索引参数，不调用其他模块。
//! 返回的索引均为宫位索引（0-11），其中 0 = 寅宫。

use crate::data::types::*;
use crate::utils::{earthly_branch_to_palace_index as eb2pi, fix_index};

// ============================================================
// 返回类型
// ============================================================

/// 紫微、天府起始宫位索引
#[derive(Debug, Clone, Copy)]
pub struct StartIndex {
    pub ziwei: usize,
    pub tianfu: usize,
}

/// 禄存、擎羊、陀罗、天马宫位索引
#[derive(Debug, Clone, Copy)]
pub struct LuYangTuoMa {
    pub lu: usize,
    pub yang: usize,
    pub tuo: usize,
    pub ma: usize,
}

/// 天魁、天钺宫位索引
#[derive(Debug, Clone, Copy)]
pub struct KuiYue {
    pub kui: usize,
    pub yue: usize,
}

/// 左辅、右弼宫位索引
#[derive(Debug, Clone, Copy)]
pub struct ZuoYou {
    pub zuo: usize,
    pub you: usize,
}

/// 文昌、文曲宫位索引
#[derive(Debug, Clone, Copy)]
pub struct ChangQu {
    pub chang: usize,
    pub qu: usize,
}

/// 日系星耀索引
#[derive(Debug, Clone, Copy)]
pub struct DailyStar {
    pub santai: usize,
    pub bazuo: usize,
    pub enguang: usize,
    pub tiangui: usize,
}

/// 时系星耀索引（台辅、封诰）
#[derive(Debug, Clone, Copy)]
pub struct TimelyStars {
    pub taifu: usize,
    pub fenggao: usize,
}

/// 地空、地劫宫位索引
#[derive(Debug, Clone, Copy)]
pub struct KongJie {
    pub kong: usize,
    pub jie: usize,
}

/// 火星、铃星宫位索引
#[derive(Debug, Clone, Copy)]
pub struct HuoLing {
    pub huo: usize,
    pub ling: usize,
}

/// 红鸾、天喜宫位索引
#[derive(Debug, Clone, Copy)]
pub struct LuanXi {
    pub hongluan: usize,
    pub tianxi: usize,
}

/// 华盖、咸池宫位索引
#[derive(Debug, Clone, Copy)]
pub struct HuagaiXianchi {
    pub huagai: usize,
    pub xianchi: usize,
}

/// 孤辰、寡宿宫位索引
#[derive(Debug, Clone, Copy)]
pub struct GuGua {
    pub guchen: usize,
    pub guasu: usize,
}

/// 月系星耀索引
#[derive(Debug, Clone, Copy)]
pub struct MonthlyStar {
    pub jieshen: usize,
    pub tianyao: usize,
    pub tianxing: usize,
    pub yinsha: usize,
    pub tianyue: usize,
    pub tianwu: usize,
}

/// 年系星耀索引
#[derive(Debug, Clone, Copy)]
pub struct YearlyStars {
    pub tiancai: usize,
    pub tianshou: usize,
    pub tianchu: usize,
    pub posui: usize,
    pub feilian: usize,
    pub longchi: usize,
    pub fengge: usize,
    pub tianku: usize,
    pub tianxu: usize,
    pub tianguan: usize,
    pub tianfu: usize,
    pub tiande: usize,
    pub yuede: usize,
    pub tiankong: usize,
    pub jielu: usize,
    pub kongwang: usize,
    pub xunkong: usize,
    pub jiekong: usize,
    pub tianshang: usize,
    pub tianshi: usize,
    pub huagai: usize,
    pub xianchi: usize,
    pub guchen: usize,
    pub guasu: usize,
    pub jiesha: usize,
    pub nianjie: usize,
    pub dahao: usize,
    pub hongluan: usize,
    pub tianxi: usize,
}

// ============================================================
// 函数实现
// ============================================================

/// 1. 获取紫微和天府的起始宫位索引
///
/// lunar_day: 农历日（1-30）
/// time_index: 时辰索引（0-12）；晚子时(12)属次日，起紫微用次日日数
/// month_day_count: 该农历月总天数（29/30），次日跨月时回卷到下月初一
/// five_elements_value: 五行局数值（2/3/4/5/6）
pub fn get_start_index(
    lunar_day: u32,
    time_index: u8,
    month_day_count: u32,
    five_elements_value: u32,
) -> StartIndex {
    let mut day = if time_index == 12 { lunar_day + 1 } else { lunar_day };
    if day > month_day_count {
        day -= month_day_count;
    }

    let mut offset: i32 = -1;
    let mut quotient: i32;
    loop {
        offset += 1;
        let divisor = day as i32 + offset;
        quotient = divisor / five_elements_value as i32;
        let remainder = divisor % five_elements_value as i32;
        if remainder == 0 {
            break;
        }
    }
    quotient %= 12;
    let mut ziwei_index = quotient - 1;
    if offset % 2 == 0 {
        ziwei_index += offset;
    } else {
        ziwei_index -= offset;
    }
    let ziwei = fix_index(ziwei_index, 12);
    let tianfu = fix_index(12 - ziwei as i32, 12);
    StartIndex { ziwei, tianfu }
}

/// 2. 获取禄存、擎羊、陀罗、天马的宫位索引
pub fn get_lu_yang_tuo_ma_index(
    stem: HeavenlyStem,
    branch: EarthlyBranch,
) -> LuYangTuoMa {
    use EarthlyBranch::*;
    use HeavenlyStem as HS;

    // 天马
    let ma_branch = match branch {
        Yin | Wu | Xu => Shen,
        Shen | Zi | Chen => Yin,
        Si | You | Chou => Hai,
        Hai | Mao | Wei => Si,
    };
    let ma = eb2pi(ma_branch);

    // 禄存
    let lu_branch = match stem {
        HS::Jia => Yin,
        HS::Yi => Mao,
        HS::Bing | HS::Wu => Si,
        HS::Ding | HS::Ji => Wu,
        HS::Geng => Shen,
        HS::Xin => You,
        HS::Ren => Hai,
        HS::Gui => Zi,
    };
    let lu = eb2pi(lu_branch);
    let yang = fix_index(lu as i32 + 1, 12);
    let tuo = fix_index(lu as i32 - 1, 12);

    LuYangTuoMa { lu, yang, tuo, ma }
}

/// 3. 获取天魁、天钺的宫位索引
pub fn get_kui_yue_index(stem: HeavenlyStem) -> KuiYue {
    use EarthlyBranch::*;
    use HeavenlyStem as HS;

    let (kui_branch, yue_branch) = match stem {
        HS::Jia | HS::Wu | HS::Geng => (Chou, Wei),
        HS::Yi | HS::Ji => (Zi, Shen),
        HS::Xin => (Wu, Yin),
        HS::Bing | HS::Ding => (Hai, You),
        HS::Ren | HS::Gui => (Mao, Si),
    };
    KuiYue {
        kui: eb2pi(kui_branch),
        yue: eb2pi(yue_branch),
    }
}

/// 4. 获取左辅、右弼的宫位索引
///
/// lunar_month: 经闰月修正后的农历月份（1-12，即 fix_lunar_month_index 结果 + 1）
pub fn get_zuo_you_index(lunar_month: u32) -> ZuoYou {
    let zuo = fix_index(
        eb2pi(EarthlyBranch::Chen) as i32 + (lunar_month as i32 - 1),
        12,
    );
    let you = fix_index(
        eb2pi(EarthlyBranch::Xu) as i32 - (lunar_month as i32 - 1),
        12,
    );
    ZuoYou { zuo, you }
}

/// 5. 获取文昌、文曲的宫位索引（按时辰）
pub fn get_chang_qu_index(time_index: u8) -> ChangQu {
    let fixed = fix_index(time_index as i32, 12) as i32;
    let chang = fix_index(eb2pi(EarthlyBranch::Xu) as i32 - fixed, 12);
    let qu = fix_index(eb2pi(EarthlyBranch::Chen) as i32 + fixed, 12);
    ChangQu { chang, qu }
}

/// 6. 获取日系星耀的宫位索引（三台、八座、恩光、天贵）
pub fn get_daily_star_index(
    lunar_day: u32,
    time_index: u8,
    zuo_index: usize,
    you_index: usize,
    chang_index: usize,
    qu_index: usize,
) -> DailyStar {
    let day = if time_index >= 12 {
        lunar_day as i32
    } else {
        lunar_day as i32 - 1
    };
    let santai = fix_index((zuo_index as i32 + day) % 12, 12);
    let bazuo = fix_index((you_index as i32 - day) % 12, 12);
    let enguang = fix_index(((chang_index as i32 + day) % 12) - 1, 12);
    let tiangui = fix_index(((qu_index as i32 + day) % 12) - 1, 12);
    DailyStar {
        santai,
        bazuo,
        enguang,
        tiangui,
    }
}

/// 7. 获取台辅、封诰的宫位索引
pub fn get_timely_star_index(time_index: u8) -> TimelyStars {
    let fixed = fix_index(time_index as i32, 12) as i32;
    let taifu = fix_index(eb2pi(EarthlyBranch::Wu) as i32 + fixed, 12);
    let fenggao = fix_index(eb2pi(EarthlyBranch::Yin) as i32 + fixed, 12);
    TimelyStars { taifu, fenggao }
}

/// 8. 获取地空、地劫的宫位索引
pub fn get_kong_jie_index(time_index: u8) -> KongJie {
    let fixed = fix_index(time_index as i32, 12) as i32;
    let hai = eb2pi(EarthlyBranch::Hai) as i32;
    let kong = fix_index(hai - fixed, 12);
    let jie = fix_index(hai + fixed, 12);
    KongJie { kong, jie }
}

/// 9. 获取火星、铃星的宫位索引
pub fn get_huo_ling_index(branch: EarthlyBranch, time_index: u8) -> HuoLing {
    use EarthlyBranch::*;

    let fixed = fix_index(time_index as i32, 12) as i32;
    let (huo_base, ling_base) = match branch {
        Yin | Wu | Xu => (eb2pi(Chou) as i32, eb2pi(Mao) as i32),
        Shen | Zi | Chen => (eb2pi(Yin) as i32, eb2pi(Xu) as i32),
        Si | You | Chou => (eb2pi(Mao) as i32, eb2pi(Xu) as i32),
        Hai | Mao | Wei => (eb2pi(You) as i32, eb2pi(Xu) as i32),
    };
    let huo = fix_index(huo_base + fixed, 12);
    let ling = fix_index(ling_base + fixed, 12);
    HuoLing { huo, ling }
}

/// 10. 获取红鸾、天喜的宫位索引
pub fn get_luan_xi_index(branch: EarthlyBranch) -> LuanXi {
    let hongluan = fix_index(
        eb2pi(EarthlyBranch::Mao) as i32 - branch.index() as i32,
        12,
    );
    let tianxi = fix_index(hongluan as i32 + 6, 12);
    LuanXi { hongluan, tianxi }
}

/// 11. 获取华盖、咸池的宫位索引
pub fn get_huagai_xianchi_index(branch: EarthlyBranch) -> HuagaiXianchi {
    use EarthlyBranch::*;

    let (huagai_branch, xianchi_branch) = match branch {
        Yin | Wu | Xu => (Xu, Mao),
        Shen | Zi | Chen => (Chen, You),
        Si | You | Chou => (Chou, Wu),
        Hai | Mao | Wei => (Wei, Zi),
    };
    HuagaiXianchi {
        huagai: eb2pi(huagai_branch),
        xianchi: eb2pi(xianchi_branch),
    }
}

/// 12. 获取孤辰、寡宿的宫位索引
pub fn get_gu_gua_index(branch: EarthlyBranch) -> GuGua {
    use EarthlyBranch::*;

    let (guchen_branch, guasu_branch) = match branch {
        Yin | Mao | Chen => (Si, Chou),
        Si | Wu | Wei => (Shen, Chen),
        Shen | You | Xu => (Hai, Wei),
        Hai | Zi | Chou => (Yin, Xu),
    };
    GuGua {
        guchen: eb2pi(guchen_branch),
        guasu: eb2pi(guasu_branch),
    }
}

/// 13. 获取劫煞调整索引
pub fn get_jiesha_adj_index(branch: EarthlyBranch) -> usize {
    use EarthlyBranch::*;
    match branch {
        Shen | Zi | Chen => 3,
        Hai | Mao | Wei => 6,
        Yin | Wu | Xu => 9,
        Si | You | Chou => 0,
    }
}

/// 14. 获取大耗的宫位索引
pub fn get_dahao_index(branch: EarthlyBranch) -> usize {
    use EarthlyBranch::*;
    // 按地支索引查表: [Wei, Wu, You, Shen, Hai, Xu, Chou, Zi, Mao, Yin, Si, Chen]
    let matched: [EarthlyBranch; 12] = [
        Wei, Wu, You, Shen, Hai, Xu, Chou, Zi, Mao, Yin, Si, Chen,
    ];
    let m = matched[branch.index()];
    fix_index(eb2pi(m) as i32 - 2, 12)
}

/// 15. 获取年解的宫位索引
pub fn get_nianjie_index(branch: EarthlyBranch) -> usize {
    use EarthlyBranch::*;
    // 按地支索引查表: [Xu, You, Shen, Wei, Wu, Si, Chen, Mao, Yin, Chou, Zi, Hai]
    let lookup: [EarthlyBranch; 12] = [
        Xu, You, Shen, Wei, Wu, Si, Chen, Mao, Yin, Chou, Zi, Hai,
    ];
    eb2pi(lookup[branch.index()])
}

/// 16. 获取年系星耀的宫位索引
///
/// soul_index: 命宫宫位索引
/// body_index: 身宫宫位索引
/// yearly_stem: 年干
/// yearly_branch: 年支
/// gender: 性别
/// algorithm: 算法
pub fn get_yearly_star_index(
    soul_index: usize,
    body_index: usize,
    yearly_stem: HeavenlyStem,
    yearly_branch: EarthlyBranch,
    gender: Gender,
    algorithm: Algorithm,
) -> YearlyStars {
    use EarthlyBranch::*;

    let branch_idx = yearly_branch.index() as i32;
    let stem_idx = yearly_stem.index();

    // 天才
    let tiancai = fix_index(soul_index as i32 + branch_idx, 12);
    // 天寿
    let tianshou = fix_index(body_index as i32 + branch_idx, 12);

    // 天厨：按天干查表 [Si, Wu, Zi, Si, Wu, Shen, Yin, Wu, You, Hai]
    let tianchu_table: [EarthlyBranch; 10] = [
        Si, Wu, Zi, Si, Wu, Shen, Yin, Wu, You, Hai,
    ];
    let tianchu = eb2pi(tianchu_table[stem_idx]);

    // 破碎：按地支索引 % 3 查表 [Si, Chou, You]
    let posui_table: [EarthlyBranch; 3] = [Si, Chou, You];
    let posui = eb2pi(posui_table[yearly_branch.index() % 3]);

    // 蜚廉：12元素查表
    let feilian_table: [EarthlyBranch; 12] = [
        Shen, You, Xu, Si, Wu, Wei, Yin, Mao, Chen, Hai, Zi, Chou,
    ];
    let feilian = eb2pi(feilian_table[yearly_branch.index()]);

    // 龙池
    let longchi = fix_index(eb2pi(Chen) as i32 + branch_idx, 12);
    // 凤阁
    let fengge = fix_index(eb2pi(Xu) as i32 - branch_idx, 12);

    // 天哭
    let tianku = fix_index(eb2pi(Wu) as i32 - branch_idx, 12);
    // 天虚
    let tianxu = fix_index(eb2pi(Wu) as i32 + branch_idx, 12);

    // 天官：10元素按天干查表
    let tianguan_table: [EarthlyBranch; 10] = [
        Wei, Chen, Si, Yin, Mao, You, Hai, You, Xu, Wu,
    ];
    let tianguan = eb2pi(tianguan_table[stem_idx]);

    // 天福：10元素按天干查表
    let tianfu_table: [EarthlyBranch; 10] = [
        You, Shen, Zi, Hai, Mao, Yin, Wu, Si, Wu, Si,
    ];
    let tianfu = eb2pi(tianfu_table[stem_idx]);

    // 天德
    let tiande = fix_index(eb2pi(You) as i32 + branch_idx, 12);
    // 月德
    let yuede = fix_index(eb2pi(Si) as i32 + branch_idx, 12);

    // 天空
    let tiankong = fix_index(eb2pi(yearly_branch) as i32 + 1, 12);

    // 截路：5元素按天干 % 5 查表 [Shen, Wu, Chen, Yin, Zi]
    let jielu_table: [EarthlyBranch; 5] = [Shen, Wu, Chen, Yin, Zi];
    let jielu = eb2pi(jielu_table[stem_idx % 5]);

    // 空亡：5元素按天干 % 5 查表 [You, Wei, Si, Mao, Chou]
    let kongwang_table: [EarthlyBranch; 5] = [You, Wei, Si, Mao, Chou];
    let kongwang = eb2pi(kongwang_table[stem_idx % 5]);

    // 旬空：从年支宫位起，加上年干到癸干的距离再进一位
    let xunkong_raw = fix_index(
        eb2pi(yearly_branch) as i32 + 9 - stem_idx as i32 + 1,
        12,
    );
    // 旬空落宫的奇偶须与年支索引的奇偶一致，不一致时进一位
    let branch_is_yang = yearly_branch.index().is_multiple_of(2);
    let xunkong = if yearly_branch.index() % 2 != xunkong_raw % 2 {
        fix_index(xunkong_raw as i32 + 1, 12)
    } else {
        xunkong_raw
    };

    // 劫空：地支为偶(阳)用截路，奇(阴)用空亡
    let jiekong = if branch_is_yang { jielu } else { kongwang };

    // 天伤 = PALACES[Friends] + soul_index = 5 + soul_index
    let mut tianshang = fix_index(5 + soul_index as i32, 12);
    // 天使 = PALACES[Health] + soul_index = 7 + soul_index
    let mut tianshi = fix_index(7 + soul_index as i32, 12);

    // 中州派算法：如果性别阴阳与年支阴阳不同，则交换天伤天使
    if algorithm == Algorithm::Zhongzhou {
        let gender_is_yang = gender == Gender::Male;
        if gender_is_yang != branch_is_yang {
            std::mem::swap(&mut tianshang, &mut tianshi);
        }
    }

    // 华盖、咸池
    let hx = get_huagai_xianchi_index(yearly_branch);
    // 孤辰、寡宿
    let gg = get_gu_gua_index(yearly_branch);
    // 劫煞
    let jiesha = get_jiesha_adj_index(yearly_branch);
    // 年解
    let nianjie = get_nianjie_index(yearly_branch);
    // 大耗
    let dahao = get_dahao_index(yearly_branch);
    // 红鸾、天喜
    let lx = get_luan_xi_index(yearly_branch);

    YearlyStars {
        tiancai,
        tianshou,
        tianchu,
        posui,
        feilian,
        longchi,
        fengge,
        tianku,
        tianxu,
        tianguan,
        tianfu,
        tiande,
        yuede,
        tiankong,
        jielu,
        kongwang,
        xunkong,
        jiekong,
        tianshang,
        tianshi,
        huagai: hx.huagai,
        xianchi: hx.xianchi,
        guchen: gg.guchen,
        guasu: gg.guasu,
        jiesha,
        nianjie,
        dahao,
        hongluan: lx.hongluan,
        tianxi: lx.tianxi,
    }
}

/// 17. 获取月系星耀的宫位索引
///
/// month_index: 月份索引（0-based，即正月=0）
pub fn get_monthly_star_index(month_index: usize) -> MonthlyStar {
    use EarthlyBranch::*;

    // 解神：按 month_index / 2 查表 [Shen, Xu, Zi, Yin, Chen, Wu]
    let jieshen_table: [EarthlyBranch; 6] = [Shen, Xu, Zi, Yin, Chen, Wu];
    let jieshen = eb2pi(jieshen_table[month_index / 2]);

    // 天姚
    let tianyao = fix_index(
        eb2pi(Chou) as i32 + month_index as i32,
        12,
    );

    // 天刑
    let tianxing = fix_index(
        eb2pi(You) as i32 + month_index as i32,
        12,
    );

    // 阴煞：按 month_index % 6 查表 [Yin, Zi, Xu, Shen, Wu, Chen]
    let yinsha_table: [EarthlyBranch; 6] = [Yin, Zi, Xu, Shen, Wu, Chen];
    let yinsha = eb2pi(yinsha_table[month_index % 6]);

    // 天月：12元素查表 [Xu, Si, Chen, Yin, Wei, Mao, Hai, Wei, Yin, Wu, Xu, Yin]
    let tianyue_table: [EarthlyBranch; 12] = [
        Xu, Si, Chen, Yin, Wei, Mao, Hai, Wei, Yin, Wu, Xu, Yin,
    ];
    let tianyue = eb2pi(tianyue_table[month_index]);

    // 天巫：按 month_index % 4 查表 [Si, Shen, Yin, Hai]
    let tianwu_table: [EarthlyBranch; 4] = [Si, Shen, Yin, Hai];
    let tianwu = eb2pi(tianwu_table[month_index % 4]);

    MonthlyStar {
        jieshen,
        tianyao,
        tianxing,
        yinsha,
        tianyue,
        tianwu,
    }
}

/// 18. 获取文昌、文曲的宫位索引（按天干）
pub fn get_chang_qu_index_by_stem(stem: HeavenlyStem) -> ChangQu {
    use EarthlyBranch::*;
    use HeavenlyStem as HS;

    let (chang_branch, qu_branch) = match stem {
        HS::Jia => (Si, You),
        HS::Yi => (Wu, Shen),
        HS::Bing | HS::Wu => (Shen, Wu),
        HS::Ding | HS::Ji => (You, Si),
        HS::Geng => (Hai, Mao),
        HS::Xin => (Zi, Yin),
        HS::Ren => (Yin, Zi),
        HS::Gui => (Mao, Hai),
    };
    ChangQu {
        chang: eb2pi(chang_branch),
        qu: eb2pi(qu_branch),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use EarthlyBranch::*;
    use HeavenlyStem as HS;

    #[test]
    fn test_get_start_index() {
        // 水二局，初一
        let si = get_start_index(1, 0, 30, 2);
        // 1+1=2, 2/2=1, remainder=0, offset=1(odd), quotient=1
        // ziwei = 1-1-1 = -1, fix_index(-1,12)=11
        assert_eq!(si.ziwei, 11);
        assert_eq!(si.tianfu, fix_index(12 - 11, 12));
    }

    #[test]
    fn test_get_start_index_late_zi() {
        // 晚子时按次日起紫微：初一晚子时等同初二
        let late = get_start_index(1, 12, 30, 2);
        let next_day = get_start_index(2, 0, 30, 2);
        assert_eq!(late.ziwei, next_day.ziwei);

        // 月末晚子时跨月回卷到初一
        let wrapped = get_start_index(29, 12, 29, 2);
        let first_day = get_start_index(1, 0, 29, 2);
        assert_eq!(wrapped.ziwei, first_day.ziwei);
    }

    #[test]
    fn test_get_lu_yang_tuo_ma() {
        let r = get_lu_yang_tuo_ma_index(HS::Jia, Yin);
        // 甲→禄存在寅, 寅的palace_index=0
        assert_eq!(r.lu, 0);
        assert_eq!(r.yang, 1);
        assert_eq!(r.tuo, 11);
        // 寅午戌→天马在申, 申的palace_index=6
        assert_eq!(r.ma, 6);
    }

    #[test]
    fn test_get_kui_yue() {
        let r = get_kui_yue_index(HS::Jia);
        // 甲→丑/未
        assert_eq!(r.kui, eb2pi(Chou));
        assert_eq!(r.yue, eb2pi(Wei));
    }

    #[test]
    fn test_get_zuo_you() {
        let r = get_zuo_you_index(1);
        // 正月: zuo=辰, you=戌
        assert_eq!(r.zuo, eb2pi(Chen));
        assert_eq!(r.you, eb2pi(Xu));
    }

    #[test]
    fn test_get_chang_qu() {
        let r = get_chang_qu_index(0);
        // time=0: chang=戌, qu=辰
        assert_eq!(r.chang, eb2pi(Xu));
        assert_eq!(r.qu, eb2pi(Chen));
    }

    #[test]
    fn test_get_kong_jie() {
        let r = get_kong_jie_index(0);
        // time=0: kong=亥, jie=亥
        assert_eq!(r.kong, eb2pi(Hai));
        assert_eq!(r.jie, eb2pi(Hai));
    }

    #[test]
    fn test_get_luan_xi() {
        let r = get_luan_xi_index(Zi);
        // hongluan = fix_index(eb2pi(Mao) - 0, 12) = eb2pi(Mao) = 1
        assert_eq!(r.hongluan, eb2pi(Mao));
        assert_eq!(r.tianxi, fix_index(eb2pi(Mao) as i32 + 6, 12));
    }

    #[test]
    fn test_get_huagai_xianchi() {
        let r = get_huagai_xianchi_index(Yin);
        assert_eq!(r.huagai, eb2pi(Xu));
        assert_eq!(r.xianchi, eb2pi(Mao));
    }

    #[test]
    fn test_get_gu_gua() {
        let r = get_gu_gua_index(Yin);
        assert_eq!(r.guchen, eb2pi(Si));
        assert_eq!(r.guasu, eb2pi(Chou));
    }

    #[test]
    fn test_get_jiesha_adj() {
        assert_eq!(get_jiesha_adj_index(Zi), 3);
        assert_eq!(get_jiesha_adj_index(Yin), 9);
        assert_eq!(get_jiesha_adj_index(Si), 0);
        assert_eq!(get_jiesha_adj_index(Hai), 6);
    }

    #[test]
    fn test_get_chang_qu_by_stem() {
        let r = get_chang_qu_index_by_stem(HS::Jia);
        assert_eq!(r.chang, eb2pi(Si));
        assert_eq!(r.qu, eb2pi(You));
    }
}

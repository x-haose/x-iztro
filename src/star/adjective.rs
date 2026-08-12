//! 杂耀安放
//!
//! 将约38颗杂耀安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::star::Star;
use crate::star::location::{DailyStar, MonthlyStar, YearlyStars};

/// 创建一颗杂耀（无亮度、无四化）
fn make_adj_star(key: StarKey, star_type: StarType, _palace_index: usize, lang: Language) -> Star {
    Star {
        key,
        name: translate_star(key, lang).to_string(),
        star_type,
        scope: Scope::Origin,
        brightness: None,
        mutagen: None,
    }
}

/// 获取杂耀的安放结果
#[allow(clippy::too_many_arguments)]
pub fn get_adjective_stars(
    yearly_stars: &YearlyStars,
    monthly_stars: &MonthlyStar,
    daily_stars: &DailyStar,
    timely_taifu: usize,
    timely_fenggao: usize,
    hongluan_index: usize,
    tianxi_index: usize,
    xianchi_index: usize,
    suiqian12: &[StarKey; 12],
    algorithm: Algorithm,
    lang: Language,
) -> [Vec<Star>; 12] {
    let mut result: [Vec<Star>; 12] = Default::default();

    // 安放顺序决定宫内星耀排列：
    // 桃花四星 → 解神 → 日/年/时/月系杂耀 → 派别专属星 → 孤寡等 → 年解收尾
    let head: [(usize, StarKey, StarType); 25] = [
        (hongluan_index, StarKey::Hongluan, StarType::Flower),
        (tianxi_index, StarKey::Tianxi, StarType::Flower),
        (monthly_stars.tianyao, StarKey::Tianyao, StarType::Flower),
        (xianchi_index, StarKey::Xianchi, StarType::Flower),
        (monthly_stars.jieshen, StarKey::Jieshen, StarType::Helper),
        (daily_stars.santai, StarKey::Santai, StarType::Adjective),
        (daily_stars.bazuo, StarKey::Bazuo, StarType::Adjective),
        (daily_stars.enguang, StarKey::Engguang, StarType::Adjective),
        (daily_stars.tiangui, StarKey::Tiangui, StarType::Adjective),
        (yearly_stars.longchi, StarKey::Longchi, StarType::Adjective),
        (yearly_stars.fengge, StarKey::Fengge, StarType::Adjective),
        (yearly_stars.tiancai, StarKey::Tiancai, StarType::Adjective),
        (
            yearly_stars.tianshou,
            StarKey::Tianshou,
            StarType::Adjective,
        ),
        (timely_taifu, StarKey::Taifu, StarType::Adjective),
        (timely_fenggao, StarKey::Fenggao, StarType::Adjective),
        (monthly_stars.tianwu, StarKey::Tianwu, StarType::Adjective),
        (yearly_stars.huagai, StarKey::Huagai, StarType::Adjective),
        (
            yearly_stars.tianguan,
            StarKey::Tianguan,
            StarType::Adjective,
        ),
        (yearly_stars.tianfu, StarKey::Tianfu, StarType::Adjective),
        (yearly_stars.tianchu, StarKey::Tianchu, StarType::Adjective),
        (
            monthly_stars.tianyue,
            StarKey::Tianyue2,
            StarType::Adjective,
        ),
        (yearly_stars.tiande, StarKey::Tiande, StarType::Adjective),
        (yearly_stars.yuede, StarKey::Yuede, StarType::Adjective),
        (
            yearly_stars.tiankong,
            StarKey::Tiankong,
            StarType::Adjective,
        ),
        (yearly_stars.xunkong, StarKey::Xunkong, StarType::Adjective),
    ];
    for (idx, key, star_type) in head {
        result[idx].push(make_adj_star(key, star_type, idx, lang));
    }

    // 派别专属：默认派安截路空亡；中州派安龙德（取岁前12中龙德所落宫）、截空、劫杀、大耗
    if algorithm != Algorithm::Zhongzhou {
        result[yearly_stars.jielu].push(make_adj_star(
            StarKey::Jielu,
            StarType::Adjective,
            yearly_stars.jielu,
            lang,
        ));
        result[yearly_stars.kongwang].push(make_adj_star(
            StarKey::Kongwang,
            StarType::Adjective,
            yearly_stars.kongwang,
            lang,
        ));
    } else {
        if let Some(longde_pos) = suiqian12.iter().position(|&k| k == StarKey::Longde) {
            result[longde_pos].push(make_adj_star(
                StarKey::Longde,
                StarType::Adjective,
                longde_pos,
                lang,
            ));
        }
        result[yearly_stars.jiekong].push(make_adj_star(
            StarKey::Jiekong,
            StarType::Adjective,
            yearly_stars.jiekong,
            lang,
        ));
        result[yearly_stars.jiesha].push(make_adj_star(
            StarKey::JieshaAdj,
            StarType::Adjective,
            yearly_stars.jiesha,
            lang,
        ));
        result[yearly_stars.dahao].push(make_adj_star(
            StarKey::Dahao,
            StarType::Adjective,
            yearly_stars.dahao,
            lang,
        ));
    }

    let tail: [(usize, StarKey, StarType); 11] = [
        (yearly_stars.guchen, StarKey::Guchen, StarType::Adjective),
        (yearly_stars.guasu, StarKey::Guasu, StarType::Adjective),
        (yearly_stars.feilian, StarKey::Feilian, StarType::Adjective),
        (yearly_stars.posui, StarKey::Posui, StarType::Adjective),
        (
            monthly_stars.tianxing,
            StarKey::Tianxing,
            StarType::Adjective,
        ),
        (monthly_stars.yinsha, StarKey::Yinsha, StarType::Adjective),
        (yearly_stars.tianku, StarKey::Tianku, StarType::Adjective),
        (yearly_stars.tianxu, StarKey::Tianxu, StarType::Adjective),
        (yearly_stars.tianshi, StarKey::Tianshi, StarType::Adjective),
        (
            yearly_stars.tianshang,
            StarKey::Tianshang,
            StarType::Adjective,
        ),
        (yearly_stars.nianjie, StarKey::Nianjie, StarType::Helper),
    ];
    for (idx, key, star_type) in tail {
        result[idx].push(make_adj_star(key, star_type, idx, lang));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::star::location;

    #[test]
    fn test_adjective_stars_default() {
        let yearly = location::get_yearly_star_index(
            0,
            6,
            HeavenlyStem::Jia,
            EarthlyBranch::Zi,
            Gender::Male,
            Algorithm::Default,
        );
        let monthly = location::get_monthly_star_index(0);
        let zuo_you = location::get_zuo_you_index(1);
        let chang_qu = location::get_chang_qu_index(0);
        let daily = location::get_daily_star_index(
            1,
            0,
            zuo_you.zuo,
            zuo_you.you,
            chang_qu.chang,
            chang_qu.qu,
        );
        let timely = location::get_timely_star_index(0);
        let luan_xi = location::get_luan_xi_index(EarthlyBranch::Zi);
        let suiqian12 = [StarKey::Suijian; 12]; // placeholder

        let result = get_adjective_stars(
            &yearly,
            &monthly,
            &daily,
            timely.taifu,
            timely.fenggao,
            luan_xi.hongluan,
            luan_xi.tianxi,
            yearly.xianchi,
            &suiqian12,
            Algorithm::Default,
            Language::ZhCN,
        );
        let total: usize = result.iter().map(|v| v.len()).sum();
        // 4 flower + 2 helper + 30 adjective + 2 (jielu + kongwang) = 38
        assert_eq!(total, 38);
    }
}

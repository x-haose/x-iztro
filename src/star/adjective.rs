//! 杂耀安放
//!
//! 将约38颗杂耀安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::star::Star;
use crate::star::location::{DailyStar, MonthlyStar, YearlyStars};

/// 创建一颗杂耀（无亮度、无四化）
fn make_adj_star(
    key: StarKey,
    star_type: StarType,
    _palace_index: usize,
    lang: Language,
) -> Star {
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

    // 桃花星（Flower）
    let flower_stars: [(usize, StarKey); 4] = [
        (hongluan_index, StarKey::Hongluan),
        (tianxi_index, StarKey::Tianxi),
        (monthly_stars.tianyao, StarKey::Tianyao),
        (xianchi_index, StarKey::Xianchi),
    ];
    for (idx, key) in flower_stars {
        result[idx].push(make_adj_star(key, StarType::Flower, idx, lang));
    }

    // 解神类（Helper）
    let helper_stars: [(usize, StarKey); 2] = [
        (monthly_stars.jieshen, StarKey::Jieshen),
        (yearly_stars.nianjie, StarKey::Nianjie),
    ];
    for (idx, key) in helper_stars {
        result[idx].push(make_adj_star(key, StarType::Helper, idx, lang));
    }

    // 杂耀（Adjective）
    let adjective_stars: [(usize, StarKey); 30] = [
        (yearly_stars.huagai, StarKey::Huagai),
        (yearly_stars.tiande, StarKey::Tiande),
        (yearly_stars.tiankong, StarKey::Tiankong),
        (monthly_stars.tianxing, StarKey::Tianxing),
        (monthly_stars.yinsha, StarKey::Yinsha),
        (yearly_stars.tianguan, StarKey::Tianguan),
        (yearly_stars.tianfu, StarKey::Tianfu),
        (yearly_stars.tianku, StarKey::Tianku),
        (yearly_stars.tianxu, StarKey::Tianxu),
        (yearly_stars.longchi, StarKey::Longchi),
        (yearly_stars.fengge, StarKey::Fengge),
        (yearly_stars.guchen, StarKey::Guchen),
        (yearly_stars.guasu, StarKey::Guasu),
        (yearly_stars.feilian, StarKey::Feilian),
        (yearly_stars.posui, StarKey::Posui),
        (timely_taifu, StarKey::Taifu),
        (timely_fenggao, StarKey::Fenggao),
        (monthly_stars.tianwu, StarKey::Tianwu),
        (monthly_stars.tianyue, StarKey::Tianyue2),
        (daily_stars.santai, StarKey::Santai),
        (daily_stars.bazuo, StarKey::Bazuo),
        (daily_stars.enguang, StarKey::Engguang),
        (daily_stars.tiangui, StarKey::Tiangui),
        (yearly_stars.tiancai, StarKey::Tiancai),
        (yearly_stars.tianshou, StarKey::Tianshou),
        (yearly_stars.tianchu, StarKey::Tianchu),
        (yearly_stars.xunkong, StarKey::Xunkong),
        (yearly_stars.yuede, StarKey::Yuede),
        (yearly_stars.tianshang, StarKey::Tianshang),
        (yearly_stars.tianshi, StarKey::Tianshi),
    ];
    for (idx, key) in adjective_stars {
        result[idx].push(make_adj_star(key, StarType::Adjective, idx, lang));
    }

    // 按算法区分
    if algorithm != Algorithm::Zhongzhou {
        // 截路、空亡
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
        // 中州派：龙德、劫空、劫煞、大耗
        // 龙德：从岁前12神中找到龙德的位置
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

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::star::location;

    #[test]
    fn test_adjective_stars_default() {
        let yearly = location::get_yearly_star_index(
            0, 6,
            HeavenlyStem::Jia,
            EarthlyBranch::Zi,
            Gender::Male,
            Algorithm::Default,
        );
        let monthly = location::get_monthly_star_index(0);
        let zuo_you = location::get_zuo_you_index(1);
        let chang_qu = location::get_chang_qu_index(0);
        let daily = location::get_daily_star_index(1, 0, zuo_you.zuo, zuo_you.you, chang_qu.chang, chang_qu.qu);
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

//! 杂耀安放
//!
//! 将约38颗杂耀安放到12宫中。

use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::star::Star;
use crate::star::location::{DailyStar, MonthlyStar, YearlyStars};

/// 创建一颗落在 `palace_index`（寅宫为 0）的杂耀。
///
/// 亮度按 `config` 的亮度表查：默认表不给杂耀亮度，只有自定义亮度表列出该星时才有；杂耀不带四化。
pub(crate) fn make_adj_star(
    key: StarKey,
    star_type: StarType,
    palace_index: usize,
    lang: Language,
    config: &Config,
) -> Star {
    Star {
        key,
        name: translate_star(key, lang).to_string(),
        star_type,
        scope: Scope::Origin,
        brightness: config.brightness_of(key, palace_index),
        mutagen: None,
    }
}

/// 获取杂耀的安放结果
///
/// `yuema` 为月马的宫位索引，只在天马兼安年马、月马（[`TianmaPlacement::Both`]）时给出。
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
    yuema: Option<usize>,
    lang: Language,
    config: &Config,
) -> [Vec<Star>; 12] {
    let mut result: [Vec<Star>; 12] = Default::default();
    let mut place = |idx: usize, key: StarKey, star_type: StarType| {
        result[idx].push(make_adj_star(key, star_type, idx, lang, config));
    };
    let zheng_pang = config.void_placement == VoidPlacement::ZhengPang;

    // 安放顺序决定宫内星耀排列：
    // 桃花四星 → 解神 → 日/年/时/月系杂耀（旬空分正傍时傍旬空紧随旬空）→ 派别专属星 → 孤寡等 → 年解 → 月马收尾
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
        place(idx, key, star_type);
    }
    if zheng_pang {
        place(
            yearly_stars.pang_xunkong,
            StarKey::PangXunkong,
            StarType::Adjective,
        );
    }

    // 派别专属：默认派安截路空亡（截空分正傍时改安截空、傍截空）；
    // 中州派安龙德（取岁前12中龙德所落宫）、截空（分正傍时另安傍截空）、劫杀、大耗
    let mut jiekong = vec![(yearly_stars.jiekong, StarKey::Jiekong)];
    if zheng_pang {
        jiekong.push((yearly_stars.pang_jiekong, StarKey::PangJiekong));
    }
    let sect: Vec<(usize, StarKey)> = if config.algorithm == Algorithm::Zhongzhou {
        let longde = suiqian12
            .iter()
            .position(|&k| k == StarKey::Longde)
            .map(|pos| (pos, StarKey::Longde));
        longde
            .into_iter()
            .chain(jiekong)
            .chain([
                (yearly_stars.jiesha, StarKey::JieshaAdj),
                (yearly_stars.dahao, StarKey::Dahao),
            ])
            .collect()
    } else if zheng_pang {
        jiekong
    } else {
        vec![
            (yearly_stars.jielu, StarKey::Jielu),
            (yearly_stars.kongwang, StarKey::Kongwang),
        ]
    };
    for (idx, key) in sect {
        place(idx, key, StarType::Adjective);
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
        place(idx, key, star_type);
    }
    if let Some(idx) = yuema {
        place(idx, StarKey::YuemaAdj, StarType::Tianma);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::star::location;

    /// 各空曜所在宫的地支；不在盘上为 `None`
    fn void_branches(date: &str, config: Config) -> [(StarKey, Option<EarthlyBranch>); 6] {
        let chart = crate::by_solar(date, 2, Gender::Female, true, Language::ZhCN, config).unwrap();
        [
            StarKey::Jielu,
            StarKey::Kongwang,
            StarKey::Jiekong,
            StarKey::PangJiekong,
            StarKey::Xunkong,
            StarKey::PangXunkong,
        ]
        .map(|k| (k, chart.star(k).map(|s| s.palace().earthly_branch)))
    }

    /// 截空、旬空分正傍：阳年阳宫为正空、阴年阴宫为正空，另一宫为傍空；默认派不再安截路空亡。
    ///
    /// 2000 年庚辰（阳）：截空午未、旬空（甲戌旬）申酉，正空午、申；
    /// 2001 年辛巳（阴）：截空辰巳、旬空申酉，正空巳、酉。
    #[test]
    fn zheng_pang_places_main_and_side_voids() {
        use EarthlyBranch::*;
        let zheng_pang = Config::default().with_void_placement(VoidPlacement::ZhengPang);
        assert_eq!(
            void_branches("2000-8-16", zheng_pang.clone()),
            [
                (StarKey::Jielu, None),
                (StarKey::Kongwang, None),
                (StarKey::Jiekong, Some(Wu)),
                (StarKey::PangJiekong, Some(Wei)),
                (StarKey::Xunkong, Some(Shen)),
                (StarKey::PangXunkong, Some(You)),
            ]
        );
        assert_eq!(
            void_branches("2001-8-16", zheng_pang.clone()),
            [
                (StarKey::Jielu, None),
                (StarKey::Kongwang, None),
                (StarKey::Jiekong, Some(Si)),
                (StarKey::PangJiekong, Some(Chen)),
                (StarKey::Xunkong, Some(You)),
                (StarKey::PangXunkong, Some(Shen)),
            ]
        );
        // 中州派原本只安正空，分正傍后另安傍空
        let zhongzhou = Config {
            algorithm: Algorithm::Zhongzhou,
            ..zheng_pang
        };
        assert_eq!(
            void_branches("2001-8-16", zhongzhou),
            [
                (StarKey::Jielu, None),
                (StarKey::Kongwang, None),
                (StarKey::Jiekong, Some(Si)),
                (StarKey::PangJiekong, Some(Chen)),
                (StarKey::Xunkong, Some(You)),
                (StarKey::PangXunkong, Some(Shen)),
            ]
        );
    }

    /// 默认安法与 iztro 一致：默认派截路在阳宫、空亡在阴宫，中州派只在正空安截空，旬空只安正空。
    #[test]
    fn default_placement_keeps_iztro_voids() {
        use EarthlyBranch::*;
        assert_eq!(
            void_branches("2001-8-16", Config::default()),
            [
                (StarKey::Jielu, Some(Chen)),
                (StarKey::Kongwang, Some(Si)),
                (StarKey::Jiekong, None),
                (StarKey::PangJiekong, None),
                (StarKey::Xunkong, Some(You)),
                (StarKey::PangXunkong, None),
            ]
        );
        let zhongzhou = Config {
            algorithm: Algorithm::Zhongzhou,
            ..Config::default()
        };
        assert_eq!(
            void_branches("2001-8-16", zhongzhou),
            [
                (StarKey::Jielu, None),
                (StarKey::Kongwang, None),
                (StarKey::Jiekong, Some(Si)),
                (StarKey::PangJiekong, None),
                (StarKey::Xunkong, Some(You)),
                (StarKey::PangXunkong, None),
            ]
        );
    }

    #[test]
    fn test_adjective_stars_default() {
        let yearly = location::get_yearly_star_index(
            0,
            6,
            HeavenlyStem::Jia,
            EarthlyBranch::Zi,
            Gender::Male,
            &Config::default(),
        );
        let monthly = location::get_monthly_star_index(0, TianwuPlacement::SiShenYinHai);
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
            None,
            Language::ZhCN,
            &Config::default(),
        );
        let total: usize = result.iter().map(|v| v.len()).sum();
        // 4 flower + 2 helper + 30 adjective + 2 (jielu + kongwang) = 38
        assert_eq!(total, 38);
    }
}

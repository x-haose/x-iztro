//! 按指定干支重排星盘
//!
//! 中州派把同一组出生数据看作三张盘：以命宫干支起五行局的**天盘**（常规排盘结果）、
//! 以身宫干支起的**地盘**、以福德宫干支起的**人盘**。三者的差别在于起五行局的干支
//! 不同，由此紫微天府的落点、十二宫名、长生十二神、大限小限全部改变。
//!
//! 本模块提供的重排以任意干支为命宫重来一遍这几步，其余星耀沿用原盘。

use crate::astro::lunar_table;
use crate::astro::palace::{get_decadals_and_ages, get_five_elements_class, get_palace_names};
use crate::data::earthly_branches::get_earthly_branch_info;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::i18n::translate_star;
use crate::models::astrolabe::Astrolabe;
use crate::models::star::Star;
use crate::star::decorative::get_changsheng12;
use crate::star::location::{get_start_index, get_tiancai_index, get_tianshang_tianshi_index};
use crate::star::major::get_major_stars;
use crate::utils::{earthly_branch_to_palace_index, fix_index};

/// 按时辰索引取身宫相对命宫的偏移格数。
///
/// 重排时命宫由传入干支直接指定，身宫不能再用「月支顺数生时」推，
/// 改由本表按生时定：子午同宫、丑未隔两格，依此类推，晚子时同早子时。
const BODY_OFFSET: [usize; 13] = [0, 2, 4, 6, 8, 10, 0, 2, 4, 6, 8, 10, 0];

impl Astrolabe {
    /// 以指定干支为命宫重排本盘，返回新盘；原盘不变。
    ///
    /// 传入的干支决定五行局，进而决定紫微天府落点、十二宫名、长生十二神与大限小限；
    /// 辅星、杂耀（天伤天使天才除外）、博士十二神、岁前将前十二神沿用原盘。
    ///
    /// 常规的天盘、地盘、人盘用 [`Config::astro_type`] 指定即可，
    /// 本方法用于从任意干支起盘。
    pub fn rearranged(&self, from_stem: HeavenlyStem, from_branch: EarthlyBranch) -> Astrolabe {
        let mut chart = self.clone();

        // 晚子时算当天时按早子时安星，与排盘入口的处理一致
        let effective_ti = if self.config.day_divide == DayDivide::Current && self.time_index >= 12
        {
            0
        } else {
            self.time_index
        };

        let soul_index = earthly_branch_to_palace_index(from_branch);
        let body_index = fix_index((BODY_OFFSET[effective_ti as usize] + soul_index) as i32, 12);

        let five_elements_class = get_five_elements_class(from_stem, from_branch);
        let palace_names = get_palace_names(soul_index);

        // 紫微天府按新五行局重新起点，主星随之全部重安
        let lunar = self.raw_dates.lunar_date;
        let signed_month = if lunar.is_leap {
            -(lunar.lunar_month as i64)
        } else {
            lunar.lunar_month as i64
        };
        let month_day_count = lunar_table::month_day_count(lunar.lunar_year, signed_month)
            .expect("已排盘星盘的 raw_dates 农历月必然存在于月表")
            as u32;
        let start_idx = get_start_index(
            lunar.lunar_day,
            effective_ti,
            month_day_count,
            five_elements_class.value() as u32,
        );

        let (yearly_stem, yearly_branch) = self.raw_dates.chinese_date.yearly;
        let major_stars = get_major_stars(
            start_idx.ziwei,
            start_idx.tianfu,
            yearly_stem,
            self.language,
            &self.config,
        );
        let changsheng12 = get_changsheng12(five_elements_class, self.gender, yearly_branch);
        let (decadals, ages) = get_decadals_and_ages(
            soul_index,
            five_elements_class,
            self.gender,
            yearly_stem,
            yearly_branch,
        );

        // 天伤天使夹迁移宫、天才随命宫，三者都跟着命宫走，需要在原有杂耀里挪位
        let (tianshang_index, tianshi_index) = get_tianshang_tianshi_index(
            self.gender,
            yearly_branch,
            soul_index,
            self.config.algorithm,
        );
        let tiancai_index = get_tiancai_index(yearly_branch, soul_index);

        for (i, palace) in chart.palaces.iter_mut().enumerate() {
            relocate_adjective_star(
                &mut palace.adjective_stars,
                StarKey::Tianshang,
                i == tianshang_index,
                self.language,
            );
            relocate_adjective_star(
                &mut palace.adjective_stars,
                StarKey::Tianshi,
                i == tianshi_index,
                self.language,
            );
            relocate_adjective_star(
                &mut palace.adjective_stars,
                StarKey::Tiancai,
                i == tiancai_index,
                self.language,
            );

            palace.name = palace_names[i];
            palace.major_stars = major_stars[i].clone();
            palace.changsheng12 = changsheng12[i];
            palace.decadal = decadals[i].clone();
            palace.ages = ages[i].clone();
            palace.is_body_palace = body_index == i;
        }

        chart.five_elements_class = five_elements_class;
        chart.earthly_branch_of_soul_palace = chart.palaces[soul_index].earthly_branch;
        chart.earthly_branch_of_body_palace = chart.palaces[body_index].earthly_branch;

        // 命主星按命宫地支查表，命宫既已挪位就要重查；
        // 中州派改按年支取，年支不随重排改变，因此维持原值。
        // 身主星一律按年支取，同理不变。
        if self.config.algorithm != Algorithm::Zhongzhou {
            chart.soul = get_earthly_branch_info(chart.earthly_branch_of_soul_palace).soul;
        }

        chart
    }
}

/// 让某颗杂耀在本宫「该在就在、不该在就不在」。
///
/// 该在却不在时追加到杂耀列表末尾 —— 位置与初次排盘时不同，这是重排的既有行为。
fn relocate_adjective_star(
    stars: &mut Vec<Star>,
    key: StarKey,
    should_be_here: bool,
    lang: Language,
) {
    match (stars.iter().position(|s| s.key == key), should_be_here) {
        (Some(pos), false) => {
            stars.remove(pos);
        }
        (None, true) => stars.push(Star {
            key,
            name: translate_star(key, lang).to_string(),
            star_type: StarType::Adjective,
            scope: Scope::Origin,
            brightness: None,
            mutagen: None,
        }),
        _ => {}
    }
}

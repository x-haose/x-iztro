//! 运限计算
//!
//! 根据出生星盘和目标日期，计算大限、小限、流年、流月、流日、流时。

use crate::astro::builder::{
    SUPPORTED_YEARS, birth_month_index, chart_moment, effective_solar_date, effective_time_index,
    parse_solar_input, validate_time_index,
};
use crate::astro::palace::get_palace_names;
use crate::calendar;
use crate::data::constants::TIGER_RULE;
use crate::data::stars::StarKey;
use crate::data::types::*;
use crate::error::IztroError;
use crate::i18n::{translate_horoscope_name, translate_star};
use crate::models::astrolabe::{Astrolabe, PalaceRef};
use crate::models::horoscope::{
    AgeItem, DecadalHoroscope, DecadalTarget, HoroscopeData, HoroscopeItem, HoroscopeRef,
    MonthPart, MonthlyHoroscope, YearlyDecStar, YearlyHoroscope, YearlyItem,
};
use crate::models::palace::PalaceData;
use crate::models::star::Star;
use crate::models::surpalaces::SurroundedPalaces;
use crate::star::decorative::get_yearly12;
use crate::star::location::{
    get_chang_qu_index_by_stem, get_kui_yue_index, get_lu_yang_tuo_ma_index, get_luan_xi_index,
    get_nianjie_index,
};
use crate::utils::{earthly_branch_to_palace_index, fix_index, time_to_index};

// ============================================================
// 童限宫位映射
// ============================================================

/// 童限宫位名称：在大限未覆盖的童年时期使用
/// 对应 TS: [命宫, 财帛, 疾厄, 夫妻, 福德, 官禄]
///
/// 末项官禄取不到：起运最晚的火六局在虚岁 6 即入第一个大限，童限最多到虚岁 5。
/// 表与取模照 iztro 原样保留，不按可达范围裁剪。
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

/// 十颗随层级更名的流耀，按 [魁, 钺, 昌, 曲, 禄, 羊, 陀, 马, 鸾, 喜] 排列；
/// 各层级的行与 `Scope::Origin` 行按列一一对应，即同列即「流耀 ↔ 本命辅星」。
///
/// 本命行的红鸾天喜：iztro 此处把红鸾写成了 `hongluanMin`，该标识在其 i18n 表中
/// 不存在，任何语言下都原样漏成星名；本命红鸾的标识是 `hongluan`，此处按正确标识安放。
const fn flow_star_row(scope: Scope) -> [StarKey; 10] {
    use StarKey::*;
    match scope {
        Scope::Decadal => [
            Yunkui, Yunyue, Yunchang, Yunqu, Yunlu, Yunyang, Yuntuo, Yunma, Yunluan, Yunxi,
        ],
        Scope::Yearly => [
            Liukui, Liuyue, Liuchang, Liuqu, Liulu, Liuyang, Liutuo, Liuma, Liuluan, Liuxi,
        ],
        Scope::Monthly => [
            Yuekui, Yueyue, Yuechang, Yuequ, Yuelu, Yueyang, Yuetuo, Yuema, Yueluan, Yuexi,
        ],
        Scope::Daily => [
            Rikui, Riyue, Richang, Riqu, Rilu, Riyang, Rituo, Rima, Riluan, Rixi,
        ],
        Scope::Hourly => [
            Shikui, Shiyue, Shichang, Shiqu, Shilu, Shiyang, Shituo, Shima, Shiluan, Shixi,
        ],
        Scope::Origin => [
            TiankuiMin,
            TianyueMin,
            WenchangMin,
            WenquMin,
            LucunMin,
            QingyangMin,
            TuoluoMin,
            TianmaMin,
            Hongluan,
            Tianxi,
        ],
    }
}

/// 流耀对应的本命辅星：`yunchang`/`liuchang`/`yuechang`/`richang`/`shichang` → 文昌，
/// 依此类推；非流耀返回 `None`。
///
/// 这是流耀语义的官方对照：流耀本身没有知识包条目，释义按对应本命辅星查。
pub fn natal_counterpart_of_flow_star(key: StarKey) -> Option<StarKey> {
    let natal = flow_star_row(Scope::Origin);
    for scope in HOROSCOPE_SCOPES {
        if let Some(pos) = flow_star_row(scope).iter().position(|k| *k == key) {
            return Some(natal[pos]);
        }
    }
    None
}

/// 全部流耀与其对应本命辅星的键值对（50 条），供绑定层导出对照表。
pub fn flow_star_counterparts() -> Vec<(StarKey, StarKey)> {
    let natal = flow_star_row(Scope::Origin);
    HOROSCOPE_SCOPES
        .into_iter()
        .flat_map(|scope| {
            flow_star_row(scope)
                .into_iter()
                .zip(natal)
                .collect::<Vec<_>>()
        })
        .collect()
}

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

    let star_keys = flow_star_row(scope);

    let mut stars: [Vec<Star>; 12] = std::array::from_fn(|_| Vec::new());

    // 流年范围额外安年解（按流年年支），先于十颗流耀安放
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

    let placements = [
        (kui_yue.kui, star_keys[0], StarType::Soft),
        (kui_yue.yue, star_keys[1], StarType::Soft),
        (chang_qu.chang, star_keys[2], StarType::Soft),
        (chang_qu.qu, star_keys[3], StarType::Soft),
        (lu_yang_tuo_ma.lu, star_keys[4], StarType::Lucun),
        (lu_yang_tuo_ma.yang, star_keys[5], StarType::Tough),
        (lu_yang_tuo_ma.tuo, star_keys[6], StarType::Tough),
        (lu_yang_tuo_ma.ma, star_keys[7], StarType::Tianma),
        (luan_xi.hongluan, star_keys[8], StarType::Flower),
        (luan_xi.tianxi, star_keys[9], StarType::Flower),
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

    stars
}

// ============================================================
// 主函数
// ============================================================

/// 根据出生星盘和目标阳历日期计算运限
///
/// # 参数
/// - `astrolabe`: 出生星盘
/// - `solar_date`: 目标阳历日期（`Y-M-D`，分隔符可为 `-` `/` `.`（前后一致），可带 ` HH:mm` 或 ` HH:mm:ss`，支持 1-9999 年）
/// - `time_index`: 目标时辰索引 (0=早子, 1=丑, ..., 12=晚子)；`None` 时按目标日期串的小时换算，
///   不带时间即早子时（与 iztro 省略 `timeIndex` 时一致）
/// - `language`: 语言
///
/// # Errors
/// 目标日期非法或目标时辰越界时返回 [`IztroError`]。
pub fn get_horoscope(
    astrolabe: &Astrolabe,
    solar_date: &str,
    time_index: impl Into<Option<u8>>,
    language: Language,
) -> Result<HoroscopeData, IztroError> {
    let target = parse_solar_input(solar_date)?;
    let time_index = match time_index.into() {
        Some(index) => {
            validate_time_index(index)?;
            index
        }
        None => time_to_index(target.hour.map_or(0, |h| h as u8)),
    };

    // ---- 1. 出生的农历日期：取星盘的 raw_dates（晚子时整个日期归次日时已是次日的农历日期） ----
    let birth = astrolabe.raw_dates.lunar_date;
    let birthday_lunar_year = birth.lunar_year;
    let birthday_lunar_month = birth.lunar_month as i32;
    let birthday_lunar_day = birth.lunar_day;

    // ---- 2. 解析目标日期的农历信息 ----
    // 目标时辰按 day_divide 归一：晚子归当天时，日柱与时柱算在当天而非次日
    // （农历年月日与时辰无关，23:00 仍属同一公历日）；晚子整个日期归次日时按次日早子时算
    let day_divide = astrolabe.config.day_divide;
    let (target_year, target_month, target_day) = effective_solar_date(
        day_divide,
        (target.year, target.month, target.day),
        time_index,
    );
    let target_moment = chart_moment(
        target_year,
        target_month,
        target_day,
        effective_time_index(day_divide, time_index),
    );
    let target_lunar = calendar::lunar_of(&target_moment);

    let target_ymd = target_lunar.date;
    let target_lunar_year = target_ymd.year;
    let target_is_leap = target_ymd.month < 0;
    let target_lunar_month = target_ymd.month.unsigned_abs() as i32;
    let target_lunar_day = target_ymd.day as u32;

    // ---- 3. 计算虚岁（分界点由配置决定） ----
    //     自然年分界：跨农历年即加一岁；生日分界：过了农历生日才加一岁
    let mut nominal_age = target_lunar_year - birthday_lunar_year;
    match astrolabe.config.age_divide {
        AgeDivide::Normal => nominal_age += 1,
        AgeDivide::Birthday => {
            // 只比农历月日，不比年：目标年恒晚于出生年，把年份也纳入相等判断
            // 会让「生日同月、生日之后」这一段在出生当年之后的每一年都少算一岁
            let passed_birthday = target_lunar_month > birthday_lunar_month
                || (target_lunar_month == birthday_lunar_month
                    && target_lunar_day > birthday_lunar_day);
            if passed_birthday {
                nominal_age += 1;
            }
        }
    }
    let nominal_age = nominal_age.max(1) as u32;

    // ---- 4. 目标日期干支四柱（年月分界点由配置决定） ----
    //     年按正月初一或立春换年；月按初一以五虎遁推算（闰月下半月归下月）或按节气精确时刻推算。
    //     日柱晚子时归次日；时柱天干随日柱推算
    let config = &astrolabe.config;
    let (target_year_stem, target_year_branch) = if config.horoscope_divide.year_by_li_chun() {
        calendar::year_by_li_chun(&target_moment, target_lunar_year)
    } else {
        target_lunar.year
    };

    // 闰月十六日起归下月：月干（初一分界）、流月命宫同此口径
    let date_leap_addition: i32 = if target_is_leap && target_lunar_day > 15 {
        1
    } else {
        0
    };
    let (target_month_stem, target_month_branch) = if config.horoscope_divide.month_by_jie() {
        calendar::month_exact(&target_moment, target_lunar_year)
    } else {
        (
            HeavenlyStem::from_index(fix_index(
                TIGER_RULE[target_year_stem.index()].index() as i32 + target_lunar_month - 1
                    + date_leap_addition,
                10,
            )),
            EarthlyBranch::from_index(fix_index(
                2 + target_lunar_month - 1 + date_leap_addition,
                12,
            )),
        )
    };
    let (target_day_stem, target_day_branch) = target_lunar.day_exact;
    let (target_time_stem, target_time_branch) = target_lunar.time;

    // ---- 5. 大限 ----
    //     虚岁落在某宫大限区间内取该宫，干支取该宫的大限干支（`decadal_stem` 决定天干）；
    //     尚未起运时为童限，落宫由 `childhood_palace` 决定，干支取该宫的本命宫干支
    let (decadal_palace_idx, is_childhood) = find_decadal_palace(astrolabe, nominal_age);
    let (decadal_stem, decadal_branch) = if is_childhood {
        let p = &astrolabe.palaces[decadal_palace_idx];
        (p.heavenly_stem, p.earthly_branch)
    } else {
        let d = &astrolabe.palaces[decadal_palace_idx].decadal;
        (d.heavenly_stem, d.earthly_branch)
    };
    let decadal_stars = get_horoscope_stars(decadal_stem, decadal_branch, Scope::Decadal, language);
    let decadal_name = if is_childhood {
        HoroscopeName::Childhood
    } else {
        HoroscopeName::Decadal
    };
    let decadal = scope_item(
        astrolabe,
        decadal_palace_idx,
        decadal_name,
        (decadal_stem, decadal_branch),
        Some(scope_mutagen_stem(
            astrolabe,
            decadal_stem,
            decadal_palace_idx,
        )),
        Some(decadal_stars.to_vec()),
        language,
    );

    // ---- 6. 小限 ----
    //     宫干四化取小限宫的宫干，`age_mutagens=None` 时不起四化
    let age_palace_idx = find_age_palace(astrolabe, nominal_age);
    let age_palace = &astrolabe.palaces[age_palace_idx];
    let age_mutagen_stem = match config.age_mutagens {
        AgeMutagens::PalaceStem => Some(age_palace.heavenly_stem),
        AgeMutagens::None => None,
    };
    let age = AgeItem {
        base: scope_item(
            astrolabe,
            age_palace_idx,
            HoroscopeName::Age,
            (age_palace.heavenly_stem, age_palace.earthly_branch),
            age_mutagen_stem,
            None,
            language,
        ),
        nominal_age,
    };

    // ---- 7. 流年（含按目标年支起的岁前/将前十二神） ----
    //     太岁宫是流年年支所在宫；流年命宫取太岁宫或小限宫（`yearly_palace`），流年干支、
    //     流耀与岁前/将前十二神都按流年干支安，不随命宫换
    let taisui_index = earthly_branch_to_palace_index(target_year_branch);
    let yearly_index = match config.yearly_palace {
        YearlyPalace::Taisui => taisui_index,
        YearlyPalace::Age => age_palace_idx,
    };
    let yearly_stars = get_horoscope_stars(
        target_year_stem,
        target_year_branch,
        Scope::Yearly,
        language,
    );
    let (yearly_suiqian12, yearly_jiangqian12) = get_yearly12(target_year_branch, config.algorithm);
    let yearly = YearlyItem {
        base: scope_item(
            astrolabe,
            yearly_index,
            HoroscopeName::Yearly,
            (target_year_stem, target_year_branch),
            Some(scope_mutagen_stem(
                astrolabe,
                target_year_stem,
                yearly_index,
            )),
            Some(yearly_stars.to_vec()),
            language,
        ),
        yearly_dec_star: YearlyDecStar {
            jiangqian12: yearly_jiangqian12.to_vec(),
            suiqian12: yearly_suiqian12.to_vec(),
        },
    };

    // ---- 8. 流月 ----
    //     斗君：太岁宫逆数生月、顺数生时得正月，每月顺行一宫；生月按星盘的闰月口径
    //     （`fix_leap`、`leap_divide`）取，`dou_jun_month=Natal` 时连本命盘晚子时不进闰月下半月的例外也一并照用。
    //     寅起：正月固定在寅宫，每月顺行一宫。两者都是闰月前半月随本月、后半月随下月
    let monthly_index = match config.monthly_palace {
        MonthlyPalace::DouJun => {
            let keep_leap_month = match config.dou_jun_month {
                DouJunMonth::Default => false,
                DouJunMonth::Natal => effective_time_index(day_divide, astrolabe.time_index) == 12,
            };
            let birth_month_index = birth_month_index(
                birth.lunar_month,
                birthday_lunar_day,
                birth.is_leap,
                astrolabe.fix_leap,
                config.leap_divide,
                keep_leap_month,
            ) as i32;
            // 出生时辰的地支原始索引 (0=子, 1=丑, ...)
            let birth_time_branch_raw_index = fix_index(astrolabe.time_index as i32, 12) as i32;
            fix_index(
                taisui_index as i32 - (birth_month_index + 1)
                    + birth_time_branch_raw_index
                    + (target_lunar_month + date_leap_addition),
                12,
            )
        }
        MonthlyPalace::MonthBranch => fix_index(target_lunar_month - 1 + date_leap_addition, 12),
    };
    let monthly_stars = get_horoscope_stars(
        target_month_stem,
        target_month_branch,
        Scope::Monthly,
        language,
    );
    let monthly = scope_item(
        astrolabe,
        monthly_index,
        HoroscopeName::Monthly,
        (target_month_stem, target_month_branch),
        Some(scope_mutagen_stem(
            astrolabe,
            target_month_stem,
            monthly_index,
        )),
        Some(monthly_stars.to_vec()),
        language,
    );

    // ---- 9. 流日 ----
    //     初一落在流月命宫（`daily_palace=AfterMonthly` 时落在其下一宫），每日顺行一宫；
    //     `leap_daily=Continuous` 时闰月前半月接着同号本月的日数连续顺数
    let mut daily_offset = target_lunar_day as i32 - 1;
    if config.daily_palace == DailyPalace::AfterMonthly {
        daily_offset += 1;
    }
    if config.leap_daily == LeapDaily::Continuous && target_is_leap && date_leap_addition == 0 {
        let regular_days = calendar::month_day_count(target_lunar_year, target_lunar_month as i64)
            .expect("有闰月的农历年必有同号的本月");
        daily_offset += regular_days as i32;
    }
    let daily_index = fix_index(monthly_index as i32 + daily_offset, 12);
    let daily_stars =
        get_horoscope_stars(target_day_stem, target_day_branch, Scope::Daily, language);
    let daily = scope_item(
        astrolabe,
        daily_index,
        HoroscopeName::Daily,
        (target_day_stem, target_day_branch),
        Some(scope_mutagen_stem(astrolabe, target_day_stem, daily_index)),
        Some(daily_stars.to_vec()),
        language,
    );

    // ---- 10. 流时 ----
    //     从流日命宫起子时顺数，或按时支直接落宫（子时在子宫，`hourly_palace=Branch`）
    let target_hour_branch_index = fix_index(time_index as i32, 12);
    let hourly_index = match config.hourly_palace {
        HourlyPalace::FromDaily => {
            fix_index(daily_index as i32 + target_hour_branch_index as i32, 12)
        }
        HourlyPalace::Branch => {
            earthly_branch_to_palace_index(EarthlyBranch::from_index(target_hour_branch_index))
        }
    };
    let hourly_stars = get_horoscope_stars(
        target_time_stem,
        target_time_branch,
        Scope::Hourly,
        language,
    );
    let hourly = scope_item(
        astrolabe,
        hourly_index,
        HoroscopeName::Hourly,
        (target_time_stem, target_time_branch),
        Some(scope_mutagen_stem(
            astrolabe,
            target_time_stem,
            hourly_index,
        )),
        Some(hourly_stars.to_vec()),
        language,
    );

    // ---- 11. 农历日期字符串（恒为中文写法，闰月带「闰」前缀） ----
    let lunar_date_str = calendar::chinese_lunar_date(
        target_lunar_year,
        target_lunar_month as u32,
        target_lunar_day,
        target_is_leap,
    );

    Ok(HoroscopeData {
        solar_date: format!("{}-{}-{}", target.year, target.month, target.day),
        lunar_date: lunar_date_str,
        decadal,
        age,
        yearly,
        monthly,
        daily,
        hourly,
    })
}

// ============================================================
// 辅助函数
// ============================================================

/// 查找大限宫位索引，返回 (宫位索引, 是否童限)。
///
/// 虚岁落在某宫大限区间内取该宫；尚未起运时为童限：`childhood_palace=Sequence` 按虚岁依次映射到
/// [命宫, 财帛, 疾厄, 夫妻, 福德, 官禄]，`Spouse` 一律取夫妻宫。
fn find_decadal_palace(astrolabe: &Astrolabe, nominal_age: u32) -> (usize, bool) {
    for p in &astrolabe.palaces {
        let (start, end) = p.decadal.range;
        if nominal_age >= start && nominal_age <= end {
            return (p.index, false);
        }
    }

    let target_palace = match astrolabe.config.childhood_palace {
        ChildhoodPalace::Sequence => {
            CHILDHOOD_PALACES[((nominal_age as usize).saturating_sub(1)) % CHILDHOOD_PALACES.len()]
        }
        ChildhoodPalace::Spouse => Palace::Spouse,
    };

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

/// 运限某层四化所用的天干：`scope_mutagen_stem=Scope` 取该层自身的干，
/// `ScopeSoulPalace` 取该层命宫（盘上第 `soul_index` 宫）的本命宫干。
fn scope_mutagen_stem(
    astrolabe: &Astrolabe,
    layer_stem: HeavenlyStem,
    soul_index: usize,
) -> HeavenlyStem {
    match astrolabe.config.scope_mutagen_stem {
        ScopeMutagenStem::Scope => layer_stem,
        ScopeMutagenStem::ScopeSoulPalace => astrolabe.palaces[soul_index].heavenly_stem,
    }
}

/// 组装一个运限层：宫名以 `index` 为命宫推排，四化取 `mutagen_stem` 化出的星（`None` 即不起四化）。
fn scope_item(
    astrolabe: &Astrolabe,
    index: usize,
    name_key: HoroscopeName,
    (heavenly_stem, earthly_branch): (HeavenlyStem, EarthlyBranch),
    mutagen_stem: Option<HeavenlyStem>,
    stars: Option<Vec<Vec<Star>>>,
    language: Language,
) -> HoroscopeItem {
    let (mutagen, extra_mutagen) = match mutagen_stem {
        Some(stem) => (
            astrolabe.config.mutagens_of(stem).to_vec(),
            astrolabe.config.extra_mutagens_of(stem).clone(),
        ),
        None => (Vec::new(), Default::default()),
    };
    HoroscopeItem {
        index,
        name: translate_horoscope_name(name_key, language).to_string(),
        name_key,
        heavenly_stem,
        earthly_branch,
        palace_names: get_palace_names(index).to_vec(),
        mutagen_stem,
        mutagen,
        extra_mutagen,
        stars,
    }
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
    pub fn age_palace<'a>(&self, astrolabe: &'a Astrolabe) -> PalaceRef<'a> {
        astrolabe.palace_at(self.age.index)
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
    ) -> Option<PalaceRef<'a>> {
        if scope == Scope::Origin {
            astrolabe.palace(name)
        } else {
            let scope_item = self.scope_item(scope)?;
            let idx = scope_item.palace_index_by_name(name)?;
            Some(astrolabe.palace_at(idx))
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
        astrolabe.surrounded_palaces(palace.index)
    }

    /// 检查指定运限的四化星是否在指定宫位中
    ///
    /// 检查该运限化出该类四化的星（四槽之星与附加星，化出它们的干见 [`HoroscopeItem::mutagen_stem`]）是否有一颗出现在目标宫位的主星或辅星中。
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
        scope_item
            .mutagens()
            .filter(|(_, m)| *m == mutagen)
            .any(|(star_key, _)| {
                palace
                    .major_stars
                    .iter()
                    .chain(palace.minor_stars.iter())
                    .any(|s| s.key == star_key)
            })
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

    /// 检查大限和流年的流耀是否包含指定星耀中的任意一颗
    pub fn has_one_of_horoscope_stars(
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
        stars.iter().any(|s| all_keys.contains(s))
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
        if let Some(stars) = self
            .yearly
            .base
            .stars
            .as_ref()
            .and_then(|s| s.get(palace_idx))
        {
            keys.extend(stars.iter().map(|s| s.key));
        }
        keys
    }

    /// 获取指定运限范围的 HoroscopeItem
    pub(crate) fn scope_item(&self, scope: Scope) -> Option<&HoroscopeItem> {
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

impl Astrolabe {
    /// 以本命盘为起点计算目标日期的运限，结果持有本盘，输出语言随本盘；查询精度
    /// （[`HoroscopeRef::precision`]）为流时，给不给时辰都一样。
    ///
    /// - `target_date`: 目标阳历日期，可带 ` HH:mm(:ss)`（文法见 [`get_horoscope`]）
    /// - `target_time_index`: 目标时辰索引 0-12，可传 `None`：按目标日期串的小时换算，不带时间即早子时
    ///
    /// # Errors
    /// 目标日期非法或目标时辰越界时返回 [`IztroError`]。
    pub fn horoscope(
        &self,
        target_date: &str,
        target_time_index: impl Into<Option<u8>>,
    ) -> Result<HoroscopeRef<'_>, IztroError> {
        Ok(HoroscopeRef {
            data: get_horoscope(self, target_date, target_time_index, self.language)?,
            astrolabe: self,
            precision: Scope::Hourly,
        })
    }
}

impl Astrolabe {
    /// 逐层运限查询用的时辰索引：由时柱地支反查，而不是出生入参 `time_index`。
    ///
    /// 晚子时（入参 12）的时柱地支是子，反查得 0——列表里的每一层都按这个时辰起运限，
    /// 与 iztro 的 `EARTHLY_BRANCHES.indexOf(kot(chineseDate.hourly[1]))` 同源。
    /// 直接用入参会让晚子时盘的列表与逐层查询对不上。
    fn list_time_index(&self) -> u8 {
        self.raw_dates.chinese_date.hourly.1.index() as u8
    }

    /// 按起运先后排列的大限列表，第 0 项为第一个大限。
    ///
    /// 每项带该限所在的本命宫名、起止虚岁与起止农历年份，以及以该宫为命宫推排的
    /// 十二宫名、该限四化与大限流曜。
    pub fn decadal_list(&self) -> Vec<DecadalHoroscope> {
        let birth_year = self.raw_dates.lunar_date.lunar_year;
        let mut palaces: Vec<&PalaceData> = self.palaces.iter().collect();
        palaces.sort_by_key(|p| p.decadal.range.0);
        palaces
            .into_iter()
            .map(|p| {
                let (stem, branch) = (p.decadal.heavenly_stem, p.decadal.earthly_branch);
                let (start, end) = p.decadal.range;
                DecadalHoroscope {
                    base: scope_item(
                        self,
                        p.index,
                        HoroscopeName::Decadal,
                        (stem, branch),
                        Some(scope_mutagen_stem(self, stem, p.index)),
                        Some(
                            get_horoscope_stars(stem, branch, Scope::Decadal, self.language)
                                .to_vec(),
                        ),
                        self.language,
                    ),
                    palace_name: p.name,
                    age_range: (start, end),
                    year_range: (birth_year + start as i64 - 1, birth_year + end as i64 - 1),
                }
            })
            .collect()
    }

    /// 定位一个大限的起止虚岁。
    fn decadal_range(&self, target: DecadalTarget) -> Result<(u32, u32), IztroError> {
        let palace = match target {
            DecadalTarget::Ordinal(n) => {
                let mut sorted: Vec<&PalaceData> = self.palaces.iter().collect();
                sorted.sort_by_key(|p| p.decadal.range.0);
                sorted.get(n).copied().ok_or_else(|| {
                    IztroError::InvalidArgument(format!(
                        "decadal ordinal {n} is out of range (0-11)"
                    ))
                })?
            }
            DecadalTarget::Name(name) => {
                self.palaces
                    .iter()
                    .find(|p| p.name == name)
                    .ok_or_else(|| {
                        IztroError::InvalidArgument(format!("palace '{}' not found", name.as_key()))
                    })?
            }
        };
        Ok(palace.decadal.range)
    }

    /// 指定大限内的全部流年，按虚岁先后排列。
    ///
    /// `target` 可写大限序号（0 为第一个大限）或本命宫名。每个流年经与逐层查询相同的
    /// [`Astrolabe::horoscope`] 算出，取该农历年六月初一为目标日期。
    ///
    /// # Errors
    /// 大限序号越界、宫名定位不到，或某一年的目标日期落在支持范围外时返回 [`IztroError`]。
    pub fn yearly_list(
        &self,
        target: impl Into<DecadalTarget>,
    ) -> Result<Vec<YearlyHoroscope>, IztroError> {
        let (start, end) = self.decadal_range(target.into())?;
        let birth_year = self.raw_dates.lunar_date.lunar_year;
        (start..=end)
            .map(|age| {
                let year = birth_year + age as i64 - 1;
                let h = self.horoscope_at_mid_year(year)?;
                Ok(YearlyHoroscope {
                    base: h.yearly.base,
                    age,
                    year,
                })
            })
            .collect()
    }

    /// 指定农历年六月初一的运限，时辰同逐层列表（[`Self::list_time_index`]）：按年取运限与流年列表共用的代表日。
    fn horoscope_at_mid_year(&self, year: i64) -> Result<HoroscopeData, IztroError> {
        let (sy, sm, sd) = calendar::solar_date_of(year, 6, 1)
            .ok_or_else(|| IztroError::Internal(format!("lunar year {year} has no sixth month")))?;
        get_horoscope(
            self,
            &format!("{sy}-{sm}-{sd}"),
            self.list_time_index(),
            self.language,
        )
    }

    /// 指定农历年的运限：目标日期取该年六月初一，与 [`Self::yearly_list`] 同一约定。
    ///
    /// 只问某一年时用它，不必自己挑公历日期——公历年初到正月初一之前仍属上一个农历年，
    /// 拿公历 1 月 1 日去算会得到上一年的流年。大限、小限与流年在默认分界配置下整个农历年不变；
    /// `age_divide` 取生日分界或 `horoscope_divide` 取立春分界时年内仍会换层，结果是六月初一当天的。
    /// 查询精度（[`HoroscopeRef::precision`]）为流年：流月、流日、流时也在数据里，是六月初一那一天的，
    /// 文本默认只写到流年，宫位查询、格局与文本的层级参数比流年细时报错。
    ///
    /// # Errors
    /// 年份不在 1-9999 时返回 [`IztroError::InvalidArgument`]；该年六月初一落在支持范围外时返回
    /// [`IztroError`]。
    pub fn horoscope_of_year(&self, year: i64) -> Result<HoroscopeRef<'_>, IztroError> {
        if !SUPPORTED_YEARS.contains(&year) {
            return Err(IztroError::InvalidArgument(format!(
                "invalid year {year}: expected {}-{}",
                SUPPORTED_YEARS.start(),
                SUPPORTED_YEARS.end()
            )));
        }
        Ok(HoroscopeRef {
            data: self.horoscope_at_mid_year(year)?,
            astrolabe: self,
            precision: Scope::Yearly,
        })
    }

    /// 指定农历年某月的运限，时辰按时柱地支取（同逐层列表）。只问某个月时用它，不必替提问的人挑一天。
    ///
    /// `month` 是月序 1-12，目标日期随 `horoscope_divide`：
    /// - 年按正月初一分界（`normal` 与 `exactMonth`）：该农历月（非闰）初一，与 [`Self::monthly_list`]
    ///   整月一段的约定相同。闰月前半月的流月随本月、后半月随下月，所以流月只按月号取，闰月的起止见
    ///   [`Self::monthly_list`]。`exactMonth` 的流月干是初一那天所在节令月的干，交节后换干，流月命宫不变。
    /// - 年按立春分界（`exact`）：第 `month` 个节令月（寅月为 1）交节次日，流年与 [`Self::horoscope_of_year`]
    ///   同属立春起算的 `year` 年。农历月初一常落在上一个节令月里，所以不取初一。
    ///
    /// 查询精度（[`HoroscopeRef::precision`]）为流月：流日与流时也在数据里，是那一天的，
    /// 文本默认只写到流月，宫位查询、格局与文本的层级参数比流月细时报错。
    ///
    /// # Errors
    /// 年份不在 1-9999、月份不在 1-12，或初一分界下该农历年没有这个月（公历 8 年、237 年这类改历年）时返回
    /// [`IztroError::InvalidArgument`]；目标日期落在支持范围外（9999 年节气分界的丑月）时返回 [`IztroError`]。
    pub fn horoscope_of_month(
        &self,
        year: i64,
        month: u32,
    ) -> Result<HoroscopeRef<'_>, IztroError> {
        if !SUPPORTED_YEARS.contains(&year) {
            return Err(IztroError::InvalidArgument(format!(
                "invalid year {year}: expected {}-{}",
                SUPPORTED_YEARS.start(),
                SUPPORTED_YEARS.end()
            )));
        }
        if !(1..=12).contains(&month) {
            return Err(IztroError::InvalidArgument(format!(
                "invalid lunar month {month}: expected 1-12"
            )));
        }
        let (sy, sm, sd) = if self.config.horoscope_divide.year_by_li_chun() {
            calendar::day_after_jie(year, month)
        } else {
            calendar::solar_date_of(year, month as i64, 1).ok_or_else(|| {
                IztroError::InvalidArgument(format!("lunar year {year} has no month {month}"))
            })?
        };
        Ok(HoroscopeRef {
            data: get_horoscope(
                self,
                &format!("{sy}-{sm}-{sd}"),
                self.list_time_index(),
                self.language,
            )?,
            astrolabe: self,
            precision: Scope::Monthly,
        })
    }

    /// 指定农历年的全部流月，按月份先后排列。
    ///
    /// 无闰月返回 12 项；有闰月且 `fix_leap` 为真时，闰月按前后半月拆成两项（共 14 项），
    /// 为假时闰月整月一项（共 13 项）。每个流月经 [`Astrolabe::horoscope`] 算出，
    /// 目标日期取该段的首日（后半段取十六）。
    ///
    /// # Errors
    /// 年份不在 1-9999 时返回 [`IztroError::InvalidArgument`]（与 iztro 的前置校验一致）；
    /// 某一段的目标日期落在支持范围外（9999 年末几个月对应公历 10000 年）时返回 [`IztroError`]。
    pub fn monthly_list(
        &self,
        year: i64,
        fix_leap: bool,
    ) -> Result<Vec<MonthlyHoroscope>, IztroError> {
        if !SUPPORTED_YEARS.contains(&year) {
            return Err(IztroError::InvalidArgument(format!(
                "invalid year {year}: expected {}-{}",
                SUPPORTED_YEARS.start(),
                SUPPORTED_YEARS.end()
            )));
        }
        let age = (year - self.raw_dates.lunar_date.lunar_year + 1).max(1) as u32;
        let time_index = self.list_time_index();
        let leap = calendar::leap_month(year);

        let mut out = Vec::new();
        for month in 1..=12u32 {
            // 闰月排在同月号的常规月之后
            let mut signed = vec![month as i64];
            if leap == Some(month as i64) {
                signed.push(-(month as i64));
            }
            for sm in signed {
                let is_leap = sm < 0;
                let Some(day_count) = calendar::month_day_count(year, sm) else {
                    continue;
                };
                let last = day_count as u32;
                let segments: Vec<(MonthPart, (u32, u32))> = if is_leap && fix_leap {
                    vec![(MonthPart::First, (1, 15)), (MonthPart::Second, (16, last))]
                } else {
                    vec![(MonthPart::Normal, (1, last))]
                };
                for (part, day_range) in segments {
                    let (sy, smo, sd) = calendar::solar_date_of(year, sm, day_range.0 as i64)
                        .expect("段首日在该农历月的天数之内");
                    let h = get_horoscope(
                        self,
                        &format!("{sy}-{smo}-{sd}"),
                        time_index,
                        self.language,
                    )?;
                    out.push(MonthlyHoroscope {
                        base: h.monthly,
                        age,
                        year,
                        month,
                        is_leap_month: is_leap,
                        part,
                        day_range,
                    });
                }
            }
        }
        Ok(out)
    }

    /// 以本命盘为起点计算**此刻**的运限，日期与时辰取本地时钟。
    ///
    /// # Errors
    /// 本地日期落在支持范围（公历 1-9999）之外时返回 [`IztroError`]。
    pub fn horoscope_now(&self) -> Result<HoroscopeRef<'_>, IztroError> {
        let now = chrono::Local::now().format("%Y-%-m-%-d %H:%M").to_string();
        self.horoscope(&now, None)
    }
}

impl<'a> HoroscopeRef<'a> {
    /// 小限宫位
    pub fn age_palace(&self) -> PalaceRef<'a> {
        self.data.age_palace(self.astrolabe)
    }

    /// 按宫位名称和运限范围取宫位；`Scope::Origin` 按本命宫名取
    ///
    /// # Errors
    /// `scope` 比查询精度（[`HoroscopeRef::precision`]）细时返回 [`IztroError::InvalidArgument`]：
    /// 那一层是内核挑的代表日的，不是查询的答案。
    pub fn palace(&self, name: Palace, scope: Scope) -> Result<Option<PalaceRef<'a>>, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self.data.palace(name, scope, self.astrolabe))
    }

    /// 按宫位名称和运限范围取三方四正
    ///
    /// # Errors
    /// 同 [`Self::palace`]。
    pub fn surround_palaces(
        &self,
        name: Palace,
        scope: Scope,
    ) -> Result<Option<SurroundedPalaces<'a>>, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self.data.surround_palaces(name, scope, self.astrolabe))
    }

    /// 该运限的指定四化星（化出它们的干见 [`HoroscopeItem::mutagen_stem`]）是否落在目标宫位（本命盘范围恒为 false）
    ///
    /// # Errors
    /// 同 [`Self::palace`]。
    pub fn has_horoscope_mutagen(
        &self,
        name: Palace,
        scope: Scope,
        mutagen: Mutagen,
    ) -> Result<bool, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self
            .data
            .has_horoscope_mutagen(name, scope, mutagen, self.astrolabe))
    }

    /// 目标宫位的大限与流年流耀是否**全部**包含指定星耀
    ///
    /// # Errors
    /// 同 [`Self::palace`]。
    pub fn has_horoscope_stars(
        &self,
        name: Palace,
        scope: Scope,
        stars: &[StarKey],
    ) -> Result<bool, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self
            .data
            .has_horoscope_stars(name, scope, stars, self.astrolabe))
    }

    /// 目标宫位的大限与流年流耀是否包含指定星耀中的**任一颗**
    ///
    /// # Errors
    /// 同 [`Self::palace`]。
    pub fn has_one_of_horoscope_stars(
        &self,
        name: Palace,
        scope: Scope,
        stars: &[StarKey],
    ) -> Result<bool, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self
            .data
            .has_one_of_horoscope_stars(name, scope, stars, self.astrolabe))
    }

    /// 目标宫位的大限与流年流耀是否**一颗都不**包含指定星耀
    ///
    /// # Errors
    /// 同 [`Self::palace`]。
    pub fn not_have_horoscope_stars(
        &self,
        name: Palace,
        scope: Scope,
        stars: &[StarKey],
    ) -> Result<bool, IztroError> {
        self.check_within_precision(scope, "scope")?;
        Ok(self
            .data
            .not_have_horoscope_stars(name, scope, stars, self.astrolabe))
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
            Config::default(),
        )
        .unwrap()
    }

    #[test]
    fn test_horoscope_holds_astrolabe() {
        let astrolabe = make_astrolabe();
        let h = astrolabe.horoscope("2023-10-15", 4).unwrap();

        // 回引拿到的是同一张盘，输出语言随本盘
        assert_eq!(h.astrolabe().solar_date, astrolabe.solar_date);
        // 解引用后即运限数据本身
        assert_eq!(h.solar_date, "2023-10-15");

        // 免传星盘的查询与显式传盘的结果一致
        let by_data = h
            .data()
            .palace(Palace::Soul, Scope::Decadal, &astrolabe)
            .unwrap();
        assert_eq!(
            h.palace(Palace::Soul, Scope::Decadal)
                .unwrap()
                .unwrap()
                .index,
            by_data.index
        );
        assert_eq!(h.age_palace().index, h.data().age_palace(&astrolabe).index);
        assert_eq!(
            h.surround_palaces(Palace::Soul, Scope::Yearly)
                .unwrap()
                .unwrap()
                .target
                .index,
            by_data.astrolabe().palace_at(h.yearly.index).index
        );
        for m in [Mutagen::Lu, Mutagen::Quan, Mutagen::Ke, Mutagen::Ji] {
            assert_eq!(
                h.has_horoscope_mutagen(Palace::Soul, Scope::Decadal, m)
                    .unwrap(),
                h.data()
                    .has_horoscope_mutagen(Palace::Soul, Scope::Decadal, m, &astrolabe)
            );
        }
        let stars = [StarKey::Yunkui, StarKey::Yunyue];
        assert_eq!(
            h.has_horoscope_stars(Palace::Soul, Scope::Decadal, &stars)
                .unwrap(),
            h.data()
                .has_horoscope_stars(Palace::Soul, Scope::Decadal, &stars, &astrolabe)
        );
        assert_eq!(
            h.has_one_of_horoscope_stars(Palace::Soul, Scope::Decadal, &stars)
                .unwrap(),
            !h.not_have_horoscope_stars(Palace::Soul, Scope::Decadal, &stars)
                .unwrap()
        );
    }

    /// 按年、按月取的运限，宫位查询的层级比查询精度细时报错，而不是答出代表日那一层的宫位；
    /// 不比查询精度细的层级与本命照常可查
    #[test]
    fn palace_queries_reject_scopes_finer_than_the_precision() {
        let astrolabe = make_astrolabe();
        let stars = [StarKey::Yunkui];
        for (h, finer, within) in [
            (
                astrolabe.horoscope_of_year(2027).unwrap(),
                Scope::Monthly,
                Scope::Yearly,
            ),
            (
                astrolabe.horoscope_of_month(2025, 7).unwrap(),
                Scope::Daily,
                Scope::Monthly,
            ),
        ] {
            let errors = [
                h.palace(Palace::Soul, finer).err(),
                h.surround_palaces(Palace::Soul, finer).err(),
                h.has_horoscope_mutagen(Palace::Soul, finer, Mutagen::Lu)
                    .err(),
                h.has_horoscope_stars(Palace::Soul, finer, &stars).err(),
                h.has_one_of_horoscope_stars(Palace::Soul, finer, &stars)
                    .err(),
                h.not_have_horoscope_stars(Palace::Soul, finer, &stars)
                    .err(),
            ];
            for err in errors {
                assert!(
                    matches!(err, Some(IztroError::InvalidArgument(_))),
                    "{finer:?}: {err:?}"
                );
            }
            for scope in [Scope::Origin, Scope::Decadal, within] {
                assert!(
                    h.palace(Palace::Soul, scope).unwrap().is_some(),
                    "{scope:?}"
                );
            }
        }
    }

    #[test]
    fn test_get_horoscope_basic() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();

        // Basic structure checks
        assert_eq!(horoscope.solar_date, "2023-10-15");
        assert!(!horoscope.lunar_date.is_empty());

        // Decadal should have palace names and stars
        assert_eq!(horoscope.decadal.palace_names.len(), 12);
        assert!(horoscope.decadal.stars.is_some());
        let dec_stars = horoscope.decadal.stars.as_ref().unwrap();
        assert_eq!(dec_stars.len(), 12);

        // Yearly should have stars including nianjie
        assert!(horoscope.yearly.stars.is_some());
        let yr_stars = horoscope.yearly.stars.as_ref().unwrap();
        assert_eq!(yr_stars.len(), 12);

        // Monthly, daily, hourly should have stars
        assert!(horoscope.monthly.stars.is_some());
        assert!(horoscope.daily.stars.is_some());
        assert!(horoscope.hourly.stars.is_some());

        // Nominal age should be reasonable
        assert!(horoscope.age.nominal_age > 0);

        // Mutagen should have 4 entries
        assert_eq!(horoscope.decadal.mutagen.len(), 4);
        assert_eq!(horoscope.yearly.mutagen.len(), 4);
        assert_eq!(horoscope.monthly.mutagen.len(), 4);
        assert_eq!(horoscope.daily.mutagen.len(), 4);
        assert_eq!(horoscope.hourly.mutagen.len(), 4);
    }

    #[test]
    fn test_get_horoscope_nominal_age() {
        let astrolabe = make_astrolabe();
        // 2000年出生，2023年查询
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();
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
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();

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
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();
        let age_palace = horoscope.age_palace(&astrolabe);
        assert!(age_palace.index < 12);
    }

    #[test]
    fn test_horoscope_surround_palaces() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();
        let sp = horoscope.surround_palaces(Palace::Soul, Scope::Origin, &astrolabe);
        assert!(sp.is_some());
        let sp = sp.unwrap();
        assert_eq!(sp.target.name, Palace::Soul);
    }

    #[test]
    fn test_childhood_decadal() {
        // Test with a very young age (birth year = target year)
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2000-12-1", 4, Language::ZhCN).unwrap();
        // Should be age 1, which may be in childhood range
        assert!(horoscope.age.nominal_age <= 2);
        // Decadal index should be valid
        assert!(horoscope.decadal.index < 12);
    }

    #[test]
    fn test_has_horoscope_mutagen() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();

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
            if horoscope.has_horoscope_mutagen(
                *palace_name,
                Scope::Decadal,
                Mutagen::Lu,
                &astrolabe,
            ) {
                found = true;
                break;
            }
        }
        // The mutagen star should be found in at least one palace
        assert!(
            found,
            "Lu mutagen star {:?} should be in some palace",
            mutagen_star
        );
    }

    #[test]
    fn test_has_horoscope_stars_and_not() {
        let astrolabe = make_astrolabe();
        let horoscope = get_horoscope(&astrolabe, "2023-10-15", 4, Language::ZhCN).unwrap();

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
            assert!(!horoscope.not_have_horoscope_stars(
                palace_name,
                Scope::Origin,
                &[key],
                &astrolabe
            ));
        }
    }

    /// 斗君所用的生月按星盘的 `fix_leap` 取：闰四月二十出生，`fix_leap` 打开时生月作五月，
    /// 关闭时整月按四月，流月宫因此差一宫（逆数生月少一格即顺移一宫）；闰月前半月两者相同。
    #[test]
    fn dou_jun_birth_month_follows_fix_leap() {
        let flow_month = |lunar_date: &str, leap: LeapMonth| {
            let chart = crate::by_lunar(
                lunar_date,
                5,
                Gender::Male,
                leap,
                Language::ZhCN,
                Config::default(),
            )
            .unwrap();
            chart.horoscope("2030-6-1", 0).unwrap().monthly.index
        };
        let fixed = flow_month("2020-4-20", LeapMonth::LeapFixed);
        let whole = flow_month("2020-4-20", LeapMonth::Leap);
        assert_eq!(whole, fix_index(fixed as i32 + 1, 12));
        assert_eq!(
            flow_month("2020-4-10", LeapMonth::LeapFixed),
            flow_month("2020-4-10", LeapMonth::Leap)
        );
    }
}

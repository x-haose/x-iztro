/**
 * Regression 金标生成器：单张固定命盘的全接口面快照。
 *
 * 覆盖 tests/regression.rs 消费的四块：
 * - util：timeToIndex / fixIndex / getAgeIndex / 天干四化表
 * - astrolabe：顶层字段与十二宫的名称、干支、星数、大限
 * - palace / surround：宫位与三方四正的查询方法在十二宫上的取值
 * - horoscope：固定目标日期的六个运限层级与运限查询方法
 *
 * 固定命盘：2000-8-16 时辰 2 女命 fixLeap=true zh-CN；运限目标 2025-1-1 时辰 0。
 *
 * 输出：regression_data.json
 */
import { astro } from 'iztro';
import { fixIndex, getAgeIndex, timeToIndex } from 'iztro/lib/utils/index.js';
import { heavenlyStems } from 'iztro/lib/data/index.js';
import { t } from 'iztro/lib/i18n/index.js';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const BIRTH = { date: '2000-8-16', timeIndex: 2, gender: '女' };
const TARGET = { date: '2025-1-1', timeIndex: 0 };

const HEAVENLY_STEMS = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
const EARTHLY_BRANCHES = ['子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥'];
/** 查询方法的实参：均为真实存在的星耀/四化，断言两侧调用同一函数比对 */
const PROBE_STAR = '紫微';
const PROBE_STAR_2 = '武曲';

const astrolabe = astro.bySolar(BIRTH.date, BIRTH.timeIndex, BIRTH.gender, true, 'zh-CN');
const horoscope = astrolabe.horoscope(TARGET.date, TARGET.timeIndex);

// ---------- util ----------

const util = {
  time_to_index: Object.fromEntries(
    Array.from({ length: 24 }, (_, h) => [String(h), timeToIndex(h)]),
  ),
  fix_index: Object.fromEntries(
    [
      [-1, 12],
      [0, 12],
      [11, 12],
      [12, 12],
      [13, 12],
      [-13, 12],
      [25, 12],
      [-1, 10],
      [10, 10],
      [21, 10],
      [-11, 10],
    ].map(([index, max]) => [`${index}_${max}`, fixIndex(index, max)]),
  ),
  get_age_index: Object.fromEntries(EARTHLY_BRANCHES.map((b) => [b, getAgeIndex(b)])),
  get_mutagens_by_heavenly_stem: Object.fromEntries(
    HEAVENLY_STEMS.map((s) => [s, heavenlyStems[
      { 甲: 'jiaHeavenly', 乙: 'yiHeavenly', 丙: 'bingHeavenly', 丁: 'dingHeavenly',
        戊: 'wuHeavenly', 己: 'jiHeavenly', 庚: 'gengHeavenly', 辛: 'xinHeavenly',
        壬: 'renHeavenly', 癸: 'guiHeavenly' }[s]
    ].mutagen.map((m) => t(m))]),
  ),
};

// ---------- astrolabe ----------

const astrolabeData = {
  solar_date: astrolabe.solarDate,
  lunar_date: astrolabe.lunarDate,
  chinese_date: astrolabe.chineseDate,
  time: astrolabe.time,
  time_range: astrolabe.timeRange,
  sign: astrolabe.sign,
  zodiac: astrolabe.zodiac,
  soul: astrolabe.soul,
  body: astrolabe.body,
  five_elements_class: astrolabe.fiveElementsClass,
  earthly_branch_of_soul_palace: astrolabe.earthlyBranchOfSoulPalace,
  earthly_branch_of_body_palace: astrolabe.earthlyBranchOfBodyPalace,
  palaces_count: astrolabe.palaces.length,
  palaces: astrolabe.palaces.map((p, index) => ({
    index,
    name: p.name,
    heavenly_stem: p.heavenlyStem,
    earthly_branch: p.earthlyBranch,
    is_body_palace: p.isBodyPalace,
    is_original_palace: p.isOriginalPalace,
    major_stars_count: p.majorStars.filter((s) => s.type === 'major').length,
    minor_stars_count: p.minorStars.length,
    adjective_stars_count: p.adjectiveStars.length,
    decadal_range: p.decadal.range,
    decadal_heavenly_stem: p.decadal.heavenlyStem,
    decadal_earthly_branch: p.decadal.earthlyBranch,
  })),
};

// ---------- palace ----------

const palaceData = {};
for (let i = 0; i < 12; i++) {
  const p = astrolabe.palace(i);
  palaceData[String(i)] = {
    name: p.name,
    index: p.index,
    [`has_${PROBE_STAR_2}`]: p.has([PROBE_STAR_2]),
    [`has_${PROBE_STAR}`]: p.has([PROBE_STAR]),
    [`not_have_${PROBE_STAR}`]: p.notHave([PROBE_STAR]),
    [`has_one_of_${PROBE_STAR_2}_${PROBE_STAR}`]: p.hasOneOf([PROBE_STAR_2, PROBE_STAR]),
    has_mutagen_禄: p.hasMutagen('禄'),
    has_mutagen_权: p.hasMutagen('权'),
    has_mutagen_科: p.hasMutagen('科'),
    has_mutagen_忌: p.hasMutagen('忌'),
    not_have_mutagen_禄: p.notHaveMutagen('禄'),
    is_empty: p.isEmpty(),
    flies_to_6_禄: p.fliesTo(6, ['禄']),
    flies_to_0_权: p.fliesTo(0, ['权']),
    self_mutaged_禄: p.selfMutaged(['禄']),
    self_mutaged_one_of_empty: p.selfMutagedOneOf([]),
    self_mutaged_one_of_禄权: p.selfMutagedOneOf(['禄', '权']),
    not_self_mutaged_empty: p.notSelfMutaged([]),
    not_self_mutaged_禄权: p.notSelfMutaged(['禄', '权']),
    mutaged_places_length: p.mutagedPlaces().length,
  };
}
palaceData.by_name = Object.fromEntries(
  astrolabe.palaces.map((p) => [p.name, { name: p.name, index: p.index }]),
);

// ---------- surround ----------

const surroundData = {};
for (let i = 0; i < 12; i++) {
  const sp = astrolabe.surroundedPalaces(i);
  surroundData[String(i)] = {
    target_name: sp.target.name,
    target_index: sp.target.index,
    opposite_name: sp.opposite.name,
    opposite_index: sp.opposite.index,
    wealth_name: sp.wealth.name,
    wealth_index: sp.wealth.index,
    career_name: sp.career.name,
    career_index: sp.career.index,
    [`have_${PROBE_STAR}`]: sp.have([PROBE_STAR]),
    [`not_have_${PROBE_STAR}`]: sp.notHave([PROBE_STAR]),
    [`have_one_of_${PROBE_STAR_2}_${PROBE_STAR}`]: sp.haveOneOf([PROBE_STAR_2, PROBE_STAR]),
    have_mutagen_禄: sp.haveMutagen('禄'),
    have_mutagen_权: sp.haveMutagen('权'),
    not_have_mutagen_禄: sp.notHaveMutagen('禄'),
  };
}

// ---------- horoscope ----------

const horoscopeData = {
  lunar_date: horoscope.lunarDate,
  solar_date: horoscope.solarDate,
  decadal_index: horoscope.decadal.index,
  decadal_heavenly_stem: horoscope.decadal.heavenlyStem,
  decadal_earthly_branch: horoscope.decadal.earthlyBranch,
  decadal_mutagen: horoscope.decadal.mutagen,
  age_index: horoscope.age.index,
  age_nominal_age: horoscope.age.nominalAge,
  yearly_index: horoscope.yearly.index,
  yearly_mutagen: horoscope.yearly.mutagen,
  monthly_index: horoscope.monthly.index,
  monthly_mutagen: horoscope.monthly.mutagen,
  daily_index: horoscope.daily.index,
  daily_mutagen: horoscope.daily.mutagen,
  hourly_index: horoscope.hourly.index,
  hourly_mutagen: horoscope.hourly.mutagen,
  age_palace_name: horoscope.agePalace().name,
  palace_命宫_origin: horoscope.palace('命宫', 'origin').name,
  palace_命宫_decadal: horoscope.palace('命宫', 'decadal').name,
  palace_命宫_yearly: horoscope.palace('命宫', 'yearly').name,
  has_horoscope_mutagen_命宫_decadal_禄: horoscope.hasHoroscopeMutagen('命宫', 'decadal', '禄'),
  has_horoscope_mutagen_命宫_yearly_禄: horoscope.hasHoroscopeMutagen('命宫', 'yearly', '禄'),
  has_horoscope_stars_命宫_decadal: horoscope.hasHoroscopeStars('命宫', 'decadal', [PROBE_STAR_2]),
  not_have_horoscope_stars_命宫_decadal: horoscope.notHaveHoroscopeStars('命宫', 'decadal', [
    PROBE_STAR_2,
  ]),
  surround_palaces_命宫_origin_target: horoscope.surroundPalaces('命宫', 'origin').target.name,
};

const out = {
  params: {
    solar_date: BIRTH.date,
    time_index: BIRTH.timeIndex,
    gender: BIRTH.gender,
    target_date: TARGET.date,
    target_time_index: TARGET.timeIndex,
    probe_star: PROBE_STAR,
    probe_star_2: PROBE_STAR_2,
  },
  util,
  astrolabe: astrolabeData,
  palace: palaceData,
  surround: surroundData,
  horoscope: horoscopeData,
};

writeFileSync(join(__dirname, 'regression_data.json'), JSON.stringify(out, null, 2));
console.log('Generated regression_data.json');

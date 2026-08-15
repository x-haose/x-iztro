/**
 * 天盘/地盘/人盘（astroType）金标生成器。
 *
 * iztro 只有 astro.withOptions 能传 astroType，天盘即常规 bySolar 结果，
 * 地盘以身宫干支、人盘以福德宫干支起五行局重排。
 *
 * 产出两份数据：
 * - astrotype.csv：60 年 × 4 个跨季日期 × 13 时辰 × 男女 × {地盘,人盘}
 *   的规范化串哈希（行格式 `date,ti,g,astroType,hash`），覆盖广度。
 * - astrotype_full.json：少量命盘的全字段导出，覆盖杂耀顺序等
 *   哈希（对辅星杂耀排序）看不见的细节。
 *
 * 单例重放：
 *   node generate_astrotype.mjs --inspect <date> <ti> <男|女> <earth|human>
 */
import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';
import { kot, t } from 'iztro/lib/i18n/index.js';
import { earthlyBranches } from 'iztro/lib/data/index.js';

const START_YEAR = 1984;
const END_YEAR = 2043;
const MONTH_DAYS = [
  [2, 15],
  [5, 6],
  [8, 23],
  [11, 30],
];
/** CSV 里性别记 0/1，与其他生成器一致 */
const GENDERS = ['男', '女'];
const ASTRO_TYPES = ['earth', 'human'];
const HASH_LEN = 32;

/**
 * 以指定视角排盘。
 *
 * 重排后身宫与命宫都会挪位，随之：身宫地支取带身宫标记那一宫的地支，
 * 命主星按新命宫地支查表。两者是十二宫数据的派生值，在此按十二宫算出，
 * 使金标与盘面自洽。
 */
const chart = (dateStr, timeIndex, gender, astroType) => {
  const result = astro.withOptions({
    type: 'solar',
    dateStr,
    timeIndex,
    gender,
    fixLeap: true,
    language: 'zh-CN',
    astroType,
  });

  const bodyPalace = result.palaces.find((p) => p.isBodyPalace);
  result.earthlyBranchOfBodyPalace = bodyPalace.earthlyBranch;

  const soulPalace = result.palaces.find((p) => kot(p.name) === 'soulPalace');
  result.soul = t(earthlyBranches[kot(soulPalace.earthlyBranch, 'Earthly')].soul);

  return result;
};

if (process.argv[2] === '--inspect') {
  const [date, t, gender, astroType] = process.argv.slice(3);
  console.log(canonicalAstrolabe(chart(date, Number(t), gender, astroType)));
  process.exit(0);
}

// ============================================================
// 1. 广度矩阵：哈希
// ============================================================

const lines = [];

for (let year = START_YEAR; year <= END_YEAR; year++) {
  for (const [month, day] of MONTH_DAYS) {
    const date = `${year}-${month}-${day}`;
    for (let timeIndex = 0; timeIndex <= 12; timeIndex++) {
      for (const g of [0, 1]) {
        for (const astroType of ASTRO_TYPES) {
          const hash = hashAstrolabe(chart(date, timeIndex, GENDERS[g], astroType)).slice(0, HASH_LEN);
          lines.push(`${date},${timeIndex},${g},${astroType},${hash}`);
        }
      }
    }
  }
}

writeFileSync('astrotype.csv', lines.join('\n') + '\n');
console.log(`astrotype.csv: ${lines.length} cases`);

// ============================================================
// 2. 全字段导出：杂耀顺序等哈希看不见的细节
// ============================================================

const FULL_CASES = [
  ['2000-8-16', 2, '女'],
  ['1990-11-5', 4, '男'],
  ['1984-2-15', 12, '男'],
  ['2043-11-30', 0, '女'],
];

const exportStar = (s) => ({
  name: s.name,
  type: s.type,
  brightness: s.brightness || null,
  mutagen: s.mutagen || null,
});

const full = [];

for (const [date, timeIndex, gender] of FULL_CASES) {
  for (const astroType of ASTRO_TYPES) {
    const result = chart(date, timeIndex, gender, astroType);
    full.push({
      params: { solar_date: date, time_index: timeIndex, gender, astro_type: astroType },
      soul_palace_branch: result.earthlyBranchOfSoulPalace,
      body_palace_branch: result.earthlyBranchOfBodyPalace,
      five_elements_class: result.fiveElementsClass,
      soul_star: result.soul,
      body_star: result.body,
      palaces: result.palaces.map((p) => ({
        name: p.name,
        heavenly_stem: p.heavenlyStem,
        earthly_branch: p.earthlyBranch,
        is_body_palace: p.isBodyPalace,
        is_original_palace: p.isOriginalPalace,
        major_stars: p.majorStars.map(exportStar),
        minor_stars: p.minorStars.map(exportStar),
        // 顺序敏感：重排时挪动的杂耀会追加到末尾，与初次排盘不同
        adjective_stars: p.adjectiveStars.map((s) => s.name),
        changsheng12: p.changsheng12,
        boshi12: p.boshi12,
        jiangqian12: p.jiangqian12,
        suiqian12: p.suiqian12,
        decadal_range: p.decadal.range,
        decadal_heavenly_stem: p.decadal.heavenlyStem,
        decadal_earthly_branch: p.decadal.earthlyBranch,
        ages: p.ages,
      })),
    });
  }
}

writeFileSync('astrotype_full.json', JSON.stringify(full));
console.log(`astrotype_full.json: ${full.length} cases`);

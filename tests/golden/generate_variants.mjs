/**
 * 变体金标生成器：by_lunar 入口、六语言词表、中州派算法。
 *
 * - by_lunar：全部闰月年的闰月日期（is_leap_month × fix_leap 组合）
 *   加每年一个普通农历日期，输出规范化串哈希（variants_bylunar.csv，
 *   行格式 `lunarDate,ti,g,isLeap,fixLeap,hash`）。
 * - 六语言：3 个命盘 × 6 语言的全字段导出（variants_languages.json）。
 * - 中州派：60 年 × 4 个跨季日期 × 13 时辰 × 男女，输出规范化串哈希
 *   （variants_zhongzhou.csv，行格式 `date,ti,g,hash`）。
 *   iztro 的 config() 为全局设置，中州派段必须最后生成。
 *
 * 单例重放：
 *   node generate_variants.mjs --inspect-zhongzhou <date> <ti> <男|女>
 *   node generate_variants.mjs --inspect-bylunar <lunarDate> <ti> <男|女> <isLeap 0|1> <fixLeap 0|1>
 */
import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { LunarYear } from 'lunar-typescript';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));

const START_YEAR = 1984;
const END_YEAR = 2043;
const HASH_LEN = 32;
const GENDERS = ['男', '女'];

if (process.argv[2] === '--inspect-zhongzhou') {
  const [date, t, gender] = process.argv.slice(3);
  astro.config({ algorithm: 'zhongzhou' });
  console.log(canonicalAstrolabe(astro.bySolar(date, Number(t), gender, true, 'zh-CN')));
  process.exit(0);
}
if (process.argv[2] === '--inspect-bylunar') {
  const [ld, t, gender, il, fl] = process.argv.slice(3);
  console.log(
    canonicalAstrolabe(astro.byLunar(ld, Number(t), gender, il === '1', fl === '1', 'zh-CN')),
  );
  process.exit(0);
}

// ============================================================
// 1. by_lunar：闰月全日期 + 普通日期采样
// ============================================================

const bylunarLines = [];

for (let year = START_YEAR; year <= END_YEAR; year++) {
  const lunarYear = LunarYear.fromYear(year);
  const leapMonth = lunarYear.getLeapMonth();

  // 普通农历日期（含 is_leap_month=true 在无闰月月份不生效的用例）
  for (const [month, day, il] of [[7, 17, 0], [7, 17, 1]]) {
    for (const g of [0, 1]) {
      const result = astro.byLunar(`${year}-${month}-${day}`, 2, GENDERS[g], il === 1, true, 'zh-CN');
      const hash = hashAstrolabe(result).slice(0, HASH_LEN);
      bylunarLines.push(`${year}-${month}-${day},2,${g},${il},1,${hash}`);
    }
  }

  if (leapMonth <= 0) continue;

  // 闰月每一天 × is_leap_month {0,1} × fix_leap {0,1} × 时辰 {0,6,12}
  // iztro 的 lunar2solar 总是先按普通月解析日期再切换闰月，
  // 因此普通月天数之外的闰月末日（如闰月三十）在 JS 侧不可达，此处排除
  const leapDays = lunarYear.getMonth(-leapMonth).getDayCount();
  const normalDays = lunarYear.getMonth(leapMonth).getDayCount();
  for (const il of [0, 1]) {
    const maxDay = il === 1 ? Math.min(leapDays, normalDays) : normalDays;
    for (let day = 1; day <= maxDay; day++) {
      for (const fl of [0, 1]) {
        for (const t of [0, 6, 12]) {
          const ld = `${year}-${leapMonth}-${day}`;
          const result = astro.byLunar(ld, t, '女', il === 1, fl === 1, 'zh-CN');
          const hash = hashAstrolabe(result).slice(0, HASH_LEN);
          bylunarLines.push(`${ld},${t},1,${il},${fl},${hash}`);
        }
      }
    }
  }
}

writeFileSync(join(__dirname, 'variants_bylunar.csv'), bylunarLines.join('\n') + '\n');
console.log(`by_lunar: ${bylunarLines.length} cases`);

// ============================================================
// 2. 六语言全字段
// ============================================================

const LANGS = ['zh-CN', 'zh-TW', 'en-US', 'ja-JP', 'ko-KR', 'vi-VN'];
const LANG_CASES = [
  ['2000-8-16', 2, '女'],
  ['2004-4-10', 12, '男'],
  ['1990-11-5', 6, '女'],
];

const langCases = [];
for (const [d, t, g] of LANG_CASES) {
  for (const lang of LANGS) {
    const r = astro.bySolar(d, t, g, true, lang);
    langCases.push({
      p: { d, t, g: g === '男' ? 0 : 1, lang },
      gender: r.gender,
      time: r.time,
      sign: r.sign,
      zodiac: r.zodiac,
      chinese_date: r.chineseDate,
      soul: r.soul,
      body: r.body,
      five_elements_class: r.fiveElementsClass,
      soul_branch: r.earthlyBranchOfSoulPalace,
      palaces: r.palaces.map((p) => ({
        name: p.name,
        hs: p.heavenlyStem,
        eb: p.earthlyBranch,
        ms: p.majorStars.map((s) => `${s.name}:${s.brightness || ''}:${s.mutagen || ''}`),
        ns: p.minorStars.map((s) => `${s.name}:${s.brightness || ''}:${s.mutagen || ''}`).sort(),
        adj: p.adjectiveStars.map((s) => s.name).sort(),
        cs: p.changsheng12,
        bo: p.boshi12,
        jq: p.jiangqian12,
        sq: p.suiqian12,
      })),
    });
  }
}
writeFileSync(join(__dirname, 'variants_languages.json'), JSON.stringify(langCases));
console.log(`languages: ${langCases.length} cases`);

// ============================================================
// 3. 中州派（全局 config，必须最后）
// ============================================================

astro.config({ algorithm: 'zhongzhou' });

// 每年取跨四季的 4 个日期：中州派差异虽由年干支/性别/命宫索引驱动，
// 多日期采样防备未知的月日相关分支
const ZZ_DATES = ['2-15', '6-1', '9-15', '12-31'];

const zzLines = [];
for (let year = START_YEAR; year <= END_YEAR; year++) {
  for (const md of ZZ_DATES) {
    const dateStr = `${year}-${md}`;
    for (let t = 0; t <= 12; t++) {
      for (const g of [0, 1]) {
        const result = astro.bySolar(dateStr, t, GENDERS[g], true, 'zh-CN');
        const hash = hashAstrolabe(result).slice(0, HASH_LEN);
        zzLines.push(`${dateStr},${t},${g},${hash}`);
      }
    }
  }
}
writeFileSync(join(__dirname, 'variants_zhongzhou.csv'), zzLines.join('\n') + '\n');
console.log(`zhongzhou: ${zzLines.length} cases`);

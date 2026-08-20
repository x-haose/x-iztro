/**
 * 1602 年闰二月窗口金标生成器。
 *
 * lunar_rust 1.0.1 对农历 1602 年（明万历三十年）闰二月的合朔算晚一天，
 * x-iztro 在换算层（src/astro/lunar_table.rs）修正该缺陷；本金标逐日覆盖
 * 受影响窗口及其两侧边界，锁定修正后与 JS iztro 的逐字节一致：
 *
 *   1602-2-20 … 1602-4-25（二月末段、闰二月全月、三月首段）
 *   × 13 时辰 × 男女，闰月日期额外生成 fix_leap=0。
 *
 * 输出：tier_1602/year_1602.csv，行格式与 tier3/tier_edge 一致
 * （`date,ti,g,fl,hash`），hash 为规范化串（canonical.mjs）SHA-256 的
 * 前 32 个 hex 字符。
 *
 * 单例重放（排查哈希不一致时用）：
 *   node generate_1602.mjs --inspect <date> <timeIndex> <男|女> <1|0>
 */
import { astro } from 'iztro';
import { solar2lunar } from 'lunar-lite';
import { mkdirSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outDir = join(__dirname, 'tier_1602');

const GENDERS = ['男', '女'];
const HASH_LEN = 32;

if (process.argv[2] === '--inspect') {
  const [date, t, gender, fl] = process.argv.slice(3);
  const result = astro.bySolar(date, Number(t), gender, fl !== '0', 'zh-CN');
  console.log(canonicalAstrolabe(result));
  process.exit(0);
}

/** 覆盖窗口内的全部公历日期（含月界两侧）。 */
function windowDates() {
  const dates = [];
  for (let d = 20; d <= 31; d++) dates.push(`1602-2-${d}`);
  for (let d = 1; d <= 31; d++) dates.push(`1602-3-${d}`);
  for (let d = 1; d <= 25; d++) dates.push(`1602-4-${d}`);
  // 1602 年 2 月只有 28 天
  return dates.filter((s) => {
    const [, m, d] = s.split('-').map(Number);
    return !(m === 2 && d > 28);
  });
}

mkdirSync(outDir, { recursive: true });

const lines = [];
for (const dateStr of windowDates()) {
  const fixLeaps = solar2lunar(dateStr).isLeap ? [1, 0] : [1];
  for (let t = 0; t <= 12; t++) {
    for (let gi = 0; gi < GENDERS.length; gi++) {
      for (const fl of fixLeaps) {
        const result = astro.bySolar(dateStr, t, GENDERS[gi], fl === 1, 'zh-CN');
        lines.push(`${dateStr},${t},${gi},${fl},${hashAstrolabe(result).slice(0, HASH_LEN)}`);
      }
    }
  }
}

writeFileSync(join(outDir, 'year_1602.csv'), lines.join('\n') + '\n');
console.log(`Done. Total cases: ${lines.length}`);

/**
 * 边界年代金标生成器：补齐其它金标（1984-2043）之外的日期区间。
 *
 * 覆盖：1-1551 每 50 年取 1 年、1582、1583-1983 与 2044-2094 每 10 年取 1 年、2100，
 * × 12 个月 × 日 {1,15,28}；另加儒略历特例日（公元 1 年元旦、世纪年的儒略闰日、改历前后两日）。
 * 每个日期 × 13 时辰 × 男女，闰月日期额外生成 fix_leap=0。历法换算本身由 calendar.csv 逐日全量覆盖，
 * 此处验证排盘流程在各年代端到端一致。
 *
 * 输出：tier_edge/year_YYYY.csv，行格式与 tier3 一致（`date,ti,g,fl,hash`），
 * hash 为规范化串（见 canonical.mjs）SHA-256 的前 32 个 hex 字符。
 * 已存在的年文件跳过，可断点续跑。
 *
 * 单例重放（排查哈希不一致时用）：
 *   node generate_tier_edge.mjs --inspect <date> <timeIndex> <男|女> <1|0>
 */
import { astro } from 'iztro';
import { solar2lunar } from 'lunar-lite';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outDir = join(__dirname, 'tier_edge');

const GENDERS = ['男', '女'];
const DAYS = [1, 15, 28];
/** 儒略历特例日：公元 1 年元旦、儒略闰日（含格里历下不存在的世纪年闰日）、改历前后两日。 */
const SPECIAL_DATES = ['1-1-1', '4-2-29', '900-2-29', '1000-2-29', '1500-2-29', '1582-10-4', '1582-10-15'];

/** 采样日期按年分组。 */
function sampleDates() {
  const years = [];
  for (let y = 1; y <= 1551; y += 50) years.push(y);
  years.push(1582);
  for (let y = 1583; y <= 1983; y += 10) years.push(y);
  for (let y = 2044; y <= 2094; y += 10) years.push(y);
  years.push(2100);
  const byYear = new Map(years.map((y) => [y, []]));
  for (const y of years) {
    for (let month = 1; month <= 12; month++) {
      for (const day of DAYS) byYear.get(y).push(`${y}-${month}-${day}`);
    }
  }
  for (const date of SPECIAL_DATES) {
    const y = Number(date.split('-')[0]);
    if (!byYear.has(y)) byYear.set(y, []);
    if (!byYear.get(y).includes(date)) byYear.get(y).push(date);
  }
  return byYear;
}

if (process.argv[2] === '--inspect') {
  const [date, t, gender, fl] = process.argv.slice(3);
  const result = astro.bySolar(date, Number(t), gender, fl !== '0', 'zh-CN');
  console.log(canonicalAstrolabe(result));
  process.exit(0);
}

mkdirSync(outDir, { recursive: true });

let totalCases = 0;
for (const [year, dates] of sampleDates()) {
  const outPath = join(outDir, `year_${year}.csv`);
  if (existsSync(outPath)) {
    console.log(`Year ${year}: exists, skipped`);
    continue;
  }

  const lines = [];
  for (const dateStr of dates) {
    const fixLeaps = solar2lunar(dateStr).isLeap ? [1, 0] : [1];
    for (let t = 0; t <= 12; t++) {
      for (let gi = 0; gi < GENDERS.length; gi++) {
        for (const fl of fixLeaps) {
          const result = astro.bySolar(dateStr, t, GENDERS[gi], fl === 1, 'zh-CN');
          lines.push(`${dateStr},${t},${gi},${fl},${hashAstrolabe(result)}`);
        }
      }
    }
  }

  writeFileSync(outPath, lines.join('\n') + '\n');
  totalCases += lines.length;
  console.log(`Year ${year}: ${lines.length} cases (total: ${totalCases})`);
}

console.log(`Done. Total cases: ${totalCases}`);

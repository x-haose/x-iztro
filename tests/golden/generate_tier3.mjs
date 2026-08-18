/**
 * Tier 3 金标生成器：全参数空间哈希。
 *
 * 覆盖：1984-2043 每一天 × 13 时辰 × 男女 × fix_leap。
 * 非闰月日期 fix_leap 不影响任何输出，只生成 fl=1；
 * 闰月日期额外生成 fl=0，打穿闰月修正的全部分支。
 *
 * 输出：tier3/year_YYYY.csv，行格式 `date,ti,g,fl,hash`，
 * hash 为规范化串（见 canonical.mjs）SHA-256 的前 32 个 hex 字符。
 * 已存在的年文件跳过，可断点续跑。
 *
 * 单例重放（排查哈希不一致时用）：
 *   node generate_tier3.mjs --inspect <date> <timeIndex> <男|女> <1|0>
 *
 * 年份分段（多进程并行跑满时可用，各段互不重叠）：
 *   node generate_tier3.mjs --range <startYear> <endYear>
 */
import { astro } from 'iztro';
import { solar2lunar } from 'lunar-lite';
import { existsSync, mkdirSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outDir = join(__dirname, 'tier3');

const START_YEAR = 1984;
const END_YEAR = 2043;
const GENDERS = ['男', '女'];
const HASH_LEN = 32;

function daysInMonth(year, month) {
  return new Date(year, month, 0).getDate();
}

if (process.argv[2] === '--inspect') {
  const [date, t, gender, fl] = process.argv.slice(3);
  const result = astro.bySolar(date, Number(t), gender, fl !== '0', 'zh-CN');
  console.log(canonicalAstrolabe(result));
  process.exit(0);
}

mkdirSync(outDir, { recursive: true });

const [rangeStart, rangeEnd] =
  process.argv[2] === '--range'
    ? [Number(process.argv[3]), Number(process.argv[4])]
    : [START_YEAR, END_YEAR];

let totalCases = 0;
const startTime = Date.now();

for (let year = rangeStart; year <= rangeEnd; year++) {
  const outPath = join(outDir, `year_${year}.csv`);
  if (existsSync(outPath)) {
    console.log(`Year ${year}: exists, skipped`);
    continue;
  }

  const lines = [];
  for (let month = 1; month <= 12; month++) {
    const maxDay = daysInMonth(year, month);
    for (let day = 1; day <= maxDay; day++) {
      const dateStr = `${year}-${month}-${day}`;
      const fixLeaps = solar2lunar(dateStr).isLeap ? [1, 0] : [1];
      for (let t = 0; t <= 12; t++) {
        for (let gi = 0; gi < GENDERS.length; gi++) {
          for (const fl of fixLeaps) {
            const result = astro.bySolar(dateStr, t, GENDERS[gi], fl === 1, 'zh-CN');
            const hash = hashAstrolabe(result).slice(0, HASH_LEN);
            lines.push(`${dateStr},${t},${gi},${fl},${hash}`);
          }
        }
      }
    }
  }

  writeFileSync(outPath, lines.join('\n') + '\n');
  totalCases += lines.length;
  const elapsed = ((Date.now() - startTime) / 1000).toFixed(0);
  console.log(`Year ${year}: ${lines.length} cases (total: ${totalCases}, ${elapsed}s)`);
}

console.log(`Done. Total cases: ${totalCases}`);

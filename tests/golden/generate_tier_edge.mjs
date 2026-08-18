/**
 * 边界年代金标生成器：补齐其它金标（1984-2043）之外的日期区间。
 *
 * 覆盖：1583-1983 与 2044-2100 每 10 年取 1 年（另单取 2100）
 * × 12 个月 × 日 {1,15,28} × 13 时辰 × 男女，闰月日期额外生成 fix_leap=0。
 * 1583 是 x-iztro 的公历下界；JS 侧 iztro/lunar-lite 对更早年份不报错而是
 * 外推，故区间上下界由 x-iztro 的入口校验决定，非 JS 能力所限。
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
const HASH_LEN = 32;

/** 早期与晚期两段各每 10 年一个采样年，2100 作为区间末端单列。 */
function sampleYears() {
  const years = [];
  for (let y = 1583; y <= 1983; y += 10) years.push(y);
  for (let y = 2044; y <= 2094; y += 10) years.push(y);
  years.push(2100);
  return years;
}

if (process.argv[2] === '--inspect') {
  const [date, t, gender, fl] = process.argv.slice(3);
  const result = astro.bySolar(date, Number(t), gender, fl !== '0', 'zh-CN');
  console.log(canonicalAstrolabe(result));
  process.exit(0);
}

mkdirSync(outDir, { recursive: true });

let totalCases = 0;
for (const year of sampleYears()) {
  const outPath = join(outDir, `year_${year}.csv`);
  if (existsSync(outPath)) {
    console.log(`Year ${year}: exists, skipped`);
    continue;
  }

  const lines = [];
  for (let month = 1; month <= 12; month++) {
    for (const day of DAYS) {
      const dateStr = `${year}-${month}-${day}`;
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
  }

  writeFileSync(outPath, lines.join('\n') + '\n');
  totalCases += lines.length;
  console.log(`Year ${year}: ${lines.length} cases (total: ${totalCases})`);
}

console.log(`Done. Total cases: ${totalCases}`);

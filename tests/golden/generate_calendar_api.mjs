/**
 * 历法查询入口的金标：按农历年取 lunar-typescript 的月份表、年干支与逐日农历→公历换算。
 *
 * 每个农历年一行 `year,hash`，hash 为下列规范化串 SHA-256 的前 32 个 hex 字符：
 * - `M 月,闰(0/1),天数,初一公历` —— 该年各月，按 LunarYear.getMonthsInYear 的先后；
 * - `P 干序,支序` —— 该年首月初一的年干支（正月初一换年，甲 = 0、子 = 0）；
 * - `D 月,闰,日,公历` —— 该年每一天对应的公历日期（Lunar.fromYmd(...).getSolar().toYmd()）。
 * 公历写 `YYYY-MM-DD`；农历 9999 年腊月初三起的 27 天落在公历 10000 年。
 *
 * Rust 侧 tests/calendar_api.rs 用公开入口（lunar_months、lunar_year_pillar、lunar_to_solar）
 * 逐行重建同一规范化串比对，并对每一天检查 solar_to_lunar 换回原日期。
 *
 * 输出：calendar_api.csv（`--range <起> <止>` 只算一段并写到 stdout，多进程并行后按年份拼接）
 * 单年重放：node generate_calendar_api.mjs --inspect <year>
 */
import { Lunar, LunarYear } from 'lunar-typescript';
import { createHash } from 'node:crypto';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const hash = (lines) => createHash('sha256').update(lines.join('\n')).digest('hex').slice(0, 32);

function yearLines(y) {
  const months = LunarYear.fromYear(y).getMonthsInYear();
  const lines = [];
  for (const m of months) {
    const signed = m.getMonth();
    const first = Lunar.fromYmd(y, signed, 1).getSolar().toYmd();
    lines.push(`M ${Math.abs(signed)},${signed < 0 ? 1 : 0},${m.getDayCount()},${first}`);
  }
  const head = Lunar.fromYmd(y, months[0].getMonth(), 1);
  lines.push(`P ${head.getYearGanIndex()},${head.getYearZhiIndex()}`);
  for (const m of months) {
    const signed = m.getMonth();
    for (let d = 1; d <= m.getDayCount(); d++) {
      const solar = Lunar.fromYmd(y, signed, d).getSolar().toYmd();
      lines.push(`D ${Math.abs(signed)},${signed < 0 ? 1 : 0},${d},${solar}`);
    }
  }
  return lines;
}

if (process.argv[2] === '--inspect') {
  console.log(yearLines(Number(process.argv[3])).join('\n'));
  process.exit(0);
}

const row = (y) => `${y},${hash(yearLines(y))}`;

if (process.argv[2] === '--range') {
  const [from, to] = process.argv.slice(3).map(Number);
  const rows = [];
  for (let y = from; y <= to; y++) rows.push(row(y));
  process.stdout.write(rows.join('\n') + '\n');
} else {
  const rows = [];
  for (let y = 1; y <= 9999; y++) rows.push(row(y));
  writeFileSync(join(__dirname, 'calendar_api.csv'), rows.join('\n') + '\n');
  console.log(`calendar_api.csv: ${rows.length} 年`);
}

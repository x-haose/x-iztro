/**
 * 历法全量金标：lunar-typescript 的公历 ↔ 农历换算、节气与八字各柱，逐年取哈希。
 *
 * 每个公历年一行 `year,yearHash,dayHash,jieQiHash`，哈希为下列规范化串 SHA-256 的前 32 个 hex 字符：
 * - year：农历该年的 15 个月（所属年、带符号月、天数、首日儒略日）、本年各月首末日的公历日期（经 Lunar.fromYmd）、
 *   闰月、31 个节气时刻（Solar.fromJulianDay(...).toYmdHms()）。
 * - day：该公历年逐日 12:30 的农历年月日、正月初一年柱、立春日年柱、精确月柱、日柱、星座序号。
 * - jieqi：该公历年每个「节」所在日的 13 个排盘时刻（0:30、1:30、3:30 … 21:30、23:30）的精确月柱、exact 日柱、时柱。
 * 干支一律写序号（甲 = 0、子 = 0）。公历日期按 iztro 的 validateDate：1-9999 年，1582-10-05…14 不存在，
 * 闰年按 SolarUtil.isLeapYear。
 *
 * 输出：calendar.csv（全量约数分钟；`--range <起> <止>` 只算一段并写到 stdout，多进程并行后按年份拼接）
 * 单年重放：node generate_calendar.mjs --inspect <year> <year|day|jieqi>
 */
import { Lunar, LunarYear, Solar, SolarUtil } from 'lunar-typescript';
import { createHash } from 'node:crypto';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 排盘时刻的小时：时辰 0-12 对应 max(2t-1, 0)。 */
const CHART_HOURS = Array.from({ length: 13 }, (_, t) => Math.max(t * 2 - 1, 0));

const hash = (lines) => createHash('sha256').update(lines.join('\n')).digest('hex').slice(0, 32);

const daysInMonth = (y, m) =>
  [31, SolarUtil.isLeapYear(y) ? 29 : 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31][m - 1];

function* solarDays(y) {
  for (let m = 1; m <= 12; m++) {
    for (let d = 1; d <= daysInMonth(y, m); d++) {
      if (y === 1582 && m === 10 && d >= 5 && d <= 14) continue;
      yield [m, d];
    }
  }
}

function yearLines(y) {
  const ly = LunarYear.fromYear(y);
  const lines = ly
    .getMonths()
    .map((m) => `M ${m.getYear()},${m.getMonth()},${m.getDayCount()},${m.getFirstJulianDay()}`);
  for (const m of ly.getMonthsInYear()) {
    const first = Lunar.fromYmd(y, m.getMonth(), 1).getSolar().toYmd();
    const last = Lunar.fromYmd(y, m.getMonth(), m.getDayCount()).getSolar().toYmd();
    lines.push(`S ${m.getMonth()},${first},${last}`);
  }
  lines.push(`L ${ly.getLeapMonth()}`);
  for (const jd of ly.getJieQiJulianDays()) lines.push(`Q ${Solar.fromJulianDay(jd).toYmdHms()}`);
  return lines;
}

function dayLines(y) {
  const lines = [];
  for (const [m, d] of solarDays(y)) {
    const solar = Solar.fromYmdHms(y, m, d, 12, 30, 0);
    const l = solar.getLunar();
    lines.push(
      [
        `${m}-${d}`,
        `${l.getYear()},${l.getMonth()},${l.getDay()}`,
        `${l.getYearGanIndex()},${l.getYearZhiIndex()}`,
        `${l.getYearGanIndexByLiChun()},${l.getYearZhiIndexByLiChun()}`,
        `${l.getMonthGanIndexExact()},${l.getMonthZhiIndexExact()}`,
        `${l.getDayGanIndex()},${l.getDayZhiIndex()}`,
        SolarUtil.XINGZUO.indexOf(solar.getXingZuo()),
      ].join(' '),
    );
  }
  return lines;
}

/** 该公历年内各「节」（节气表的偶数位）所在日，去重后按日期排序。 */
function jieDays(y) {
  const ly = LunarYear.fromYear(y);
  const days = new Set();
  ly.getJieQiJulianDays().forEach((jd, i) => {
    if (i % 2 !== 0) return;
    const s = Solar.fromJulianDay(jd);
    if (s.getYear() === y) days.add(`${s.getMonth()}-${s.getDay()}`);
  });
  return [...days].map((k) => k.split('-').map(Number)).sort((a, b) => a[0] - b[0] || a[1] - b[1]);
}

function jieQiLines(y) {
  return jieDays(y).map(([m, d]) => {
    const cells = CHART_HOURS.map((h) => {
      const l = Solar.fromYmdHms(y, m, d, h, 30, 0).getLunar();
      return [
        l.getMonthGanIndexExact(),
        l.getMonthZhiIndexExact(),
        l.getDayGanIndexExact(),
        l.getDayZhiIndexExact(),
        l.getTimeGanIndex(),
        l.getTimeZhiIndex(),
      ].join(',');
    });
    return `${m}-${d} ${cells.join(' ')}`;
  });
}

const LAYERS = { year: yearLines, day: dayLines, jieqi: jieQiLines };

if (process.argv[2] === '--inspect') {
  const [y, layer] = [Number(process.argv[3]), process.argv[4]];
  console.log(LAYERS[layer](y).join('\n'));
  process.exit(0);
}

const row = (y) => `${y},${hash(yearLines(y))},${hash(dayLines(y))},${hash(jieQiLines(y))}`;

if (process.argv[2] === '--range') {
  const [from, to] = process.argv.slice(3).map(Number);
  const rows = [];
  for (let y = from; y <= to; y++) rows.push(row(y));
  process.stdout.write(rows.join('\n') + '\n');
} else {
  const rows = [];
  for (let y = 1; y <= 9999; y++) rows.push(row(y));
  writeFileSync(join(__dirname, 'calendar.csv'), rows.join('\n') + '\n');
  console.log(`calendar.csv: ${rows.length} 年`);
}

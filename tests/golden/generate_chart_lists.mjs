/**
 * 夹宫与三个运限列表的金标（iztro v2.6.0 新增的四组 API）。
 *
 * 覆盖 tests/golden_chart_lists.rs 消费的四块：
 * - flankingPalaces：十二宫逐宫的前后邻宫索引与宫名
 * - decadalList：十二个大限的宫名、虚岁与年份区间、干支、四化、十二宫名
 * - yearlyList：每个大限内的十个流年
 * - monthlyList：fixLeap 真假两种，覆盖有闰月与无闰月的年份
 *
 * 取样盘刻意含晚子时（列表用的时辰取自时柱地支，与出生入参差 12）与闰月出生。
 *
 * 输出：chart_lists.json
 */
import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

/** 取样盘：普通、晚子时、闰月出生、早子时、跨世纪。 */
const CHARTS = [
  { d: '2000-8-16', t: 2, g: '女' },
  { d: '2000-8-16', t: 12, g: '男' },
  { d: '2004-3-21', t: 5, g: '女' },
  { d: '1984-2-15', t: 0, g: '男' },
  { d: '1999-12-31', t: 11, g: '女' },
];

/** 流月取样年：2020 闰四月、2021 无闰月、2023 闰二月。 */
const MONTHLY_YEARS = [2020, 2021, 2023];

const cases = [];

for (const { d, t, g } of CHARTS) {
  const a = astro.bySolar(d, t, g, true, 'zh-CN');
  const birth = { d, t, g };

  const flanking = [];
  for (let i = 0; i < 12; i++) {
    const f = a.flankingPalaces(i);
    flanking.push({
      index: i,
      previous: { index: f.previous.index, name: f.previous.name },
      next: { index: f.next.index, name: f.next.name },
    });
  }

  const decadals = a.decadalList().map((x) => ({
    index: x.index,
    name: x.name,
    palaceName: x.palaceName,
    ageRange: x.ageRange,
    yearRange: x.yearRange,
    heavenlyStem: x.heavenlyStem,
    earthlyBranch: x.earthlyBranch,
    palaceNames: x.palaceNames,
    mutagen: x.mutagen,
  }));

  // 每盘取三个大限的流年，覆盖首、中、末
  const yearly = {};
  for (const ordinal of [0, 5, 11]) {
    yearly[ordinal] = a.yearlyList(ordinal).map((y) => ({
      index: y.index,
      age: y.age,
      year: y.year,
      heavenlyStem: y.heavenlyStem,
      earthlyBranch: y.earthlyBranch,
      palaceNames: y.palaceNames,
      mutagen: y.mutagen,
    }));
  }

  const monthly = {};
  for (const year of MONTHLY_YEARS) {
    for (const fixLeap of [true, false]) {
      monthly[`${year}_${fixLeap}`] = a.monthlyList(year, fixLeap).map((m) => ({
        index: m.index,
        age: m.age,
        year: m.year,
        month: m.month,
        isLeapMonth: m.isLeapMonth,
        part: m.part,
        dayRange: m.dayRange,
        heavenlyStem: m.heavenlyStem,
        earthlyBranch: m.earthlyBranch,
        mutagen: m.mutagen,
      }));
    }
  }

  cases.push({ birth, flanking, decadals, yearly, monthly });
}

writeFileSync(join(__dirname, 'chart_lists.json'), JSON.stringify(cases, null, 1));
console.log(
  `chart_lists.json: ${cases.length} 盘 × (夹宫 12 + 大限 12 + 流年 3×10 + 流月 ${
    MONTHLY_YEARS.length * 2
  } 组)`
);

/**
 * 运限金标生成器。
 *
 * 命盘集：60 年（1984-2043）× 2 月 15 日 × 男女 × 时辰 {0, 6, 12}。
 * 每盘目标日期集：
 *   - 出生后 1..12 年的 6 月 15 日（覆盖 12 流年支，目标时辰随序号轮转 0-12）
 *   - 出生次年 3 月 1 日（童限）
 *   - 出生后 45 年 6 月 15 日（高龄大限）
 *   - 出生后 12 年窗口内第一个「闰月且农历日>15」的日期（闰月修正分支）
 *   - 出生后 2 年 8 月 15 日晚子时（目标日柱进位与流时索引）
 *
 * 每例导出六个运限层级的 index/name/干支/四化星名/流耀分布，
 * 以及小限虚岁与流年岁前/将前十二神。
 *
 * 输出：horoscope_data.json
 */
import { astro } from 'iztro';
import { solar2lunar } from 'lunar-lite';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outPath = join(__dirname, 'horoscope_data.json');

const START_YEAR = 1984;
const END_YEAR = 2043;
const GENDERS = ['男', '女'];
const BIRTH_TIME_INDICES = [0, 6, 12];

/** 从 fromYear 年 1 月 1 日起逐日扫描，返回第一个闰月下半月的日期（最多扫 4 年，闰月周期内必有）。 */
function findLeapSecondHalfDate(fromYear) {
  for (let year = fromYear; year < fromYear + 4; year++) {
    for (let month = 1; month <= 12; month++) {
      const maxDay = new Date(year, month, 0).getDate();
      for (let day = 1; day <= maxDay; day++) {
        const dateStr = `${year}-${month}-${day}`;
        const l = solar2lunar(dateStr);
        if (l.isLeap && l.lunarDay > 15) return dateStr;
      }
    }
  }
  throw new Error(`No leap second-half date found from ${fromYear}`);
}

/** 单个运限层级的紧凑导出（宫位名序列原序，流耀分布每宫排序）。 */
function scopeItem(item) {
  const out = {
    i: item.index,
    n: item.name,
    hs: item.heavenlyStem,
    eb: item.earthlyBranch,
    pn: item.palaceNames,
    m: item.mutagen,
  };
  if (item.stars) {
    out.s = item.stars.map((g) => g.map((s) => s.name).sort());
  }
  return out;
}

const cases = [];

for (let year = START_YEAR; year <= END_YEAR; year++) {
  const birthDate = `${year}-2-15`;
  const leapTarget = findLeapSecondHalfDate(year + 1);

  for (const gender of GENDERS) {
    for (const bt of BIRTH_TIME_INDICES) {
      const astrolabe = astro.bySolar(birthDate, bt, gender, true, 'zh-CN');

      const targets = [];
      for (let k = 1; k <= 12; k++) {
        targets.push([`${year + k}-6-15`, k % 13]);
      }
      targets.push([`${year + 1}-3-1`, 4]);
      targets.push([`${year + 45}-6-15`, 8]);
      targets.push([leapTarget, 6]);
      targets.push([`${year + 2}-8-15`, 12]);

      for (const [td, tt] of targets) {
        const h = astrolabe.horoscope(td, tt);
        cases.push({
          p: { d: birthDate, t: bt, g: gender === '男' ? 0 : 1 },
          td,
          tt,
          ld: h.lunarDate,
          dec: scopeItem(h.decadal),
          age: { ...scopeItem(h.age), na: h.age.nominalAge },
          yr: {
            ...scopeItem(h.yearly),
            sq: h.yearly.yearlyDecStar.suiqian12,
            jq: h.yearly.yearlyDecStar.jiangqian12,
          },
          mo: scopeItem(h.monthly),
          da: scopeItem(h.daily),
          hr: scopeItem(h.hourly),
        });
      }
    }
  }
}

writeFileSync(outPath, JSON.stringify(cases));
console.log(`Generated ${cases.length} horoscope cases -> ${outPath}`);

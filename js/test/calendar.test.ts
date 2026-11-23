/**
 * 历法查询入口对照 lunar-typescript 的金标 tests/golden/calendar_api.csv（generate_calendar_api.mjs 生成；
 * Rust 的 calendar_api.rs、Python 的 test_calendar_golden.py、Go 的 calendar_test.go 按同一规则重建）。
 * 按年重建规范化串，验证 JS 包装的字段映射与闰月选项，并检查每一天公历换回原农历日期。
 */

import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { calendar, EarthlyBranch, HeavenlyStem, init, IztroError, type RawSolarDate } from 'x-iztro';
import { GOLDEN } from './helpers.ts';

await init();

const EXPECTED = new Map(
  readFileSync(new URL('calendar_api.csv', GOLDEN), 'utf8')
    .trim()
    .split('\n')
    .map((line) => line.split(',') as [string, string]),
);

const pad = (n: number, width: number) => String(n).padStart(width, '0');
const ymd = (d: RawSolarDate) => `${pad(d.solarYear, 4)}-${pad(d.solarMonth, 2)}-${pad(d.solarDay, 2)}`;

function yearLines(y: number): string[] {
  const months = calendar.lunarMonths(y);
  const lines = months.map((m) => `M ${m.month},${Number(m.isLeap)},${m.dayCount},${ymd(m.firstDay)}`);
  const [stem, branch] = calendar.lunarYearPillar(y);
  lines.push(`P ${Object.values(HeavenlyStem).indexOf(stem)},${Object.values(EarthlyBranch).indexOf(branch)}`);
  for (const m of months) {
    for (let d = 1; d <= m.dayCount; d++) {
      const solar = calendar.lunarToSolar(y, m.month, d, { isLeap: m.isLeap });
      if (solar.solarYear <= 9999) {
        assert.deepEqual(
          calendar.solarToLunar(solar.solarYear, solar.solarMonth, solar.solarDay),
          { lunarYear: y, lunarMonth: m.month, lunarDay: d, isLeap: m.isLeap },
          ymd(solar),
        );
      }
      lines.push(`D ${m.month},${Number(m.isLeap)},${d},${ymd(solar)}`);
    }
  }
  return lines;
}

// 闰月年、无闰月年、缺三月的 237 年、改历的 1582 年、尾月落在公历 10000 年的 9999 年
for (const y of [1, 237, 1582, 2023, 2024, 9999]) {
  test(`农历 ${y} 年与 lunar-typescript 一致`, () => {
    const digest = createHash('sha256').update(yearLines(y).join('\n')).digest('hex').slice(0, 32);
    assert.equal(digest, EXPECTED.get(String(y)));
  });
}

test('年干支返回标识', () => {
  assert.deepEqual(calendar.lunarYearPillar(2024), [HeavenlyStem.JIA, EarthlyBranch.CHEN]);
});

test('不存在的日期报错，不退回常规月', () => {
  const code = (call: () => unknown) => {
    try {
      call();
    } catch (e) {
      assert.ok(e instanceof IztroError);
      return e.code;
    }
    return 'no error';
  };
  assert.equal(code(() => calendar.solarToLunar(1582, 10, 10)), 'invalid_date');
  assert.equal(code(() => calendar.solarToLunar(2023, 2, 29)), 'invalid_date');
  assert.equal(code(() => calendar.lunarToSolar(2024, 2, 1, { isLeap: true })), 'invalid_date');
  assert.equal(code(() => calendar.lunarToSolar(237, 3, 1)), 'invalid_date');
  assert.equal(code(() => calendar.lunarMonths(10000)), 'invalid_argument');
  assert.equal(code(() => calendar.lunarYearPillar(0)), 'invalid_argument');
});

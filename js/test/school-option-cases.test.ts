/**
 * 流派口径开关的四侧共用用例 tests/data/school_option_cases.json（期望由 Rust school_option_cases.rs 生成并核对，
 * Python test_school_option_cases.py、Go school_option_cases_test.go 读同一份）。
 */

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import {
  astro,
  init,
  type Astrolabe,
  type ChartConfig,
  type Gender,
  type PatternConfig,
  type TimeIndex,
} from 'x-iztro';

await init();

interface Case {
  name: string;
  chart: { solarDate: string; timeIndex: TimeIndex; gender: Gender };
  config: ChartConfig;
  patternConfig?: PatternConfig;
  expect: unknown;
}

const DOC = JSON.parse(
  readFileSync(new URL('../../tests/data/school_option_cases.json', import.meta.url), 'utf8'),
) as { target: [string, TimeIndex]; cases: Case[]; patternCases: Case[] };
const [TARGET_DATE, TARGET_TIME_INDEX] = DOC.target;

const chartOf = (c: Case): Astrolabe =>
  astro.bySolar(c.chart.solarDate, c.chart.timeIndex, c.chart.gender, { config: c.config });

for (const c of DOC.cases) {
  test(`流派口径: ${c.name}`, () => {
    const chart = chartOf(c);
    const lunar = chart.rawDates.lunarDate;
    const got = {
      lunarDate: [lunar.lunarYear, lunar.lunarMonth, lunar.lunarDay, lunar.isLeap],
      yearlyKeys: [...chart.rawDates.chineseDate.yearlyKeys],
      palaces: chart.palaces.map((p) => ({
        stars: [...p.majorStars, ...p.minorStars, ...p.adjectiveStars].map((s) => s.key),
        changsheng12Key: p.changsheng12Key,
        suiqian12Key: p.suiqian12Key,
        jiangqian12Key: p.jiangqian12Key,
        isOriginalPalace: p.isOriginalPalace,
        isSecondaryOriginalPalace: p.isSecondaryOriginalPalace === true,
      })),
      monthlyIndex: chart.horoscope(TARGET_DATE, TARGET_TIME_INDEX).monthly.index,
    };
    assert.deepEqual(got, c.expect);
  });
}

for (const c of DOC.patternCases) {
  test(`格局口径: ${c.name}`, () => {
    const hits = chartOf(c)
      .patterns(c.patternConfig)
      .map((h) => [h.key, h.variant ?? null]);
    assert.deepEqual(hits, c.expect);
  });
}

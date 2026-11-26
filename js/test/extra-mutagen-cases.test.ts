/**
 * 附加四化星的四侧共用用例 tests/data/extra_mutagen_cases.json（期望由 Rust extra_mutagen_cases.rs 生成并核对，
 * Python test_extra_mutagen_cases.py、Go extra_mutagen_cases_test.go 读同一份）。
 */

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { astro, init, Mutagen, type ChartConfig, type Gender, type TimeIndex } from 'x-iztro';

await init();

interface Case {
  name: string;
  chart: { solarDate: string; timeIndex: TimeIndex; gender: Gender };
  config: ChartConfig;
  palaces: {
    index: number;
    mutagenStarKeys: string[];
    mutagenStars: Record<string, string[]>;
    mutagedPlaces: (number | null)[];
    selfMutaged: Record<string, boolean>;
  }[];
}

const CASES = (
  JSON.parse(readFileSync(new URL('../../tests/data/extra_mutagen_cases.json', import.meta.url), 'utf8')) as {
    cases: Case[];
  }
).cases;

for (const c of CASES) {
  test(`附加四化星: ${c.name}`, () => {
    const chart = astro.bySolar(c.chart.solarDate, c.chart.timeIndex, c.chart.gender, { config: c.config });
    for (const want of c.palaces) {
      const p = chart.palaces[want.index]!;
      assert.deepEqual(p.mutagenStarKeys, want.mutagenStarKeys);
      for (const m of Object.values(Mutagen)) {
        assert.deepEqual(p.mutagenStars(m), want.mutagenStars[m], `${want.index} ${m}`);
        assert.equal(p.selfMutaged(m), want.selfMutaged[m], `${want.index} ${m}`);
      }
      assert.deepEqual(
        p.mutagedPlaces().map((q) => q?.index ?? null),
        want.mutagedPlaces,
      );
    }
  });
}

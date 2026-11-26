/**
 * 按主题选宫：四侧共用用例 tests/data/topic_palaces_cases.json（Rust topic_palaces.rs 按各层宫名另行推得并
 * 核对期望，Python test_topic.py、Go topic_test.go 读同一份），以及层级比查询精度细时报错。
 */

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { astro, init, IztroError, KnowledgePack, type Gender, type Scope, type TimeIndex, type TopicSelection } from 'x-iztro';

await init();

interface Case {
  name: string;
  chart: { solarDate: string; timeIndex: TimeIndex; gender: Gender };
  target?: { targetDate?: string; targetTimeIndex?: TimeIndex; targetYear?: number; targetMonth?: number };
  scope?: Scope;
  topic: string;
  knowledge?: object;
  expect: TopicSelection;
}

const CASES = (
  JSON.parse(readFileSync(new URL('../../tests/data/topic_palaces_cases.json', import.meta.url), 'utf8')) as {
    cases: Case[];
  }
).cases;

function select(c: Case): TopicSelection {
  const chart = astro.bySolar(c.chart.solarDate, c.chart.timeIndex, c.chart.gender);
  const knowledge = c.knowledge === undefined ? true : KnowledgePack.fromObject(c.knowledge);
  const t = c.target;
  if (t === undefined) return chart.topicPalaces(c.topic, { knowledge });
  const h =
    t.targetDate !== undefined
      ? chart.horoscope(t.targetDate, t.targetTimeIndex)
      : t.targetMonth !== undefined
        ? chart.horoscopeOfMonth(t.targetYear!, t.targetMonth)
        : chart.horoscopeOfYear(t.targetYear!);
  return h.topicPalaces(c.topic, c.scope!, { knowledge });
}

for (const c of CASES) {
  test(`主题选宫: ${c.name}`, () => {
    assert.deepEqual(select(c), c.expect);
  });
}

test('层级比查询精度细报错', () => {
  const h = astro.bySolar('2000-8-16', 2, 'female').horoscopeOfYear(2025);
  for (const call of [() => h.topicPalaces('romance', 'monthly'), () => h.topicToText('romance', 'monthly')]) {
    assert.throws(call, (e: unknown) => e instanceof IztroError && /only specifies layers down to 'yearly'/.test(e.message));
  }
});

/**
 * 格局：金标是 tests/golden/pattern_snapshots/*.json（Rust 的 tests/pattern_snapshot.rs 写出），每份含一张盘的
 * 排盘入参与本命、大限、流年、流月、流日、流时六层的命中 DTO。按文件里的入参重排，逐键逐值比对；
 * Python 与 Go 读同一批文件。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import { astro, BrightnessSource, init, IztroError, PalaceName, PatternKey, Scope, type ChartConfig, type PatternConfig, type PatternHitData, type Gender, type Language, type TimeIndex } from 'x-iztro';
import { golden, goldenFiles } from './helpers.ts';

await init();

interface PatternSnapshot {
  params: {
    solarDate: string;
    timeIndex: TimeIndex;
    gender: Gender;
    fixLeap: boolean;
    language: Language;
    config: Required<Pick<ChartConfig, 'yearDivide' | 'horoscopeDivide' | 'ageDivide' | 'dayDivide' | 'algorithm' | 'astroType'>>;
    patternConfig: Required<PatternConfig>;
    targetDate: string;
    targetTimeIndex: TimeIndex;
  };
  origin: PatternHitData[];
  decadal: PatternHitData[];
  yearly: PatternHitData[];
  monthly: PatternHitData[];
  daily: PatternHitData[];
  hourly: PatternHitData[];
}

const files = goldenFiles('pattern_snapshots').filter((f) => f.endsWith('.json'));

test('快照恰有 24 份（4 张盘 × 6 种语言）', () => {
  assert.equal(files.length, 24);
});

for (const file of files) {
  test(`格局命中与快照一致：${file}`, () => {
    const snap = golden<PatternSnapshot>(`pattern_snapshots/${file}`);
    const p = snap.params;
    const chart = astro.bySolar(p.solarDate, p.timeIndex, p.gender, { fixLeap: p.fixLeap, language: p.language, config: p.config });
    const config = p.patternConfig;
    const h = chart.horoscope(p.targetDate, p.targetTimeIndex);
    const actual = {
      origin: chart.patterns(config),
      decadal: h.patterns(Scope.DECADAL, config),
      yearly: h.patterns(Scope.YEARLY, config),
      monthly: h.patterns(Scope.MONTHLY, config),
      daily: h.patterns(Scope.DAILY, config),
      hourly: h.patterns(Scope.HOURLY, config),
    };
    for (const [layer, hits] of Object.entries(actual)) {
      const expected = snap[layer as keyof typeof actual];
      assert.equal(hits.length, expected.length, `${layer} 命中数`);
      hits.forEach((hit, i) => {
        const want = expected[i]!;
        assert.deepEqual(hit.toJSON(), want, `${layer} ${hit.key}`);
        assert.equal(hit.key, want.key);
        assert.equal(hit.scope, want.scope);
        assert.equal(hit.palaceIndex, want.palaceIndex);
        assert.equal(hit.palaceNameKey, want.palaceNameKey);
        assert.equal(hit.variant, want.variant);
        assert.deepEqual(hit.stars.map((s) => s.key), want.stars.map((s) => s.key));
      });
    }
  });
}

const chart = astro.bySolar('2000-8-16', 2, 'female');

test('命中可直接与枚举比较', () => {
  const hit = chart.patterns().find((h) => h.key === PatternKey.FU_XIANG_CHAO_YUAN)!;
  assert.ok(hit.is(PatternKey.FU_XIANG_CHAO_YUAN));
  assert.ok(hit.is('fu_xiang_chao_yuan'));
  assert.ok(!hit.is(PatternKey.ZI_FU_TONG_GONG));
  assert.ok(hit.inPalace(PalaceName.SOUL));
  assert.ok(hit.inPalace(hit.palaceName));
  assert.equal(hit.scope, Scope.ORIGIN);
  assert.ok(hit.stars.every((s) => s.palaceIndex >= 0 && s.palaceIndex < 12));
});

test('运限层由 scope 选定，未知 scope 报错', () => {
  const h = chart.horoscope('2026-8-19', 3);
  assert.ok(h.patterns(Scope.DECADAL).every((x) => x.scope === Scope.DECADAL));
  assert.ok(h.patterns(Scope.YEARLY).every((x) => x.scope === Scope.YEARLY));
  assert.deepEqual(h.patterns(Scope.ORIGIN).map((x) => x.toJSON()), chart.patterns().map((x) => x.toJSON()));
  assert.throws(() => h.patterns('nope' as Scope), (e: unknown) => e instanceof IztroError && /scope/.test(e.message));
});

test('口径经绑定层传到内核：位置法亮度改判日月类格局', () => {
  const a = astro.bySolar('1985-6-10', 8, 'female');
  const table = new Set(a.patterns().map((x) => x.key));
  const positional = new Set(a.patterns({ brightnessSource: BrightnessSource.POSITIONAL }).map((x) => x.key));
  assert.ok(!table.has(PatternKey.RI_YUE_FAN_BEI));
  assert.ok(positional.has(PatternKey.RI_YUE_FAN_BEI));
});

test('排盘配置随命中查询送回内核：中州派与默认派的命中不同', () => {
  const plain = astro.bySolar('1985-6-11', 11, 'male');
  const zhongzhou = astro.bySolar('1985-6-11', 11, 'male', { config: { algorithm: 'zhongzhou' } });
  assert.ok(plain.patterns().some((x) => x.key === PatternKey.SHENG_BU_FENG_SHI));
  assert.ok(!zhongzhou.patterns().some((x) => x.key === PatternKey.SHENG_BU_FENG_SHI));
});

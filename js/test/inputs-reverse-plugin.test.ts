/**
 * 日期写法与运限省略时辰（对照 JS iztro 金标 tests/golden/inputs.json）、反推的往返一致性、插件机制。
 * 与 Python 的 test_inputs_golden.py / test_reverse.py / test_plugin.py 同一批断言面。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import {
  astro,
  EarthlyBranch,
  FIVE_ELEMENTS_CLASS_NUMBER,
  HeavenlyStem,
  init,
  IztroError,
  MajorStar,
  MinorStar,
  PalaceName,
  reverseChart,
  solarDatesByBazi,
  util,
  type Astrolabe,
  type TimeIndex,
} from 'x-iztro';
import { golden, lineStartingWith } from './helpers.ts';

await init();

// ------ 日期写法与运限目标 ------

interface InputsGolden {
  solar: { input: string; solarDate?: string; error?: string }[];
  horoscope: {
    p: { d: string; t: TimeIndex };
    td: string;
    tt: TimeIndex | null;
    sd: string;
    error?: string;
    hr: { hs: string; eb: string };
    da: { hs: string; eb: string };
  }[];
}

const inputs = golden<InputsGolden>('inputs.json');

for (const c of inputs.solar) {
  test(`阳历日期写法 ${c.input}`, () => {
    if (c.error !== undefined) {
      assert.throws(() => astro.bySolar(c.input, 2, 'female'), IztroError);
      return;
    }
    assert.equal(astro.bySolar(c.input, 2, 'female').solarDate, c.solarDate);
  });
}

for (const c of inputs.horoscope.filter((h) => h.error === undefined)) {
  test(`运限目标时辰 ${c.td} ${c.tt}`, () => {
    const h = astro.bySolar(c.p.d, c.p.t, 'female').horoscope(c.td, c.tt ?? undefined);
    assert.equal(h.solarDate, c.sd);
    assert.equal(h.hourly.heavenlyStem + h.hourly.earthlyBranch, c.hr.hs + c.hr.eb);
    assert.equal(h.daily.heavenlyStem + h.daily.earthlyBranch, c.da.hs + c.da.eb);
    // 再发起计算转发原始目标参数：运限文本里流时一节的干支须与本次运限一致
    assert.ok(lineStartingWith(h.toText(), '## 流时').includes(`(${c.hr.hs}${c.hr.eb})`));
  });
}

test('运限拒绝非法目标', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  for (const c of inputs.horoscope.filter((h) => h.error !== undefined)) {
    assert.throws(() => a.horoscope(c.td, c.tt ?? undefined), IztroError, c.td);
  }
});

// ------ 反推 ------

test('八字往返', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  const p = a.rawDates.chineseDate;
  const got = solarDatesByBazi(p.yearlyKeys, p.monthlyKeys, p.dailyKeys, p.hourlyKeys);
  assert.ok(got.some((c) => c.solarDate === '2000-8-16' && c.timeIndex === 2));
  assert.equal(got.length, 3);
});

test('八字阴阳不配报错', () => {
  const jiaZi = [HeavenlyStem.JIA, EarthlyBranch.ZI] as const;
  assert.throws(() => solarDatesByBazi([HeavenlyStem.JIA, EarthlyBranch.CHOU], jiaZi, jiaZi, jiaZi), /polarity/);
});

test('星盘特征往返', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  const ziwei = a.starInPalace(MajorStar.ZIWEI)!;
  const mutagens = util.getMutagensByHeavenlyStem(a.rawDates.chineseDate.yearlyKeys[0]);
  const r = reverseChart({
    soulBranch: a.earthlyBranchOfSoulPalaceKey,
    bodyBranch: a.earthlyBranchOfBodyPalaceKey,
    fiveElementsClass: a.fiveElementsClassKey,
    stars: [
      { star: MajorStar.ZIWEI, branch: ziwei[1].earthlyBranchKey },
      { star: MinorStar.LUCUN, branch: a.starInPalace(MinorStar.LUCUN)![1].earthlyBranchKey },
    ],
    mutagens: [mutagens[0]!, null, null, mutagens[3]!],
    yearRange: [1999, 2001],
  });
  assert.ok(r.candidates.some((c) => c.solarDate === '2000-8-16' && c.timeIndex === 2));
  assert.equal(r.truncated, false);
  for (const c of r.candidates.slice(0, 5)) {
    const b = astro.bySolar(c.solarDate, c.timeIndex, 'female');
    assert.equal(b.earthlyBranchOfSoulPalaceKey, a.earthlyBranchOfSoulPalaceKey);
    assert.equal(b.earthlyBranchOfBodyPalaceKey, a.earthlyBranchOfBodyPalaceKey);
    const bm = util.getMutagensByHeavenlyStem(b.rawDates.chineseDate.yearlyKeys[0]);
    assert.deepEqual([bm[0], bm[3]], [mutagens[0], mutagens[3]]);
    assert.equal(b.starInPalace(MajorStar.ZIWEI)![1].earthlyBranchKey, ziwei[1].earthlyBranchKey);
  }
});

test('反推的截断与报错', () => {
  const r = reverseChart({ stars: [{ star: MinorStar.HUOXING, branch: EarthlyBranch.YIN }], limit: 10 });
  assert.equal(r.candidates.length, 10);
  assert.equal(r.truncated, true);
  assert.throws(() => reverseChart({}), IztroError);
  assert.throws(() => reverseChart({ stars: [{ star: 'liulu', branch: EarthlyBranch.ZI }] }), IztroError);
});

// ------ 插件：与 tests/extension.rs、Python test_plugin.py、Go plugin_test.go 是同一个插件 ------

declare module 'x-iztro' {
  interface Astrolabe {
    /** 命宫主星（空宫借对宫） */
    majorStar(): string;
    /** 五行局的局数 */
    fiveElementsValue(): number;
    /** 宫位数 */
    palaceCount(): number;
  }
}

const before = astro.bySolar('2000-8-16', 2, 'female');

astro.loadPlugins([
  (cls) => {
    cls.prototype.majorStar = function (this: Astrolabe): string {
      const soul = this.palace(PalaceName.SOUL)!;
      const source = soul.isEmpty() ? soul.oppositePalace()! : soul;
      return source.majorStars.map((s) => s.name).join(',');
    };
  },
  (cls) => {
    cls.prototype.fiveElementsValue = function (this: Astrolabe): number {
      return FIVE_ELEMENTS_CLASS_NUMBER[this.fiveElementsClassKey];
    };
  },
]);

test('插件方法挂上所有星盘', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  assert.equal(a.majorStar(), '紫微');
  assert.equal(a.fiveElementsValue(), 3);
});

test('插件输出随盘语言', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female', { language: 'en-US' });
  assert.equal(a.majorStar(), 'emperor');
  assert.equal(a.fiveElementsValue(), 3);
});

test('加载插件之前排好的盘也拿得到新方法', () => {
  astro.loadPlugin((cls) => {
    cls.prototype.palaceCount = function (this: Astrolabe): number {
      return this.palaces.length;
    };
  });
  assert.equal(before.palaceCount(), 12);
});

test('插件不是函数即报 TypeError', () => {
  assert.throws(() => astro.loadPlugin('not a plugin' as unknown as () => void), TypeError);
});

/**
 * 夹宫、大限 / 流年 / 流月列表与按农历年、农历月取运限。
 *
 * 这些查询都经内核无状态再发起排盘，因此同时盯两件事：返回值符合契约，以及排盘上下文（重排起点、修正闰月开关）
 * 真的被转发。列表部分对照 JS iztro 生成的金标 tests/golden/chart_lists.json（Rust、Python 读同一份）。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import {
  astro,
  DecadalListItem,
  EarthlyBranch,
  FlankingPalaces,
  HeavenlyStem,
  init,
  IztroError,
  KnowledgePack,
  MonthlyListItem,
  MonthPart,
  MUTAGEN_ORDER,
  Mutagen,
  PalaceName,
  Scope,
  YearlyListItem,
  type Gender,
  type SolarOptions,
  type TimeIndex,
} from 'x-iztro';
import { golden } from './helpers.ts';

await init();

/** 闰四月年，用来分辨修正闰月开关是否随查询转发 */
const LEAP_YEAR = 2001;
const chart = (options: SolarOptions = {}) => astro.bySolar('2000-8-16', 2, 'female', options);
const isInvalidArgument = (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument';

// ------ 对照 JS iztro 金标 ------

interface ListCase {
  birth: { d: string; t: TimeIndex; g: string };
  flanking: unknown[];
  decadals: unknown[];
  yearly: Record<string, unknown[]>;
  monthly: Record<string, unknown[]>;
}

for (const c of golden<ListCase[]>('chart_lists.json')) {
  const gender: Gender = c.birth.g === '男' ? 'male' : 'female';
  const a = astro.bySolar(c.birth.d, c.birth.t, gender);
  const id = `${c.birth.d}-t${c.birth.t}-${c.birth.g}`;

  test(`夹宫与 JS 一致 ${id}`, () => {
    const got = Array.from({ length: 12 }, (_, i) => {
      const f = a.flankingPalaces(i)!;
      return { index: i, previous: { index: f.previous.index, name: f.previous.name }, next: { index: f.next.index, name: f.next.name } };
    });
    assert.deepEqual(got, c.flanking);
  });

  test(`大限列表与 JS 一致 ${id}`, () => {
    const got = a.decadalList().map((d) => ({
      index: d.index,
      name: d.name,
      palaceName: d.palaceName,
      ageRange: [...d.ageRange],
      yearRange: [...d.yearRange],
      heavenlyStem: d.heavenlyStem,
      earthlyBranch: d.earthlyBranch,
      palaceNames: [...d.palaceNames],
      mutagen: [...d.mutagen],
    }));
    assert.deepEqual(got, c.decadals);
  });

  test(`流年列表与 JS 一致 ${id}`, () => {
    for (const [ordinal, want] of Object.entries(c.yearly)) {
      const got = a.yearlyList(Number(ordinal)).map((y) => ({
        index: y.index,
        age: y.age,
        year: y.year,
        heavenlyStem: y.heavenlyStem,
        earthlyBranch: y.earthlyBranch,
        palaceNames: [...y.palaceNames],
        mutagen: [...y.mutagen],
      }));
      assert.deepEqual(got, want, `大限#${ordinal}`);
    }
  });

  test(`流月列表与 JS 一致 ${id}`, () => {
    // 金标键形如 "2020_true"：年份 + monthlyList 的 fixLeap 入参（与排盘的同名开关无关）
    for (const [key, want] of Object.entries(c.monthly)) {
      const [year, fixLeap] = key.split('_');
      const got = a.monthlyList(Number(year), fixLeap === 'true').map((m) => ({
        index: m.index,
        age: m.age,
        year: m.year,
        month: m.month,
        isLeapMonth: m.isLeapMonth,
        part: m.part,
        dayRange: [...m.dayRange],
        heavenlyStem: m.heavenlyStem,
        earthlyBranch: m.earthlyBranch,
        mutagen: [...m.mutagen],
      }));
      assert.deepEqual(got, want, key);
    }
  });
}

// ------ 夹宫 ------

test('夹宫是前后相邻两宫', () => {
  const a = chart();
  for (const p of a.palaces) {
    const f = a.flankingPalaces(p.index)!;
    assert.ok(f instanceof FlankingPalaces);
    assert.equal(f.previous.index, (p.index + 11) % 12);
    assert.equal(f.next.index, (p.index + 1) % 12);
  }
});

test('夹宫接受各种定位方式', () => {
  const a = chart();
  const soul = a.palace(PalaceName.SOUL)!;
  for (const target of [PalaceName.SOUL, soul.name, PalaceName.BODY]) assert.ok(a.flankingPalaces(target) !== undefined);
  const indexes = (f: FlankingPalaces) => [f.previous.index, f.next.index];
  assert.deepEqual(indexes(a.flankingPalaces(PalaceName.SOUL)!), indexes(a.flankingPalaces(soul.index)!));
  assert.equal(a.flankingPalaces('不是宫名'), undefined);
});

test('夹宫挂回本盘', () => {
  const a = chart();
  const f = a.flankingPalaces(PalaceName.SOUL)!;
  assert.equal(f.astrolabe(), a);
  assert.equal(f.next.astrolabe(), a);
  assert.ok(f.previous.toText());
});

test('夹宫的星耀判断', () => {
  const f = chart().flankingPalaces(PalaceName.SOUL)!;
  const stars = [f.previous, f.next].flatMap((p) => [...p.majorStars, ...p.minorStars].map((s) => s.key));
  assert.ok(stars.length > 0);
  assert.ok(f.have(stars));
  assert.ok(f.haveOneOf([stars[0]!, 'wenquMin']));
  assert.ok(!f.notHave([stars[0]!]));
  assert.ok(f.notHave(['不存在的星']));
});

test('夹宫的四化判断', () => {
  const a = chart();
  let hits = 0;
  for (const p of a.palaces) {
    const f = a.flankingPalaces(p.index)!;
    for (const m of MUTAGEN_ORDER) {
      const present = f.previous.hasMutagen(m) || f.next.hasMutagen(m);
      assert.equal(f.haveMutagen(m), present);
      assert.equal(f.notHaveMutagen(m), !present);
      if (present) hits += 1;
    }
  }
  assert.equal(hits, 8);
});

test('夹宫按重排后的布局取', () => {
  const a = chart();
  const r = a.rearranged(HeavenlyStem.JIA, EarthlyBranch.ZI);
  const f = r.flankingPalaces(PalaceName.SOUL)!;
  assert.equal(f.previous.nameKey, r.palaces[f.previous.index]!.nameKey);
  assert.notEqual(f.previous.nameKey, a.palaces[f.previous.index]!.nameKey);
});

// ------ 大限列表 ------

test('大限列表按起运先后覆盖十二宫', () => {
  const a = chart();
  const items = a.decadalList();
  assert.equal(items.length, 12);
  assert.ok(items.every((d) => d instanceof DecadalListItem));
  assert.deepEqual(
    items.map((d) => d.index),
    a.palaces.map((p) => p.index).sort((x, y) => a.palaces[x]!.decadal.range[0] - a.palaces[y]!.decadal.range[0]),
  );
  items.forEach((d, i) => {
    assert.equal(d.nameKey, 'decadal');
    assert.equal(d.palaceNameKey, a.palaces[d.index]!.nameKey);
    assert.equal(d.palaceName, a.palaces[d.index]!.name);
    assert.deepEqual([...d.ageRange], [...a.palaces[d.index]!.decadal.range]);
    assert.equal(d.yearRange[1] - d.yearRange[0], 9);
    assert.equal(d.palaceNames.length, 12);
    assert.equal(d.palaceNameKeys.length, 12);
    assert.equal(d.mutagen.length, 4);
    assert.equal(d.mutagenStarKeys.length, 4);
    if (i > 0) {
      assert.equal(d.ageRange[0], items[i - 1]!.ageRange[1] + 1);
      assert.equal(d.yearRange[0], items[i - 1]!.yearRange[1] + 1);
    }
  });
});

test('大限列表按重排后的布局', () => {
  const a = chart();
  const r = a.rearranged(HeavenlyStem.JIA, EarthlyBranch.ZI);
  assert.notDeepEqual(r.decadalList().map((d) => d.index), a.decadalList().map((d) => d.index));
});

// ------ 流年列表 ------

test('流年列表覆盖整个大限', () => {
  const a = chart();
  const decadal = a.decadalList()[0]!;
  const items = a.yearlyList(0);
  assert.equal(items.length, 10);
  assert.ok(items.every((y) => y instanceof YearlyListItem));
  const range = (from: number, to: number) => Array.from({ length: to - from + 1 }, (_, i) => from + i);
  assert.deepEqual(items.map((y) => y.age), range(...decadal.ageRange));
  assert.deepEqual(items.map((y) => y.year), range(...decadal.yearRange));
  assert.ok(items.every((y) => y.nameKey === 'yearly'));
});

test('流年列表两种定位方式一致', () => {
  const a = chart();
  a.decadalList().forEach((d, ordinal) => {
    assert.deepEqual(a.yearlyList(ordinal), a.yearlyList(d.palaceNameKey));
  });
});

test('流年列表必须给定位', () => {
  const a = chart();
  assert.throws(() => a.yearlyList(), isInvalidArgument);
  assert.throws(() => a.yearlyList(12), IztroError);
  assert.throws(() => a.yearlyList('不是宫名' as PalaceName), IztroError);
});

// ------ 流月列表 ------

test('无闰月的年份 12 个流月', () => {
  const items = chart().monthlyList(2000);
  assert.equal(items.length, 12);
  assert.ok(items.every((m) => m instanceof MonthlyListItem));
  assert.deepEqual(items.map((m) => m.month), Array.from({ length: 12 }, (_, i) => i + 1));
  assert.ok(items.every((m) => m.part === MonthPart.NORMAL && !m.isLeapMonth));
  assert.ok(items.every((m) => m.dayRange[0] === 1 && (m.dayRange[1] === 29 || m.dayRange[1] === 30)));
  assert.ok(items.every((m) => m.year === 2000 && m.age === 1 && m.nameKey === 'monthly'));
});

for (const [fixLeap, count, parts] of [
  [true, 14, [MonthPart.FIRST, MonthPart.SECOND]],
  [false, 13, [MonthPart.NORMAL]],
] as const) {
  test(`闰月按 fixLeap=${fixLeap} 拆段`, () => {
    const items = chart().monthlyList(LEAP_YEAR, fixLeap);
    assert.equal(items.length, count);
    const leap = items.filter((m) => m.isLeapMonth);
    assert.deepEqual(leap.map((m) => m.part), [...parts]);
    assert.deepEqual(leap.map((m) => m.month), parts.map(() => 4));
    assert.equal(leap[0]!.dayRange[0], 1);
    assert.ok(leap.at(-1)!.dayRange[1] >= 29);
    assert.equal(items[items.indexOf(leap[0]!) - 1]!.month, 4);
  });
}

test('列表的 fixLeap 独立于排盘的同名开关', () => {
  for (const fixLeap of [true, false]) {
    const a = chart({ fixLeap });
    assert.equal(a.monthlyList(LEAP_YEAR).length, 14);
    assert.equal(a.monthlyList(LEAP_YEAR, false).length, 13);
  }
});

test('流月列表拒绝不支持的年份', () => {
  assert.throws(() => chart().monthlyList(99999), IztroError);
  assert.throws(() => chart().monthlyListToText(99999), isInvalidArgument);
});

// ------ 按农历年取运限 ------

test('按年取运限是农历年，不是公历元旦', () => {
  const a = chart();
  const ofYear = a.horoscopeOfYear(2027);
  assert.deepEqual([ofYear.yearly.heavenlyStemKey, ofYear.yearly.earthlyBranchKey], [HeavenlyStem.DING, EarthlyBranch.WEI]);
  const newYear = a.horoscope('2027-1-1', 0);
  assert.deepEqual([newYear.yearly.heavenlyStemKey, newYear.yearly.earthlyBranchKey], [HeavenlyStem.BING, EarthlyBranch.WU]);
  for (const year of [0, 10000]) assert.throws(() => a.horoscopeOfYear(year), isInvalidArgument);
});

test('按年取运限再发起时转发年份，查询精度为流年', () => {
  const a = chart();
  const ofYear = a.horoscopeOfYear(2027);
  assert.equal(ofYear.precision(), Scope.YEARLY);
  const text = ofYear.toText();
  assert.ok(text.includes('## 流年 · '));
  assert.ok(!text.includes('## 流月 · '));
  assert.equal(text, a.horoscope(ofYear.solarDate, 2).toText({ through: Scope.YEARLY }));
  assert.ok(ofYear.palace(PalaceName.SOUL, Scope.YEARLY) !== undefined);
  const messages: string[] = [];
  for (const call of [
    () => ofYear.toText({ through: Scope.MONTHLY }),
    () => ofYear.patterns(Scope.MONTHLY),
    () => ofYear.palace(PalaceName.SOUL, Scope.MONTHLY),
    () => ofYear.hasHoroscopeMutagen(PalaceName.SOUL, Scope.DAILY, Mutagen.LU),
  ]) {
    assert.throws(call, (e: unknown) => {
      assert.ok(isInvalidArgument(e));
      messages.push((e as IztroError).message);
      return true;
    });
  }
  assert.equal(messages[2], messages[1]);
});

test('流年一览拒绝颠倒或越界的区间', () => {
  for (const [from, to] of [[2012, 2001], [0, 1], [9999, 10000]] as const) {
    assert.throws(() => chart().yearlyOverviewToText(from, to), isInvalidArgument, `${from}-${to}`);
  }
});

// ------ 按农历月取运限 ------

test('按月取运限与流月列表同月号的常规月一致', () => {
  const a = chart();
  const listed = new Map(a.monthlyList(2025).filter((m) => !m.isLeapMonth).map((m) => [m.month, m]));
  for (let month = 1; month <= 12; month++) {
    const m = a.horoscopeOfMonth(2025, month).monthly;
    const want = listed.get(month)!;
    assert.deepEqual(
      [m.index, m.heavenlyStemKey, m.earthlyBranchKey, [...m.mutagenStarKeys]],
      [want.index, want.heavenlyStemKey, want.earthlyBranchKey, [...want.mutagenStarKeys]],
      String(month),
    );
  }
  for (const [year, month] of [[2025, 0], [2025, 13], [2025, -1], [0, 1], [10000, 1]] as const) {
    assert.throws(() => a.horoscopeOfMonth(year, month), isInvalidArgument, `${year}-${month}`);
  }
});

test('按月取运限再发起时转发年月，查询精度为流月（农历 2025 年七月初一是 2025-8-23）', () => {
  const a = chart();
  const ofMonth = a.horoscopeOfMonth(2025, 7);
  assert.equal(ofMonth.solarDate, '2025-8-23');
  assert.equal(ofMonth.precision(), Scope.MONTHLY);
  const sameHour = a.horoscope('2025-8-23', 2);
  const text = ofMonth.toText();
  assert.equal(text, sameHour.toText({ through: Scope.MONTHLY }));
  assert.ok(text.includes('## 流月 · '));
  for (const heading of ['## 流日 · ', '## 流时 · ']) assert.ok(!text.includes(heading));
  assert.deepEqual(ofMonth.patterns(Scope.MONTHLY).map((x) => x.toJSON()), sameHour.patterns(Scope.MONTHLY).map((x) => x.toJSON()));
  assert.equal(ofMonth.patternsToText(Scope.MONTHLY), sameHour.patternsToText(Scope.MONTHLY));
  const stars = KnowledgePack.builtin().forHoroscope(ofMonth).toJSON().stars as Record<string, unknown>;
  const keysOf = (groups: readonly (readonly { key: string }[])[] | undefined) => new Set((groups ?? []).flat().map((s) => s.key));
  const monthly = keysOf(ofMonth.monthly.stars);
  const daily = keysOf(ofMonth.daily.stars);
  assert.ok(monthly.size > 0 && daily.size > 0);
  assert.ok([...monthly].every((k) => k in stars));
  assert.ok(![...daily].some((k) => k in stars));
});

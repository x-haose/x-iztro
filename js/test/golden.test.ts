/**
 * 端到端金标：对照 tests/golden/ 下由 JS iztro 生成的数据，验证从 WASM 内核到 JS 对象的完整链路
 * （排盘、运限、配置、方法层）。Python 的 test_golden.py 读同一批数据。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import {
  Algorithm,
  astro,
  init,
  MajorStar,
  Mutagen,
  MUTAGEN_ORDER,
  PalaceName,
  Suiqian12,
  YearDivide,
  type Gender,
  type TimeIndex,
} from 'x-iztro';
import { golden, sample } from './helpers.ts';

await init();

interface Tier1Star {
  name: string;
}
interface Tier1Palace {
  name: string;
  heavenly_stem: string;
  earthly_branch: string;
  is_body_palace: boolean;
  major_stars: Tier1Star[];
  minor_stars: Tier1Star[];
  adjective_stars: Tier1Star[];
  changsheng12: string;
  boshi12: string;
  decadal_range: number[];
  ages: number[];
}
interface Tier1Case {
  params: { solar_date: string; time_index: TimeIndex; gender: string };
  soul_star: string;
  body_star: string;
  five_elements_class: string;
  lunar_date: string;
  chinese_date: string;
  time: string;
  sign: string;
  zodiac: string;
  palaces: Tier1Palace[];
}

const tier1 = sample(golden<Tier1Case[]>('tier1_data.json'), 60);
const genderOf = (g: string): Gender => (g === '男' ? 'male' : 'female');

test('抽样跨越时辰与性别两个维度', () => {
  assert.ok(new Set(tier1.map((c) => c.params.time_index)).size >= 10);
  assert.equal(new Set(tier1.map((c) => c.params.gender)).size, 2);
});

for (const c of tier1) {
  const p = c.params;
  test(`tier1 ${p.solar_date} t${p.time_index} ${p.gender}`, () => {
    const a = astro.bySolar(p.solar_date, p.time_index, genderOf(p.gender));
    assert.equal(a.soul, c.soul_star);
    assert.equal(a.body, c.body_star);
    assert.equal(a.fiveElementsClass, c.five_elements_class);
    assert.equal(a.lunarDate, c.lunar_date);
    assert.equal(a.chineseDate, c.chinese_date);
    assert.equal(a.time, c.time);
    assert.equal(a.sign, c.sign);
    assert.equal(a.zodiac, c.zodiac);
    a.palaces.forEach((palace, i) => {
      const exp = c.palaces[i] as Tier1Palace;
      const names = (stars: readonly { name: string }[]) => stars.map((s) => s.name);
      assert.equal(palace.name, exp.name);
      assert.equal(palace.heavenlyStem, exp.heavenly_stem);
      assert.equal(palace.earthlyBranch, exp.earthly_branch);
      assert.equal(palace.isBodyPalace, exp.is_body_palace);
      assert.deepEqual(names(palace.majorStars), names(exp.major_stars));
      assert.deepEqual(new Set(names(palace.minorStars)), new Set(names(exp.minor_stars)));
      assert.deepEqual(new Set(names(palace.adjectiveStars)), new Set(names(exp.adjective_stars)));
      assert.equal(palace.changsheng12, exp.changsheng12);
      assert.equal(palace.boshi12, exp.boshi12);
      assert.deepEqual([...palace.decadal.range], exp.decadal_range);
      assert.deepEqual([...palace.ages], exp.ages);
    });
  });
}

interface LayerCase {
  i: number;
  n: string;
  hs: string;
  eb: string;
  m: string[];
  s?: string[][];
}
interface HoroscopeCase {
  p: { d: string; t: TimeIndex; g: number };
  td: string;
  tt: TimeIndex | null;
  ld: string;
  dec: LayerCase;
  age: LayerCase & { na: number };
  yr: LayerCase & { sq: string[]; jq: string[] };
  mo: LayerCase;
  da: LayerCase;
  hr: LayerCase;
}

for (const c of sample(golden<HoroscopeCase[]>('horoscope_data.json'), 40)) {
  test(`运限 ${c.p.d} → ${c.td}`, () => {
    const a = astro.bySolar(c.p.d, c.p.t, c.p.g === 0 ? 'male' : 'female');
    const h = a.horoscope(c.td, c.tt ?? undefined);
    assert.equal(h.lunarDate, c.ld);
    const layers = [
      ['dec', h.decadal],
      ['age', h.age],
      ['yr', h.yearly],
      ['mo', h.monthly],
      ['da', h.daily],
      ['hr', h.hourly],
    ] as const;
    for (const [key, item] of layers) {
      const exp = c[key];
      assert.equal(item.index, exp.i, key);
      assert.equal(item.name, exp.n, key);
      assert.equal(item.heavenlyStem, exp.hs, key);
      assert.equal(item.earthlyBranch, exp.eb, key);
      assert.deepEqual([...item.mutagen], exp.m, key);
      if (exp.s !== undefined) {
        assert.ok(item.stars !== undefined, key);
        exp.s.forEach((group, pi) => {
          assert.deepEqual(item.stars?.[pi]?.map((s) => s.name).sort(), group, `${key} p${pi}`);
        });
      }
    }
    assert.equal(h.age.nominalAge, c.age.na);
    assert.deepEqual([...h.yearly.yearlyDecStar.suiqian12], c.yr.sq);
    assert.deepEqual([...h.yearly.yearlyDecStar.jiangqian12], c.yr.jq);
  });
}

test('中州派配置生效：岁前十二神含岁破', () => {
  const a = astro.bySolar('1990-11-5', 4, 'male', { config: { algorithm: Algorithm.ZHONGZHOU } });
  assert.equal(a.config.algorithm, Algorithm.ZHONGZHOU);
  assert.ok(a.palaces.some((p) => p.suiqian12 === '岁破'));
});

test('农历入口与阳历入口指向同一天时结果一致', () => {
  const solar = astro.bySolar('2000-8-16', 2, 'female');
  const lunar = astro.byLunar('2000-7-17', 2, 'female');
  assert.equal(lunar.solarDate, solar.solarDate);
  assert.equal(lunar.chineseDate, solar.chineseDate);
  assert.deepEqual(
    lunar.palaces.map((p) => p.name),
    solar.palaces.map((p) => p.name),
  );
});

test('宫位查询与飞星方法链路', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  const soul = a.palace('命宫');
  assert.ok(soul !== undefined);
  assert.ok(soul.has(soul.majorStars.map((s) => s.name)));
  assert.equal(a.surroundedPalaces(soul.index)?.target.index, soul.index);
  const places = soul.mutagedPlaces(a.palaces);
  assert.equal(places.length, 4);
  assert.ok(places.every((p) => p !== undefined));
});

test('星盘与运限的语义化文本是 Markdown 子集', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female');
  const natal = a.toText();
  assert.ok(natal.startsWith('# 命盘 2000-8-16 寅时 女\n'));
  assert.ok(natal.includes('\n## 基本信息\n') && natal.includes('\n## 十二宫\n'));
  const fortune = a.horoscope('2024-1-1', 0).toText();
  assert.ok(fortune.startsWith('# 运限 2024-1-1 (') && fortune.includes('\n## 流年 · 命宫: 本命'));
});

test('枚举基于语言无关标识：任何输出语言的星盘上判断一致', () => {
  const charts = Object.fromEntries(
    (['zh-CN', 'en-US', 'ja-JP'] as const).map((lang) => [lang, astro.bySolar('2000-8-16', 2, 'female', { language: lang })]),
  );
  for (const [lang, a] of Object.entries(charts)) {
    const soul = a.palace(PalaceName.SOUL);
    assert.ok(soul !== undefined, lang);
    assert.equal(soul.nameKey, PalaceName.SOUL, lang);
    assert.ok(soul.has([MajorStar.ZIWEI]), lang);
    assert.equal(a.soulKey, MajorStar.POJUN, lang);
    assert.ok(a.starInPalace(MajorStar.WUQU)?.[0].withMutagen(Mutagen.QUAN), lang);
  }
  const zh = charts['zh-CN'];
  const en = charts['en-US'];
  assert.ok(zh !== undefined && en !== undefined);
  for (const m of MUTAGEN_ORDER) {
    assert.deepEqual(
      zh.palaces.map((p) => p.hasMutagen(m)),
      en.palaces.map((p) => p.hasMutagen(m)),
    );
  }
});

test('四化飞星基于天干标识表，非中文星盘同样正确', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female', { language: 'en-US' });
  const soul = a.palace(0);
  assert.ok(soul !== undefined);
  const places = soul.mutagedPlaces(a.palaces);
  assert.equal(places.length, 4);
  assert.ok(places.every((p) => p !== undefined));
  assert.ok(a.palaces.some((t) => soul.fliesTo(t, Mutagen.LU)));
});

test('排盘配置', () => {
  const a = astro.bySolar('1990-11-5', 4, 'male', {
    config: { algorithm: Algorithm.ZHONGZHOU, yearDivide: YearDivide.EXACT },
  });
  assert.equal(a.config.algorithm, Algorithm.ZHONGZHOU);
  assert.equal(a.config.yearDivide, YearDivide.EXACT);
  assert.ok(a.palaces.some((p) => p.suiqian12Key === Suiqian12.SUIPO));
});

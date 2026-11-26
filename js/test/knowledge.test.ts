/** 知识包：默认包可取、查询有类型、合并走内核且语义与 Rust 一致，释义融入文本与 Rust 快照逐字节相同 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import {
  astro,
  BrightnessSource,
  init,
  IztroError,
  KnowledgeDepth,
  KnowledgePack,
  MajorStar,
  Mutagen,
  PalaceName,
  PatternKey,
  Scope,
  TextDetail,
  type Astrolabe,
  type Horoscope,
  type KnowledgeOptions,
  type Knowledge,
} from 'x-iztro';
import { snapshot } from './helpers.ts';

await init();

const pack = KnowledgePack.builtin();
/** 与 tests/text_snapshot.rs 同一张盘与运限目标 */
const chart = astro.bySolar('2000-8-16', 2, 'female', { fixLeap: true });
const horoscope = chart.horoscope('2025-1-1', 0);

/** 九类语义化文本，键与快照文件名前缀一致；流年一览取农历 2001-2012 年，流月一览取农历 2025 年 */
function texts(a: Astrolabe, h: Horoscope, options: KnowledgeOptions = {}): Record<string, string> {
  const soul = a.palace(PalaceName.SOUL)!;
  return {
    astrolabe: a.toText(options),
    horoscope: h.toText(options),
    patterns: a.patternsToText(options),
    palace: soul.toText(options),
    surrounded: soul.surroundedPalaces()!.toText(options),
    flanking: a.flankingPalaces(PalaceName.SOUL)!.toText(options),
    decadals: a.decadalListToText(options),
    yearlies: a.yearlyOverviewToText(2001, 2012, options),
    monthlies: a.monthlyListToText(2025, options),
  };
}

/** 释义穿插在各宫事实之后，故不带释义文本的每一行须按原顺序出现在带释义文本里 */
function keepsFacts(plain: string, explained: string): boolean {
  const lines = explained.split('\n');
  let at = 0;
  return plain.split('\n').every((line) => {
    const found = lines.indexOf(line, at);
    if (found < 0) return false;
    at = found + 1;
    return true;
  });
}

test('默认包的元信息与条目', () => {
  assert.equal(pack.schema, 1);
  assert.equal(pack.id, 'iztro-docs');
  assert.equal(pack.language, 'zh-CN');
  assert.equal(pack.source.license, 'MIT');
  const ziwei = pack.star(MajorStar.ZIWEI)!;
  assert.equal(ziwei.name, '紫微');
  assert.equal(ziwei.category, 'major');
  assert.equal(ziwei.attributes.yinYang, 'yin');
  assert.equal(ziwei.attributes.fiveElements, 'earth');
  assert.ok(ziwei.attributes.aliases?.includes('帝王星'));
  assert.ok(MajorStar.TIANFU in ziwei.combinations);
  assert.ok(ziwei.summary && ziwei.combinationSummaries[MajorStar.TIANFU]);
  assert.ok(pack.pattern(PatternKey.ZI_FU_TONG_GONG)?.summary && pack.mutagen(Mutagen.JI)?.summary);
  assert.equal(pack.pattern(PatternKey.ZI_FU_TONG_GONG)?.name, '紫府同宫');
  assert.ok(pack.patternIntro(PatternKey.ZI_FU_TONG_GONG));
  assert.equal(pack.palace(PalaceName.SOUL)?.name, '命宫');
  assert.ok(pack.mutagen(Mutagen.JI)?.intro);
  assert.ok(pack.concept('tong-gong') !== undefined);
  assert.equal(Object.keys(pack.toJSON().concepts as object).length, 51);
  assert.equal(pack.patterns().length, 64);
  assert.equal(pack.star('nope'), undefined);
  assert.throws(() => KnowledgePack.builtin('en-US'), IztroError);
});

test('按键查询只认包自己的键，原型链上的名字查不到', () => {
  for (const name of ['constructor', 'toString', 'hasOwnProperty', '__proto__']) {
    assert.equal(pack.star(name), undefined, name);
    assert.equal(pack.pattern(name), undefined, name);
    assert.equal(pack.palace(name), undefined, name);
    assert.equal(pack.mutagen(name), undefined, name);
    assert.equal(pack.concept(name), undefined, name);
    assert.equal(pack.topic(name), undefined, name);
    assert.equal(pack.starIntro(name), undefined, name);
  }
});

test('本命格局命中都能在包里查到释义', () => {
  for (const hit of chart.patterns()) assert.ok(pack.pattern(hit.key) !== undefined, hit.key);
});

test('合并只覆盖给出的字段', () => {
  const overlay = KnowledgePack.fromObject({
    schema: 1,
    id: 'mine',
    version: '1',
    language: 'zh-CN',
    extends: 'iztro-docs',
    stars: { ziweiMaj: { intro: '我的紫微', attributes: { aliases: ['我的别号'] } } },
    patterns: { zi_fu_tong_gong: { intro: '我的紫府同宫' } },
  });
  const merged = pack.merged(overlay);
  assert.equal(merged.id, 'mine');
  const ziwei = merged.star(MajorStar.ZIWEI)!;
  assert.equal(ziwei.intro, '我的紫微');
  assert.equal(ziwei.name, '紫微');
  assert.deepEqual(ziwei.attributes.aliases, ['我的别号']);
  assert.equal(ziwei.attributes.chemistry, pack.star(MajorStar.ZIWEI)!.attributes.chemistry);
  assert.equal(merged.patternIntro(PatternKey.ZI_FU_TONG_GONG), '我的紫府同宫');
  assert.deepEqual(merged.pattern(PatternKey.ZI_FU_TONG_GONG)?.quotes, pack.pattern(PatternKey.ZI_FU_TONG_GONG)?.quotes);
  assert.notEqual(pack.starIntro(MajorStar.ZIWEI), '我的紫微');
  assert.deepEqual(pack.merged(overlay.toJSON()).toJSON(), merged.toJSON());
  assert.deepEqual(KnowledgePack.fromJSON(JSON.stringify(merged)).toJSON(), merged.toJSON());
  assert.throws(() => pack.merged({ schema: 99 }), IztroError);
});

test('包对象的格式校验与内核同语义', () => {
  for (const bad of [[], 'x', {}, { schema: 0 }, { schema: true }, { schema: 2 }]) {
    assert.throws(() => KnowledgePack.fromObject(bad as object), (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument');
  }
  assert.throws(() => KnowledgePack.fromJSON('{'), (e: unknown) => e instanceof IztroError && e.issues?.[0]?.code === 'invalid_json');
});

test('knowledge: true 的九类文本与 Rust 快照逐字节相同', () => {
  for (const [kind, text] of Object.entries(texts(chart, horoscope, { knowledge: true }))) {
    assert.equal(text, snapshot(`${kind}_knowledge_zh-CN`), kind);
  }
});

test('摘要层的九类文本与 Rust 快照逐字节相同', () => {
  for (const [kind, text] of Object.entries(texts(chart, horoscope, { knowledge: true, knowledgeDepth: KnowledgeDepth.SUMMARY }))) {
    assert.equal(text, snapshot(`${kind}_summary_zh-CN`), kind);
  }
});

test('摘要层要有包且长正文都有摘要，缺了即报错并列出缺项', () => {
  assert.throws(
    () => chart.toText({ knowledgeDepth: KnowledgeDepth.SUMMARY }),
    (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument',
  );
  const mine = pack.merged({ schema: 1, id: 'mine', version: '1', language: 'zh-CN', stars: { ziweiMaj: { intro: '长'.repeat(121) } } });
  assert.throws(() => chart.toText({ knowledge: mine, knowledgeDepth: KnowledgeDepth.SUMMARY }), /stars\.ziweiMaj/);
  assert.throws(() => chart.toText({ detail: 'brief' as TextDetail }), IztroError);
  assert.equal(chart.toText({ detail: TextDetail.FULL }), snapshot('astrolabe_zh-CN'));
});

test('主题文本与 Rust 快照逐字节相同，主题定义可从包上读到', () => {
  assert.equal(chart.topicToText('romance'), snapshot('topic_knowledge_zh-CN'));
  assert.equal(chart.topicToText('romance', { knowledgeDepth: KnowledgeDepth.SUMMARY }), snapshot('topic_summary_zh-CN'));
  assert.equal(chart.horoscope('2025-1-1', 0).topicToText('romance', 'monthly'), snapshot('topic_monthly_knowledge_zh-CN'));
  const romance = pack.topic('romance')!;
  assert.equal(romance.title, '婚恋');
  assert.deepEqual(romance.surround, [PalaceName.SPOUSE]);
  const slugs = new Set(pack.topics().map((t) => t.slug));
  for (const s of ['overall', 'romance', 'wealth', 'career']) assert.ok(slugs.has(s));
  assert.throws(
    () => chart.topicToText('nope'),
    (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument' && e.message.includes('romance'),
  );
  assert.throws(() => chart.topicToText('romance', { knowledge: false }), IztroError);
});

test('传内嵌包实例与传 true 同解', () => {
  assert.deepEqual(texts(chart, horoscope, { knowledge: pack }), texts(chart, horoscope, { knowledge: true }));
});

test('不带释义的形态不受影响', () => {
  for (const options of [{}, { knowledge: null }, { knowledge: false }]) {
    for (const [kind, text] of Object.entries(texts(chart, horoscope, options))) {
      assert.equal(text, snapshot(`${kind}_zh-CN`), kind);
    }
  }
  assert.equal(String(chart), snapshot('astrolabe_zh-CN'));
  assert.equal(String(horoscope), snapshot('horoscope_zh-CN'));
});

test('本命子包只取盘上有的', () => {
  const sub = pack.forAstrolabe(chart);
  assert.ok(sub instanceof KnowledgePack);
  assert.equal(sub.id, pack.id);
  assert.equal(sub.language, pack.language);
  const onChart = new Set<string>();
  for (const p of chart.palaces) {
    for (const s of [...p.majorStars, ...p.minorStars, ...p.adjectiveStars]) onChart.add(s.key);
    for (const k of [p.changsheng12Key, p.boshi12Key, p.jiangqian12Key, p.suiqian12Key]) onChart.add(k);
  }
  const subKeys = new Set(sub.stars().map((e) => e.key));
  assert.ok([...subKeys].every((k) => onChart.has(k)));
  assert.deepEqual(new Set([...onChart].filter((k) => pack.star(k) !== undefined)), subKeys);
  assert.deepEqual(new Set(chart.patterns().map((h) => h.key)), new Set(sub.patterns().map((e) => e.key)));
  const raw = sub.toJSON();
  assert.equal(Object.keys(raw.mutagens as object).length, 4);
  const isEmpty = (v: unknown) => v === undefined || Object.keys(v as object).length === 0;
  assert.ok(isEmpty(raw.palaces) && isEmpty(raw.concepts));
  assert.deepEqual(KnowledgePack.fromJSON(JSON.stringify(sub)).toJSON(), raw);
  assert.equal(sub.merged({ schema: 1, id: 'x', version: '1', language: 'zh-CN' }).id, 'x');
});

test('运限子包在本命子包之上再加流耀与各层格局', () => {
  const natal = pack.forAstrolabe(chart);
  const flow = pack.forHoroscope(horoscope);
  const keys = (p: KnowledgePack) => new Set(p.stars().map((e) => e.key));
  assert.ok([...keys(natal)].every((k) => keys(flow).has(k)));
  const patternKeys = (p: KnowledgePack) => new Set(p.patterns().map((e) => e.key));
  assert.ok([...patternKeys(natal)].every((k) => patternKeys(flow).has(k)));
  assert.ok(flow.stars().some((e) => e.category === 'flow'));
  assert.equal(Object.keys(flow.toJSON().mutagens as object).length, 4);
});

test('子包格局按传入口径判定', () => {
  const a = astro.bySolar('1985-6-10', 8, 'female');
  const h = a.horoscope('2025-1-1', 0);
  const positional = { brightnessSource: BrightnessSource.POSITIONAL };
  const natalKeys = new Set(pack.forAstrolabe(a, positional).patterns().map((e) => e.key));
  assert.deepEqual(natalKeys, new Set(a.patterns(positional).map((x) => x.key)));
  assert.ok(natalKeys.has(PatternKey.RI_YUE_FAN_BEI));
  assert.ok(!pack.forAstrolabe(a).patterns().some((e) => e.key === PatternKey.RI_YUE_FAN_BEI));
  const expected = new Set(a.patterns(positional).map((x) => x.key));
  for (const scope of [Scope.DECADAL, Scope.YEARLY, Scope.MONTHLY, Scope.DAILY, Scope.HOURLY]) {
    for (const x of h.patterns(scope, positional)) expected.add(x.key);
  }
  assert.deepEqual(new Set(pack.forHoroscope(h, positional).patterns().map((e) => e.key)), expected);
});

test('英文盘没有内嵌包：true 报错不回退，传 zh-CN 包实例照常可用', () => {
  const en = astro.bySolar('2000-8-16', 2, 'female', { language: 'en-US' });
  assert.throws(() => en.toText({ knowledge: true }), (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument');
  const explained = en.toText({ knowledge: pack });
  assert.ok(explained.includes('**') && keepsFacts(en.toText(), explained));
  assert.ok(pack.forAstrolabe(en).stars().length > 0);
});

test('自定义包：释义用覆盖后的文本，不改盘面事实行；knowledge 类型不对报 TypeError', () => {
  const mine = pack.merged({
    schema: 1,
    id: 'mine',
    version: '1',
    language: 'zh-CN',
    extends: 'iztro-docs',
    stars: { ziweiMaj: { intro: '我的紫微释义' } },
  });
  const text = chart.toText({ knowledge: mine });
  assert.ok(text.includes('我的紫微释义'));
  assert.ok(keepsFacts(snapshot('astrolabe_zh-CN'), text));
  assert.ok(!chart.toText({ knowledge: true }).includes('我的紫微释义'));
  assert.throws(() => chart.toText({ knowledge: 'builtin' as unknown as Knowledge }), TypeError);
  assert.throws(() => chart.toText({ knowledge: pack.toJSON() as unknown as Knowledge }), TypeError);
});

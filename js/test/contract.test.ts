/**
 * 序列化契约与对象字段：对照 JS iztro 的 `JSON.stringify` 金标 tests/golden/contract_data.json（Rust 的
 * golden_contract 读同一份）。
 *
 * 两层对照：`toJSON()` 含金标里的每个键且值相同（多出的是 x-iztro 的扩展键）；星盘、运限对象自身的字段与
 * `toJSON()` 逐键逐值相同——对象由构造器逐字段从 DTO 拷出，拷漏、拷错在这里暴露。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import { astro, init, type Algorithm, type Language, type TimeIndex } from 'x-iztro';
import { golden } from './helpers.ts';

await init();

interface ContractCase {
  p: { d: string; t: TimeIndex; g: number; lang: Language; algorithm: Algorithm; td: string };
  astrolabe: Record<string, unknown>;
  horoscope: Record<string, unknown> | null;
}

/** `expected` 的每个键都在 `actual` 里且值相同；数组逐项、长度相同 */
function assertContains(actual: unknown, expected: unknown, path: string): void {
  if (Array.isArray(expected)) {
    assert.ok(Array.isArray(actual), `${path} 应为数组`);
    assert.equal(actual.length, expected.length, `${path} 长度`);
    expected.forEach((e, i) => assertContains(actual[i], e, `${path}[${i}]`));
  } else if (typeof expected === 'object' && expected !== null) {
    assert.ok(typeof actual === 'object' && actual !== null, `${path} 应为对象`);
    for (const [k, e] of Object.entries(expected)) {
      assertContains((actual as Record<string, unknown>)[k], e, `${path}.${k}`);
    }
  } else {
    assert.equal(actual, expected, path);
  }
}

/** 星耀对象把 DTO 里的空串亮度、四化记为 undefined（与 Python 的 None 一致） */
const EMPTY_AS_UNDEFINED = new Set(['brightness', 'mutagen']);

/** 对象自身的字段与 DTO 逐键逐值相同，且对象没有 DTO 之外的有值字段 */
function assertMirrors(object: unknown, dto: unknown, path: string): void {
  if (Array.isArray(dto)) {
    assert.ok(Array.isArray(object), `${path} 应为数组`);
    assert.equal(object.length, dto.length, `${path} 长度`);
    dto.forEach((d, i) => assertMirrors(object[i], d, `${path}[${i}]`));
    return;
  }
  if (typeof dto === 'object' && dto !== null) {
    assert.ok(typeof object === 'object' && object !== null, `${path} 应为对象`);
    const own = object as Record<string, unknown>;
    for (const [k, d] of Object.entries(dto)) {
      if (EMPTY_AS_UNDEFINED.has(k) && d === '') {
        assert.equal(own[k], undefined, `${path}.${k}`);
      } else {
        assertMirrors(own[k], d, `${path}.${k}`);
      }
    }
    for (const [k, v] of Object.entries(own)) {
      if (v !== undefined) assert.ok(k in dto, `${path}.${k} 不在 DTO 里`);
    }
    return;
  }
  assert.equal(object, dto, path);
}

const cases = golden<ContractCase[]>('contract_data.json');

test('金标覆盖多组排盘', () => {
  assert.ok(cases.length >= 10);
});

for (const c of cases) {
  const id = `${c.p.d} t${c.p.t} g${c.p.g} ${c.p.lang} ${c.p.algorithm}`;
  test(`契约与对象字段：${id}`, () => {
    const chart = astro.bySolar(c.p.d, c.p.t, c.p.g === 0 ? 'male' : 'female', {
      language: c.p.lang,
      config: { algorithm: c.p.algorithm },
    });
    const dto = chart.toJSON();
    assertContains(dto, c.astrolabe, 'astrolabe');
    assertMirrors(chart, dto, 'astrolabe');
    if (c.horoscope !== null) {
      // 金标生成器以卯时（索引 3）取运限
      const h = chart.horoscope(c.p.td, 3);
      const hDto = h.toJSON();
      assertContains(hDto, c.horoscope, 'horoscope');
      assertMirrors(h, hDto, 'horoscope');
    }
  });
}

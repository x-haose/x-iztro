/**
 * 知识包校验：四侧共用用例 tests/data/knowledge_check_cases.json（Rust knowledge_check.rs、Python
 * test_knowledge_check.py、Go knowledge_check_test.go 读同一份），默认包无问题，构造入口按 error 级问题拒绝。
 */

import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';
import { init, IztroError, KnowledgeIssueCode, KnowledgePack, type KnowledgeIssue } from 'x-iztro';

await init();

interface Case {
  name: string;
  pack?: unknown;
  packJson?: string;
  base?: 'builtin';
  expect: { level: string; path: (string | number)[]; code: string; params?: Record<string, unknown> }[];
}

// packJson 用例给的是 JSON 原文，适用于收文本的入口（Rust check_json、Go CheckKnowledgePack）；
// JS 的 check 收对象，非法 JSON 在 JSON.parse 阶段就报 SyntaxError
const CASES = (
  JSON.parse(readFileSync(new URL('../../tests/data/knowledge_check_cases.json', import.meta.url), 'utf8')) as {
    cases: Case[];
  }
).cases.filter((c) => 'pack' in c);

const shape = (issue: KnowledgeIssue, withParams: boolean) => ({
  level: issue.level,
  path: issue.path,
  code: issue.code,
  ...(withParams ? { params: issue.params } : {}),
});

for (const c of CASES) {
  test(c.name, () => {
    const base = c.base === 'builtin' ? KnowledgePack.builtin() : undefined;
    const issues = KnowledgePack.check(c.pack, { base });
    assert.deepEqual(
      issues.map((issue, i) => shape(issue, c.expect[i]?.params !== undefined)),
      c.expect,
    );
  });
}

test('默认包无问题', () => {
  assert.deepEqual(KnowledgePack.check(KnowledgePack.builtin().toJSON()), []);
});

test('fromObject 有 error 级问题即拒绝，只有 warning 照常构造', () => {
  for (const c of CASES.filter((x) => x.base === undefined)) {
    const hasError = c.expect.some((e) => e.level === 'error');
    if (!hasError) {
      KnowledgePack.fromObject(c.pack as object);
      continue;
    }
    assert.throws(
      () => KnowledgePack.fromObject(c.pack as object),
      (e: unknown) => {
        assert.ok(e instanceof IztroError, c.name);
        assert.equal(e.code, 'invalid_argument');
        assert.ok(e.message.startsWith('invalid knowledge pack: '));
        assert.deepEqual(
          e.issues?.map((i) => i.code),
          c.expect.map((x) => x.code),
          c.name,
        );
        return true;
      },
    );
  }
});

test('merged 先校验覆盖包对象', () => {
  assert.throws(
    () => KnowledgePack.builtin().merged({ schema: 1, id: 'o', version: '1', language: 'zh-CN', stars: { x: {} } }),
    (e: unknown) => e instanceof IztroError && e.issues?.[0]?.code === KnowledgeIssueCode.UNKNOWN_KEY,
  );
});

test('用法错误抛 TypeError', () => {
  assert.throws(() => KnowledgePack.check({ schema: 1, n: 1n }), TypeError);
  assert.throws(() => KnowledgePack.check({}, { base: {} as KnowledgePack }), TypeError);
});

const PARSE_CASES = (
  JSON.parse(readFileSync(new URL('../../tests/data/knowledge_check_cases.json', import.meta.url), 'utf8')) as {
    parseCases: { name: string; packJson: string; star: string; intro: string | null; combinations: object }[];
  }
).parseCases;

for (const c of PARSE_CASES) {
  test(`构造: ${c.name}`, () => {
    const star = KnowledgePack.fromJSON(c.packJson).star(c.star)!;
    assert.equal(star.intro ?? null, c.intro);
    assert.deepEqual(star.combinations, c.combinations);
  });
}

test('fromObject 持有规范化后的副本', () => {
  const raw = { schema: 1, id: 't', version: '1', language: 'zh-CN', extra: 1, stars: { ziweiMaj: { intro: 'a' } } };
  const pack = KnowledgePack.fromObject(raw);
  raw.stars.ziweiMaj.intro = 'b';
  assert.equal(pack.starIntro('ziweiMaj'), 'a');
  assert.equal('extra' in pack.toJSON(), false);
});

test('fromJSON 的非法 JSON 是一条 invalid_json 问题', () => {
  assert.throws(
    () => KnowledgePack.fromJSON('{bad'),
    (e: unknown) => e instanceof IztroError && e.issues?.[0]?.code === KnowledgeIssueCode.INVALID_JSON,
  );
});

test('NaN / Infinity 报 invalid_argument，不悄悄当 null', () => {
  const pack = { schema: 1, id: 't', version: '1', language: 'zh-CN', stars: { ziweiMaj: { intro: Number.NaN } } };
  for (const call of [() => KnowledgePack.check(pack), () => KnowledgePack.fromObject(pack)]) {
    assert.throws(call, (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument');
  }
});

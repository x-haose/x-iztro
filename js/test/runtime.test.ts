/**
 * 内核装载与调用协定：未初始化即报错、各种来源都能装载、并发装载只装一次、trap 后实例作废可重装。
 *
 * node:test 每个测试文件一个进程，本文件开头时内核尚未装载；各种来源的装载放进子进程，互不影响。
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { astro, init, isInitialized, IztroError } from 'x-iztro';

const PACKAGE_DIR = fileURLToPath(new URL('..', import.meta.url));
const WASM = new URL('../dist/x_iztro.wasm', import.meta.url);

/** 在包目录下起一个 Node 子进程跑 ESM 代码（按包名自引用 x-iztro），返回标准输出 */
function run(code: string): string {
  const r = spawnSync(process.execPath, ['--input-type=module', '-e', code], { cwd: PACKAGE_DIR, encoding: 'utf8' });
  assert.equal(r.status, 0, r.stderr);
  return r.stdout.trim();
}

/**
 * 手工编码的最小 wasm：导出内核要求的全部符号，四个功能入口一进去就执行 unreachable。
 * 用来以真实的 WebAssembly trap 验证实例作废逻辑；真内核在合法与非法入参下都不 trap。
 */
function trappingKernel(): Uint8Array {
  const bytes: number[] = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
  // 段长是无符号 LEB128
  const leb = (n: number): number[] => (n < 0x80 ? [n] : [(n & 0x7f) | 0x80, ...leb(n >> 7)]);
  const section = (id: number, body: number[]) => bytes.push(id, ...leb(body.length), ...body);
  const name = (s: string) => [s.length, ...Buffer.from(s)];
  // 类型：0 (i32)->i32 分配；1 (i32,i32)->() 释放；2 (i32,i32)->i64 功能入口
  section(1, [3, 0x60, 1, 0x7f, 1, 0x7f, 0x60, 2, 0x7f, 0x7f, 0, 0x60, 2, 0x7f, 0x7f, 1, 0x7e]);
  section(3, [6, 0, 1, 2, 2, 2, 2]);
  section(5, [1, 0x00, 1]);
  const entries = ['by_solar', 'by_lunar', 'horoscope', 'query'];
  section(7, [
    7,
    ...name('memory'), 2, 0,
    ...name('iztro_wasm_alloc'), 0, 0,
    ...name('iztro_wasm_free'), 0, 1,
    ...entries.flatMap((e, i) => [...name(`iztro_wasm_${e}`), 0, 2 + i]),
  ]);
  const unreachable = [3, 0x00, 0x00, 0x0b];
  section(10, [6, 4, 0x00, 0x41, 0x00, 0x0b, 2, 0x00, 0x0b, ...unreachable, ...unreachable, ...unreachable, ...unreachable]);
  return new Uint8Array(bytes);
}

test('未初始化时调用报 Error 并提示先 init', () => {
  assert.equal(isInitialized(), false);
  assert.throws(() => astro.bySolar('2000-8-16', 2, 'female'), /await init\(\)/);
});

test('trap 后实例作废，报 internal，再 init 换回可用实例', async () => {
  await init(trappingKernel());
  assert.equal(isInitialized(), true);
  assert.throws(
    () => astro.bySolar('2000-8-16', 2, 'female'),
    (e: unknown) => e instanceof IztroError && e.code === 'internal' && e.cause instanceof WebAssembly.RuntimeError,
  );
  assert.equal(isInitialized(), false);
  await init();
  assert.equal(astro.bySolar('2000-8-16', 2, 'female').soul, '破军');
});

test('不是内核的 wasm 报 internal，且可重试', async () => {
  const empty = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00]);
  const r = run(`
    import { init, isInitialized } from 'x-iztro';
    try { await init(new Uint8Array(${JSON.stringify([...empty])})); } catch (e) { console.log(e.name, e.code); }
    console.log(isInitialized());
    await init();
    console.log(isInitialized());
  `);
  assert.equal(r, 'IztroError internal\nfalse\ntrue');
});

for (const [label, expr] of [
  ['省略来源（读包内文件）', ''],
  ['file: URL', `new URL(${JSON.stringify(WASM.href)})`],
  ['文件路径字符串', JSON.stringify(fileURLToPath(WASM))],
  ['字节', `await (await import('node:fs/promises')).readFile(${JSON.stringify(fileURLToPath(WASM))})`],
  ['ArrayBuffer', `(await (await import('node:fs/promises')).readFile(${JSON.stringify(fileURLToPath(WASM))})).buffer`],
  ['已编译的 WebAssembly.Module', `await WebAssembly.compile(await (await import('node:fs/promises')).readFile(${JSON.stringify(fileURLToPath(WASM))}))`],
  ['Response', `new Response(await (await import('node:fs/promises')).readFile(${JSON.stringify(fileURLToPath(WASM))}), { headers: { 'content-type': 'application/wasm' } })`],
] as const) {
  test(`装载来源：${label}`, () => {
    const out = run(`
      import { init, astro } from 'x-iztro';
      await init(${expr});
      console.log(astro.bySolar('2000-8-16', 2, 'female').soul);
    `);
    assert.equal(out, '破军');
  });
}

test('并发调用 init 只装一次，重复调用直接返回', () => {
  const out = run(`
    import { init, astro, isInitialized } from 'x-iztro';
    await Promise.all([init(), init(), init()]);
    await init();
    console.log(isInitialized(), astro.bySolar('2000-8-16', 2, 'female').soul);
  `);
  assert.equal(out, 'true 破军');
});

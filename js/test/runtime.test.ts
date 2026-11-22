/**
 * 内核装载与调用协定：未初始化即报错、各种来源都能装载、并发装载只装一次、内核执行中断后实例作废可重装、
 * 2 GiB 以上的地址、入参里的非有限数字。
 *
 * node:test 每个测试文件一个进程，本文件开头时内核尚未装载；需要独立状态的场景放进子进程，互不影响。
 */

import assert from 'node:assert/strict';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { test } from 'node:test';
import { astro, init, isInitialized, IztroError, reverseChart, MinorStar, EarthlyBranch } from 'x-iztro';

const PACKAGE_DIR = fileURLToPath(new URL('..', import.meta.url));
const WASM = new URL('../dist/x_iztro.wasm', import.meta.url);
const WASM_PATH = JSON.stringify(fileURLToPath(WASM));

/** 在包目录下起一个 Node 子进程跑 ESM 代码（按包名自引用 x-iztro），返回标准输出 */
function run(code: string): string {
  const r = spawnSync(process.execPath, ['--input-type=module', '-e', code], { cwd: PACKAGE_DIR, encoding: 'utf8' });
  assert.equal(r.status, 0, r.stderr);
  return r.stdout.trim();
}

/** 无符号 LEB128 */
function uleb(n: number): number[] {
  return n < 0x80 ? [n] : [(n & 0x7f) | 0x80, ...uleb(Math.floor(n / 128))];
}

/** 有符号 LEB128（i32.const / i64.const 的立即数） */
function sleb(value: bigint): number[] {
  const out: number[] = [];
  let v = value;
  for (;;) {
    const byte = Number(v & 0x7fn);
    v >>= 7n;
    const done = (v === 0n && (byte & 0x40) === 0) || (v === -1n && (byte & 0x40) !== 0);
    out.push(done ? byte : byte | 0x80);
    if (done) return out;
  }
}

/** 假内核的函数体（不含长度前缀） */
const body = {
  /** 返回常量 i32 */
  i32: (v: bigint) => [0x00, 0x41, ...sleb(v), 0x0b],
  /** 返回常量 i64 */
  i64: (v: bigint) => [0x00, 0x42, ...sleb(v), 0x0b],
  nop: [0x00, 0x0b],
  unreachable: [0x00, 0x00, 0x0b],
};

/** 把无符号 32 位数按位转成 i32 立即数 */
const asI32 = (n: number): bigint => BigInt.asIntN(32, BigInt(n));

/**
 * 手工编码的假内核：导出真内核要求的全部符号，函数体由调用方给出。用来在真实的 WebAssembly 执行里
 * 复现真内核不会出现的情形（trap、越界结果、2 GiB 以上的地址）。
 */
function fakeKernel(spec: {
  alloc: number[];
  free: number[];
  entry: number[];
  pages: number;
  data?: { offset: number; text: string };
}): number[] {
  const bytes: number[] = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
  const section = (id: number, content: number[]) => bytes.push(id, ...uleb(content.length), ...content);
  const name = (s: string) => [s.length, ...Buffer.from(s)];
  const fn = (b: number[]) => [...uleb(b.length), ...b];
  // 类型：0 (i32)->i32 分配；1 (i32,i32)->() 释放；2 (i32,i32)->i64 功能入口
  section(1, [3, 0x60, 1, 0x7f, 1, 0x7f, 0x60, 2, 0x7f, 0x7f, 0, 0x60, 2, 0x7f, 0x7f, 1, 0x7e]);
  section(3, [6, 0, 1, 2, 2, 2, 2]);
  section(5, [1, 0x00, ...uleb(spec.pages)]);
  const entries = ['by_solar', 'by_lunar', 'horoscope', 'query'];
  section(7, [
    7,
    ...name('memory'), 2, 0,
    ...name('iztro_wasm_alloc'), 0, 0,
    ...name('iztro_wasm_free'), 0, 1,
    ...entries.flatMap((e, i) => [...name(`iztro_wasm_${e}`), 0, 2 + i]),
  ]);
  section(10, [6, ...fn(spec.alloc), ...fn(spec.free), ...entries.flatMap(() => fn(spec.entry))]);
  if (spec.data !== undefined) {
    const content = [...Buffer.from(spec.data.text)];
    section(11, [1, 0x00, 0x41, ...sleb(asI32(spec.data.offset)), 0x0b, ...uleb(content.length), ...content]);
  }
  return bytes;
}

/** 结果缓冲的打包值：(地址 << 32) | 长度，按 i64 立即数写 */
const packed = (ptr: number, len: number): bigint => BigInt.asIntN(64, (BigInt(ptr) << 32n) | BigInt(len));

const RESULT = '{"value":42}';

/** 在子进程里用假内核初始化后调用一次 `util.fixIndex`（统一查询入口），输出结果或错误 */
function callFake(kernel: number[]): string {
  return run(`
    import { init, util, isInitialized } from 'x-iztro';
    await init(new Uint8Array(${JSON.stringify(kernel)}));
    try { console.log('ok', util.fixIndex(1)); }
    catch (e) { console.log(e.name, e.code, isInitialized(), e.cause?.name); }
  `);
}

test('未初始化时调用报 Error 并提示先 init', () => {
  assert.equal(isInitialized(), false);
  assert.throws(() => astro.bySolar('2000-8-16', 2, 'female'), /await init\(\)/);
});

test('trap 后实例作废，报 internal，再 init 换回可用实例', async () => {
  await init(new Uint8Array(fakeKernel({ alloc: body.i32(0n), free: body.nop, entry: body.unreachable, pages: 1 })));
  assert.equal(isInitialized(), true);
  assert.throws(
    () => astro.bySolar('2000-8-16', 2, 'female'),
    (e: unknown) => e instanceof IztroError && e.code === 'internal' && e.cause instanceof WebAssembly.RuntimeError,
  );
  assert.equal(isInitialized(), false);
  await init();
  assert.equal(astro.bySolar('2000-8-16', 2, 'female').soul, '破军');
});

test('释放缓冲时 trap 同样作废实例并报 internal', () => {
  const kernel = fakeKernel({
    alloc: body.i32(0n),
    free: body.unreachable,
    entry: body.i64(packed(0x100, RESULT.length)),
    pages: 1,
    data: { offset: 0x100, text: RESULT },
  });
  assert.equal(callFake(kernel), 'IztroError internal false RuntimeError');
});

test('结果缓冲越出线性内存时作废实例并报 internal', () => {
  const kernel = fakeKernel({ alloc: body.i32(0n), free: body.nop, entry: body.i64(packed(0xfff0, 0x100)), pages: 1 });
  assert.equal(callFake(kernel), 'IztroError internal false RangeError');
});

test('线性内存 2 GiB 以上的地址按无符号读', () => {
  // 入参缓冲在 0x80000000，结果在 0x80000100：两者的最高位都是 1，按有符号读是负数
  const kernel = fakeKernel({
    alloc: body.i32(asI32(0x80000000)),
    free: body.nop,
    entry: body.i64(packed(0x80000100, RESULT.length)),
    pages: 32769,
    data: { offset: 0x80000100, text: RESULT },
  });
  assert.equal(callFake(kernel), 'ok 42');
});

test('不是内核的 wasm 报 internal，且可重试', () => {
  const empty = [0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00];
  const r = run(`
    import { init, isInitialized } from 'x-iztro';
    try { await init(new Uint8Array(${JSON.stringify(empty)})); } catch (e) { console.log(e.name, e.code); }
    console.log(isInitialized());
    await init();
    console.log(isInitialized());
  `);
  assert.equal(r, 'IztroError internal\nfalse\ntrue');
});

const readWasm = `await (await import('node:fs/promises')).readFile(${WASM_PATH})`;

for (const [label, expr] of [
  ['省略来源（读包内文件）', ''],
  ['file: URL', `new URL(${JSON.stringify(WASM.href)})`],
  ['文件路径字符串', WASM_PATH],
  ['字节', readWasm],
  ['ArrayBuffer', `(${readWasm}).buffer`],
  ['已编译的 WebAssembly.Module', `await WebAssembly.compile(${readWasm})`],
  ['Content-Type 为 application/wasm 的 Response（流式编译）', `new Response(${readWasm}, { headers: { 'content-type': 'application/wasm' } })`],
  ['Content-Type 不是 wasm 的 Response（整体读完再编译）', `new Response(${readWasm}, { headers: { 'content-type': 'application/octet-stream' } })`],
  ['Promise<Response>', `Promise.resolve(new Response(${readWasm}))`],
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

for (const [label, expr, reason] of [
  ['非 2xx 的 Response', `new Response('missing', { status: 404, statusText: 'Not Found' })`, 'HTTP 404'],
  ['被拒绝的 Promise<Response>', `Promise.reject(new Error('offline'))`, 'offline'],
  ['不存在的文件路径', JSON.stringify('/nonexistent/x_iztro.wasm'), 'ENOENT'],
  ['Windows 绝对路径按文件路径读，不当成 URL', JSON.stringify('C:\\nonexistent\\x_iztro.wasm'), 'ENOENT'],
] as const) {
  test(`装载失败报 internal 并可重试：${label}`, () => {
    const out = run(`
      import { init, isInitialized } from 'x-iztro';
      try { await init(${expr}); } catch (e) { console.log(e.name, e.code, e.message.includes(${JSON.stringify(reason)})); }
      console.log(isInitialized());
      await init();
      console.log(isInitialized());
    `);
    assert.equal(out, 'IztroError internal true\nfalse\ntrue');
  });
}

test('并发调用 init 只实例化一次，重复调用直接返回', () => {
  const out = run(`
    // Node 自己也会实例化 wasm（模块词法分析、fetch 的 HTTP 解析器），只数从本包 runtime 发起的
    let count = 0;
    const instantiate = WebAssembly.instantiate;
    WebAssembly.instantiate = (...args) => {
      if (new Error().stack.includes('/dist/runtime.js')) count += 1;
      return instantiate(...args);
    };
    import('x-iztro').then(async ({ init, astro, isInitialized }) => {
      await Promise.all([init(), init(), init()]);
      await init();
      console.log(count, isInitialized(), astro.bySolar('2000-8-16', 2, 'female').soul);
    });
  `);
  assert.equal(out, '1 true 破军');
});

test('入参里的非有限数字报 invalid_argument，不被当作「未给」', async () => {
  await init();
  const chart = astro.bySolar('2000-8-16', 2, 'female');
  const isInvalid = (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument';
  assert.throws(() => chart.horoscopeOfMonth(2024, Number.NaN), isInvalid);
  assert.throws(() => chart.horoscopeOfYear(Number.POSITIVE_INFINITY), isInvalid);
  assert.throws(() => chart.horoscope('2024-1-1', Number.NaN as 0), isInvalid);
  assert.throws(() => reverseChart({ stars: [{ star: MinorStar.HUOXING, branch: EarthlyBranch.YIN }], limit: Number.NaN }), isInvalid);
  assert.throws(() => chart.yearlyOverviewToText(2024, Number.NEGATIVE_INFINITY), isInvalid);
  assert.equal(isInitialized(), true);
});

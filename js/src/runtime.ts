/**
 * WebAssembly 内核的装载与调用。
 *
 * 内核是 `wasm32-unknown-unknown` 目标编出的 x-iztro，不导入任何宿主函数。
 * 调用协定见 Rust 侧 `src/wasm.rs`：入参是写进内核缓冲区的 UTF-8 JSON，
 * 返回 `(ptr << 32) | len` 打包的结果缓冲，内容是 DTO JSON 或 `{"error", "code"}`；
 * 两个缓冲都由 `iztro_wasm_free` 释放。Python 与 Go 绑定走同一套分派，三侧行为不会分叉。
 *
 * 装载是异步的（取文件、编译），装好之后的每次调用都是同步的纯计算。
 */

import { invalidArgument, IztroError } from './errors.js';

/**
 * 内核的来源：
 * - `URL` / `string`：wasm 文件地址，浏览器端用 `fetch` 取，Node 端 `file:` 地址读文件、其余用 `fetch`
 * - `Response` 或其 Promise：已经发出的请求，`Content-Type` 为 `application/wasm` 时边下边编译
 * - `ArrayBuffer` / `TypedArray`：wasm 文件的字节
 * - `WebAssembly.Module`：已编译的模块，如 Cloudflare Workers 的 wasm 导入
 */
export type WasmSource = URL | string | Response | Promise<Response> | ArrayBuffer | ArrayBufferView | WebAssembly.Module;

/** 内核导出的函数与线性内存 */
interface KernelExports {
  /** 内核的线性内存；内核分配时可能扩容，每次读写都要重新取 `buffer` */
  readonly memory: WebAssembly.Memory;
  /** 分配 `len` 字节的入参缓冲，返回起始地址 */
  iztro_wasm_alloc(len: number): number;
  /** 释放内核分配或返回的缓冲 */
  iztro_wasm_free(ptr: number, len: number): void;
  /** 阳历排盘 */
  iztro_wasm_by_solar(ptr: number, len: number): bigint;
  /** 农历排盘 */
  iztro_wasm_by_lunar(ptr: number, len: number): bigint;
  /** 运限 */
  iztro_wasm_horoscope(ptr: number, len: number): bigint;
  /** 统一查询，返回 `{"value": …}` */
  iztro_wasm_query(ptr: number, len: number): bigint;
}

/** 内核的四个功能入口 */
type Entry = 'by_solar' | 'by_lunar' | 'horoscope' | 'query';

/** 当前可用的内核实例；未初始化或实例 trap 后为 `undefined` */
let kernel: KernelExports | undefined;

/** 进行中的初始化，并发调用 `init` 时共用 */
let loading: Promise<void> | undefined;

const encoder = new TextEncoder();
const decoder = new TextDecoder('utf-8', { fatal: true });

/** 由来源取得可以实例化的输入：地址交给 `resolve`（各入口自己决定怎么取），其余原样 */
export type Resolver = (location: URL | string) => Promise<Response | ArrayBufferView | ArrayBuffer>;

/**
 * 装载内核。已装好时直接返回；装载进行中时后到的调用共用那一次装载，自己传入的 `source` 不被使用。
 *
 * 失败时抛 `IztroError`（code 为 internal），原因在 `cause` 里；失败后可以再次调用重试。
 */
export function initialize(source: WasmSource, resolve: Resolver): Promise<void> {
  if (kernel !== undefined) {
    return Promise.resolve();
  }
  loading ??= (async () => {
    try {
      const input = source instanceof URL || typeof source === 'string' ? await resolve(source) : await source;
      kernel = exportsOf(await instantiate(input));
    } catch (cause) {
      throw new IztroError('internal', `failed to load x_iztro.wasm: ${String(cause)}`, { cause });
    } finally {
      loading = undefined;
    }
  })();
  return loading;
}

/** 内核是否已装好可用 */
export function isInitialized(): boolean {
  return kernel !== undefined;
}

async function instantiate(
  input: Response | ArrayBuffer | ArrayBufferView | WebAssembly.Module,
): Promise<WebAssembly.Instance> {
  if (input instanceof WebAssembly.Module) {
    return WebAssembly.instantiate(input, {});
  }
  if (typeof Response !== 'undefined' && input instanceof Response) {
    if (!input.ok) {
      throw new Error(`HTTP ${input.status} ${input.statusText} for ${input.url}`);
    }
    // instantiateStreaming 只接受 Content-Type 为 application/wasm 的响应，否则直接拒绝；
    // 静态托管与桌面 WebView 的自定义协议未必给出这个类型。此时改为整体读完再编译：
    // 结果是同一个实例，只是不能边下边编；读取或编译本身的失败照样抛出
    if (input.headers.get('content-type')?.split(';')[0]?.trim() === 'application/wasm') {
      return (await WebAssembly.instantiateStreaming(input, {})).instance;
    }
    return (await WebAssembly.instantiate(await input.arrayBuffer(), {})).instance;
  }
  // 共享内存视图（SharedArrayBuffer）也是 ArrayBufferView，编译接口不收，由 instantiate 自己报错
  return (await WebAssembly.instantiate(input as BufferSource, {})).instance;
}

function exportsOf(instance: WebAssembly.Instance): KernelExports {
  const exports = instance.exports as Partial<KernelExports>;
  const names: (keyof KernelExports)[] = [
    'memory',
    'iztro_wasm_alloc',
    'iztro_wasm_free',
    'iztro_wasm_by_solar',
    'iztro_wasm_by_lunar',
    'iztro_wasm_horoscope',
    'iztro_wasm_query',
  ];
  const missing = names.filter((name) => exports[name] === undefined);
  if (missing.length > 0) {
    throw new Error(`not an x-iztro kernel: missing exports ${missing.join(', ')}`);
  }
  return exports as KernelExports;
}

/**
 * 入参转 JSON 时拒绝非有限数字：`JSON.stringify` 会把 NaN、±Infinity 写成 null，
 * 内核把 null 当作「未给」，按缺省值算出一个看似正常的结果。
 */
function finiteNumbers(key: string, value: unknown): unknown {
  if (typeof value === 'number' && !Number.isFinite(value)) {
    throw invalidArgument(`${key || 'input'} must be a finite number, got ${value}`);
  }
  return value;
}

/**
 * 以 JSON 入参调用内核的一个入口，返回解析后的结果。
 *
 * 值为 `undefined` 的键不进 JSON，内核按缺省处理。
 *
 * 内核不导入任何宿主函数，调用期间抛出的任何异常（trap、wasm 栈溢出、结果缓冲越界或不是合法 UTF-8）
 * 都表示内核执行中断或产出了坏数据：实例随即作废，抛 `IztroError`（internal），下次 `init()` 重建。
 *
 * @throws IztroError 入参含非有限数字（invalid_argument）；内核拒绝入参（code 见 {@link ErrorCode}）；
 *   内核执行中断（internal，实例随即作废）
 * @throws Error 尚未初始化
 */
export function invoke(entry: Entry, input: object): unknown {
  const k = kernel;
  if (k === undefined) {
    throw new Error('x-iztro is not initialized: await init() before calling it');
  }
  const bytes = encoder.encode(JSON.stringify(input, finiteNumbers));
  let text: string;
  try {
    // 地址与打包结果按无符号读：线性内存超过 2 GiB 后，i32 / i64 的最高位为 1，按有符号读是负数
    const inPtr = k.iztro_wasm_alloc(bytes.length) >>> 0;
    new Uint8Array(k.memory.buffer, inPtr, bytes.length).set(bytes);
    const packed = BigInt.asUintN(64, k[`iztro_wasm_${entry}`](inPtr, bytes.length));
    const outPtr = Number(packed >> 32n);
    const outLen = Number(packed & 0xffffffffn);
    text = decoder.decode(new Uint8Array(k.memory.buffer, outPtr, outLen));
    k.iztro_wasm_free(outPtr, outLen);
    k.iztro_wasm_free(inPtr, bytes.length);
  } catch (cause) {
    if (kernel === k) kernel = undefined;
    const reason = cause instanceof Error ? `${cause.name}: ${cause.message}` : String(cause);
    throw new IztroError('internal', `x_iztro.wasm stopped mid-call (${reason}); the instance is retired, await init() again`, {
      cause,
    });
  }
  const result: unknown = JSON.parse(text);
  if (isErrorPayload(result)) {
    throw new IztroError(result.code, result.error);
  }
  return result;
}

/** 内核错误 JSON：恰有 `error` 与 `code` 两个键。DTO 与查询结果（`{"value"}`）都不是这个形状 */
function isErrorPayload(value: unknown): value is { error: string; code: IztroError['code'] } {
  if (typeof value !== 'object' || value === null) return false;
  const keys = Object.keys(value);
  return keys.length === 2 && 'error' in value && 'code' in value;
}

/** 调用统一查询入口：`kind` 决定调用哪个查询，其余键按需取用 */
export function query(kind: string, params: object = {}): unknown {
  return (invoke('query', { kind, ...params }) as { value: unknown }).value;
}

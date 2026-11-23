/**
 * Node.js 入口（package.json `exports` 的 node 条件；Next.js 服务端渲染与路由处理也落在这里）。
 *
 * 与浏览器入口的 API 完全相同，区别只在默认取内核的方式：`file:` 地址读文件，Node 的 `fetch` 不支持该协议。
 */

import { readFile } from 'node:fs/promises';
import { initialize, type WasmSource } from './runtime.js';

export * from './index.js';

/**
 * 字符串来源里当作 URL 的协议；其余字符串一律是文件路径。不能用「能否解析成 URL」来判断：
 * Windows 绝对路径 `C:\x\x_iztro.wasm` 会被解析成协议为 `c:` 的 URL。
 */
const URL_SCHEME = /^(?:file|https?|data):/i;

/**
 * 装载 WebAssembly 内核。其余 API 都是同步的，调用前必须等它完成。已装好时直接返回；
 * 装载进行中时后到的调用共用那一次装载，自己传入的 `source` 不被使用。
 *
 * @param source 内核来源；省略时读包内的 x_iztro.wasm。字符串可以是 URL 或文件路径
 * @throws IztroError 读文件或实例化失败（code 为 internal），可再次调用重试
 */
export function init(source?: WasmSource): Promise<void> {
  return initialize(source ?? new URL('./x_iztro.wasm', import.meta.url), async (location) => {
    if (typeof location === 'string' && !URL_SCHEME.test(location)) return readFile(location);
    const url = new URL(location);
    return url.protocol === 'file:' ? readFile(url) : fetch(url);
  });
}

/**
 * Node.js 入口（package.json `exports` 的 node 条件；Next.js 服务端渲染与路由处理也落在这里）。
 *
 * 与浏览器入口的 API 完全相同，区别只在默认取内核的方式：`file:` 地址读文件，Node 的 `fetch` 不支持该协议。
 */

import { readFile } from 'node:fs/promises';
import { initialize, type WasmSource } from './runtime.js';

export * from './index.js';

/**
 * 装载 WebAssembly 内核。其余 API 都是同步的，调用前必须等它完成；重复调用直接返回。
 *
 * @param source 内核来源；省略时读包内的 x_iztro.wasm。字符串可以是 URL 或文件路径
 * @throws IztroError 读文件或实例化失败（code 为 internal），可再次调用重试
 */
export function init(source?: WasmSource): Promise<void> {
  return initialize(source ?? new URL('./x_iztro.wasm', import.meta.url), async (location) => {
    // 字符串既可以是 URL，也可以是文件路径
    if (typeof location === 'string' && !URL.canParse(location)) return readFile(location);
    const url = new URL(location);
    return url.protocol === 'file:' ? readFile(url) : fetch(url);
  });
}

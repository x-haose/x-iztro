/**
 * 浏览器与打包器入口（package.json `exports` 的 default 条件；Vite、Next.js 客户端、Tauri 前端都落在这里）。
 *
 * 内核文件随包放在本文件旁边，按 `new URL('./x_iztro.wasm', import.meta.url)` 取用：
 * Vite、webpack、Turbopack 都识别这个写法，把 wasm 作为静态资源产出并改写成最终地址。
 */

import { initialize, type WasmSource } from './runtime.js';

export * from './index.js';

/**
 * 装载 WebAssembly 内核。其余 API 都是同步的，调用前必须等它完成；重复调用直接返回。
 *
 * @param source 内核来源；省略时取包内的 x_iztro.wasm。自行托管 wasm（CDN、离线包）时传它的地址或字节
 * @throws IztroError 取文件或实例化失败（code 为 internal），可再次调用重试
 */
export function init(source?: WasmSource): Promise<void> {
  return initialize(source ?? new URL('./x_iztro.wasm', import.meta.url), (location) => fetch(location));
}

// 把刚编出的 wasm32-unknown-unknown 内核放到 dist/ 根目录，与 browser.js、node.js 同级：
// 两个入口都按 new URL('./x_iztro.wasm', import.meta.url) 取它，打包器也按这个相对位置收录。
import { copyFile, mkdir } from 'node:fs/promises';

const from = new URL('../../target/wasm32-unknown-unknown/release/x_iztro.wasm', import.meta.url);
const dist = new URL('../dist/', import.meta.url);
await mkdir(dist, { recursive: true });
await copyFile(from, new URL('x_iztro.wasm', dist));

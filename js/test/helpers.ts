/**
 * 测试共用：仓库金标目录、读 JSON / 文本、等距互素抽样。
 *
 * 测试按包名 `x-iztro` 导入（包自引用），走的是 package.json `exports` 的 node 条件与构建产物 dist/，
 * 与使用者装包后的加载路径相同。
 */

import { readFileSync, readdirSync } from 'node:fs';

/** 仓库的金标目录 tests/golden/ */
export const GOLDEN = new URL('../../tests/golden/', import.meta.url);

/** 读金标目录下的 JSON */
export function golden<T>(relative: string): T {
  return JSON.parse(readFileSync(new URL(relative, GOLDEN), 'utf8')) as T;
}

/** 读语义化文本快照 `text_snapshots/<name>.txt` */
export function snapshot(name: string): string {
  return readFileSync(new URL(`text_snapshots/${name}.txt`, GOLDEN), 'utf8');
}

/** 金标目录某子目录下的文件名 */
export function goldenFiles(dir: string): string[] {
  return readdirSync(new URL(`${dir}/`, GOLDEN)).sort();
}

/**
 * 抽样步长：金标用例按「年 × 时辰 × 性别」等嵌套循环平铺，等距切片的步长一旦与某一层的周期同余
 * 就会把该层锁死在单一取值。改用与用例总数互素的模步进，保证抽样在每一层都遍历。
 */
const SAMPLE_STRIDE = 101;

function gcd(a: number, b: number): number {
  return b === 0 ? a : gcd(b, a % b);
}

/** 从用例里等距互素地抽 `limit` 个 */
export function sample<T>(cases: readonly T[], limit: number): T[] {
  if (gcd(SAMPLE_STRIDE, cases.length) !== 1) {
    throw new Error(`stride ${SAMPLE_STRIDE} is not coprime with ${cases.length} cases`);
  }
  return Array.from({ length: Math.min(limit, cases.length) }, (_, i) => cases[(i * SAMPLE_STRIDE) % cases.length] as T);
}

/** 取文本中以 prefix 开头的第一行 */
export function lineStartingWith(text: string, prefix: string): string {
  const line = text.split('\n').find((l) => l.startsWith(prefix));
  if (line === undefined) throw new Error(`no line starts with ${prefix}`);
  return line;
}

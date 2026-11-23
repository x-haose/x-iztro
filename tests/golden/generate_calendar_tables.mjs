/**
 * 历法数据表抽取：把 lunar-typescript 的寿星历法系数表与农历闰月特例表写成 Rust 源码。
 *
 * 读取的是运行时取值（`ShouXingUtil.XL0` 等），不解析源码文本；SB / QB 输出解码后的修正串。
 * f64 用 JS 的最短往返写法输出，Rust 解析得到同一个双精度值。
 *
 * 输出：../../src/calendar/tables.rs（写出后经 rustfmt 排版）
 */
import { ShouXingUtil, LunarYear } from 'lunar-typescript';
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const out = join(__dirname, '../../src/calendar/tables.rs');
const version = JSON.parse(
  readFileSync(join(__dirname, 'node_modules/lunar-typescript/package.json'), 'utf8'),
).version;

/** f64 字面量：JS 最短往返写法，整数补 `.0` 以免被 Rust 当成整数字面量。 */
const float = (v) => {
  const s = String(v);
  return /[.e]/.test(s) ? s : `${s}.0`;
};
const floats = (name, doc, values) =>
  `/// ${doc}\npub(super) static ${name}: [f64; ${values.length}] = [${values.map(float).join(', ')}];\n`;
const ints = (name, doc, values) =>
  `/// ${doc}\npub(super) static ${name}: [i64; ${values.length}] = [${values.join(', ')}];\n`;
const digits = (name, doc, s) => {
  if (!/^[012]*$/.test(s)) throw new Error(`${name} 含 0/1/2 以外的字符`);
  return `/// ${doc}\npub(super) static ${name}: &[u8; ${s.length}] = b"${s}";\n`;
};

const xl1 = ShouXingUtil.XL1.map((row, i) => floats(`XL1_${i}`, `月球黄经级数第 ${i} 阶（t 的 ${i} 次项），每 6 个数一项。`, row));

const src = `//! 寿星天文历的系数表与农历闰月特例表。
//!
//! 由 \`tests/golden/generate_calendar_tables.mjs\` 从 lunar-typescript ${version} 的运行时取值生成，勿手改。
//! 版权与许可见同目录 \`LICENSE-lunar-typescript\`。

${floats('NUT_B', '章动级数，每 5 个数一项：相位常数、一次项、二次项系数，黄经振幅与交角振幅（0.01 角秒）。', ShouXingUtil.NUT_B)}
${floats('DT_AT', 'ΔT（力学时 − 世界时，秒）分段三次多项式，每 5 个数一段：起始年、四个系数；末两项为表尾年份与该年 ΔT。', ShouXingUtil.DT_AT)}
${floats('XL0', '地球日心黄经 VSOP87 截断级数：首项为缩放因子，其后 7 个数是 6 阶各自的起止下标，再后每 3 个数一项（振幅、相位、频率）。', ShouXingUtil.XL0)}
${xl1.join('\n')}
${floats('SHUO_KB', '古历平朔参数：首朔儒略日与朔望月长度交替排列，末项为定朔修正表的起点儒略日。', ShouXingUtil.SHUO_KB)}
${floats('QI_KB', '古历平气参数：首气儒略日与气长交替排列，末项为定气修正表的起点儒略日。', ShouXingUtil.QI_KB)}
${digits('SB', '619 年起逐个朔日的修正：b\'0\' 不修正，b\'1\' 加一日，b\'2\' 减一日。', ShouXingUtil.SB)}
${digits('QB', '1645 年起逐个节气的修正：b\'0\' 不修正，b\'1\' 加一日，b\'2\' 减一日。', ShouXingUtil.QB)}
${ints('LEAP_11', '闰月按历史记载定为闰十一月的农历年。', LunarYear._LEAP_11)}
${ints('LEAP_12', '闰月按历史记载定为闰十二月的农历年。', LunarYear._LEAP_12)}`;

writeFileSync(out, src);
execFileSync('rustfmt', ['--edition', '2024', out]);
console.log(`tables.rs: lunar-typescript ${version}，XL0 ${ShouXingUtil.XL0.length}，SB ${ShouXingUtil.SB.length}，QB ${ShouXingUtil.QB.length}`);

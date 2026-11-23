/**
 * 输入写法金标：日期串的文法与运限省略时辰的换算。
 *
 * - solar：同一生辰的各种公历写法（分隔符 - / .、补零、时间部分）各排一张盘，记 solarDate 回显
 *   与规范化哈希；非法写法记 iztro 的报错。
 * - lunar：农历入参同理（含闰月）。
 * - horoscope：同一张本命盘，目标日期取不同写法与时间，时辰省略或给出，按运限紧凑格式导出；
 *   非法目标日期记报错。
 *
 * 输出：inputs.json
 */
import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { hashAstrolabe } from './canonical.mjs';
import { horoscopeCase } from './horoscope_case.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));

const SOLAR = [
  '2000-8-16',
  '2000/8/16',
  '2000.8.16',
  '2000-08-16',
  '2000/08/06',
  '2000-8-16 10:30',
  '2000/8/16 23:59:59',
  '2000.08.16 0:0',
  '2000-8-16 00:00:00',
  '2024-2-29',
  // 非法写法
  '2000-8/16',
  '2000-8-16T10:30',
  '2000-8-16 24:00',
  '2000-8-16 10:60',
  '2000-8-16 10:30:60',
  '2000-8-16 10',
  '2000-8-16 ',
  '+2000-8-16',
  '2000-8',
  '2000-13-1',
  '2000-2-30',
  '2001-2-29',
  '10000-1-1',
];

const LUNAR = [
  ['2000-7-17', false],
  ['2000/7/17', false],
  ['2000.07.17 10:00', false],
  ['2020-4-16', true],
  ['2020/04/16', false],
  ['2020.4.16 23:30:00', true],
  // 非法写法
  ['2000-7/17', false],
  ['2000-13-1', false],
  ['2000-7-17T10:00', false],
];

const HOROSCOPE_TARGETS = [
  ['2026-3-1', null],
  ['2026-3-1 0:10', null],
  ['2026-3-1 1:00', null],
  ['2026-3-1 12:59', null],
  ['2026-3-1 13:20', null],
  ['2026-3-1 22:59:59', null],
  ['2026-3-1 23:30', null],
  ['2026/3/1 13:20', null],
  ['2026.03.01 13:20', null],
  ['2026-03-01', 6],
  ['2026-3-1 13:20', 2],
  // 非法目标日期
  ['2026-3/1', null],
  ['2026-3-1 24:00', null],
  ['2026-2-30', null],
];

const attempt = (fn) => {
  try {
    return fn();
  } catch (e) {
    return { error: e.message };
  }
};

const solar = SOLAR.map((input) => ({
  input,
  ...attempt(() => {
    const a = astro.bySolar(input, 2, '女', true, 'zh-CN');
    return { solarDate: a.solarDate, hash: hashAstrolabe(a) };
  }),
}));

const lunar = LUNAR.map(([input, leap]) => ({
  input,
  leap,
  ...attempt(() => {
    const a = astro.byLunar(input, 2, '女', leap, true, 'zh-CN');
    return { solarDate: a.solarDate, hash: hashAstrolabe(a) };
  }),
}));

const birth = { d: '2000-8-16', t: 2, g: 1 };
const base = astro.bySolar(birth.d, birth.t, '女', true, 'zh-CN');
const horoscope = HOROSCOPE_TARGETS.map(([td, tt]) => {
  const out = attempt(() => horoscopeCase(base, birth, td, tt));
  return 'error' in out ? { p: birth, td, tt, ...out } : out;
});

writeFileSync(join(__dirname, 'inputs.json'), JSON.stringify({ solar, lunar, horoscope }, null, 1));
const errors = (list) => list.filter((c) => 'error' in c).length;
console.log(
  `inputs.json: solar ${solar.length}（非法 ${errors(solar)}）, lunar ${lunar.length}（非法 ${errors(lunar)}）, horoscope ${horoscope.length}（非法 ${errors(horoscope)}）`,
);

/**
 * 绑定契约金标生成器：JS iztro 的完整 JSON.stringify 输出。
 *
 * Rust 侧 DTO（camelCase + 翻译值）序列化后必须与这里的对象逐键逐值一致。
 * 导出前删除实现细节字段：顶层 plugins/copyright、运限对象内嵌的 astrolabe。
 *
 * 采样：多年份 × 性别 × 时辰（含晚子时）× 语言 × 算法，含闰月命盘；
 * 每个命盘附带一个固定目标日期的运限对象。
 *
 * 输出：contract_data.json
 */
import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const DEFAULTS = {
  yearDivide: 'normal',
  horoscopeDivide: 'normal',
  ageDivide: 'normal',
  dayDivide: 'forward',
  algorithm: 'default',
};

/** 序列化时过滤私有反向引用与运限内嵌盘。 */
const replacer = (k, v) =>
  k === '_astrolabe' || k === '_palace' || k === 'astrolabe' ? undefined : v;

/** JSON 化并删除实现细节字段。 */
function cleanAstrolabe(result) {
  const j = JSON.parse(JSON.stringify(result, replacer));
  delete j.plugins;
  delete j.copyright;
  return j;
}

function cleanHoroscope(h) {
  return JSON.parse(JSON.stringify(h, replacer));
}

const CASES = [
  // [date, ti, gender, language, algorithm]
  ['2000-8-16', 2, '女', 'zh-CN', 'default'],
  ['1984-2-15', 0, '男', 'zh-CN', 'default'],
  ['1995-6-1', 6, '女', 'zh-CN', 'default'],
  ['2004-4-10', 12, '男', 'zh-CN', 'default'],
  ['2017-7-20', 8, '女', 'zh-CN', 'default'],
  ['2023-8-15', 12, '男', 'zh-CN', 'default'],
  ['2043-12-31', 11, '女', 'zh-CN', 'default'],
  ['2000-8-16', 2, '女', 'zh-TW', 'default'],
  ['2000-8-16', 2, '女', 'en-US', 'default'],
  ['2000-8-16', 2, '女', 'ja-JP', 'default'],
  ['2000-8-16', 2, '女', 'ko-KR', 'default'],
  ['2000-8-16', 2, '女', 'vi-VN', 'default'],
  ['1990-11-5', 4, '男', 'zh-CN', 'zhongzhou'],
];

const out = [];
for (const [d, t, g, lang, algorithm] of CASES) {
  astro.config({ ...DEFAULTS, algorithm });
  const result = astro.bySolar(d, t, g, true, lang);
  // 目标取出生后第 25 年，保证晚于出生
  const td = `${Number(d.split('-')[0]) + 25}-6-15`;
  const horoscope = result.horoscope(td, 3);
  out.push({
    p: { d, t, g: g === '男' ? 0 : 1, lang, algorithm, td },
    astrolabe: cleanAstrolabe(result),
    horoscope: cleanHoroscope(horoscope),
  });
}

writeFileSync(join(__dirname, 'contract_data.json'), JSON.stringify(out));
console.log(`Generated ${out.length} contract cases`);

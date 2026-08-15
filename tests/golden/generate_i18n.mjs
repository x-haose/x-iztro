/**
 * i18n 金标：iztro 全部可翻译标识在六种语言下的译文，以及每条译文的反查结果。
 *
 * iztro 的 t(key) 与 kot(text) 面向同一张翻译表，表内混放星耀、宫位、干支、
 * 亮度、四化、五行局、性别、生肖、时辰、星座、运限层级共十一类标识。
 * 逐条落盘后供 tests/golden_i18n.rs 校验 x-iztro 的 translate_key / key_of。
 *
 * 同一译文在多个标识下同形时（en-US 的 horse 既是生肖马也是天马），kot 的取值
 * 取决于它逐语言逐标识的扫描顺序，故反查结果必须逐条落盘对照，不能只校验
 * 「反查到某个译文相同的标识」——那样查不出顺序分叉。
 *
 * 输出 i18n_table.json 与 i18n_kot.json。
 */
import fs from 'node:fs';
import { kot } from 'iztro/lib/i18n/index.js';
import zhCN from 'iztro/lib/i18n/locales/zh-CN/index.js';
import zhTW from 'iztro/lib/i18n/locales/zh-TW/index.js';
import enUS from 'iztro/lib/i18n/locales/en-US/index.js';
import jaJP from 'iztro/lib/i18n/locales/ja-JP/index.js';
import koKR from 'iztro/lib/i18n/locales/ko-KR/index.js';
import viVN from 'iztro/lib/i18n/locales/vi-VN/index.js';

const LOCALES = {
  'zh-CN': zhCN.default,
  'zh-TW': zhTW.default,
  'en-US': enUS.default,
  'ja-JP': jaJP.default,
  'ko-KR': koKR.default,
  'vi-VN': viVN.default,
};

const keys = Object.keys(LOCALES['zh-CN']);
const table = {};

for (const key of keys) {
  table[key] = Object.fromEntries(
    Object.entries(LOCALES).map(([lang, dict]) => [lang, dict[key]]),
  );
}

// 各语言条目数必须一致，否则说明 iztro 的翻译表本身有缺漏
for (const [lang, dict] of Object.entries(LOCALES)) {
  const missing = keys.filter((k) => dict[k] === undefined);
  if (missing.length) {
    console.warn(`${lang} 缺 ${missing.length} 条: ${missing.slice(0, 10).join(', ')}`);
  }
}

fs.writeFileSync('i18n_table.json', JSON.stringify(table, null, 2));
console.log(`i18n_table.json: ${keys.length} 个标识 × ${Object.keys(LOCALES).length} 种语言`);

// 反查：每条译文一例，记下 kot 落到哪个标识。
// 取样全来自翻译表本身，故每例都能查到，不存在 kot 返回原文的兜底情形。
const lookups = [];
for (const key of keys) {
  for (const [lang, dict] of Object.entries(LOCALES)) {
    const text = dict[key];
    if (typeof text !== 'string') continue;
    lookups.push({ text, lang, key, kot: kot(text) });
  }
}

fs.writeFileSync('i18n_kot.json', JSON.stringify(lookups, null, 2));
console.log(`i18n_kot.json: ${lookups.length} 例反查`);

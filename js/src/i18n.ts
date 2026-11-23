/**
 * 标识与译名的双向查找。
 *
 * 星盘对象的每个字段都同时给出译名与 `*Key` 标识，通常不必再手工翻译；这三个函数用于手上只有标识
 * （或只有某种语言的译名）、需要换算的场合。覆盖星耀、宫位（含身宫、来因宫）、天干、地支、亮度、四化、
 * 五行局、性别、生肖、时辰、星座、运限层级，合计 260 个标识。
 */

import type { Language } from './enums.js';
import { query } from './runtime.js';

/** 标识译成指定语言的文本；未知标识为 undefined */
export function translate(key: string, language: Language = 'zh-CN'): string | undefined {
  return (query('translate', { key, language }) as string | null) ?? undefined;
}

/**
 * 由任意语言的译名反查标识（iztro 的 `kot`）；查不到为 undefined。
 *
 * 逐语言、每种语言内逐标识比对取先命中者，顺序与 iztro 一致。同一译名在多个类目下同形时
 * （en-US 的 "horse" 既是生肖马也是天马），用 `keyFilter` 限定标识名须含的子串来消歧：
 * "Maj" 只看十四主星、"Min" 只看辅星、"Heavenly" / "Earthly" 只看干支、"Palace" 只看宫位、"Hour" 只看时辰。
 * 限定后无匹配为 undefined，不退回未限定的结果。
 */
export function keyOf(text: string, keyFilter?: string): string | undefined {
  return (query('keyOf', { text, keyFilter }) as string | null) ?? undefined;
}

/**
 * 全部 260 个可翻译标识。顺序即 `keyOf` 的反查次序，与 iztro 各语言翻译文件的合并次序一致：
 * 运限层级、生肖、时辰、星座、五行局、天干、地支、亮度、四化、星耀、宫位、性别。
 */
export function allKeys(): string[] {
  return query('allKeys') as string[];
}

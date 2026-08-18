/**
 * 排盘结果的规范化字符串。
 *
 * 与 Rust 侧 tests/common/mod.rs 的 canonical_astrolabe() 严格同构：
 * 逐字节一致的字符串经 SHA-256 后即为 tier3 金标哈希。
 *
 * 格式：
 *   顶层字段 '|' 连接，之后每宫一段，段间 '#' 连接。
 *   星耀条目为 `名:类型:范围:亮度:四化`，无亮度/四化者取空串；
 *   主星、辅星、杂耀三类用同一条目格式。
 *   辅星与杂耀按字符串排序（安放顺序双边无约定），主星保持安放顺序。
 *
 * 排序等价前提：JS 的 Array.sort() 按 UTF-16 码元序，Rust 的 slice::sort()
 * 按 UTF-8 字节序，两者仅在基本多文种平面（BMP）内等价。星名、类型键、
 * 范围键、亮度、四化全部落在 BMP 内，故双边排序结果一致。
 */
import crypto from 'node:crypto';

const star = (s) => `${s.name}:${s.type}:${s.scope}:${s.brightness || ''}:${s.mutagen || ''}`;

export function canonicalAstrolabe(result) {
  const top = [
    result.gender,
    result.solarDate,
    result.lunarDate,
    result.chineseDate,
    result.time,
    result.timeRange,
    result.sign,
    result.zodiac,
    result.earthlyBranchOfSoulPalace,
    result.earthlyBranchOfBodyPalace,
    result.soul,
    result.body,
    result.fiveElementsClass,
  ].join('|');

  const palaces = result.palaces
    .map((p) =>
      [
        p.name,
        p.heavenlyStem,
        p.earthlyBranch,
        p.isBodyPalace ? 1 : 0,
        p.isOriginalPalace ? 1 : 0,
        p.majorStars.map(star).join(';'),
        p.minorStars.map(star).sort().join(';'),
        p.adjectiveStars.map(star).sort().join(';'),
        p.changsheng12,
        p.boshi12,
        p.jiangqian12,
        p.suiqian12,
        `${p.decadal.range[0]}-${p.decadal.range[1]}`,
        p.ages.join(','),
      ].join('|'),
    )
    .join('#');

  return `${top}#${palaces}`;
}

export function hashAstrolabe(result) {
  return crypto.createHash('sha256').update(canonicalAstrolabe(result)).digest('hex');
}

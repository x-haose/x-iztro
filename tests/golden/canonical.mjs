/**
 * 排盘结果的规范化字符串。
 *
 * 与 Rust 侧 tests/common/mod.rs 的 canonical_astrolabe() 严格同构：
 * 逐字节一致的字符串经 SHA-256 后即为 tier3 金标哈希。
 *
 * 格式：
 *   顶层字段 '|' 连接，之后每宫一段，段间 '#' 连接。
 *   星耀条目为 `名:亮度:四化`（无则为空串）；杂耀仅星名。
 *   辅星与杂耀按字符串排序（安放顺序双边无约定），主星保持安放顺序。
 */
import crypto from 'node:crypto';

const star = (s) => `${s.name}:${s.brightness || ''}:${s.mutagen || ''}`;

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
        p.adjectiveStars.map((s) => s.name).sort().join(';'),
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

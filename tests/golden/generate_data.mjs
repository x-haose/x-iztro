/**
 * 数据表金标：把 iztro `data` 模块导出的四张查表原样落盘。
 *
 * 覆盖 STARS_INFO（星耀亮度表 / 五行 / 阴阳）、heavenlyStems（天干阴阳 / 五行 /
 * 对冲 / 四化）、earthlyBranches（地支阴阳 / 五行 / 对冲 / 命主 / 身主 /
 * 脏腑 / 身体部位 / 健康提示）与 constants 里的顺序常量。
 *
 * 输出 data_tables.json 供 tests/golden_data.rs 逐键逐值比对。
 */
import fs from 'node:fs';
import { data } from 'iztro';

const {
  STARS_INFO,
  MUTAGEN,
  heavenlyStems,
  earthlyBranches,
  LANGUAGES,
  HEAVENLY_STEMS,
  EARTHLY_BRANCHES,
  ZODIAC,
  PALACES,
  GENDER,
  CHINESE_TIME,
  TIME_RANGE,
  TIGER_RULE,
  RAT_RULE,
} = data;

const out = {
  starsInfo: STARS_INFO,
  mutagen: MUTAGEN,
  heavenlyStems,
  earthlyBranches,
  constants: {
    LANGUAGES,
    HEAVENLY_STEMS,
    EARTHLY_BRANCHES,
    ZODIAC,
    PALACES,
    GENDER,
    CHINESE_TIME,
    TIME_RANGE,
    TIGER_RULE,
    RAT_RULE,
  },
};

fs.writeFileSync('data_tables.json', JSON.stringify(out, null, 2));

const starCount = Object.keys(STARS_INFO).length;
console.log(`data_tables.json: ${starCount} 星耀, 10 天干, 12 地支`);

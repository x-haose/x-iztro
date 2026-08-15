/**
 * star 模块金标：iztro 各安星函数在一批出生数据上的逐值输出。
 *
 * 覆盖 iztro `star` 模块对外的全部函数：紫微天府起宫、各组落宫索引、
 * 主星/辅星/杂耀的十二宫分布、长生与博士12神、岁前与将前12神，
 * 以及流耀（按干支与运限层级）。
 *
 * 星耀一律记 key 而非译名，与 Rust 侧的 StarKey 直接可比。
 * 输出 star_cases.json 供 tests/golden_star.rs 零容忍比对。
 */
import fs from 'node:fs';
import { star, astro } from 'iztro';
import { kot } from 'iztro/lib/i18n/index.js';

const {
  getStartIndex,
  getLuYangTuoMaIndex,
  getKuiYueIndex,
  getChangQuIndex,
  getKongJieIndex,
  getTimelyStarIndex,
  getLuanXiIndex,
  getDailyStarIndex,
  getMonthlyStarIndex,
  getYearlyStarIndex,
  getMajorStar,
  getMinorStar,
  getAdjectiveStar,
  getchangsheng12,
  getBoShi12,
  getYearly12,
  getHoroscopeStar,
  getChangesheng12StartIndex,
  getJiangqian12StartIndex,
  getZuoYouIndex,
  getHuoLingIndex,
  getHuagaiXianchiIndex,
  getGuGuaIndex,
  getJieshaAdjIndex,
  getDahaoIndex,
  getNianjieIndex,
  getTianshiTianshangIndex,
  getChangQuIndexByHeavenlyStem,
} = star;

/**
 * 译名反查 key。
 *
 * 星耀的 key 不带统一后缀（`ziweiMaj` / `changsheng` / `boshi` 混杂），
 * 因此不能用 kot 的 k 参数过滤，只能全表反查；反查失败会原样返回中文，
 * 这里直接判死，避免把未转换的译名当作 key 落进金标。
 */
const toKey = (name) => {
  const key = kot(name);
  if (!/^[a-zA-Z0-9]+$/.test(key)) {
    throw new Error(`译名 ${name} 反查 key 失败，得到 ${key}`);
  }
  return key;
};

/** 星耀二维数组 → 每宫的 key 列表 */
const starKeys = (groups) => groups.map((palace) => palace.map((s) => toKey(s.name)));

/** 十二神数组 → key 列表 */
const shenKeys = (names) => names.map(toKey);

// 覆盖闰月、晚子时、跨季、男女与五种五行局
const CASES = [
  { solarDate: '2000-8-16', timeIndex: 2, gender: '女', fixLeap: true },
  { solarDate: '2000-8-16', timeIndex: 12, gender: '男', fixLeap: true },
  { solarDate: '1990-2-4', timeIndex: 0, gender: '男', fixLeap: true },
  { solarDate: '1984-6-28', timeIndex: 7, gender: '女', fixLeap: false },
  { solarDate: '2023-6-18', timeIndex: 5, gender: '男', fixLeap: true },
  { solarDate: '2023-3-22', timeIndex: 11, gender: '女', fixLeap: true },
  { solarDate: '1976-9-15', timeIndex: 4, gender: '男', fixLeap: true },
  { solarDate: '2012-12-21', timeIndex: 9, gender: '女', fixLeap: false },
];

const STEMS = ['甲', '乙', '丙', '丁', '戊', '己', '庚', '辛', '壬', '癸'];
const BRANCHES = ['子', '丑', '寅', '卯', '辰', '巳', '午', '未', '申', '酉', '戌', '亥'];
const SCOPES = ['origin', 'decadal', 'yearly', 'monthly', 'daily', 'hourly'];
const FIVE_ELEMENTS = ['水二局', '木三局', '金四局', '土五局', '火六局'];

const cases = CASES.map((param) => ({
  param,
  startIndex: getStartIndex(param),
  luYangTuoMa: getLuYangTuoMaIndex(...astroYearlyPillar(param)),
  kuiYue: getKuiYueIndex(astroYearlyPillar(param)[0]),
  changQu: getChangQuIndex(param.timeIndex),
  kongJie: getKongJieIndex(param.timeIndex),
  timely: getTimelyStarIndex(param.timeIndex),
  luanXi: getLuanXiIndex(astroYearlyPillar(param)[1]),
  daily: getDailyStarIndex(param.solarDate, param.timeIndex, param.fixLeap),
  monthly: getMonthlyStarIndex(param.solarDate, param.timeIndex, param.fixLeap),
  yearly: getYearlyStarIndex(param),
  majorStars: starKeys(getMajorStar(param)),
  minorStars: starKeys(getMinorStar(param.solarDate, param.timeIndex, param.fixLeap)),
  adjectiveStars: starKeys(getAdjectiveStar(param)),
  changsheng12: shenKeys(getchangsheng12(param)),
  boshi12: shenKeys(getBoShi12(param.solarDate, param.gender)),
  yearly12: (() => {
    const { suiqian12, jiangqian12 } = getYearly12(param.solarDate);
    return { suiqian12: shenKeys(suiqian12), jiangqian12: shenKeys(jiangqian12) };
  })(),
}));

/** 从排盘结果取年柱干支：安星函数需要它，而它的分界由配置决定 */
function astroYearlyPillar({ solarDate, timeIndex, gender, fixLeap }) {
  const chart = astro.bySolar(solarDate, timeIndex, gender, fixLeap);
  const [stem, branch] = chart.rawDates.chineseDate.yearly;
  return [stem, branch];
}

// 流耀：干支全组合 × 六个运限层级
const horoscopeStars = [];
for (const stem of STEMS) {
  for (const branch of BRANCHES) {
    for (const scope of SCOPES) {
      horoscopeStars.push({
        stem,
        branch,
        scope,
        stars: starKeys(getHoroscopeStar(stem, branch, scope)),
      });
    }
  }
}

// 两个起始索引函数：五行局 / 年支全覆盖
const changsheng12StartIndex = FIVE_ELEMENTS.map((fe) => ({
  fiveElementsClass: kot(fe),
  index: getChangesheng12StartIndex(fe),
}));

const jiangqian12StartIndex = BRANCHES.map((b) => ({
  branch: kot(b, 'Earthly'),
  index: getJiangqian12StartIndex(b),
}));

// 低层落宫函数：不收出生数据、只收已算好的中间量，故按各自入参域全覆盖。
// 起盘流水线之外单独调用它们时走的就是这条路径。
const locations = {
  zuoYou: Array.from({ length: 12 }, (_, i) => ({
    lunarMonth: i + 1,
    ...getZuoYouIndex(i + 1),
  })),
  huoLing: BRANCHES.flatMap((b) =>
    Array.from({ length: 13 }, (_, t) => ({
      branch: kot(b, 'Earthly'),
      timeIndex: t,
      ...getHuoLingIndex(b, t),
    })),
  ),
  huagaiXianchi: BRANCHES.map((b) => ({
    branch: kot(b, 'Earthly'),
    ...getHuagaiXianchiIndex(b),
  })),
  guGua: BRANCHES.map((b) => ({ branch: kot(b, 'Earthly'), ...getGuGuaIndex(b) })),
  jieshaAdj: BRANCHES.map((b) => ({
    branch: kot(b, 'Earthly'),
    index: getJieshaAdjIndex(kot(b, 'Earthly')),
  })),
  dahao: BRANCHES.map((b) => ({
    branch: kot(b, 'Earthly'),
    index: getDahaoIndex(kot(b, 'Earthly')),
  })),
  nianjie: BRANCHES.map((b) => ({ branch: kot(b, 'Earthly'), index: getNianjieIndex(b) })),
  changQuByStem: STEMS.map((s) => ({
    stem: kot(s, 'Heavenly'),
    ...getChangQuIndexByHeavenlyStem(s),
  })),
  // 天伤天使按派别分支：中州派在阴男阳女时与通行派对调，故两个 algorithm 都取
  tianshiTianshang: ['default', 'zhongzhou'].flatMap((algorithm) => {
    astro.config({ algorithm });
    return ['男', '女'].flatMap((gender) =>
      BRANCHES.flatMap((b) =>
        Array.from({ length: 12 }, (_, soulIndex) => ({
          algorithm,
          gender: kot(gender),
          branch: kot(b, 'Earthly'),
          soulIndex,
          ...getTianshiTianshangIndex(gender, kot(b, 'Earthly'), soulIndex),
        })),
      ),
    );
  }),
};
astro.config({ algorithm: 'default' });

fs.writeFileSync(
  'star_cases.json',
  JSON.stringify(
    { cases, horoscopeStars, changsheng12StartIndex, jiangqian12StartIndex, locations },
    null,
    2,
  ),
);

const locationCount = Object.values(locations).reduce((n, v) => n + v.length, 0);
console.log(
  `star_cases.json: ${cases.length} 例出生数据, ${horoscopeStars.length} 例流耀, ` +
    `${changsheng12StartIndex.length} 例长生起点, ${jiangqian12StartIndex.length} 例将前起点, ` +
    `${locationCount} 例低层落宫`,
);

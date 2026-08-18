/**
 * Config 非默认取值金标生成器。
 *
 * iztro 的 config() 只更新传入项且无重置接口，每组生成前显式传全部配置项。
 *
 * - yearDivide=exact：立春/初一分歧窗口（每年 1/20-2/20 逐日）排盘哈希
 *   → config_yeardivide.csv（date,ti,g,hash）
 * - dayDivide=current：晚子时用例（含农历月末跨月场景）排盘哈希
 *   → config_daydivide.csv（date,ti,g,hash）
 * - ageDivide=birthday：骑农历生日的运限用例（虚岁分界）
 *   → config_agedivide.json
 * - horoscopeDivide=exact：立春窗口目标日期的运限用例（干支节气分界）
 *   → config_horoscopedivide.json
 *
 * 单例重放：
 *   node generate_config.mjs --inspect <yearDivide> <dayDivide> <date> <ti> <男|女>
 */
import { astro } from 'iztro';
import { solar2lunar, lunar2solar } from 'lunar-lite';
import { LunarYear } from 'lunar-typescript';
import { writeFileSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { canonicalAstrolabe, hashAstrolabe } from './canonical.mjs';

const __dirname = dirname(fileURLToPath(import.meta.url));
const HASH_LEN = 32;
const GENDERS = ['男', '女'];

const DEFAULTS = {
  yearDivide: 'normal',
  horoscopeDivide: 'normal',
  ageDivide: 'normal',
  dayDivide: 'forward',
  algorithm: 'default',
};

if (process.argv[2] === '--inspect') {
  const [yd, dd, date, t, gender] = process.argv.slice(3);
  astro.config({ ...DEFAULTS, yearDivide: yd, dayDivide: dd });
  console.log(canonicalAstrolabe(astro.bySolar(date, Number(t), gender, true, 'zh-CN')));
  process.exit(0);
}

/** 单个运限层级的紧凑导出（宫位名序列原序，流耀分布每宫排序）。 */
function scopeItem(item) {
  const out = {
    i: item.index,
    n: item.name,
    hs: item.heavenlyStem,
    eb: item.earthlyBranch,
    pn: item.palaceNames,
    m: item.mutagen,
  };
  if (item.stars) {
    out.s = item.stars.map((g) => g.map((s) => s.name).sort());
  }
  return out;
}

/** 完整运限用例导出（与 generate_horoscope.mjs 同构）。 */
function horoscopeCase(astrolabe, birth, td, tt) {
  const h = astrolabe.horoscope(td, tt);
  return {
    p: birth,
    td,
    tt,
    ld: h.lunarDate,
    dec: scopeItem(h.decadal),
    age: { ...scopeItem(h.age), na: h.age.nominalAge },
    yr: {
      ...scopeItem(h.yearly),
      sq: h.yearly.yearlyDecStar.suiqian12,
      jq: h.yearly.yearlyDecStar.jiangqian12,
    },
    mo: scopeItem(h.monthly),
    da: scopeItem(h.daily),
    hr: scopeItem(h.hourly),
  };
}

// ============================================================
// 1. yearDivide=exact：立春/初一分歧窗口逐日
// ============================================================

astro.config({ ...DEFAULTS, yearDivide: 'exact' });

const ydLines = [];
for (let year = 1984; year <= 2043; year++) {
  for (let month = 1; month <= 2; month++) {
    const [dayStart, dayEnd] = month === 1 ? [20, 31] : [1, 20];
    for (let day = dayStart; day <= dayEnd; day++) {
      const dateStr = `${year}-${month}-${day}`;
      for (const t of [0, 6]) {
        for (const g of [0, 1]) {
          const result = astro.bySolar(dateStr, t, GENDERS[g], true, 'zh-CN');
          ydLines.push(`${dateStr},${t},${g},${hashAstrolabe(result).slice(0, HASH_LEN)}`);
        }
      }
    }
  }
}
writeFileSync(join(__dirname, 'config_yeardivide.csv'), ydLines.join('\n') + '\n');
console.log(`yearDivide=exact: ${ydLines.length} cases`);

// ============================================================
// 2. dayDivide=current：晚子时用例（含农历月末跨月）
// ============================================================

astro.config({ ...DEFAULTS, dayDivide: 'current' });

const ddLines = [];
for (let year = 1984; year <= 2043; year++) {
  const dates = [`${year}-2-15`, `${year}-8-15`];
  // 找当年一个农历廿九的阳历日期，覆盖晚子时不进位下的月末场景
  outer: for (let month = 3; month <= 12; month++) {
    for (let day = 1; day <= 28; day++) {
      const dateStr = `${year}-${month}-${day}`;
      if (solar2lunar(dateStr).lunarDay === 29) {
        dates.push(dateStr);
        break outer;
      }
    }
  }
  for (const dateStr of dates) {
    for (const g of [0, 1]) {
      const result = astro.bySolar(dateStr, 12, GENDERS[g], true, 'zh-CN');
      ddLines.push(`${dateStr},12,${g},${hashAstrolabe(result).slice(0, HASH_LEN)}`);
    }
  }
}
writeFileSync(join(__dirname, 'config_daydivide.csv'), ddLines.join('\n') + '\n');
console.log(`dayDivide=current: ${ddLines.length} cases`);

// ============================================================
// 3. ageDivide=birthday：骑农历生日的运限
// ============================================================

astro.config({ ...DEFAULTS, ageDivide: 'birthday' });

const adCases = [];
for (let year = 1984; year <= 2043; year += 5) {
  const birthDate = `${year}-2-15`;
  const bl = solar2lunar(birthDate);
  for (const g of [0, 1]) {
    const astrolabe = astro.bySolar(birthDate, 6, GENDERS[g], true, 'zh-CN');
    const birth = { d: birthDate, t: 6, g };
    for (const k of [3, 10, 27]) {
      // 目标农历周年生日的前一天、当天、后一天，精确骑生日分界；
      // 周年当月天数不足时截断到该月末日
      const targetYear = bl.lunarYear + k;
      const maxDay = LunarYear.fromYear(targetYear).getMonth(bl.lunarMonth).getDayCount();
      const anniversary = lunar2solar(
        `${targetYear}-${bl.lunarMonth}-${Math.min(bl.lunarDay, maxDay)}`,
      );
      const base = new Date(anniversary.solarYear, anniversary.solarMonth - 1, anniversary.solarDay);
      for (const offset of [-1, 0, 1]) {
        const d = new Date(base);
        d.setDate(d.getDate() + offset);
        const td = `${d.getFullYear()}-${d.getMonth() + 1}-${d.getDate()}`;
        adCases.push(horoscopeCase(astrolabe, birth, td, 4));
      }
    }
  }
}
writeFileSync(join(__dirname, 'config_agedivide.json'), JSON.stringify(adCases));
console.log(`ageDivide=birthday: ${adCases.length} cases`);

// ============================================================
// 4. horoscopeDivide=exact：立春窗口目标日期的运限
// ============================================================

astro.config({ ...DEFAULTS, horoscopeDivide: 'exact' });

const hdCases = [];
for (let year = 1984; year <= 2043; year += 5) {
  const birthDate = `${year}-2-15`;
  for (const g of [0, 1]) {
    const astrolabe = astro.bySolar(birthDate, 6, GENDERS[g], true, 'zh-CN');
    const birth = { d: birthDate, t: 6, g };
    for (const k of [5, 20]) {
      // 立春窗口逐日 + 一个年中普通日期
      for (const md of ['2-2', '2-3', '2-4', '2-5', '2-6', '7-15']) {
        hdCases.push(horoscopeCase(astrolabe, birth, `${year + k}-${md}`, 8));
      }
    }
  }
}
writeFileSync(join(__dirname, 'config_horoscopedivide.json'), JSON.stringify(hdCases));
console.log(`horoscopeDivide=exact: ${hdCases.length} cases`);

// ============================================================
// 5. 开关组合（排盘层）：yearDivide/dayDivide 与中州派算法的交叉
// ============================================================

/** 排盘层组合：cfg 标识 -> 覆盖在 DEFAULTS 上的配置项。 */
const CHART_COMBOS = {
  yd_dd: { yearDivide: 'exact', dayDivide: 'current' },
  zz: { algorithm: 'zhongzhou' },
  zz_yd: { algorithm: 'zhongzhou', yearDivide: 'exact' },
  zz_dd: { algorithm: 'zhongzhou', dayDivide: 'current' },
  zz_yd_dd: { algorithm: 'zhongzhou', yearDivide: 'exact', dayDivide: 'current' },
};

const comboLines = [];
for (const [cfg, overrides] of Object.entries(CHART_COMBOS)) {
  astro.config({ ...DEFAULTS, ...overrides });
  for (let year = 1984; year <= 2043; year += 5) {
    // 立春窗口三天打穿 yearDivide 分界，8-15 为窗口外对照
    for (const md of ['2-3', '2-4', '2-5', '8-15']) {
      const dateStr = `${year}-${md}`;
      // 时辰 0 与晚子时 12 打穿 dayDivide 分界
      for (const t of [0, 12]) {
        for (const g of [0, 1]) {
          const result = astro.bySolar(dateStr, t, GENDERS[g], true, 'zh-CN');
          comboLines.push(`${cfg},${dateStr},${t},${g},${hashAstrolabe(result).slice(0, HASH_LEN)}`);
        }
      }
    }
  }
}
writeFileSync(join(__dirname, 'config_combos.csv'), comboLines.join('\n') + '\n');
console.log(`chart combos: ${comboLines.length} cases`);

// ============================================================
// 6. 开关组合（运限层）：ageDivide/horoscopeDivide 与中州派算法的交叉
// ============================================================

/** 运限层组合：cfg 标识 -> 覆盖在 DEFAULTS 上的配置项。 */
const HOROSCOPE_COMBOS = {
  zz_age: { algorithm: 'zhongzhou', ageDivide: 'birthday' },
  zz_hd: { algorithm: 'zhongzhou', horoscopeDivide: 'exact' },
  age_hd: { ageDivide: 'birthday', horoscopeDivide: 'exact' },
  zz_age_hd: { algorithm: 'zhongzhou', ageDivide: 'birthday', horoscopeDivide: 'exact' },
};

const comboHoroscopes = [];
for (const [cfg, overrides] of Object.entries(HOROSCOPE_COMBOS)) {
  astro.config({ ...DEFAULTS, ...overrides });
  for (let year = 1984; year <= 2043; year += 10) {
    const birthDate = `${year}-2-15`;
    const bl = solar2lunar(birthDate);
    for (const g of [0, 1]) {
      const astrolabe = astro.bySolar(birthDate, 6, GENDERS[g], true, 'zh-CN');
      const birth = { d: birthDate, t: 6, g };
      // 农历生日当天（ageDivide 分界）与立春前后两天（horoscopeDivide 分界）
      const anniversary = lunar2solar(`${bl.lunarYear + 20}-${bl.lunarMonth}-${bl.lunarDay}`);
      const targets = [
        `${anniversary.solarYear}-${anniversary.solarMonth}-${anniversary.solarDay}`,
        `${year + 20}-2-3`,
        `${year + 20}-2-5`,
        `${year + 20}-9-9`,
      ];
      for (const td of targets) {
        comboHoroscopes.push({ cfg, ...horoscopeCase(astrolabe, birth, td, 8) });
      }
    }
  }
}
writeFileSync(join(__dirname, 'config_combos_horoscope.json'), JSON.stringify(comboHoroscopes));
console.log(`horoscope combos: ${comboHoroscopes.length} cases`);

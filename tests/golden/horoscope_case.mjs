/**
 * 运限用例的紧凑导出，运限、Config 与输入写法三个金标生成器共用；
 * Rust 侧由 tests/common 的 check_horoscope_case 逐字段对照。
 */

/** 单个运限层级的紧凑导出（宫位名序列原序，流耀分布每宫排序）。 */
export function scopeItem(item) {
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

/**
 * 一个运限用例：`birth` 为 `{ d, t, g }`（出生日期、时辰、0 男 1 女），`td` / `tt` 为
 * 目标日期串与时辰；`tt` 为 null 表示省略，由 iztro 按 `td` 的时间部分换算。
 */
export function horoscopeCase(astrolabe, birth, td, tt) {
  const h = tt === null ? astrolabe.horoscope(td) : astrolabe.horoscope(td, tt);
  return {
    p: birth,
    td,
    tt,
    sd: h.solarDate,
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

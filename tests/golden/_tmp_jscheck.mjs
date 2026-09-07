import { astro } from 'iztro';
const [bd, bt, g, td, tt, dd, ad, hd] = process.argv.slice(2);
astro.config({ yearDivide:'normal', horoscopeDivide: hd, ageDivide: ad, dayDivide: dd, algorithm:'default' });
const a = astro.bySolar(bd, Number(bt), g === '0' ? '男' : '女', true, 'zh-CN');
const h = a.horoscope(td, Number(tt));
const item = (i) => ({ i: i.index, hs: i.heavenlyStem, eb: i.earthlyBranch, m: i.mutagen, pn: i.palaceNames, s: i.stars ? i.stars.map(gg=>gg.map(s=>s.name).sort()) : undefined });
console.log(JSON.stringify({ ld: h.lunarDate, sd: h.solarDate, na: h.age.nominalAge, dec: item(h.decadal), age: item(h.age), yr: item(h.yearly), mo: item(h.monthly), da: item(h.daily), hr: item(h.hourly) }));

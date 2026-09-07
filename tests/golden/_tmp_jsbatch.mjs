import { astro } from 'iztro';
import { readFileSync } from 'node:fs';
const lines = readFileSync(process.argv[2],'utf8').trim().split('\n');
const item = (i) => ({ i: i.index, hs: i.heavenlyStem, eb: i.earthlyBranch, m: i.mutagen, pn: i.palaceNames, s: i.stars ? i.stars.map(g=>g.map(s=>s.name).sort()) : null });
const out=[];
for (const l of lines) {
  const [bd,bt,g,td,tt,dd,ad,hd] = l.split(' ');
  astro.config({ yearDivide:'normal', horoscopeDivide: hd, ageDivide: ad, dayDivide: dd, algorithm:'default' });
  const a = astro.bySolar(bd, Number(bt), g==='0'?'男':'女', true, 'zh-CN');
  const h = a.horoscope(td, Number(tt));
  out.push(JSON.stringify({ ld: h.lunarDate, sd: h.solarDate, na: h.age.nominalAge, dec: item(h.decadal), age: item(h.age), yr: item(h.yearly), mo: item(h.monthly), da: item(h.daily), hr: item(h.hourly) }));
}
console.log(out.join('\n'));

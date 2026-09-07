import { astro } from 'iztro';
astro.config({yearDivide:'normal',horoscopeDivide:'exact',ageDivide:'normal',dayDivide:'forward',algorithm:'default'});
const [d,t]=process.argv.slice(2);
console.log(JSON.stringify(astro.bySolar(d,Number(t),'男',true,'zh-CN').rawDates.chineseDate));

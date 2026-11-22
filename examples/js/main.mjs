// x-iztro JavaScript 示例
//
// 前置步骤（在仓库根目录）：
//     cd js && npm ci && npm run build
// 运行：
//     cd examples/js && npm install && npm start

import { init, astro, IztroError, MajorStar, Mutagen, PalaceName, Scope, TextDetail } from 'x-iztro';

// 装载 WebAssembly 内核，只需一次；之后的调用都是同步的
await init();

console.log('===== 1. 阳历排盘 =====\n');
const chart = astro.bySolar('2000-8-16', 2, 'female');
console.log(`阳历：${chart.solarDate}`);
console.log(`农历：${chart.lunarDate}`);
console.log(`干支：${chart.chineseDate}`);
console.log(`时辰：${chart.time} (${chart.timeRange})`);
console.log(`星座：${chart.sign}　生肖：${chart.zodiac}`);
console.log(`命主：${chart.soul}　身主：${chart.body}　五行局：${chart.fiveElementsClass}\n`);

console.log('===== 2. 农历排盘 =====\n');
const lunar = astro.byLunar('2000-7-17', 2, 'female');
console.log(`阳历：${lunar.solarDate}　农历：${lunar.lunarDate}\n`);

console.log('===== 3. 十二宫概览 =====\n');
for (const p of chart.palaces) {
  const stars = p.majorStars.map((s) => s.name).join(' ') || '（空宫）';
  console.log(`${p.name.padEnd(2, '　')} ${p.heavenlyStem}${p.earthlyBranch}　${stars}`);
}
console.log();

console.log('===== 4. 宫位判断与飞星 =====\n');
const soul = chart.palace(PalaceName.SOUL);
console.log(`命宫有紫微：${soul.has([MajorStar.ZIWEI])}`);
console.log(`命宫自化权：${soul.selfMutaged(Mutagen.QUAN)}`);
console.log(`命宫四化飞入：${soul.mutagedPlaces().map((p) => p?.name).join('、')}`);
console.log(`三方四正有武曲：${chart.isSurrounded(PalaceName.SOUL, [MajorStar.WUQU])}\n`);

console.log('===== 5. 运限 =====\n');
const horoscope = chart.horoscopeOfYear(2026);
console.log(`流年：${horoscope.yearly.heavenlyStem}${horoscope.yearly.earthlyBranch}，流年命宫在本命${chart.palaces[horoscope.yearly.index].name}`);
console.log(`流年四化：${horoscope.yearly.mutagen.join('、')}`);
console.log(`流年命宫：${horoscope.palace(PalaceName.SOUL, Scope.YEARLY).name}\n`);

console.log('===== 6. 格局 =====\n');
for (const hit of chart.patterns()) console.log(`${hit.name}（${hit.palaceName}）`);
console.log();

console.log('===== 7. 语义化文本 =====\n');
console.log(chart.toText({ detail: TextDetail.OVERVIEW }).split('\n').slice(0, 9).join('\n'));
console.log();

console.log('===== 8. JSON 导出与错误处理 =====\n');
console.log(`JSON 长度：${JSON.stringify(chart).length}`);
try {
  astro.bySolar('2000-2-30', 2, 'female');
} catch (e) {
  if (e instanceof IztroError) console.log(`IztroError ${e.code}: ${e.message}`);
}

console.log('\n===== 示例完毕 =====');

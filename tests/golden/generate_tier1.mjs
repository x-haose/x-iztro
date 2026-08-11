// Tier 1 golden data generator
// 60 years (1984-2043) × 13 time indices (0-12) = 780 cases
// Fixed: month=2, day=15, gender='男', fixLeap=true, language='zh-CN'

import { astro } from 'iztro';
import { writeFileSync } from 'node:fs';

const results = [];

for (let year = 1984; year <= 2043; year++) {
  for (let timeIndex = 0; timeIndex <= 12; timeIndex++) {
    const solarDate = `${year}-2-15`;
    const result = astro.bySolar(solarDate, timeIndex, '男', true, 'zh-CN');

    const entry = {
      params: {
        solar_date: solarDate,
        time_index: timeIndex,
        gender: '男',
      },
      gender: result.gender,
      lunar_date: result.lunarDate,
      chinese_date: result.chineseDate,
      time: result.time,
      time_range: result.timeRange,
      sign: result.sign,
      zodiac: result.zodiac,
      soul_palace_branch: result.earthlyBranchOfSoulPalace,
      body_palace_branch: result.earthlyBranchOfBodyPalace,
      five_elements_class: result.fiveElementsClass,
      soul_star: result.soul,
      body_star: result.body,
      palaces: result.palaces.map(p => ({
        name: p.name,
        heavenly_stem: p.heavenlyStem,
        earthly_branch: p.earthlyBranch,
        is_body_palace: p.isBodyPalace,
        is_original_palace: p.isOriginalPalace,
        major_stars: p.majorStars.map(s => ({
          name: s.name,
          type: s.type,
          brightness: s.brightness || null,
          mutagen: s.mutagen || null,
        })),
        minor_stars: p.minorStars.map(s => ({
          name: s.name,
          type: s.type,
          brightness: s.brightness || null,
          mutagen: s.mutagen || null,
        })),
        adjective_stars: p.adjectiveStars.map(s => ({
          name: s.name,
          type: s.type,
        })),
        changsheng12: p.changsheng12,
        boshi12: p.boshi12,
        jiangqian12: p.jiangqian12,
        suiqian12: p.suiqian12,
        decadal_range: p.decadal.range,
        decadal_heavenly_stem: p.decadal.heavenlyStem,
        decadal_earthly_branch: p.decadal.earthlyBranch,
        ages: p.ages,
      })),
    };

    results.push(entry);
  }
}

writeFileSync('tier1_data.json', JSON.stringify(results));
console.log(`Generated ${results.length} cases`);

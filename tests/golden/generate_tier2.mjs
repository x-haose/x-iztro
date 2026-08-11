/**
 * Tier 2 golden test generator
 * Coverage: 60 years (1984-2043) × 12 months × {day 1, day 15} × 13 time indices × 2 genders
 * ~18,720 cases stored as 60 files (one per year)
 */
import { astro } from 'iztro';
import { writeFileSync, mkdirSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outDir = join(__dirname, 'tier2');
mkdirSync(outDir, { recursive: true });

const GENDERS = ['男', '女'];
const DAYS = [1, 15];
const START_YEAR = 1984;
const END_YEAR = 2043;

for (let year = START_YEAR; year <= END_YEAR; year++) {
  const cases = [];

  for (let month = 1; month <= 12; month++) {
    for (const day of DAYS) {
      for (let t = 0; t <= 12; t++) {
        for (let gi = 0; gi < GENDERS.length; gi++) {
          const dateStr = `${year}-${month}-${day}`;
          try {
            const result = astro.bySolar(dateStr, t, GENDERS[gi], true, 'zh-CN');

            const entry = {
              d: dateStr,
              t,
              g: gi,
              sb: result.earthlyBranchOfSoulPalace,
              bb: result.earthlyBranchOfBodyPalace,
              fc: result.fiveElementsClass,
              ss: result.soul,
              bs: result.body,
              pn: result.palaces.map(p => p.name),
              ms: result.palaces.map(p => p.majorStars.map(s => s.name)),
              ns: result.palaces.map(p => p.minorStars.map(s => s.name)),
              dr: result.palaces.map(p => p.decadal.range),
            };

            cases.push(entry);
          } catch {
            // Skip invalid dates (e.g., Feb 30)
          }
        }
      }
    }
  }

  const outPath = join(outDir, `year_${year}.json`);
  writeFileSync(outPath, JSON.stringify(cases), 'utf-8');
  console.log(`${outPath}: ${cases.length} cases`);
}

console.log('Done.');

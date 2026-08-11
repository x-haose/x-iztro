/**
 * Tier 3 golden test generator
 * Coverage: 60 years (1984-2043) × all days × 13 time indices × 1 gender (男)
 * ~284,700 cases stored as SHA-256 hashes in a single CSV file
 */
import { astro } from 'iztro';
import crypto from 'node:crypto';
import { createWriteStream } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const outPath = join(__dirname, 'tier3_hashes.csv');

const START_YEAR = 1984;
const END_YEAR = 2043;
const GENDER = '男';

function daysInMonth(year, month) {
  return new Date(year, month, 0).getDate();
}

function hashAstrolabe(result) {
  const obj = {
    sb: result.earthlyBranchOfSoulPalace,
    bb: result.earthlyBranchOfBodyPalace,
    fc: result.fiveElementsClass,
    ss: result.soul,
    bs: result.body,
    palaces: result.palaces.map(p => ({
      n: p.name,
      hs: p.heavenlyStem,
      eb: p.earthlyBranch,
      ms: p.majorStars.map(s => s.name),
      ns: p.minorStars.map(s => s.name),
      cs: p.changsheng12,
      bo: p.boshi12,
      jq: p.jiangqian12,
      sq: p.suiqian12,
      dr: p.decadal.range,
    })),
  };
  const json = JSON.stringify(obj);
  return crypto.createHash('sha256').update(json).digest('hex');
}

const ws = createWriteStream(outPath, 'utf-8');
ws.write('solar_date,time_index,hash\n');

let totalCases = 0;

for (let year = START_YEAR; year <= END_YEAR; year++) {
  let yearCases = 0;

  for (let month = 1; month <= 12; month++) {
    const maxDay = daysInMonth(year, month);
    for (let day = 1; day <= maxDay; day++) {
      for (let t = 0; t <= 12; t++) {
        const dateStr = `${year}-${month}-${day}`;
        try {
          const result = astro.bySolar(dateStr, t, GENDER, true, 'zh-CN');
          const hash = hashAstrolabe(result);
          ws.write(`${dateStr},${t},${hash}\n`);
          yearCases++;
        } catch {
          // Skip invalid dates
        }
      }
    }
  }

  totalCases += yearCases;
  console.log(`Year ${year}: ${yearCases} cases (total: ${totalCases})`);
}

ws.end(() => {
  console.log(`Done. Total cases: ${totalCases}`);
  console.log(`Output: ${outPath}`);
});

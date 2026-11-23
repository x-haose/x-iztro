// 以使用者的身份引用包的每个入口与主要类型，只做类型检查、不运行
import {
  astro,
  init,
  IztroError,
  KnowledgePack,
  PalaceName,
  Scope,
  solarDatesByBazi,
  util,
  type Astrolabe,
  type AstrolabeData,
  type HoroscopeData,
  type WasmSource,
} from 'x-iztro';
import { MajorStar, type PalaceData } from 'x-iztro/types';

export async function consume(source: WasmSource): Promise<string> {
  await init(source);
  const chart: Astrolabe = astro.bySolar('2000-8-16', 2, 'female', { config: { yearDivide: 'exact' } });
  const data: AstrolabeData = chart.toJSON();
  const soul: PalaceData | undefined = data.palaces.find((p) => p.nameKey === PalaceName.SOUL);
  const horoscope: HoroscopeData = chart.horoscopeOfYear(2027).toJSON();
  const pack = KnowledgePack.builtin();
  try {
    solarDatesByBazi(...chart.rawDates.chineseDate.pillarKeys());
  } catch (e) {
    if (e instanceof IztroError) return e.code;
  }
  return [
    soul?.majorStars.some((s) => s.key === MajorStar.ZIWEI),
    horoscope.yearly.name,
    chart.horoscope('2025-1-1', 0).toText({ through: Scope.YEARLY, knowledge: pack }),
    util.translateChineseDate(chart.rawDates.chineseDate.pillarKeys()),
  ].join('\n');
}

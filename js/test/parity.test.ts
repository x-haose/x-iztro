/**
 * 对齐 iztro 的对象方法与各模块函数。断言值与 Rust / Python / Go 侧同一张盘（2000-8-16 寅时女命）的结果一致，
 * 任一侧行为漂移都会在这里暴露。
 */

import assert from 'node:assert/strict';
import { test } from 'node:test';
import {
  AgeDivide,
  Algorithm,
  astro,
  AstroType,
  Brightness,
  DayDivide,
  data,
  EarthlyBranch,
  FIVE_ELEMENTS_CLASS_NUMBER,
  FiveElementsClass,
  HeavenlyStem,
  HoroscopeDivide,
  HoroscopeStar,
  i18n,
  init,
  IztroError,
  MajorStar,
  Mutagen,
  MUTAGEN_ORDER,
  PalaceName,
  Scope,
  star,
  TextDetail,
  util,
  YearDivide,
  type Astrolabe,
  type ChartConfig,
} from 'x-iztro';
import { lineStartingWith, snapshot } from './helpers.ts';

await init();

const chart = astro.bySolar('2000-8-16', 2, 'female');
const soulOf = (a: Astrolabe) => a.palace(PalaceName.SOUL)!;

test('星耀回指宫位与星盘', () => {
  const s = chart.star(MajorStar.ZIWEI)!;
  assert.equal(s.palace()?.nameKey, PalaceName.SOUL);
  assert.equal(s.oppositePalace()?.index, (s.palace()!.index + 6) % 12);
  assert.equal(s.surroundedPalaces()?.target.index, s.palace()!.index);
});

test('宫位回指星盘', () => {
  assert.equal(soulOf(chart).astrolabe(), chart);
});

test('定位身宫与来因宫', () => {
  const body = chart.palace(PalaceName.BODY)!;
  assert.ok(body.isBodyPalace);
  assert.equal(body.nameKey, PalaceName.CAREER);
  const original = chart.palace(PalaceName.ORIGINAL)!;
  assert.ok(original.isOriginalPalace);
  assert.equal(original.nameKey, PalaceName.SPOUSE);
});

test('命宫壬干四化依次为天梁、紫微、左辅、武曲', () => {
  assert.deepEqual(soulOf(chart).mutagenStars(MUTAGEN_ORDER), ['tianliangMaj', 'ziweiMaj', 'zuofuMin', 'wuquMaj']);
});

test('紫微坐命宫而壬干化权为紫微，故命宫自化权', () => {
  const soul = soulOf(chart);
  assert.ok(soul.selfMutaged(Mutagen.QUAN));
  assert.ok(soul.selfMutagedOneOf());
  assert.ok(!soul.notSelfMutaged());
});

test('复刻 iztro：fliesTo 空列表为假，fliesOneOfTo / notFlyTo 为真', () => {
  const soul = soulOf(chart);
  assert.equal(soul.fliesTo(0, []), false);
  assert.equal(soul.fliesOneOfTo(0, []), true);
  assert.equal(soul.notFlyTo(0, []), true);
});

test('fliesTo 接受宫位对象、索引与宫名', () => {
  const soul = soulOf(chart);
  const target = chart.palaces[0]!;
  const byObject = soul.fliesTo(target, Mutagen.LU);
  assert.equal(soul.fliesTo(0, Mutagen.LU), byObject);
  assert.equal(soul.fliesTo(target.nameKey, Mutagen.LU), byObject);
});

test('mutagedPlaces 不传参取本盘十二宫', () => {
  const soul = soulOf(chart);
  const places = soul.mutagedPlaces();
  assert.equal(places.length, 4);
  assert.deepEqual(places, soul.mutagedPlaces(chart.palaces));
  assert.equal(places[1]?.nameKey, PalaceName.SOUL);
});

test('isEmpty 的 excludeStars', () => {
  const empty = chart.palaces.find((p) => p.isEmpty())!;
  assert.ok(empty.minorStars.length > 0, '该盘的空宫应仍有辅星');
  assert.equal(empty.isEmpty(), true);
  assert.equal(empty.isEmpty([empty.minorStars[0]!.key]), false);
});

test('三方四正按宫名取与判断', () => {
  const sp = chart.surroundedPalaces(PalaceName.SOUL)!;
  assert.equal(sp.target.nameKey, PalaceName.SOUL);
  const first = sp.target.majorStars[0]!.key;
  assert.equal(chart.isSurrounded(PalaceName.SOUL, [first]), true);
  assert.equal(chart.isSurroundedOneOf(PalaceName.SOUL, [first]), true);
  assert.equal(chart.notSurrounded(PalaceName.SOUL, [first]), false);
});

test('流耀判断三者一致', () => {
  const h = chart.horoscope('2024-10-1', 0);
  const probe = ['流禄'];
  const has = h.hasHoroscopeStars(PalaceName.SOUL, Scope.YEARLY, probe);
  assert.equal(h.hasOneOfHoroscopeStars(PalaceName.SOUL, Scope.YEARLY, probe), has);
  assert.notEqual(h.notHaveHoroscopeStars(PalaceName.SOUL, Scope.YEARLY, probe), has);
});

test('运限持有发起它的星盘，免传星盘与显式传盘一致', () => {
  const h = chart.horoscope('2024-10-1', 0);
  assert.equal(h.astrolabe(), chart);
  assert.equal(h.palace(PalaceName.SOUL, Scope.YEARLY), h.palace(PalaceName.SOUL, Scope.YEARLY, chart));
  assert.equal(h.agePalace(), h.agePalace(chart));
  assert.equal(h.surroundPalaces(PalaceName.SOUL, Scope.YEARLY)?.target.index, h.palace(PalaceName.SOUL, Scope.YEARLY)?.index);
});

// ------ 轻量查询 ------

test('生肖与星座与整盘一致，标识随盘携带', () => {
  assert.equal(astro.getZodiacBySolarDate('2000-8-16'), chart.zodiac);
  assert.equal(astro.getSignBySolarDate('2000-8-16'), chart.sign);
  assert.equal(chart.zodiacKey, 'dragon');
  assert.equal(chart.signKey, 'leo');
});

test('农历取星座与阳历同', () => {
  assert.equal(astro.getSignByLunarDate('2000-7-17'), astro.getSignBySolarDate('2000-8-16'));
});

test('命宫主星', () => {
  assert.equal(astro.getMajorStarBySolarDate('2000-8-16', 2), '紫微');
  assert.equal(astro.getMajorStarByLunarDate('2000-7-17', 2), '紫微');
});

test('命宫主星标识与文本同一结果', () => {
  const keys = astro.getMajorStarKeysBySolarDate('2000-8-16', 2);
  assert.ok(keys.length > 0);
  assert.deepEqual(keys, astro.getMajorStarKeysByLunarDate('2000-7-17', 2));
  const soul = soulOf(chart);
  const allowed = new Set([...soul.majorStars, ...soul.oppositePalace()!.majorStars].map((s) => s.key));
  assert.ok(keys.every((k) => allowed.has(k)));
});

test('流耀对照表 50 条', () => {
  const table = data.flowStarCounterparts();
  assert.equal(Object.keys(table).length, 50);
  assert.equal(table.liuchang, 'wenchangMin');
});

test('轻量查询随语言输出（与 Go TestQueryParity 同值）', () => {
  assert.equal(astro.getZodiacBySolarDate('2000-8-16', 'en-US'), 'dragon');
  assert.equal(astro.getSignBySolarDate('2000-8-16', 'en-US'), 'leo');
  assert.equal(astro.getMajorStarBySolarDate('2000-8-16', 2, { language: 'en-US' }), 'emperor');
});

test('轻量查询拒绝非法入参', () => {
  assert.throws(() => astro.getZodiacBySolarDate('2000-2-30'), IztroError);
  assert.throws(() => astro.getMajorStarBySolarDate('2000-8-16', 13 as 0), IztroError);
});

// ------ 工具函数 ------

const YEAR_STEM = HeavenlyStem.GENG;

test('索引工具', () => {
  assert.equal(util.fixIndex(-1), 11);
  assert.equal(util.fixIndex(12), 0);
  assert.equal(util.fixIndex(5, 6), 5);
  assert.equal(util.timeToIndex(0), 0);
  assert.equal(util.timeToIndex(3), 2);
  assert.equal(util.timeToIndex(23), 12);
  assert.equal(util.earthlyBranchIndexToPalaceIndex(EarthlyBranch.YIN), 0);
  assert.equal(util.fixLunarDayIndex(15, 12), 15);
});

test('五行局与整盘一致', () => {
  const soul = soulOf(chart);
  assert.equal(astro.getFiveElementsClass(soul.heavenlyStemKey, soul.earthlyBranchKey), chart.fiveElementsClassKey);
});

test('十二宫名与整盘一致', () => {
  assert.deepEqual(
    astro.getPalaceNames(soulOf(chart).index),
    chart.palaces.map((p) => p.nameKey),
  );
});

test('命身宫推算与整盘一致', () => {
  const lunar = chart.rawDates.lunarDate;
  const monthIndex = util.fixLunarMonthIndex(lunar.lunarMonth, lunar.lunarDay, lunar.isLeap, chart.timeIndex, chart.fixLeap);
  const r = astro.getSoulAndBody(monthIndex, chart.timeIndex, YEAR_STEM);
  assert.equal(r.soulIndex, soulOf(chart).index);
  assert.equal(r.bodyIndex, chart.palace(PalaceName.BODY)!.index);
  assert.equal(r.earthlyBranchOfSoul, chart.earthlyBranchOfSoulPalaceKey);
});

test('亮度与四化查表与排盘写入的字段一致', () => {
  let checked = 0;
  for (const palace of chart.palaces) {
    for (const s of [...palace.majorStars, ...palace.minorStars]) {
      assert.equal(util.getBrightness(s.key, palace.index), s.brightnessKey);
      assert.equal(util.getMutagen(s.key, YEAR_STEM), s.mutagenKey);
      checked += 1;
    }
  }
  assert.ok(checked >= 20);
});

test('天干四化（与 Go TestUtilParity 同值）', () => {
  assert.deepEqual(util.getMutagensByHeavenlyStem(HeavenlyStem.REN), ['tianliangMaj', 'ziweiMaj', 'zuofuMin', 'wuquMaj']);
});

test('工具函数拒绝未知标识', () => {
  assert.throws(() => util.getBrightness('nope', 0), IztroError);
  assert.throws(() => astro.getFiveElementsClass('x' as HeavenlyStem, 'y' as EarthlyBranch), IztroError);
  assert.throws(() => util.timeToIndex(24), IztroError);
});

// ------ 自定义四化与亮度表 ------

/** iztro 文档给出的另一派庚干四化：太阳、武曲、天同、天相 */
const CUSTOM_GENG: ChartConfig = {
  mutagens: { [HeavenlyStem.GENG]: [MajorStar.TAIYANG, MajorStar.WUQU, MajorStar.TIANTONG, MajorStar.TIANXIANG] },
};

const starsWithMutagen = (a: Astrolabe, m: Mutagen) =>
  a.palaces.flatMap((p) => [...p.majorStars, ...p.minorStars]).filter((s) => s.mutagenKey === m).map((s) => s.key);

test('自定义四化表改本命盘', () => {
  const a = astro.bySolar('2000-8-16', 2, 'female', { config: CUSTOM_GENG });
  assert.deepEqual(starsWithMutagen(a, Mutagen.KE), [MajorStar.TIANTONG]);
  assert.deepEqual(starsWithMutagen(a, Mutagen.JI), [MajorStar.TIANXIANG]);
  assert.deepEqual(starsWithMutagen(a, Mutagen.LU), [MajorStar.TAIYANG]);
});

test('默认配置下庚干化科仍是太阴', () => {
  assert.deepEqual(starsWithMutagen(chart, Mutagen.KE), [MajorStar.TAIYIN]);
});

test('自定义四化表不波及其他天干', () => {
  assert.equal(util.getMutagensByHeavenlyStem(HeavenlyStem.GENG, CUSTOM_GENG)[3], MajorStar.TIANXIANG);
  assert.deepEqual(util.getMutagensByHeavenlyStem(HeavenlyStem.REN, CUSTOM_GENG), ['tianliangMaj', 'ziweiMaj', 'zuofuMin', 'wuquMaj']);
});

test('排盘后改动调用方的配置对象，不影响这张盘再发起的计算', () => {
  const config: ChartConfig = { mutagens: { ...CUSTOM_GENG.mutagens } };
  const a = astro.bySolar('2000-8-16', 2, 'female', { config });
  const before = a.horoscope('2030-6-1', 0).yearly.mutagenStarKeys;
  config.mutagens = undefined;
  assert.deepEqual(a.horoscope('2030-6-1', 0).yearly.mutagenStarKeys, before);
});

test('自定义亮度表生效', () => {
  const config: ChartConfig = { brightness: { [MajorStar.TANLANG]: Array(12).fill(Brightness.WANG) } };
  const a = astro.bySolar('2000-8-16', 2, 'female', { config });
  assert.equal(a.star(MajorStar.TANLANG)?.brightnessKey, Brightness.WANG);
  assert.equal(util.getBrightness(MajorStar.TANLANG, 5, config), Brightness.WANG);
});

test('非法覆盖表被拒', () => {
  const bad: ChartConfig[] = [
    { mutagens: { nope: ['a', 'b', 'c', 'd'] } as ChartConfig['mutagens'] },
    { mutagens: { [HeavenlyStem.GENG]: [MajorStar.TAIYANG] } as unknown as ChartConfig['mutagens'] },
    { brightness: { [MajorStar.TANLANG]: ['wang'] } },
  ];
  for (const config of bad) {
    assert.throws(() => astro.bySolar('2000-8-16', 2, 'female', { config }), IztroError);
  }
});

// ------ 天盘 / 地盘 / 人盘与重排 ------

const typed = (astroType: AstroType) => astro.bySolar('2000-8-16', 2, 'female', { config: { astroType } });

test('地盘以身宫为命宫', () => {
  const earth = typed(AstroType.EARTH);
  assert.equal(soulOf(earth).index, chart.palace(PalaceName.BODY)!.index);
  assert.equal(earth.config.astroType, AstroType.EARTH);
});

test('人盘以福德宫为命宫', () => {
  assert.equal(soulOf(typed(AstroType.HUMAN)).index, chart.palace(PalaceName.SPIRIT)!.index);
});

test('以身宫干支重排等同地盘', () => {
  const body = chart.palace(PalaceName.BODY)!;
  const manual = chart.rearranged(body.heavenlyStemKey, body.earthlyBranchKey);
  const earth = typed(AstroType.EARTH);
  assert.equal(manual.fiveElementsClassKey, earth.fiveElementsClassKey);
  assert.deepEqual(manual.palaces.map((p) => p.nameKey), earth.palaces.map((p) => p.nameKey));
  assert.deepEqual(
    manual.palaces.map((p) => p.majorStars.map((s) => s.key)),
    earth.palaces.map((p) => p.majorStars.map((s) => s.key)),
  );
});

test('重排拒绝未知天干', () => {
  assert.throws(() => chart.rearranged('notAStem' as HeavenlyStem, EarthlyBranch.ZI), IztroError);
});

test('重排盘上的运限与文本按重排后的布局计算', () => {
  const a = astro.bySolar('1990-4-23', 2, 'male');
  const re = a.rearranged(HeavenlyStem.GENG, EarthlyBranch.CHEN);
  const plain = a.horoscope('2024-6-1', 0);
  const reH = re.horoscope('2024-6-1', 0);
  assert.notDeepEqual(reH.toJSON(), plain.toJSON());
  assert.notEqual(re.toText(), a.toText());
  assert.notEqual(reH.toText(), plain.toText());
});

// ------ 工具函数收尾与运限默认参数 ------

test('translateChineseDate 与星盘字段一致', () => {
  const c = chart.rawDates.chineseDate;
  const pillars = [c.yearlyKeys, c.monthlyKeys, c.dailyKeys, c.hourlyKeys];
  assert.equal(util.translateChineseDate(pillars), chart.chineseDate);
  assert.equal(util.translateChineseDate(pillars, 'en-US'), 'geng chen - jia shen - bing woo - geng yin');
  assert.throws(() => util.translateChineseDate(pillars.slice(0, 3)), IztroError);
  assert.throws(
    () => util.translateChineseDate([['notAStem' as HeavenlyStem, EarthlyBranch.ZI], ...pillars.slice(1)]),
    IztroError,
  );
});

test('mergeStars 按宫位首尾相接', () => {
  const majors = chart.palaces.map((p) => p.majorStars);
  const minors = chart.palaces.map((p) => p.minorStars);
  const merged = util.mergeStars(majors, minors);
  assert.equal(merged.length, 12);
  merged.forEach((stars, i) => {
    assert.equal(stars.length, majors[i]!.length + minors[i]!.length);
    assert.deepEqual(stars.slice(0, majors[i]!.length), [...majors[i]!]);
  });
  assert.deepEqual(util.mergeStars(), Array.from({ length: 12 }, () => []));
  assert.throws(() => util.mergeStars(majors.slice(0, 11)), RangeError);
});

test('运限两个参数都可省略，取本地时钟的此刻', () => {
  const now = new Date();
  const explicit = chart.horoscope(`${now.getFullYear()}-${now.getMonth() + 1}-${now.getDate()}`, util.timeToIndex(now.getHours()));
  const byNow = chart.horoscope();
  assert.equal(byNow.solarDate, explicit.solarDate);
  assert.equal(byNow.decadal.index, explicit.decadal.index);
  assert.equal(byNow.hourly.index, explicit.hourly.index);
});

test('运限目标可用 Date', () => {
  assert.deepEqual(chart.horoscope(new Date(2024, 9, 1, 13, 20)).toJSON(), chart.horoscope('2024-10-1 13:20:0').toJSON());
});

// ------ star / data / i18n：与 Go parity_check_test.go 同值 ------

const BIRTH = { solarDate: '2000-8-16', timeIndex: 2, gender: 'female', fixLeap: true } as const;

test('单独安星与整盘对应字段一致', () => {
  const start = star.getStartIndex(BIRTH);
  assert.equal(start.ziweiIndex, 4);
  assert.equal(start.tianfuIndex, 8);
  const major = star.getMajorStar(BIRTH);
  const minor = star.getMinorStar(BIRTH);
  chart.palaces.forEach((p, i) => {
    assert.deepEqual(p.majorStars.map((s) => s.key), major[i]!.map((s) => s.key));
    assert.deepEqual(p.minorStars.map((s) => s.key), minor[i]!.map((s) => s.key));
  });
  assert.deepEqual(chart.palaces.map((p) => p.changsheng12Key), star.getChangsheng12(BIRTH));
  assert.deepEqual(chart.palaces.map((p) => p.boshi12Key), star.getBoShi12(BIRTH));
  const yearly12 = star.getYearly12(BIRTH);
  assert.deepEqual(chart.palaces.map((p) => p.suiqian12Key), yearly12.suiqian12);
  assert.deepEqual(chart.palaces.map((p) => p.jiangqian12Key), yearly12.jiangqian12);
});

test('本命层级红鸾用正确标识 hongluan', () => {
  assert.equal(star.getHoroscopeStar(HeavenlyStem.JIA, EarthlyBranch.ZI, Scope.ORIGIN)[1]?.[1]?.key, 'hongluan');
  assert.equal(star.getHoroscopeStar(HeavenlyStem.JIA, EarthlyBranch.ZI, Scope.DECADAL)[1]?.[1]?.key, 'yunluan');
});

test('十二神起始宫位', () => {
  assert.equal(star.getChangesheng12StartIndex(FiveElementsClass.WATER_2), 6);
  assert.equal(star.getJiangqian12StartIndex(EarthlyBranch.ZI), 10);
});

test('低层落宫函数与年系杂耀合并结果一致', () => {
  const yearBranch = chart.rawDates.chineseDate.yearlyKeys[1];
  const yearly = star.getYearlyStarIndex(BIRTH);
  const huagai = star.getHuagaiXianchiIndex(yearBranch);
  assert.deepEqual([huagai.huagaiIndex, huagai.xianchiIndex], [yearly.huagaiIndex, yearly.xianchiIndex]);
  const guGua = star.getGuGuaIndex(yearBranch);
  assert.deepEqual([guGua.guchenIndex, guGua.guasuIndex], [yearly.guchenIndex, yearly.guasuIndex]);
  assert.equal(star.getJieshaAdjIndex(yearBranch), yearly.jieshaAdjIndex);
  assert.equal(star.getDahaoIndex(yearBranch), yearly.dahaoAdjIndex);
  assert.equal(star.getNianjieIndex(yearBranch), yearly.nianjieIndex);
  const tianshi = star.getTianshiTianshangIndex('female', yearBranch, soulOf(chart).index);
  assert.deepEqual([tianshi.tianshangIndex, tianshi.tianshiIndex], [yearly.tianshangIndex, yearly.tianshiIndex]);
});

test('左辅右弼、火铃、按天干的昌曲落在整盘对应宫位', () => {
  const yearBranch = chart.rawDates.chineseDate.yearlyKeys[1];
  const palaceOf = (key: string) => chart.starInPalace(key)![1].index;
  const lunar = chart.rawDates.lunarDate;
  const lunarMonth = util.fixLunarMonthIndex(lunar.lunarMonth, lunar.lunarDay, lunar.isLeap, 2, true) + 1;
  const zuoYou = star.getZuoYouIndex(lunarMonth);
  assert.equal(zuoYou.zuoIndex, palaceOf('zuofuMin'));
  assert.equal(zuoYou.youIndex, palaceOf('youbiMin'));
  const huoLing = star.getHuoLingIndex(yearBranch, 2);
  assert.equal(huoLing.huoIndex, palaceOf('huoxingMin'));
  assert.equal(huoLing.lingIndex, palaceOf('lingxingMin'));
  const decadal = star.getHoroscopeStar(HeavenlyStem.JIA, EarthlyBranch.ZI, Scope.DECADAL);
  const changQu = star.getChangQuIndexByHeavenlyStem(HeavenlyStem.JIA);
  assert.ok(decadal[changQu.changIndex]?.some((s) => s.key === HoroscopeStar.YUN_CHANG));
  assert.ok(decadal[changQu.quIndex]?.some((s) => s.key === HoroscopeStar.YUN_QU));
});

test('按出生数据安的其余星组都在整盘上', () => {
  const where = (key: string) => chart.starInPalace(key)?.[1].index;
  const kuiYue = star.getKuiYueIndex(BIRTH);
  assert.deepEqual([kuiYue.kuiIndex, kuiYue.yueIndex], [where('tiankuiMin'), where('tianyueMin')]);
  const changQu = star.getChangQuIndex(BIRTH);
  assert.deepEqual([changQu.changIndex, changQu.quIndex], [where('wenchangMin'), where('wenquMin')]);
  const kongJie = star.getKongJieIndex(BIRTH);
  assert.deepEqual([kongJie.kongIndex, kongJie.jieIndex], [where('dikongMin'), where('dijieMin')]);
  const lytm = star.getLuYangTuoMaIndex(BIRTH);
  assert.deepEqual(
    [lytm.luIndex, lytm.yangIndex, lytm.tuoIndex, lytm.maIndex],
    [where('lucunMin'), where('qingyangMin'), where('tuoluoMin'), where('tianmaMin')],
  );
  const timely = star.getTimelyStarIndex(BIRTH);
  assert.deepEqual([timely.taifuIndex, timely.fenggaoIndex], [where('taifu'), where('fenggao')]);
  const luanXi = star.getLuanXiIndex(BIRTH);
  assert.deepEqual([luanXi.hongluanIndex, luanXi.tianxiIndex], [where('hongluan'), where('tianxi')]);
  const daily = star.getDailyStarIndex(BIRTH);
  assert.deepEqual(
    [daily.santaiIndex, daily.bazuoIndex, daily.enguangIndex, daily.tianguiIndex],
    [where('santai'), where('bazuo'), where('engguang'), where('tiangui')],
  );
  const monthly = star.getMonthlyStarIndex(BIRTH);
  assert.deepEqual(
    [monthly.yuejieIndex, monthly.tianyaoIndex, monthly.tianxingIndex, monthly.yinshaIndex, monthly.tianyueIndex, monthly.tianwuIndex],
    [where('jieshen'), where('tianyao'), where('tianxing'), where('yinsha'), where('tianyue'), where('tianwu')],
  );
  const adjective = star.getAdjectiveStar(BIRTH);
  chart.palaces.forEach((p, i) => {
    assert.deepEqual(new Set(p.adjectiveStars.map((s) => s.key)), new Set(adjective[i]!.map((s) => s.key)));
  });
});

test('i18n 反查的限定子串消歧', () => {
  assert.equal(i18n.keyOf('horse'), 'horse');
  assert.equal(i18n.keyOf('horse', 'Min'), 'tianmaMin');
  assert.equal(i18n.keyOf('dragon'), 'dragon');
  assert.equal(i18n.keyOf('유시'), 'hourly');
  assert.equal(i18n.keyOf('유시', 'Hour'), 'roosterHour');
  assert.equal(i18n.keyOf('horse', 'Palace'), undefined);
});

test('标识表的规模、次序与逐条可译性', () => {
  const keys = i18n.allKeys();
  assert.equal(keys.length, 260);
  assert.equal(new Set(keys).size, 260);
  assert.deepEqual(keys.slice(0, 2), ['decadal', 'childhood']);
  assert.deepEqual(keys.slice(-2), ['male', 'female']);
  assert.ok(keys.every((k) => i18n.translate(k) !== undefined));
});

test('数据表', () => {
  const info = data.starsInfo();
  assert.equal(Object.keys(info).length, 20);
  assert.equal(info.ziweiMaj?.fiveElements, '土');
  assert.equal(info.ziweiMaj?.yinYang, '阴');
  assert.equal(info.ziweiMaj?.brightness[0], 'wang');
  assert.equal(info.taiyangMaj?.fiveElements, null);
  assert.equal(info.taiyangMaj?.yinYang, null);
  const stems = data.heavenlyStems();
  assert.deepEqual([stems.jiaHeavenly.yinYang, stems.jiaHeavenly.fiveElements, stems.jiaHeavenly.crash], ['阳', '木', 'gengHeavenly']);
  assert.deepEqual(stems.jiaHeavenly.mutagen, ['lianzhenMaj', 'pojunMaj', 'wuquMaj', 'taiyangMaj']);
  assert.equal(stems.wuHeavenly.crash, null);
  const zi = data.earthlyBranches().ziEarthly!;
  assert.equal(zi.crash, 'wuEarthly');
  assert.equal(zi.soul, 'tanlangMaj');
  assert.equal(zi.inside, '胆');
  const c = data.constants();
  assert.deepEqual(c.languages, ['en-US', 'ja-JP', 'ko-KR', 'zh-CN', 'zh-TW', 'vi-VN']);
  assert.equal(c.chineseTime.length, 13);
  assert.equal(c.chineseTime[12], 'lateRatHour');
  assert.deepEqual(c.gender, { male: '阳', female: '阴' });
  assert.equal(c.tigerRule.jiaHeavenly, 'bingHeavenly');
  assert.equal(c.ratRule.jiaHeavenly, 'jiaHeavenly');
  assert.deepEqual(c.fiveElementsClass, FIVE_ELEMENTS_CLASS_NUMBER);
});

test('i18n 往返', () => {
  for (const [lang, want] of [['zh-CN', '紫微'], ['en-US', 'emperor'], ['ko-KR', '자미']] as const) {
    assert.equal(i18n.translate('ziweiMaj', lang), want);
    assert.equal(i18n.keyOf(want), 'ziweiMaj');
  }
  assert.equal(i18n.translate('soulPalace', 'en-US'), 'soul');
  assert.equal(i18n.translate('bodyPalace'), '身宫');
  assert.equal(i18n.translate('nosuchkey'), undefined);
  assert.equal(i18n.keyOf('查无此名'), undefined);
});

test('大限与小限推算与整盘一致', () => {
  const [stem, branch] = chart.rawDates.chineseDate.yearlyKeys;
  const got = astro.getHoroscope(soulOf(chart).index, chart.fiveElementsClassKey, chart.genderKey, stem, branch);
  chart.palaces.forEach((p, i) => {
    assert.deepEqual([...p.ages], got.ages[i]);
    assert.deepEqual(p.decadal, got.decadals[i]);
  });
});

test('宫位自身可取对宫与三方四正', () => {
  const soul = chart.palace('soulPalace')!;
  assert.equal(soul.oppositePalace()?.nameKey, 'surfacePalace');
  const fromPalace = soul.surroundedPalaces()!;
  const fromChart = chart.surroundedPalaces('soulPalace')!;
  const idx = (s: typeof fromPalace) => [s.target, s.opposite, s.wealth, s.career].map((p) => p.index);
  assert.deepEqual(idx(fromPalace), idx(fromChart));
  for (const p of chart.palaces) {
    assert.equal(p.oppositePalace()?.oppositePalace()?.index, p.index);
  }
});

test('配置开关的取值（与 Go TestConfigConstantsParity 同值）', () => {
  assert.deepEqual([YearDivide.NORMAL, YearDivide.EXACT], ['normal', 'exact']);
  assert.deepEqual([HoroscopeDivide.NORMAL, HoroscopeDivide.EXACT], ['normal', 'exact']);
  assert.deepEqual([AgeDivide.NORMAL, AgeDivide.BIRTHDAY], ['normal', 'birthday']);
  assert.deepEqual([DayDivide.FORWARD, DayDivide.CURRENT], ['forward', 'current']);
  assert.deepEqual([Algorithm.DEFAULT, Algorithm.ZHONGZHOU], ['default', 'zhongzhou']);
  assert.deepEqual([AstroType.HEAVEN, AstroType.EARTH, AstroType.HUMAN], ['heaven', 'earth', 'human']);
  const zz = astro.bySolar('2000-8-16', 2, 'female', { config: { algorithm: Algorithm.ZHONGZHOU, yearDivide: YearDivide.EXACT } });
  assert.ok(zz.fiveElementsClass);
  assert.ok(astro.bySolar('2000-8-16', 12, 'female', { config: { dayDivide: DayDivide.CURRENT } }).time);
});

// ------ 自定义四化表贯穿全链路 ------

const custom = () => astro.bySolar('2000-8-16', 2, 'female', { config: CUSTOM_GENG });

test('宫位四化星取自排盘时生效的表', () => {
  const geng = custom().palaces.filter((p) => p.heavenlyStemKey === HeavenlyStem.GENG);
  assert.deepEqual(geng.map((p) => p.nameKey), [PalaceName.SPOUSE]);
  assert.deepEqual(geng[0]!.mutagenStars(MUTAGEN_ORDER), [MajorStar.TAIYANG, MajorStar.WUQU, MajorStar.TIANTONG, MajorStar.TIANXIANG]);
  assert.deepEqual(chart.palaces[geng[0]!.index]!.mutagenStars(Mutagen.JI), [MajorStar.TIANTONG]);
});

test('飞星判断走覆盖后的四化星', () => {
  const geng = custom().palaces.find((p) => p.heavenlyStemKey === HeavenlyStem.GENG)!;
  assert.equal(geng.fliesTo(PalaceName.WEALTH, Mutagen.JI), true);
  assert.equal(geng.notFlyTo(PalaceName.WEALTH, Mutagen.JI), false);
  assert.deepEqual(
    geng.mutagedPlaces().map((p) => p?.nameKey),
    [PalaceName.CHILDREN, PalaceName.WEALTH, PalaceName.HEALTH, PalaceName.WEALTH],
  );
});

test('覆盖表随运限再发起不丢失（2030 年为庚戌年）', () => {
  assert.deepEqual(
    [...custom().horoscope('2030-6-1', 0).yearly.mutagenStarKeys],
    [MajorStar.TAIYANG, MajorStar.WUQU, MajorStar.TIANTONG, MajorStar.TIANXIANG],
  );
});

test('覆盖表随语义化文本再发起不丢失', () => {
  assert.equal(lineStartingWith(custom().toText(), '- 生年四化'), '- 生年四化: 太阳化禄→子女, 武曲化权→财帛, 天同化科→疾厄, 天相化忌→财帛');
  assert.equal(lineStartingWith(chart.toText(), '- 生年四化'), '- 生年四化: 太阳化禄→子女, 武曲化权→财帛, 太阴化科→仆役, 天同化忌→疾厄');
});

test('亮度覆盖表随重排带过去', () => {
  const config: ChartConfig = { brightness: { [MajorStar.TANLANG]: Array(12).fill(Brightness.WANG) } };
  const a = astro.bySolar('2000-8-16', 2, 'female', { config });
  const body = a.palace(PalaceName.BODY)!;
  assert.equal(a.rearranged(body.heavenlyStemKey, body.earthlyBranchKey).star(MajorStar.TANLANG)?.brightnessKey, Brightness.WANG);
});

// ------ 语义化文本与 Rust 快照逐字节一致 ------

for (const language of ['zh-CN', 'en-US'] as const) {
  test(`九类文本与 Rust 快照逐字节相同（${language}）`, () => {
    const a = astro.bySolar('2000-8-16', 2, 'female', { language });
    const soul = soulOf(a);
    const snap = (kind: string) => snapshot(`${kind}_${language}`);
    assert.equal(a.toText(), snap('astrolabe'));
    assert.equal(a.horoscope('2025-1-1', 0).toText(), snap('horoscope'));
    assert.equal(a.patternsToText(), snap('patterns'));
    assert.equal(soul.toText(), snap('palace'));
    assert.equal(soul.surroundedPalaces()!.toText(), snap('surrounded'));
    assert.equal(a.flankingPalaces(PalaceName.SOUL)!.toText(), snap('flanking'));
    assert.equal(a.decadalListToText(), snap('decadals'));
    assert.equal(a.yearlyOverviewToText(2001, 2012), snap('yearlies'));
    assert.equal(a.monthlyListToText(2025), snap('monthlies'));
    const detail = TextDetail.OVERVIEW;
    assert.equal(a.toText({ detail }), snap('astrolabe_overview'));
    assert.equal(a.horoscope('2025-1-1', 0).toText({ detail }), snap('horoscope_overview'));
    assert.equal(soul.toText({ detail }), snap('palace_overview'));
    assert.equal(soul.surroundedPalaces()!.toText({ detail }), snap('surrounded_overview'));
    assert.equal(a.flankingPalaces(PalaceName.SOUL)!.toText({ detail }), snap('flanking_overview'));
  });
}

test('String() 即语义化文本', () => {
  const h = chart.horoscope('2025-1-1', 0);
  assert.equal(String(chart), chart.toText());
  assert.equal(`${h}`, h.toText());
});

test('through 写到指定层级为止，origin 与未知层级报错', () => {
  const h = chart.horoscope('2024-10-1', 0);
  const text = h.toText({ through: Scope.YEARLY });
  for (const heading of ['## 大限 · ', '## 小限 · ', '## 流年 · ']) assert.ok(text.includes(heading));
  for (const heading of ['## 流月 · ', '## 流日 · ', '## 流时 · ']) assert.ok(!text.includes(heading));
  assert.equal(h.toText({ through: Scope.HOURLY }), h.toText());
  for (const bad of [Scope.ORIGIN, 'weekly']) {
    assert.throws(
      () => h.toText({ through: bad as 'yearly' }),
      (e: unknown) => e instanceof IztroError && e.code === 'invalid_argument',
    );
  }
});

test('未起运的大限层 nameKey 为 childhood', () => {
  const h = chart.horoscope('2001-10-1', 0);
  assert.equal(h.decadal.nameKey, 'childhood');
  assert.ok(h.toText().includes('童限'));
  assert.equal(chart.horoscope('2025-1-1', 0).decadal.nameKey, 'decadal');
});

test('运限视角的格局文本非空且不同于本命', () => {
  const text = chart.horoscope('2025-1-1', 0).patternsToText(Scope.YEARLY);
  assert.ok(text);
  assert.notEqual(text, chart.patternsToText());
});

// ------ JSON 导出 ------

test('toJSON 即 iztro 的 JSON：camelCase 键、按语言翻译的值，且是深拷贝', () => {
  const raw = chart.toJSON();
  assert.equal(raw.solarDate, chart.solarDate);
  assert.equal(raw.fiveElementsClassKey, chart.fiveElementsClassKey);
  assert.deepEqual(raw.palaces.map((p) => p.name), chart.palaces.map((p) => p.name));
  assert.deepEqual(raw.palaces[0]!.mutagenStarKeys, [...chart.palaces[0]!.mutagenStarKeys]);
  raw.solarDate = 'changed';
  assert.equal(chart.toJSON().solarDate, chart.solarDate);
});

test('JSON.stringify 往返', () => {
  const text = JSON.stringify(chart);
  assert.ok(text.includes('紫微'));
  assert.deepEqual(JSON.parse(text), chart.toJSON());
  const h = chart.horoscope('2024-10-1', 0);
  assert.deepEqual(JSON.parse(JSON.stringify(h)), h.toJSON());
  assert.deepEqual(h.toJSON().yearly.mutagenStarKeys, [...h.yearly.mutagenStarKeys]);
});

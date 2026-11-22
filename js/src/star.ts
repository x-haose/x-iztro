/**
 * 按出生数据安星（iztro 的 `star` 模块）。
 *
 * 不排整盘、只取某一组星耀时用这些函数。返回的星耀标识与索引都与语言无关，可直接同星盘对象的
 * `*Key` 字段比对。索引一律为宫位索引：0 为寅宫，11 为丑宫。
 */

import type { EarthlyBranch, FiveElementsClass, Gender, HeavenlyStem, Language, Scope, TimeIndex } from './enums.js';
import { query } from './runtime.js';
import { Star } from './star-object.js';
import type { ChartConfig, StarData } from './types.js';

/** 安星所需的出生数据 */
export interface BirthParam {
  /** 阳历日期，如 "2000-8-16" */
  solarDate: string;
  /** 时辰索引 0-12 */
  timeIndex: TimeIndex;
  /** 性别，默认 male；只影响与性别有关的星（如天伤天使的中州派安法） */
  gender?: Gender | undefined;
  /** 是否修正闰月，默认 true */
  fixLeap?: boolean | undefined;
  /** 星名的输出语言，默认 zh-CN */
  language?: Language | undefined;
  /** 排盘配置 */
  config?: ChartConfig | undefined;
}

/** 可从别的干支起五行局的出生数据（中州派地盘、人盘） */
export interface RearrangeableBirthParam extends BirthParam {
  /** 起五行局的天干标识；与 fromBranch 同时给出 */
  fromStem?: HeavenlyStem | undefined;
  /** 起五行局的地支标识；与 fromStem 同时给出 */
  fromBranch?: EarthlyBranch | undefined;
}

function birth(param: RearrangeableBirthParam): Record<string, unknown> {
  return {
    solarDate: param.solarDate,
    timeIndex: param.timeIndex,
    gender: param.gender ?? 'male',
    fixLeap: param.fixLeap ?? true,
    language: param.language ?? 'zh-CN',
    config: param.config,
    fromStem: param.fromStem,
    fromBranch: param.fromBranch,
  };
}

function stars(groups: StarData[][]): Star[][] {
  return groups.map((palace) => palace.map((s) => new Star(s)));
}

/** 紫微、天府的起始宫位索引；给出 fromStem 与 fromBranch 时改用该干支起五行局 */
export function getStartIndex(param: RearrangeableBirthParam): { ziweiIndex: number; tianfuIndex: number } {
  return query('getStartIndex', birth(param)) as { ziweiIndex: number; tianfuIndex: number };
}

/** 禄存、擎羊、陀罗、天马的宫位索引（按年干支） */
export function getLuYangTuoMaIndex(param: BirthParam): {
  luIndex: number;
  yangIndex: number;
  tuoIndex: number;
  maIndex: number;
} {
  return query('getLuYangTuoMaIndex', birth(param)) as ReturnType<typeof getLuYangTuoMaIndex>;
}

/** 天魁、天钺的宫位索引（按年干） */
export function getKuiYueIndex(param: BirthParam): { kuiIndex: number; yueIndex: number } {
  return query('getKuiYueIndex', birth(param)) as { kuiIndex: number; yueIndex: number };
}

/** 文昌、文曲的宫位索引（按时支） */
export function getChangQuIndex(param: BirthParam): { changIndex: number; quIndex: number } {
  return query('getChangQuIndex', birth(param)) as { changIndex: number; quIndex: number };
}

/** 地空、地劫的宫位索引（按时支） */
export function getKongJieIndex(param: BirthParam): { kongIndex: number; jieIndex: number } {
  return query('getKongJieIndex', birth(param)) as { kongIndex: number; jieIndex: number };
}

/** 台辅、封诰的宫位索引（按时支） */
export function getTimelyStarIndex(param: BirthParam): { taifuIndex: number; fenggaoIndex: number } {
  return query('getTimelyStarIndex', birth(param)) as { taifuIndex: number; fenggaoIndex: number };
}

/** 红鸾、天喜的宫位索引（按年支） */
export function getLuanXiIndex(param: BirthParam): { hongluanIndex: number; tianxiIndex: number } {
  return query('getLuanXiIndex', birth(param)) as { hongluanIndex: number; tianxiIndex: number };
}

/** 日系星的宫位索引：三台、八座、恩光、天贵 */
export function getDailyStarIndex(param: BirthParam): {
  santaiIndex: number;
  bazuoIndex: number;
  enguangIndex: number;
  tianguiIndex: number;
} {
  return query('getDailyStarIndex', birth(param)) as ReturnType<typeof getDailyStarIndex>;
}

/** 月系星的宫位索引：解神、天姚、天刑、阴煞、天月、天巫 */
export function getMonthlyStarIndex(param: BirthParam): {
  yuejieIndex: number;
  tianyaoIndex: number;
  tianxingIndex: number;
  yinshaIndex: number;
  tianyueIndex: number;
  tianwuIndex: number;
} {
  return query('getMonthlyStarIndex', birth(param)) as ReturnType<typeof getMonthlyStarIndex>;
}

/** 年系杂耀的宫位索引（一整组，年支按 `horoscopeDivide` 分界） */
export interface YearlyStarIndex {
  /** 咸池 */
  xianchiIndex: number;
  /** 华盖 */
  huagaiIndex: number;
  /** 孤辰 */
  guchenIndex: number;
  /** 寡宿 */
  guasuIndex: number;
  /** 天才 */
  tiancaiIndex: number;
  /** 天寿 */
  tianshouIndex: number;
  /** 天厨 */
  tianchuIndex: number;
  /** 破碎 */
  posuiIndex: number;
  /** 蜚廉 */
  feilianIndex: number;
  /** 龙池 */
  longchiIndex: number;
  /** 凤阁 */
  fenggeIndex: number;
  /** 天哭 */
  tiankuIndex: number;
  /** 天虚 */
  tianxuIndex: number;
  /** 天官 */
  tianguanIndex: number;
  /** 天福 */
  tianfuIndex: number;
  /** 天德 */
  tiandeIndex: number;
  /** 月德 */
  yuedeIndex: number;
  /** 天空 */
  tiankongIndex: number;
  /** 截路 */
  jieluIndex: number;
  /** 空亡 */
  kongwangIndex: number;
  /** 旬空 */
  xunkongIndex: number;
  /** 天伤 */
  tianshangIndex: number;
  /** 天使 */
  tianshiIndex: number;
  /** 截空（中州派） */
  jiekongIndex: number;
  /** 劫杀（中州派） */
  jieshaAdjIndex: number;
  /** 年解 */
  nianjieIndex: number;
  /** 大耗（中州派杂耀） */
  dahaoAdjIndex: number;
}

/** 年系杂耀的宫位索引 */
export function getYearlyStarIndex(param: BirthParam): YearlyStarIndex {
  return query('getYearlyStarIndex', birth(param)) as YearlyStarIndex;
}

/** 十四主星在十二宫的分布，按宫位索引排列 */
export function getMajorStar(param: RearrangeableBirthParam): Star[][] {
  return stars(query('getMajorStar', birth(param)) as StarData[][]);
}

/** 十四辅星在十二宫的分布，按宫位索引排列 */
export function getMinorStar(param: BirthParam): Star[][] {
  return stars(query('getMinorStar', birth(param)) as StarData[][]);
}

/** 杂耀在十二宫的分布，按宫位索引排列 */
export function getAdjectiveStar(param: BirthParam): Star[][] {
  return stars(query('getAdjectiveStar', birth(param)) as StarData[][]);
}

/** 长生十二神标识，从寅宫起按宫位索引排列 */
export function getChangsheng12(param: RearrangeableBirthParam): string[] {
  return query('getChangsheng12', birth(param)) as string[];
}

/** 博士十二神标识，从寅宫起按宫位索引排列 */
export function getBoShi12(param: BirthParam): string[] {
  return query('getBoShi12', birth(param)) as string[];
}

/** 岁前十二神与将前十二神标识，从寅宫起按宫位索引排列；年支按 `horoscopeDivide` 分界 */
export function getYearly12(param: BirthParam): { suiqian12: string[]; jiangqian12: string[] } {
  return query('getYearly12', birth(param)) as { suiqian12: string[]; jiangqian12: string[] };
}

/**
 * 流耀在十二宫的分布：魁钺昌曲禄羊陀马鸾喜，流年层级另加年解。
 *
 * @param stem 该运限层级的天干标识
 * @param branch 该运限层级的地支标识
 * @param scope 运限层级，决定星名（大限为运魁、流年为流魁……）
 */
export function getHoroscopeStar(
  stem: HeavenlyStem,
  branch: EarthlyBranch,
  scope: Scope,
  language: Language = 'zh-CN',
): Star[][] {
  return stars(query('getHoroscopeStar', { stemKey: stem, branchKey: branch, scope, language }) as StarData[][]);
}

/** 长生十二神的起始宫位索引：水二局在申、木三局在亥、金四局在巳、土五局在申、火六局在寅 */
export function getChangesheng12StartIndex(fiveElementsClass: FiveElementsClass): number {
  return query('getChangesheng12StartIndex', { fiveElementsClass }) as number;
}

/** 将前十二神的起始宫位索引：寅午戌年在午、申子辰年在子、巳酉丑年在酉、亥卯未年在卯 */
export function getJiangqian12StartIndex(branch: EarthlyBranch): number {
  return query('getJiangqian12StartIndex', { branchKey: branch }) as number;
}

/**
 * 左辅、右弼的宫位索引（按农历月份）。
 *
 * @param lunarMonth 经闰月修正后的农历月份 1-12，即 `util.fixLunarMonthIndex` 结果加一
 */
export function getZuoYouIndex(lunarMonth: number): { zuoIndex: number; youIndex: number } {
  return query('getZuoYouIndex', { lunarMonth }) as { zuoIndex: number; youIndex: number };
}

/** 火星、铃星的宫位索引（按年支与时辰） */
export function getHuoLingIndex(branch: EarthlyBranch, timeIndex: TimeIndex): { huoIndex: number; lingIndex: number } {
  return query('getHuoLingIndex', { branchKey: branch, timeIndex }) as { huoIndex: number; lingIndex: number };
}

/** 华盖、咸池的宫位索引（按年支） */
export function getHuagaiXianchiIndex(branch: EarthlyBranch): { huagaiIndex: number; xianchiIndex: number } {
  return query('getHuagaiXianchiIndex', { branchKey: branch }) as { huagaiIndex: number; xianchiIndex: number };
}

/** 孤辰、寡宿的宫位索引（按年支） */
export function getGuGuaIndex(branch: EarthlyBranch): { guchenIndex: number; guasuIndex: number } {
  return query('getGuGuaIndex', { branchKey: branch }) as { guchenIndex: number; guasuIndex: number };
}

/** 劫煞（杂耀）的宫位索引（按年支），取值只有 0、3、6、9 */
export function getJieshaAdjIndex(branch: EarthlyBranch): number {
  return query('getJieshaAdjIndex', { branchKey: branch }) as number;
}

/** 大耗（杂耀）的宫位索引（按年支） */
export function getDahaoIndex(branch: EarthlyBranch): number {
  return query('getDahaoIndex', { branchKey: branch }) as number;
}

/** 年解的宫位索引（按年支） */
export function getNianjieIndex(branch: EarthlyBranch): number {
  return query('getNianjieIndex', { branchKey: branch }) as number;
}

/**
 * 天伤、天使的宫位索引（按性别、年支与命宫位置）。
 *
 * 二者夹迁移宫：通行派天伤居仆役位、天使居疾厄位；中州派在阴男阳女时二者对调，由 `config.algorithm` 决定。
 */
export function getTianshiTianshangIndex(
  gender: Gender,
  branch: EarthlyBranch,
  soulIndex: number,
  config?: ChartConfig,
): { tianshangIndex: number; tianshiIndex: number } {
  return query('getTianshiTianshangIndex', { gender, branchKey: branch, soulIndex, config }) as {
    tianshangIndex: number;
    tianshiIndex: number;
  };
}

/** 文昌、文曲的宫位索引（按天干）：运限层级的流昌流曲用这一支，本命盘的昌曲按时支走 `getChangQuIndex` */
export function getChangQuIndexByHeavenlyStem(stem: HeavenlyStem): { changIndex: number; quIndex: number } {
  return query('getChangQuIndexByHeavenlyStem', { stemKey: stem }) as { changIndex: number; quIndex: number };
}

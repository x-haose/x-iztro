/**
 * 两个入口（`node.ts`、`browser.ts`）共有的公开 API；入口只在此之上各自提供 `init`。
 *
 * 模块划分与 iztro 对应：`astro`（排盘与轻量查询）、`util`、`star`、`data`，另有 `i18n` 与知识包、格局、反推。
 */

export * as astro from './astro.js';
export * as util from './util.js';
export * as star from './star.js';
export * as data from './data.js';
export * as i18n from './i18n.js';

export * from './enums.js';
export { IztroError, type ErrorCode } from './errors.js';
export type { WasmSource } from './runtime.js';
export { isInitialized } from './runtime.js';

export { Astrolabe } from './astrolabe.js';
export { Palace, type Decadal, type PalaceTarget } from './palace.js';
export { RawChineseDate, type RawDates } from './raw-dates.js';
export { Star } from './star-object.js';
export { SurroundedPalaces, FlankingPalaces } from './surpalaces.js';
export {
  Horoscope,
  HoroscopeItem,
  AgeItem,
  HoroscopeYearly,
  DecadalListItem,
  YearlyListItem,
  MonthlyListItem,
  type PatternTextOptions,
} from './horoscope.js';
export { PatternHit, PatternStar } from './pattern.js';
export {
  KnowledgePack,
  type Source,
  type StarAttributes,
  type StarEntry,
  type PatternEntry,
  type TextEntry,
  type ConceptEntry,
  type TopicEntry,
} from './knowledge.js';
export {
  solarDatesByBazi,
  reverseChart,
  type Pillar,
  type BirthCandidate,
  type StarPosition,
  type ReverseCriteria,
  type ReverseResult,
} from './reverse.js';
export type { SolarOptions, LunarOptions, SoulAndBody, DecadalsAndAges, Plugin } from './astro.js';
export type { BirthParam, RearrangeableBirthParam, YearlyStarIndex } from './star.js';
export type { StarInfo, HeavenlyStemInfo, EarthlyBranchInfo, Constants } from './data.js';
export type * from './types.js';

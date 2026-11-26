/**
 * 枚举与常量。
 *
 * 全部取值是语言无关的标识（iztro i18n key），与数据对象上的 `key` / `*Key` 字段直接比较，
 * 因此在任何输出语言的星盘上都能正确判断。展示文本读数据对象上对应的翻译字段（如 `Star.name`）。
 *
 * 每个枚举同时是一个常量对象（`MajorStar.ZIWEI`）和同名的取值联合类型（`MajorStar`）。
 */

/** 性别（排盘入参与 `Astrolabe.genderKey`） */
export const Gender = {
  MALE: 'male',
  FEMALE: 'female',
} as const;
/** 性别（排盘入参与 `Astrolabe.genderKey`） */
export type Gender = (typeof Gender)[keyof typeof Gender];

/** 输出语言（排盘入参与 `Astrolabe.language`） */
export const Language = {
  ZH_CN: 'zh-CN',
  ZH_TW: 'zh-TW',
  EN_US: 'en-US',
  JA_JP: 'ja-JP',
  KO_KR: 'ko-KR',
  VI_VN: 'vi-VN',
} as const;
/** 输出语言（排盘入参与 `Astrolabe.language`） */
export type Language = (typeof Language)[keyof typeof Language];

/** 算法派别（`ChartConfig.algorithm`） */
export const Algorithm = {
  DEFAULT: 'default',
  ZHONGZHOU: 'zhongzhou',
} as const;
/** 算法派别（`ChartConfig.algorithm`） */
export type Algorithm = (typeof Algorithm)[keyof typeof Algorithm];

/** 排盘视角（`ChartConfig.astroType`）：中州派的天盘、地盘、人盘 */
export const AstroType = {
  /** 天盘：以命宫干支起五行局，即常规排盘结果 */
  HEAVEN: 'heaven',
  /** 地盘：以身宫干支起五行局，身宫即为新盘的命宫 */
  EARTH: 'earth',
  /** 人盘：以福德宫干支起五行局，福德宫即为新盘的命宫 */
  HUMAN: 'human',
} as const;
/** 排盘视角（`ChartConfig.astroType`）：中州派的天盘、地盘、人盘 */
export type AstroType = (typeof AstroType)[keyof typeof AstroType];

/** 年分界点（`ChartConfig.yearDivide`）：安星年干支按正月初一或立春换年 */
export const YearDivide = {
  NORMAL: 'normal',
  EXACT: 'exact',
} as const;
/** 年分界点（`ChartConfig.yearDivide`）：安星年干支按正月初一或立春换年 */
export type YearDivide = (typeof YearDivide)[keyof typeof YearDivide];

/** 运限分界点（`ChartConfig.horoscopeDivide`）：流年按正月初一或立春换年，流月干与月柱按初一或节气换月 */
export const HoroscopeDivide = {
  /** 年按正月初一，月按初一（五虎遁） */
  NORMAL: 'normal',
  /** 年按立春，月按节气 */
  EXACT: 'exact',
  /** 年按正月初一，月按节气：流月四化交节换干，流月命宫仍按农历月 */
  EXACT_MONTH: 'exactMonth',
} as const;
/** 运限分界点（`ChartConfig.horoscopeDivide`）：流年按正月初一或立春换年，流月干与月柱按初一或节气换月 */
export type HoroscopeDivide = (typeof HoroscopeDivide)[keyof typeof HoroscopeDivide];

/** 虚岁分界点（`ChartConfig.ageDivide`）：跨农历年即加一岁或过生日才加 */
export const AgeDivide = {
  NORMAL: 'normal',
  BIRTHDAY: 'birthday',
} as const;
/** 虚岁分界点（`ChartConfig.ageDivide`）：跨农历年即加一岁或过生日才加 */
export type AgeDivide = (typeof AgeDivide)[keyof typeof AgeDivide];

/** 晚子时归属（`ChartConfig.dayDivide`） */
export const DayDivide = {
  /** 与 iztro 一致：日柱、起紫微的日数与日系杂耀按次日算，生月与年干支仍按当天 */
  FORWARD: 'forward',
  /** 晚子时归当天，按当日早子时排盘 */
  CURRENT: 'current',
  /** 晚子时整个日期归次日，按次日早子时排盘：月底的晚子进下月、除夕的晚子进下一年 */
  FORWARD_CARRY: 'forwardCarry',
} as const;
/** 晚子时归属（`ChartConfig.dayDivide`） */
export type DayDivide = (typeof DayDivide)[keyof typeof DayDivide];

/** 闰月出生时安星用哪个月（`ChartConfig.leapDivide`） */
export const LeapDivide = {
  /** 按排盘入参 fixLeap：打开时闰月十五日及以前按本月、十六日起按下月，关闭时整月按本月 */
  DEFAULT: 'default',
  /** 闰月整月按下月，fixLeap 不起作用 */
  NEXT_MONTH: 'nextMonth',
} as const;
/** 闰月出生时安星用哪个月（`ChartConfig.leapDivide`） */
export type LeapDivide = (typeof LeapDivide)[keyof typeof LeapDivide];

/** 天马的安法（`ChartConfig.tianmaPlacement`） */
export const TianmaPlacement = {
  /** 天马安年马（按生年地支） */
  YEARLY: 'yearly',
  /** 天马安月马（按生月：正五九申、二六十巳、三七十一寅、四八十二亥） */
  MONTHLY: 'monthly',
  /** 天马安年马，另在月马之位安杂耀月天马（`AdjectiveStar.YUEMA_ADJ`） */
  BOTH: 'both',
} as const;
/** 天马的安法（`ChartConfig.tianmaPlacement`） */
export type TianmaPlacement = (typeof TianmaPlacement)[keyof typeof TianmaPlacement];

/** 长生十二神的排列方向（`ChartConfig.changshengDirection`） */
export const ChangshengDirection = {
  /** 阳男阴女顺行、阴男阳女逆行 */
  YIN_YANG: 'yinYang',
  /** 男命顺行、女命逆行 */
  GENDER: 'gender',
  /** 一律顺行 */
  CLOCKWISE: 'clockwise',
} as const;
/** 长生十二神的排列方向（`ChartConfig.changshengDirection`） */
export type ChangshengDirection = (typeof ChangshengDirection)[keyof typeof ChangshengDirection];

/** 天伤、天使的安法（`ChartConfig.tianshangTianshi`） */
export const TianshangTianshi = {
  /** 与 iztro 一致：天伤在仆役、天使在疾厄；中州派阴男阳女两者对调 */
  DEFAULT: 'default',
  /** 天使在仆役、天伤在疾厄，不分男女与派别 */
  TIANSHI_FRIENDS: 'tianshiFriends',
} as const;
/** 天伤、天使的安法（`ChartConfig.tianshangTianshi`） */
export type TianshangTianshi = (typeof TianshangTianshi)[keyof typeof TianshangTianshi];

/** 天巫的安法（`ChartConfig.tianwuPlacement`），按生月每四个月一轮 */
export const TianwuPlacement = {
  /** 正五九巳、二六十申、三七十一寅、四八十二亥 */
  SI_SHEN_YIN_HAI: 'siShenYinHai',
  /** 正五九巳、二六十申、三七十一亥、四八十二寅 */
  SI_SHEN_HAI_YIN: 'siShenHaiYin',
} as const;
/** 天巫的安法（`ChartConfig.tianwuPlacement`） */
export type TianwuPlacement = (typeof TianwuPlacement)[keyof typeof TianwuPlacement];

/** 岁前、将前十二神安在哪些盘上（`ChartConfig.yearly12Placement`） */
export const Yearly12Placement = {
  /** 本命盘与流年都安 */
  NATAL_AND_YEARLY: 'natalAndYearly',
  /** 只在流年安；本命各宫的 suiqian12、jiangqian12 及其标识为空串 */
  YEARLY_ONLY: 'yearlyOnly',
} as const;
/** 岁前、将前十二神安在哪些盘上（`ChartConfig.yearly12Placement`） */
export type Yearly12Placement = (typeof Yearly12Placement)[keyof typeof Yearly12Placement];

/** 来因宫的标法（`ChartConfig.originalPalace`） */
export const OriginalPalaceMarking = {
  /** 只标寅至亥十宫里宫干等于年干的那一宫 */
  PRIMARY: 'primary',
  /** 另把辛年丑宫、壬年子宫标为辅来因宫（`Palace.isSecondaryOriginalPalace`） */
  WITH_SECONDARY: 'withSecondary',
} as const;
/** 来因宫的标法（`ChartConfig.originalPalace`） */
export type OriginalPalaceMarking = (typeof OriginalPalaceMarking)[keyof typeof OriginalPalaceMarking];

/** 推斗君时生月的取法（`ChartConfig.douJunMonth`） */
export const DouJunMonth = {
  /** 与 iztro 一致：fixLeap 打开时闰月十六日起作下月，晚子时也算 */
  DEFAULT: 'default',
  /** 与本命盘同一规则：晚子时（dayDivide=forward）不把闰月十六日起作下月 */
  NATAL: 'natal',
} as const;
/** 推斗君时生月的取法（`ChartConfig.douJunMonth`） */
export type DouJunMonth = (typeof DouJunMonth)[keyof typeof DouJunMonth];

/** 大限天干的取法（`ChartConfig.decadalStem`） */
export const DecadalStem = {
  /** 与 iztro 一致：所在宫的宫干 */
  PALACE: 'palace',
  /** 命宫宫干起，按天干次序随大限方向连续排（顺行加一干、逆行减一干） */
  CONTINUOUS: 'continuous',
} as const;
/** 大限天干的取法（`ChartConfig.decadalStem`） */
export type DecadalStem = (typeof DecadalStem)[keyof typeof DecadalStem];

/** 童限落在哪一宫（`ChartConfig.childhoodPalace`） */
export const ChildhoodPalace = {
  /** 与 iztro 一致：依次落命宫、财帛、疾厄、夫妻、福德、官禄 */
  SEQUENCE: 'sequence',
  /** 一律落夫妻宫，四化取夫妻宫干 */
  SPOUSE: 'spouse',
} as const;
/** 童限落在哪一宫（`ChartConfig.childhoodPalace`） */
export type ChildhoodPalace = (typeof ChildhoodPalace)[keyof typeof ChildhoodPalace];

/** 小限是否起四化（`ChartConfig.ageMutagens`） */
export const AgeMutagens = {
  /** 与 iztro 一致：取小限宫干起四化 */
  PALACE_STEM: 'palaceStem',
  /** 不起四化 */
  NONE: 'none',
} as const;
/** 小限是否起四化（`ChartConfig.ageMutagens`） */
export type AgeMutagens = (typeof AgeMutagens)[keyof typeof AgeMutagens];

/** 流年命宫取哪一宫（`ChartConfig.yearlyPalace`） */
export const YearlyPalace = {
  /** 与 iztro 一致：太岁宫（流年年支所在宫） */
  TAISUI: 'taisui',
  /** 小限宫；流年干支、流耀、岁前将前与斗君仍按流年年支与太岁宫 */
  AGE: 'age',
} as const;
/** 流年命宫取哪一宫（`ChartConfig.yearlyPalace`） */
export type YearlyPalace = (typeof YearlyPalace)[keyof typeof YearlyPalace];

/** 运限各层四化取哪个干（`ChartConfig.scopeMutagenStem`） */
export const ScopeMutagenStem = {
  /** 与 iztro 一致：该层自身的干 */
  SCOPE: 'scope',
  /** 该层命宫的本命宫干 */
  SCOPE_SOUL_PALACE: 'scopeSoulPalace',
} as const;
/** 运限各层四化取哪个干（`ChartConfig.scopeMutagenStem`） */
export type ScopeMutagenStem = (typeof ScopeMutagenStem)[keyof typeof ScopeMutagenStem];

/** 流月命宫的起法（`ChartConfig.monthlyPalace`） */
export const MonthlyPalace = {
  /** 与 iztro 一致：斗君为正月 */
  DOU_JUN: 'douJun',
  /** 正月固定在寅宫，每月落在月建所在宫 */
  MONTH_BRANCH: 'monthBranch',
} as const;
/** 流月命宫的起法（`ChartConfig.monthlyPalace`） */
export type MonthlyPalace = (typeof MonthlyPalace)[keyof typeof MonthlyPalace];

/** 流日命宫的起法（`ChartConfig.dailyPalace`） */
export const DailyPalace = {
  /** 与 iztro 一致：初一在流月命宫 */
  FROM_MONTHLY: 'fromMonthly',
  /** 初一在流月命宫的下一宫 */
  AFTER_MONTHLY: 'afterMonthly',
} as const;
/** 流日命宫的起法（`ChartConfig.dailyPalace`） */
export type DailyPalace = (typeof DailyPalace)[keyof typeof DailyPalace];

/** 闰月里流日的数法（`ChartConfig.leapDaily`） */
export const LeapDaily = {
  /** 与 iztro 一致：闰月前半月从流月命宫重新起初一 */
  DEFAULT: 'default',
  /** 闰月前半月接着同号本月的日数连续顺数；十六日起同默认 */
  CONTINUOUS: 'continuous',
} as const;
/** 闰月里流日的数法（`ChartConfig.leapDaily`） */
export type LeapDaily = (typeof LeapDaily)[keyof typeof LeapDaily];

/** 流时命宫的起法（`ChartConfig.hourlyPalace`） */
export const HourlyPalace = {
  /** 与 iztro 一致：子时在流日命宫 */
  FROM_DAILY: 'fromDaily',
  /** 按时支落宫：子时在子宫、丑时在丑宫 */
  BRANCH: 'branch',
} as const;
/** 流时命宫的起法（`ChartConfig.hourlyPalace`） */
export type HourlyPalace = (typeof HourlyPalace)[keyof typeof HourlyPalace];

/** 截空与旬空的安法（`ChartConfig.voidPlacement`） */
export const VoidPlacement = {
  /** 与 iztro 一致：默认派安截路（阳宫）与空亡（阴宫），中州派只在正空安截空；旬空只安正空 */
  DEFAULT: 'default',
  /** 截空、旬空各安正空与傍空两宫：正空安截空、旬空，傍空安傍截空、傍旬空；默认派不再安截路、空亡 */
  ZHENG_PANG: 'zhengPang',
} as const;
/** 截空与旬空的安法（`ChartConfig.voidPlacement`） */
export type VoidPlacement = (typeof VoidPlacement)[keyof typeof VoidPlacement];

/** 天干（对应 `heavenlyStemKey` 字段） */
export const HeavenlyStem = {
  JIA: 'jiaHeavenly',
  YI: 'yiHeavenly',
  BING: 'bingHeavenly',
  DING: 'dingHeavenly',
  WU: 'wuHeavenly',
  JI: 'jiHeavenly',
  GENG: 'gengHeavenly',
  XIN: 'xinHeavenly',
  REN: 'renHeavenly',
  GUI: 'guiHeavenly',
} as const;
/** 天干（对应 `heavenlyStemKey` 字段） */
export type HeavenlyStem = (typeof HeavenlyStem)[keyof typeof HeavenlyStem];

/** 地支（对应 `earthlyBranchKey` 字段） */
export const EarthlyBranch = {
  ZI: 'ziEarthly',
  CHOU: 'chouEarthly',
  YIN: 'yinEarthly',
  MAO: 'maoEarthly',
  CHEN: 'chenEarthly',
  SI: 'siEarthly',
  WU: 'wuEarthly',
  WEI: 'weiEarthly',
  SHEN: 'shenEarthly',
  YOU: 'youEarthly',
  XU: 'xuEarthly',
  HAI: 'haiEarthly',
} as const;
/** 地支（对应 `earthlyBranchKey` 字段） */
export type EarthlyBranch = (typeof EarthlyBranch)[keyof typeof EarthlyBranch];

/** 十二宫（对应 `Palace.nameKey`，可直接传入 `Astrolabe.palace()` 等查询方法） */
export const PalaceName = {
  /** 命宫 */
  SOUL: 'soulPalace',
  /** 兄弟 */
  SIBLINGS: 'siblingsPalace',
  /** 夫妻 */
  SPOUSE: 'spousePalace',
  /** 子女 */
  CHILDREN: 'childrenPalace',
  /** 财帛 */
  WEALTH: 'wealthPalace',
  /** 疾厄 */
  HEALTH: 'healthPalace',
  /** 迁移 */
  SURFACE: 'surfacePalace',
  /** 仆役（又名交友） */
  FRIENDS: 'friendsPalace',
  /** 官禄 */
  CAREER: 'careerPalace',
  /** 田宅 */
  PROPERTY: 'propertyPalace',
  /** 福德 */
  SPIRIT: 'spiritPalace',
  /** 父母 */
  PARENTS: 'parentsPalace',
  /** 身宫。不是十二宫之一，而是「被标记为身宫的那一宫」，供查询方法定位使用 */
  BODY: 'bodyPalace',
  /** 来因宫。同上，定位被标记为来因宫的那一宫；一张盘可能没有 */
  ORIGINAL: 'originalPalace',
} as const;
/** 十二宫（对应 `Palace.nameKey`，可直接传入 `Astrolabe.palace()` 等查询方法） */
export type PalaceName = (typeof PalaceName)[keyof typeof PalaceName];

/**
 * 五行局（对应 `Astrolabe.fiveElementsClassKey`）。
 *
 * 局数见 `FIVE_ELEMENTS_CLASS_NUMBER`，局数决定紫微起宫与大限起运岁数。
 */
export const FiveElementsClass = {
  /** 水二局 */
  WATER_2: 'water2nd',
  /** 木三局 */
  WOOD_3: 'wood3rd',
  /** 金四局 */
  METAL_4: 'metal4th',
  /** 土五局 */
  EARTH_5: 'earth5th',
  /** 火六局 */
  FIRE_6: 'fire6th',
} as const;
/** 五行局（对应 `Astrolabe.fiveElementsClassKey`）。 */
export type FiveElementsClass = (typeof FiveElementsClass)[keyof typeof FiveElementsClass];

/** 四化（对应 `mutagenKey` 字段，可传入 `Palace.hasMutagen()` 等方法） */
export const Mutagen = {
  /** 化禄 */
  LU: 'sihuaLu',
  /** 化权 */
  QUAN: 'sihuaQuan',
  /** 化科 */
  KE: 'sihuaKe',
  /** 化忌 */
  JI: 'sihuaJi',
} as const;
/** 四化（对应 `mutagenKey` 字段，可传入 `Palace.hasMutagen()` 等方法） */
export type Mutagen = (typeof Mutagen)[keyof typeof Mutagen];

/** 星耀亮度（对应 `Star.brightnessKey`） */
export const Brightness = {
  /** 庙 */
  MIAO: 'miao',
  /** 旺 */
  WANG: 'wang',
  /** 得 */
  DE: 'de',
  /** 利 */
  LI: 'li',
  /** 平 */
  PING: 'ping',
  /** 不 */
  BU: 'bu',
  /** 陷 */
  XIAN: 'xian',
  /** 闲（xián）：排在平之下、陷之上；iztro 无此级，只出现在自定义亮度表里 */
  XIAN2: 'xian2',
  /** 地：照录王亭之《安星法及推断实例》庙陷总表，原书未定义；iztro 无此级，只出现在自定义亮度表里 */
  DI: 'di',
} as const;
/** 星耀亮度（对应 `Star.brightnessKey`） */
export type Brightness = (typeof Brightness)[keyof typeof Brightness];

/** 星耀类型（`Star.type`） */
export const StarType = {
  /** 主星 */
  MAJOR: 'major',
  /** 吉星 */
  SOFT: 'soft',
  /** 煞星 */
  TOUGH: 'tough',
  /** 杂耀 */
  ADJECTIVE: 'adjective',
  /** 桃花星 */
  FLOWER: 'flower',
  /** 解神类 */
  HELPER: 'helper',
  /** 禄存 */
  LUCUN: 'lucun',
  /** 天马 */
  TIANMA: 'tianma',
} as const;
/** 星耀类型（`Star.type`） */
export type StarType = (typeof StarType)[keyof typeof StarType];

/** 作用范围（`Star.scope`、运限查询方法的 scope 参数与 `Horoscope.toText` 的 through 参数） */
export const Scope = {
  /** 本命 */
  ORIGIN: 'origin',
  /** 大限 */
  DECADAL: 'decadal',
  /** 流年 */
  YEARLY: 'yearly',
  /** 流月 */
  MONTHLY: 'monthly',
  /** 流日 */
  DAILY: 'daily',
  /** 流时 */
  HOURLY: 'hourly',
} as const;
/** 作用范围（`Star.scope`、运限查询方法的 scope 参数与 `Horoscope.toText` 的 through 参数） */
export type Scope = (typeof Scope)[keyof typeof Scope];

/** 流月分段（`MonthlyListItem.part`）：闰月在修正闰月的盘上拆成前后两段 */
export const MonthPart = {
  /** 整月一段（非闰月，或闰月但不拆分） */
  NORMAL: 'normal',
  /** 闰月前半段（初一至十五） */
  FIRST: 'first',
  /** 闰月后半段（十六至月末） */
  SECOND: 'second',
} as const;
/** 流月分段（`MonthlyListItem.part`）：闰月在修正闰月的盘上拆成前后两段 */
export type MonthPart = (typeof MonthPart)[keyof typeof MonthPart];

/** 十四主星（对应 `Star.key`） */
export const MajorStar = {
  /** 紫微 */
  ZIWEI: 'ziweiMaj',
  /** 天机 */
  TIANJI: 'tianjiMaj',
  /** 太阳 */
  TAIYANG: 'taiyangMaj',
  /** 武曲 */
  WUQU: 'wuquMaj',
  /** 天同 */
  TIANTONG: 'tiantongMaj',
  /** 廉贞 */
  LIANZHEN: 'lianzhenMaj',
  /** 天府 */
  TIANFU: 'tianfuMaj',
  /** 太阴 */
  TAIYIN: 'taiyinMaj',
  /** 贪狼 */
  TANLANG: 'tanlangMaj',
  /** 巨门 */
  JUMEN: 'jumenMaj',
  /** 天相 */
  TIANXIANG: 'tianxiangMaj',
  /** 天梁 */
  TIANLIANG: 'tianliangMaj',
  /** 七杀 */
  QISHA: 'qishaMaj',
  /** 破军 */
  POJUN: 'pojunMaj',
} as const;
/** 十四主星（对应 `Star.key`） */
export type MajorStar = (typeof MajorStar)[keyof typeof MajorStar];

/** 十四辅星（对应 `Star.key`） */
export const MinorStar = {
  /** 左辅 */
  ZUOFU: 'zuofuMin',
  /** 右弼 */
  YOUBI: 'youbiMin',
  /** 文昌 */
  WENCHANG: 'wenchangMin',
  /** 文曲 */
  WENQU: 'wenquMin',
  /** 禄存 */
  LUCUN: 'lucunMin',
  /** 天马 */
  TIANMA: 'tianmaMin',
  /** 擎羊 */
  QINGYANG: 'qingyangMin',
  /** 陀罗 */
  TUOLUO: 'tuoluoMin',
  /** 火星 */
  HUOXING: 'huoxingMin',
  /** 铃星 */
  LINGXING: 'lingxingMin',
  /** 天魁 */
  TIANKUI: 'tiankuiMin',
  /** 天钺 */
  TIANYUE: 'tianyueMin',
  /** 地空 */
  DIKONG: 'dikongMin',
  /** 地劫 */
  DIJIE: 'dijieMin',
} as const;
/** 十四辅星（对应 `Star.key`） */
export type MinorStar = (typeof MinorStar)[keyof typeof MinorStar];

/** 杂耀（对应 `Star.key`） */
export const AdjectiveStar = {
  /** 劫杀（中州派） */
  JIESHA: 'jieshaAdj',
  /** 天空 */
  TIANKONG: 'tiankong',
  /** 天刑 */
  TIANXING: 'tianxing',
  /** 天姚 */
  TIANYAO: 'tianyao',
  /** 解神 */
  JIESHEN: 'jieshen',
  /** 阴煞 */
  YINSHA: 'yinsha',
  /** 天喜 */
  TIANXI: 'tianxi',
  /** 天官 */
  TIANGUAN: 'tianguan',
  /** 天福 */
  TIANFU: 'tianfu',
  /** 天哭 */
  TIANKU: 'tianku',
  /** 天虚 */
  TIANXU: 'tianxu',
  /** 龙池 */
  LONGCHI: 'longchi',
  /** 凤阁 */
  FENGGE: 'fengge',
  /** 红鸾 */
  HONGLUAN: 'hongluan',
  /** 孤辰 */
  GUCHEN: 'guchen',
  /** 寡宿 */
  GUASU: 'guasu',
  /** 蜚廉 */
  FEILIAN: 'feilian',
  /** 破碎 */
  POSUI: 'posui',
  /** 台辅 */
  TAIFU: 'taifu',
  /** 封诰 */
  FENGGAO: 'fenggao',
  /** 天巫 */
  TIANWU: 'tianwu',
  /** 天月 */
  TIANYUE: 'tianyue',
  /** 三台 */
  SANTAI: 'santai',
  /** 八座 */
  BAZUO: 'bazuo',
  /** 恩光 */
  ENGGUANG: 'engguang',
  /** 天贵 */
  TIANGUI: 'tiangui',
  /** 天才 */
  TIANCAI: 'tiancai',
  /** 天寿 */
  TIANSHOU: 'tianshou',
  /** 截空（中州派） */
  JIEKONG: 'jiekong',
  /** 旬空 */
  XUNKONG: 'xunkong',
  /** 旬中 */
  XUNZHONG: 'xunzhong',
  /** 空亡 */
  KONGWANG: 'kongwang',
  /** 截路 */
  JIELU: 'jielu',
  /** 傍截空：截空两宫中的傍空一宫（`VoidPlacement.ZHENG_PANG`） */
  PANG_JIEKONG: 'pangJiekong',
  /** 傍旬空：旬空两宫中的傍空一宫（`VoidPlacement.ZHENG_PANG`） */
  PANG_XUNKONG: 'pangXunkong',
  /** 月天马：按生月安的天马（书中称月马），只在 `TianmaPlacement.BOTH` 时安放 */
  YUEMA_ADJ: 'yuemaAdj',
  /** 月德 */
  YUEDE: 'yuede',
  /** 天伤 */
  TIANSHANG: 'tianshang',
  /** 天使 */
  TIANSHI: 'tianshi',
  /** 天厨 */
  TIANCHU: 'tianchu',
  /** 年解 */
  NIANJIE: 'nianjie',
  /** 咸池 */
  XIANCHI: 'xianchi',
  /** 华盖 */
  HUAGAI: 'huagai',
  /** 天德 */
  TIANDE: 'tiande',
  /** 龙德（中州派） */
  LONGDE: 'longde',
  /** 大耗（中州派杂耀） */
  DAHAO: 'dahao',
} as const;
/** 杂耀（对应 `Star.key`） */
export type AdjectiveStar = (typeof AdjectiveStar)[keyof typeof AdjectiveStar];

/** 长生十二神（对应 `Palace.changsheng12Key`） */
export const Changsheng12 = {
  /** 长生 */
  CHANGSHENG: 'changsheng',
  /** 沐浴 */
  MUYU: 'muyu',
  /** 冠带 */
  GUANDAI: 'guandai',
  /** 临官 */
  LINGUAN: 'linguan',
  /** 帝旺 */
  DIWANG: 'diwang',
  /** 衰 */
  SHUAI: 'shuai',
  /** 病 */
  BING: 'bing',
  /** 死 */
  SI: 'si',
  /** 墓 */
  MU: 'mu',
  /** 绝 */
  JUE: 'jue',
  /** 胎 */
  TAI: 'tai',
  /** 养 */
  YANG: 'yang',
} as const;
/** 长生十二神（对应 `Palace.changsheng12Key`） */
export type Changsheng12 = (typeof Changsheng12)[keyof typeof Changsheng12];

/** 博士十二神（对应 `Palace.boshi12Key`） */
export const Boshi12 = {
  /** 博士 */
  BOSHI: 'boshi',
  /** 力士 */
  LISHI: 'lishi',
  /** 青龙 */
  QINGLONG: 'qinglong',
  /** 小耗 */
  XIAOHAO: 'xiaohao',
  /** 将军 */
  JIANGJUN: 'jiangjun',
  /** 奏书 */
  ZHOUSHU: 'zhoushu',
  /** 飞廉 */
  FEILIAN: 'faylian',
  /** 喜神 */
  XISHEN: 'xishen',
  /** 病符 */
  BINGFU: 'bingfu',
  /** 大耗 */
  DAHAO: 'dahao',
  /** 伏兵 */
  FUBING: 'fubing',
  /** 官府 */
  GUANFU: 'guanfu',
} as const;
/** 博士十二神（对应 `Palace.boshi12Key`） */
export type Boshi12 = (typeof Boshi12)[keyof typeof Boshi12];

/** 岁前十二神（对应 `Palace.suiqian12Key`） */
export const Suiqian12 = {
  /** 岁建 */
  SUIJIAN: 'suijian',
  /** 晦气 */
  HUIQI: 'huiqi',
  /** 丧门 */
  SANGMEN: 'sangmen',
  /** 贯索 */
  GUANSUO: 'guansuo',
  /** 官符 */
  GWANFU: 'gwanfu',
  /** 小耗 */
  XIAOHAO: 'xiaohao',
  /** 大耗 */
  DAHAO: 'dahao',
  /** 岁破（中州派） */
  SUIPO: 'suipo',
  /** 龙德 */
  LONGDE: 'longde',
  /** 白虎 */
  BAIHU: 'baihu',
  /** 天德 */
  TIANDE: 'tiande',
  /** 吊客 */
  DIAOKE: 'diaoke',
  /** 病符 */
  BINGFU: 'bingfu',
} as const;
/** 岁前十二神（对应 `Palace.suiqian12Key`） */
export type Suiqian12 = (typeof Suiqian12)[keyof typeof Suiqian12];

/** 将前十二神（对应 `Palace.jiangqian12Key`） */
export const Jiangqian12 = {
  /** 将星 */
  JIANGXING: 'jiangxing',
  /** 攀鞍 */
  PANAN: 'panan',
  /** 岁驿 */
  SUIYI: 'suiyi',
  /** 息神 */
  XISHEN: 'xiishen',
  /** 华盖 */
  HUAGAI: 'huagai',
  /** 劫煞 */
  JIESHA: 'jiesha',
  /** 灾煞 */
  ZHAISHA: 'zhaisha',
  /** 天煞 */
  TIANSHA: 'tiansha',
  /** 指背 */
  ZHIBEI: 'zhibei',
  /** 咸池 */
  XIANCHI: 'xianchi',
  /** 月煞 */
  YUESHA: 'yuesha',
  /** 亡神 */
  WANGSHEN: 'wangshen',
} as const;
/** 将前十二神（对应 `Palace.jiangqian12Key`） */
export type Jiangqian12 = (typeof Jiangqian12)[keyof typeof Jiangqian12];

/**
 * 流耀（对应运限对象 `stars` 里的 `Star.key`）。
 *
 * 同一颗星在每个运限层级各有一个标识与名称，前缀标出层级：
 * `yun*` 大限、`liu*` 流年、`yue*` 流月、`ri*` 流日、`shi*` 流时。
 * 本命层级的对应星耀在 `MinorStar` 与 `AdjectiveStar` 里。
 */
export const HoroscopeStar = {
  // 大限
  /** 运魁（大限层级的天魁） */
  YUN_KUI: 'yunkui',
  /** 运钺（大限层级的天钺） */
  YUN_YUE: 'yunyue',
  /** 运昌（大限层级的文昌） */
  YUN_CHANG: 'yunchang',
  /** 运曲（大限层级的文曲） */
  YUN_QU: 'yunqu',
  /** 运鸾（大限层级的红鸾） */
  YUN_LUAN: 'yunluan',
  /** 运喜（大限层级的天喜） */
  YUN_XI: 'yunxi',
  /** 运禄（大限层级的禄存） */
  YUN_LU: 'yunlu',
  /** 运羊（大限层级的擎羊） */
  YUN_YANG: 'yunyang',
  /** 运陀（大限层级的陀罗） */
  YUN_TUO: 'yuntuo',
  /** 运马（大限层级的天马） */
  YUN_MA: 'yunma',
  // 流年
  /** 流魁（流年层级的天魁） */
  LIU_KUI: 'liukui',
  /** 流钺（流年层级的天钺） */
  LIU_YUE: 'liuyue',
  /** 流昌（流年层级的文昌） */
  LIU_CHANG: 'liuchang',
  /** 流曲（流年层级的文曲） */
  LIU_QU: 'liuqu',
  /** 流鸾（流年层级的红鸾） */
  LIU_LUAN: 'liuluan',
  /** 流喜（流年层级的天喜） */
  LIU_XI: 'liuxi',
  /** 流禄（流年层级的禄存） */
  LIU_LU: 'liulu',
  /** 流羊（流年层级的擎羊） */
  LIU_YANG: 'liuyang',
  /** 流陀（流年层级的陀罗） */
  LIU_TUO: 'liutuo',
  /** 流马（流年层级的天马） */
  LIU_MA: 'liuma',
  // 流月
  /** 月魁（流月层级的天魁） */
  YUE_KUI: 'yuekui',
  /** 月钺（流月层级的天钺） */
  YUE_YUE: 'yueyue',
  /** 月昌（流月层级的文昌） */
  YUE_CHANG: 'yuechang',
  /** 月曲（流月层级的文曲） */
  YUE_QU: 'yuequ',
  /** 月鸾（流月层级的红鸾） */
  YUE_LUAN: 'yueluan',
  /** 月喜（流月层级的天喜） */
  YUE_XI: 'yuexi',
  /** 月禄（流月层级的禄存） */
  YUE_LU: 'yuelu',
  /** 月羊（流月层级的擎羊） */
  YUE_YANG: 'yueyang',
  /** 月陀（流月层级的陀罗） */
  YUE_TUO: 'yuetuo',
  /** 月马（流月层级的天马） */
  YUE_MA: 'yuema',
  // 流日
  /** 日魁（流日层级的天魁） */
  RI_KUI: 'rikui',
  /** 日钺（流日层级的天钺） */
  RI_YUE: 'riyue',
  /** 日昌（流日层级的文昌） */
  RI_CHANG: 'richang',
  /** 日曲（流日层级的文曲） */
  RI_QU: 'riqu',
  /** 日鸾（流日层级的红鸾） */
  RI_LUAN: 'riluan',
  /** 日喜（流日层级的天喜） */
  RI_XI: 'rixi',
  /** 日禄（流日层级的禄存） */
  RI_LU: 'rilu',
  /** 日羊（流日层级的擎羊） */
  RI_YANG: 'riyang',
  /** 日陀（流日层级的陀罗） */
  RI_TUO: 'rituo',
  /** 日马（流日层级的天马） */
  RI_MA: 'rima',
  // 流时
  /** 时魁（流时层级的天魁） */
  SHI_KUI: 'shikui',
  /** 时钺（流时层级的天钺） */
  SHI_YUE: 'shiyue',
  /** 时昌（流时层级的文昌） */
  SHI_CHANG: 'shichang',
  /** 时曲（流时层级的文曲） */
  SHI_QU: 'shiqu',
  /** 时鸾（流时层级的红鸾） */
  SHI_LUAN: 'shiluan',
  /** 时喜（流时层级的天喜） */
  SHI_XI: 'shixi',
  /** 时禄（流时层级的禄存） */
  SHI_LU: 'shilu',
  /** 时羊（流时层级的擎羊） */
  SHI_YANG: 'shiyang',
  /** 时陀（流时层级的陀罗） */
  SHI_TUO: 'shituo',
  /** 时马（流时层级的天马） */
  SHI_MA: 'shima',
} as const;
/** 流耀（对应运限对象 `stars` 里的 `Star.key`）。 */
export type HoroscopeStar = (typeof HoroscopeStar)[keyof typeof HoroscopeStar];

/** 格局（对应 `PatternHit.key`） */
export const PatternKey = {
  /** 君臣庆会 */
  JUN_CHEN_QING_HUI: 'jun_chen_qing_hui',
  /** 紫府同宫 */
  ZI_FU_TONG_GONG: 'zi_fu_tong_gong',
  /** 金舆扶驾 */
  JIN_YU_FU_JIA: 'jin_yu_fu_jia',
  /** 紫府夹命 */
  ZI_FU_JIA_MING: 'zi_fu_jia_ming',
  /** 极向离明 */
  JI_XIANG_LI_MING: 'ji_xiang_li_ming',
  /** 极居卯酉 */
  JI_JU_MAO_YOU: 'ji_ju_mao_you',
  /** 机月同梁 */
  JI_YUE_TONG_LIANG: 'ji_yue_tong_liang',
  /** 善荫朝纲 */
  SHAN_YIN_CHAO_GANG: 'shan_yin_chao_gang',
  /** 机巨同临 */
  JI_JU_TONG_LIN: 'ji_ju_tong_lin',
  /** 机巨居卯 */
  JI_JU_JU_MAO: 'ji_ju_ju_mao',
  /** 日月同宫 */
  RI_YUE_TONG_GONG: 'ri_yue_tong_gong',
  /** 巨日同宫 */
  JU_RI_TONG_GONG: 'ju_ri_tong_gong',
  /** 日照雷门 */
  RI_ZHAO_LEI_MEN: 'ri_zhao_lei_men',
  /** 日月并明 */
  RI_YUE_BING_MING: 'ri_yue_bing_ming',
  /** 日月反背 */
  RI_YUE_FAN_BEI: 'ri_yue_fan_bei',
  /** 日月照璧 */
  RI_YUE_ZHAO_BI: 'ri_yue_zhao_bi',
  /** 金灿光辉 */
  JIN_CAN_GUANG_HUI: 'jin_can_guang_hui',
  /** 日月藏辉 */
  RI_YUE_CANG_HUI: 'ri_yue_cang_hui',
  /** 丹墀桂墀 */
  DAN_CHI_GUI_CHI: 'dan_chi_gui_chi',
  /** 日月夹命 */
  RI_YUE_JIA_MING: 'ri_yue_jia_ming',
  /** 日月夹财 */
  RI_YUE_JIA_CAI: 'ri_yue_jia_cai',
  /** 月朗天门 */
  YUE_LANG_TIAN_MEN: 'yue_lang_tian_men',
  /** 月生沧海 */
  YUE_SHENG_CANG_HAI: 'yue_sheng_cang_hai',
  /** 明珠出海 */
  MING_ZHU_CHU_HAI: 'ming_zhu_chu_hai',
  /** 武贪同行 */
  WU_TAN_TONG_XING: 'wu_tan_tong_xing',
  /** 铃昌陀武 */
  LING_CHANG_TUO_WU: 'ling_chang_tuo_wu',
  /** 刑囚夹印 */
  XING_QIU_JIA_YIN: 'xing_qiu_jia_yin',
  /** 生不逢时 */
  SHENG_BU_FENG_SHI: 'sheng_bu_feng_shi',
  /** 雄宿朝元 */
  XIONG_SU_CHAO_YUAN: 'xiong_su_chao_yuan',
  /** 府相朝垣 */
  FU_XIANG_CHAO_YUAN: 'fu_xiang_chao_yuan',
  /** 火贪 */
  HUO_TAN: 'huo_tan',
  /** 铃贪 */
  LING_TAN: 'ling_tan',
  /** 石中隐玉 */
  SHI_ZHONG_YIN_YU: 'shi_zhong_yin_yu',
  /** 梁马飘荡 */
  LIANG_MA_PIAO_DANG: 'liang_ma_piao_dang',
  /** 阳梁昌禄 */
  YANG_LIANG_CHANG_LU: 'yang_liang_chang_lu',
  /** 杀破狼 */
  SHA_PO_LANG: 'sha_po_lang',
  /** 七杀朝斗 */
  QI_SHA_CHAO_DOU: 'qi_sha_chao_dou',
  /** 禄衰马困 */
  LU_SHUAI_MA_KUN: 'lu_shuai_ma_kun',
  /** 英星入庙 */
  YING_XING_RU_MIAO: 'ying_xing_ru_miao',
  /** 众水朝东 */
  ZHONG_SHUI_CHAO_DONG: 'zhong_shui_chao_dong',
  /** 三奇加会 */
  SAN_QI_JIA_HUI: 'san_qi_jia_hui',
  /** 禄马交驰 */
  LU_MA_JIAO_CHI: 'lu_ma_jiao_chi',
  /** 禄合鸳鸯 */
  LU_HE_YUAN_YANG: 'lu_he_yuan_yang',
  /** 明禄暗禄 */
  MING_LU_AN_LU: 'ming_lu_an_lu',
  /** 禄马佩印 */
  LU_MA_PEI_YIN: 'lu_ma_pei_yin',
  /** 两重华盖 */
  LIANG_CHONG_HUA_GAI: 'liang_chong_hua_gai',
  /** 风云际会 */
  FENG_YUN_JI_HUI: 'feng_yun_ji_hui',
  /** 羊陀夹命 */
  YANG_TUO_JIA_MING: 'yang_tuo_jia_ming',
  /** 马头带箭 */
  MA_TOU_DAI_JIAN: 'ma_tou_dai_jian',
  /** 左右同宫 */
  ZUO_YOU_TONG_GONG: 'zuo_you_tong_gong',
  /** 左右夹命 */
  ZUO_YOU_JIA_MING: 'zuo_you_jia_ming',
  /** 辅弼拱主 */
  FU_BI_GONG_ZHU: 'fu_bi_gong_zhu',
  /** 魁钺夹命 */
  KUI_YUE_JIA_MING: 'kui_yue_jia_ming',
  /** 坐贵向贵 */
  ZUO_GUI_XIANG_GUI: 'zuo_gui_xiang_gui',
  /** 劫空夹命 */
  JIE_KONG_JIA_MING: 'jie_kong_jia_ming',
  /** 禄逢两杀 */
  LU_FENG_LIANG_SHA: 'lu_feng_liang_sha',
  /** 文贵文华 */
  WEN_GUI_WEN_HUA: 'wen_gui_wen_hua',
  /** 文星朝命 */
  WEN_XING_CHAO_MING: 'wen_xing_chao_ming',
  /** 昌曲夹命 */
  CHANG_QU_JIA_MING: 'chang_qu_jia_ming',
  /** 文星暗拱 */
  WEN_XING_AN_GONG: 'wen_xing_an_gong',
  /** 权禄生逢 */
  QUAN_LU_SHENG_FENG: 'quan_lu_sheng_feng',
  /** 科明暗禄 */
  KE_MING_AN_LU: 'ke_ming_an_lu',
  /** 科权禄夹 */
  KE_QUAN_LU_JIA: 'ke_quan_lu_jia',
  /** 甲第登庸 */
  JIA_DI_DENG_YONG: 'jia_di_deng_yong',
} as const;
/** 格局（对应 `PatternHit.key`） */
export type PatternKey = (typeof PatternKey)[keyof typeof PatternKey];

/** 格局判定中日月明暗的依据（`PatternConfig.brightnessSource`） */
export const BrightnessSource = {
  /** 按 iztro 亮度表：庙旺为明，陷与「不」为暗 */
  TABLE: 'table',
  /** 按传统位置：太阳寅至午明、酉至丑暗；太阴酉至丑明、卯至未暗 */
  POSITIONAL: 'positional',
} as const;
/** 格局判定中日月明暗的依据（`PatternConfig.brightnessSource`） */
export type BrightnessSource = (typeof BrightnessSource)[keyof typeof BrightnessSource];

/** 语义化文本的事实详略（`toText({ detail })`） */
export const TextDetail = {
  /**
   * 总览：本命只写基本信息、十二宫总览表与格局；运限各层只写宫名对照、四化、流耀与格局；
   *     宫位段落只写主星、辅星与杂耀
   */
  OVERVIEW: 'overview',
  /** 完整：全部事实 */
  FULL: 'full',
} as const;
/** 语义化文本的事实详略（`toText({ detail })`） */
export type TextDetail = (typeof TextDetail)[keyof typeof TextDetail];

/** 语义化文本的释义深度（`toText({ knowledgeDepth })`，给了 `knowledge` 才有意义） */
export const KnowledgeDepth = {
  /** 摘要层：条目摘要，或本身够短的正文；格局不附成立条件。包缺摘要即报错 */
  SUMMARY: 'summary',
  /** 全文：正文全文，格局附成立条件 */
  FULL: 'full',
} as const;
/** 语义化文本的释义深度（`toText({ knowledgeDepth })`，给了 `knowledge` 才有意义） */
export type KnowledgeDepth = (typeof KnowledgeDepth)[keyof typeof KnowledgeDepth];

/** 知识包校验问题的种类（`KnowledgeIssue.code`）；级别与出处见文档站「知识包 · 校验」 */
export const KnowledgeIssueCode = {
  /** 文本不是合法 JSON */
  INVALID_JSON: 'invalid_json',
  /** 包或条目不是 JSON 对象 */
  NOT_OBJECT: 'not_object',
  /** schema 缺失、为 null 或 0 */
  SCHEMA_MISSING: 'schema_missing',
  /** schema 比支持的版本新 */
  SCHEMA_UNSUPPORTED: 'schema_unsupported',
  /** 字段类型与规范不符 */
  WRONG_TYPE: 'wrong_type',
  /** id / version / language 缺失或为空串 */
  REQUIRED_MISSING: 'required_missing',
  /** language 不是 x-iztro 语言码 */
  UNKNOWN_LANGUAGE: 'unknown_language',
  /** 条目键不是内核认识的标识 */
  UNKNOWN_KEY: 'unknown_key',
  /** 双星组合的键不是主星 */
  COMBINATION_KEY_NOT_MAJOR: 'combination_key_not_major',
  /** category / yinYang / fiveElements / stem 取值不在规定集合内 */
  INVALID_VALUE: 'invalid_value',
  /** 术语或主题的 slug 含小写字母、数字、连字符以外的字符 */
  INVALID_SLUG: 'invalid_slug',
  /** 主题的宫位不认识 */
  UNKNOWN_PALACE: 'unknown_palace',
  /** 正文含 HTML 标签 */
  HTML_IN_TEXT: 'html_in_text',
  /** 正文含图片 */
  IMAGE_IN_TEXT: 'image_in_text',
  /** 正文含站外链接 */
  EXTERNAL_LINK: 'external_link',
  /** 规范之外的字段，解析时被丢弃 */
  UNKNOWN_FIELD: 'unknown_field',
  /** 字段为空字符串 */
  EMPTY_STRING: 'empty_string',
  /** 正文超过 240 宽却没有摘要 */
  SUMMARY_MISSING: 'summary_missing',
  /** 正文不超过 240 宽却另写了摘要 */
  SUMMARY_NOT_NEEDED: 'summary_not_needed',
  /** 摘要超过 100 宽 */
  SUMMARY_TOO_WIDE: 'summary_too_wide',
  /** 摘要不是单段 */
  SUMMARY_MULTI_PARAGRAPH: 'summary_multi_paragraph',
  /** 摘要没有对应的正文 */
  SUMMARY_WITHOUT_BODY: 'summary_without_body',
  /** 正文依赖页面上下文 */
  PAGE_REFERENCE: 'page_reference',
  /** 主题一个宫都不取 */
  TOPIC_WITHOUT_PALACES: 'topic_without_palaces',
  /** 以空串覆盖底包里有值的字段 */
  OVERRIDES_WITH_EMPTY: 'overrides_with_empty',
  /** 语言与底包不同 */
  LANGUAGE_DIFFERS_FROM_BASE: 'language_differs_from_base',
  /** extends 与底包的 id 不同 */
  EXTENDS_DIFFERS_FROM_BASE: 'extends_differs_from_base',
} as const;
/** 知识包校验问题的种类（`KnowledgeIssue.code`） */
export type KnowledgeIssueCode = (typeof KnowledgeIssueCode)[keyof typeof KnowledgeIssueCode];

/** 五行局标识 → 局数 2-6，与数据表 `data.constants().fiveElementsClass` 同源 */
export const FIVE_ELEMENTS_CLASS_NUMBER: Readonly<Record<FiveElementsClass, number>> = {
  water2nd: 2,
  wood3rd: 3,
  metal4th: 4,
  earth5th: 5,
  fire6th: 6,
};

/** 时辰索引：0 为早子时（00:00-00:59），1-11 为丑至亥，12 为晚子时（23:00-23:59） */
export type TimeIndex = 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12;

/** 四化的固定顺序：禄、权、科、忌。宫位与运限的 `mutagenStarKeys` 按此顺序排列 */
export const MUTAGEN_ORDER: readonly Mutagen[] = [Mutagen.LU, Mutagen.QUAN, Mutagen.KE, Mutagen.JI];

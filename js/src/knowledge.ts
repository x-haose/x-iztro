/**
 * 知识包：语言无关标识 → 解读文本与门派属性。
 *
 * 内核只负责事实判定；星耀怎么解读、格局意味着什么、宫位与四化的含义属于门派观点，全部放在知识包里。
 * 格式见仓库 `knowledge/SCHEMA.md`。内核内嵌一份默认包（源自 iztro-docs《学习》页，MIT），
 * 使用者可整包替换，或用覆盖包逐条合并；合并在内核完成，各语言绑定规则一致。
 */

import { IztroError } from './errors.js';
import type { Language } from './enums.js';
import { chartQuery, horoscopeQuery, packState, packStates } from './internal.js';
import type { Astrolabe } from './astrolabe.js';
import type { Horoscope } from './horoscope.js';
import { query } from './runtime.js';
import type { KnowledgeIssue, PatternConfig } from './types.js';


/** 包的来源与许可信息 */
export interface Source {
  /** 来源名称（如 iztro-docs） */
  name?: string;
  /** 来源仓库或页面地址 */
  url?: string;
  /** 提取时锁定的来源 commit */
  commit?: string;
  /** 来源的许可协议（如 MIT） */
  license?: string;
  /** 来源作者 */
  author?: string;
  /** 提取日期 YYYY-MM-DD */
  retrievedAt?: string;
  /** 改写说明：文本经过整理改写时注明 */
  adapted?: string;
}

/** 星耀的门派属性（全部可选） */
export interface StarAttributes {
  /** 阴阳（yin / yang） */
  yinYang?: string;
  /** 五行（wood / fire / earth / metal / water） */
  fiveElements?: string;
  /** 五行所带天干（jia…gui） */
  stem?: string;
  /** 五行的补充说明 */
  fiveElementsNote?: string;
  /** 斗分 */
  dipper?: string;
  /** 化气 */
  chemistry?: string;
  /** 职业（主何事） */
  career?: string;
  /** 职务 */
  duty?: string;
  /** 别号 */
  aliases?: string[];
  /** 五行色 */
  elementColor?: string;
  /** 能量色 */
  energyColor?: string;
}

/** 一颗星耀的知识条目 */
export interface StarEntry {
  /** 星耀标识 */
  key: string;
  /** 该语言的显示名 */
  name?: string;
  /** 类别（major / minor / adjective / dec / flow） */
  category?: string;
  /** 分组（杂耀的分类、神煞的组别） */
  group?: string;
  /** 门派属性；包里没写时为空对象 */
  attributes: StarAttributes;
  /** 解读正文 */
  intro?: string;
  /** 正文的摘要（单段）；正文不超过摘要层上限（显示宽度 240，中文 120 字）的条目不写 */
  summary?: string;
  /** 与另一颗主星同宫的组合解读，键为对方星耀标识；包里没写时为空对象 */
  combinations: Record<string, string>;
  /** 组合解读的摘要，与 combinations 同键成对；包里没写时为空对象 */
  combinationSummaries: Record<string, string>;
}

/** 一条格局的知识条目 */
export interface PatternEntry {
  /** 格局标识 */
  key: string;
  /** 该语言的显示名 */
  name?: string;
  /** 古籍引文 */
  quotes?: string[];
  /** 来源对成立条件的文字描述 */
  conditions?: string;
  /** 解读正文 */
  intro?: string;
  /** 正文的摘要（单段）；正文不超过摘要层上限的条目不写 */
  summary?: string;
}

/** 只有名称与正文的条目（宫位、四化） */
export interface TextEntry {
  /** 标识 */
  key: string;
  /** 该语言的显示名 */
  name?: string;
  /** 正文 */
  intro?: string;
  /** 正文的摘要（单段）；正文不超过摘要层上限的条目不写 */
  summary?: string;
}

/** 术语与基础概念条目 */
export interface ConceptEntry {
  /** 条目 slug */
  slug: string;
  /** 标题 */
  title?: string;
  /** 正文 */
  intro?: string;
}

/** 解读主题（如婚恋、财运）：读这类问题要看哪些宫，`Astrolabe.topicToText` 按它选宫 */
export interface TopicEntry {
  /** 主题 slug */
  slug: string;
  /** 显示名 */
  title?: string;
  /** 连同各自三方四正一起取的宫位标识，按出现顺序渲染 */
  surround?: string[];
  /** 只取本宫的宫位标识（可含 bodyPalace / originalPalace），排在 surround 各组之后 */
  palaces?: string[];
  /** 这个主题看什么、为什么取这些宫 */
  intro?: string;
  /** 正文的摘要（单段）；正文不超过摘要层上限的条目不写 */
  summary?: string;
}

type Raw = Record<string, unknown>;

/** 有 error 级问题时抛 `IztroError`（code 为 invalid_argument），全部问题挂在 `issues` 上 */
function rejectInvalid(issues: KnowledgeIssue[]): void {
  const errors = issues.filter((i) => i.level === 'error');
  if (errors.length > 0) {
    throw new IztroError('invalid_argument', `invalid knowledge pack: ${errors.map((i) => i.message).join('; ')}`, {
      issues,
    });
  }
}

/** `KnowledgePack.check` 的选项 */
export interface CheckOptions {
  /** 它要叠加的底包；给出时另报相对底包的提醒，摘要规则按合并后的条目判断 */
  base?: KnowledgePack | undefined;
}

/** 取包里某一段（stars、patterns……）的对象；缺段时为空对象 */
function section(raw: Raw, name: string): Record<string, Raw> {
  return (raw[name] ?? {}) as Record<string, Raw>;
}

/** 取某一段里键为 `key` 的条目；只认包自己的键，`constructor`、`toString` 这类原型链上的名字不算 */
function entryOf(raw: Raw, name: string, key: string): Raw | undefined {
  const sec = section(raw, name);
  return Object.hasOwn(sec, key) ? sec[key] : undefined;
}

function starEntry(key: string, d: Raw): StarEntry {
  return {
    ...(d as Omit<StarEntry, 'key' | 'attributes' | 'combinations' | 'combinationSummaries'>),
    key,
    attributes: { ...((d.attributes ?? {}) as StarAttributes) },
    combinations: { ...((d.combinations ?? {}) as Record<string, string>) },
    combinationSummaries: { ...((d.combinationSummaries ?? {}) as Record<string, string>) },
  };
}

/**
 * 一份知识包。持有包对象，查询方法返回条目对象；`toJSON()` 是包对象的深拷贝，`JSON.stringify(pack)` 即包的 JSON。
 *
 * 构造：`KnowledgePack.builtin()` 取内嵌默认包，`fromObject` / `fromJSON` 读自己的包，
 * `merged(...overlays)` 叠加覆盖包，`forAstrolabe` / `forHoroscope` 按盘取材成子包。
 */
export class KnowledgePack {
  /** 只能经静态构造方法创建，以保证包对象已通过格式校验 */
  private constructor(raw: Raw, builtinLanguage: string | undefined) {
    packStates.set(this, { raw, builtinLanguage });
  }

  // ------ 构造 ------

  /**
   * 内嵌的默认包（源自 iztro-docs，MIT）。
   *
   * @throws IztroError 该语言没有默认包（目前只有 zh-CN）
   */
  static builtin(language: Language = 'zh-CN'): KnowledgePack {
    return new KnowledgePack(query('knowledgePack', { language }) as Raw, language);
  }

  /**
   * 由包对象构造（保留引用，不复制）。先经 `check` 校验，有 error 级问题即拒绝。
   *
   * @throws IztroError 有 error 级问题（code 为 invalid_argument）；全部问题在 `issues` 上
   * @throws TypeError 包对象含 JSON 无法表示的值（如 BigInt、循环引用）
   */
  static fromObject(raw: object): KnowledgePack {
    rejectInvalid(KnowledgePack.check(raw));
    return new KnowledgePack(raw as Raw, undefined);
  }

  /**
   * 由 JSON 文本构造，校验同 `fromObject`。
   *
   * @throws IztroError 有 error 级问题（code 为 invalid_argument）；全部问题在 `issues` 上
   * @throws SyntaxError JSON 语法非法
   */
  static fromJSON(text: string): KnowledgePack {
    return KnowledgePack.fromObject(JSON.parse(text) as object);
  }

  /**
   * 逐条校验一份包，一次返回全部问题（不在第一处停）。输入不是对象等也作为 error 级问题返回。
   *
   * @throws TypeError 包对象含 JSON 无法表示的值，或 `options.base` 不是本库构造的 KnowledgePack
   */
  static check(raw: unknown, options: CheckOptions = {}): KnowledgeIssue[] {
    const packJson = JSON.stringify(raw) ?? 'null';
    const basePack = options.base === undefined ? undefined : packState(options.base).raw;
    return query('checkKnowledgePack', { packJson, basePack }) as KnowledgeIssue[];
  }

  /**
   * 按本命盘取材的子包：只含盘上出现的星（十二宫主辅杂与四组十二神；同宫主星的双星组合解读保留，
   * 不同宫的剔除）、按 config 口径命中的格局、四化四条；不含宫位与术语条目。
   * 元信息沿用本包，结果仍是标准包，可再合并、序列化或按标识查。重排盘按重排后的布局取材。
   */
  forAstrolabe(chart: Astrolabe, config?: PatternConfig): KnowledgePack {
    return new KnowledgePack(
      chartQuery(chart, 'knowledgeForChart', { knowledge: this, patternConfig: config }) as Raw,
      undefined,
    );
  }

  /**
   * 按运限取材的子包：在 `forAstrolabe` 之上再加各层流耀的条目与各层视角命中的格局，
   * 只取到运限的查询精度（`Horoscope.precision()`）那一层。
   */
  forHoroscope(horoscope: Horoscope, config?: PatternConfig): KnowledgePack {
    return new KnowledgePack(
      horoscopeQuery(horoscope, 'knowledgeForChart', { knowledge: this, patternConfig: config }) as Raw,
      undefined,
    );
  }

  /** 包对象的深拷贝 */
  toJSON(): Record<string, unknown> {
    return structuredClone(packState(this).raw);
  }

  /**
   * 以本包为底依次叠加覆盖包，返回新包（本包不变）。
   *
   * 规则见 knowledge/SCHEMA.md：覆盖包的非空字段覆盖同键条目的对应字段，attributes / combinations
   * 逐字段合并，数组字段整体替换；覆盖条目换了正文却没给对应摘要时，合并结果去掉描述旧正文的摘要。
   *
   * @throws IztroError 某个覆盖包对象有 error 级问题（同 `fromObject`）
   */
  merged(...overlays: (KnowledgePack | object)[]): KnowledgePack {
    const packs = [
      packState(this).raw,
      ...overlays.map((o) => packState(o instanceof KnowledgePack ? o : KnowledgePack.fromObject(o)).raw),
    ];
    return new KnowledgePack(query('mergeKnowledgePacks', { knowledgePacks: packs }) as Raw, undefined);
  }

  // ------ 元信息 ------

  /** 格式版本 */
  get schema(): number {
    return Number(packState(this).raw.schema ?? 0);
  }

  /** 包标识 */
  get id(): string {
    return String(packState(this).raw.id ?? '');
  }

  /** 包版本 */
  get version(): string {
    return String(packState(this).raw.version ?? '');
  }

  /** 文本语言 */
  get language(): string {
    return String(packState(this).raw.language ?? '');
  }

  /** 覆盖包所覆盖的包标识；独立包为 undefined */
  get extends(): string | undefined {
    return packState(this).raw.extends as string | undefined;
  }

  /** 来源与许可 */
  get source(): Source {
    return { ...((packState(this).raw.source ?? {}) as Source) };
  }

  // ------ 查询 ------

  /** 星耀条目（键为星耀标识） */
  star(key: string): StarEntry | undefined {
    const d = entryOf(packState(this).raw, 'stars', key);
    return d === undefined ? undefined : starEntry(key, d);
  }

  /** 格局条目（键为格局标识） */
  pattern(key: string): PatternEntry | undefined {
    const d = entryOf(packState(this).raw, 'patterns', key);
    return d === undefined ? undefined : { ...d, key };
  }

  /** 宫位条目（键为宫位标识） */
  palace(key: string): TextEntry | undefined {
    const d = entryOf(packState(this).raw, 'palaces', key);
    return d === undefined ? undefined : { ...d, key };
  }

  /** 四化条目（键为四化标识） */
  mutagen(key: string): TextEntry | undefined {
    const d = entryOf(packState(this).raw, 'mutagens', key);
    return d === undefined ? undefined : { ...d, key };
  }

  /** 术语条目 */
  concept(slug: string): ConceptEntry | undefined {
    const d = entryOf(packState(this).raw, 'concepts', slug);
    return d === undefined ? undefined : { ...d, slug };
  }

  /** 主题条目 */
  topic(slug: string): TopicEntry | undefined {
    const d = entryOf(packState(this).raw, 'topics', slug);
    return d === undefined ? undefined : { ...d, slug };
  }

  /** 全部主题条目 */
  topics(): TopicEntry[] {
    return Object.entries(section(packState(this).raw, 'topics')).map(([slug, d]) => ({ ...d, slug }));
  }

  /** 全部星耀条目 */
  stars(): StarEntry[] {
    return Object.entries(section(packState(this).raw, 'stars')).map(([key, d]) => starEntry(key, d));
  }

  /** 全部格局条目 */
  patterns(): PatternEntry[] {
    return Object.entries(section(packState(this).raw, 'patterns')).map(([key, d]) => ({ ...d, key }));
  }

  /** 星耀解读正文 */
  starIntro(key: string): string | undefined {
    return this.star(key)?.intro;
  }

  /** 格局解读正文 */
  patternIntro(key: string): string | undefined {
    return this.pattern(key)?.intro;
  }
}

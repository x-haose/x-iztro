import Link from 'next/link';
import type { Metadata } from 'next';
import { ChartPreview } from '@/components/chart-preview';
import { CodeSample } from '@/components/code-sample';
import { SITE_URL, languageAlternates } from '@/lib/site';

const COPY = {
  zh: {
    metaTitle: 'x-iztro — 紫微斗数排盘引擎 · Rust / Python / Go',
    metaDescription:
      '紫微斗数排盘引擎：与 JS iztro 逐字段零差异（716,314 例金标），附 64 条格局判定、知识包与生辰反推，一次调用转成大模型可读文本。Rust 核心，Rust / Python / Go 直接调用。',
    ogImage: '/og.png',
    tagline: '排盘归它算 · 解读归 AI',
    headline: '紫微斗数排盘引擎',
    lede: '输入公历生日和出生时辰，输出一张完整的紫微斗数命盘——十二宫、上百颗星、各层运限；再调用一次，就变成大模型读得懂的文字。Rust 核心，Rust / Python / Go 直接调用，排盘结果与 JS iztro 逐字段一致，716,314 例金标守着这条线。',
    start: '快速开始',
    concepts: '先看懂命盘',
    nonDev: '不写代码？从这页开始 →',
    ariaStats: '关键数字',
    ariaInstall: '安装',
    ariaFeatures: '特性',
    ariaProduction: '生产就绪',
    stats: [
      { value: '716,314', label: '金标对照用例' },
      { value: '0', label: '允许的差异' },
      { value: '64', label: '条格局判定' },
      { value: '6', label: '种输出语言' },
    ],
    promptTitle: '大模型拿到的是这样一段文字',
    promptCaption:
      '一次调用生成整盘的结构化文本，直接贴进任何大模型就能开始问；括号是亮度与四化标记，运限另有同款输出。',
    promptLink: '它是怎么生成的 →',
    promptSample: `=== 基本信息 ===
性别: 女
阳历: 2000-8-16
农历: 二〇〇〇年七月十七
干支: 庚辰 甲申 丙午 庚寅
…
命宫地支: 午
身宫地支: 戌
命主: 破军
身主: 文昌
五行局: 木三局
生年四化: 太阳禄, 武曲权, 太阴科, 天同忌

=== 十二宫 ===

--- 财帛 ---
…
主星: 武曲(得)[权], 天相(庙)
辅星: 天马
杂耀: 解神, 三台, 天寿, 天巫, 天厨, 阴煞, 天哭
…（其余十一宫依次列全）`,
    iztroTitle: '与 iztro 的关系',
    iztroAligned: {
      title: '与 JS iztro 完全一致',
      items: [
        '排盘、十二宫（含身宫、来因宫）、六层运限',
        '宫位查询、三方四正、飞星',
        '序列化 JSON 逐键一致',
        '716,314 例金标，零容忍差异',
      ],
    },
    iztroBeyond: {
      title: '在 iztro 之上',
      items: [
        '64 条格局判定——命中带成格宫位、口径与参与成格的星曜',
        '知识包——解读文本外置成可替换 JSON，内嵌默认包',
        '生辰反推——八字四柱 / 盘面特征反查候选生辰',
      ],
    },
    iztroLink: '完整对应关系 →',
    langs: [
      { name: 'Rust', install: 'cargo add x-iztro', href: 'rust' },
      { name: 'Python', install: 'pip install x-iztro', href: 'python' },
      { name: 'Go', install: 'go get github.com/x-haose/x-iztro/go/iztro', href: 'go' },
    ],
    sample: `from x_iztro import Astro

astro = Astro()
# 时辰索引 2 = 寅时
chart = astro.by_solar("2000-8-16", 2, "female")

chart.soul                # 命主
chart.palace("soulPalace")  # 命宫
chart.patterns()          # 格局命中
astro.astrolabe_to_prompt(chart)  # 整盘 → 文本`,
    features: [
      {
        title: '排盘与运限完整',
        body: '本命盘加大限（起运前为童限）、小限、流年、流月、流日、流时六个层级，四化、流曜、三方四正全部可取。',
      },
      {
        title: '流派与分界点可配置',
        body: '年分界、运限分界、虚岁、晚子时归属、安星流派、盘型六个开关，默认值与 iztro 一致；另支持自定义四化表与亮度表。',
      },
      {
        title: '语言无关的判断标识',
        body: '每个可翻译字段配一个稳定 key。判断逻辑写一次，在六种输出语言的盘上结果相同。',
      },
      {
        title: '面向 AI 的接口',
        body: 'Prompt 输出格式确定、字段顺序稳定，可直接当 tool call 的返回值；文档站自带 llms.txt，任意文档页追加 .md 即纯文本。',
      },
    ],
    prodStrip: ['无全局状态', '非法输入不 panic', 'Python abi3 零依赖', 'Go 无 cgo', 'MIT 开源'],
  },
  en: {
    metaTitle: 'x-iztro — Zi Wei Dou Shu (Purple Star Astrology) chart engine for Rust, Python & Go',
    metaDescription:
      'A Chinese astrology (Zi Wei Dou Shu / Purple Star) chart engine, field-for-field identical to JS iztro — 716,314 golden cases — plus 64 pattern rules, knowledge packs, reverse birth-date lookup and LLM-ready output. Rust core with Python and Go bindings.',
    ogImage: '/og-en.png',
    tagline: 'Code does the chart. AI does the reading.',
    headline: 'Chinese astrology (Zi Wei Dou Shu) chart engine',
    lede: 'Zi Wei Dou Shu — Chinese "Purple Star" astrology — charts a life from a birth date and hour. A Rust core casts it, with bindings for Rust, Python and Go; every field carries a translated name plus a stable key, and one call turns the whole chart into text an LLM can read. Charts match JS iztro field for field — 716,314 golden cases hold that line.',
    start: 'Get started',
    concepts: 'Learn the chart',
    nonDev: 'Not writing code? Start here →',
    ariaStats: 'key numbers',
    ariaInstall: 'install',
    ariaFeatures: 'features',
    ariaProduction: 'production-ready',
    stats: [
      { value: '716,314', label: 'golden cases' },
      { value: '0', label: 'tolerated deviations' },
      { value: '64', label: 'pattern rules' },
      { value: '6', label: 'output languages' },
    ],
    promptTitle: 'What the model gets',
    promptCaption:
      'One call renders the whole chart as structured text — paste it into any LLM and start asking. Brackets are brightness and the four transformations; there is a matching call for horoscopes.',
    promptLink: 'How it is generated →',
    promptSample: `=== Basic Info ===
Gender: female
Solar Date: 2000-8-16
Lunar Date: 二〇〇〇年七月十七
Chinese Date: geng chen - jia shen - bing woo - geng yin
…
Soul Palace Branch: woo
Body Palace Branch: xu
Soul Star: rebel
Body Star: scholar
Five Elements Class: wood 3rd
Birth-Year Mutagen: sunA, generalB, moonC, fortunateD

=== Palaces ===

--- wealth ---
…
Major Stars: general([+1])[B], minister([+3])
Minor Stars: horse
Adjective Stars: considery, senior, ageless, psychic, gourmet, gloomy, upset
… (the other eleven palaces)`,
    iztroTitle: 'x-iztro and iztro',
    iztroAligned: {
      title: 'Identical to JS iztro',
      items: [
        'Charts, twelve palaces (body and Original palace included), six horoscope levels',
        'Palace queries, surrounded palaces, flying stars',
        'Serialized JSON, key for key',
        '716,314 golden cases, zero tolerated deviations',
      ],
    },
    iztroBeyond: {
      title: 'Beyond iztro',
      items: [
        '64 pattern rules — each hit names its palace, variant and the stars that triggered it',
        'Knowledge packs — reading texts in swappable JSON, default pack included',
        'Reverse lookup — BaZi pillars or chart features back to birth dates',
      ],
    },
    iztroLink: 'Full comparison with iztro →',
    langs: [
      { name: 'Rust', install: 'cargo add x-iztro', href: 'rust' },
      { name: 'Python', install: 'pip install x-iztro', href: 'python' },
      { name: 'Go', install: 'go get github.com/x-haose/x-iztro/go/iztro', href: 'go' },
    ],
    sample: `from x_iztro import Astro

astro = Astro()
# time_index 2 = Tiger hour
chart = astro.by_solar("2000-8-16", 2, "female")

chart.soul                # soul star
chart.palace("soulPalace")  # soul palace
chart.patterns()          # pattern hits
astro.astrolabe_to_prompt(chart)  # chart -> text`,
    features: [
      {
        title: 'Complete charts and horoscopes',
        body: 'Natal chart plus six horoscope levels — decadal (a childhood scope runs before it begins), age fortune, yearly, monthly, daily and hourly — with mutagens, flowing stars and surrounded palaces.',
      },
      {
        title: 'Configurable schools and boundaries',
        body: 'Six switches for the year, horoscope, age and late-Rat-hour boundaries, the school and the chart perspective. Defaults match iztro exactly; custom mutagen and brightness tables are supported.',
      },
      {
        title: 'Language-independent keys',
        body: 'Every translatable field carries a stable key. Write the logic once; it behaves identically across all six output languages.',
      },
      {
        title: 'An interface built for AI',
        body: 'Prompt output is deterministic, with a stable field order — usable directly as a tool-call result. The docs ship llms.txt, and every docs page serves plain text with .md appended.',
      },
    ],
    prodStrip: [
      'No global state',
      'Invalid input errors, never panics',
      'Python: abi3, zero deps',
      'Go: no cgo',
      'MIT licensed',
    ],
  },
} as const;

export async function generateMetadata({ params }: PageProps<'/[lang]'>): Promise<Metadata> {
  const { lang } = await params;
  const t = COPY[lang as keyof typeof COPY] ?? COPY.en;

  return {
    title: t.metaTitle,
    description: t.metaDescription,
    alternates: {
      canonical: `${SITE_URL}/${lang}`,
      // 首页在各语言下的地址，与文档内页及 sitemap 的 hreflang 同源。
      languages: languageAlternates((l) => `${SITE_URL}/${l}`),
    },
    openGraph: {
      title: t.metaTitle,
      description: t.metaDescription,
      url: `${SITE_URL}/${lang}`,
      siteName: 'x-iztro',
      type: 'website',
      images: [{ url: t.ogImage, width: 2400, height: 1260, alt: t.metaTitle }],
    },
    twitter: { card: 'summary_large_image' },
  };
}

export default async function HomePage({ params }: PageProps<'/[lang]'>) {
  const { lang } = await params;
  const t = COPY[lang as keyof typeof COPY] ?? COPY.en;

  return (
    <main className="flex flex-1 flex-col">
      <section
        aria-labelledby="hero-heading"
        className="mx-auto grid w-full max-w-6xl items-center gap-12 px-6 py-20 lg:grid-cols-[1.1fr_1fr] lg:py-28"
      >
        <div className="flex flex-col gap-6">
          <p className="text-xs font-medium tracking-[0.08em] text-fd-muted-foreground uppercase">
            {t.tagline}
          </p>
          <h1 id="hero-heading" className="text-5xl leading-[1.15] font-bold tracking-tight">
            {t.headline}
          </h1>
          <p className="max-w-xl text-lg leading-relaxed text-fd-muted-foreground">{t.lede}</p>

          <div className="mt-2 flex flex-wrap items-center gap-3">
            <Link
              href={`/${lang}/docs/guide/getting-started`}
              className="rounded-xl bg-fd-primary px-5 py-2.5 text-sm font-medium text-fd-primary-foreground transition-opacity hover:opacity-90"
            >
              {t.start}
            </Link>
            <Link
              href={`/${lang}/docs/guide/concepts`}
              className="rounded-xl border border-fd-border px-5 py-2.5 text-sm font-medium transition-colors hover:bg-fd-accent"
            >
              {t.concepts}
            </Link>
          </div>

          <Link
            href={`/${lang}/docs/guide/guides/for-non-developers`}
            className="text-sm text-fd-muted-foreground underline-offset-4 transition-colors hover:text-fd-foreground hover:underline"
          >
            {t.nonDev}
          </Link>
        </div>

        <div className="flex justify-center lg:justify-end">
          <ChartPreview lang={lang} />
        </div>
      </section>

      <section aria-label={t.ariaStats} className="border-y border-fd-border">
        <dl className="mx-auto grid w-full max-w-5xl grid-cols-2 gap-px px-6 sm:grid-cols-4">
          {t.stats.map((s) => (
            <div key={s.label} className="flex flex-col-reverse py-8">
              <dt className="mt-1 text-sm text-fd-muted-foreground">{s.label}</dt>
              <dd className="text-3xl font-semibold tabular-nums">{s.value}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section
        aria-labelledby="prompt-heading"
        className="mx-auto grid w-full max-w-5xl items-center gap-8 px-6 py-20 md:grid-cols-[1fr_1.3fr]"
      >
        <div className="flex flex-col gap-4">
          <h2 id="prompt-heading" className="text-2xl font-semibold tracking-tight">
            {t.promptTitle}
          </h2>
          <p className="text-sm leading-relaxed text-fd-muted-foreground">{t.promptCaption}</p>
          <Link
            href={`/${lang}/docs/guide/guides/ai-prompt`}
            className="text-sm font-medium underline-offset-4 hover:underline"
          >
            {t.promptLink}
          </Link>
        </div>
        <pre className="overflow-x-auto rounded-xl border border-fd-border bg-fd-card p-5 text-xs leading-relaxed sm:text-sm">
          <code>{t.promptSample}</code>
        </pre>
      </section>

      <section aria-labelledby="iztro-heading" className="border-t border-fd-border">
        <div className="mx-auto w-full max-w-5xl px-6 py-20">
          <h2 id="iztro-heading" className="text-2xl font-semibold tracking-tight">
            {t.iztroTitle}
          </h2>
          <div className="mt-8 grid gap-6 md:grid-cols-2">
            <div className="rounded-xl border border-fd-border p-6">
              <h3 className="text-base font-semibold">{t.iztroAligned.title}</h3>
              <ul className="mt-3 flex flex-col gap-2 text-sm leading-relaxed text-fd-muted-foreground">
                {t.iztroAligned.items.map((item) => (
                  <li key={item} className="flex gap-2">
                    <span aria-hidden className="select-none">
                      ✓
                    </span>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
            <div className="rounded-xl border border-fd-primary/40 p-6">
              <h3 className="text-base font-semibold">{t.iztroBeyond.title}</h3>
              <ul className="mt-3 flex flex-col gap-2 text-sm leading-relaxed text-fd-muted-foreground">
                {t.iztroBeyond.items.map((item) => (
                  <li key={item} className="flex gap-2">
                    <span aria-hidden className="select-none">
                      +
                    </span>
                    {item}
                  </li>
                ))}
              </ul>
            </div>
          </div>
          <Link
            href={`/${lang}/docs/guide/about/iztro-parity`}
            className="mt-6 inline-block text-sm font-medium underline-offset-4 hover:underline"
          >
            {t.iztroLink}
          </Link>
        </div>
      </section>

      <section aria-label={t.ariaInstall} className="border-t border-fd-border">
        <div className="mx-auto grid w-full max-w-5xl gap-6 px-6 py-20 md:grid-cols-[1fr_1.2fr]">
          <ul className="flex flex-col gap-3">
            {t.langs.map((l) => (
              <li key={l.name}>
                <Link
                  href={`/${lang}/docs/${l.href}`}
                  className="flex items-baseline justify-between gap-4 rounded-xl border border-fd-border px-4 py-3 transition-colors hover:bg-fd-accent"
                >
                  <span className="font-medium">{l.name}</span>
                  <code className="text-right text-xs break-all text-fd-muted-foreground">
                    {l.install}
                  </code>
                </Link>
              </li>
            ))}
          </ul>

          <CodeSample
            lang="python"
            code={t.sample}
            className="overflow-x-auto rounded-xl border border-fd-border bg-fd-card py-5 text-sm leading-relaxed [--padding-left:1.25rem] [--padding-right:1.25rem]"
          />
        </div>
      </section>

      <section aria-label={t.ariaFeatures} className="mx-auto w-full max-w-5xl px-6 py-20">
        <ul className="grid gap-6 sm:grid-cols-2">
          {t.features.map((f) => (
            <li key={f.title} className="rounded-xl border border-fd-border p-6">
              <h3 className="text-base font-semibold">{f.title}</h3>
              <p className="mt-2 text-sm leading-relaxed text-fd-muted-foreground">{f.body}</p>
            </li>
          ))}
        </ul>
      </section>

      <section aria-label={t.ariaProduction} className="border-t border-fd-border">
        <ul className="mx-auto flex w-full max-w-5xl flex-wrap items-center justify-center gap-x-8 gap-y-2 px-6 py-8 text-sm text-fd-muted-foreground">
          {t.prodStrip.map((item) => (
            <li key={item}>{item}</li>
          ))}
        </ul>
      </section>
    </main>
  );
}

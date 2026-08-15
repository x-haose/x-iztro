import Link from 'next/link';
import { ChartPreview } from '@/components/chart-preview';

const COPY = {
  zh: {
    tagline: '紫微斗数排盘引擎',
    headline: '一次实现，三种语言',
    lede: 'Rust 核心，供 Rust、Python、Go 调用。排盘结果与 JS iztro 逐字段一致 —— 约 66 万例金标测试守着这条线。',
    start: '快速开始',
    concepts: '先看懂命盘',
    stats: [
      { value: '66 万', label: '金标对照用例' },
      { value: '3', label: '语言绑定' },
      { value: '6', label: '输出语言' },
      { value: '0', label: '容忍的差异' },
    ],
    features: [
      {
        title: '排盘与运限完整',
        body: '本命盘加大限、小限、童限、流年、流月、流日、流时六个层级，四化、流耀、三方四正全部可取。',
      },
      {
        title: '流派与分界点可配置',
        body: '年分界、运限分界、虚岁分界、晚子时归属、算法派别五个开关，默认值与 iztro 一致。',
      },
      {
        title: '语言无关的判断标识',
        body: '每个可翻译字段配一个稳定 key。判断逻辑写一次，在六种输出语言的盘上结果相同。',
      },
      {
        title: '面向 AI 的输出',
        body: '内置 Prompt 生成，把命盘与运限转成结构化文本，直接交给大模型分析。',
      },
    ],
    langs: [
      { name: 'Rust', install: 'cargo add x-iztro', href: 'rust' },
      { name: 'Python', install: 'pip install x-iztro', href: 'python' },
      { name: 'Go', install: 'go get .../go/iztro', href: 'go' },
    ],
  },
  en: {
    tagline: 'Zi Wei Dou Shu chart engine',
    headline: 'One core, three languages',
    lede: 'A Rust engine callable from Rust, Python and Go. Charts match JS iztro field for field — guarded by roughly 660,000 golden test cases.',
    start: 'Get started',
    concepts: 'Learn the chart',
    stats: [
      { value: '660k', label: 'golden cases' },
      { value: '3', label: 'language bindings' },
      { value: '6', label: 'output languages' },
      { value: '0', label: 'tolerated deviations' },
    ],
    features: [
      {
        title: 'Complete charts and horoscopes',
        body: 'Natal chart plus decadal, age, childhood, yearly, monthly, daily and hourly scopes — with mutagens, scope stars and surrounded palaces.',
      },
      {
        title: 'Configurable schools and boundaries',
        body: 'Five switches for year, horoscope, age and late-Zi-hour boundaries plus the algorithm school. Defaults match iztro exactly.',
      },
      {
        title: 'Language-independent keys',
        body: 'Every translatable field carries a stable key. Write the logic once; it behaves identically across all six output languages.',
      },
      {
        title: 'Built for AI',
        body: 'Prompt generation turns a chart or horoscope into structured text you can hand straight to a language model.',
      },
    ],
    langs: [
      { name: 'Rust', install: 'cargo add x-iztro', href: 'rust' },
      { name: 'Python', install: 'pip install x-iztro', href: 'python' },
      { name: 'Go', install: 'go get .../go/iztro', href: 'go' },
    ],
  },
} as const;

const SAMPLE = `from x_iztro import Astro

astro = Astro()
chart = astro.by_solar("2000-8-16", 2, "female")

chart.soul                        # 命主
chart.palace("soulPalace")        # 命宫
astro.get_horoscope(chart, "2024-10-1", 0)`;

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
        </div>

        <div className="flex justify-center lg:justify-end">
          <ChartPreview lang={lang} />
        </div>
      </section>

      <section aria-label="stats" className="border-y border-fd-border">
        <dl className="mx-auto grid w-full max-w-5xl grid-cols-2 gap-px px-6 sm:grid-cols-4">
          {t.stats.map((s) => (
            <div key={s.label} className="py-8">
              <dt className="text-3xl font-semibold tabular-nums">{s.value}</dt>
              <dd className="mt-1 text-sm text-fd-muted-foreground">{s.label}</dd>
            </div>
          ))}
        </dl>
      </section>

      <section
        aria-label="install"
        className="mx-auto grid w-full max-w-5xl gap-6 px-6 py-20 md:grid-cols-[1fr_1.2fr]"
      >
        <ul className="flex flex-col gap-3">
          {t.langs.map((l) => (
            <li key={l.name}>
              <Link
                href={`/${lang}/docs/${l.href}`}
                className="flex items-baseline justify-between gap-4 rounded-xl border border-fd-border px-4 py-3 transition-colors hover:bg-fd-accent"
              >
                <span className="font-medium">{l.name}</span>
                <code className="text-xs text-fd-muted-foreground">{l.install}</code>
              </Link>
            </li>
          ))}
        </ul>

        <pre className="overflow-x-auto rounded-xl border border-fd-border bg-fd-card p-5 text-sm leading-relaxed">
          <code>{SAMPLE}</code>
        </pre>
      </section>

      <section aria-label="features" className="mx-auto w-full max-w-5xl px-6 pb-24">
        <ul className="grid gap-6 sm:grid-cols-2">
          {t.features.map((f) => (
            <li key={f.title} className="rounded-xl border border-fd-border p-6">
              <h2 className="text-base font-semibold">{f.title}</h2>
              <p className="mt-2 text-sm leading-relaxed text-fd-muted-foreground">{f.body}</p>
            </li>
          ))}
        </ul>
      </section>
    </main>
  );
}

import Link from 'next/link';
import { REPO_URL, LEARN_URL } from '@/lib/layout.shared';

const TEXT = {
  zh: {
    docs: '文档',
    guide: '指南',
    concepts: '紫微斗数概念',
    dataModel: '数据结构',
    api: 'API 参考',
    forAI: '给 AI 用',
    llmsIndex: '站点索引',
    llmsFull: '全文',
    howTo: '接入说明',
    project: '项目',
    about: '关于',
    accuracy: '准确性',
    architecture: '架构',
    learn: '紫微扫盲',
    tagline: '紫微斗数排盘引擎，Rust 核心，三语言绑定。',
  },
  en: {
    docs: 'Docs',
    guide: 'Guide',
    concepts: 'Concepts',
    dataModel: 'Data model',
    api: 'API',
    forAI: 'For AI',
    llmsIndex: 'Site index',
    llmsFull: 'Full text',
    howTo: 'How to use',
    project: 'Project',
    about: 'About',
    accuracy: 'Accuracy',
    architecture: 'Architecture',
    learn: 'Learn Zi Wei',
    tagline: 'Zi Wei Dou Shu chart engine. Rust core, three language bindings.',
  },
} as const;

function Column({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <div className="flex flex-col gap-2.5">
      <h2 className="text-xs font-medium tracking-wide text-fd-foreground">{title}</h2>
      {children}
    </div>
  );
}

const linkClass =
  'text-sm text-fd-muted-foreground transition-colors hover:text-fd-foreground';

/**
 * 全站页脚。
 *
 * 除常规导航外，单列出「给 AI 用」的三个端点——它们不在任何页面的正文里，
 * 光靠文档目录读者不会知道站点提供了这些。
 */
export function Footer({ lang }: { lang: string }) {
  const t = TEXT[lang as keyof typeof TEXT] ?? TEXT.en;
  const doc = (path: string) => `/${lang}/docs/${path}`;

  return (
    <footer className="border-t border-fd-border">
      <div className="mx-auto grid w-full max-w-5xl gap-10 px-6 py-14 sm:grid-cols-2 lg:grid-cols-4">
        <div className="flex flex-col gap-2.5">
          <span className="text-sm font-semibold">x-iztro</span>
          <p className="text-sm leading-relaxed text-fd-muted-foreground">{t.tagline}</p>
        </div>

        <Column title={t.docs}>
          <Link className={linkClass} href={doc('guide')}>{t.guide}</Link>
          <Link className={linkClass} href={doc('guide/concepts')}>{t.concepts}</Link>
          <Link className={linkClass} href={doc('guide/data-model')}>{t.dataModel}</Link>
          <Link className={linkClass} href={doc('rust')}>{t.api} · Rust</Link>
          <Link className={linkClass} href={doc('python')}>{t.api} · Python</Link>
          <Link className={linkClass} href={doc('go')}>{t.api} · Go</Link>
        </Column>

        <Column title={t.forAI}>
          <Link className={linkClass} href={doc('guide/guides/llm')}>{t.howTo}</Link>
          <a className={`${linkClass} font-mono`} href="/llms.txt">
            llms.txt <span className="font-sans">· {t.llmsIndex}</span>
          </a>
          <a className={`${linkClass} font-mono`} href="/llms-full.txt">
            llms-full.txt <span className="font-sans">· {t.llmsFull}</span>
          </a>
        </Column>

        <Column title={t.project}>
          <Link className={linkClass} href={doc('guide/about')}>{t.about}</Link>
          <Link className={linkClass} href={doc('guide/about/accuracy')}>{t.accuracy}</Link>
          <Link className={linkClass} href={doc('guide/about/architecture')}>{t.architecture}</Link>
          <a className={linkClass} href={REPO_URL} target="_blank" rel="noreferrer">GitHub</a>
          <a className={linkClass} href={LEARN_URL} target="_blank" rel="noreferrer">{t.learn}</a>
        </Column>
      </div>
    </footer>
  );
}

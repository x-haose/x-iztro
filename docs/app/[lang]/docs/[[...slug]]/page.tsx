import { source } from '@/lib/source';
import {
  DocsBody,
  DocsDescription,
  DocsPage,
  DocsTitle,
  MarkdownCopyButton,
  ViewOptionsPopover,
} from 'fumadocs-ui/layouts/notebook/page';
import { notFound } from 'next/navigation';
import { getMDXComponents } from '@/components/mdx';
import { REPO_URL } from '@/lib/layout.shared';
import { SITE_URL, languageAlternates } from '@/lib/site';
import type { Metadata } from 'next';

export default async function Page(props: PageProps<'/[lang]/docs/[[...slug]]'>) {
  const params = await props.params;
  const page = source.getPage(params.slug, params.lang);
  if (!page) notFound();

  const MDX = page.data.body;

  return (
    <DocsPage toc={page.data.toc}>
      <DocsTitle>{page.data.title}</DocsTitle>
      <DocsDescription>{page.data.description}</DocsDescription>
      {/*
        页面操作：复制本页 Markdown，或在下拉里选择用哪个 AI 打开、去 GitHub 看源文件。
        两者都指向同一个 `.md` 端点，与 AI 代理经 Accept 协商拿到的是同一份内容。
      */}
      <div className="mb-6 flex flex-row items-center gap-2 border-b pb-6">
        <MarkdownCopyButton markdownUrl={`${page.url}.md`} />
        <ViewOptionsPopover
          markdownUrl={`${page.url}.md`}
          githubUrl={`${REPO_URL}/blob/main/docs/content/docs/${page.path}`}
        />
      </div>
      <DocsBody>
        <MDX components={getMDXComponents()} />
      </DocsBody>
    </DocsPage>
  );
}

export async function generateStaticParams() {
  return source.generateParams();
}

export async function generateMetadata(
  props: PageProps<'/[lang]/docs/[[...slug]]'>,
): Promise<Metadata> {
  const params = await props.params;
  const page = source.getPage(params.slug, params.lang);
  if (!page) notFound();

  return {
    title: page.data.title,
    description: page.data.description,
    // 分享卡按页面语言取对应的社交图（docs/public/og*.png）。
    openGraph: {
      title: page.data.title,
      description: page.data.description,
      images: [{ url: params.lang === 'en' ? '/og-en.png' : '/og.png', width: 2400, height: 1260 }],
    },
    twitter: { card: 'summary_large_image' },
    alternates: {
      canonical: `${SITE_URL}${page.url}`,
      // 本页在各语言下的地址，与 sitemap 的 hreflang 同源。
      languages: languageAlternates(
        (l) => `${SITE_URL}/${l}${page.url.slice(params.lang.length + 1)}`,
      ),
      // 声明本页的 Markdown 形态。AI 代理据此发现纯文本版本，
      // 不必知道「URL 追加 .md」这个站内约定。
      types: { 'text/markdown': `${SITE_URL}${page.url}.md` },
    },
  };
}

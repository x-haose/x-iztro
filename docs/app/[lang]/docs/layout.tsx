import { source } from '@/lib/source';
import { DocsLayout } from 'fumadocs-ui/layouts/notebook';
import { baseOptions } from '@/lib/layout.shared';
import { Footer } from '@/components/footer';

/**
 * 文档版式。
 *
 * 用 notebook 版式并把栏目切换放进顶栏：x-iztro、Rust、Python、Go
 * 是四个并列的一级栏目，平铺在顶端一次点击即可切换，
 * 侧栏因此只承载当前栏目的页面树。
 */
export default async function Layout({ params, children }: LayoutProps<'/[lang]/docs'>) {
  const { lang } = await params;

  return (
    <DocsLayout
      {...baseOptions(lang)}
      tree={source.getPageTree(lang)}
      tabMode="navbar"
      nav={{ ...baseOptions(lang).nav, mode: 'top' }}
      // 栏目切换已在顶栏，再挂一份指南/API 链接是重复
      links={[]}
    >
      {children}
      {/*
        文档页也挂页脚：页脚里「给 AI 用」的三个端点不出现在任何页面正文中，
        而按 URL 抓文档的人恰好停在文档页上，这里是唯一露出的机会。
        notebook 版式的容器是 CSS 网格，`col-span-full` 让页脚落在
        正文与侧栏下方的整行，不占具名区域之间的空格子。
      */}
      <Footer lang={lang} className="col-span-full" />
    </DocsLayout>
  );
}

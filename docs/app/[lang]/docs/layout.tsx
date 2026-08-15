import { source } from '@/lib/source';
import { DocsLayout } from 'fumadocs-ui/layouts/notebook';
import { baseOptions } from '@/lib/layout.shared';

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
    </DocsLayout>
  );
}

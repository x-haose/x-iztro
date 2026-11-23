import { createI18nMiddleware } from 'fumadocs-core/i18n/middleware';
import { isMarkdownPreferred, rewritePath } from 'fumadocs-core/negotiation';
import { NextResponse, type NextFetchEvent, type NextRequest } from 'next/server';
import { i18n } from '@/lib/i18n';

const i18nMiddleware = createI18nMiddleware(i18n);

/**
 * 文档页路径 → 该页 Markdown 原文端点。
 *
 * 两条规则：通配那条覆盖 `/{lang}/docs/**` 下的所有页面，
 * 但通配段不匹配空串，栏目首页 `/{lang}/docs` 要另一条兜住。
 */
const toMarkdown = [
  rewritePath('/:lang/docs/*path', '/llms.mdx/:lang/docs/*path'),
  rewritePath('/:lang/docs', '/llms.mdx/:lang/docs'),
];

/**
 * 请求入口。
 *
 * AI 代理通常不知道「URL 追加 .md」这个约定，但会在 `Accept` 里声明
 * 更想要 Markdown。这里据此把文档页请求导向 Markdown 原文端点，
 * 让代理拿到几 KB 的正文而不是几十 KB 的整页 HTML。
 *
 * 两种方式可以同时出现（`.md` 结尾且 `Accept: text/markdown`）。
 * 中间件跑在 next.config 的 rewrite 之前，此时 `.md` / `.mdx` 后缀还在路径上，
 * 直接改写会把后缀带进 slug 而查不到页面，故先剥掉再改写。
 *
 * 其余请求交给语言协商中间件。
 */
export default function proxy(request: NextRequest, event: NextFetchEvent) {
  if (isMarkdownPreferred(request)) {
    const path = request.nextUrl.pathname.replace(/\.mdx?$/, '');
    for (const rule of toMarkdown) {
      const rewritten = rule.rewrite(path);
      if (rewritten) return NextResponse.rewrite(new URL(rewritten, request.url));
    }
  }

  return i18nMiddleware(request, event);
}

export const config = {
  /**
   * 跳过内部资源与带扩展名的路径。
   *
   * `_next` 与 `api` 是框架内部路由；`llms.mdx` 是单页 Markdown 的内部
   * 重写目标，不该再被语言协商重定向到 `/zh/...`。
   *
   * 末段带扩展名的路径同样跳过：`robots.txt`、`sitemap.xml`、`llms.txt`、
   * `llms-full.txt` 与 `public/` 下的静态文件都是按固定路径取用的，
   * 加语言前缀就取不到了。`.md` / `.mdx` 是例外——文档页追加后缀取原文时
   * 仍要走上面的协商与语言前缀补全。
   *
   * 带语言前缀的 `/{lang}/llms.txt` 与 `/{lang}/llms-full.txt` 因此也不进
   * 中间件：路径已带合法语言段，本就无须协商。
   */
  matcher: ['/((?!api|_next|llms\\.mdx|.*\\.(?!mdx?$)[^/.]+$).*)'],
};

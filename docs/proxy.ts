import { createI18nMiddleware } from 'fumadocs-core/i18n/middleware';
import { isMarkdownPreferred, rewritePath } from 'fumadocs-core/negotiation';
import { NextResponse, type NextFetchEvent, type NextRequest } from 'next/server';
import { i18n } from '@/lib/i18n';

const i18nMiddleware = createI18nMiddleware(i18n);

/** 文档页路径 → 该页 Markdown 原文端点。 */
const toMarkdown = rewritePath('/:lang/docs/*path', '/llms.mdx/:lang/docs/*path');

/**
 * 请求入口。
 *
 * AI 代理通常不知道「URL 追加 .md」这个约定，但会在 `Accept` 里声明
 * 更想要 Markdown。这里据此把文档页请求导向 Markdown 原文端点，
 * 让代理拿到几 KB 的正文而不是几十 KB 的整页 HTML。
 *
 * 其余请求交给语言协商中间件。
 */
export default function proxy(request: NextRequest, event: NextFetchEvent) {
  if (isMarkdownPreferred(request)) {
    const rewritten = toMarkdown.rewrite(request.nextUrl.pathname);
    if (rewritten) return NextResponse.rewrite(new URL(rewritten, request.url));
  }

  return i18nMiddleware(request, event);
}

export const config = {
  /**
   * 跳过内部资源与不分语言的端点。
   *
   * `llms.txt` / `llms-full.txt` / `llms.mdx` 是给模型抓取的纯文本，
   * `robots.txt` / `sitemap.xml` 是爬虫按固定路径找的文件——
   * 两类都不该被语言协商重定向到 `/zh/...`。
   */
  matcher: [
    '/((?!api|_next/static|_next/image|favicon.ico|robots\\.txt|sitemap\\.xml|llms\\.txt|llms-full\\.txt|llms\\.mdx).*)',
  ],
};

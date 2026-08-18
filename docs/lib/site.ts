import { i18n, type Language } from '@/lib/i18n';

/**
 * 站点绝对地址。
 *
 * `robots.txt`、`sitemap.xml` 与页面 `<link rel="alternate">` 都要绝对 URL，
 * 而这些内容在构建期生成，拿不到运行期的请求主机名。
 * 部署时用 `NEXT_PUBLIC_SITE_URL` 指定；本地开发回落到 dev server 地址。
 */
export const SITE_URL = (
  process.env.NEXT_PUBLIC_SITE_URL ?? 'http://localhost:3000'
).replace(/\/$/, '');

/**
 * 一处内容在各语言下的绝对地址，键为 hreflang 值。
 *
 * `toUrl` 把语言代码映射成该语言的地址。除各语言自身外附一条 `x-default`
 * 指向默认语言，供搜索引擎在读者语言不在支持列表时选用。
 * sitemap 与页面 `<head>` 两处的 hreflang 由此保持同一份取值。
 */
export function languageAlternates(
  toUrl: (lang: Language) => string,
): Record<string, string> {
  return Object.fromEntries([
    ...i18n.languages.map((lang) => [lang, toUrl(lang)]),
    ['x-default', toUrl(i18n.defaultLanguage)],
  ]);
}

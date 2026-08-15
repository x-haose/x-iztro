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

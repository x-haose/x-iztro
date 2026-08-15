import { defineI18n } from 'fumadocs-core/i18n';

/**
 * 文档站语言配置。
 *
 * 中文为默认语言，其内容文件不带语言后缀（`index.mdx`）；
 * 英文内容以 `.en.mdx` 后缀并存于同一目录。
 * URL 始终携带语言前缀（`/zh/docs/...`、`/en/docs/...`），
 * 使两种语言的链接可以互相分享而不依赖浏览器语言协商。
 */
export const i18n = defineI18n({
  defaultLanguage: 'zh',
  languages: ['zh', 'en'],
});

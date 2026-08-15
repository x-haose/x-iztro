import type { MetadataRoute } from 'next';
import { SITE_URL } from '@/lib/site';

/**
 * 爬虫规则。
 *
 * 全站允许抓取，并指向 sitemap。给模型准备的三个端点
 * （`/llms.txt`、`/llms-full.txt`、单页 `.md`）都是纯文本，
 * 同样开放——它们正是希望被抓取的东西。
 */
export default function robots(): MetadataRoute.Robots {
  return {
    rules: { userAgent: '*', allow: '/' },
    sitemap: `${SITE_URL}/sitemap.xml`,
  };
}

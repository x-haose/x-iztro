import type { MetadataRoute } from 'next';
import { source } from '@/lib/source';
import { i18n } from '@/lib/i18n';
import { SITE_URL } from '@/lib/site';

/** 站点地图：各语言首页与全部文档页。 */
export default function sitemap(): MetadataRoute.Sitemap {
  const entries: MetadataRoute.Sitemap = i18n.languages.map((lang) => ({
    url: `${SITE_URL}/${lang}`,
    changeFrequency: 'weekly',
    priority: 1,
  }));

  for (const lang of i18n.languages) {
    for (const page of source.getPages(lang)) {
      entries.push({
        url: `${SITE_URL}${page.url}`,
        changeFrequency: 'weekly',
        priority: 0.8,
      });
    }
  }

  return entries;
}

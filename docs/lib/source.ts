import { loader } from 'fumadocs-core/source';
import { docs } from '@/.source/server';
import { i18n } from '@/lib/i18n';
import { createElement } from 'react';
import * as icons from 'lucide-react';

/** 文档页面树与页面查询入口，按语言切分。 */
export const source = loader({
  baseUrl: '/docs',
  source: docs.toFumadocsSource(),
  i18n,
  /** meta.json 的 `icon` 字段按名字解析为 lucide 图标组件。 */
  icon(icon) {
    if (!icon) return;
    if (icon in icons) {
      return createElement(icons[icon as keyof typeof icons] as React.FC);
    }
  },
});

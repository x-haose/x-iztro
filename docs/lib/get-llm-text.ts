import type { source } from '@/lib/source';

/**
 * 把一篇文档转成 LLM 可直接消费的 Markdown 全文。
 *
 * 输出保留标题、页面 URL 与正文原文，供 `/llms-full.txt` 汇总
 * 与单页 `.md` 端点复用。
 */
export async function getLLMText(page: (typeof source)['$inferPage']) {
  const processed = await page.data.getText('processed');

  return `# ${page.data.title} (${page.url})

${page.data.description ?? ''}

${processed}`;
}

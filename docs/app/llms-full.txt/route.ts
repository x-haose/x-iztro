import { source } from '@/lib/source';
import { getLLMText } from '@/lib/get-llm-text';

/** 内容随构建产物固定，无需重新验证。 */
export const revalidate = false;

/** `/llms-full.txt`：中文全站文档的 Markdown 全文，一次抓取即可喂给模型。 */
export async function GET() {
  const pages = source.getPages('zh');
  const texts = await Promise.all(pages.map(getLLMText));

  return new Response(texts.join('\n\n'), {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
}

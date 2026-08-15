import { source } from '@/lib/source';
import { llms } from 'fumadocs-core/source';

/** 内容随构建产物固定，无需重新验证。 */
export const revalidate = false;

/** `/llms.txt`：站点结构索引，供 LLM 定位需要抓取的页面。 */
export function GET() {
  return new Response(llms(source).index(), {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
}

import { llmsIndex } from '@/lib/llms';
import { i18n } from '@/lib/i18n';

/** 内容随构建产物固定，无需重新验证。 */
export const revalidate = false;

/**
 * `/llms.txt`：站点结构索引，供 LLM 定位需要抓取的页面。
 *
 * 这是 llms.txt 约定的根路径，内容取默认语言；
 * 末尾的交叉指路小节列出其余语言的 `/{lang}/llms.txt` 与各语言全文端点。
 */
export function GET() {
  return new Response(llmsIndex(i18n.defaultLanguage), {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
}

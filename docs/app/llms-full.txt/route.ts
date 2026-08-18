import { llmsFullText } from '@/lib/llms';
import { i18n } from '@/lib/i18n';

/** 内容随构建产物固定，无需重新验证。 */
export const revalidate = false;

/**
 * `/llms-full.txt`：全站文档的 Markdown 全文，一次抓取即可喂给模型。
 *
 * 根路径取默认语言。全文按语言分开而非合并成一份，
 * 是因为它的用途就是整份塞进上下文，混入用不上的语言只会挤占窗口。
 * 其余语言在 `/{lang}/llms-full.txt`。
 */
export async function GET() {
  return new Response(await llmsFullText(i18n.defaultLanguage), {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
}

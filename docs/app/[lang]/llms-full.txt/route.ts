import { llmsFullText } from '@/lib/llms';
import { i18n, isLanguage } from '@/lib/i18n';
import { notFound } from 'next/navigation';

/** 内容随构建产物固定，无需重新验证。 */
export const revalidate = false;

/** `/{lang}/llms-full.txt`：该语言全站文档的 Markdown 全文。 */
export async function GET(
  _req: Request,
  { params }: { params: Promise<{ lang: string }> },
) {
  const { lang } = await params;
  if (!isLanguage(lang)) notFound();

  return new Response(await llmsFullText(lang), {
    headers: { 'Content-Type': 'text/plain; charset=utf-8' },
  });
}

export function generateStaticParams() {
  return i18n.languages.map((lang) => ({ lang }));
}

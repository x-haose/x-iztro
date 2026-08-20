import '@/app/global.css';
import { RootProvider } from 'fumadocs-ui/provider/next';
import { translations } from '@/lib/layout.shared';
import { i18nProvider } from 'fumadocs-ui/i18n';
import { SITE_URL } from '@/lib/site';
import type { Metadata } from 'next';

/** 相对路径元数据（og:image 等）的解析基址，与 canonical / hreflang 同一来源。 */
export const metadata: Metadata = { metadataBase: new URL(SITE_URL) };

export default async function Layout({ params, children }: LayoutProps<'/[lang]'>) {
  const { lang } = await params;
  return (
    <html lang={lang} suppressHydrationWarning>
      <body
        style={{
          display: 'flex',
          flexDirection: 'column',
          minHeight: '100vh',
        }}
      >
        <RootProvider i18n={i18nProvider(translations, lang)}>{children}</RootProvider>
      </body>
    </html>
  );
}

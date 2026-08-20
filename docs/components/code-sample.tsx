import { highlight } from 'fumadocs-core/highlight';
import type { ReactNode } from 'react';

/**
 * 落地页的静态代码示例。
 *
 * 用与文档正文同一套 Shiki 主题（source.config.ts 的 github-light /
 * github-dark-default）做服务端高亮，亮暗切换由 fumadocs 预设里的 `.shiki`
 * 规则接管（token 颜色走 --shiki-light / --shiki-dark 变量）。
 *
 * 外观走落地页自己的卡片样式：剥掉 Shiki 内联在 `<pre>` 上的主题背景色，
 * 背景与边框由调用方的 className 决定；横向内边距经 --padding-left/right
 * 交给 `.line` 规则，调用方不要再加 px-*。
 */
export async function CodeSample({
  code,
  lang,
  className,
}: {
  code: string;
  lang: string;
  className?: string;
}): Promise<ReactNode> {
  return highlight(code, {
    lang,
    themes: { light: 'github-light', dark: 'github-dark-default' },
    // 亮暗色都以 --shiki-* 变量输出（而非把亮色内联成实际 color），
    // 否则暗色页面会顶着亮色主题的近黑前景。
    defaultColor: false,
    components: {
      pre: ({ style: _style, className: shikiClass, ...props }) => (
        <pre className={[shikiClass, className].filter(Boolean).join(' ')} {...props} />
      ),
    },
  });
}

import { Cards } from 'fumadocs-ui/components/card';
import type { ComponentProps } from 'react';

/**
 * 四种语言并列的卡片组。栏数按正文宽度切换：42rem 以上四张等宽一行，24–42rem 两两一行，更窄单列。
 *
 * 只放 Rust、Python、Go、JavaScript / TypeScript 四张，与其他卡片分组：
 * 默认的 `Cards` 是两栏，四张语言卡混在多于四张的组里会断在行中间。
 * `Cards` 自身是容器，容器查询只对后代生效，所以外面再包一层容器给它查；
 * fumadocs 的 `Card` 在容器窄于 32rem 时自己占满一行，这里统一改为只占一格，栏数全由网格决定。
 */
export function LanguageCards({ className, ...props }: ComponentProps<typeof Cards>) {
  return (
    <div className="@container">
      <Cards className={className ? `${COLUMNS} ${className}` : COLUMNS} {...props} />
    </div>
  );
}

const COLUMNS = 'grid-cols-1 @sm:grid-cols-2 @2xl:grid-cols-4 [&>[data-card]]:col-span-1';

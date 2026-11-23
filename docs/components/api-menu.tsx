import {
  NavbarMenu,
  NavbarMenuContent,
  NavbarMenuLink,
  NavbarMenuTrigger,
} from 'fumadocs-ui/layouts/home/navbar';
import type { ReactNode } from 'react';

export interface ApiMenuItem {
  text: string;
  description: string;
  url: string;
  icon: ReactNode;
}

/**
 * 顶栏「API 参考」下拉：四种语言在宽屏一行四张等宽，中屏两两一行，窄屏单列。
 *
 * fumadocs 内置的 `type: 'menu'` 下拉固定为最多三栏，四张卡会把最后一张挤到第二行，
 * 所以桌面顶栏用这个组件（`type: 'custom'`），移动端菜单仍用内置的 `menu`。
 * 卡片的内部结构与内置下拉项一致：图标、名称、一行说明。
 */
export function ApiNavMenu({ text, items }: { text: string; items: ApiMenuItem[] }) {
  return (
    <NavbarMenu>
      <NavbarMenuTrigger>{text}</NavbarMenuTrigger>
      <NavbarMenuContent className="lg:grid-cols-4">
        {items.map((item) => (
          <NavbarMenuLink key={item.url} href={item.url}>
            <div className="w-fit rounded-md border bg-fd-muted p-1 [&_svg]:size-4">{item.icon}</div>
            <p className="text-base font-medium">{item.text}</p>
            <p className="text-sm text-fd-muted-foreground">{item.description}</p>
          </NavbarMenuLink>
        ))}
      </NavbarMenuContent>
    </NavbarMenu>
  );
}

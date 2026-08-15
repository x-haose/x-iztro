import defaultMdxComponents from 'fumadocs-ui/mdx';
import { File, Files, Folder } from 'fumadocs-ui/components/files';
import { Accordion, Accordions } from 'fumadocs-ui/components/accordion';
import { Step, Steps } from 'fumadocs-ui/components/steps';
import { Tab, Tabs } from 'fumadocs-ui/components/tabs';
import { TypeTable } from 'fumadocs-ui/components/type-table';
import { Endpoint } from '@/components/endpoint';
import type { MDXComponents } from 'mdx/types';

/**
 * MDX 可用组件表。
 *
 * 除 fumadocs 默认组件（Callout、Cards、Card、代码块等）外注册：
 * - 文件树：概念页展示星盘的数据层次
 * - 折叠块：API 条目的「边界与陷阱」逐条收纳，不撑长正文
 * - 步骤条：多步流程
 * - 页签：同一件事的多种写法并排
 * - 类型表：参数与字段的结构化呈现
 * - 端点地址：按当前域名渲染站内端点的完整 URL
 */
export function getMDXComponents(components?: MDXComponents) {
  return {
    ...defaultMdxComponents,
    Files,
    Folder,
    File,
    Accordion,
    Accordions,
    Step,
    Steps,
    Tab,
    Tabs,
    TypeTable,
    Endpoint,
    ...components,
  } satisfies MDXComponents;
}

export const useMDXComponents = getMDXComponents;

declare global {
  type MDXProvidedComponents = ReturnType<typeof getMDXComponents>;
}

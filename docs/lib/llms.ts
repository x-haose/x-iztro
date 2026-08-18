import { llms } from 'fumadocs-core/source';
import type * as PageTree from 'fumadocs-core/page-tree';
import { source } from '@/lib/source';
import { i18n, type Language } from '@/lib/i18n';
import { getLLMText } from '@/lib/get-llm-text';

/**
 * 各语言的端点地址与条目文案，用于在索引末尾互相指路。
 *
 * 文案带英文括注：抓到中文那份的模型未必读中文，反之亦然。
 */
const ENDPOINTS: Record<
  Language,
  { index: string; indexLabel: string; full: string; fullLabel: string }
> = {
  zh: {
    index: '/zh/llms.txt',
    indexLabel: '中文索引 (Chinese index)',
    full: '/zh/llms-full.txt',
    fullLabel: '中文全文 (Chinese full text)',
  },
  en: {
    index: '/en/llms.txt',
    indexLabel: 'English index',
    full: '/en/llms-full.txt',
    fullLabel: 'English full text',
  },
};

/**
 * 索引末尾的交叉指路小节。
 *
 * 单看某个语言的 `llms.txt` 看不到另一种语言的存在，也看不到全文端点，
 * 因此每份索引都在末尾列出本语言的全文与其余语言的入口。
 */
function crossReferences(lang: Language): string {
  const here = ENDPOINTS[lang];
  const others = i18n.languages.filter((l) => l !== lang).map((l) => ENDPOINTS[l]);

  return [
    '',
    '## Full text and other languages',
    '',
    `- [${here.fullLabel}](${here.full})`,
    ...others.flatMap((o) => [
      `- [${o.indexLabel}](${o.index})`,
      `- [${o.fullLabel}](${o.full})`,
    ]),
    '',
  ].join('\n');
}

/** 某个语言的站点结构索引：页面标题、描述与链接，末尾附交叉指路。 */
export function llmsIndex(lang: Language): string {
  return llms(source).index(lang) + '\n' + crossReferences(lang);
}

/**
 * 按页面树顺序展开出页面节点。
 *
 * 文件夹先出自己的 index 页再出子页，分隔符不对应页面故跳过，
 * 结果即侧栏自上而下的阅读顺序。
 */
function itemsInTreeOrder(nodes: PageTree.Node[]): PageTree.Item[] {
  return nodes.flatMap((node) => {
    if (node.type === 'page') return [node];
    if (node.type === 'folder') {
      return [...(node.index ? [node.index] : []), ...itemsInTreeOrder(node.children)];
    }
    return [];
  });
}

/**
 * 某个语言的全部页面，按导航顺序排列。
 *
 * 顺序取自页面树（各级 meta.json 的 `pages` 定义），而非文件扫描顺序——
 * 全文是给模型一次读完的，章节次序必须与人读文档的次序一致。
 * 未被任何 meta.json 列入的页面不在树上，补在末尾，避免漏掉正文。
 */
function pagesInReadingOrder(lang: Language) {
  const tree = source.getPageTree(lang);
  const ordered = itemsInTreeOrder(tree.children)
    .map((item) => source.getNodePage(item, lang))
    .filter((page) => page !== undefined);

  const seen = new Set(ordered.map((page) => page.url));
  const rest = source.getPages(lang).filter((page) => !seen.has(page.url));

  return [...ordered, ...rest];
}

/** 某个语言的全站文档 Markdown 全文，一次抓取即可作为完整上下文。 */
export async function llmsFullText(lang: Language): Promise<string> {
  const texts = await Promise.all(pagesInReadingOrder(lang).map(getLLMText));
  return texts.join('\n\n');
}

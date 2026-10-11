import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();

/**
 * Dokploy 上 MCP 服务的源站。
 *
 * 文档首页 `/` 仍由本站渲染。`/mcp`、`/healthz`、`/install` 经 rewrite
 * 反代到这里。Vercel 在构建期读取 `ZIWEI_MCP_ORIGIN`；未设置或空白时用这个地址。
 */
const DEFAULT_MCP_ORIGIN =
  'https://xziwei-xziwei-e0rno8-e6d7d0-161-118-223-190.sslip.io';

/** 源站根地址，去掉末尾斜杠，避免和路径拼出双斜杠。 */
function mcpOrigin() {
  const configured = process.env.ZIWEI_MCP_ORIGIN?.trim();
  return (configured || DEFAULT_MCP_ORIGIN).replace(/\/+$/, '');
}

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  /** 不自动生成 AGENTS.md / CLAUDE.md，本仓库的约定文件由项目自己维护。 */
  agentRules: false,
  /**
   * 两类重写，互不覆盖。
   *
   * beforeFiles 把 MCP 入口反代到源站，跑在文件系统和 `[lang]` 动态路由之前，
   * `/mcp` 因此不会被当成语言段。不重写 `/`。
   * `/mcp` 是 Streamable HTTP 的端点；`/mcp/:path*` 把它下面的子路径一并送过去。
   * 精确的 `/mcp` 单独一条，避免可选捕获在目标地址上多出一个斜杠。
   * 这几条路径还要从 proxy.ts 的语言协商里排除：协商跑在 rewrite 之前，
   * 先重定向到 `/{lang}/mcp` 的话反代接不到原始路径。
   *
   * afterFiles 让任意文档页追加 `.md` 或 `.mdx` 即得到该页的 Markdown 原文，
   * 供页面上的复制按钮与外部 AI 代理直接抓取。
   * 代理若不知道这个约定，也可以在请求文档页时把 `Accept` 设成
   * `text/markdown`，由 proxy.ts 的内容协商导向同一处。
   */
  async rewrites() {
    const origin = mcpOrigin();
    return {
      beforeFiles: [
        {
          source: '/mcp',
          destination: `${origin}/mcp`,
        },
        {
          source: '/mcp/:path*',
          destination: `${origin}/mcp/:path*`,
        },
        {
          source: '/healthz',
          destination: `${origin}/healthz`,
        },
        {
          source: '/install',
          destination: `${origin}/install`,
        },
      ],
      afterFiles: [
        {
          source: '/:lang/docs/:path*.md',
          destination: '/llms.mdx/:lang/docs/:path*',
        },
        {
          source: '/:lang/docs/:path*.mdx',
          destination: '/llms.mdx/:lang/docs/:path*',
        },
      ],
    };
  },
};

export default withMDX(config);

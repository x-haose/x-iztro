import { createMDX } from 'fumadocs-mdx/next';

const withMDX = createMDX();

/** @type {import('next').NextConfig} */
const config = {
  reactStrictMode: true,
  /** 不自动生成 AGENTS.md / CLAUDE.md，本仓库的约定文件由项目自己维护。 */
  agentRules: false,
  /**
   * 任意文档页追加 `.md` 或 `.mdx` 即得到该页的 Markdown 原文，
   * 供页面上的复制按钮与外部 AI 代理直接抓取。
   *
   * 代理若不知道这个约定，也可以在请求文档页时把 `Accept` 设成
   * `text/markdown`，由 proxy.ts 的内容协商导向同一处。
   */
  async rewrites() {
    return [
      {
        source: '/:lang/docs/:path*.md',
        destination: '/llms.mdx/:lang/docs/:path*',
      },
      {
        source: '/:lang/docs/:path*.mdx',
        destination: '/llms.mdx/:lang/docs/:path*',
      },
    ];
  },
};

export default withMDX(config);

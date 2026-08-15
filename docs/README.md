# x-iztro 文档站

基于 [Fumadocs](https://fumadocs.dev)（Next.js App Router）构建，中英双语，
部署到 Cloudflare Workers。

## 安装

```bash
cd docs && npm install
```

`postinstall` 会自动运行 `fumadocs-mdx` 生成 `.source/`（页面索引，已在 `.gitignore` 中）。

技术栈：fumadocs v16 + Next.js 16（Turbopack）+ Tailwind v4 + React 19。

## 开发

```bash
npm run dev          # http://localhost:3000
npm run types:check  # 类型检查
npm run build        # 生产构建
```

## 部署到 Cloudflare

```bash
npx wrangler login   # 首次
npm run preview      # 本地跑一次 Workers 运行时
npm run deploy       # 部署
```

`wrangler.jsonc` 里的 `name` 决定 `*.workers.dev` 子域，自定义域在 Cloudflare 控制台绑定。

## 目录

| 路径 | 内容 |
| --- | --- |
| `content/docs/` | 文档正文。无后缀为中文，`.en.mdx` 为英文 |
| `lib/i18n.ts` | 语言配置（默认 `zh`） |
| `lib/source.ts` | 页面树与查询入口 |
| `lib/layout.shared.tsx` | 导航栏与站点标识 |
| `app/[lang]/` | 页面路由 |
| `app/llms.txt/`、`app/llms-full.txt/`、`app/llms.mdx/` | 给 LLM 的纯文本端点 |
| `source.config.ts` | MDX 处理配置 |
| `app/global.css` | 设计令牌：亮色宣纸墨黑、暗色深靛夜空，共用紫微紫强调 |
| `components/chart-preview.tsx` | 首页十二宫盘，数据取自真实排盘结果 |

## 新增一页

1. 在 `content/docs/<章节>/` 下建 `<页面>.mdx`，写 frontmatter 的 `title` 与 `description`
2. 在该章节的 `meta.json` 的 `pages` 数组里加上文件名（不含扩展名）
3. 英文版同名加 `.en` 后缀：`<页面>.en.mdx`

## 网络提示

npm 官方源在部分网络下极慢。安装失败或超时时可换源：

```bash
npm install --registry=https://registry.npmmirror.com ...
```

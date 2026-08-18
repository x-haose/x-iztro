# x-iztro 文档站

基于 [Fumadocs](https://fumadocs.dev)（Next.js App Router）构建，中英双语，
部署在 Vercel：<https://ziwei.x-hoase.com>。

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

## 部署

Vercel 项目连着本仓库，Root Directory 为 `docs`：推到 `main` 即发布正式站，
其它分支与 PR 各得一个预览地址。站点绝对地址默认取 `lib/site.ts` 里的正式域名，
预览环境可用 `NEXT_PUBLIC_SITE_URL` 覆盖。

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

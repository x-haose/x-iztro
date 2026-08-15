'use client';

import { useEffect, useState } from 'react';

/**
 * 站内端点的完整地址。
 *
 * 文档是静态生成的，构建期拿不到最终域名。这里在浏览器里用 `location.origin`
 * 拼出真实地址——本地、预览、生产各自显示各自的，读者复制即可用。
 *
 * 服务端渲染与水合前显示相对路径，链接本身两种形态都可点。
 */
export function Endpoint({ path }: { path: string }) {
  const [origin, setOrigin] = useState('');

  useEffect(() => setOrigin(window.location.origin), []);

  return (
    <a
      href={path}
      target="_blank"
      rel="noreferrer"
      className="font-mono text-sm break-all underline-offset-4 hover:underline"
    >
      {origin}
      {path}
    </a>
  );
}

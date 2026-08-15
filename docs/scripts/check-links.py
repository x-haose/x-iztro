#!/usr/bin/env python3
"""校验文档内链与导航配置。

检查两件事：
1. MDX 正文里指向站内文档的链接，目标页面是否存在；
2. meta.json 的 pages 数组引用的条目，是否有对应的 mdx 文件或子目录 index。

在 docs/ 目录下运行：python3 scripts/check-links.py
有问题时以非零状态退出，可直接接进 CI。
"""

from __future__ import annotations

import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent / "content" / "docs"
LOCALES = ("zh", "en")
LINK_RE = re.compile(r"\]\((/(?:%s)/docs[^)\s]*)\)" % "|".join(LOCALES))


def page_urls() -> set[str]:
    """已存在的页面 URL 集合，两种语言各一份。"""
    urls: set[str] = set()
    for path in ROOT.rglob("*.mdx"):
        if path.name.endswith(".en.mdx"):
            continue
        rel = str(path.relative_to(ROOT).with_suffix(""))
        rel = re.sub(r"(^|/)index$", "", rel).strip("/")
        for locale in LOCALES:
            urls.add(f"/{locale}/docs" + (f"/{rel}" if rel else ""))
    return urls


def broken_links(urls: set[str]) -> list[tuple[str, str]]:
    out = []
    for path in ROOT.rglob("*.mdx"):
        for match in LINK_RE.finditer(path.read_text(encoding="utf-8")):
            target = match.group(1).split("#")[0].rstrip("/")
            if target not in urls:
                out.append((str(path.relative_to(ROOT)), match.group(1)))
    return out


def broken_meta() -> list[tuple[str, str]]:
    out = []
    for meta in ROOT.rglob("meta.json"):
        pages = json.loads(meta.read_text(encoding="utf-8")).get("pages", [])
        for entry in pages:
            if entry.startswith("---"):  # 分隔符，不是页面
                continue
            has_file = (meta.parent / f"{entry}.mdx").exists()
            has_dir = (meta.parent / entry / "index.mdx").exists()
            if not has_file and not has_dir:
                out.append((str(meta.relative_to(ROOT)), entry))
    return out


def main() -> int:
    urls = page_urls()
    links = broken_links(urls)
    metas = broken_meta()

    print(f"页面 {len(urls) // len(LOCALES)}｜失效内链 {len(links)}｜meta 失效 {len(metas)}")
    for source, target in links:
        print(f"  失效链接 {source} → {target}")
    for source, entry in metas:
        print(f"  失效导航 {source} → {entry}")

    return 1 if links or metas else 0


if __name__ == "__main__":
    sys.exit(main())

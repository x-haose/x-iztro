#!/usr/bin/env python3
"""从 PR 正文里抽出发布说明，作为 GitHub Release 的描述。

约定见 .github/pull_request_template.md：正文里「## 发布说明」这一节的内容
即 release 描述，抽到下一个二级标题为止。模板注释（HTML 注释）会被剔除。

抽不到内容时以非零码退出——宁可让发版失败，也不要发出一个空描述的 release。

用法：cat pr_body.md | python3 release_notes.py
"""

import re
import sys

HEADING = "## 发布说明"


def extract(body: str) -> str:
    """取「## 发布说明」到下一个二级标题之间的正文。"""
    lines = body.replace("\r\n", "\n").split("\n")

    start = next((i for i, l in enumerate(lines) if l.strip() == HEADING), None)
    if start is None:
        sys.exit(f"PR 正文里没有「{HEADING}」一节，无法生成 release 描述。")

    rest = lines[start + 1 :]
    end = next((i for i, l in enumerate(rest) if l.startswith("## ")), len(rest))
    section = "\n".join(rest[:end])

    # 剔除模板里的填写提示
    section = re.sub(r"<!--.*?-->", "", section, flags=re.DOTALL)

    return section.strip()


def main() -> None:
    notes = extract(sys.stdin.read())
    if not notes:
        sys.exit(f"「{HEADING}」一节是空的，请在 PR 正文里写明本次发布的内容。")
    sys.stdout.write(notes + "\n")


if __name__ == "__main__":
    main()

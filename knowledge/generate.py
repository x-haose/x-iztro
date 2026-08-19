#!/usr/bin/env python3
"""默认知识包生成器：把 iztro-docs 的 learn/ 页面转成 x-iztro 知识包 JSON。

用法：`python3 knowledge/generate.py`（Python 3.10+，纯标准库）。

输入是 GitHub 上 SylarLong/iztro-docs 仓库锁定 commit 的八个 Markdown 页面，
下载后缓存在 `knowledge/source/<COMMIT>/`（已 gitignore，缓存命中则不再联网）。
输出是 `src/data/knowledge/iztro_docs.zh-CN.json`，格式见 `knowledge/SCHEMA.md`。

知识包的键全部是 x-iztro 的语言无关标识，映射表不写死在本脚本里，而是从
Rust 源码现取：星耀取 `src/data/stars.rs` 的 `as_key` 与 `src/i18n/zh_cn.rs`
的 `star_name`，格局取 `src/pattern/keys.rs` 与 `src/i18n/patterns.rs`，
宫位与四化取 `src/data/types.rs` 与 `src/i18n/zh_cn.rs`。这样内核改名时生成器
会立刻失配报错，而不是默默产出对不上的键。

升级来源版本：手动改本文件顶部的 `COMMIT`，删掉旧的 `knowledge/source/` 后重跑。
"""

from __future__ import annotations

import json
import re
import sys
import urllib.request
from pathlib import Path

# ---------------------------------------------------------------- 来源与常量

# 锁定的 iztro-docs commit。生成结果只依赖这一个值，换版本必须手动改这里。
COMMIT = "ec2d58bb8b2a0d243d91212a1e3c87ab866858ee"
RAW_URL = "https://raw.githubusercontent.com/SylarLong/iztro-docs/{commit}/learn/{name}.md"
REPO_URL = "https://github.com/SylarLong/iztro-docs"

# 抓取日期，写进 source.retrievedAt 与 version。
RETRIEVED_AT = "2026-08-19"

# 需要抓取的页面（顺序即解析顺序；dec-star 必须排在 adj-star 之前，见 merge_star）。
PAGES = [
    "major-star",
    "minor-star",
    "dec-star",
    "adj-star",
    "pattern",
    "palace",
    "mutagen",
    "basis",
]

ROOT = Path(__file__).resolve().parent.parent
SOURCE_DIR = ROOT / "knowledge" / "source" / COMMIT
OUT_PATH = ROOT / "src" / "data" / "knowledge" / "iztro_docs.zh-CN.json"

# 天干中文名 → 语言无关标识，用于拆星耀名片里「己土」这类五行值。
STEMS = {
    "甲": "jia", "乙": "yi", "丙": "bing", "丁": "ding", "戊": "wu",
    "己": "ji", "庚": "geng", "辛": "xin", "壬": "ren", "癸": "gui",
}

# 五行中文名 → 语言无关标识。
ELEMENTS = {"木": "wood", "火": "fire", "土": "earth", "金": "metal", "水": "water"}

# 名片字段中文名 → attributes 键名。「五行」单独处理（要拆成三个字段）。
CARD_FIELDS = {
    "斗分": "dipper",
    "化气": "chemistry",
    "职业": "career",
    "职务": "duty",
    "五行色": "elementColor",
    "能量色": "energyColor",
}

# basis.md 术语标题 → concepts 的 slug。页面没有稳定的英文/拼音标识，只能显式维护。
CONCEPT_SLUGS = {
    "性别阴阳": "xing-bie-yin-yang",
    "顺行/逆行": "shun-xing-ni-xing",
    "排盘": "pai-pan",
    "解盘": "jie-pan",
    "本命盘": "ben-ming-pan",
    "大限盘": "da-xian-pan",
    "流年盘": "liu-nian-pan",
    "天地人三才": "tian-di-ren-san-cai",
    "格局": "ge-ju",
    "宫干": "gong-gan",
    "本宫": "ben-gong",
    "对宫": "dui-gong",
    "三合位": "san-he-wei",
    "四正位": "si-zheng-wei",
    "三方四正": "san-fang-si-zheng",
    "天罗、地网": "tian-luo-di-wang",
    "空宫": "kong-gong",
    "夹宫": "jia-gong",
    "天门、地门": "tian-men-di-men",
    "雷门": "lei-men",
    "天宫、地宫": "tian-gong-di-gong",
    "流耀": "liu-yao",
    "遇、加、逢、同宫、同度": "tong-gong",
    "会、照、冲、扶拱": "hui-zhao",
    "坐、锯、守": "zuo-shou",
    "空星、空耀": "kong-xing",
    "暗耀": "an-yao",
    "英星": "ying-xing",
    "将星": "jiang-xing",
    "阴阳五行": "yin-yang-wu-xing",
    "天干地支": "tian-gan-di-zhi",
    "十天干": "shi-tian-gan",
    "十二地支": "shi-er-di-zhi",
    "地支六合": "di-zhi-liu-he",
    "地支三合局": "di-zhi-san-he-ju",
    "地支六冲": "di-zhi-liu-chong",
    "十二时辰": "shi-er-shi-chen",
    "五行局": "wu-xing-ju",
}

# 页面标题与 x-iztro 星耀名对不上时的显式别名（页面标题 → zh-CN 星耀名）。
# 目前所有标题都能按「原样匹配，再去掉尾字『星』」两步对上，故为空；
# 若来源改名，把差异登记在这里而不是改匹配逻辑。
STAR_TITLE_ALIASES: dict[str, str] = {}

# 页面运营文字：与知识内容无关的转载声明、赞助引导，整段丢弃。
DROP_LINE_PATTERNS = [
    re.compile(r"^>?\s*\*?本系列文章为原创文章[^\n]*$", re.M),
    re.compile(r"^[^\n]*硬广[：:]\s*$", re.M),
]

# 引文/示例盘的引导语。古籍引文另存进 quotes，示例星盘整表丢弃，这类引导语会落空，
# 命中即整句删掉；未命中的冒号结尾句只把冒号改成句号，不丢内容。
LEADIN_RE = re.compile(r"以下|如下|描述|记载|定义|原文|例子|断语|口诀|布局|是这样|列出|说$|如次")

# 主星页面「XX星组合」里的交叉引用小节，正文只有一句「见 某组合」。
CROSS_REF_RE = re.compile(r"^见\s*\S+$")


# ---------------------------------------------------------------- 抓取与缓存

def fetch_pages() -> dict[str, str]:
    """按锁定 commit 取回八个页面，命中 `knowledge/source/<COMMIT>/` 缓存则不联网。"""
    SOURCE_DIR.mkdir(parents=True, exist_ok=True)
    pages = {}
    for name in PAGES:
        path = SOURCE_DIR / f"{name}.md"
        if not path.exists():
            url = RAW_URL.format(commit=COMMIT, name=name)
            print(f"下载 {url}")
            with urllib.request.urlopen(url) as resp:
                path.write_bytes(resp.read())
        pages[name] = path.read_text(encoding="utf-8")
    return pages


# ---------------------------------------------------------------- Rust 侧映射

def read_rust(rel: str) -> str:
    return (ROOT / rel).read_text(encoding="utf-8")


def rust_key_map(src: str, enum: str) -> dict[str, str]:
    """抓 `Enum::Variant => "key"` 形式的 as_key match 臂，得到 变体名 → 标识。"""
    return dict(re.findall(rf'{enum}::(\w+)\s*=>\s*"([^"]+)"', src))


def build_star_keys() -> tuple[dict[str, str], dict[str, str]]:
    """返回 (中文星名 → StarKey 标识, StarKey 标识 → 中文星名)。"""
    keys = rust_key_map(read_rust("src/data/stars.rs"), "StarKey")
    names = rust_key_map(read_rust("src/i18n/zh_cn.rs"), "StarKey")
    by_name, by_key = {}, {}
    for variant, name in names.items():
        key = keys[variant]
        assert name not in by_name, f"星名重复：{name}"
        by_name[name] = key
        by_key[key] = name
    return by_name, by_key


def build_pattern_keys() -> tuple[dict[str, str], dict[str, str]]:
    """返回 (中文格局名 → PatternKey 标识, PatternKey 标识 → 中文格局名)。"""
    keys = rust_key_map(read_rust("src/pattern/keys.rs"), "PatternKey")
    names = dict(
        re.findall(r'^\s{4}(\w+):\s*\["([^"]+)"', read_rust("src/i18n/patterns.rs"), re.M)
    )
    by_name = {name: keys[variant] for variant, name in names.items()}
    by_key = {keys[variant]: name for variant, name in names.items()}
    return by_name, by_key


def build_palace_keys() -> tuple[dict[str, str], dict[str, str]]:
    keys = rust_key_map(read_rust("src/data/types.rs"), "Palace")
    names = rust_key_map(read_rust("src/i18n/zh_cn.rs"), "Palace")
    return (
        {name: keys[v] for v, name in names.items()},
        {keys[v]: name for v, name in names.items()},
    )


def build_mutagen_keys() -> tuple[dict[str, str], dict[str, str]]:
    keys = rust_key_map(read_rust("src/data/types.rs"), "Mutagen")
    names = rust_key_map(read_rust("src/i18n/zh_cn.rs"), "Mutagen")
    # zh_cn 里四化叫「禄/权/科/忌」，页面标题叫「化禄」，两边都收。
    by_name = {}
    for variant, short in names.items():
        by_name[short] = keys[variant]
        by_name[f"化{short}"] = keys[variant]
    return by_name, {keys[v]: f"化{n}" for v, n in names.items()}


# ---------------------------------------------------------------- 文本清洗

EMOJI_RE = re.compile(
    "[\U0001F000-\U0001FAFF☀-➿⬀-⯿️‍]"
)
ASTROLABE_RE = re.compile(r'<table class="astrolabe">.*?</table>', re.S)
CARD_RE = re.compile(r'<table class="star-card">(.*?)</table>', re.S)
CONTAINER_RE = re.compile(r"^:::(\w+)([^\n]*)\n(.*?)^:::[ \t]*$", re.S | re.M)
QUOTE_RE = re.compile(r"(?:^>[^\n]*\n?)+", re.M)


def strip_emoji(text: str) -> str:
    return EMOJI_RE.sub("", text)


def strip_tags(text: str) -> str:
    return re.sub(r"<[^>]*>", "", text)


def containers_to_quotes(text: str) -> str:
    """VitePress 的 `:::tip 标题` 容器转成 Markdown 引用块，内容原样保留。"""

    def repl(m: re.Match[str]) -> str:
        title = m.group(2).strip()
        body = m.group(3).strip("\n")
        lines = [f"> **{title}**", ">"] if title else []
        lines += [("> " + ln).rstrip() for ln in body.split("\n")]
        return "\n".join(lines)

    return CONTAINER_RE.sub(repl, text)


def dedent_uniform(text: str) -> str:
    """basis.md 的小节正文整体缩进两格，去掉公共缩进免得整段被当成代码块或嵌套列表。"""
    lines = [ln for ln in text.split("\n") if ln.strip()]
    if not lines:
        return text
    indent = min(len(ln) - len(ln.lstrip(" ")) for ln in lines)
    if indent == 0:
        return text
    return re.sub(r"^ {%d}" % indent, "", text, flags=re.M)


def trim_dangling_leadins(text: str) -> str:
    """去掉因删表/摘引文而落空的引导语（见 LEADIN_RE）；后面跟着列表的引导语保留。"""
    blocks = re.split(r"\n{2,}", text)
    out = []
    for i, block in enumerate(blocks):
        nxt = blocks[i + 1] if i + 1 < len(blocks) else ""
        body = block.rstrip()
        introduces_list = re.match(r"^\s*(?:[-*+]|\d+\.)\s", nxt) is not None
        is_quote_or_list = re.match(r"^\s*(?:>|[-*+]|\d+\.)\s", body) is not None
        if body.endswith("：") and not introduces_list and not is_quote_or_list:
            tail = re.search(r"[^。！？]*：$", body)
            if tail and LEADIN_RE.search(tail.group(0)):
                body = body[: tail.start()].rstrip()
            else:
                body = body[:-1] + "。"
        if body.strip():
            out.append(body)
    return "\n\n".join(out)


def clean(text: str) -> str:
    """把 VitePress/HTML 残留清成纯 Markdown；保留列表、加粗与链接文字。"""
    text = re.sub(r"<script setup>.*?</script>", "", text, flags=re.S)
    text = ASTROLABE_RE.sub("", text)
    text = CARD_RE.sub("", text)
    text = re.sub(r"<Donate\s*/?>", "", text)
    text = re.sub(r"<Badge[^>]*/?>", "", text)
    text = re.sub(r"<img[^>]*/?>", "", text)
    text = re.sub(r"<br\s*/?>", "\n", text)
    text = containers_to_quotes(text)
    # 站内链接只留文字，外链同样只留文字：知识包脱离文档站使用，URL 无意义。
    text = re.sub(r"!?\[([^\]]*)\]\([^)]*\)", r"\1", text)
    for pat in DROP_LINE_PATTERNS:
        text = pat.sub("", text)
    # 页面用 `---` 做视觉分隔，剥离上下文后没有意义。
    text = re.sub(r"^[ \t]*(?:-{3,}|\*{3,})[ \t]*$", "", text, flags=re.M)
    text = dedent_uniform(text)
    text = re.sub(r"[ \t]+\n", "\n", text)
    text = re.sub(r"\n{3,}", "\n\n", text)
    return trim_dangling_leadins(text.strip())


def take_quotes(text: str) -> tuple[list[str], str]:
    """摘出正文里的引用块（古籍引文），返回 (引文列表, 去掉引文的正文)。

    必须在 `clean` 之前调用：`clean` 会把 `:::tip` 容器也变成引用块。
    """
    quotes: list[str] = []
    for block in QUOTE_RE.finditer(text):
        for line in block.group(0).split("\n"):
            line = line.lstrip(">").strip()
            if line:
                quotes.append(line)
    return quotes, QUOTE_RE.sub("\n", text)


def split_sections(text: str, level: int) -> list[tuple[str, str]]:
    """按指定层级的标题切分，返回 [(标题原文, 该节正文含更深层级标题)]。"""
    pat = re.compile(r"^#{%d}(?!#)[ \t]+(.+?)[ \t]*$" % level, re.M)
    marks = list(pat.finditer(text))
    out = []
    for i, m in enumerate(marks):
        end = marks[i + 1].start() if i + 1 < len(marks) else len(text)
        out.append((m.group(1), text[m.end():end]))
    return out


def heading_title(raw: str) -> str:
    """标题去掉 Badge、反引号与首尾空白，得到纯标题文字。"""
    return re.sub(r"<Badge[^>]*/?>", "", raw).replace("`", "").strip()


# ---------------------------------------------------------------- 名片解析

def parse_card(body: str) -> dict[str, object]:
    """解析 `<table class="star-card">` 九宫格，返回 attributes（缺省字段不出现）。"""
    m = CARD_RE.search(body)
    if not m:
        return {}
    cells = [strip_tags(c).strip() for c in re.findall(r"<td>(.*?)</td>", m.group(1), re.S)]
    raw = {k: v for k, v in zip(cells[0::2], cells[1::2])}
    attrs: dict[str, object] = {}

    yin_yang = strip_emoji(raw.get("阴阳", "")).strip()
    if yin_yang:
        attrs["yinYang"] = "yang" if "阳" in yin_yang else "yin"

    attrs.update(parse_five_elements(raw.get("五行", "")))

    for zh, key in CARD_FIELDS.items():
        val = strip_emoji(raw.get(zh, "")).strip()
        if val and val != "-":
            attrs[key] = val

    aliases = strip_emoji(raw.get("别号", "")).strip()
    if aliases and aliases != "-":
        parts = [p.strip() for p in re.split(r"[，、。]", aliases) if p.strip()]
        if parts:
            attrs["aliases"] = parts
    return attrs


def parse_five_elements(value: str) -> dict[str, str]:
    """拆「己土」「甲木(气为水)」「癸水 | 己土 (藏金、木)」为 stem/五行/补充说明。"""
    text = re.sub(r"\s+", " ", strip_emoji(value)).strip()
    if not text or text == "-":
        return {}
    m = re.match(r"([甲乙丙丁戊己庚辛壬癸])?([木火土金水])", text)
    if not m:
        return {}
    out = {}
    if m.group(1):
        out["stem"] = STEMS[m.group(1)]
    out["fiveElements"] = ELEMENTS[m.group(2)]
    note = text[m.end():].strip().lstrip("|").strip()
    # 去 emoji 后残留的空格：括号内外与中文顿号前后都不该有空格。
    note = re.sub(r"\s*([(（])\s*", r"\1", note)
    note = re.sub(r"\s+([、，。)）])", r"\1", note).strip()
    if note.startswith("(") and note.endswith(")"):
        note = note[1:-1].strip()
    if note:
        out["fiveElementsNote"] = note
    return out


# ---------------------------------------------------------------- 星耀

def resolve_star(title: str, by_name: dict[str, str], missed: list[str]) -> list[str]:
    """页面标题 → StarKey 标识列表（「三台星、八座星」这类合写条目返回多个）。"""
    keys = []
    for part in re.split(r"[、,，]", heading_title(title)):
        part = part.strip()
        if not part:
            continue
        part = STAR_TITLE_ALIASES.get(part, part)
        # 先原样匹配再去尾字「星」：火星、铃星本身就以「星」结尾，顺序不能反。
        key = by_name.get(part) or by_name.get(part.removesuffix("星"))
        if key:
            keys.append(key)
        else:
            missed.append(part)
    return keys


def merge_star(stars: dict[str, dict], key: str, entry: dict) -> None:
    """同一 StarKey 被多个页面/分组讲到时合并。

    iztro-docs 把华盖、咸池、天德列进杂耀，同时它们又是将前/岁前12神；
    大耗、小耗、病符在博士12神与岁前12神下各讲一次。x-iztro 的 StarKey 只有一个，
    故先到的条目定 category/group 与属性（PAGES 里 dec-star 排在 adj-star 之前，
    使这几颗按内核实际归属落在神煞），后到的正文以带组别前缀的段落追加，不丢内容。
    """
    old = stars.get(key)
    if old is None:
        stars[key] = entry
        return
    extra = entry.get("intro")
    if not extra:
        return
    label = entry.get("group") or entry.get("category")
    old["intro"] = f"{old.get('intro', '')}\n\n**{label}**：{extra}".strip()


def parse_major_stars(text: str, by_name, stars: dict, missed: list) -> None:
    """主星页：名片 → attributes，「XX星特性」→ intro，「XX星组合」→ combinations。"""
    combos_by_pair: dict[str, str] = {}
    pending: list[tuple[str, str, str]] = []  # (本星 key, 另一星 key, 组合标题)

    for raw_title, body in split_sections(text, 2):
        title = heading_title(raw_title)
        if title == "前言":
            continue
        keys = resolve_star(raw_title, by_name, missed)
        if not keys:
            continue
        key = keys[0]
        entry = {
            "name": by_name and title.removesuffix("星"),
            "category": "major",
            "group": None,
            "attributes": parse_card(body),
            "intro": None,
            "combinations": {},
        }
        for sub_title, sub_body in split_sections(body, 3):
            sub = heading_title(sub_title)
            if sub.endswith("特性"):
                entry["intro"] = clean(sub_body) or None
            elif sub.endswith("组合"):
                for combo_title, combo_body in split_sections(sub_body, 4):
                    pair = heading_title(combo_title)
                    other = pair.replace(title.removesuffix("星"), "", 1).strip()
                    other_key = by_name.get(other)
                    if not other_key:
                        missed.append(pair)
                        continue
                    body_text = clean(combo_body)
                    if body_text and not CROSS_REF_RE.match(body_text):
                        combos_by_pair[pair] = body_text
                    pending.append((key, other_key, pair))
        merge_star(stars, key, entry)

    # 「见 紫微七杀」这类交叉引用小节，回填另一处写全的正文。
    for key, other_key, pair in pending:
        text_ = combos_by_pair.get(pair)
        if text_:
            stars[key]["combinations"][other_key] = text_


def parse_grouped_stars(
    text: str, by_name, stars: dict, missed: list, category: str, with_card: bool
) -> None:
    """辅星/杂耀/神煞页：一级标题是分组，二级标题是星，正文即 intro。"""
    for raw_group, group_body in split_sections(text, 2):
        group = heading_title(raw_group)
        if group == "前言":
            continue
        for raw_title, body in split_sections(group_body, 3):
            keys = resolve_star(raw_title, by_name, missed)
            if not keys:
                continue
            attrs = parse_card(body) if with_card else parse_badge(raw_title)
            intro = clean(body) or None
            for key in keys:
                merge_star(
                    stars,
                    key,
                    {
                        "name": None,  # 调用方按 StarKey 回填规范名
                        "category": category,
                        "group": group,
                        "attributes": dict(attrs),
                        "intro": intro,
                        "combinations": {},
                    },
                )


def parse_badge(raw_title: str) -> dict[str, str]:
    """杂耀/神煞标题里的 `<Badge text="阴水 🌊" />` → yinYang + fiveElements。"""
    m = re.search(r'<Badge[^>]*text="([^"]*)"', raw_title)
    if not m:
        return {}
    val = strip_emoji(m.group(1)).strip()
    attrs: dict[str, str] = {}
    if val.startswith(("阴", "阳")):
        attrs["yinYang"] = "yang" if val[0] == "阳" else "yin"
        val = val[1:]
    val = val.strip()
    if val in ELEMENTS:
        attrs["fiveElements"] = ELEMENTS[val]
    return attrs


# ---------------------------------------------------------------- 格局

# 首段谈成格条件的信号词：命中则该段进 conditions，否则整节都算解读。
CONDITION_HINT = re.compile(r"指|条件|成格|构成|形成|同宫|同度|夹|三方四正|坐守|入庙")


def parse_patterns(text: str, by_name, missed: list) -> dict[str, dict]:
    out: dict[str, dict] = {}
    for raw_title, body in split_sections(text, 2):
        title = heading_title(raw_title)
        if title == "前言":
            continue
        # 「火贪、铃贪」合写一节，拆成两条同文的格局。
        names = [n.strip() for n in title.split("、") if n.strip()]
        keys = []
        for name in names:
            key = by_name.get(name)
            if key:
                keys.append(key)
            else:
                missed.append(name)
        if not keys:
            continue

        quotes, rest = take_quotes(body)
        rest = clean(rest)
        blocks = [b for b in re.split(r"\n{2,}", rest) if b.strip()]
        conditions = None
        if blocks and CONDITION_HINT.search(blocks[0]):
            # 首段之后紧跟的有序列表（如君臣庆会的三种形式）同属成格条件。
            take = 1
            while take < len(blocks) and re.match(r"^\d+\.\s", blocks[take]):
                take += 1
            conditions = "\n\n".join(blocks[:take])
            blocks = blocks[take:]
        intro = "\n\n".join(blocks) or None

        for key in keys:
            out[key] = {
                "name": None,
                "quotes": quotes or None,
                "conditions": conditions,
                "intro": intro,
            }
    return out


# ---------------------------------------------------------------- 宫位与四化

def parse_simple_sections(text: str, by_name, missed: list, strip_suffix: str = "") -> dict:
    """标题即条目名、正文即 intro 的页面（宫位、四化）。"""
    out: dict[str, dict] = {}
    for raw_title, body in split_sections(text, 2):
        title = heading_title(raw_title)
        key = by_name.get(title) or (
            by_name.get(title.removesuffix(strip_suffix)) if strip_suffix else None
        )
        if not key:
            continue
        out[key] = {"name": None, "intro": clean(body) or None}
    return out


def mutagen_card_line(body: str) -> str:
    """四化名片（五行 + 意象）没有对应的 schema 字段，转成正文首行的 Markdown。"""
    m = CARD_RE.search(body)
    if not m:
        return ""
    cells = [strip_tags(c).strip() for c in re.findall(r"<td>(.*?)</td>", m.group(1), re.S)]
    raw = {k: strip_emoji(v).strip() for k, v in zip(cells[0::2], cells[1::2])}
    parts = [f"**{k}**：{v}" for k, v in raw.items() if v]
    return "；".join(parts)


# ---------------------------------------------------------------- 术语概念

# basis.md 里整节收成一条概念的顶级小节（其余顶级小节按子标题/条目拆）。
WHOLE_SECTION_CONCEPTS = {"阴阳五行", "十二时辰", "五行局"}


def split_bullets(text: str) -> list[tuple[str, str]]:
    """拆顶格的 `- 术语` 条目，返回 [(术语, 缩进正文)]；嵌套子列表留在正文里。

    只收带缩进正文的条目：`性别阴阳` 下的「阳男/阴女」是枚举项而非术语定义，跳过。
    """
    marks = list(re.finditer(r"^-[ \t]+(.+?)[ \t]*$", text, re.M))
    out = []
    for i, m in enumerate(marks):
        end = marks[i + 1].start() if i + 1 < len(marks) else len(text)
        body = text[m.end():end]
        if re.search(r"^[ \t]+\S", body, re.M):
            body = re.sub(r"^ {1,2}", "", body, flags=re.M)
            out.append((m.group(1).replace("`", "").strip(), body))
    return out


def parse_concepts(text: str, missed: list) -> dict[str, dict]:
    out: dict[str, dict] = {}

    def add(title: str, body: str) -> None:
        slug = CONCEPT_SLUGS.get(title)
        if not slug:
            missed.append(title)
            return
        intro = clean(body)
        if intro:
            out[slug] = {"title": title, "intro": intro}

    for raw_h2, h2_body in split_sections(text, 2):
        h2 = heading_title(raw_h2)
        if h2 == "术语解释":
            for raw_h3, h3_body in split_sections(h2_body, 3):
                bullets = split_bullets(h3_body)
                if bullets:
                    # 有条目的小节，标题只是「星盘」「宫位」这类指路，条目才是术语。
                    for title, body in bullets:
                        add(title, body)
                else:
                    add(heading_title(raw_h3), h3_body)
        elif h2 in WHOLE_SECTION_CONCEPTS:
            add(h2, h2_body)
        elif h2 == "天干地支":
            subs = split_sections(h2_body, 3)
            head = h2_body[: h2_body.index("### ")] if subs else h2_body
            add(h2, head)
            for raw_h3, h3_body in subs:
                deeper = split_sections(h3_body, 4)
                head = h3_body[: h3_body.index("#### ")] if deeper else h3_body
                add(heading_title(raw_h3), head)
                for raw_h4, h4_body in deeper:
                    add(heading_title(raw_h4), h4_body)
    return out


# ---------------------------------------------------------------- 组装与自检

def drop_empty(entry: dict) -> dict:
    """去掉值为 None 或空容器的字段：缺省与「没写」在知识包里同义。"""
    return {k: v for k, v in entry.items() if v not in (None, {}, [], "")}


def main() -> int:
    pages = fetch_pages()
    star_by_name, star_by_key = build_star_keys()
    pattern_by_name, pattern_by_key = build_pattern_keys()
    palace_by_name, palace_by_key = build_palace_keys()
    mutagen_by_name, mutagen_by_key = build_mutagen_keys()

    missed: list[str] = []
    stars: dict[str, dict] = {}

    parse_major_stars(pages["major-star"], star_by_name, stars, missed)
    parse_grouped_stars(pages["minor-star"], star_by_name, stars, missed, "minor", True)
    parse_grouped_stars(pages["dec-star"], star_by_name, stars, missed, "dec", False)
    parse_grouped_stars(pages["adj-star"], star_by_name, stars, missed, "adjective", False)
    for key, entry in stars.items():
        entry["name"] = star_by_key[key]

    patterns = parse_patterns(pages["pattern"], pattern_by_name, missed)
    for key, entry in patterns.items():
        entry["name"] = pattern_by_key[key]

    palaces = parse_simple_sections(pages["palace"], palace_by_name, missed, "宫")
    for key, entry in palaces.items():
        entry["name"] = palace_by_key[key]

    mutagens = parse_simple_sections(pages["mutagen"], mutagen_by_name, missed)
    for raw_title, body in split_sections(pages["mutagen"], 2):
        key = mutagen_by_name.get(heading_title(raw_title))
        line = mutagen_card_line(body) if key else ""
        if line and mutagens.get(key, {}).get("intro"):
            mutagens[key]["intro"] = f"{line}\n\n{mutagens[key]['intro']}"
    for key, entry in mutagens.items():
        entry["name"] = mutagen_by_key[key]

    concepts = parse_concepts(pages["basis"], missed)

    pack = {
        "schema": 1,
        "id": "iztro-docs",
        "version": f"{RETRIEVED_AT}+{COMMIT[:7]}",
        "language": "zh-CN",
        "extends": None,
        "source": {
            "name": "iztro-docs",
            "url": REPO_URL,
            "commit": COMMIT,
            "license": "MIT",
            "author": "Sylar Long",
            "retrievedAt": RETRIEVED_AT,
        },
        "stars": {k: drop_empty(v) for k, v in stars.items()},
        "patterns": {k: drop_empty(v) for k, v in patterns.items()},
        "palaces": {k: drop_empty(v) for k, v in palaces.items()},
        "mutagens": {k: drop_empty(v) for k, v in mutagens.items()},
        "concepts": concepts,
    }

    OUT_PATH.parent.mkdir(parents=True, exist_ok=True)
    OUT_PATH.write_text(
        json.dumps(pack, ensure_ascii=False, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    # ------------------------------------------------------------ 健康检查
    all_stars = set(star_by_key)
    majors = {k for k in all_stars if k.endswith("Maj")}
    minors = {k for k in all_stars if k.endswith("Min")}
    have = set(pack["stars"])

    print(f"\n输出 {OUT_PATH.relative_to(ROOT)}  {OUT_PATH.stat().st_size / 1024:.1f} KiB")
    print(f"星耀   {len(have)}/{len(all_stars)}"
          f"（主星 {len(majors & have)}/{len(majors)}、"
          f"辅星 {len(minors & have)}/{len(minors)}、"
          f"杂耀 {sum(1 for v in pack['stars'].values() if v['category'] == 'adjective')}、"
          f"神煞 {sum(1 for v in pack['stars'].values() if v['category'] == 'dec')}）")
    print(f"双星组合 {sum(len(v.get('combinations', {})) for v in pack['stars'].values())}")
    print(f"格局   {len(pack['patterns'])}/{len(pattern_by_key)}")
    print(f"宫位   {len(pack['palaces'])}/{len(palace_by_key)}")
    print(f"四化   {len(pack['mutagens'])}/{len(mutagen_by_key)}")
    print(f"概念   {len(pack['concepts'])}/{len(CONCEPT_SLUGS)}")
    if missed:
        print(f"未对上的标题（{len(missed)}）：{sorted(set(missed))}")

    assert majors <= have, f"主星缺条目：{sorted(majors - have)}"
    assert minors <= have, f"辅星缺条目：{sorted(minors - have)}"
    assert set(pattern_by_key) == set(pack["patterns"]), \
        f"格局缺条目：{sorted(set(pattern_by_key) - set(pack['patterns']))}"
    assert set(palace_by_key) == set(pack["palaces"]), \
        f"宫位缺条目：{sorted(set(palace_by_key) - set(pack['palaces']))}"
    assert set(mutagen_by_key) == set(pack["mutagens"]), \
        f"四化缺条目：{sorted(set(mutagen_by_key) - set(pack['mutagens']))}"
    assert not missed, f"有标题没能对上标识：{sorted(set(missed))}"
    print("健康检查通过")
    return 0


if __name__ == "__main__":
    sys.exit(main())

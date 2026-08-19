# 知识包（Knowledge Pack）格式 v1

知识包是「语言无关标识 → 解读文本与属性」的 JSON 文件。x-iztro 内核只负责事实判定
（排盘、运限、格局），星耀怎么解读、格局意味着什么、宫位与四化的含义，都属于门派观点，
放在知识包里：内核内嵌一份默认包（源自 iztro-docs，MIT），使用者可以整包替换或逐条覆盖。

所有键都是 x-iztro 的语言无关标识：星耀用 `StarKey`（`ziweiMaj`）、格局用 `PatternKey`
（`zi_fu_tong_gong`）、宫位用 `Palace`（`soulPalace`）、四化用 `Mutagen`（`sihuaLu`）。
文本字段是 Markdown（允许纯文本）。所有条目与字段都可选：缺什么就是没写。

```jsonc
{
  "schema": 1,                       // 格式版本，当前 1
  "id": "iztro-docs",                // 包标识；覆盖包用自己的 id
  "version": "2026-08-12+a1b2c3d",   // 包版本，默认包用来源 commit
  "language": "zh-CN",               // 文本语言（x-iztro 语言码）
  "extends": null,                   // 覆盖包写被覆盖包的 id；独立包为 null
  "source": {                        // 来源与许可（可选）
    "name": "iztro-docs", "url": "https://github.com/SylarLong/iztro-docs",
    "commit": "a1b2c3d", "license": "MIT", "author": "Sylar Long", "retrievedAt": "2026-08-19",
    "adapted": "文本由 x-iztro 在原文基础上整理改写"   // 文本非逐字转载时注明
  },
  "stars": {
    "ziweiMaj": {
      "name": "紫微",                  // 该语言的显示名（冗余，便于脱离内核使用）
      "category": "major",           // major | minor | adjective | dec
      "group": null,                 // 杂耀的分类（交际类…）、神煞的组别（长生12神…）
      "attributes": {                // 门派属性，全部可选
        "yinYang": "yin",            // yin | yang
        "fiveElements": "earth",     // wood | fire | earth | metal | water
        "stem": "ji",                // 天干（五行带干时）：jia…gui
        "fiveElementsNote": null,    // 五行的补充说明（如「气为水」「藏金木」）
        "dipper": "中天星系",          // 斗分
        "chemistry": "尊贵",           // 化气
        "career": "官禄主",            // 职业（主何事）
        "duty": "众星枢纽，长五行，孕万物", // 职务
        "aliases": ["帝王星", "老板星", "俸禄星"], // 别号
        "elementColor": "黄色",       // 五行色
        "energyColor": "紫光"          // 能量色
      },
      "intro": "…",                  // 星耀特性/解读正文
      "combinations": {              // 主星的双星组合解读（仅主星）
        "tianfuMaj": "…"
      }
    }
  },
  "patterns": {
    "zi_fu_tong_gong": {
      "name": "紫府同宫",
      "quotes": ["紫府同宫终身福厚"], // 古籍引文
      "conditions": "…",             // 来源对成立条件的文字描述
      "intro": "…"                   // 解读正文
    }
  },
  "palaces": { "soulPalace": { "name": "命宫", "intro": "…" } },
  "mutagens": { "sihuaLu": { "name": "化禄", "intro": "…" } },
  "concepts": {                      // 术语与基础概念（键为 slug）
    "tong-gong": { "title": "同宫", "intro": "…" }
  }
}
```

## 合并规则（覆盖包）

以被覆盖包为底，逐段（stars/patterns/palaces/mutagens/concepts）按键合并：
覆盖包里出现的条目，其**非 null 字段**覆盖底包同键条目的对应字段，未出现的字段保留；
`attributes`、`combinations` 同样按字段/子键合并；数组字段（`aliases`、`quotes`）整体替换。
把某字段显式写成 `null` 不删除底包内容（缺省与 null 同义）；要删除请整包替换。

## 默认包的维护方式

内嵌默认包 `src/data/knowledge/iztro_docs.zh-CN.json` 以 iztro-docs 锁定 commit
（`source.commit`）的《学习》页为底本，文本经 x-iztro 整理改写为第三人称释义口吻
（`source.adapted` 注明；命理内容一仍其旧，只换表达）。该 JSON 是手工维护的源文件：
修改内容就是直接编辑它，格式与完整性由 `tests/knowledge_pack.rs`（含口吻红线断言）守护。
同步上游：对照 iztro-docs 新旧 commit 的页面差异，把变化的条目改写后写回，更新 `source.commit`。

# rs-iztro: TypeScript → Rust 移植实现计划

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 将 iztro 紫微斗数库从 TypeScript 完整移植到 Rust，提供 crate 和 PyO3 绑定。

**Architecture:** 分层架构 — data（常量/枚举） → models（结构体） → star（安星算法） → astro（排盘主逻辑） → query（查询方法）。所有计算为纯函数，无副作用。通过 serde 提供 JSON 序列化，通过 PyO3 提供 Python 绑定。

**Tech Stack:** Rust 2024 edition, serde/serde_json（序列化）, lunar_rust（农历转换）, pyo3（Python 绑定）, pretty_assertions（测试）

---

## Task 1: 项目骨架与依赖配置

**Files:**
- Modify: `Cargo.toml`
- Create: `src/lib.rs`
- Create: `src/data/mod.rs`
- Create: `src/models/mod.rs`
- Create: `src/star/mod.rs`
- Create: `src/astro/mod.rs`
- Create: `src/i18n/mod.rs`
- Create: `src/utils.rs`
- Create: `src/prompt.rs`

**Step 1: 更新 Cargo.toml**

```toml
[package]
name = "rs-iztro"
version = "0.1.0"
edition = "2024"
description = "紫微斗数 Rust 核心库"
license = "MIT"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
lunar_rust = "1.0"

[dev-dependencies]
pretty_assertions = "1"
```

**Step 2: 创建目录结构和模块文件**

```rust
// src/lib.rs
pub mod data;
pub mod models;
pub mod star;
pub mod astro;
pub mod i18n;
pub mod utils;
pub mod prompt;
```

每个子模块的 `mod.rs` 先留空或写 `// TODO`。

**Step 3: 验证编译**

Run: `cd /Users/haose/code/rs-iztro && cargo build`
Expected: BUILD SUCCESS

**Step 4: Commit**

```bash
git add -A
git commit -m "feat: 初始化项目骨架与依赖配置"
```

---

## Task 2: 基础枚举与常量 — 天干、地支、宫位

**Files:**
- Create: `src/data/constants.rs`
- Create: `src/data/types.rs`
- Modify: `src/data/mod.rs`

**对应 TS 源码:** `iztro/src/data/constants.ts`, `iztro/src/data/types/general.ts`

**Step 1: 定义核心枚举**

```rust
// src/data/types.rs
use serde::{Serialize, Deserialize};

/// 阴阳
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YinYang {
    Yang, // 阳
    Yin,  // 阴
}

/// 五行
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiveElements {
    Wood,  // 木
    Metal, // 金
    Water, // 水
    Fire,  // 火
    Earth, // 土
}

/// 十天干
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HeavenlyStem {
    Jia,  // 甲
    Yi,   // 乙
    Bing, // 丙
    Ding, // 丁
    Wu,   // 戊
    Ji,   // 己
    Geng, // 庚
    Xin,  // 辛
    Ren,  // 壬
    Gui,  // 癸
}

/// 十二地支
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EarthlyBranch {
    Zi,   // 子
    Chou, // 丑
    Yin,  // 寅
    Mao,  // 卯
    Chen, // 辰
    Si,   // 巳
    Wu,   // 午
    Wei,  // 未
    Shen, // 申
    You,  // 酉
    Xu,   // 戌
    Hai,  // 亥
}

/// 十二宫位
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Palace {
    Soul,     // 命宫
    Parents,  // 父母
    Spirit,   // 福德
    Property, // 田宅
    Career,   // 官禄
    Friends,  // 交友
    Surface,  // 迁移
    Health,   // 疾厄
    Wealth,   // 财帛
    Children, // 子女
    Spouse,   // 夫妻
    Siblings, // 兄弟
}

/// 五行局
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FiveElementsClass {
    Water2nd = 2, // 水二局
    Wood3rd  = 3, // 木三局
    Metal4th = 4, // 金四局
    Earth5th = 5, // 土五局
    Fire6th  = 6, // 火六局
}

/// 四化
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Mutagen {
    Lu,   // 禄
    Quan, // 权
    Ke,   // 科
    Ji,   // 忌
}

/// 星耀亮度
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Brightness {
    Miao, // 庙
    Wang, // 旺
    De,   // 得
    Li,   // 利
    Ping, // 平
    Bu,   // 不
    Xian, // 陷
}

/// 星耀类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StarType {
    Major,     // 主星
    Soft,      // 吉星
    Tough,     // 煞星
    Adjective, // 杂耀
    Flower,    // 桃花星
    Helper,    // 解神
    Lucun,     // 禄存
    Tianma,    // 天马
}

/// 作用范围
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Scope {
    Origin,  // 本命
    Decadal, // 大限
    Yearly,  // 流年
    Monthly, // 流月
    Daily,   // 流日
    Hourly,  // 流时
}

/// 性别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gender {
    Male,   // 男
    Female, // 女
}

/// 语言
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    ZhCN, // 简体中文
    ZhTW, // 繁体中文
    EnUS, // 英文
    JaJP, // 日文
    KoKR, // 韩文
    ViVN, // 越南文
}

/// 安星算法
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    Default,   // 默认
    Zhongzhou, // 中州派
}

/// 时辰索引 (0=早子, 1=丑, ..., 12=晚子)
pub type TimeIndex = u8;
```

**Step 2: 定义常量数组与映射表**

```rust
// src/data/constants.rs
use crate::data::types::*;

/// 天干顺序数组
pub const HEAVENLY_STEMS: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia, HeavenlyStem::Yi, HeavenlyStem::Bing, HeavenlyStem::Ding,
    HeavenlyStem::Wu, HeavenlyStem::Ji, HeavenlyStem::Geng, HeavenlyStem::Xin,
    HeavenlyStem::Ren, HeavenlyStem::Gui,
];

/// 地支顺序数组
pub const EARTHLY_BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi, EarthlyBranch::Chou, EarthlyBranch::Yin, EarthlyBranch::Mao,
    EarthlyBranch::Chen, EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei,
    EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu, EarthlyBranch::Hai,
];

/// 十二宫顺序数组
pub const PALACES: [Palace; 12] = [
    Palace::Soul, Palace::Parents, Palace::Spirit, Palace::Property,
    Palace::Career, Palace::Friends, Palace::Surface, Palace::Health,
    Palace::Wealth, Palace::Children, Palace::Spouse, Palace::Siblings,
];

/// 时辰对应时间范围
pub const TIME_RANGES: [&str; 13] = [
    "00:00~01:00", "01:00~03:00", "03:00~05:00", "05:00~07:00",
    "07:00~09:00", "09:00~11:00", "11:00~13:00", "13:00~15:00",
    "15:00~17:00", "17:00~19:00", "19:00~21:00", "21:00~23:00",
    "23:00~00:00",
];

/// 五虎遁 — 年干 → 月干起始
pub const TIGER_RULE: [HeavenlyStem; 10] = [
    HeavenlyStem::Bing, // 甲 → 丙
    HeavenlyStem::Wu,   // 乙 → 戊
    HeavenlyStem::Geng, // 丙 → 庚
    HeavenlyStem::Ren,  // 丁 → 壬
    HeavenlyStem::Jia,  // 戊 → 甲
    HeavenlyStem::Bing, // 己 → 丙
    HeavenlyStem::Wu,   // 庚 → 戊
    HeavenlyStem::Geng, // 辛 → 庚
    HeavenlyStem::Ren,  // 壬 → 壬
    HeavenlyStem::Jia,  // 癸 → 甲
];

/// 五鼠遁 — 日干 → 时干起始
pub const RAT_RULE: [HeavenlyStem; 10] = [
    HeavenlyStem::Jia,  // 甲 → 甲
    HeavenlyStem::Bing, // 乙 → 丙
    HeavenlyStem::Wu,   // 丙 → 戊
    HeavenlyStem::Geng, // 丁 → 庚
    HeavenlyStem::Ren,  // 戊 → 壬
    HeavenlyStem::Jia,  // 己 → 甲
    HeavenlyStem::Bing, // 庚 → 丙
    HeavenlyStem::Wu,   // 辛 → 戊
    HeavenlyStem::Geng, // 壬 → 庚
    HeavenlyStem::Ren,  // 癸 → 壬
];
```

**Step 3: 注册模块**

```rust
// src/data/mod.rs
pub mod constants;
pub mod types;
```

**Step 4: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 5: Commit**

```bash
git add src/data/
git commit -m "feat: 添加基础枚举（天干地支宫位四化亮度等）与常量表"
```

---

## Task 3: 星耀数据表 — 星耀键、亮度表、天干四化表、地支属性表

**Files:**
- Create: `src/data/stars.rs`
- Create: `src/data/heavenly_stems.rs`
- Create: `src/data/earthly_branches.rs`
- Modify: `src/data/mod.rs`

**对应 TS 源码:** `iztro/src/data/stars.ts`, `iztro/src/data/heavenlyStems.ts`, `iztro/src/data/earthlyBranches.ts`

**Step 1: 定义星耀枚举（所有 164 个星耀键）**

```rust
// src/data/stars.rs
use serde::{Serialize, Deserialize};
use crate::data::types::Brightness;

/// 星耀键 — 对应 TS 的 StarKey
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StarKey {
    // 14 主星
    ZiweiMaj, TianjiMaj, TaiyangMaj, WuquMaj, TiantongMaj, LianzhenMaj,
    TianfuMaj, TaiyinMaj, TanlangMaj, JumenMaj, TianxiangMaj, TianliangMaj,
    QishaMaj, PojunMaj,
    // 14 辅星
    ZuofuMin, YoubiMin, WenchangMin, WenquMin, LucunMin, TianmaMin,
    QingyangMin, TuoluoMin, HuoxingMin, LingxingMin, TiankuiMin, TianyueMin,
    DikongMin, DijieMin,
    // 杂耀 + 花星 + 解神等（按 TS i18n 的 star key 补全）
    JieshaAdj, Tiankong, Tianxing, Tianyao, Jieshen, Yinsha,
    Tianxi, Tianguan, Tianfu, Tiande, Yuede, Tiancai, Tianshou,
    Tianchu, Longchi, Fengge, Tianku, Tianxu, Hongluan,
    Santai, Bazuo, Enguang, Tiangui, Taifu, Fenggao,
    Posu, Feilian, Huagai, Xianchi, Guchen, Guasu,
    Jiekong, Jieshen2, Xunkong, Kongwang, Jielu,
    Tianshang, Tianshi, Tianyue2, Tianwu, Longde, Dahao,
    Nianjie,
    // 长生12神
    Changsheng, Muyu, Guandai, Linguan, Diwang,
    Shuai, Bing, Si, Mu, Jue, Tai, Yang,
    // 博士12神
    Boshi, Lishi, Qinglong, Xiaohao, Jiangjun, Zoushu,
    FeilianBoshi, Xishen, Bingfu, DahaoBoshi, Fubing, Guanfu2,
    // 流年岁前12神
    Suijian, Huiqi, Sangmen, Guansuo, Guanfu, XiaohaoDec,
    DahaoDec, Longde2, Baihu, Tiande2, Diaoke, BingfuDec,
    // 中州派变体
    Suipo,
    // 将前12神
    Jiangxing, Panan, Suiyi, Xishen2, HuagaiDec, JieshaDec,
    Zaisha, Tiansha, Zhibei, XianchiDec, Yuesha, Wangshen,
    // 运限流耀
    YunchangAdj, YunquAdj, YunluiAdj, YunkuiAdj, YunyueAdj,
    YunluAdj, YunyangAdj, YuntuoAdj, YunmaAdj, YunhuoAdj, YunlingAdj,
    YunkongAdj, YunjieAdj, YunxiAdj, YunluanAdj,
    LiunianchangAdj, LiunianquAdj, LiuniankuiAdj, LiunianyueAdj,
    LiunianluAdj, LiunianyangAdj, LiuniantuoAdj, LiunianmaAdj,
    LiunianhuoAdj, LiunianlingAdj, LiuniankongAdj, LiunianjieAdj,
    LiunianxiAdj, LiunianluanAdj,
    LiuyuechangAdj, LiuyuequAdj, LiuyuekuiAdj, LiuyueyueAdj,
    LiuyueluAdj, LiuyueyangAdj, LiuyuetuoAdj, LiuyuemaAdj,
    LiuyuehuoAdj, LiuyuelingAdj, LiuyuekongAdj, LiuyuejieAdj,
    LiuyuexiAdj, LiuyueluanAdj,
    LiurichangAdj, LiuriquAdj, LiurikuiAdj, LiuriyueAdj,
    LiuriluAdj, LiuriyangAdj, LiurituoAdj, LiurimaAdj,
    LiurihuoAdj, LiurilingAdj, LiurikongAdj, LiurijieAdj,
    LiurixiAdj, LiuriluanAdj,
    LiushichangAdj, LiushiquAdj, LiushikuiAdj, LiushiyueAdj,
    LiushiluAdj, LiushiyangAdj, LiushituoAdj, LiushimaAdj,
    LiushihuoAdj, LiushilingAdj, LiushikongAdj, LiushijieAdj,
    LiushixiAdj, LiushiluanAdj,
}

/// 主星 + 部分辅星的亮度表（12 宫位，从寅宫开始）
/// 对应 TS: STARS_INFO
/// 索引 0=寅, 1=卯, ..., 11=丑
pub fn get_brightness_table(key: StarKey) -> Option<[Option<Brightness>; 12]> {
    use Brightness::*;
    match key {
        StarKey::ZiweiMaj => Some([Some(Wang), Some(Wang), Some(De), Some(Wang), Some(Miao), Some(Miao), Some(Wang), Some(Wang), Some(De), Some(Wang), Some(Ping), Some(Miao)]),
        StarKey::TianjiMaj => Some([Some(De), Some(Wang), Some(Li), Some(Ping), Some(Miao), Some(Xian), Some(De), Some(Wang), Some(Li), Some(Ping), Some(Miao), Some(Xian)]),
        StarKey::TaiyangMaj => Some([Some(Wang), Some(Miao), Some(Miao), Some(Wang), Some(De), Some(Ping), Some(Li), Some(Xian), Some(Xian), Some(Xian), Some(Xian), Some(Bu)]),
        StarKey::WuquMaj => Some([Some(Li), Some(Miao), Some(Ping), Some(Wang), Some(Miao), Some(De), Some(Li), Some(Miao), Some(Ping), Some(Wang), Some(Miao), Some(De)]),
        StarKey::TiantongMaj => Some([Some(Ping), Some(Xian), Some(De), Some(Miao), Some(De), Some(Wang), Some(Ping), Some(Xian), Some(De), Some(Miao), Some(De), Some(Wang)]),
        StarKey::LianzhenMaj => Some([Some(Ping), Some(Miao), Some(Wang), Some(Xian), Some(Li), Some(De), Some(Ping), Some(Miao), Some(Wang), Some(Xian), Some(Li), Some(De)]),
        StarKey::TianfuMaj => Some([Some(Miao), Some(De), Some(Miao), Some(De), Some(Wang), Some(De), Some(Miao), Some(De), Some(Miao), Some(De), Some(Wang), Some(De)]),
        StarKey::TaiyinMaj => Some([Some(Xian), Some(Xian), Some(Xian), Some(Xian), Some(Bu), Some(Li), Some(De), Some(Ping), Some(Wang), Some(Miao), Some(Miao), Some(Wang)]),
        StarKey::TanlangMaj => Some([Some(Ping), Some(Wang), Some(Miao), Some(Wang), Some(Xian), Some(Miao), Some(Ping), Some(Wang), Some(Miao), Some(Wang), Some(Xian), Some(Miao)]),
        StarKey::JumenMaj => Some([Some(Miao), Some(Wang), Some(Wang), Some(Miao), Some(Xian), Some(Bu), Some(Miao), Some(Wang), Some(Wang), Some(Miao), Some(Xian), Some(Bu)]),
        StarKey::TianxiangMaj => Some([Some(Miao), Some(Xian), Some(De), Some(Wang), Some(Wang), Some(De), Some(Miao), Some(Xian), Some(De), Some(Wang), Some(Wang), Some(De)]),
        StarKey::TianliangMaj => Some([Some(Miao), Some(Miao), Some(De), Some(Wang), Some(Xian), Some(Wang), Some(Miao), Some(Miao), Some(De), Some(Wang), Some(Xian), Some(Wang)]),
        StarKey::QishaMaj => Some([Some(Miao), Some(Wang), Some(Ping), Some(Wang), Some(Miao), Some(De), Some(Miao), Some(Wang), Some(Ping), Some(Wang), Some(Miao), Some(De)]),
        StarKey::PojunMaj => Some([Some(De), Some(Miao), Some(Miao), Some(Xian), Some(Wang), Some(Ping), Some(De), Some(Miao), Some(Miao), Some(Xian), Some(Wang), Some(Ping)]),
        StarKey::QingyangMin => Some([None, Some(Xian), Some(Miao), None, Some(Xian), Some(Miao), None, Some(Xian), Some(Miao), None, Some(Xian), Some(Miao)]),
        StarKey::TuoluoMin => Some([Some(Xian), None, None, Some(Miao), None, None, Some(Xian), None, None, Some(Miao), None, None]),
        StarKey::HuoxingMin => Some([Some(Miao), Some(Li), Some(Wang), Some(Miao), Some(Li), Some(Wang), Some(Miao), Some(Li), Some(Wang), Some(Miao), Some(Li), Some(Wang)]),
        StarKey::LingxingMin => Some([Some(Wang), Some(Xian), Some(Miao), Some(Xian), Some(Wang), Some(Miao), Some(Wang), Some(Xian), Some(Miao), Some(Xian), Some(Wang), Some(Miao)]),
        StarKey::ZuofuMin => Some([None; 12]),
        StarKey::YoubiMin => Some([None; 12]),
        StarKey::WenchangMin => Some([Some(Xian), Some(De), None, None, Some(Xian), Some(De), None, None, Some(Xian), Some(De), None, None]),
        StarKey::WenquMin => Some([None, None, Some(De), Some(Xian), None, None, Some(De), Some(Xian), None, None, Some(De), Some(Xian)]),
        StarKey::TiankuiMin => Some([None; 12]),
        StarKey::TianyueMin => Some([None; 12]),
        StarKey::LucunMin => Some([None; 12]),
        StarKey::TianmaMin => Some([None; 12]),
        StarKey::DikongMin => Some([None; 12]),
        StarKey::DijieMin => Some([None; 12]),
        _ => None,
    }
}
```

**Step 2: 天干属性表**

```rust
// src/data/heavenly_stems.rs
use crate::data::types::*;
use crate::data::stars::StarKey;

pub struct HeavenlyStemInfo {
    pub yin_yang: YinYang,
    pub five_elements: FiveElements,
    pub crash: Option<HeavenlyStem>,
    pub mutagen: [StarKey; 4], // [禄, 权, 科, 忌]
}

/// 十天干属性表
pub fn get_heavenly_stem_info(stem: HeavenlyStem) -> HeavenlyStemInfo {
    use HeavenlyStem::*;
    use YinYang::*;
    use FiveElements::*;
    use StarKey::*;
    match stem {
        Jia  => HeavenlyStemInfo { yin_yang: Yang, five_elements: Wood,  crash: Some(Geng), mutagen: [LianzhenMaj, PojunMaj, WuquMaj, TaiyangMaj] },
        Yi   => HeavenlyStemInfo { yin_yang: Yin,  five_elements: Wood,  crash: Some(Xin),  mutagen: [TianjiMaj, TianliangMaj, ZiweiMaj, TaiyinMaj] },
        Bing => HeavenlyStemInfo { yin_yang: Yang, five_elements: Fire,  crash: Some(Ren),  mutagen: [TiantongMaj, TianjiMaj, WenchangMin, LianzhenMaj] },
        Ding => HeavenlyStemInfo { yin_yang: Yin,  five_elements: Fire,  crash: Some(Gui),  mutagen: [TaiyinMaj, TiantongMaj, TianjiMaj, JumenMaj] },
        Wu   => HeavenlyStemInfo { yin_yang: Yang, five_elements: Earth, crash: None,        mutagen: [TanlangMaj, TaiyinMaj, YoubiMin, TianjiMaj] },
        Ji   => HeavenlyStemInfo { yin_yang: Yin,  five_elements: Earth, crash: None,        mutagen: [WuquMaj, TanlangMaj, TianliangMaj, WenquMin] },
        Geng => HeavenlyStemInfo { yin_yang: Yang, five_elements: Metal, crash: Some(Jia),   mutagen: [TaiyangMaj, WuquMaj, TaiyinMaj, TiantongMaj] },
        Xin  => HeavenlyStemInfo { yin_yang: Yin,  five_elements: Metal, crash: Some(Yi),    mutagen: [JumenMaj, TaiyangMaj, WenquMin, WenchangMin] },
        Ren  => HeavenlyStemInfo { yin_yang: Yang, five_elements: Water, crash: Some(Bing),  mutagen: [TianliangMaj, ZiweiMaj, ZuofuMin, WuquMaj] },
        Gui  => HeavenlyStemInfo { yin_yang: Yin,  five_elements: Water, crash: Some(Ding),  mutagen: [PojunMaj, JumenMaj, TaiyinMaj, TanlangMaj] },
    }
}
```

**Step 3: 地支属性表**

```rust
// src/data/earthly_branches.rs
use crate::data::types::*;
use crate::data::stars::StarKey;

pub struct EarthlyBranchInfo {
    pub yin_yang: YinYang,
    pub five_elements: FiveElements,
    pub crash: EarthlyBranch,
    pub soul: StarKey,   // 命主星
    pub body: StarKey,   // 身主星
}

/// 十二地支属性表
pub fn get_earthly_branch_info(branch: EarthlyBranch) -> EarthlyBranchInfo {
    use EarthlyBranch::*;
    use YinYang::*;
    use FiveElements::*;
    use StarKey::*;
    match branch {
        Zi   => EarthlyBranchInfo { yin_yang: Yang, five_elements: Water, crash: Wu,   soul: TanlangMaj,   body: HuoxingMin },
        Chou => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Earth, crash: Wei,  soul: JumenMaj,     body: TianxiangMaj },
        Yin  => EarthlyBranchInfo { yin_yang: Yang, five_elements: Wood,  crash: Shen, soul: LucunMin,     body: TianliangMaj },
        Mao  => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Wood,  crash: You,  soul: WenquMin,     body: TiantongMaj },
        Chen => EarthlyBranchInfo { yin_yang: Yang, five_elements: Earth, crash: Xu,   soul: LianzhenMaj,  body: WenchangMin },
        Si   => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Fire,  crash: Hai,  soul: WuquMaj,      body: TianjiMaj },
        Wu   => EarthlyBranchInfo { yin_yang: Yang, five_elements: Fire,  crash: Zi,   soul: PojunMaj,     body: TiantongMaj },
        Wei  => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Earth, crash: Chou, soul: WuquMaj,      body: TaiyinMaj },
        Shen => EarthlyBranchInfo { yin_yang: Yang, five_elements: Metal, crash: Yin,  soul: LianzhenMaj,  body: WenchangMin },
        You  => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Metal, crash: Mao,  soul: WenquMin,     body: TiantongMaj },
        Xu   => EarthlyBranchInfo { yin_yang: Yang, five_elements: Earth, crash: Chen, soul: LucunMin,     body: TianliangMaj },
        Hai  => EarthlyBranchInfo { yin_yang: Yin,  five_elements: Water, crash: Si,   soul: JumenMaj,     body: TianjiMaj },
    }
}
```

**Step 4: 更新 mod.rs**

```rust
// src/data/mod.rs
pub mod constants;
pub mod types;
pub mod stars;
pub mod heavenly_stems;
pub mod earthly_branches;
```

**Step 5: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 6: Commit**

```bash
git add src/data/
git commit -m "feat: 添加星耀亮度表、天干四化表、地支属性表"
```

---

## Task 4: 枚举的索引转换方法

**Files:**
- Modify: `src/data/types.rs`

**说明:** 天干地支的大量运算依赖索引（0~11），需要为枚举实现 index() / from_index() 方法。

**Step 1: 为核心枚举实现索引方法**

为 `HeavenlyStem`、`EarthlyBranch`、`Palace` 等实现：
- `fn index(&self) -> usize` — 枚举转索引
- `fn from_index(i: usize) -> Self` — 索引转枚举（取模循环）

```rust
impl HeavenlyStem {
    pub fn index(&self) -> usize { *self as usize }
    pub fn from_index(i: usize) -> Self {
        HEAVENLY_STEMS[i % 10]
    }
}

impl EarthlyBranch {
    pub fn index(&self) -> usize { *self as usize }
    pub fn from_index(i: usize) -> Self {
        EARTHLY_BRANCHES[i % 12]
    }
}

impl Palace {
    pub fn index(&self) -> usize { *self as usize }
    pub fn from_index(i: usize) -> Self {
        PALACES[i % 12]
    }
}

impl FiveElementsClass {
    pub fn value(&self) -> usize { *self as usize }
}
```

**Step 2: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 3: Commit**

```bash
git add src/data/types.rs
git commit -m "feat: 为枚举添加索引转换方法"
```

---

## Task 5: 工具函数 — fix_index 及相关

**Files:**
- Modify: `src/utils.rs`

**对应 TS 源码:** `iztro/src/utils/index.ts`

**Step 1: 实现工具函数**

```rust
// src/utils.rs

/// 将索引约束在 0..max 范围内（循环取模）
/// 对应 TS: fixIndex(index, max=12)
pub fn fix_index(index: i32, max: i32) -> usize {
    let result = index % max;
    if result < 0 { (result + max) as usize } else { result as usize }
}

/// 地支索引转宫位索引（寅宫为 0）
/// 对应 TS: fixEarthlyBranchIndex / earthlyBranchIndexToPalaceIndex
pub fn earthly_branch_to_palace_index(branch: EarthlyBranch) -> usize {
    fix_index(branch.index() as i32 - EarthlyBranch::Yin.index() as i32, 12)
}

/// 时辰转时辰索引
/// 对应 TS: timeToIndex(hour)
pub fn time_to_index(hour: u8) -> u8 {
    match hour {
        0 => 0,
        23 => 12,
        h => (h + 1) / 2,
    }
}

/// 获取小限起始宫位索引
/// 对应 TS: getAgeIndex(earthlyBranchName)
pub fn get_age_index(branch: EarthlyBranch) -> usize {
    use EarthlyBranch::*;
    let start = match branch {
        Yin | Wu | Xu   => Chen,
        Shen | Zi | Chen => Xu,
        Si | You | Chou  => Wei,
        Hai | Mao | Wei  => Chou,
    };
    earthly_branch_to_palace_index(start)
}
```

**Step 2: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 3: Commit**

```bash
git add src/utils.rs
git commit -m "feat: 添加 fix_index、地支宫位转换、时辰索引等工具函数"
```

---

## Task 6: 模型定义 — Star, Palace, Astrolabe, Horoscope 结构体

**Files:**
- Create: `src/models/star.rs`
- Create: `src/models/palace.rs`
- Create: `src/models/astrolabe.rs`
- Create: `src/models/horoscope.rs`
- Modify: `src/models/mod.rs`

**对应 TS 源码:** `iztro/src/data/types/star.ts`, `palace.ts`, `astro.ts`

**Step 1: 定义结构体**

```rust
// src/models/star.rs
use serde::{Serialize, Deserialize};
use crate::data::types::*;
use crate::data::stars::StarKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    pub key: StarKey,
    pub name: String,         // i18n 翻译后的名称
    pub star_type: StarType,
    pub scope: Scope,
    pub brightness: Option<Brightness>,
    pub mutagen: Option<Mutagen>,
}
```

```rust
// src/models/palace.rs
use serde::{Serialize, Deserialize};
use crate::data::types::*;
use crate::models::star::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decadal {
    pub range: (u32, u32),
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PalaceData {
    pub index: usize,
    pub name: Palace,
    pub is_body_palace: bool,
    pub is_original_palace: bool,
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
    pub major_stars: Vec<Star>,
    pub minor_stars: Vec<Star>,
    pub adjective_stars: Vec<Star>,
    pub changsheng12: StarKey,
    pub boshi12: StarKey,
    pub jiangqian12: StarKey,
    pub suiqian12: StarKey,
    pub decadal: Decadal,
    pub ages: Vec<u32>,
}
```

```rust
// src/models/astrolabe.rs
use serde::{Serialize, Deserialize};
use crate::data::types::*;
use crate::data::stars::StarKey;
use crate::models::palace::PalaceData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Astrolabe {
    pub gender: Gender,
    pub solar_date: String,
    pub lunar_date: String,
    pub chinese_date: String,
    pub time: String,
    pub time_range: String,
    pub sign: String,
    pub zodiac: String,
    pub earthly_branch_of_soul_palace: EarthlyBranch,
    pub earthly_branch_of_body_palace: EarthlyBranch,
    pub soul: StarKey,
    pub body: StarKey,
    pub five_elements_class: FiveElementsClass,
    pub palaces: Vec<PalaceData>,
}
```

```rust
// src/models/horoscope.rs
use serde::{Serialize, Deserialize};
use crate::data::types::*;
use crate::data::stars::StarKey;
use crate::models::star::Star;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeItem {
    pub index: usize,
    pub name: String,
    pub heavenly_stem: HeavenlyStem,
    pub earthly_branch: EarthlyBranch,
    pub palace_names: Vec<Palace>,
    pub mutagen: Vec<StarKey>,
    pub stars: Option<Vec<Vec<Star>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgeItem {
    #[serde(flatten)]
    pub base: HoroscopeItem,
    pub nominal_age: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HoroscopeData {
    pub solar_date: String,
    pub lunar_date: String,
    pub decadal: HoroscopeItem,
    pub age: AgeItem,
    pub yearly: HoroscopeItem,
    pub monthly: HoroscopeItem,
    pub daily: HoroscopeItem,
    pub hourly: HoroscopeItem,
}
```

**Step 2: 更新 mod.rs**

```rust
// src/models/mod.rs
pub mod star;
pub mod palace;
pub mod astrolabe;
pub mod horoscope;
```

**Step 3: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 4: Commit**

```bash
git add src/models/
git commit -m "feat: 添加 Star/Palace/Astrolabe/Horoscope 结构体定义"
```

---

## Task 7: i18n 多语言系统

**Files:**
- Create: `src/i18n/zh_cn.rs`
- Create: `src/i18n/zh_tw.rs`
- Create: `src/i18n/en_us.rs`
- Create: `src/i18n/ja_jp.rs`
- Create: `src/i18n/ko_kr.rs`
- Create: `src/i18n/vi_vn.rs`
- Modify: `src/i18n/mod.rs`

**对应 TS 源码:** `iztro/src/i18n/locales/`

**Step 1: 实现翻译 trait 和函数**

```rust
// src/i18n/mod.rs
pub mod zh_cn;
pub mod zh_tw;
pub mod en_us;
pub mod ja_jp;
pub mod ko_kr;
pub mod vi_vn;

use crate::data::types::*;
use crate::data::stars::StarKey;

/// 根据语言获取星耀翻译名
pub fn translate_star(key: StarKey, lang: Language) -> &'static str {
    match lang {
        Language::ZhCN => zh_cn::star_name(key),
        Language::ZhTW => zh_tw::star_name(key),
        Language::EnUS => en_us::star_name(key),
        Language::JaJP => ja_jp::star_name(key),
        Language::KoKR => ko_kr::star_name(key),
        Language::ViVN => vi_vn::star_name(key),
    }
}

/// 根据语言获取天干翻译
pub fn translate_heavenly_stem(stem: HeavenlyStem, lang: Language) -> &'static str { ... }

/// 根据语言获取地支翻译
pub fn translate_earthly_branch(branch: EarthlyBranch, lang: Language) -> &'static str { ... }

/// 根据语言获取宫位翻译
pub fn translate_palace(palace: Palace, lang: Language) -> &'static str { ... }

/// 根据语言获取亮度翻译
pub fn translate_brightness(b: Brightness, lang: Language) -> &'static str { ... }

/// 根据语言获取四化翻译
pub fn translate_mutagen(m: Mutagen, lang: Language) -> &'static str { ... }
```

每个语言文件（如 `zh_cn.rs`）实现对应的 `star_name()`、`palace_name()` 等函数，返回 `&'static str`。

**Step 2: 实现 zh_cn.rs 作为参考模板**

```rust
// src/i18n/zh_cn.rs
use crate::data::stars::StarKey;

pub fn star_name(key: StarKey) -> &'static str {
    match key {
        StarKey::ZiweiMaj => "紫微",
        StarKey::TianjiMaj => "天机",
        StarKey::TaiyangMaj => "太阳",
        // ... 完整的 164 个映射
    }
}
```

**Step 3: 其余语言照搬结构，从 TS i18n locale 文件翻译**

**Step 4: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 5: Commit**

```bash
git add src/i18n/
git commit -m "feat: 添加 6 种语言的 i18n 翻译系统"
```

---

## Task 8: 星耀位置计算 — location.rs（核心算法）

**Files:**
- Create: `src/star/location.rs`
- Modify: `src/star/mod.rs`

**对应 TS 源码:** `iztro/src/star/location.ts`（908 行，20+ 函数）

**Step 1: 实现紫微天府起始索引**

```rust
// src/star/location.rs

/// 计算紫微星和天府星的宫位索引
/// 对应 TS: getStartIndex()
pub fn get_start_index(lunar_day: u32, five_elements_class: FiveElementsClass) -> (usize, usize) {
    let fec_value = five_elements_class.value() as u32;
    let mut remainder: i32 = -1;
    let mut quotient: u32;
    let mut offset: i32 = -1;

    while remainder != 0 {
        offset += 1;
        let divisor = lunar_day + offset as u32;
        quotient = divisor / fec_value;
        remainder = (divisor % fec_value) as i32;
    }

    let q = (quotient % 12) as i32;
    let mut ziwei_index = q - 1;
    if offset % 2 == 0 {
        ziwei_index += offset;
    } else {
        ziwei_index -= offset;
    }
    let ziwei = fix_index(ziwei_index, 12);
    let tianfu = fix_index(12 - ziwei_index, 12);
    (ziwei, tianfu)
}
```

**Step 2: 依次实现所有位置计算函数**

按照 TS `location.ts` 逐个翻译：
- `get_lu_yang_tuo_ma_index()` — 禄存擎羊陀罗天马
- `get_kui_yue_index()` — 天魁天钺
- `get_zuo_you_index()` — 左辅右弼
- `get_chang_qu_index()` — 文昌文曲
- `get_kong_jie_index()` — 地空地劫
- `get_huo_ling_index()` — 火星铃星
- `get_luan_xi_index()` — 红鸾天喜
- `get_huagai_xianchi_index()` — 华盖咸池
- `get_gu_gua_index()` — 孤辰寡宿
- `get_jiesha_adj_index()` — 劫杀
- `get_dahao_index()` — 大耗
- `get_yearly_star_index()` — 23+ 流年星
- `get_monthly_star_index()` — 6 流月星
- `get_daily_star_index()` — 三台八座恩光天贵
- `get_timely_star_index()` — 台辅封诰
- `get_chang_qu_index_by_stem()` — 流昌流曲

每个函数都是：输入索引/天干/地支 → 查表/取模 → 返回宫位索引。

**Step 3: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 4: Commit**

```bash
git add src/star/
git commit -m "feat: 实现全部星耀位置计算函数（location.rs）"
```

---

## Task 9: 安星算法 — 主星、辅星、杂耀、流耀

**Files:**
- Create: `src/star/major.rs`
- Create: `src/star/minor.rs`
- Create: `src/star/adjective.rs`
- Create: `src/star/decorative.rs`
- Modify: `src/star/mod.rs`

**对应 TS 源码:** `majorStar.ts`, `minorStar.ts`, `adjectiveStar.ts`, `decorativeStar.ts`

**Step 1: 实现主星安放**

```rust
// src/star/major.rs
/// 安放 14 主星到 12 宫位
/// 对应 TS: getMajorStar()
pub fn get_major_stars(param: &AstroParam) -> [Vec<Star>; 12] {
    // 1. 获取紫微天府索引
    // 2. 紫微系：紫微、天机、_、太阳、武曲、天同、_、_、廉贞（逆时针）
    // 3. 天府系：天府、太阴、贪狼、巨门、天相、天梁、七杀、_、_、_、破军（顺时针）
    // 4. 设置亮度和四化
}
```

**Step 2: 实现辅星安放（14颗）**

从 location 函数获取索引，创建 Star 对象放入宫位。

**Step 3: 实现杂耀安放（38+颗）**

**Step 4: 实现长生12神、博士12神、岁前12神、将前12神**

**Step 5: 验证编译**

Run: `cargo build`
Expected: BUILD SUCCESS

**Step 6: Commit**

```bash
git add src/star/
git commit -m "feat: 实现主星、辅星、杂耀、流耀安放算法"
```

---

## Task 10: 宫位计算 — 命宫身宫、五行局、大限小限

**Files:**
- Create: `src/astro/palace.rs`
- Modify: `src/astro/mod.rs`

**对应 TS 源码:** `iztro/src/astro/palace.ts`

**Step 1: 实现命宫身宫计算**

```rust
/// 计算命宫和身宫索引
/// 对应 TS: getSoulAndBody()
pub fn get_soul_and_body(
    lunar_month_index: usize,
    time_index: u8,
    yearly_stem: HeavenlyStem,
) -> SoulAndBody {
    // soul_index = fix_index(month_index - time_earthly_branch_index)
    // body_index = fix_index(month_index + time_earthly_branch_index)
    // 通过五虎遁获取命宫天干
    // 命宫地支 = EARTHLY_BRANCHES[fix_index(soul_index + 2)]
}
```

**Step 2: 实现五行局计算**

```rust
/// 计算五行局
/// 对应 TS: getFiveElementsClass()
pub fn get_five_elements_class(stem: HeavenlyStem, branch: EarthlyBranch) -> FiveElementsClass {
    // stem_num = stem.index() / 2 + 1
    // branch_num = fix_index(branch.index(), 6) / 2 + 1
    // sum = stem_num + branch_num, while sum > 5: sum -= 5
    // 映射: 1→Wood3rd, 2→Metal4th, 3→Water2nd, 4→Fire6th, 5→Earth5th
}
```

**Step 3: 实现大限和小限计算**

```rust
/// 计算 12 宫的大限和小限
/// 对应 TS: getHoroscope() in palace.ts
pub fn get_decadals_and_ages(...) -> ([Decadal; 12], [Vec<u32>; 12]) { ... }
```

**Step 4: 验证编译 + Commit**

```bash
git add src/astro/
git commit -m "feat: 实现命宫身宫、五行局、大限小限计算"
```

---

## Task 11: 排盘主流程 — by_solar / by_lunar

**Files:**
- Create: `src/astro/builder.rs`
- Modify: `src/astro/mod.rs`

**对应 TS 源码:** `iztro/src/astro/astro.ts`

**Step 1: 实现主排盘函数**

```rust
/// 通过阳历生成星盘
pub fn by_solar(
    solar_date: &str,    // YYYY-M-D
    time_index: u8,      // 0~12
    gender: Gender,
    fix_leap: bool,
    language: Language,
) -> Astrolabe {
    // 1. 阳历转农历（lunar_rust）
    // 2. 获取年月日时干支
    // 3. 计算命宫身宫（get_soul_and_body）
    // 4. 计算五行局（get_five_elements_class）
    // 5. 安主星（get_major_stars）
    // 6. 安辅星（get_minor_stars）
    // 7. 安杂耀（get_adjective_stars）
    // 8. 安长生/博士/流年12神
    // 9. 计算大限小限
    // 10. 组装 12 个 PalaceData
    // 11. 组装 Astrolabe 返回
}

/// 通过农历生成星盘
pub fn by_lunar(...) -> Astrolabe { ... }
```

**Step 2: 对接 lunar_rust 库**

```rust
use lunar_rust::Solar;
use lunar_rust::Lunar;

fn solar_to_lunar(date_str: &str) -> LunarInfo {
    let parts: Vec<&str> = date_str.split('-').collect();
    let solar = Solar::from_ymd(y, m, d);
    let lunar = solar.get_lunar();
    // 提取农历年月日、天干地支等
}
```

**Step 3: 验证编译 + Commit**

```bash
git add src/astro/
git commit -m "feat: 实现排盘主流程 by_solar / by_lunar"
```

---

## Task 12: 查询方法 — has、fliesTo、surroundedPalaces 等

**Files:**
- Create: `src/astro/query.rs`
- Create: `src/astro/surpalaces.rs`
- Modify: `src/astro/mod.rs`

**对应 TS 源码:** `FunctionalPalace.ts`, `FunctionalSurpalaces.ts`, `analyzer.ts`

**Step 1: 为 PalaceData 实现查询方法**

```rust
impl PalaceData {
    /// 判断宫位是否包含所有指定星耀
    pub fn has(&self, stars: &[StarKey]) -> bool { ... }

    /// 判断宫位是否不包含所有指定星耀
    pub fn not_have(&self, stars: &[StarKey]) -> bool { ... }

    /// 判断宫位是否包含任一指定星耀
    pub fn has_one_of(&self, stars: &[StarKey]) -> bool { ... }

    /// 判断是否空宫
    pub fn is_empty(&self) -> bool { ... }

    /// 判断是否有指定四化
    pub fn has_mutagen(&self, mutagen: Mutagen) -> bool { ... }

    /// 飞化到目标宫位
    pub fn flies_to(&self, astrolabe: &Astrolabe, target: usize, mutagen: Mutagen) -> bool { ... }

    /// 自化判断
    pub fn self_mutaged(&self, mutagen: Mutagen) -> bool { ... }
}
```

**Step 2: 实现三方四正**

```rust
// src/astro/surpalaces.rs
pub struct SurroundedPalaces<'a> {
    pub target: &'a PalaceData,
    pub opposite: &'a PalaceData,   // +6
    pub wealth: &'a PalaceData,     // +8 (财帛位)
    pub career: &'a PalaceData,     // +4 (官禄位)
}

impl<'a> SurroundedPalaces<'a> {
    pub fn have(&self, stars: &[StarKey]) -> bool { ... }
    pub fn not_have(&self, stars: &[StarKey]) -> bool { ... }
    pub fn have_one_of(&self, stars: &[StarKey]) -> bool { ... }
    pub fn have_mutagen(&self, mutagen: Mutagen) -> bool { ... }
}
```

**Step 3: 为 Astrolabe 实现便捷方法**

```rust
impl Astrolabe {
    pub fn palace(&self, index: usize) -> &PalaceData { ... }
    pub fn palace_by_name(&self, name: Palace) -> &PalaceData { ... }
    pub fn surrounded_palaces(&self, index: usize) -> SurroundedPalaces { ... }
    pub fn star(&self, key: StarKey) -> Option<&Star> { ... }
}
```

**Step 4: 验证编译 + Commit**

```bash
git add src/astro/
git commit -m "feat: 实现宫位查询方法（has/fliesTo/三方四正等）"
```

---

## Task 13: 运限计算 — horoscope

**Files:**
- Create: `src/astro/horoscope.rs`
- Modify: `src/astro/mod.rs`

**对应 TS 源码:** `FunctionalHoroscope.ts`, `_getHoroscopeBySolarDate()`

**Step 1: 实现运限计算**

```rust
/// 计算指定日期的运限
pub fn get_horoscope(
    astrolabe: &Astrolabe,
    target_date: &str,    // YYYY-M-D
    time_index: Option<u8>,
) -> HoroscopeData {
    // 1. 计算虚岁
    // 2. 找大限宫位（匹配 decadal.range）
    // 3. 找小限宫位（匹配 ages）
    // 4. 计算流年/流月/流日/流时索引
    // 5. 为每个层级计算天干地支、四化、流耀
    // 6. 组装 HoroscopeData 返回
}
```

**Step 2: 实现运限查询方法**

```rust
impl HoroscopeData {
    pub fn palace(&self, name: Palace, scope: Scope) -> usize { ... }
    pub fn has_horoscope_mutagen(&self, name: Palace, scope: Scope, mutagen: Mutagen) -> bool { ... }
    pub fn has_horoscope_stars(&self, name: Palace, scope: Scope, stars: &[StarKey]) -> bool { ... }
}
```

**Step 3: 验证编译 + Commit**

```bash
git add src/astro/
git commit -m "feat: 实现运限计算与查询方法"
```

---

## Task 14: 公开 API 与 JSON 序列化

**Files:**
- Modify: `src/lib.rs`

**Step 1: 设计公开 API**

```rust
// src/lib.rs
pub use astro::builder::{by_solar, by_lunar};
pub use astro::horoscope::get_horoscope;
pub use models::astrolabe::Astrolabe;
pub use models::horoscope::HoroscopeData;
pub use data::types::*;

/// 便捷函数：排盘并返回 JSON
pub fn by_solar_json(
    solar_date: &str,
    time_index: u8,
    gender: Gender,
    fix_leap: bool,
    language: Language,
) -> String {
    let astrolabe = by_solar(solar_date, time_index, gender, fix_leap, language);
    serde_json::to_string(&astrolabe).unwrap()
}
```

**Step 2: 验证编译 + Commit**

```bash
git add src/lib.rs
git commit -m "feat: 整理公开 API 与 JSON 序列化接口"
```

---

## Task 15: AI 提示词模板

**Files:**
- Modify: `src/prompt.rs`

**Step 1: 实现提示词生成**

```rust
// src/prompt.rs
impl Astrolabe {
    /// 生成 AI 分析用的提示词文本
    pub fn to_ai_prompt(&self, lang: Language) -> String {
        // 格式化输出：
        // - 基本信息（性别、日期、时辰、命主、身主、五行局）
        // - 12 宫位信息（宫名、主星、辅星、亮度、四化）
        // - 大限信息
    }
}
```

**Step 2: 验证编译 + Commit**

```bash
git add src/prompt.rs
git commit -m "feat: 添加 AI 提示词模板生成"
```

---

## Task 16: 回归测试框架

**Files:**
- Create: `tests/regression.rs`
- Copy: `tests/expected_results.json`（从 py-iztro 的 JS 生成）

**Step 1: 搭建回归测试框架**

```rust
// tests/regression.rs
use rs_iztro::*;
use pretty_assertions::assert_eq;
use serde_json::Value;

fn load_expected() -> Value {
    let data = include_str!("expected_results.json");
    serde_json::from_str(data).unwrap()
}

#[test]
fn test_astrolabe_basic() {
    let expected = load_expected();
    let params = &expected["params"];
    let astrolabe = by_solar(
        params["solar_date"].as_str().unwrap(),
        params["time_index"].as_u64().unwrap() as u8,
        Gender::Female,
        true,
        Language::ZhCN,
    );

    let exp = &expected["astrolabe"];
    assert_eq!(astrolabe.gender, Gender::Female);
    assert_eq!(astrolabe.palaces.len(), exp["palaces_count"].as_u64().unwrap() as usize);
    // ... 逐字段对比
}

#[test]
fn test_palace_queries() { ... }

#[test]
fn test_horoscope() { ... }
```

**Step 2: 验证测试通过**

Run: `cargo test`
Expected: ALL PASS

**Step 3: Commit**

```bash
git add tests/
git commit -m "feat: 添加基于 JS 生成数据的回归测试框架"
```

---

## Task 17: 扩充测试用例生成器

**Files:**
- Modify: `py-iztro/tests/js/generate_test_data.js`

**说明:** 此任务在 py-iztro 项目中进行，扩充 JS 测试数据生成器，生成更大范围的测试用例供 rs-iztro 回归测试使用。

**Step 1: 扩充测试参数矩阵**

```javascript
const TEST_MATRIX = [];
// 覆盖 60 年甲子
const years = [1940, 1950, 1960, 1970, 1980, 1990, 2000, 2010, 2020];
const months = [1, 3, 6, 9, 12];
const days = [1, 8, 15, 22, 29];
const timeIndices = [0, 2, 5, 8, 11, 12]; // 含早子晚子
const genders = ["男", "女"];

for (const y of years) {
    for (const m of months) {
        for (const d of days) {
            for (const t of timeIndices) {
                for (const g of genders) {
                    TEST_MATRIX.push({
                        solar_date: `${y}-${m}-${d}`,
                        time_index: t,
                        gender: g,
                    });
                }
            }
        }
    }
}
```

**Step 2: 为每组参数生成完整排盘 JSON**

**Step 3: 输出到 `tests/data/matrix_results.json`**

**Step 4: Commit**

```bash
git add tests/js/generate_test_data.js
git commit -m "feat: 扩充测试用例矩阵，覆盖多年份/时辰/性别组合"
```

---

## Task 18（可选）: PyO3 Python 绑定

**Files:**
- Create: `python/` 目录
- Modify: `Cargo.toml` 添加 pyo3 feature

**说明:** 此任务在核心算法完全通过回归测试后进行。

**Step 1: 在 Cargo.toml 添加 PyO3 依赖**

```toml
[lib]
name = "rs_iztro"
crate-type = ["cdylib", "rlib"]

[dependencies.pyo3]
version = "0.23"
features = ["extension-module"]
optional = true

[features]
python = ["pyo3"]
```

**Step 2: 创建 Python 绑定模块**

```rust
#[cfg(feature = "python")]
mod python_bindings {
    use pyo3::prelude::*;

    #[pyfunction]
    fn by_solar(solar_date: &str, time_index: u8, gender: &str, ...) -> String {
        // 调用核心函数，返回 JSON
    }

    #[pymodule]
    fn rs_iztro(m: &Bound<'_, PyModule>) -> PyResult<()> {
        m.add_function(wrap_pyfunction!(by_solar, m)?)?;
        Ok(())
    }
}
```

**Step 3: 使用 maturin 构建**

```bash
pip install maturin
maturin develop --features python
```

**Step 4: 验证 Python 调用**

```python
import rs_iztro
result = rs_iztro.by_solar("2000-8-16", 2, "female")
print(result)
```

**Step 5: Commit**

```bash
git add python/ Cargo.toml src/
git commit -m "feat: 添加 PyO3 Python 绑定"
```

---

## 任务依赖关系

```
Task 1 (骨架)
  → Task 2 (枚举常量)
    → Task 3 (数据表)
      → Task 4 (索引方法)
        → Task 5 (工具函数)
          → Task 6 (模型定义)
          → Task 7 (i18n) [可与 Task 6 并行]
            → Task 8 (location 算法)
              → Task 9 (安星算法)
                → Task 10 (宫位计算)
                  → Task 11 (排盘主流程)
                    → Task 12 (查询方法)
                    → Task 13 (运限计算)
                      → Task 14 (公开 API)
                        → Task 15 (AI 提示词)
                        → Task 16 (回归测试)
                          → Task 17 (扩充测试)
                            → Task 18 (PyO3 绑定)
```

## 预估工作量

| 任务组 | 任务 | 预估对话轮数 |
|--------|------|------------|
| 基础层 | Task 1-5 | 1 轮 |
| 模型+i18n | Task 6-7 | 1 轮 |
| 核心算法 | Task 8-9 | 1-2 轮 |
| 排盘+查询 | Task 10-13 | 1-2 轮 |
| 集成+测试 | Task 14-17 | 1 轮 |
| Python 绑定 | Task 18 | 1 轮 |
| **总计** | | **6-8 轮对话** |

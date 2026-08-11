# 60 甲子全覆盖黄金测试 Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** 建立三层测试金字塔，用 JS iztro 生成黄金标准数据，覆盖 60 甲子完整周期（阳历 1984-2043），对 rs-iztro 排盘结果进行全面回归验证。

**Architecture:** 三层结构 — Tier 1（780 组全字段对比）→ Tier 2（按年分文件紧凑字段对比，~18K 组）→ Tier 3（全量 SHA-256 hash 校验，~285K 组）。JS 脚本生成数据，Rust 测试消费数据。Tier 3 用 `#[ignore]` 标记，仅 release 前手动运行。

**Tech Stack:** Node.js + iztro（数据生成），Rust + serde_json + sha2（测试消费），pretty_assertions（diff 输出）

---

## Task 1: JS 生成脚手架 — package.json 与依赖

**Files:**
- Create: `tests/golden/package.json`
- Create: `tests/golden/.gitignore`

**Step 1: 创建 package.json**

```json
{
  "name": "rs-iztro-golden-generator",
  "version": "1.0.0",
  "private": true,
  "type": "module",
  "scripts": {
    "gen:tier1": "node generate_tier1.mjs",
    "gen:tier2": "node generate_tier2.mjs",
    "gen:tier3": "node generate_tier3.mjs",
    "gen:all": "npm run gen:tier1 && npm run gen:tier2 && npm run gen:tier3"
  },
  "dependencies": {
    "iztro": "^2"
  }
}
```

**Step 2: 创建 .gitignore**

```
node_modules/
```

**Step 3: 安装依赖**

Run: `cd /Users/haose/code/rs-iztro/tests/golden && npm install`
Expected: iztro 安装成功

**Step 4: Commit**

```bash
git add tests/golden/package.json tests/golden/package-lock.json tests/golden/.gitignore
git commit -m "chore: add JS golden test data generator scaffold"
```

---

## Task 2: Tier 1 JS 生成脚本 — 60 干支 × 13 时辰全字段

**Files:**
- Create: `tests/golden/generate_tier1.mjs`

**覆盖范围:** 60 个干支年 × 13 个时辰 = 780 组，固定月=1，日=15，性别=男，算法=默认

**存储的字段（全字段）:**
- 星盘元信息：命宫地支、身宫地支、五行局、命主星、身主星
- 12 宫完整数据：宫名、天干、地支、主星列表（含亮度和四化）、辅星列表、杂耀列表
- 长生12、博士12、将前12、岁前12
- 大限范围、大限天干地支
- 小限数组

**Step 1: 编写生成脚本**

```javascript
// tests/golden/generate_tier1.mjs
import { astro } from 'iztro';
import fs from 'fs';

// 60 甲子对应的阳历日期（取每年正月十五附近的阳历日期）
// 我们用 1984-2043 这 60 年覆盖完整甲子周期
// 固定取每年 2 月 15 日（大约在正月中旬）
const START_YEAR = 1984;
const END_YEAR = 2043;
const FIXED_MONTH = 2;
const FIXED_DAY = 15;

const TIME_INDICES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];

function extractPalace(palace) {
  return {
    name: palace.name,
    heavenly_stem: palace.heavenlyStem,
    earthly_branch: palace.earthlyBranch,
    is_body_palace: palace.isBodyPalace,
    is_original_palace: palace.isOriginalPalace,
    major_stars: palace.majorStars.map(s => ({
      name: s.name,
      type: s.type,
      brightness: s.brightness || null,
      mutagen: s.mutagen || null,
    })),
    minor_stars: palace.minorStars.map(s => ({
      name: s.name,
      type: s.type,
      brightness: s.brightness || null,
      mutagen: s.mutagen || null,
    })),
    adjective_stars: palace.adpiritStars.map(s => ({
      name: s.name,
      type: s.type,
    })),
    changsheng12: palace.changsheng12,
    boshi12: palace.boshi12,
    jiangqian12: palace.jiangqian12,
    suiqian12: palace.suiqian12,
    decadal_range: [palace.decadal.range.start, palace.decadal.range.end],
    decadal_heavenly_stem: palace.decadal.heavenlyStem,
    decadal_earthly_branch: palace.decadal.earthlyBranch,
    ages: palace.ages,
  };
}

function generateCase(solarDate, timeIndex) {
  try {
    const result = astro.bySolar(solarDate, timeIndex, '男', true, 'zh-CN');
    return {
      params: {
        solar_date: solarDate,
        time_index: timeIndex,
        gender: '男',
      },
      soul_palace_branch: result.earthlyBranchOfSoulPalace,
      body_palace_branch: result.earthlyBranchOfBodyPalace,
      five_elements_class: result.fiveElementsClass,
      soul_star: result.soul,
      body_star: result.body,
      palaces: result.palaces.map(extractPalace),
    };
  } catch (e) {
    console.error(`Error for ${solarDate} t=${timeIndex}: ${e.message}`);
    return null;
  }
}

console.log('Generating Tier 1 golden data...');
const cases = [];

for (let year = START_YEAR; year <= END_YEAR; year++) {
  const solarDate = `${year}-${FIXED_MONTH}-${FIXED_DAY}`;
  for (const t of TIME_INDICES) {
    const c = generateCase(solarDate, t);
    if (c) cases.push(c);
  }
  process.stdout.write(`\r  Year ${year}/${END_YEAR} (${cases.length} cases)`);
}

console.log(`\nGenerated ${cases.length} Tier 1 cases.`);
fs.writeFileSync('tier1_data.json', JSON.stringify(cases, null, 0));
console.log('Written to tier1_data.json');
```

注意：上面脚本中的 iztro API 调用方式需要在实际运行时根据 iztro 实际版本的 API 进行调整。关键是：
1. 先 `npm install` 后检查 `node_modules/iztro` 的实际导出
2. 查看 iztro 的 `astro.bySolar` 或 `astro.astrolabeBySolarDate` 等方法签名
3. 查看返回值中宫位和星耀的实际属性名

**Step 2: 运行并检查 iztro API**

先写一个小测试脚本确认 API：
```bash
cd /Users/haose/code/rs-iztro/tests/golden
node -e "import('iztro').then(m => console.log(Object.keys(m)))"
```

根据实际 API 调整 `generate_tier1.mjs` 中的调用方式。

**Step 3: 运行生成**

Run: `cd /Users/haose/code/rs-iztro/tests/golden && node generate_tier1.mjs`
Expected: 生成 `tier1_data.json`，包含 780 条记录

**Step 4: Commit**

```bash
git add tests/golden/generate_tier1.mjs tests/golden/tier1_data.json
git commit -m "feat: add Tier 1 golden data generator (60 stems × 13 times)"
```

---

## Task 3: Tier 2 JS 生成脚本 — 按年分文件、紧凑格式

**Files:**
- Create: `tests/golden/generate_tier2.mjs`
- Create: `tests/golden/tier2/` (60 个 JSON 文件)

**覆盖范围:** 60 年 × 12 月 × {1日, 15日} × 13 时辰 × 2 性别 = 18,720 组（每年 312 组）

**紧凑格式设计：**
```json
{
  "d": "1984-2-1",    // solar_date
  "t": 0,             // time_index
  "g": 0,             // gender: 0=男 1=女
  "sb": 4,            // soul palace branch index (0=子..11=亥)
  "bb": 8,            // body palace branch index
  "fc": 3,            // five_elements_class value (2/3/4/5/6)
  "ss": "贪狼",       // soul star
  "bs": "天同",       // body star
  "pn": [0,1,2,...],  // 12 palace name indices (Palace enum order)
  "ms": [[0,1],[2],[],[],...],  // 12 palaces × major star name list
  "ns": [[5,6],[7],[],[],...],  // 12 palaces × minor star name list
  "dr": [[2,11],[12,21],...],   // 12 decadal ranges
}
```

**Step 1: 编写生成脚本**

```javascript
// tests/golden/generate_tier2.mjs
// 生成紧凑格式数据，按年分文件存储到 tier2/ 目录
import { astro } from 'iztro';
import fs from 'fs';

const START_YEAR = 1984;
const END_YEAR = 2043;
const DAYS = [1, 15];
const TIME_INDICES = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const GENDERS = ['男', '女'];

const BRANCH_MAP = { '子':0,'丑':1,'寅':2,'卯':3,'辰':4,'巳':5,'午':6,'未':7,'申':8,'酉':9,'戌':10,'亥':11 };
const PALACE_MAP = { '命宫':0,'父母':1,'福德':2,'田宅':3,'官禄':4,'交友':5,'仆役':5,'迁移':6,'疾厄':7,'财帛':8,'子女':9,'夫妻':10,'兄弟':11 };

function compact(solarDate, timeIndex, genderStr, result) {
  return {
    d: solarDate,
    t: timeIndex,
    g: genderStr === '男' ? 0 : 1,
    sb: BRANCH_MAP[result.earthlyBranchOfSoulPalace],
    bb: BRANCH_MAP[result.earthlyBranchOfBodyPalace],
    fc: result.fiveElementsClass,  // 需要根据实际返回值调整
    ss: result.soul,
    bs: result.body,
    pn: result.palaces.map(p => PALACE_MAP[p.name]),
    ms: result.palaces.map(p => p.majorStars.map(s => s.name)),
    ns: result.palaces.map(p => p.minorStars.map(s => s.name)),
    dr: result.palaces.map(p => [p.decadal.range.start, p.decadal.range.end]),
  };
}

fs.mkdirSync('tier2', { recursive: true });

for (let year = START_YEAR; year <= END_YEAR; year++) {
  const cases = [];
  for (let month = 1; month <= 12; month++) {
    for (const day of DAYS) {
      const solarDate = `${year}-${month}-${day}`;
      for (const t of TIME_INDICES) {
        for (const g of GENDERS) {
          try {
            const result = astro.bySolar(solarDate, t, g, true, 'zh-CN');
            cases.push(compact(solarDate, t, g, result));
          } catch (e) {
            // Skip invalid dates
          }
        }
      }
    }
  }
  const filename = `tier2/year_${year}.json`;
  fs.writeFileSync(filename, JSON.stringify(cases));
  console.log(`${filename}: ${cases.length} cases`);
}
```

**Step 2: 运行生成**

Run: `cd /Users/haose/code/rs-iztro/tests/golden && node generate_tier2.mjs`
Expected: tier2/ 目录下生成 60 个文件，每个 ~50-100 KB

**Step 3: Commit**

```bash
git add tests/golden/generate_tier2.mjs tests/golden/tier2/
git commit -m "feat: add Tier 2 golden data (60 years × compact format)"
```

---

## Task 4: Tier 3 JS 生成脚本 — 全量 hash

**Files:**
- Create: `tests/golden/generate_tier3.mjs`

**覆盖范围:** 60 年 × 365 天 × 13 时辰 × 1 性别 = ~284,700 组
（仅男性，因为性别只影响大限方向和长生方向，Tier 2 已覆盖性别差异）

**存储格式：** 每行一个 `solar_date,time_index,sha256_hex`，纯文本 CSV

**Step 1: 编写生成脚本**

```javascript
// tests/golden/generate_tier3.mjs
import { astro } from 'iztro';
import crypto from 'crypto';
import fs from 'fs';

const START_YEAR = 1984;
const END_YEAR = 2043;

function daysInMonth(year, month) {
  return new Date(year, month, 0).getDate();
}

function hashAstrolabe(result) {
  // 将关键排盘结果序列化后取 hash
  const data = JSON.stringify({
    sb: result.earthlyBranchOfSoulPalace,
    bb: result.earthlyBranchOfBodyPalace,
    fc: result.fiveElementsClass,
    ss: result.soul,
    bs: result.body,
    palaces: result.palaces.map(p => ({
      n: p.name,
      hs: p.heavenlyStem,
      eb: p.earthlyBranch,
      ms: p.majorStars.map(s => s.name),
      ns: p.minorStars.map(s => s.name),
      cs: p.changsheng12,
      bo: p.boshi12,
      jq: p.jiangqian12,
      sq: p.suiqian12,
      dr: [p.decadal.range.start, p.decadal.range.end],
    })),
  });
  return crypto.createHash('sha256').update(data).digest('hex');
}

const out = fs.createWriteStream('tier3_hashes.csv');
out.write('solar_date,time_index,hash\n');

let count = 0;
for (let year = START_YEAR; year <= END_YEAR; year++) {
  for (let month = 1; month <= 12; month++) {
    const days = daysInMonth(year, month);
    for (let day = 1; day <= days; day++) {
      const solarDate = `${year}-${month}-${day}`;
      for (let t = 0; t <= 12; t++) {
        try {
          const result = astro.bySolar(solarDate, t, '男', true, 'zh-CN');
          const hash = hashAstrolabe(result);
          out.write(`${solarDate},${t},${hash}\n`);
          count++;
        } catch (e) {
          // skip
        }
      }
    }
  }
  process.stdout.write(`\r  Year ${year}/${END_YEAR} (${count} hashes)`);
}

out.end();
console.log(`\nGenerated ${count} hashes → tier3_hashes.csv`);
```

**Step 2: 运行生成**

Run: `cd /Users/haose/code/rs-iztro/tests/golden && node generate_tier3.mjs`
Expected: 生成 `tier3_hashes.csv`，约 285K 行，~25 MB
注意：此脚本可能需要数分钟运行。

**Step 3: Commit**

```bash
git add tests/golden/generate_tier3.mjs tests/golden/tier3_hashes.csv
git commit -m "feat: add Tier 3 hash data (full 60-year coverage)"
```

---

## Task 5: Rust Cargo.toml 添加 sha2 依赖

**Files:**
- Modify: `Cargo.toml`

**Step 1: 添加 sha2 到 dev-dependencies**

在 `[dev-dependencies]` 中添加：
```toml
[dev-dependencies]
pretty_assertions = "1"
sha2 = "0.10"
hex = "0.4"
```

**Step 2: 验证编译**

Run: `cd /Users/haose/code/rs-iztro && cargo check`
Expected: 编译成功

**Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore: add sha2 and hex dev-dependencies for golden tests"
```

---

## Task 6: Tier 1 Rust 测试 — 全字段对比

**Files:**
- Create: `tests/golden_tier1.rs`

**测试逻辑：**
1. 加载 `tests/golden/tier1_data.json`
2. 对每条记录调用 `by_solar()` 生成星盘
3. 对比所有字段：命宫地支、身宫地支、五行局、命主星、身主星
4. 对比 12 宫的宫名、天干、地支、主星（名称+亮度+四化）、辅星、杂耀
5. 对比长生12、博士12、将前12、岁前12
6. 对比大限范围

**Step 1: 编写测试文件**

```rust
// tests/golden_tier1.rs
//! Tier 1 golden tests: 60 干支年 × 13 时辰 = 780 cases, full field comparison.

use rs_iztro::data::types::*;
use rs_iztro::i18n::{
    translate_brightness, translate_earthly_branch, translate_five_elements_class,
    translate_mutagen, translate_palace, translate_star,
};
use rs_iztro::by_solar;
use serde_json::Value;

static TIER1_DATA: &str = include_str!("golden/tier1_data.json");

fn parse_gender(s: &str) -> Gender {
    match s {
        "男" => Gender::Male,
        "女" => Gender::Female,
        _ => panic!("Unknown gender: {}", s),
    }
}

#[test]
fn test_tier1_golden() {
    let cases: Vec<Value> = serde_json::from_str(TIER1_DATA)
        .expect("Failed to parse tier1_data.json");

    let lang = Language::ZhCN;
    let mut failures: Vec<String> = Vec::new();

    for (i, case) in cases.iter().enumerate() {
        let params = &case["params"];
        let solar_date = params["solar_date"].as_str().unwrap();
        let time_index = params["time_index"].as_u64().unwrap() as u8;
        let gender = parse_gender(params["gender"].as_str().unwrap());

        let astrolabe = by_solar(solar_date, time_index, gender, true, lang, Algorithm::Default);
        let ctx = format!("case[{}] {}@t{}", i, solar_date, time_index);

        // === 元信息对比 ===
        let check = |field: &str, actual: &str, expected: &str| {
            if actual != expected {
                format!("{}: {} expected '{}', got '{}'", ctx, field, expected, actual)
            } else {
                String::new()
            }
        };

        let r = check(
            "soul_branch",
            &translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang),
            case["soul_palace_branch"].as_str().unwrap(),
        );
        if !r.is_empty() { failures.push(r); }

        let r = check(
            "body_branch",
            &translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang),
            case["body_palace_branch"].as_str().unwrap(),
        );
        if !r.is_empty() { failures.push(r); }

        let r = check(
            "five_elements_class",
            &translate_five_elements_class(astrolabe.five_elements_class, lang),
            case["five_elements_class"].as_str().unwrap(),
        );
        if !r.is_empty() { failures.push(r); }

        let r = check(
            "soul_star",
            &translate_star(astrolabe.soul, lang),
            case["soul_star"].as_str().unwrap(),
        );
        if !r.is_empty() { failures.push(r); }

        let r = check(
            "body_star",
            &translate_star(astrolabe.body, lang),
            case["body_star"].as_str().unwrap(),
        );
        if !r.is_empty() { failures.push(r); }

        // === 12 宫对比 ===
        let expected_palaces = case["palaces"].as_array().unwrap();
        for (pi, exp_p) in expected_palaces.iter().enumerate() {
            let palace = &astrolabe.palaces[pi];
            let pctx = format!("{}:palace[{}]", ctx, pi);

            // 宫名
            let exp_name = exp_p["name"].as_str().unwrap();
            let act_name = translate_palace(palace.name, lang);
            if act_name != exp_name {
                failures.push(format!("{}: name expected '{}', got '{}'", pctx, exp_name, act_name));
            }

            // 天干
            let exp_stem = exp_p["heavenly_stem"].as_str().unwrap();
            let act_stem = translate_heavenly_stem(palace.heavenly_stem, lang);
            if act_stem != exp_stem {
                failures.push(format!("{}: stem expected '{}', got '{}'", pctx, exp_stem, act_stem));
            }

            // 地支
            let exp_branch = exp_p["earthly_branch"].as_str().unwrap();
            let act_branch = translate_earthly_branch(palace.earthly_branch, lang);
            if act_branch != exp_branch {
                failures.push(format!("{}: branch expected '{}', got '{}'", pctx, exp_branch, act_branch));
            }

            // 主星列表
            let exp_majors = exp_p["major_stars"].as_array().unwrap();
            if palace.major_stars.len() != exp_majors.len() {
                failures.push(format!(
                    "{}: major_stars count expected {}, got {}",
                    pctx, exp_majors.len(), palace.major_stars.len()
                ));
            } else {
                for (si, exp_s) in exp_majors.iter().enumerate() {
                    let star = &palace.major_stars[si];
                    let exp_sname = exp_s["name"].as_str().unwrap();
                    let act_sname = translate_star(star.key, lang);
                    if act_sname != exp_sname {
                        failures.push(format!(
                            "{}:major[{}] name expected '{}', got '{}'",
                            pctx, si, exp_sname, act_sname
                        ));
                    }
                    // 亮度
                    if let Some(exp_b) = exp_s["brightness"].as_str() {
                        if let Some(b) = star.brightness {
                            let act_b = translate_brightness(b, lang);
                            if act_b != exp_b {
                                failures.push(format!(
                                    "{}:major[{}] brightness expected '{}', got '{}'",
                                    pctx, si, exp_b, act_b
                                ));
                            }
                        } else {
                            failures.push(format!(
                                "{}:major[{}] brightness expected '{}', got None",
                                pctx, si, exp_b
                            ));
                        }
                    }
                    // 四化
                    let exp_m = exp_s["mutagen"].as_str();
                    match (exp_m, star.mutagen) {
                        (Some(em), Some(am)) => {
                            let act_m = translate_mutagen(am, lang);
                            if act_m != em {
                                failures.push(format!(
                                    "{}:major[{}] mutagen expected '{}', got '{}'",
                                    pctx, si, em, act_m
                                ));
                            }
                        }
                        (Some(em), None) => {
                            failures.push(format!(
                                "{}:major[{}] mutagen expected '{}', got None",
                                pctx, si, em
                            ));
                        }
                        (None, Some(am)) => {
                            let act_m = translate_mutagen(am, lang);
                            failures.push(format!(
                                "{}:major[{}] mutagen expected None, got '{}'",
                                pctx, si, act_m
                            ));
                        }
                        (None, None) => {}
                    }
                }
            }

            // 辅星列表（同样逻辑，检查名称和四化）
            let exp_minors = exp_p["minor_stars"].as_array().unwrap();
            if palace.minor_stars.len() != exp_minors.len() {
                failures.push(format!(
                    "{}: minor_stars count expected {}, got {}",
                    pctx, exp_minors.len(), palace.minor_stars.len()
                ));
            } else {
                for (si, exp_s) in exp_minors.iter().enumerate() {
                    let star = &palace.minor_stars[si];
                    let exp_sname = exp_s["name"].as_str().unwrap();
                    let act_sname = translate_star(star.key, lang);
                    if act_sname != exp_sname {
                        failures.push(format!(
                            "{}:minor[{}] name expected '{}', got '{}'",
                            pctx, si, exp_sname, act_sname
                        ));
                    }
                }
            }

            // 长生12 / 博士12 / 将前12 / 岁前12
            let decoratives = [
                ("changsheng12", palace.changsheng12),
                ("boshi12", palace.boshi12),
                ("jiangqian12", palace.jiangqian12),
                ("suiqian12", palace.suiqian12),
            ];
            for (dname, dkey) in &decoratives {
                if let Some(exp_d) = exp_p[*dname].as_str() {
                    let act_d = translate_star(*dkey, lang);
                    if act_d != exp_d {
                        failures.push(format!(
                            "{}:{} expected '{}', got '{}'",
                            pctx, dname, exp_d, act_d
                        ));
                    }
                }
            }

            // 大限范围
            if let Some(exp_dr) = exp_p["decadal_range"].as_array() {
                let exp_start = exp_dr[0].as_u64().unwrap() as u32;
                let exp_end = exp_dr[1].as_u64().unwrap() as u32;
                if palace.decadal.range != (exp_start, exp_end) {
                    failures.push(format!(
                        "{}: decadal_range expected ({},{}), got ({},{})",
                        pctx, exp_start, exp_end,
                        palace.decadal.range.0, palace.decadal.range.1
                    ));
                }
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "\n{} Tier 1 failures out of {} cases:\n{}",
            failures.len(),
            cases.len(),
            failures[..failures.len().min(50)].join("\n")
        );
    }

    println!("All {} Tier 1 golden cases passed.", cases.len());
}
```

注意：需要导入 `translate_heavenly_stem`，该函数已存在于 `rs_iztro::i18n`。

**Step 2: 验证编译**

Run: `cargo test --test golden_tier1 --no-run`
Expected: 编译成功（运行需要先有数据文件）

**Step 3: 运行测试**

Run: `cargo test --test golden_tier1 -- --nocapture`
Expected: 780 cases 全部通过

**Step 4: Commit**

```bash
git add tests/golden_tier1.rs
git commit -m "feat: add Tier 1 Rust golden test (780 cases, full field)"
```

---

## Task 7: Tier 2 Rust 测试 — 按年文件紧凑对比

**Files:**
- Create: `tests/golden_tier2.rs`

**测试逻辑：**
1. 遍历 `tests/golden/tier2/year_*.json` 文件
2. 对每条紧凑记录调用 `by_solar()` 生成星盘
3. 对比关键字段：命宫地支、身宫地支、五行局、命主星、身主星、宫名、主星分布、辅星分布、大限

**Step 1: 编写测试**

```rust
// tests/golden_tier2.rs
//! Tier 2 golden tests: 60 years × compact format, ~18K cases.
//! Each year is a separate JSON file in tests/golden/tier2/.

use rs_iztro::data::types::*;
use rs_iztro::i18n::{
    translate_earthly_branch, translate_five_elements_class, translate_palace, translate_star,
};
use rs_iztro::by_solar;
use serde_json::Value;
use std::fs;

const TIER2_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/tier2");
const BRANCHES: [EarthlyBranch; 12] = [
    EarthlyBranch::Zi, EarthlyBranch::Chou, EarthlyBranch::Yin, EarthlyBranch::Mao,
    EarthlyBranch::Chen, EarthlyBranch::Si, EarthlyBranch::Wu, EarthlyBranch::Wei,
    EarthlyBranch::Shen, EarthlyBranch::You, EarthlyBranch::Xu, EarthlyBranch::Hai,
];

#[test]
fn test_tier2_golden() {
    let lang = Language::ZhCN;
    let mut total = 0usize;
    let mut failures: Vec<String> = Vec::new();

    let mut entries: Vec<_> = fs::read_dir(TIER2_DIR)
        .expect("Cannot read tier2 directory")
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension().is_some_and(|ext| ext == "json"))
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in &entries {
        let content = fs::read_to_string(entry.path())
            .unwrap_or_else(|e| panic!("Cannot read {:?}: {}", entry.path(), e));
        let cases: Vec<Value> = serde_json::from_str(&content)
            .unwrap_or_else(|e| panic!("Cannot parse {:?}: {}", entry.path(), e));

        for case in &cases {
            total += 1;
            let solar_date = case["d"].as_str().unwrap();
            let time_index = case["t"].as_u64().unwrap() as u8;
            let gender = if case["g"].as_u64().unwrap() == 0 { Gender::Male } else { Gender::Female };

            let astrolabe = by_solar(solar_date, time_index, gender, true, lang, Algorithm::Default);
            let ctx = format!("{}@t{}g{}", solar_date, time_index, case["g"]);

            // 命宫地支
            let exp_sb = case["sb"].as_u64().unwrap() as usize;
            let act_sb = astrolabe.earthly_branch_of_soul_palace;
            if act_sb != BRANCHES[exp_sb] {
                failures.push(format!("{}: soul_branch idx expected {}, got {:?}", ctx, exp_sb, act_sb));
            }

            // 身宫地支
            let exp_bb = case["bb"].as_u64().unwrap() as usize;
            let act_bb = astrolabe.earthly_branch_of_body_palace;
            if act_bb != BRANCHES[exp_bb] {
                failures.push(format!("{}: body_branch idx expected {}, got {:?}", ctx, exp_bb, act_bb));
            }

            // 五行局
            let exp_fc = case["fc"].as_str().unwrap();
            let act_fc = translate_five_elements_class(astrolabe.five_elements_class, lang);
            if act_fc != exp_fc {
                failures.push(format!("{}: fec expected '{}', got '{}'", ctx, exp_fc, act_fc));
            }

            // 命主星 / 身主星
            let exp_ss = case["ss"].as_str().unwrap();
            let act_ss = translate_star(astrolabe.soul, lang);
            if act_ss != exp_ss {
                failures.push(format!("{}: soul_star expected '{}', got '{}'", ctx, exp_ss, act_ss));
            }
            let exp_bs = case["bs"].as_str().unwrap();
            let act_bs = translate_star(astrolabe.body, lang);
            if act_bs != exp_bs {
                failures.push(format!("{}: body_star expected '{}', got '{}'", ctx, exp_bs, act_bs));
            }

            // 12 宫名称
            if let Some(pn) = case["pn"].as_array() {
                for (pi, exp_idx) in pn.iter().enumerate() {
                    let exp_name_idx = exp_idx.as_u64().unwrap() as usize;
                    let act_name = translate_palace(astrolabe.palaces[pi].name, lang);
                    let palace_names = ["命宫","父母","福德","田宅","官禄","交友","迁移","疾厄","财帛","子女","夫妻","兄弟"];
                    if exp_name_idx < palace_names.len() && act_name != palace_names[exp_name_idx] {
                        failures.push(format!(
                            "{}:p[{}] name expected '{}', got '{}'",
                            ctx, pi, palace_names[exp_name_idx], act_name
                        ));
                    }
                }
            }

            // 主星分布
            if let Some(ms) = case["ms"].as_array() {
                for (pi, exp_stars) in ms.iter().enumerate() {
                    let exp_names: Vec<&str> = exp_stars.as_array()
                        .unwrap()
                        .iter()
                        .map(|v| v.as_str().unwrap())
                        .collect();
                    let act_names: Vec<String> = astrolabe.palaces[pi]
                        .major_stars
                        .iter()
                        .map(|s| translate_star(s.key, lang).to_string())
                        .collect();
                    if act_names.len() != exp_names.len() {
                        failures.push(format!(
                            "{}:p[{}] major count expected {}, got {}",
                            ctx, pi, exp_names.len(), act_names.len()
                        ));
                    } else {
                        for (si, exp_n) in exp_names.iter().enumerate() {
                            if act_names[si] != *exp_n {
                                failures.push(format!(
                                    "{}:p[{}]:major[{}] expected '{}', got '{}'",
                                    ctx, pi, si, exp_n, act_names[si]
                                ));
                            }
                        }
                    }
                }
            }

            // 大限范围
            if let Some(dr) = case["dr"].as_array() {
                for (pi, exp_range) in dr.iter().enumerate() {
                    let arr = exp_range.as_array().unwrap();
                    let exp_start = arr[0].as_u64().unwrap() as u32;
                    let exp_end = arr[1].as_u64().unwrap() as u32;
                    let act = astrolabe.palaces[pi].decadal.range;
                    if act != (exp_start, exp_end) {
                        failures.push(format!(
                            "{}:p[{}] decadal expected ({},{}), got ({},{})",
                            ctx, pi, exp_start, exp_end, act.0, act.1
                        ));
                    }
                }
            }
        }
    }

    if !failures.is_empty() {
        panic!(
            "\n{} Tier 2 failures out of {} cases:\n{}",
            failures.len(),
            total,
            failures[..failures.len().min(100)].join("\n")
        );
    }

    println!("All {} Tier 2 golden cases passed.", total);
}
```

**Step 2: 验证编译**

Run: `cargo test --test golden_tier2 --no-run`
Expected: 编译成功

**Step 3: 运行测试**

Run: `cargo test --test golden_tier2 -- --nocapture`
Expected: ~18,720 cases 全部通过

**Step 4: Commit**

```bash
git add tests/golden_tier2.rs
git commit -m "feat: add Tier 2 Rust golden test (~18K cases, compact)"
```

---

## Task 8: Tier 3 Rust 测试 — 全量 hash 校验

**Files:**
- Create: `tests/golden_tier3.rs`

**测试逻辑：**
1. 加载 `tier3_hashes.csv`（~285K 行）
2. 对每行解析 solar_date, time_index
3. 调用 `by_solar()` 生成星盘
4. 用同样的逻辑构造 JSON 字符串，计算 SHA-256
5. 对比 hash
6. 标记 `#[ignore]` — 仅手动运行

**Step 1: 编写测试**

```rust
// tests/golden_tier3.rs
//! Tier 3 golden tests: full 60-year hash verification (~285K cases).
//! Marked #[ignore] — run manually with: cargo test --test golden_tier3 -- --ignored

use rs_iztro::data::types::*;
use rs_iztro::i18n::{
    translate_earthly_branch, translate_five_elements_class, translate_palace, translate_star,
};
use rs_iztro::by_solar;
use sha2::{Sha256, Digest};

static TIER3_DATA: &str = include_str!("golden/tier3_hashes.csv");

fn hash_astrolabe(astrolabe: &rs_iztro::Astrolabe, lang: Language) -> String {
    // 必须与 JS 端 hashAstrolabe 函数的序列化格式完全一致
    let mut parts = Vec::new();
    parts.push(format!(
        r#"{{"sb":"{}","bb":"{}","fc":"{}","ss":"{}","bs":"{}","palaces":["#,
        translate_earthly_branch(astrolabe.earthly_branch_of_soul_palace, lang),
        translate_earthly_branch(astrolabe.earthly_branch_of_body_palace, lang),
        translate_five_elements_class(astrolabe.five_elements_class, lang),
        translate_star(astrolabe.soul, lang),
        translate_star(astrolabe.body, lang),
    ));

    for (i, p) in astrolabe.palaces.iter().enumerate() {
        if i > 0 { parts.push(",".into()); }
        let ms: Vec<String> = p.major_stars.iter()
            .map(|s| format!(r#""{}""#, translate_star(s.key, lang)))
            .collect();
        let ns: Vec<String> = p.minor_stars.iter()
            .map(|s| format!(r#""{}""#, translate_star(s.key, lang)))
            .collect();
        parts.push(format!(
            r#"{{"n":"{}","hs":"{}","eb":"{}","ms":[{}],"ns":[{}],"cs":"{}","bo":"{}","jq":"{}","sq":"{}","dr":[{},{}]}}"#,
            translate_palace(p.name, lang),
            translate_earthly_branch(p.earthly_branch, lang), // 注意：JS 用 heavenlyStem
            translate_earthly_branch(p.earthly_branch, lang),
            ms.join(","),
            ns.join(","),
            translate_star(p.changsheng12, lang),
            translate_star(p.boshi12, lang),
            translate_star(p.jiangqian12, lang),
            translate_star(p.suiqian12, lang),
            p.decadal.range.0,
            p.decadal.range.1,
        ));
    }
    parts.push("]}".into());
    let json = parts.join("");

    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    hex::encode(hasher.finalize())
}

#[test]
#[ignore]
fn test_tier3_golden_hashes() {
    let lang = Language::ZhCN;
    let mut total = 0usize;
    let mut mismatches = 0usize;
    let mut first_mismatches: Vec<String> = Vec::new();

    for line in TIER3_DATA.lines().skip(1) {
        // CSV: solar_date,time_index,hash
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() != 3 { continue; }

        let solar_date = fields[0];
        let time_index: u8 = fields[1].parse().unwrap();
        let expected_hash = fields[2];

        let astrolabe = by_solar(solar_date, time_index, Gender::Male, true, lang, Algorithm::Default);
        let actual_hash = hash_astrolabe(&astrolabe, lang);

        total += 1;
        if actual_hash != expected_hash {
            mismatches += 1;
            if first_mismatches.len() < 20 {
                first_mismatches.push(format!(
                    "{}@t{}: expected {}..., got {}...",
                    solar_date, time_index,
                    &expected_hash[..16], &actual_hash[..16]
                ));
            }
        }

        if total % 10000 == 0 {
            eprintln!("  Verified {}/~285000 hashes ({} mismatches)", total, mismatches);
        }
    }

    if mismatches > 0 {
        panic!(
            "\n{} hash mismatches out of {} cases:\n{}",
            mismatches,
            total,
            first_mismatches.join("\n")
        );
    }

    eprintln!("All {} Tier 3 golden hashes verified.", total);
}
```

**重要注意事项：** Tier 3 的 hash 校验要求 Rust 端和 JS 端的序列化格式**完全一致**（包括字段顺序、引号风格等）。最稳妥的做法是：
1. 先让 Rust 端生成一批 hash
2. JS 端对同样的输入生成 hash
3. 对比两端的序列化输出，确保格式一致
4. 如有不一致，统一格式

如果格式对齐困难，可以改用 **Rust 自身生成基准 hash**（即第一次运行时 Rust 生成 hash 作为基准，后续验证无回归），而不是与 JS 交叉验证。

**Step 2: 验证编译**

Run: `cargo test --test golden_tier3 --no-run`
Expected: 编译成功

**Step 3: 运行测试**

Run: `cargo test --test golden_tier3 -- --ignored --nocapture`
Expected: ~285K hashes 全部通过（预计 3-5 分钟）

**Step 4: Commit**

```bash
git add tests/golden_tier3.rs
git commit -m "feat: add Tier 3 Rust golden test (285K hash verification)"
```

---

## Task 9: Rust 自生成基准模式（Tier 3 替代方案）

**Files:**
- Create: `tests/golden_selfcheck.rs`

**背景：** 如果 JS/Rust 两端 hash 格式难以对齐，提供一个 Rust 自生成基准方案：
1. 第一次运行：Rust 生成所有 hash 并保存到文件
2. 后续运行：Rust 生成 hash 与保存的基准对比

这种方式不验证"与 JS 一致"，但保证"Rust 自身无回归"。

**Step 1: 编写自检测试**

```rust
// tests/golden_selfcheck.rs
//! Self-check golden test: generates baseline hashes on first run,
//! then verifies no regression on subsequent runs.
//! Run: cargo test --test golden_selfcheck -- --ignored --nocapture

use rs_iztro::data::types::*;
use rs_iztro::by_solar;
use sha2::{Sha256, Digest};
use std::fs;
use std::path::Path;

const BASELINE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/selfcheck_baseline.csv");

fn hash_json(astrolabe: &rs_iztro::Astrolabe) -> String {
    let json = serde_json::to_string(astrolabe).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(json.as_bytes());
    hex::encode(hasher.finalize())
}

#[test]
#[ignore]
fn test_selfcheck_golden() {
    let lang = Language::ZhCN;
    let baseline_exists = Path::new(BASELINE_PATH).exists();

    let mut baseline: std::collections::HashMap<String, String> = std::collections::HashMap::new();

    if baseline_exists {
        let content = fs::read_to_string(BASELINE_PATH).unwrap();
        for line in content.lines().skip(1) {
            let fields: Vec<&str> = line.split(',').collect();
            if fields.len() == 3 {
                let key = format!("{},{}", fields[0], fields[1]);
                baseline.insert(key, fields[2].to_string());
            }
        }
        eprintln!("Loaded {} baseline hashes", baseline.len());
    } else {
        eprintln!("No baseline found. Will generate baseline to {}", BASELINE_PATH);
    }

    let mut output = String::from("solar_date,time_index,hash\n");
    let mut total = 0usize;
    let mut mismatches = 0usize;

    for year in 1984..=2043 {
        for month in 1..=12u32 {
            let days_in_month = match month {
                1|3|5|7|8|10|12 => 31,
                4|6|9|11 => 30,
                2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
                _ => unreachable!(),
            };
            for day in 1..=days_in_month {
                let solar_date = format!("{}-{}-{}", year, month, day);
                for t in 0..=12u8 {
                    let astrolabe = by_solar(&solar_date, t, Gender::Male, true, lang, Algorithm::Default);
                    let hash = hash_json(&astrolabe);

                    let key = format!("{},{}", solar_date, t);
                    if baseline_exists {
                        if let Some(expected) = baseline.get(&key) {
                            if *expected != hash {
                                mismatches += 1;
                                if mismatches <= 10 {
                                    eprintln!("MISMATCH: {} expected {}..., got {}...",
                                        key, &expected[..16], &hash[..16]);
                                }
                            }
                        }
                    }

                    output.push_str(&format!("{},{},{}\n", solar_date, t, hash));
                    total += 1;
                }
            }
        }
        eprint!("\r  Year {}/2043 ({} cases)", year, total);
    }

    if !baseline_exists {
        fs::write(BASELINE_PATH, &output).unwrap();
        eprintln!("\nBaseline generated: {} hashes → {}", total, BASELINE_PATH);
    } else if mismatches > 0 {
        panic!("\n{} hash mismatches out of {} cases!", mismatches, total);
    } else {
        eprintln!("\nAll {} hashes match baseline.", total);
    }
}
```

**Step 2: 首次运行生成基准**

Run: `cargo test --test golden_selfcheck -- --ignored --nocapture`
Expected: 生成 `selfcheck_baseline.csv`（~25 MB）

**Step 3: 再次运行验证无回归**

Run: `cargo test --test golden_selfcheck -- --ignored --nocapture`
Expected: 所有 hash 匹配

**Step 4: Commit**

```bash
git add tests/golden_selfcheck.rs
# 注意：selfcheck_baseline.csv 较大，考虑加入 .gitignore 或 Git LFS
git commit -m "feat: add self-check golden test (Rust baseline hash verification)"
```

---

## Task 10: 更新 .gitignore 和文档

**Files:**
- Modify: `.gitignore`（如果存在）或创建

**Step 1: 更新 .gitignore**

确保以下内容：
```
# Golden test data (large files)
tests/golden/node_modules/
tests/golden/selfcheck_baseline.csv
```

tier1_data.json、tier2/ 目录应当 **纳入 git 追踪**（总计 <10 MB）。
tier3_hashes.csv 如果 >20 MB 建议用 Git LFS 或 .gitignore。

**Step 2: Commit**

```bash
git add .gitignore
git commit -m "chore: update .gitignore for golden test data"
```

---

## 执行顺序总结

```
Task 1 → Task 2 → Task 3 → Task 4   (JS 生成端，顺序执行)
Task 5                                 (Rust 依赖，可并行)
Task 6 → Task 7 → Task 8 → Task 9    (Rust 测试端，顺序执行，依赖 Task 1-4 的数据)
Task 10                                (收尾)
```

**关键依赖：**
- Task 6-8 依赖 Task 2-4 生成的数据文件
- Task 5 可与 Task 1-4 并行
- Task 9 是 Task 8 的替代方案，如果 JS/Rust hash 格式对齐困难则使用 Task 9

**预估总存储：**
| 文件 | 大小 |
|------|------|
| tier1_data.json | ~3 MB |
| tier2/*.json (60 files) | ~5 MB |
| tier3_hashes.csv | ~25 MB |
| selfcheck_baseline.csv | ~25 MB |

**预估运行时间：**
| 层级 | JS 生成 | Rust 测试 |
|------|---------|-----------|
| Tier 1 | ~10 秒 | ~2 秒 |
| Tier 2 | ~2 分钟 | ~15 秒 |
| Tier 3 | ~10 分钟 | ~3 分钟 |
| Self-check | N/A | ~3 分钟 |

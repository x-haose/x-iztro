//! 绑定覆盖率：核心分派的每个标识都必须在 Python、Go 与 JS 的绑定源码里出现。
//!
//! 三侧的功能测试各自只跑自己写过的接口，因此「bridge 分派了某个 kind，但
//! Python/Go/JS 没有写类型化包装」这类漏洞在任何一侧都测不出来——对外等于该
//! 能力不存在。星耀标识同理：枚举漏一个，用户就没法用类型化方式引用它。
//!
//! 查询 kind 没有运行期清单，从 `src/bridge.rs` 的源码文本抽取；星耀与格局标识取
//! `StarKey::KEYS` / `PatternKey::KEYS`。再要求每个字符串以带引号的形式出现在绑定层的
//! 非测试源码中。bridge 入参结构体的字段（绑定层发给内核的 JSON 键）同样从源码抽取，
//! 要求各侧都发送：Go 以带引号的 camelCase 键、Python 以 snake_case 关键字、JS 以 camelCase 对象键出现。

use std::fs;
use std::path::Path;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

/// 读取仓库内的文本文件，路径相对仓库根。
fn read(rel: &str) -> String {
    let path = Path::new(ROOT).join(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

/// 取源码中以 `header` 开头、到下一个顶格 `}` 为止的一段（即该函数体）。
fn fn_body(src: &str, header: &str) -> String {
    let start = src
        .find(header)
        .unwrap_or_else(|| panic!("`{header}` not found — 分派结构变了，本测试需同步更新"));
    let rest = &src[start..];
    let end = rest
        .find("\n}\n")
        .unwrap_or_else(|| panic!("`{header}` has no top-level closing brace"));
    rest[..end].to_string()
}

/// 抽出片段里形如 `"someIdentifier"` / `"some_snake_key"` 的标识字面量。
///
/// 只收纯标识符（字母开头、仅含字母数字与下划线——kind 是 camelCase、
/// 格局 key 是 snake_case），从而滤掉错误信息、格式串等含空格或标点的字符串。
fn identifier_literals(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'"' {
            i += 1;
            continue;
        }
        let start = i + 1;
        let mut j = start;
        while j < bytes.len() && bytes[j] != b'"' {
            j += 1;
        }
        if j < bytes.len() {
            let lit = &src[start..j];
            let is_ident = !lit.is_empty()
                && lit.chars().next().unwrap().is_ascii_alphabetic()
                && lit.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
            if is_ident && !out.contains(&lit.to_string()) {
                out.push(lit.to_string());
            }
        }
        i = j + 1;
    }
    out
}

/// 拼接一个目录下指定后缀的源码文本，跳过测试文件。
fn binding_sources(dir: &str, ext: &str) -> String {
    let path = Path::new(ROOT).join(dir);
    let mut out = String::new();
    for entry in fs::read_dir(&path).unwrap_or_else(|e| panic!("read_dir {}: {e}", path.display()))
    {
        let entry = entry.unwrap();
        let name = entry.file_name().to_string_lossy().to_string();
        if !name.ends_with(ext)
            || name.ends_with(&format!("_test{ext}"))
            || name.starts_with("test_")
        {
            continue;
        }
        out.push_str(&fs::read_to_string(entry.path()).unwrap());
        out.push('\n');
    }
    assert!(!out.is_empty(), "no {ext} sources found in {dir}");
    out
}

/// 标识是否以带引号的形式出现（Go 只有双引号，Python 与 JS 两种引号都可能）。
fn mentions(src: &str, ident: &str) -> bool {
    src.contains(&format!("\"{ident}\"")) || src.contains(&format!("'{ident}'"))
}

/// 去掉 `starts_with("...")` 形式的前缀探测，其参数是前缀而非完整 kind。
fn strip_starts_with_probes(src: &str) -> String {
    let mut out = String::new();
    let mut rest = src;
    while let Some(pos) = rest.find("starts_with(") {
        out.push_str(&rest[..pos]);
        rest = &rest[pos..];
        let close = rest.find(')').expect("unterminated starts_with(");
        rest = &rest[close + 1..];
    }
    out.push_str(rest);
    out
}

/// bridge 分派的全部查询 kind：`query` 的顶层 match 加 `is_star_kind` 的白名单。
fn bridge_query_kinds() -> Vec<String> {
    let src = read("src/bridge.rs");
    // `k.starts_with("get")` 里的前缀不是 kind，抽取前先剔除这类前缀探测
    let query_body = strip_starts_with_probes(&fn_body(&src, "pub fn query(input: &QueryInput)"));
    let mut kinds = identifier_literals(&query_body);
    for k in identifier_literals(&fn_body(&src, "fn is_star_kind(kind: &str)")) {
        if !kinds.contains(&k) {
            kinds.push(k);
        }
    }
    assert!(
        kinds.len() > 40,
        "只抽出 {} 个 kind，抽取逻辑与 bridge.rs 结构脱节了",
        kinds.len()
    );
    kinds
}

/// bridge 入参结构体（排盘、运限、统一查询）的全部字段名，snake_case，取自各结构体的源码。
///
/// 字段是绑定层发给内核的 JSON 键：哪一侧漏发了某个键，内核取缺省值，结果看着正常却是错的。
fn bridge_input_fields() -> Vec<String> {
    let src = read("src/bridge.rs");
    let mut fields = Vec::new();
    for header in [
        "pub struct SolarChartInput",
        "pub struct LunarChartInput",
        "pub struct HoroscopeInput",
        "pub struct QueryInput",
    ] {
        for line in fn_body(&src, header).lines() {
            let Some((name, _)) = line
                .trim()
                .strip_prefix("pub ")
                .and_then(|rest| rest.split_once(':'))
            else {
                continue;
            };
            let is_field = !name.is_empty()
                && name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_');
            if is_field && !fields.iter().any(|f| f == name) {
                fields.push(name.to_string());
            }
        }
    }
    assert!(
        fields.len() > 40,
        "只抽出 {} 个入参字段，抽取逻辑与 bridge.rs 结构脱节了",
        fields.len()
    );
    fields
}

/// snake_case 转 camelCase（绑定层 JSON 键的写法）
fn camel(snake: &str) -> String {
    let mut out = String::new();
    let mut upper = false;
    for c in snake.chars() {
        if c == '_' {
            upper = true;
        } else if upper {
            out.push(c.to_ascii_uppercase());
            upper = false;
        } else {
            out.push(c);
        }
    }
    out
}

/// 标识是否作为一个完整的词出现（前后都不是字母、数字或下划线）：
/// Python 以 snake_case 关键字传参，JS 以 camelCase 对象键（含简写属性）传参，都不带引号。
fn mentions_word(src: &str, ident: &str) -> bool {
    let is_word = |c: char| c.is_ascii_alphanumeric() || c == '_';
    src.match_indices(ident).any(|(i, _)| {
        let before = src[..i].chars().next_back();
        let after = src[i + ident.len()..].chars().next();
        !before.is_some_and(is_word) && !after.is_some_and(is_word)
    })
}

/// Go 侧不经内核、就地计算的查询所独有的入参：`fixIndex` 的 `max` 与 `timeToIndex` 的 `hour`
/// （取模与整数除法，`go/iztro/utils.go` 的 `FixIndex` / `TimeToIndex`），Go 不发送它们
const GO_LOCAL_FIELDS: [&str; 2] = ["max", "hour"];

#[test]
fn bridge_input_fields_are_sent_by_bindings() {
    let fields = bridge_input_fields();
    let python = binding_sources("python/x_iztro", ".py");
    let go = binding_sources("go/iztro", ".go");
    let js = binding_sources("js/src", ".ts");

    let mut missing = Vec::new();
    for field in &fields {
        let key = camel(field);
        if !mentions_word(&python, field) {
            missing.push(format!("python: {field}"));
        }
        if !GO_LOCAL_FIELDS.contains(&field.as_str()) && !mentions(&go, &key) {
            missing.push(format!("go: \"{key}\""));
        }
        if !mentions_word(&js, &key) {
            missing.push(format!("js: {key}"));
        }
    }
    assert!(
        missing.is_empty(),
        "\n\n{} 个 bridge 入参字段在绑定层没有发送（内核会静默取缺省值）：\n  {}\n",
        missing.len(),
        missing.join("\n  "),
    );
    eprintln!(
        "binding coverage: all {} bridge input fields sent",
        fields.len()
    );
}

fn star_keys() -> Vec<String> {
    x_iztro::StarKey::KEYS
        .iter()
        .map(|k| k.to_string())
        .collect()
}

fn pattern_keys() -> Vec<String> {
    x_iztro::PatternKey::KEYS
        .iter()
        .map(|k| k.to_string())
        .collect()
}

fn report_missing(label: &str, idents: &[String], src: &str) -> Vec<String> {
    idents
        .iter()
        .filter(|k| !mentions(src, k))
        .map(|k| format!("{label}: {k}"))
        .collect()
}

#[test]
fn bridge_query_kinds_are_wrapped_in_bindings() {
    let kinds = bridge_query_kinds();
    let python = binding_sources("python/x_iztro", ".py");
    let go = binding_sources("go/iztro", ".go");
    let js = binding_sources("js/src", ".ts");

    let mut missing = report_missing("python", &kinds, &python);
    missing.extend(report_missing("go", &kinds, &go));
    missing.extend(report_missing("js", &kinds, &js));

    assert!(
        missing.is_empty(),
        "\n\n{} 个 bridge kind 在绑定层没有对应包装（对外等于不可用）：\n  {}\n",
        missing.len(),
        missing.join("\n  "),
    );
    eprintln!("binding coverage: all {} bridge kinds wrapped", kinds.len());
}

#[test]
fn star_keys_are_exposed_in_bindings() {
    let keys = star_keys();
    let python = binding_sources("python/x_iztro", ".py");
    let go = binding_sources("go/iztro", ".go");
    let js = binding_sources("js/src", ".ts");

    let mut missing = report_missing("python", &keys, &python);
    missing.extend(report_missing("go", &keys, &go));
    missing.extend(report_missing("js", &keys, &js));

    assert!(
        missing.is_empty(),
        "\n\n{} 个星耀标识在绑定层枚举里缺失：\n  {}\n",
        missing.len(),
        missing.join("\n  "),
    );
    eprintln!("binding coverage: all {} star keys exposed", keys.len());
}

#[test]
fn pattern_keys_are_exposed_in_bindings() {
    let keys = pattern_keys();
    let python = binding_sources("python/x_iztro", ".py");
    let go = binding_sources("go/iztro", ".go");
    let js = binding_sources("js/src", ".ts");

    let mut missing = report_missing("python", &keys, &python);
    missing.extend(report_missing("go", &keys, &go));
    missing.extend(report_missing("js", &keys, &js));

    assert!(
        missing.is_empty(),
        "\n\n{} 个格局标识在绑定层枚举里缺失：\n  {}\n",
        missing.len(),
        missing.join("\n  "),
    );
    eprintln!("binding coverage: all {} pattern keys exposed", keys.len());
}

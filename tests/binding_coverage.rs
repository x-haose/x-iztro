//! 绑定覆盖率：核心分派的每个标识都必须在 Python 与 Go 的绑定源码里出现。
//!
//! 三侧的功能测试各自只跑自己写过的接口，因此「bridge 分派了某个 kind，但
//! Python/Go 没有写类型化包装」这类漏洞在任何一侧都测不出来——对外等于该
//! 能力不存在。星耀标识同理：枚举漏一个，用户就没法用类型化方式引用它。
//!
//! 本测试不调用被测代码，而是读源码文本做交叉核对：从 `src/bridge.rs` 抽出
//! 全部查询 kind、从 `src/data/stars.rs` 抽出全部星耀 key，再要求每个字符串
//! 以带引号的形式出现在绑定层的非测试源码中。

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

/// 抽出片段里形如 `"someIdentifier"` 的标识字面量。
///
/// 只收纯标识符（字母开头、仅含字母数字），从而滤掉错误信息、格式串等
/// 含空格或标点的字符串。
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
                && lit.chars().all(|c| c.is_ascii_alphanumeric());
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

/// 标识是否以带引号的形式出现（Go 只有双引号，Python 两种引号都可能）。
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

/// StarKey::as_key 表里的全部星耀标识。
fn star_keys() -> Vec<String> {
    let src = read("src/data/stars.rs");
    let keys = identifier_literals(&fn_body(&src, "    pub fn as_key(&self) -> &'static str {"));
    assert!(
        keys.len() > 150,
        "只抽出 {} 个星耀 key，抽取逻辑与 stars.rs 结构脱节了",
        keys.len()
    );
    keys
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

    let mut missing = report_missing("python", &kinds, &python);
    missing.extend(report_missing("go", &kinds, &go));

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

    let mut missing = report_missing("python", &keys, &python);
    missing.extend(report_missing("go", &keys, &go));

    assert!(
        missing.is_empty(),
        "\n\n{} 个星耀标识在绑定层枚举里缺失：\n  {}\n",
        missing.len(),
        missing.join("\n  "),
    );
    eprintln!("binding coverage: all {} star keys exposed", keys.len());
}

# 开发与检查

## 环境

- Rust：最低版本见 `Cargo.toml` 的 `rust-version`，安装 rustfmt、Clippy 和 `wasm32-wasip1`、`wasm32-unknown-unknown` 两个 target。
- Python：版本范围见 `pyproject.toml`；根目录 `.venv` 中安装 maturin 与 pytest。
- Go：最低版本见 `go/iztro/go.mod`。
- JS 绑定：Node.js 版本见 `js/package.json` 的 `engines`，在 `js/` 执行 `npm ci`。
- 文档站：与 `.github/workflows/ci.yml` 相同的 Node.js 环境，在 `docs/` 执行 `npm ci`。

检查命令不自动安装工具或依赖。首次准备 Python 环境：

```bash
python3 -m venv .venv
.venv/bin/python -m pip install maturin pytest
```

## 检查入口

```bash
make check                 # Rust、Python、Go、JS、文档站，任一步失败即停止
make check-full            # 再跑 Tier 3 全量金标与全日期可排盘测试
```

`make check` 包含：

| 目标 | 内容 |
| --- | --- |
| `check-rust` | 格式、Clippy、schemars feature 编译、零告警 API 文档、release 构建、常规测试（含契约与文本快照）、历法全量金标 |
| `check-python` | 从源码构建并安装原生扩展，再跑 Python 全部测试 |
| `check-go` | 对仓库内嵌 WASM 跑竞态检测测试，重编并刷新 WASM 后再测，构建 Go 示例 |
| `check-js` | 编出 `wasm32-unknown-unknown` 内核并编译 TypeScript，类型检查测试，在 Node 里对真实内核跑全部测试，用 publint 与 attw 检查包的导出与类型声明 |
| `check-docs` | 生成路由类型、类型检查、生产构建 |

各目标可单独执行。Python 环境不在 `.venv` 时用 `make check VENV=/绝对路径`。
Go 检查会刷新 `go/iztro/x_iztro.wasm`；它是随 Go 包分发的构建产物，变动需要审查。
检查不重建金标和文本快照；测试报告的 ignored 项不等于已经验证。

CI 按语言拆分作业，并额外检查 Rust 最低支持版本、Python 与 Node.js 支持版本两端。
本机检查不代替这些环境矩阵；`make check-full` 的两项额外全量测试不属于常规 CI。

## 取材接口

知识包格式以 [knowledge/SCHEMA.md](knowledge/SCHEMA.md) 为准。
`topic_to_text` 按包中主题生成本命文本；`for_astrolabe` / `for_horoscope` 裁剪知识条目，
不返回事实与引文逐条关联的解读结果。新增能力需同时核对 Rust、Python、Go、JS 的参数、结果和失败语义。

内核负责排盘与事实判定，知识包提供论法和解释，服务负责网络、部署与调用编组。
金标证明与锁定版本的上游计算一致，不证明传统解释的预测有效性。

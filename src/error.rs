//! 排盘入口与绑定层的错误类型。

use std::fmt;

/// 排盘与运限入口的参数错误。
///
/// 入口函数对全部外部输入做前置校验并以本类型报错，库内部不因非法参数
/// panic。这对 wasm 目标尤为关键：wasm 下 panic 即 abort（trap），且每次
/// trap 都会永久损耗模块实例的栈空间，累积后连合法调用也会失败。
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IztroError {
    /// 日期字符串非法：格式不是 "YYYY-M-D"、该日期不存在，
    /// 或超出支持范围（公历 1583-9999 年）。
    InvalidDate(String),
    /// 时辰索引超出 0-12（0=早子时，12=晚子时）。
    InvalidTimeIndex(u8),
    /// 入参不合法：干支阴阳不配、反推条件为空或包含流耀、年份范围颠倒等
    /// 无法归入日期与时辰两类的调用方错误。
    InvalidArgument(String),
    /// 依赖的历法库未能给出本应存在的干支或星座取值。
    /// 属于库内部缺陷而非调用方过错，以错误返回而非 panic——
    /// wasm 上 panic 会 trap 并损耗实例栈空间。
    Internal(String),
}

impl IztroError {
    /// 机器可读的错误分类标识，随错误 JSON 的 `code` 字段跨语言传递。
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidDate(_) => "invalid_date",
            Self::InvalidTimeIndex(_) => "invalid_time_index",
            Self::InvalidArgument(_) => "invalid_argument",
            Self::Internal(_) => "internal",
        }
    }
}

impl fmt::Display for IztroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDate(msg) => f.write_str(msg),
            Self::InvalidTimeIndex(t) => write!(f, "time_index must be 0-12, got {t}"),
            Self::InvalidArgument(msg) => f.write_str(msg),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

impl std::error::Error for IztroError {}

/// 绑定层（C FFI / wasm / PyO3）对外报错的统一形状。
///
/// 三条出口都把它落成 `{"error": "<message>", "code": "<code>"}`：`message`
/// 面向人，`code` 面向程序——各语言绑定据此映射到自己的异常类型或哨兵错误，
/// 无需解析文案。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeError {
    /// 机器可读的错误分类：
    /// `invalid_date`（日期非法）、`invalid_time_index`（时辰越界）、
    /// `invalid_argument`（其余入参或配置非法）、`internal`（库内部缺陷）。
    pub code: &'static str,
    /// 面向人的错误描述，小写起首、以冒号引出细节。
    pub message: String,
}

impl BridgeError {
    /// 入参或配置非法（`invalid_argument`）。
    pub fn invalid_argument(message: impl Into<String>) -> Self {
        Self {
            code: "invalid_argument",
            message: message.into(),
        }
    }

    /// 库内部缺陷，含 `catch_unwind` 兜住的 panic（`internal`）。
    pub fn internal(message: impl Into<String>) -> Self {
        Self {
            code: "internal",
            message: message.into(),
        }
    }
}

impl From<IztroError> for BridgeError {
    fn from(e: IztroError) -> Self {
        Self {
            code: e.code(),
            message: e.to_string(),
        }
    }
}

impl fmt::Display for BridgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for BridgeError {}

#[cfg(test)]
mod tests {
    use super::*;

    const ERROR_CODES_SNAPSHOT: &str =
        concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden/error_codes.json");

    /// 按枚举声明顺序收集每个变体的 `code()`。
    ///
    /// 匹配必须穷尽，所以新增变体时这里先无法编译。每个变体还要有代表值：
    /// 漏构造则对应槽位仍是 `None`，本函数 panic。枚举标了 `non_exhaustive`，
    /// 这段匹配只能放在本 crate 里——crate 外必须写兜底臂，新增变体会被兜底吞掉。
    fn engine_codes() -> Vec<&'static str> {
        let samples = [
            IztroError::InvalidDate(String::new()),
            IztroError::InvalidTimeIndex(0),
            IztroError::InvalidArgument(String::new()),
            IztroError::Internal(String::new()),
        ];
        let mut invalid_date = None;
        let mut invalid_time_index = None;
        let mut invalid_argument = None;
        let mut internal = None;
        for error in &samples {
            match error {
                IztroError::InvalidDate(_) => invalid_date = Some(error.code()),
                IztroError::InvalidTimeIndex(_) => invalid_time_index = Some(error.code()),
                IztroError::InvalidArgument(_) => invalid_argument = Some(error.code()),
                IztroError::Internal(_) => internal = Some(error.code()),
            }
        }
        [
            invalid_date.expect("InvalidDate 没有代表值"),
            invalid_time_index.expect("InvalidTimeIndex 没有代表值"),
            invalid_argument.expect("InvalidArgument 没有代表值"),
            internal.expect("Internal 没有代表值"),
        ]
        .to_vec()
    }

    /// `tests/golden/error_codes.json` 的 `engineCodes` 与 `IztroError::code()` 一致。
    ///
    /// 快照是下游钉契约用的稳定列表。改码或新增变体必须同时改快照，否则失败。
    #[test]
    fn engine_error_codes_match_snapshot() {
        let raw = std::fs::read_to_string(ERROR_CODES_SNAPSHOT)
            .unwrap_or_else(|e| panic!("读取 {ERROR_CODES_SNAPSHOT} 失败: {e}"));
        let file: serde_json::Value = serde_json::from_str(&raw)
            .unwrap_or_else(|e| panic!("{ERROR_CODES_SNAPSHOT} 不是合法 JSON: {e}"));
        let obj = file
            .as_object()
            .unwrap_or_else(|| panic!("{ERROR_CODES_SNAPSHOT} 必须是对象"));
        let top: Vec<&str> = obj.keys().map(String::as_str).collect();
        assert_eq!(
            top,
            ["description", "engineCodes"],
            "error_codes.json 顶层只允许 description 与 engineCodes"
        );
        let description = obj["description"].as_str().unwrap_or("");
        assert!(
            !description.trim().is_empty(),
            "error_codes.json 的 description 必须说明这些码来自 IztroError::code()"
        );

        let snapshot: Vec<&str> = obj["engineCodes"]
            .as_array()
            .unwrap_or_else(|| panic!("engineCodes 必须是数组"))
            .iter()
            .map(|v| {
                v.as_str()
                    .unwrap_or_else(|| panic!("engineCodes 的元素必须是字符串"))
            })
            .collect();
        let codes = engine_codes();
        assert_eq!(
            snapshot, codes,
            "IztroError::code() 与 {ERROR_CODES_SNAPSHOT} 不一致"
        );

        let paired = [
            IztroError::InvalidDate("bad date".into()),
            IztroError::InvalidTimeIndex(13),
            IztroError::InvalidArgument("bad argument".into()),
            IztroError::Internal("boom".into()),
        ];
        for error in paired {
            let bridge = BridgeError::from(error.clone());
            assert_eq!(bridge.code, error.code());
            assert_eq!(bridge.message, error.to_string());
            assert!(!bridge.message.is_empty());
        }
        assert_eq!(
            BridgeError::invalid_argument("probe").code,
            IztroError::InvalidArgument("probe".into()).code()
        );
        assert_eq!(
            BridgeError::internal("probe").code,
            IztroError::Internal("probe".into()).code()
        );
    }
}

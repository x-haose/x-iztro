//! 排盘入口的错误类型。

use std::fmt;

/// 排盘与运限入口的参数错误。
///
/// 入口函数对全部外部输入做前置校验并以本类型报错，库内部不因非法参数
/// panic。这对 wasm 目标尤为关键：wasm 下 panic 即 abort（trap），且每次
/// trap 都会永久损耗模块实例的栈空间，累积后连合法调用也会失败。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IztroError {
    /// 日期字符串非法：格式不是 "YYYY-M-D"、该日期不存在，
    /// 或超出支持范围（公历 1583-9999 年）。
    InvalidDate(String),
    /// 时辰索引超出 0-12（0=早子时，12=晚子时）。
    InvalidTimeIndex(u8),
}

impl fmt::Display for IztroError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDate(msg) => f.write_str(msg),
            Self::InvalidTimeIndex(t) => write!(f, "time_index must be 0-12, got {t}"),
        }
    }
}

impl std::error::Error for IztroError {}

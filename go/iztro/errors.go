package iztro

import "errors"

// 结构化错误。
//
// 本包所有失败都返回 *Error：Code 是机器可读的类别（与 Rust 核心 IztroError
// 的 code 一一对应），Message 是原始英文描述。用 errors.Is 按类别判断、
// 用 errors.As 取出 Code 与 Message：
//
//	_, err := iztro.BySolar("2000-13-1", 2, "male", true, "zh-CN", nil)
//	errors.Is(err, iztro.ErrInvalidDate)   // true
//
//	var e *iztro.Error
//	errors.As(err, &e)                     // e.Code == "invalid_date"

// 错误类别哨兵，配合 errors.Is 使用。
var (
	// ErrInvalidDate 表示日期格式非法、日期不存在，或超出支持范围（公历 1583-9999）。
	ErrInvalidDate = errors.New("invalid date")
	// ErrInvalidTimeIndex 表示时辰索引越界（合法值 0-12）。
	ErrInvalidTimeIndex = errors.New("invalid time index")
	// ErrInvalidArgument 表示其余入参或配置非法，如未知的性别、语言、星耀标识。
	ErrInvalidArgument = errors.New("invalid argument")
	// ErrInternal 表示库内部缺陷或 Go 侧的运行时故障，如 wasm 实例化失败、结果解码失败。
	ErrInternal = errors.New("internal error")
)

// 错误类别代码，即 *Error 的 Code 字段取值。
const (
	// CodeInvalidDate 对应 ErrInvalidDate
	CodeInvalidDate = "invalid_date"
	// CodeInvalidTimeIndex 对应 ErrInvalidTimeIndex
	CodeInvalidTimeIndex = "invalid_time_index"
	// CodeInvalidArgument 对应 ErrInvalidArgument
	CodeInvalidArgument = "invalid_argument"
	// CodeInternal 对应 ErrInternal
	CodeInternal = "internal"
)

// Error 为带类别的错误。
type Error struct {
	// Code 为错误类别代码（Code* 常量之一）
	Code string
	// Message 为错误描述原文，如 "invalid gender 'x': expected 'male' or 'female'"
	Message string
}

// Error 返回 "iztro: " 前缀加错误描述。
func (e *Error) Error() string {
	return "iztro: " + e.Message
}

// Unwrap 返回本错误所属类别的哨兵，使 errors.Is 可按类别匹配；
// 无法识别的 Code 归为 ErrInternal。
func (e *Error) Unwrap() error {
	switch e.Code {
	case CodeInvalidDate:
		return ErrInvalidDate
	case CodeInvalidTimeIndex:
		return ErrInvalidTimeIndex
	case CodeInvalidArgument:
		return ErrInvalidArgument
	default:
		return ErrInternal
	}
}

// invalidArgument 构造入参非法错误，供 Go 侧实现的校验使用。
func invalidArgument(message string) *Error {
	return &Error{Code: CodeInvalidArgument, Message: message}
}

// internalError 构造内部故障错误，供 Go 侧的 wasm 与编解码失败使用。
func internalError(message string) *Error {
	return &Error{Code: CodeInternal, Message: message}
}

// Package iztro 提供紫微斗数排盘与运限计算的 Go API。
//
// 实现方式：内嵌 x-iztro 编译出的 WebAssembly 模块（wasm32-wasip1），
// 通过纯 Go 的 wazero 运行时调用——无 cgo，保留 Go 的交叉编译能力，
// `go get` 即用，无需本机安装 Rust 工具链。
//
// 返回类型化的 Astrolabe/Horoscope 结构体（见 types.go），字段同时携带
// 翻译文本与语言无关标识（keys.go 常量），判断方法在任何输出语言下可用。
// 计算失败时返回 *Error，可用 errors.Is 按类别判断（见 errors.go）。
//
// 跨 wasm 的入口都有 Context 变体（BySolarContext 等），用于取消等待与超时；
// 无 Context 的形式等价于传 context.Background()。首次调用会编译 wasm 模块，
// 想把这份开销提前完成时调用 Warmup。
//
// 更新内嵌 wasm：在仓库根目录执行
//
//	cargo build --release --target wasm32-wasip1
//	cp target/wasm32-wasip1/release/x_iztro.wasm go/iztro/
package iztro

import (
	"context"
	"encoding/json"
	"fmt"
	"time"
)

// Config 为排盘配置；nil 或缺省键取默认值（与 JS iztro 默认一致）。
type Config struct {
	// YearDivide 为年干支的换年时点："normal" 正月初一（默认）、"exact" 立春。
	YearDivide string `json:"yearDivide,omitempty"`

	// HoroscopeDivide 为流年神煞取年支的换年时点："normal" 正月初一（默认）、"exact" 立春。
	HoroscopeDivide string `json:"horoscopeDivide,omitempty"`

	// AgeDivide 为小限虚岁的进位时点："normal" 农历生日（默认）、"birthday" 阳历生日。
	AgeDivide string `json:"ageDivide,omitempty"`

	// DayDivide 为晚子时的归日方式："forward" 归次日（默认）、"current" 归当日。
	DayDivide string `json:"dayDivide,omitempty"`

	// Algorithm 为算法派别："default" 通行派（默认）、"zhongzhou" 中州派。
	Algorithm string `json:"algorithm,omitempty"`

	// AstroType 为排盘视角："heaven" 天盘（默认）、"earth" 地盘、"human" 人盘。
	// 地盘以身宫干支、人盘以福德宫干支起五行局重排。
	AstroType string `json:"astroType,omitempty"`

	// Mutagens 为自定义四化表：天干标识 → 四颗星标识（禄、权、科、忌）。
	// 按天干整表替换默认值，未列出的天干仍用默认表。
	Mutagens map[string][]string `json:"mutagens,omitempty"`

	// Brightness 为自定义亮度表：星耀标识 → 十二宫亮度标识（十二项，
	// 空串表示该宫无亮度，索引 0 为寅宫）。按星耀整表替换默认值。
	Brightness map[string][]string `json:"brightness,omitempty"`
}

// decodeAstrolabe 把排盘结果 JSON 解成星盘，回填反向引用与原始配置。
func decodeAstrolabe(raw []byte, config *Config) (*Astrolabe, error) {
	var out Astrolabe
	if err := json.Unmarshal(raw, &out); err != nil {
		return nil, internalError("decode astrolabe: " + err.Error())
	}
	out.reqConfig = config
	out.link()
	return &out, nil
}

// BySolar 以阳历日期排盘，返回类型化星盘。
//   - solarDate: "YYYY-M-D"，如 "2000-8-16"
//   - timeIndex: 时辰索引 0-12（0=早子时，12=晚子时）
//   - gender: GenderMale / GenderFemale
//   - fixLeap: 阳历日期落在闰月十五之后时是否视作次月
//   - language: 盘面语言（Language* 常量）
//   - config: 排盘配置，nil 取默认
func BySolar(solarDate string, timeIndex uint8, gender Gender, fixLeap bool, language Language, config *Config) (*Astrolabe, error) {
	return BySolarContext(context.Background(), solarDate, timeIndex, gender, fixLeap, language, config)
}

// BySolarContext 为 BySolar 的 Context 变体；ctx 用于取消等待 wasm 实例。
func BySolarContext(ctx context.Context, solarDate string, timeIndex uint8, gender Gender, fixLeap bool, language Language, config *Config) (*Astrolabe, error) {
	raw, err := callWasm(ctx, fnBySolar, map[string]any{
		"solarDate": solarDate,
		"timeIndex": timeIndex,
		"gender":    gender,
		"fixLeap":   fixLeap,
		"language":  language,
		"config":    config,
	})
	if err != nil {
		return nil, err
	}
	return decodeAstrolabe(raw, config)
}

// ByLunar 以农历日期排盘，返回类型化星盘。
//   - lunarDate: "YYYY-M-D"，如 "2000-7-17"
//   - leap: 输入月是否闰月及闰月处理方式（NotLeapMonth / LeapMonthKeep / LeapMonthFixed）；
//     标为闰月但该月没有闰月时按普通月处理
//   - 其余参数同 BySolar
func ByLunar(lunarDate string, timeIndex uint8, gender Gender, leap LeapMonth, language Language, config *Config) (*Astrolabe, error) {
	return ByLunarContext(context.Background(), lunarDate, timeIndex, gender, leap, language, config)
}

// ByLunarContext 为 ByLunar 的 Context 变体；ctx 用于取消等待 wasm 实例。
func ByLunarContext(ctx context.Context, lunarDate string, timeIndex uint8, gender Gender, leap LeapMonth, language Language, config *Config) (*Astrolabe, error) {
	isLeapMonth, fixLeap, err := leap.flags()
	if err != nil {
		return nil, err
	}
	raw, err := callWasm(ctx, fnByLunar, map[string]any{
		"lunarDate":   lunarDate,
		"timeIndex":   timeIndex,
		"gender":      gender,
		"isLeapMonth": isLeapMonth,
		"fixLeap":     fixLeap,
		"language":    language,
		"config":      config,
	})
	if err != nil {
		return nil, err
	}
	return decodeAstrolabe(raw, config)
}

// Rearranged 以指定干支为命宫重排本盘，返回新盘；本盘不变。
//
// 传入的干支决定五行局，进而决定紫微天府落点、十二宫名、长生十二神与大限小限；
// 辅星、杂耀（天伤天使天才除外）、博士十二神、岁前将前十二神沿用原盘。
//
// 常规的天盘、地盘、人盘用 Config.AstroType 指定即可，本方法用于从任意干支起盘。
// fromStemKey / fromBranchKey 取 Stem* / Branch* 常量。
func (a *Astrolabe) Rearranged(fromStemKey string, fromBranchKey string) (*Astrolabe, error) {
	return a.RearrangedContext(context.Background(), fromStemKey, fromBranchKey)
}

// RearrangedContext 为 Rearranged 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) RearrangedContext(ctx context.Context, fromStemKey string, fromBranchKey string) (*Astrolabe, error) {
	if a == nil {
		return nil, invalidArgument("rearranged: nil astrolabe")
	}
	config := a.requestConfig()
	raw, err := callWasm(ctx, fnBySolar, map[string]any{
		"solarDate":  a.SolarDate,
		"timeIndex":  a.TimeIndex,
		"gender":     a.GenderKey,
		"fixLeap":    a.FixLeap,
		"language":   a.Language,
		"config":     config,
		"fromStem":   fromStemKey,
		"fromBranch": fromBranchKey,
	})
	if err != nil {
		return nil, err
	}
	out, err := decodeAstrolabe(raw, config)
	if err != nil {
		return nil, err
	}
	// 记住重排起点：格局判定等再次发起计算的接口靠它把重排带给内核
	out.fromStem = fromStemKey
	out.fromBranch = fromBranchKey
	return out, nil
}

// Horoscope 以本命盘为起点计算目标日期的运限，返回的运限持有本盘。
//
//   - targetDate: 目标阳历日期，"YYYY-M-D"（分隔符可为 - / .，可带 " HH:mm" 或 " HH:mm:ss"）
//   - targetTimeIndex: 目标时辰索引 0-12
//
// 排盘上下文（出生日期、时辰、性别、闰月修正、语言、配置）取自本盘，
// 与 wasm 侧的无状态调用协定一致。
func (a *Astrolabe) Horoscope(targetDate string, targetTimeIndex uint8) (*Horoscope, error) {
	return a.HoroscopeContext(context.Background(), targetDate, targetTimeIndex)
}

// HoroscopeContext 为 Horoscope 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeContext(ctx context.Context, targetDate string, targetTimeIndex uint8) (*Horoscope, error) {
	return a.horoscope(ctx, &Horoscope{targetDate: targetDate, targetTimeIndex: &targetTimeIndex})
}

// HoroscopeAt 以本命盘为起点计算目标日期的运限，时辰由 targetDate 的时间部分换算
// （23 点为晚子），不带时间即早子时；对应 iztro 省略 timeIndex 的 horoscope(targetDate)。
func (a *Astrolabe) HoroscopeAt(targetDate string) (*Horoscope, error) {
	return a.HoroscopeAtContext(context.Background(), targetDate)
}

// HoroscopeAtContext 为 HoroscopeAt 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeAtContext(ctx context.Context, targetDate string) (*Horoscope, error) {
	return a.horoscope(ctx, &Horoscope{targetDate: targetDate})
}

// HoroscopeAtTime 以本命盘为起点计算时刻 t 的运限：日期取 t 的年月日，时辰由 t 的小时换算；
// 对应 iztro 传 Date 对象的 horoscope(date)。时区按 t 自身的时区读年月日时。
func (a *Astrolabe) HoroscopeAtTime(t time.Time) (*Horoscope, error) {
	return a.HoroscopeAtTimeContext(context.Background(), t)
}

// HoroscopeAtTimeContext 为 HoroscopeAtTime 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeAtTimeContext(ctx context.Context, t time.Time) (*Horoscope, error) {
	date := fmt.Sprintf("%d-%d-%d %d:%d:%d", t.Year(), int(t.Month()), t.Day(), t.Hour(), t.Minute(), t.Second())
	return a.horoscope(ctx, &Horoscope{targetDate: date})
}

// HoroscopeNow 以本命盘为起点计算**此刻**的运限，日期与时辰取本地时钟。
func (a *Astrolabe) HoroscopeNow() (*Horoscope, error) {
	return a.HoroscopeNowContext(context.Background())
}

// HoroscopeNowContext 为 HoroscopeNow 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeNowContext(ctx context.Context) (*Horoscope, error) {
	return a.HoroscopeAtTimeContext(ctx, time.Now())
}

// HoroscopeOfYear 返回指定农历年的运限，返回的运限持有本盘：目标日期取该年六月初一，
// 与 YearlyList 同一约定。
//
// 只问某一年时用它，不必自己挑公历日期——公历年初到正月初一之前仍属上一个农历年，
// 拿公历 1 月 1 日去算会得到上一年的流年。大限、小限与流年在默认分界配置下整个农历年不变；
// Config.AgeDivide 取生日分界或 Config.HoroscopeDivide 取立春分界时年内仍会换层，结果是六月初一当天的。
// 流月、流日、流时也在结果里，是六月初一那一天的，文本用 TextOptions{Through: ScopeYearly} 只写到流年。
// 返回的运限再发起的计算（ToText、Patterns、按盘取材）仍按该农历年取运限。
//
// lunarYear 不在 1-9999 时返回 ErrInvalidArgument。
func (a *Astrolabe) HoroscopeOfYear(lunarYear int) (*Horoscope, error) {
	return a.HoroscopeOfYearContext(context.Background(), lunarYear)
}

// HoroscopeOfYearContext 为 HoroscopeOfYear 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeOfYearContext(ctx context.Context, lunarYear int) (*Horoscope, error) {
	return a.horoscope(ctx, &Horoscope{targetYear: &lunarYear})
}

// horoscope 按 out 上已填好的运限目标（targetDate 与 targetTimeIndex，或 targetYear）发起运限计算，
// 把结果解进 out 并让它持有本盘；目标留在 out 上，供它再发起计算时转发。
func (a *Astrolabe) horoscope(ctx context.Context, out *Horoscope) (*Horoscope, error) {
	if a == nil {
		return nil, invalidArgument("horoscope: nil astrolabe")
	}
	payload := map[string]any{
		"solarDate": a.SolarDate,
		"timeIndex": a.TimeIndex,
		"gender":    a.GenderKey,
		"fixLeap":   a.FixLeap,
		"language":  a.Language,
		"config":    a.requestConfig(),
	}
	out.addTarget(payload)
	a.addRearrange(payload)
	raw, err := callWasm(ctx, fnHoroscope, payload)
	if err != nil {
		return nil, err
	}
	if err := json.Unmarshal(raw, out); err != nil {
		return nil, internalError("decode horoscope: " + err.Error())
	}
	out.astrolabe = a
	return out, nil
}

package iztro

import (
	"context"
	"fmt"
)

// 格局判定在 wasm 内核完成（规则见文档站「格局」一页），这里只把命中结果包成类型化对象。
// 本命盘调用 Astrolabe.Patterns，运限某层视角调用 Horoscope.Patterns；
// 两者都是无状态接口，从星盘自带的排盘上下文重新发起计算。

// PatternConfig 为格局判定口径。多口径的格局一律以 PatternHit.Variant 报出，
// 这里只放会改变事实判定本身的开关；nil 取默认（亮度按 iztro 表、借宫、流曜参与）。
//
// 两个布尔开关是 *bool：内核默认都是 true，值类型 bool 的零值会把「未设置」
// 静默当成 false——只设 BrightnessSource 的部分构造就会悄悄关掉借宫与流曜。
// nil 表示未设置（序列化时省略该键、由内核取默认），显式取值用 Bool 构造：
//
//	cfg := &PatternConfig{BrightnessSource: BrightnessSourcePositional} // 借宫、流曜仍为默认 true
//	cfg := &PatternConfig{Borrow: Bool(false)}                          // 只关借宫
type PatternConfig struct {
	// BrightnessSource 为日月明暗依据：BrightnessSourceTable（默认）或 BrightnessSourcePositional；空串取默认
	BrightnessSource string `json:"brightnessSource,omitempty"`
	// Borrow 为空宫是否借对宫主星参与判定；nil 取内核默认 true
	Borrow *bool `json:"borrow,omitempty"`
	// FlowStars 为运限视角下流曜（运禄/流禄等）是否等同对应本命辅星参与判定；nil 取内核默认 true
	FlowStars *bool `json:"flowStars,omitempty"`
}

// Bool 返回 v 的指针，给 PatternConfig、ReverseCriteria 的 *bool 可选字段赋值用。
func Bool(v bool) *bool {
	return &v
}

// DefaultPatternConfig 返回默认口径：亮度按 iztro 表、借宫、流曜参与。
func DefaultPatternConfig() *PatternConfig {
	return &PatternConfig{
		BrightnessSource: BrightnessSourceTable,
		Borrow:           Bool(true),
		FlowStars:        Bool(true),
	}
}

// PatternStar 为参与成格的一颗星及其落宫。
type PatternStar struct {
	// Key 为语言无关星耀标识
	Key string `json:"key"`
	// Name 为按排盘语言翻译的星耀名称
	Name string `json:"name"`
	// PalaceIndex 为落宫索引（0-11，寅宫为 0）
	PalaceIndex int `json:"palaceIndex"`
	// Brightness 为亮度显示文本，无亮度为空串
	Brightness string `json:"brightness,omitempty"`
	// BrightnessKey 为语言无关亮度标识（如 BrightnessMiao），无亮度为空串
	BrightnessKey string `json:"brightnessKey,omitempty"`
	// Mutagen 为判定视角下的四化显示文本（本命为生年四化，运限为该层四化），无四化为空串
	Mutagen string `json:"mutagen,omitempty"`
	// MutagenKey 为语言无关四化标识（如 MutagenLu），无四化为空串
	MutagenKey string `json:"mutagenKey,omitempty"`
}

// PatternHit 为一次格局命中。
type PatternHit struct {
	// Key 为语言无关格局标识（如 PatternZiFuTongGong）
	Key string `json:"key"`
	// Name 为按排盘语言翻译的格局名称
	Name string `json:"name"`
	// Scope 为判定视角（ScopeOrigin 或运限层级）
	Scope string `json:"scope"`
	// PalaceIndex 为成格所在宫位索引（0-11）：多数为命宫，「身命」类可为身宫，任一宫成格的为实际落宫
	PalaceIndex int `json:"palaceIndex"`
	// PalaceName 为成格所在宫位在该视角下的宫名（按排盘语言翻译）
	PalaceName string `json:"palaceName"`
	// PalaceNameKey 为语言无关宫名标识（如 PalaceSoul）
	PalaceNameKey string `json:"palaceNameKey"`
	// Variant 为多口径格局命中的口径（如君臣庆会四形式），单口径为空串
	Variant string `json:"variant,omitempty"`
	// Broken 为页面称「破格 / 加杀平常」的条件是否触发：成格照报，仅作标记
	Broken bool `json:"broken"`
	// Stars 为参与成格的星与落宫
	Stars []PatternStar `json:"stars"`
}

// Is 判断是否为指定格局（传 PatternZiFuTongGong 等常量）。
func (h *PatternHit) Is(patternKey string) bool {
	return h != nil && h.Key == patternKey
}

// InPalace 判断成格宫位在该视角下是否为指定宫名（宫位标识或当前语言宫名）。
func (h *PatternHit) InPalace(nameKeyOrName string) bool {
	return h != nil && (h.PalaceNameKey == nameKeyOrName || h.PalaceName == nameKeyOrName)
}

// Patterns 返回本命盘的全部格局命中；config 传 nil 取默认口径。
func (a *Astrolabe) Patterns(config *PatternConfig) ([]PatternHit, error) {
	return a.PatternsContext(context.Background(), config)
}

// PatternsContext 为 Patterns 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) PatternsContext(ctx context.Context, config *PatternConfig) ([]PatternHit, error) {
	if a == nil {
		return nil, invalidArgument("patterns: nil astrolabe")
	}
	payload := map[string]any{
		"kind":          "patterns",
		"solarDate":     a.SolarDate,
		"timeIndex":     a.TimeIndex,
		"gender":        a.GenderKey,
		"fixLeap":       a.FixLeap,
		"language":      a.Language,
		"config":        a.requestConfig(),
		"patternConfig": config,
	}
	a.addRearrange(payload)
	var out []PatternHit
	return out, utilQueryContext(ctx, payload, &out)
}

// Patterns 返回某运限层视角的格局命中：以该层命宫为命宫、合并该层流曜与四化后重跑全部规则，
// 另加两条行运格（禄衰马困、风云际会）。scope 传 ScopeDecadal 等常量，ScopeOrigin 等同本命盘；
// config 传 nil 取默认口径。
func (h *Horoscope) Patterns(scope string, config *PatternConfig) ([]PatternHit, error) {
	return h.PatternsContext(context.Background(), scope, config)
}

// PatternsContext 为 Patterns 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (h *Horoscope) PatternsContext(ctx context.Context, scope string, config *PatternConfig) ([]PatternHit, error) {
	if h == nil || h.astrolabe == nil {
		return nil, invalidArgument("horoscopePatterns: horoscope must be created by Astrolabe.Horoscope")
	}
	a := h.astrolabe
	payload := map[string]any{
		"kind":            "horoscopePatterns",
		"solarDate":       a.SolarDate,
		"timeIndex":       a.TimeIndex,
		"gender":          a.GenderKey,
		"fixLeap":         a.FixLeap,
		"language":        a.Language,
		"config":          a.requestConfig(),
		"targetDate":      h.SolarDate,
		"targetTimeIndex": h.targetTimeIndex,
		"scope":           scope,
		"patternConfig":   config,
	}
	a.addRearrange(payload)
	var out []PatternHit
	return out, utilQueryContext(ctx, payload, &out)
}

// PatternsToText 生成本命盘格局命中的语义化文本；config 传 nil 取默认口径。
func (a *Astrolabe) PatternsToText(config *PatternConfig) (string, error) {
	return a.PatternsToTextContext(context.Background(), config)
}

// PatternsToTextContext 为 PatternsToText 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) PatternsToTextContext(ctx context.Context, config *PatternConfig) (string, error) {
	return a.PatternsToTextWithContext(ctx, config, Knowledge{})
}

// PatternsToTextWith 生成本命盘格局命中的语义化文本并按 knowledge 追加命中格局的释义节；
// config 传 nil 取默认口径。
func (a *Astrolabe) PatternsToTextWith(config *PatternConfig, knowledge Knowledge) (string, error) {
	return a.PatternsToTextWithContext(context.Background(), config, knowledge)
}

// PatternsToTextWithContext 为 PatternsToTextWith 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) PatternsToTextWithContext(ctx context.Context, config *PatternConfig, knowledge Knowledge) (string, error) {
	if a == nil {
		return "", invalidArgument("patternsToText: nil astrolabe")
	}
	payload := a.textPayload("patternsToText")
	payload["patternConfig"] = config
	knowledge.apply(payload)
	var out string
	return out, utilQueryContext(ctx, payload, &out)
}

// PatternsToText 生成某运限层视角格局命中的语义化文本。
// scope 传 ScopeDecadal 等常量，ScopeOrigin 等同本命盘；config 传 nil 取默认口径。
func (h *Horoscope) PatternsToText(scope string, config *PatternConfig) (string, error) {
	return h.PatternsToTextContext(context.Background(), scope, config)
}

// PatternsToTextContext 为 PatternsToText 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (h *Horoscope) PatternsToTextContext(ctx context.Context, scope string, config *PatternConfig) (string, error) {
	return h.PatternsToTextWithContext(ctx, scope, config, Knowledge{})
}

// PatternsToTextWith 生成某运限层视角格局命中的语义化文本并按 knowledge 追加命中格局的释义节。
// scope 传 ScopeDecadal 等常量，ScopeOrigin 等同本命盘；config 传 nil 取默认口径。
func (h *Horoscope) PatternsToTextWith(scope string, config *PatternConfig, knowledge Knowledge) (string, error) {
	return h.PatternsToTextWithContext(context.Background(), scope, config, knowledge)
}

// PatternsToTextWithContext 为 PatternsToTextWith 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (h *Horoscope) PatternsToTextWithContext(ctx context.Context, scope string, config *PatternConfig, knowledge Knowledge) (string, error) {
	if h == nil || h.astrolabe == nil {
		return "", invalidArgument("horoscopePatternsToText: horoscope must be created by Astrolabe.Horoscope")
	}
	payload := h.astrolabe.textPayload("horoscopePatternsToText")
	payload["targetDate"] = h.SolarDate
	payload["targetTimeIndex"] = h.targetTimeIndex
	payload["scope"] = scope
	payload["patternConfig"] = config
	knowledge.apply(payload)
	var out string
	return out, utilQueryContext(ctx, payload, &out)
}

// String 返回「格局名(宫名)」形式的简要描述，便于日志与调试。
func (h PatternHit) String() string {
	if h.Variant != "" {
		return fmt.Sprintf("%s(%s,%s)", h.Name, h.PalaceName, h.Variant)
	}
	return fmt.Sprintf("%s(%s)", h.Name, h.PalaceName)
}

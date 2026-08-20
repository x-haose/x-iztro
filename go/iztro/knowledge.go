package iztro

import (
	"context"
	"encoding/json"
	"fmt"
)

// knowledgeSchemaVersion 为支持的知识包格式版本，与 Rust 内核 knowledge::SCHEMA_VERSION 一致。
const knowledgeSchemaVersion = 1

// 知识包：语言无关标识 → 解读文本与门派属性。
// 内核只负责事实判定，星耀怎么解读、格局意味着什么、宫位与四化的含义属于门派观点，
// 全部放在知识包里。格式见仓库 knowledge/SCHEMA.md；内核内嵌一份默认包
// （源自 iztro-docs《学习》页，MIT），使用者可整包替换或用覆盖包逐条合并（合并在 wasm 内核完成）。
// 所有键都是 x-iztro 的语言无关标识（Star*/Pattern*/Palace*/Mutagen* 常量），文本字段为 Markdown。

// KnowledgeSource 为包的来源与许可信息。
type KnowledgeSource struct {
	Name        string `json:"name,omitempty"`
	URL         string `json:"url,omitempty"`
	Commit      string `json:"commit,omitempty"`
	License     string `json:"license,omitempty"`
	Author      string `json:"author,omitempty"`
	RetrievedAt string `json:"retrievedAt,omitempty"`
	// Adapted 为改写说明：文本经过整理改写时注明
	Adapted string `json:"adapted,omitempty"`
}

// StarAttributes 为星耀的门派属性（全部可选，缺省为空）。
type StarAttributes struct {
	// YinYang 为阴阳（yin / yang）
	YinYang string `json:"yinYang,omitempty"`
	// FiveElements 为五行（wood / fire / earth / metal / water）
	FiveElements string `json:"fiveElements,omitempty"`
	// Stem 为五行所带天干（jia…gui）
	Stem string `json:"stem,omitempty"`
	// FiveElementsNote 为五行的补充说明
	FiveElementsNote string `json:"fiveElementsNote,omitempty"`
	// Dipper 为斗分
	Dipper string `json:"dipper,omitempty"`
	// Chemistry 为化气
	Chemistry string `json:"chemistry,omitempty"`
	// Career 为职业（主何事）
	Career string `json:"career,omitempty"`
	// Duty 为职务
	Duty string `json:"duty,omitempty"`
	// Aliases 为别号
	Aliases []string `json:"aliases,omitempty"`
	// ElementColor 为五行色
	ElementColor string `json:"elementColor,omitempty"`
	// EnergyColor 为能量色
	EnergyColor string `json:"energyColor,omitempty"`
}

// StarEntry 为一颗星耀的知识条目。
type StarEntry struct {
	// Name 为该语言的显示名
	Name string `json:"name,omitempty"`
	// Category 为类别（major / minor / adjective / dec）
	Category string `json:"category,omitempty"`
	// Group 为分组（杂耀的分类、神煞的组别）
	Group string `json:"group,omitempty"`
	// Attributes 为门派属性
	Attributes StarAttributes `json:"attributes"`
	// Intro 为解读正文
	Intro string `json:"intro,omitempty"`
	// Combinations 为与另一颗主星同宫的组合解读，键为对方星耀标识
	Combinations map[string]string `json:"combinations,omitempty"`
}

// PatternEntry 为一条格局的知识条目。
type PatternEntry struct {
	// Name 为该语言的显示名
	Name string `json:"name,omitempty"`
	// Quotes 为古籍引文
	Quotes []string `json:"quotes,omitempty"`
	// Conditions 为来源对成立条件的文字描述
	Conditions string `json:"conditions,omitempty"`
	// Intro 为解读正文
	Intro string `json:"intro,omitempty"`
}

// TextEntry 为只有名称与正文的条目（宫位、四化）。
type TextEntry struct {
	// Name 为该语言的显示名
	Name string `json:"name,omitempty"`
	// Intro 为正文
	Intro string `json:"intro,omitempty"`
}

// ConceptEntry 为术语与基础概念条目。
type ConceptEntry struct {
	// Title 为标题
	Title string `json:"title,omitempty"`
	// Intro 为正文
	Intro string `json:"intro,omitempty"`
}

// KnowledgePack 为一份知识包。
type KnowledgePack struct {
	// Schema 为格式版本
	Schema int `json:"schema"`
	// ID 为包标识
	ID string `json:"id"`
	// Version 为包版本
	Version string `json:"version"`
	// Language 为文本语言（如 LanguageZhCN）
	Language string `json:"language"`
	// Extends 为覆盖包所覆盖的包标识；独立包为空
	Extends string `json:"extends,omitempty"`
	// Source 为来源与许可
	Source KnowledgeSource `json:"source"`
	// Stars 为星耀条目，键为星耀标识
	Stars map[string]StarEntry `json:"stars"`
	// Patterns 为格局条目，键为格局标识
	Patterns map[string]PatternEntry `json:"patterns"`
	// Palaces 为宫位条目，键为宫位标识
	Palaces map[string]TextEntry `json:"palaces"`
	// Mutagens 为四化条目，键为四化标识
	Mutagens map[string]TextEntry `json:"mutagens"`
	// Concepts 为术语与基础概念，键为 slug
	Concepts map[string]ConceptEntry `json:"concepts"`
}

// BuiltinKnowledgePack 返回内嵌的默认包（源自 iztro-docs，MIT）；该语言没有默认包时返回错误（目前只有 zh-CN）。
func BuiltinKnowledgePack(language Language) (*KnowledgePack, error) {
	return BuiltinKnowledgePackContext(context.Background(), language)
}

// BuiltinKnowledgePackContext 为 BuiltinKnowledgePack 的 Context 变体；ctx 用于取消等待 wasm 实例。
func BuiltinKnowledgePackContext(ctx context.Context, language Language) (*KnowledgePack, error) {
	var out KnowledgePack
	if err := utilQueryContext(ctx, map[string]any{
		"kind":     "knowledgePack",
		"language": language,
	}, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// ParseKnowledgePack 由 JSON 文本解析一份包，并校验格式版本
// （与 Rust 内核解析同语义：未声明 schema 或版本比支持的新都拒绝）。
func ParseKnowledgePack(data []byte) (*KnowledgePack, error) {
	var out KnowledgePack
	if err := json.Unmarshal(data, &out); err != nil {
		return nil, invalidArgument("invalid knowledge pack: " + err.Error())
	}
	if out.Schema <= 0 {
		return nil, invalidArgument(`knowledge pack must declare "schema" (currently 1)`)
	}
	if out.Schema > knowledgeSchemaVersion {
		return nil, invalidArgument(fmt.Sprintf(
			"knowledge pack schema %d is newer than supported %d", out.Schema, knowledgeSchemaVersion))
	}
	return &out, nil
}

// Merged 以本包为底依次叠加覆盖包，返回新包（本包不变）。
// 合并规则见 knowledge/SCHEMA.md：覆盖包的非空字段覆盖同键条目的对应字段，
// attributes / combinations 逐字段合并，数组字段整体替换。
func (p *KnowledgePack) Merged(overlays ...*KnowledgePack) (*KnowledgePack, error) {
	return p.MergedContext(context.Background(), overlays...)
}

// MergedContext 为 Merged 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (p *KnowledgePack) MergedContext(ctx context.Context, overlays ...*KnowledgePack) (*KnowledgePack, error) {
	if p == nil {
		return nil, invalidArgument("mergeKnowledgePacks: nil base pack")
	}
	packs := make([]any, 0, 1+len(overlays))
	packs = append(packs, p)
	for _, o := range overlays {
		if o == nil {
			return nil, invalidArgument("mergeKnowledgePacks: nil overlay pack")
		}
		packs = append(packs, o)
	}
	var out KnowledgePack
	if err := utilQueryContext(ctx, map[string]any{
		"kind":           "mergeKnowledgePacks",
		"knowledgePacks": packs,
	}, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// Star 取星耀条目（传 StarZiwei 等常量），没有返回 nil。
func (p *KnowledgePack) Star(starKey string) *StarEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Stars[starKey]; ok {
		return &e
	}
	return nil
}

// Pattern 取格局条目（传 PatternZiFuTongGong 等常量），没有返回 nil。
func (p *KnowledgePack) Pattern(patternKey string) *PatternEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Patterns[patternKey]; ok {
		return &e
	}
	return nil
}

// Palace 取宫位条目（传 PalaceSoul 等常量），没有返回 nil。
func (p *KnowledgePack) Palace(palaceKey string) *TextEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Palaces[palaceKey]; ok {
		return &e
	}
	return nil
}

// Mutagen 取四化条目（传 MutagenLu 等常量），没有返回 nil。
func (p *KnowledgePack) Mutagen(mutagenKey string) *TextEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Mutagens[mutagenKey]; ok {
		return &e
	}
	return nil
}

// Concept 取术语条目，没有返回 nil。
func (p *KnowledgePack) Concept(slug string) *ConceptEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Concepts[slug]; ok {
		return &e
	}
	return nil
}

// StarIntro 取星耀解读正文，没有返回空串。
func (p *KnowledgePack) StarIntro(starKey string) string {
	if e := p.Star(starKey); e != nil {
		return e.Intro
	}
	return ""
}

// PatternIntro 取格局解读正文，没有返回空串。
func (p *KnowledgePack) PatternIntro(patternKey string) string {
	if e := p.Pattern(patternKey); e != nil {
		return e.Intro
	}
	return ""
}

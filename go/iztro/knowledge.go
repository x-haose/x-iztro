package iztro

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// 知识包：语言无关标识 → 解读文本与门派属性。
// 内核只负责事实判定，星耀怎么解读、格局意味着什么、宫位与四化的含义属于门派观点，
// 全部放在知识包里。格式见仓库 knowledge/SCHEMA.md；内核内嵌一份默认包
// （源自 iztro-docs《学习》页，MIT），使用者可整包替换或用覆盖包逐条合并（合并在 wasm 内核完成）。
// 所有键都是 x-iztro 的语言无关标识（Star*/Pattern*/Palace*/Mutagen* 常量），文本字段为 Markdown。

// KnowledgeSource 为包的来源与许可信息。
type KnowledgeSource struct {
	// Name 为来源名称（如 iztro-docs）
	Name string `json:"name,omitempty"`
	// URL 为来源仓库或页面地址
	URL string `json:"url,omitempty"`
	// Commit 为提取时锁定的来源 commit
	Commit string `json:"commit,omitempty"`
	// License 为来源的许可协议（如 MIT）
	License string `json:"license,omitempty"`
	// Author 为来源作者
	Author string `json:"author,omitempty"`
	// RetrievedAt 为提取日期（YYYY-MM-DD）
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
	// Category 为类别（major / minor / adjective / dec / flow）
	Category string `json:"category,omitempty"`
	// Group 为分组（杂耀的分类、神煞的组别）
	Group string `json:"group,omitempty"`
	// Attributes 为门派属性
	Attributes StarAttributes `json:"attributes"`
	// Intro 为解读正文
	Intro string `json:"intro,omitempty"`
	// Summary 为正文的摘要（单段）；正文不超过摘要层上限（显示宽度 240，中文 120 字）的条目为空
	Summary string `json:"summary,omitempty"`
	// Combinations 为与另一颗主星同宫的组合解读，键为对方星耀标识
	Combinations map[string]string `json:"combinations,omitempty"`
	// CombinationSummaries 为组合解读的摘要，与 Combinations 同键成对
	CombinationSummaries map[string]string `json:"combinationSummaries,omitempty"`
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
	// Summary 为正文的摘要（单段）；正文不超过摘要层上限的条目为空
	Summary string `json:"summary,omitempty"`
}

// TextEntry 为只有名称与正文的条目（宫位、四化）。
type TextEntry struct {
	// Name 为该语言的显示名
	Name string `json:"name,omitempty"`
	// Intro 为正文
	Intro string `json:"intro,omitempty"`
	// Summary 为正文的摘要（单段）；正文不超过摘要层上限的条目为空
	Summary string `json:"summary,omitempty"`
}

// ConceptEntry 为术语与基础概念条目。
type ConceptEntry struct {
	// Title 为标题
	Title string `json:"title,omitempty"`
	// Intro 为正文
	Intro string `json:"intro,omitempty"`
}

// TopicEntry 为一个解读主题（如婚恋、财运）：读这类问题要看哪些宫，由 Astrolabe.TopicToText 按它选宫。
type TopicEntry struct {
	// Title 为显示名
	Title string `json:"title,omitempty"`
	// Surround 为连同各自三方四正一起取的宫位标识，按出现顺序渲染
	Surround []string `json:"surround,omitempty"`
	// Palaces 为只取本宫的宫位标识（可含 bodyPalace / originalPalace），排在 Surround 各组之后
	Palaces []string `json:"palaces,omitempty"`
	// Intro 为这个主题看什么、为什么取这些宫
	Intro string `json:"intro,omitempty"`
	// Summary 为正文的摘要（单段）；正文不超过摘要层上限的条目为空
	Summary string `json:"summary,omitempty"`
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
	// Topics 为解读主题，键为 slug
	Topics map[string]TopicEntry `json:"topics"`
}

// Knowledge 为语义化文本（ToTextWith 家族）的释义材料来源。零值表示不带释义，
// 文本只含盘面事实，与不带 With 的方法输出完全一致。
type Knowledge struct {
	// builtin 为真时取盘语言的内嵌包；该语言没有内嵌包（目前只有 zh-CN）时调用报 ErrInvalidArgument，不静默回退
	builtin bool
	// pack 为显式给定的知识包（自定义或合并后的包），builtin 为假且 pack 为 nil 即零值
	pack *KnowledgePack
}

// BuiltinKnowledge 表示用盘语言的内嵌知识包作释义；英文盘等无内嵌包的语言调用时报 ErrInvalidArgument。
func BuiltinKnowledge() Knowledge {
	return Knowledge{builtin: true}
}

// KnowledgeFrom 表示用给定的知识包作释义；传 nil 等同零值（不带释义）。
func KnowledgeFrom(pack *KnowledgePack) Knowledge {
	return Knowledge{pack: pack}
}

// apply 把释义来源写进查询入参；零值不加键，内核即只输出盘面事实。
func (k Knowledge) apply(payload map[string]any) {
	switch {
	case k.builtin:
		payload["knowledge"] = "builtin"
	case k.pack != nil:
		payload["knowledge"] = k.pack
	}
}

// TextOptions 为 ToTextWith 家族的输出选项。零值只输出盘面事实，与不带 With 的方法输出完全一致。
type TextOptions struct {
	// Knowledge 为释义材料来源；零值不带释义
	Knowledge Knowledge
	// Detail 为事实详略：TextDetailOverview 或 TextDetailFull；空串取完整
	Detail string
	// KnowledgeDepth 为释义深度：KnowledgeDepthSummary 或 KnowledgeDepthFull；空串取全文。
	// 取摘要层须同时给 Knowledge，所给包缺摘要时报 ErrInvalidArgument 并列出缺项
	KnowledgeDepth string
	// Through 为运限文本写到哪一层为止，更细的层级不写：ScopeDecadal 只写大限，ScopeYearly 写大限、
	// 小限、流年，ScopeMonthly / ScopeDaily / ScopeHourly 依次各多一层；空串写到运限的查询精度
	// （Horoscope.Precision）。只写到大限时大限段不列风云际会（它要读小限与流年才判得出）。
	// 只对 Horoscope.ToTextWith 起作用；ScopeOrigin、不是运限层级、比查询精度细的取值报 ErrInvalidArgument
	Through string
	// PatternConfig 为格局判定口径，作用于文本的格局节与格局释义；nil 取内核默认
	PatternConfig *PatternConfig
}

// apply 把全部选项写进查询入参；零值不加任何键。
func (o TextOptions) apply(payload map[string]any) {
	o.Knowledge.apply(payload)
	if o.Detail != "" {
		payload["detail"] = o.Detail
	}
	if o.KnowledgeDepth != "" {
		payload["knowledgeDepth"] = o.KnowledgeDepth
	}
	if o.Through != "" {
		payload["through"] = o.Through
	}
	if o.PatternConfig != nil {
		payload["patternConfig"] = o.PatternConfig
	}
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

// ForAstrolabe 按本命盘从本材料来源取材，返回只含盘上内容的子包：stars 为盘上出现的星
// （含四组十二神，同宫主星的组合解读保留）、patterns 为按 config 口径命中的格局（nil 取默认口径）、
// mutagens 为四化 4 条；palaces 与 concepts 为空，元信息沿用来源包。
// BuiltinKnowledge 在内核内直接读内嵌包；显式包整包发给内核，就地改过的条目照样进入子包。
// 零值 Knowledge 没有材料可取，报 ErrInvalidArgument。
// 子包仍是标准知识包，可再作 Merged 的底包或 KnowledgeFrom 的来源。
func (k Knowledge) ForAstrolabe(chart *Astrolabe, config *PatternConfig) (*KnowledgePack, error) {
	return k.ForAstrolabeContext(context.Background(), chart, config)
}

// ForAstrolabeContext 为 ForAstrolabe 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (k Knowledge) ForAstrolabeContext(ctx context.Context, chart *Astrolabe, config *PatternConfig) (*KnowledgePack, error) {
	if chart == nil {
		return nil, invalidArgument("knowledgeForChart: nil astrolabe")
	}
	return k.forChart(ctx, chart.textPayload("knowledgeForChart"), config)
}

// ForHoroscope 按运限取材：在 ForAstrolabe 的本命内容之上，另加各运限层的流耀与各层按 config 口径命中的格局，
// 只取到运限的查询精度（Horoscope.Precision）那一层。
func (k Knowledge) ForHoroscope(horoscope *Horoscope, config *PatternConfig) (*KnowledgePack, error) {
	return k.ForHoroscopeContext(context.Background(), horoscope, config)
}

// ForHoroscopeContext 为 ForHoroscope 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (k Knowledge) ForHoroscopeContext(ctx context.Context, horoscope *Horoscope, config *PatternConfig) (*KnowledgePack, error) {
	if horoscope == nil || horoscope.astrolabe == nil {
		return nil, invalidArgument("knowledgeForChart: horoscope must be created by Astrolabe.Horoscope")
	}
	payload := horoscope.astrolabe.textPayload("knowledgeForChart")
	horoscope.addTarget(payload)
	return k.forChart(ctx, payload, config)
}

// forChart 把材料来源与格局口径写进已定位到盘（或运限）的 knowledgeForChart 入参并查询内核。
func (k Knowledge) forChart(ctx context.Context, payload map[string]any, config *PatternConfig) (*KnowledgePack, error) {
	if !k.builtin && k.pack == nil {
		return nil, invalidArgument("knowledgeForChart: zero Knowledge has no pack to draw from")
	}
	k.apply(payload)
	if config != nil {
		payload["patternConfig"] = config
	}
	var out KnowledgePack
	if err := utilQueryContext(ctx, payload, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// ForAstrolabe 以本包为材料按本命盘取材，等同 KnowledgeFrom(p).ForAstrolabe；nil 包报 ErrInvalidArgument。
func (p *KnowledgePack) ForAstrolabe(chart *Astrolabe, config *PatternConfig) (*KnowledgePack, error) {
	return p.ForAstrolabeContext(context.Background(), chart, config)
}

// ForAstrolabeContext 为 ForAstrolabe 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (p *KnowledgePack) ForAstrolabeContext(ctx context.Context, chart *Astrolabe, config *PatternConfig) (*KnowledgePack, error) {
	if p == nil {
		return nil, invalidArgument("knowledgeForChart: nil knowledge pack")
	}
	return KnowledgeFrom(p).ForAstrolabeContext(ctx, chart, config)
}

// ForHoroscope 以本包为材料按运限取材，等同 KnowledgeFrom(p).ForHoroscope；nil 包报 ErrInvalidArgument。
func (p *KnowledgePack) ForHoroscope(horoscope *Horoscope, config *PatternConfig) (*KnowledgePack, error) {
	return p.ForHoroscopeContext(context.Background(), horoscope, config)
}

// ForHoroscopeContext 为 ForHoroscope 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (p *KnowledgePack) ForHoroscopeContext(ctx context.Context, horoscope *Horoscope, config *PatternConfig) (*KnowledgePack, error) {
	if p == nil {
		return nil, invalidArgument("knowledgeForChart: nil knowledge pack")
	}
	return KnowledgeFrom(p).ForHoroscopeContext(ctx, horoscope, config)
}

// 知识包校验问题的种类（KnowledgeIssue.Code）；级别与出处见文档站「知识包 · 校验」。
const (
	// IssueInvalidJSON 为文本不是合法 JSON
	IssueInvalidJSON = "invalid_json"
	// IssueNotObject 为包或条目不是 JSON 对象
	IssueNotObject = "not_object"
	// IssueSchemaMissing 为schema 缺失、为 null 或 0
	IssueSchemaMissing = "schema_missing"
	// IssueSchemaUnsupported 为schema 比支持的版本新
	IssueSchemaUnsupported = "schema_unsupported"
	// IssueWrongType 为字段类型与规范不符
	IssueWrongType = "wrong_type"
	// IssueRequiredMissing 为id / version / language 缺失或为空串
	IssueRequiredMissing = "required_missing"
	// IssueUnknownLanguage 为language 不是 x-iztro 语言码
	IssueUnknownLanguage = "unknown_language"
	// IssueUnknownKey 为条目键不是内核认识的标识
	IssueUnknownKey = "unknown_key"
	// IssueCombinationKeyNotMajor 为双星组合的键不是主星
	IssueCombinationKeyNotMajor = "combination_key_not_major"
	// IssueInvalidValue 为category / yinYang / fiveElements / stem 取值不在规定集合内
	IssueInvalidValue = "invalid_value"
	// IssueInvalidSlug 为术语或主题的 slug 含小写字母、数字、连字符以外的字符
	IssueInvalidSlug = "invalid_slug"
	// IssueUnknownPalace 为主题的宫位不认识
	IssueUnknownPalace = "unknown_palace"
	// IssueHTMLInText 为正文含 HTML 标签
	IssueHTMLInText = "html_in_text"
	// IssueImageInText 为正文含图片
	IssueImageInText = "image_in_text"
	// IssueExternalLink 为正文含站外链接
	IssueExternalLink = "external_link"
	// IssueUnknownField 为规范之外的字段，解析时被丢弃
	IssueUnknownField = "unknown_field"
	// IssueEmptyString 为字段为空字符串
	IssueEmptyString = "empty_string"
	// IssueSummaryMissing 为正文超过 240 宽却没有摘要
	IssueSummaryMissing = "summary_missing"
	// IssueSummaryNotNeeded 为正文不超过 240 宽却另写了摘要
	IssueSummaryNotNeeded = "summary_not_needed"
	// IssueSummaryTooWide 为摘要超过 100 宽
	IssueSummaryTooWide = "summary_too_wide"
	// IssueSummaryMultiParagraph 为摘要不是单段
	IssueSummaryMultiParagraph = "summary_multi_paragraph"
	// IssueSummaryWithoutBody 为摘要没有对应的正文
	IssueSummaryWithoutBody = "summary_without_body"
	// IssuePageReference 为正文依赖页面上下文
	IssuePageReference = "page_reference"
	// IssueTopicWithoutPalaces 为主题一个宫都不取
	IssueTopicWithoutPalaces = "topic_without_palaces"
	// IssueOverridesWithEmpty 为以空串覆盖底包里有值的字段
	IssueOverridesWithEmpty = "overrides_with_empty"
	// IssueLanguageDiffersFromBase 为语言与底包不同
	IssueLanguageDiffersFromBase = "language_differs_from_base"
	// IssueExtendsDiffersFromBase 为extends 与底包的 id 不同
	IssueExtendsDiffersFromBase = "extends_differs_from_base"
)

// KnowledgeIssue 为知识包校验的一条问题（CheckKnowledgePack）。
type KnowledgeIssue struct {
	// Level 为 "error"（违反规范的「必须 / 不得」或内核用到该处时会失败，构造知识包时拒绝）
	// 或 "warning"（「应 / 不应」、被丢弃的字段、启发式匹配与相对底包的提醒，只报告）
	Level string `json:"level"`
	// Path 为问题所在的 JSON 路径：对象键为 string、数组下标为 int；空切片指包本身
	Path IssuePath `json:"path"`
	// Code 为问题种类（Issue* 常量之一）
	Code string `json:"code"`
	// Params 为补充数据，如 {"key": "ziweiMjj", "suggestion": "ziweiMaj"}
	Params map[string]any `json:"params"`
	// Message 为带路径的英文说明，供日志；界面应按 Code 与 Params 组织文案
	Message string `json:"message"`
}

// IssuePath 为 JSON 路径，元素是 string（对象键）或 int（数组下标）。
type IssuePath []any

// UnmarshalJSON 把数组下标解成 int（encoding/json 默认解成 float64）。
func (p *IssuePath) UnmarshalJSON(data []byte) error {
	var raw []json.RawMessage
	if err := json.Unmarshal(data, &raw); err != nil {
		return err
	}
	out := make(IssuePath, 0, len(raw))
	for _, seg := range raw {
		var key string
		if err := json.Unmarshal(seg, &key); err == nil {
			out = append(out, key)
			continue
		}
		var index int
		if err := json.Unmarshal(seg, &index); err != nil {
			return fmt.Errorf("issue path segment %s is neither a key nor an index", seg)
		}
		out = append(out, index)
	}
	*p = out
	return nil
}

// CheckKnowledgePack 逐条校验一份包的 JSON 文本，一次返回全部问题（不在第一处停）；
// 文本不是合法 JSON、不是对象等也作为 error 级问题返回。base 非 nil 时另报相对底包的提醒
// （空串覆盖底包的值、语言或 extends 与底包不符），摘要规则按合并后的条目判断。
// 返回的 error 只表示调用失败（base 本身不合法、wasm 故障），不表示包有问题。
func CheckKnowledgePack(data []byte, base *KnowledgePack) ([]KnowledgeIssue, error) {
	return CheckKnowledgePackContext(context.Background(), data, base)
}

// CheckKnowledgePackContext 为 CheckKnowledgePack 的 Context 变体；ctx 用于取消等待 wasm 实例。
func CheckKnowledgePackContext(ctx context.Context, data []byte, base *KnowledgePack) ([]KnowledgeIssue, error) {
	payload := map[string]any{"kind": "checkKnowledgePack", "packJson": string(data)}
	if base != nil {
		payload["basePack"] = base
	}
	var out []KnowledgeIssue
	if err := utilQueryContext(ctx, payload, &out); err != nil {
		return nil, err
	}
	return out, nil
}

// ParseKnowledgePack 由 JSON 文本解析一份包。按 CheckKnowledgePack 的规则校验，有 error 级问题即返回
// ErrInvalidArgument 类错误（*Error，全部问题在其 Issues 字段）；结构体取自内核规范化后的包
// （规范外的字段丢弃、映射里的 null 略去、键区分大小写），与其他语言对同一输入得到同一份包。
func ParseKnowledgePack(data []byte) (*KnowledgePack, error) {
	return ParseKnowledgePackContext(context.Background(), data)
}

// ParseKnowledgePackContext 为 ParseKnowledgePack 的 Context 变体；ctx 用于取消等待 wasm 实例。
func ParseKnowledgePackContext(ctx context.Context, data []byte) (*KnowledgePack, error) {
	// 内核解析并返回规范化的包（规范外的字段已丢弃、映射里的 null 已略去、键已是规范写法），
	// 再由它落成结构体：直接用 encoding/json 解原文会按大小写不敏感收下 "Intro" 一类键，与其他语言不一致
	var result struct {
		Pack   json.RawMessage  `json:"pack"`
		Issues []KnowledgeIssue `json:"issues"`
	}
	if err := utilQueryContext(ctx, map[string]any{"kind": "parseKnowledgePack", "packJson": string(data)}, &result); err != nil {
		return nil, err
	}
	if string(result.Pack) == "null" {
		var messages []string
		for _, issue := range result.Issues {
			if issue.Level == "error" {
				messages = append(messages, issue.Message)
			}
		}
		return nil, &Error{
			Code:    CodeInvalidArgument,
			Message: "invalid knowledge pack: " + strings.Join(messages, "; "),
			Issues:  result.Issues,
		}
	}
	var out KnowledgePack
	if err := json.Unmarshal(result.Pack, &out); err != nil {
		return nil, internalError("decode normalized knowledge pack: " + err.Error())
	}
	return &out, nil
}

// Merged 以本包为底依次叠加覆盖包，返回新包（本包不变）。
// 合并规则见 knowledge/SCHEMA.md：覆盖包的非空字段覆盖同键条目的对应字段，
// attributes / combinations 逐字段合并，数组字段整体替换；覆盖条目换了正文却没给
// 对应摘要时，合并结果去掉描述旧正文的摘要。
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

// Topic 取主题条目，没有返回 nil。
func (p *KnowledgePack) Topic(slug string) *TopicEntry {
	if p == nil {
		return nil
	}
	if e, ok := p.Topics[slug]; ok {
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

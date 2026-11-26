package iztro

import "context"

// 按主题选宫：知识包主题的宫位定义落到盘上某一层，连同选中宫相关的三类四化。
// 本命层按本命宫名定位；运限层按该层重排的宫名定位，三方四正以该层的本宫起算；
// PalaceBody / PalaceOriginal 是本命盘上的标记，在任何一层都落在本命那一宫。
// 与主题文本（Astrolabe.TopicToText、Horoscope.TopicToText）同一套选宫。

// TopicGroup 为主题定义里的一组宫落到盘上的位置。宫位索引 0-11，寅宫为 0。
type TopicGroup struct {
	// Kind 为 "surround"（连同该层的三方四正）或 "only"（只取本宫）
	Kind string `json:"kind"`
	// Key 为主题定义里的宫位标识：宫名标识、PalaceBody 或 PalaceOriginal
	Key string `json:"key"`
	// Target 为本宫索引
	Target int `json:"target"`
	// Opposite 为对宫索引；only 组为 nil
	Opposite *int `json:"opposite"`
	// Wealth 为财帛位索引；only 组为 nil
	Wealth *int `json:"wealth"`
	// Career 为官禄位索引；only 组为 nil
	Career *int `json:"career"`
}

// Flight 为一宫宫干飞出的一化。
type Flight struct {
	// MutagenKey 为四化标识（MutagenLu 等常量）
	MutagenKey string `json:"mutagenKey"`
	// StarKey 为化出的星耀标识
	StarKey string `json:"starKey"`
	// PalaceIndex 为星所在的宫位索引；星不在盘上时为 nil
	PalaceIndex *int `json:"palaceIndex"`
	// SelfMutaged 为是否自化（落回本宫）
	SelfMutaged bool `json:"selfMutaged"`
}

// TopicPalace 为选中的一宫。
type TopicPalace struct {
	// Index 为宫位索引
	Index int `json:"index"`
	// Role 为角色："target" / "opposite" / "wealth" / "career" / "related"，取首次出现的那一组
	Role string `json:"role"`
	// Group 为首次出现的组在 TopicSelection.Groups 里的下标
	Group int `json:"group"`
	// NameKey 为所选层的宫名标识
	NameKey string `json:"nameKey"`
	// NatalNameKey 为本命宫名标识
	NatalNameKey string `json:"natalNameKey"`
	// StemKey 为宫干飞化所用天干的标识，取法见 TopicSelection.PalaceStemSource
	StemKey string `json:"stemKey"`
	// Flying 为宫干飞化，按禄、权、科、忌四条
	Flying []Flight `json:"flying"`
}

// MutagenLanding 为生年四化落在选中宫的一条。
type MutagenLanding struct {
	// MutagenKey 为四化标识
	MutagenKey string `json:"mutagenKey"`
	// StarKey 为化出的星耀标识
	StarKey string `json:"starKey"`
	// PalaceIndex 为星所在的宫位索引
	PalaceIndex int `json:"palaceIndex"`
}

// ScopeMutagenLanding 为运限某层的四化落在选中宫的一条。
type ScopeMutagenLanding struct {
	// Scope 为层级（ScopeDecadal 等常量）
	Scope string `json:"scope"`
	// NameKey 为层级名标识：大限层未起运时为 "childhood"
	NameKey string `json:"nameKey"`
	// StemKey 为化出这条四化的天干标识，取法见 TopicSelection.ScopeMutagenStemSource
	StemKey string `json:"stemKey"`
	// MutagenKey 为四化标识
	MutagenKey string `json:"mutagenKey"`
	// StarKey 为化出的星耀标识
	StarKey string `json:"starKey"`
	// PalaceIndex 为星所在的宫位索引
	PalaceIndex int `json:"palaceIndex"`
}

// TopicSelection 为一个主题在某一层选中的宫与相关四化。
type TopicSelection struct {
	// Topic 为主题 slug
	Topic string `json:"topic"`
	// Scope 为所选层；本命为 ScopeOrigin
	Scope string `json:"scope"`
	// Groups 为主题定义的各组，先 surround 后 palaces，各按定义顺序；来因宫不在盘上时不成组
	Groups []TopicGroup `json:"groups"`
	// Palaces 为选中的宫，每宫一次，即主题文本的成段顺序
	Palaces []TopicPalace `json:"palaces"`
	// PalaceStemSource 为宫干飞化所用天干的取法："natalPalace"（本命各宫宫干，运限各层沿用）
	PalaceStemSource string `json:"palaceStemSource"`
	// ScopeMutagenStemSource 为运限四化所用天干的取法："scope"（该层自身的干：大限宫干、流年年干、流月月干……）
	ScopeMutagenStemSource string `json:"scopeMutagenStemSource"`
	// BirthMutagens 为生年四化落在选中宫的，按禄、权、科、忌
	BirthMutagens []MutagenLanding `json:"birthMutagens"`
	// ScopeMutagens 为大限至所选层各层四化落在选中宫的，按层级再按禄、权、科、忌；本命层为空
	ScopeMutagens []ScopeMutagenLanding `json:"scopeMutagens"`
}

// TopicPalaces 返回主题在本命盘上选中的宫：各组位置、每宫的角色与宫干飞化，以及落在这些宫里的生年四化。
// 主题定义取自 knowledge，零值报 ErrInvalidArgument；包里没有该主题、主题里有不认识的宫位标识同样报
// ErrInvalidArgument。
func (a *Astrolabe) TopicPalaces(topic string, knowledge Knowledge) (*TopicSelection, error) {
	return a.TopicPalacesContext(context.Background(), topic, knowledge)
}

// TopicPalacesContext 为 TopicPalaces 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) TopicPalacesContext(ctx context.Context, topic string, knowledge Knowledge) (*TopicSelection, error) {
	if a == nil {
		return nil, invalidArgument("topicPalaces: nil astrolabe")
	}
	payload := a.textPayload("topicPalaces")
	payload["topic"] = topic
	knowledge.apply(payload)
	var out TopicSelection
	if err := utilQueryContext(ctx, payload, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// TopicPalaces 返回主题在 scope 层选中的宫：宫名按该层定位、三方四正以该层的本宫起算，每宫的角色与宫干飞化，
// 落在这些宫里的生年四化，以及大限至该层各层落在这些宫里的四化。scope 传 ScopeDecadal 等常量，
// ScopeOrigin 等同 Astrolabe.TopicPalaces；比查询精度（Precision）细时报 ErrInvalidArgument。
func (h *Horoscope) TopicPalaces(topic, scope string, knowledge Knowledge) (*TopicSelection, error) {
	return h.TopicPalacesContext(context.Background(), topic, scope, knowledge)
}

// TopicPalacesContext 为 TopicPalaces 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (h *Horoscope) TopicPalacesContext(ctx context.Context, topic, scope string, knowledge Knowledge) (*TopicSelection, error) {
	if h == nil || h.astrolabe == nil {
		return nil, invalidArgument("horoscopeTopicPalaces: horoscope must be created by Astrolabe.Horoscope")
	}
	payload := h.astrolabe.textPayload("horoscopeTopicPalaces")
	h.addTarget(payload)
	payload["topic"] = topic
	payload["scope"] = scope
	knowledge.apply(payload)
	var out TopicSelection
	if err := utilQueryContext(ctx, payload, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// TopicToText 生成主题在 scope 层的语义化文本：选宫同 TopicPalaces。标题写运限时间与命盘，标题下逐层写
// 大限至该层的命宫位置与四化；每宫段落写该层宫名并补本命宫名，宫内星曜照本命，另写各层落在本宫的四化与流耀；
// 格局取该层视角在选中宫内的命中。ScopeOrigin 与 Astrolabe.TopicToText 逐字相同。
// 主题定义与释义取自 opts.Knowledge，零值报 ErrInvalidArgument；opts.Through 不影响输出，
// 但取值照样校验：ScopeOrigin 或不是层级标识报 ErrInvalidArgument。
func (h *Horoscope) TopicToText(topic, scope string, opts TextOptions) (string, error) {
	return h.TopicToTextContext(context.Background(), topic, scope, opts)
}

// TopicToTextContext 为 TopicToText 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (h *Horoscope) TopicToTextContext(ctx context.Context, topic, scope string, opts TextOptions) (string, error) {
	if h == nil || h.astrolabe == nil {
		return "", invalidArgument("horoscopeTopicToText: horoscope must be created by Astrolabe.Horoscope")
	}
	payload := h.astrolabe.textPayload("horoscopeTopicToText")
	h.addTarget(payload)
	payload["topic"] = topic
	payload["scope"] = scope
	opts.apply(payload)
	var out string
	return out, utilQueryContext(ctx, payload, &out)
}

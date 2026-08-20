package iztro

// 本文件定义星盘与运限的类型化数据结构。
// 每个对象携带两层信息：
//   - 翻译字段（Name、Brightness 等）：按排盘语言本地化的展示文本；
//   - 标识字段（Key、NameKey 等）：语言无关的 iztro i18n key（见 keys.go 常量）。
// 判断方法基于标识字段比较，传入 keys.go 的常量即可在任何输出语言的
// 星盘上正确工作；接受星耀/宫位参数的方法同时兼容当前语言的翻译名。

// Star 为一颗星耀。
type Star struct {
	// Key 为语言无关星耀标识（如 StarZiwei）
	Key string `json:"key"`
	// Name 为按排盘语言翻译的星耀名称
	Name string `json:"name"`
	// Type 为星耀类型（major/soft/tough/adjective/flower/helper/lucun/tianma）
	Type string `json:"type"`
	// Scope 为作用范围（origin/decadal/yearly/monthly/daily/hourly）
	Scope string `json:"scope"`
	// Brightness 为亮度显示文本；无亮度为空串（杂耀与流耀无此字段）
	Brightness string `json:"brightness,omitempty"`
	// BrightnessKey 为语言无关亮度标识（如 BrightnessMiao），无亮度为空串
	BrightnessKey string `json:"brightnessKey,omitempty"`
	// Mutagen 为四化显示文本；无四化为空串
	Mutagen string `json:"mutagen,omitempty"`
	// MutagenKey 为语言无关四化标识（如 MutagenLu），无四化为空串
	MutagenKey string `json:"mutagenKey,omitempty"`

	// palace、astrolabe 为反向引用，由星盘解析后回填，不参与序列化。
	palace    *Palace
	astrolabe *Astrolabe
}

// WithMutagen 判断星耀是否具有列出的任一四化（传 MutagenLu 等常量）。
func (s *Star) WithMutagen(mutagenKeys ...string) bool {
	if s == nil || s.MutagenKey == "" {
		return false
	}
	for _, k := range mutagenKeys {
		if s.MutagenKey == k {
			return true
		}
	}
	return false
}

// WithBrightness 判断星耀是否处于列出的任一亮度（传 BrightnessMiao 等常量）。
func (s *Star) WithBrightness(brightnessKeys ...string) bool {
	if s == nil || s.BrightnessKey == "" {
		return false
	}
	for _, k := range brightnessKeys {
		if s.BrightnessKey == k {
			return true
		}
	}
	return false
}

// Palace 返回星耀所在的宫位；脱离星盘单独构造的星耀返回 nil。
func (s *Star) Palace() *Palace {
	if s == nil {
		return nil
	}
	return s.palace
}

// OppositePalace 返回星耀所在宫位的对宫。
func (s *Star) OppositePalace() *Palace {
	if s == nil || s.palace == nil || s.astrolabe == nil {
		return nil
	}
	return s.astrolabe.PalaceByIndex((s.palace.Index + 6) % 12)
}

// SurroundedPalaces 返回星耀所在宫位的三方四正。
func (s *Star) SurroundedPalaces() *SurroundedPalaces {
	if s == nil || s.palace == nil || s.astrolabe == nil {
		return nil
	}
	return s.astrolabe.SurroundedPalacesByIndex(s.palace.Index)
}

// Decadal 为宫位的大限区间与干支。
type Decadal struct {
	// Range 为大限起止年龄 [起始, 截止]
	Range [2]int `json:"range"`
	// HeavenlyStem 为大限天干（翻译文本）
	HeavenlyStem string `json:"heavenlyStem"`
	// HeavenlyStemKey 为大限天干标识（如 StemJia）
	HeavenlyStemKey string `json:"heavenlyStemKey"`
	// EarthlyBranch 为大限地支（翻译文本）
	EarthlyBranch string `json:"earthlyBranch"`
	// EarthlyBranchKey 为大限地支标识（如 BranchZi）
	EarthlyBranchKey string `json:"earthlyBranchKey"`
}

// Palace 为一个宫位。
type Palace struct {
	// Index 为宫位索引（0-11，寅宫为 0）
	Index int `json:"index"`
	// Name 为宫位名称（翻译文本）
	Name string `json:"name"`
	// NameKey 为语言无关宫位标识（如 PalaceSoul）
	NameKey string `json:"nameKey"`
	// IsBodyPalace 表示是否身宫
	IsBodyPalace bool `json:"isBodyPalace"`
	// IsOriginalPalace 表示是否来因宫
	IsOriginalPalace bool `json:"isOriginalPalace"`
	// HeavenlyStem 为宫位天干（翻译文本）
	HeavenlyStem string `json:"heavenlyStem"`
	// HeavenlyStemKey 为宫位天干标识
	HeavenlyStemKey string `json:"heavenlyStemKey"`
	// EarthlyBranch 为宫位地支（翻译文本）
	EarthlyBranch string `json:"earthlyBranch"`
	// EarthlyBranchKey 为宫位地支标识
	EarthlyBranchKey string `json:"earthlyBranchKey"`
	// MajorStars 为主星列表
	MajorStars []Star `json:"majorStars"`
	// MinorStars 为辅星列表
	MinorStars []Star `json:"minorStars"`
	// AdjectiveStars 为杂耀列表
	AdjectiveStars []Star `json:"adjectiveStars"`
	// Changsheng12 为长生十二神（翻译文本）
	Changsheng12 string `json:"changsheng12"`
	// Changsheng12Key 为长生十二神标识
	Changsheng12Key string `json:"changsheng12Key"`
	// Boshi12 为博士十二神（翻译文本）
	Boshi12 string `json:"boshi12"`
	// Boshi12Key 为博士十二神标识
	Boshi12Key string `json:"boshi12Key"`
	// Jiangqian12 为将前十二神（翻译文本）
	Jiangqian12 string `json:"jiangqian12"`
	// Jiangqian12Key 为将前十二神标识
	Jiangqian12Key string `json:"jiangqian12Key"`
	// Suiqian12 为岁前十二神（翻译文本）
	Suiqian12 string `json:"suiqian12"`
	// Suiqian12Key 为岁前十二神标识
	Suiqian12Key string `json:"suiqian12Key"`
	// Decadal 为大限信息
	Decadal Decadal `json:"decadal"`
	// Ages 为小限经过年龄
	Ages []int `json:"ages"`
	// MutagenStarKeys 为本宫天干化出的四颗星标识，顺序为禄、权、科、忌。
	// 取值随排盘配置的自定义四化表变化，飞星判断以它为准。
	MutagenStarKeys [4]string `json:"mutagenStarKeys"`

	// astrolabe 为反向引用，由星盘解析后回填，不参与序列化。
	astrolabe *Astrolabe
	// starIDs 为宫内星耀的 key 与译名集合，由 link 预计算，供包含判断复用。
	starIDs map[string]struct{}
}

// starIdentifiers 取宫内所有星耀的 key 与翻译名，供查询词匹配。
// 挂在星盘上的宫位用 link 预计算好的集合；单独构造的宫位现算，不写回缓存。
func (p *Palace) starIdentifiers() map[string]struct{} {
	if p == nil {
		return nil
	}
	if p.starIDs != nil {
		return p.starIDs
	}
	return buildStarIdentifiers(p)
}

// buildStarIdentifiers 汇集宫内所有星耀的 key 与翻译名。
func buildStarIdentifiers(p *Palace) map[string]struct{} {
	out := make(map[string]struct{})
	for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
		for _, s := range group {
			out[s.Key] = struct{}{}
			out[s.Name] = struct{}{}
		}
	}
	return out
}

// Has 判断宫位是否包含指定的所有星耀（接受 keys.go 常量或当前语言的星名）。
func (p *Palace) Has(stars ...string) bool {
	ids := p.starIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HasOneOf 判断宫位是否包含指定星耀中的至少一颗。
func (p *Palace) HasOneOf(stars ...string) bool {
	ids := p.starIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return true
		}
	}
	return false
}

// NotHave 判断宫位是否不包含指定的所有星耀。
func (p *Palace) NotHave(stars ...string) bool {
	ids := p.starIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return false
		}
	}
	return true
}

// HasMutagen 判断宫位是否有指定四化（只检查主星和辅星；传 MutagenLu 等常量）。
func (p *Palace) HasMutagen(mutagenKey string) bool {
	if p == nil {
		return false
	}
	for _, group := range [][]Star{p.MajorStars, p.MinorStars} {
		for _, s := range group {
			if s.MutagenKey == mutagenKey {
				return true
			}
		}
	}
	return false
}

// NotHaveMutagen 判断宫位是否没有指定四化。
func (p *Palace) NotHaveMutagen(mutagenKey string) bool {
	return !p.HasMutagen(mutagenKey)
}

// IsEmpty 判断宫位是否为空宫（无十四主星）。
//
// excludeStars 中任一星耀在宫内时不作空宫论，用于「借星」判断。
func (p *Palace) IsEmpty(excludeStars ...string) bool {
	if p == nil {
		return true
	}
	if len(p.MajorStars) > 0 {
		return false
	}
	if len(excludeStars) > 0 && p.HasOneOf(excludeStars...) {
		return false
	}
	return true
}

// Astrolabe 返回宫位所属星盘；脱离星盘单独构造的宫位返回 nil。
func (p *Palace) Astrolabe() *Astrolabe {
	if p == nil {
		return nil
	}
	return p.astrolabe
}

// OppositePalace 返回本宫的对宫，即索引 +6 的那一宫；
// 脱离星盘单独构造的宫位返回 nil。
//
// 对宫与本宫永远相对而看：命宫对迁移、财帛对福德，依此类推。
func (p *Palace) OppositePalace() *Palace {
	if p == nil || p.astrolabe == nil {
		return nil
	}
	return p.astrolabe.PalaceByIndex((p.Index + 6) % 12)
}

// SurroundedPalaces 返回本宫的三方四正；脱离星盘单独构造的宫位返回 nil。
func (p *Palace) SurroundedPalaces() *SurroundedPalaces {
	if p == nil || p.astrolabe == nil {
		return nil
	}
	return p.astrolabe.SurroundedPalacesByIndex(p.Index)
}

// MutagenStars 返回本宫天干在指定四化位上对应的星耀标识，顺序与传入一致。
func (p *Palace) MutagenStars(mutagenKeys ...string) []string {
	if p == nil {
		return nil
	}
	out := make([]string, 0, len(mutagenKeys))
	for _, k := range mutagenKeys {
		idx, ok := mutagenIndex[k]
		if !ok || p.MutagenStarKeys[idx] == "" {
			continue
		}
		out = append(out, p.MutagenStarKeys[idx])
	}
	return out
}

// FliesTo 判断本宫天干的指定四化星是否全部落在目标宫；四化为空时返回 false。
func (p *Palace) FliesTo(to *Palace, mutagenKeys ...string) bool {
	if to == nil {
		return false
	}
	stars := p.MutagenStars(mutagenKeys...)
	if len(stars) == 0 {
		return false
	}
	return to.Has(stars...)
}

// FliesOneOfTo 判断本宫天干的指定四化星是否有任一颗落在目标宫；四化为空时返回 true。
func (p *Palace) FliesOneOfTo(to *Palace, mutagenKeys ...string) bool {
	if to == nil {
		return false
	}
	stars := p.MutagenStars(mutagenKeys...)
	if len(stars) == 0 {
		return true
	}
	return to.HasOneOf(stars...)
}

// NotFlyTo 判断本宫天干的指定四化星是否一颗都不落在目标宫；四化为空时返回 true。
func (p *Palace) NotFlyTo(to *Palace, mutagenKeys ...string) bool {
	if to == nil {
		return false
	}
	stars := p.MutagenStars(mutagenKeys...)
	if len(stars) == 0 {
		return true
	}
	return to.NotHave(stars...)
}

// SelfMutaged 判断本宫天干的指定四化星是否全部落在本宫（自化）。
func (p *Palace) SelfMutaged(mutagenKeys ...string) bool {
	return p.Has(p.MutagenStars(mutagenKeys...)...)
}

// SelfMutagedOneOf 判断本宫是否有指定四化中任一种自化；不传表示检查全部四化。
func (p *Palace) SelfMutagedOneOf(mutagenKeys ...string) bool {
	return p.HasOneOf(p.MutagenStars(orAllMutagens(mutagenKeys)...)...)
}

// NotSelfMutaged 判断本宫是否没有指定四化中的任何自化；不传表示检查全部四化。
func (p *Palace) NotSelfMutaged(mutagenKeys ...string) bool {
	return p.NotHave(p.MutagenStars(orAllMutagens(mutagenKeys)...)...)
}

// MutagedPlaces 返回本宫四化分别飞入的宫位，顺序为禄、权、科、忌；
// 未找到的项为 nil。脱离星盘的宫位返回 nil。
func (p *Palace) MutagedPlaces() []*Palace {
	if p == nil || p.astrolabe == nil {
		return nil
	}
	stars := p.MutagenStars(allMutagens[:]...)
	out := make([]*Palace, 0, len(stars))
	for _, star := range stars {
		var found *Palace
		for i := range p.astrolabe.Palaces {
			if p.astrolabe.Palaces[i].Has(star) {
				found = &p.astrolabe.Palaces[i]
				break
			}
		}
		out = append(out, found)
	}
	return out
}

// RawLunarDate 为数字化农历生日。
type RawLunarDate struct {
	// LunarYear 为农历年
	LunarYear int `json:"lunarYear"`
	// LunarMonth 为农历月（1-12，闰月与否见 IsLeap）
	LunarMonth int `json:"lunarMonth"`
	// LunarDay 为农历日（1-30）
	LunarDay int `json:"lunarDay"`
	// IsLeap 表示是否闰月
	IsLeap bool `json:"isLeap"`
}

// RawChineseDate 为四柱干支（每柱 [天干, 地支]，未本地化的干支原文）。
type RawChineseDate struct {
	// Yearly 为年柱
	Yearly [2]string `json:"yearly"`
	// Monthly 为月柱
	Monthly [2]string `json:"monthly"`
	// Daily 为日柱
	Daily [2]string `json:"daily"`
	// Hourly 为时柱
	Hourly [2]string `json:"hourly"`
	// YearlyKeys 为年柱的语言无关标识（Stem* / Branch* 常量）
	YearlyKeys [2]string `json:"yearlyKeys"`
	// MonthlyKeys 为月柱的语言无关标识
	MonthlyKeys [2]string `json:"monthlyKeys"`
	// DailyKeys 为日柱的语言无关标识
	DailyKeys [2]string `json:"dailyKeys"`
	// HourlyKeys 为时柱的语言无关标识
	HourlyKeys [2]string `json:"hourlyKeys"`
}

// PillarKeys 返回四柱标识 [年, 月, 日, 时]，可直接交给 TranslateChineseDate。
func (d RawChineseDate) PillarKeys() [4][2]string {
	return [4][2]string{d.YearlyKeys, d.MonthlyKeys, d.DailyKeys, d.HourlyKeys}
}

// RawDates 为结构化的出生日期信息。
type RawDates struct {
	// LunarDate 为数字化农历生日
	LunarDate RawLunarDate `json:"lunarDate"`
	// ChineseDate 为四柱干支
	ChineseDate RawChineseDate `json:"chineseDate"`
}

// Astrolabe 为完整星盘。
type Astrolabe struct {
	// Gender 为性别（翻译文本）
	Gender string `json:"gender"`
	// GenderKey 为机器可读性别（GenderMale / GenderFemale）
	GenderKey Gender `json:"genderKey"`
	// SolarDate 为阳历日期
	SolarDate string `json:"solarDate"`
	// LunarDate 为农历日期
	LunarDate string `json:"lunarDate"`
	// ChineseDate 为干支纪年日期
	ChineseDate string `json:"chineseDate"`
	// RawDates 为结构化的农历生日与四柱干支
	RawDates RawDates `json:"rawDates"`
	// Time 为时辰
	Time string `json:"time"`
	// TimeRange 为时辰对应的时间段
	TimeRange string `json:"timeRange"`
	// Sign 为星座
	Sign string `json:"sign"`
	// Zodiac 为生肖
	Zodiac string `json:"zodiac"`
	// EarthlyBranchOfSoulPalace 为命宫地支（翻译文本）
	EarthlyBranchOfSoulPalace string `json:"earthlyBranchOfSoulPalace"`
	// EarthlyBranchOfSoulPalaceKey 为命宫地支标识
	EarthlyBranchOfSoulPalaceKey string `json:"earthlyBranchOfSoulPalaceKey"`
	// EarthlyBranchOfBodyPalace 为身宫地支（翻译文本）
	EarthlyBranchOfBodyPalace string `json:"earthlyBranchOfBodyPalace"`
	// EarthlyBranchOfBodyPalaceKey 为身宫地支标识
	EarthlyBranchOfBodyPalaceKey string `json:"earthlyBranchOfBodyPalaceKey"`
	// Soul 为命主星（翻译文本）
	Soul string `json:"soul"`
	// SoulKey 为命主星标识
	SoulKey string `json:"soulKey"`
	// Body 为身主星（翻译文本）
	Body string `json:"body"`
	// BodyKey 为身主星标识
	BodyKey string `json:"bodyKey"`
	// FiveElementsClass 为五行局（翻译文本）
	FiveElementsClass string `json:"fiveElementsClass"`
	// FiveElementsClassKey 为五行局标识（"water2nd" 等）
	FiveElementsClassKey string `json:"fiveElementsClassKey"`
	// Palaces 为十二宫数据
	Palaces []Palace `json:"palaces"`
	// TimeIndex 为出生时辰索引（0-12）
	TimeIndex uint8 `json:"timeIndex"`
	// FixLeap 表示是否修正闰月
	FixLeap bool `json:"fixLeap"`
	// Language 为盘面语言（Language* 常量）
	Language Language `json:"language"`
	// Config 为排盘配置
	Config Config `json:"config"`

	// reqConfig 为发起排盘时传入的原始配置，不参与序列化。
	reqConfig *Config
	// fromStem / fromBranch 为 Rearranged 的重排起点干支标识，不参与序列化；
	// 空串即本盘不是重排盘。wasm 侧调用无状态，格局判定等再次发起计算的
	// 接口必须带上它们，让内核先重排再判——否则判的是原盘。
	fromStem   string
	fromBranch string
}

// requestConfig 返回发起排盘时传入的原始配置，供运限、重排与 prompt 等
// 后续调用复用。序列化回来的 Config 只含六个标量键，自定义四化表与亮度表
// 不在其中，拿它当入参会静默丢掉覆盖表。
func (a *Astrolabe) requestConfig() *Config {
	if a == nil {
		return nil
	}
	if a.reqConfig != nil {
		return a.reqConfig
	}
	return &a.Config
}

// addRearrange 把本盘的重排起点干支写进查询入参；本盘不是重排盘时不加键。
func (a *Astrolabe) addRearrange(payload map[string]any) {
	if a.fromStem != "" && a.fromBranch != "" {
		payload["fromStem"] = a.fromStem
		payload["fromBranch"] = a.fromBranch
	}
}

// Palace 通过宫位标识（PalaceSoul 等常量）或当前语言宫名获取宫位；未找到返回 nil。
//
// 另外接受两个特殊标识：PalaceBody 定位身宫，PalaceOriginal 定位来因宫。
func (a *Astrolabe) Palace(nameKeyOrName string) *Palace {
	if a == nil {
		return nil
	}
	switch nameKeyOrName {
	case PalaceBody:
		for i := range a.Palaces {
			if a.Palaces[i].IsBodyPalace {
				return &a.Palaces[i]
			}
		}
		return nil
	case PalaceOriginal:
		for i := range a.Palaces {
			if a.Palaces[i].IsOriginalPalace {
				return &a.Palaces[i]
			}
		}
		return nil
	}

	for i := range a.Palaces {
		p := &a.Palaces[i]
		if p.NameKey == nameKeyOrName || p.Name == nameKeyOrName {
			return p
		}
	}
	return nil
}

// PalaceByIndex 通过宫位索引（0-11）获取宫位；越界返回 nil。
func (a *Astrolabe) PalaceByIndex(index int) *Palace {
	if a == nil || index < 0 || index >= len(a.Palaces) {
		return nil
	}
	return &a.Palaces[index]
}

// Star 通过星耀标识（StarZiwei 等常量）或当前语言星名查找星耀及其所在宫位；
// 未找到返回 (nil, nil)。
func (a *Astrolabe) Star(keyOrName string) (*Star, *Palace) {
	if a == nil {
		return nil, nil
	}
	for i := range a.Palaces {
		p := &a.Palaces[i]
		for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
			for j := range group {
				if group[j].Key == keyOrName || group[j].Name == keyOrName {
					return &group[j], p
				}
			}
		}
	}
	return nil, nil
}

// SurroundedPalaces 为三方四正宫位。
type SurroundedPalaces struct {
	// Target 为本宫
	Target *Palace
	// Opposite 为对宫
	Opposite *Palace
	// Wealth 为财帛位（三方）
	Wealth *Palace
	// Career 为官禄位（三方）
	Career *Palace
}

// SurroundedPalacesByIndex 获取指定宫位索引的三方四正；索引对 12 取模，
// 因此负数索引也能正确回绕。星盘不足十二宫（零值星盘）时返回 nil。
func (a *Astrolabe) SurroundedPalacesByIndex(index int) *SurroundedPalaces {
	if a == nil || len(a.Palaces) < 12 {
		return nil
	}
	i := ((index % 12) + 12) % 12
	return &SurroundedPalaces{
		Target:   &a.Palaces[i],
		Opposite: &a.Palaces[(i+6)%12],
		Wealth:   &a.Palaces[(i+8)%12],
		Career:   &a.Palaces[(i+4)%12],
	}
}

// SurroundedPalaces 获取指定宫位的三方四正，接受宫位标识、当前语言宫名，
// 以及 PalaceBody / PalaceOriginal；宫位定位不到返回 nil。
func (a *Astrolabe) SurroundedPalaces(nameKeyOrName string) *SurroundedPalaces {
	p := a.Palace(nameKeyOrName)
	if p == nil {
		return nil
	}
	return a.SurroundedPalacesByIndex(p.Index)
}

// IsSurrounded 判断指定宫位的三方四正是否包含全部指定星耀。
func (a *Astrolabe) IsSurrounded(nameKeyOrName string, stars ...string) bool {
	sp := a.SurroundedPalaces(nameKeyOrName)
	return sp != nil && sp.Have(stars...)
}

// IsSurroundedOneOf 判断指定宫位的三方四正是否包含指定星耀中的任意一颗。
func (a *Astrolabe) IsSurroundedOneOf(nameKeyOrName string, stars ...string) bool {
	sp := a.SurroundedPalaces(nameKeyOrName)
	return sp != nil && sp.HaveOneOf(stars...)
}

// NotSurrounded 判断指定宫位的三方四正是否一颗都不包含指定星耀。
func (a *Astrolabe) NotSurrounded(nameKeyOrName string, stars ...string) bool {
	sp := a.SurroundedPalaces(nameKeyOrName)
	return sp != nil && sp.NotHave(stars...)
}

// link 回填宫位与星耀的反向引用，使 Palace.Astrolabe()、Star.Palace()、
// 飞星与三方四正等关系查询可用。JSON 反序列化后必须调用一次。
func (a *Astrolabe) link() {
	for i := range a.Palaces {
		p := &a.Palaces[i]
		p.astrolabe = a
		p.starIDs = buildStarIdentifiers(p)
		for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
			for j := range group {
				group[j].palace = p
				group[j].astrolabe = a
			}
		}
	}
}

// Have 判断三方四正是否包含指定的所有星耀。
func (sp *SurroundedPalaces) Have(stars ...string) bool {
	ids := sp.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HaveOneOf 判断三方四正是否包含指定星耀中的至少一颗。
func (sp *SurroundedPalaces) HaveOneOf(stars ...string) bool {
	ids := sp.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return true
		}
	}
	return false
}

// NotHave 判断三方四正是否一颗都不包含指定星耀。
func (sp *SurroundedPalaces) NotHave(stars ...string) bool {
	ids := sp.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return false
		}
	}
	return true
}

// allStarIdentifiers 汇集四宫内所有星耀的 key 与翻译名。
func (sp *SurroundedPalaces) allStarIdentifiers() map[string]struct{} {
	ids := make(map[string]struct{})
	if sp == nil {
		return ids
	}
	for _, p := range []*Palace{sp.Target, sp.Opposite, sp.Wealth, sp.Career} {
		for k := range p.starIdentifiers() {
			ids[k] = struct{}{}
		}
	}
	return ids
}

// HaveMutagen 判断三方四正中是否有指定四化。
func (sp *SurroundedPalaces) HaveMutagen(mutagenKey string) bool {
	if sp == nil {
		return false
	}
	for _, p := range []*Palace{sp.Target, sp.Opposite, sp.Wealth, sp.Career} {
		if p.HasMutagen(mutagenKey) {
			return true
		}
	}
	return false
}

// NotHaveMutagen 判断三方四正中是否没有指定四化。
func (sp *SurroundedPalaces) NotHaveMutagen(mutagenKey string) bool {
	return !sp.HaveMutagen(mutagenKey)
}

// HoroscopeScope 为运限的单个层级（大限/小限/流年/流月/流日/流时）。
type HoroscopeScope struct {
	// Index 为该运限所在宫位索引（0-11）
	Index int `json:"index"`
	// Name 为层级显示名（大限/童限/小限/流年/流月/流日/流时，翻译文本）
	Name string `json:"name"`
	// HeavenlyStem 为该运限天干（翻译文本）
	HeavenlyStem string `json:"heavenlyStem"`
	// HeavenlyStemKey 为该运限天干标识
	HeavenlyStemKey string `json:"heavenlyStemKey"`
	// EarthlyBranch 为该运限地支（翻译文本）
	EarthlyBranch string `json:"earthlyBranch"`
	// EarthlyBranchKey 为该运限地支标识
	EarthlyBranchKey string `json:"earthlyBranchKey"`
	// PalaceNames 为该运限的十二宫名（翻译文本，按宫位索引排列）
	PalaceNames []string `json:"palaceNames"`
	// PalaceNameKeys 为该运限的十二宫标识
	PalaceNameKeys []string `json:"palaceNameKeys"`
	// Mutagen 为四化星名 [禄, 权, 科, 忌]（翻译文本）
	Mutagen []string `json:"mutagen"`
	// MutagenKeys 为四化星标识 [禄, 权, 科, 忌]
	MutagenKeys []string `json:"mutagenKeys"`
	// Stars 为流耀在十二宫的分布（无流耀的层级为 nil）
	Stars [][]Star `json:"stars,omitempty"`
	// NominalAge 为虚岁（仅小限层级有值）
	NominalAge int `json:"nominalAge,omitempty"`
	// YearlyDecStar 为流年十二神（仅流年层级有值）
	YearlyDecStar *YearlyDecStar `json:"yearlyDecStar,omitempty"`
}

// PalaceIndexByName 在该运限层级下按宫位标识（PalaceSoul 等常量）或当前语言
// 宫名查宫位索引；找不到返回 -1。
//
// 运限层级会把宫名重排（该层级所在位置视为命宫），因此同一格在不同层级下宫名不同。
func (item *HoroscopeScope) PalaceIndexByName(nameKeyOrName string) int {
	if item == nil {
		return -1
	}
	for i, key := range item.PalaceNameKeys {
		if key == nameKeyOrName {
			return i
		}
	}
	for i, name := range item.PalaceNames {
		if name == nameKeyOrName {
			return i
		}
	}
	return -1
}

// YearlyDecStar 为流年十二神（按目标年支排布，索引即宫位索引）。
type YearlyDecStar struct {
	// Suiqian12 为岁前十二神（翻译文本）
	Suiqian12 []string `json:"suiqian12"`
	// Suiqian12Keys 为岁前十二神标识
	Suiqian12Keys []string `json:"suiqian12Keys"`
	// Jiangqian12 为将前十二神（翻译文本）
	Jiangqian12 []string `json:"jiangqian12"`
	// Jiangqian12Keys 为将前十二神标识
	Jiangqian12Keys []string `json:"jiangqian12Keys"`
}

// Horoscope 为一次运限查询的完整结果。
type Horoscope struct {
	// LunarDate 为目标农历日期
	LunarDate string `json:"lunarDate"`
	// SolarDate 为目标阳历日期
	SolarDate string `json:"solarDate"`
	// Decadal 为大限（未起运时为童限）
	Decadal HoroscopeScope `json:"decadal"`
	// Age 为小限
	Age HoroscopeScope `json:"age"`
	// Yearly 为流年
	Yearly HoroscopeScope `json:"yearly"`
	// Monthly 为流月
	Monthly HoroscopeScope `json:"monthly"`
	// Daily 为流日
	Daily HoroscopeScope `json:"daily"`
	// Hourly 为流时
	Hourly HoroscopeScope `json:"hourly"`

	// astrolabe 为发起这次运限查询的本命盘。
	// 运限依附于某一张盘，宫位与流耀查询都要落回盘上，因此运限随身携带它。
	astrolabe *Astrolabe
	// targetTimeIndex 为发起这次查询的目标时辰索引；再次发起运限层计算（如格局）时用。
	targetTimeIndex uint8
}

// Astrolabe 返回发起这次运限查询的本命盘。
func (h *Horoscope) Astrolabe() *Astrolabe {
	if h == nil {
		return nil
	}
	return h.astrolabe
}

// ScopeItem 按层级取运限项；scope 传 ScopeDecadal 等常量，取不到返回 nil。
// ScopeOrigin 不属于运限层级，返回 nil。
func (h *Horoscope) ScopeItem(scope string) *HoroscopeScope {
	if h == nil {
		return nil
	}
	switch scope {
	case ScopeDecadal:
		return &h.Decadal
	case ScopeYearly:
		return &h.Yearly
	case ScopeMonthly:
		return &h.Monthly
	case ScopeDaily:
		return &h.Daily
	case ScopeHourly:
		return &h.Hourly
	}
	return nil
}

// AgePalace 返回小限所在的宫位。
func (h *Horoscope) AgePalace() *Palace {
	if h == nil || h.astrolabe == nil {
		return nil
	}
	return h.astrolabe.PalaceByIndex(h.Age.Index)
}

// Palace 取指定运限层级下某宫名对应的宫位。
//
// 运限层级会把宫名重排（该层级所在位置视为命宫），因此同一格在不同层级下宫名不同。
// scope 传 ScopeOrigin 时直接按本命盘宫名查找。
func (h *Horoscope) Palace(nameKeyOrName string, scope string) *Palace {
	if h == nil {
		return nil
	}
	a := h.astrolabe
	if a == nil {
		return nil
	}
	if scope == ScopeOrigin {
		return a.Palace(nameKeyOrName)
	}
	idx := h.ScopeItem(scope).PalaceIndexByName(nameKeyOrName)
	if idx < 0 {
		return nil
	}
	return a.PalaceByIndex(idx)
}

// SurroundPalaces 取指定运限层级下某宫的三方四正。
func (h *Horoscope) SurroundPalaces(nameKeyOrName string, scope string) *SurroundedPalaces {
	p := h.Palace(nameKeyOrName, scope)
	if p == nil {
		return nil
	}
	return h.astrolabe.SurroundedPalacesByIndex(p.Index)
}

// HasHoroscopeMutagen 判断指定运限层级下某宫是否落有该层级的四化星。
// scope 为 ScopeOrigin 时恒为 false（本命四化请用 Palace.HasMutagen）。
func (h *Horoscope) HasHoroscopeMutagen(nameKeyOrName string, scope string, mutagenKey string) bool {
	if scope == ScopeOrigin {
		return false
	}
	item := h.ScopeItem(scope)
	if item == nil {
		return false
	}
	idx, ok := mutagenIndex[mutagenKey]
	if !ok || idx >= len(item.MutagenKeys) {
		return false
	}
	p := h.Palace(nameKeyOrName, scope)
	if p == nil {
		return false
	}
	starKey := item.MutagenKeys[idx]
	for _, group := range [][]Star{p.MajorStars, p.MinorStars} {
		for _, s := range group {
			if s.Key == starKey {
				return true
			}
		}
	}
	return false
}

// horoscopeStarIdentifiers 汇集指定宫位上大限与流年两层流耀的 key 与翻译名。
func (h *Horoscope) horoscopeStarIdentifiers(palaceIndex int) map[string]struct{} {
	ids := make(map[string]struct{})
	if h == nil {
		return ids
	}
	for _, layer := range []*HoroscopeScope{&h.Decadal, &h.Yearly} {
		if palaceIndex < len(layer.Stars) {
			for _, s := range layer.Stars[palaceIndex] {
				ids[s.Key] = struct{}{}
				ids[s.Name] = struct{}{}
			}
		}
	}
	return ids
}

// HasHoroscopeStars 判断指定运限宫位是否包含全部指定流耀。
func (h *Horoscope) HasHoroscopeStars(nameKeyOrName string, scope string, stars []string) bool {
	p := h.Palace(nameKeyOrName, scope)
	if p == nil {
		return false
	}
	ids := h.horoscopeStarIdentifiers(p.Index)
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HasOneOfHoroscopeStars 判断指定运限宫位是否包含指定流耀中的任意一颗。
func (h *Horoscope) HasOneOfHoroscopeStars(nameKeyOrName string, scope string, stars []string) bool {
	p := h.Palace(nameKeyOrName, scope)
	if p == nil {
		return false
	}
	ids := h.horoscopeStarIdentifiers(p.Index)
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return true
		}
	}
	return false
}

// NotHaveHoroscopeStars 判断指定运限宫位是否一颗指定流耀都不包含。
func (h *Horoscope) NotHaveHoroscopeStars(nameKeyOrName string, scope string, stars []string) bool {
	p := h.Palace(nameKeyOrName, scope)
	if p == nil {
		return false
	}
	ids := h.horoscopeStarIdentifiers(p.Index)
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return false
		}
	}
	return true
}

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
}

// WithMutagen 判断星耀是否具有指定四化（传 MutagenLu 等常量）。
func (s *Star) WithMutagen(mutagenKey string) bool {
	return s.MutagenKey != "" && s.MutagenKey == mutagenKey
}

// WithBrightness 判断星耀是否具有指定亮度（传 BrightnessMiao 等常量）。
func (s *Star) WithBrightness(brightnessKey string) bool {
	return s.BrightnessKey != "" && s.BrightnessKey == brightnessKey
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
}

// allStarIdentifiers 汇集宫内所有星耀的 key 与翻译名，供查询词匹配。
func (p *Palace) allStarIdentifiers() map[string]struct{} {
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
	ids := p.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HasOneOf 判断宫位是否包含指定星耀中的至少一颗。
func (p *Palace) HasOneOf(stars ...string) bool {
	ids := p.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return true
		}
	}
	return false
}

// NotHave 判断宫位是否不包含指定的所有星耀。
func (p *Palace) NotHave(stars ...string) bool {
	ids := p.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return false
		}
	}
	return true
}

// HasMutagen 判断宫位是否有指定四化（只检查主星和辅星；传 MutagenLu 等常量）。
func (p *Palace) HasMutagen(mutagenKey string) bool {
	for _, group := range [][]Star{p.MajorStars, p.MinorStars} {
		for _, s := range group {
			if s.MutagenKey == mutagenKey {
				return true
			}
		}
	}
	return false
}

// IsEmpty 判断宫位是否为空宫（无主星）。
func (p *Palace) IsEmpty() bool {
	return len(p.MajorStars) == 0
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
	// GenderKey 为机器可读性别（"male"/"female"）
	GenderKey string `json:"genderKey"`
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
	// Language 为排盘语言（"zh_cn" 等）
	Language string `json:"language"`
	// Config 为排盘配置
	Config Config `json:"config"`
}

// Palace 通过宫位标识（PalaceSoul 等常量）或当前语言宫名获取宫位；未找到返回 nil。
func (a *Astrolabe) Palace(nameKeyOrName string) *Palace {
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
	if index < 0 || index >= len(a.Palaces) {
		return nil
	}
	return &a.Palaces[index]
}

// Star 通过星耀标识（StarZiwei 等常量）或当前语言星名查找星耀及其所在宫位；
// 未找到返回 (nil, nil)。
func (a *Astrolabe) Star(keyOrName string) (*Star, *Palace) {
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

// SurroundedPalaces 获取指定宫位索引的三方四正。
func (a *Astrolabe) SurroundedPalaces(index int) SurroundedPalaces {
	return SurroundedPalaces{
		Target:   &a.Palaces[index%12],
		Opposite: &a.Palaces[(index+6)%12],
		Wealth:   &a.Palaces[(index+8)%12],
		Career:   &a.Palaces[(index+4)%12],
	}
}

// Have 判断三方四正是否包含指定的所有星耀。
func (sp *SurroundedPalaces) Have(stars ...string) bool {
	ids := make(map[string]struct{})
	for _, p := range []*Palace{sp.Target, sp.Opposite, sp.Wealth, sp.Career} {
		for k := range p.allStarIdentifiers() {
			ids[k] = struct{}{}
		}
	}
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HaveMutagen 判断三方四正中是否有指定四化。
func (sp *SurroundedPalaces) HaveMutagen(mutagenKey string) bool {
	for _, p := range []*Palace{sp.Target, sp.Opposite, sp.Wealth, sp.Career} {
		if p.HasMutagen(mutagenKey) {
			return true
		}
	}
	return false
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
}

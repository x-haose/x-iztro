package iztro

// 排盘算法用到的查表与顺序常量。
//
// 取值一律为语言无关标识（keys.go 常量），可直接同星盘对象的 *Key 字段比对。
// 表是排盘的输入而非输出，因此与输出语言无关。

// StarInfo 为星耀基础信息。
type StarInfo struct {
	// Brightness 为十二宫亮度标识，索引 0 为寅宫；该宫无亮度时为空串
	Brightness []string `json:"brightness"`
	// FiveElements 为五行；原表中太阳、七杀与六颗辅星未填，为空串
	FiveElements string `json:"fiveElements"`
	// YinYang 为阴阳；原表中部分星耀未填，为空串
	YinYang string `json:"yinYang"`
}

// HeavenlyStemInfo 为天干信息。
type HeavenlyStemInfo struct {
	// YinYang 为阴阳
	YinYang string `json:"yinYang"`
	// FiveElements 为五行
	FiveElements string `json:"fiveElements"`
	// Crash 为对冲天干标识；戊己无对冲，为空串
	Crash string `json:"crash"`
	// Mutagen 为四化四星标识，顺序为禄、权、科、忌
	Mutagen []string `json:"mutagen"`
}

// EarthlyBranchInfo 为地支信息。
type EarthlyBranchInfo struct {
	// YinYang 为阴阳
	YinYang string `json:"yinYang"`
	// FiveElements 为五行
	FiveElements string `json:"fiveElements"`
	// Crash 为对冲地支标识
	Crash string `json:"crash"`
	// Soul 为命主星标识
	Soul string `json:"soul"`
	// Body 为身主星标识
	Body string `json:"body"`
	// Inside 为对应脏腑；只有中文一种写法，不参与国际化
	Inside string `json:"inside"`
	// Outside 为对应身体部位；只有中文一种写法，不参与国际化
	Outside string `json:"outside"`
	// HealthTip 为健康提示；只有中文一种写法，不参与国际化
	HealthTip string `json:"healthTip"`
}

// Constants 为顺序常量与推算规则表。
type Constants struct {
	// Languages 为支持的语言代码
	Languages []string `json:"LANGUAGES"`
	// HeavenlyStems 为天干顺序
	HeavenlyStems []string `json:"HEAVENLY_STEMS"`
	// EarthlyBranches 为地支顺序
	EarthlyBranches []string `json:"EARTHLY_BRANCHES"`
	// Zodiac 为生肖标识，按地支顺序
	Zodiac []string `json:"ZODIAC"`
	// Signs 为星座标识，按黄道顺序
	Signs []string `json:"SIGNS"`
	// Palaces 为十二宫名，按命宫起的顺序
	Palaces []string `json:"PALACES"`
	// Gender 为男女各自的阴阳
	Gender map[string]string `json:"GENDER"`
	// ChineseTime 为时辰标识，索引 0-12
	ChineseTime []string `json:"CHINESE_TIME"`
	// TimeRange 为时辰对应的时间区间
	TimeRange []string `json:"TIME_RANGE"`
	// TigerRule 为五虎遁：年干推正月天干
	TigerRule map[string]string `json:"TIGER_RULE"`
	// RatRule 为五鼠遁：日干推子时天干
	RatRule map[string]string `json:"RAT_RULE"`
	// Mutagen 为四化顺序：禄、权、科、忌
	Mutagen []string `json:"MUTAGEN"`
}

// StarsInfo 取星耀基础信息表：十四主星与文昌、文曲、火星、铃星、擎羊、陀罗共二十颗。
func StarsInfo() (map[string]StarInfo, error) {
	var out map[string]StarInfo
	return out, utilQuery(map[string]any{"kind": "starsInfo"}, &out)
}

// HeavenlyStems 取天干信息表，键为天干标识。
func HeavenlyStems() (map[string]HeavenlyStemInfo, error) {
	var out map[string]HeavenlyStemInfo
	return out, utilQuery(map[string]any{"kind": "heavenlyStems"}, &out)
}

// EarthlyBranches 取地支信息表，键为地支标识。
func EarthlyBranches() (map[string]EarthlyBranchInfo, error) {
	var out map[string]EarthlyBranchInfo
	return out, utilQuery(map[string]any{"kind": "earthlyBranches"}, &out)
}

// GetConstants 取顺序常量与推算规则表。
func GetConstants() (Constants, error) {
	var out Constants
	return out, utilQuery(map[string]any{"kind": "constants"}, &out)
}

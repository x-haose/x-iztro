package iztro

import (
	"context"
	"encoding/json"
	"fmt"
)

// 排盘算法的公开工具函数。
//
// 参数与返回值中的枚举一律使用语言无关 key（keys.go 常量），
// 因此这些函数与输出语言无关，可直接与星盘对象的 *Key 字段互操作。

// utilQuery 调用 wasm 查询入口，把 value 解到 out 指向的值。
func utilQuery(payload map[string]any, out any) error {
	return utilQueryContext(context.Background(), payload, out)
}

// utilQueryContext 为 utilQuery 的 Context 变体。
func utilQueryContext(ctx context.Context, payload map[string]any, out any) error {
	raw, err := callWasm(ctx, fnQuery, payload)
	if err != nil {
		return err
	}
	var envelope struct {
		Value json.RawMessage `json:"value"`
	}
	if err := json.Unmarshal(raw, &envelope); err != nil {
		return internalError("decode query result: " + err.Error())
	}
	if err := json.Unmarshal(envelope.Value, out); err != nil {
		return internalError("decode query value: " + err.Error())
	}
	return nil
}

// FixIndex 把索引约束到 0..max 的循环区间；负数也能正确回绕。
//
// max 传 0 时取默认值 12——这是 iztro fixIndex 默认参数的等价写法，
// 与 Go 零值直觉相反，十二宫回绕请直接用 FixIndex12。max 为负数时返回错误。
//
// 语义同 wasm 查询 "fixIndex"；只是取模，故在 Go 侧直接算，不往返 wasm。
func FixIndex(index int, max int) (int, error) {
	if max < 0 {
		return 0, invalidArgument(fmt.Sprintf("invalid max '%d': expected a positive integer", max))
	}
	if max == 0 {
		max = 12
	}
	return ((index % max) + max) % max, nil
}

// FixIndex12 把索引约束到 0..11 的循环区间，即十二宫的回绕；负数也能正确回绕。
func FixIndex12(index int) int {
	return ((index % 12) + 12) % 12
}

// EarthlyBranchToPalaceIndex 地支标识转宫位索引（寅宫为 0）。
// branchKey 不是合法地支标识时返回错误。
//
// 语义同 wasm 查询 "earthlyBranchToPalaceIndex"；只是查序号加取模，
// 故在 Go 侧直接算，不往返 wasm。
func EarthlyBranchToPalaceIndex(branchKey string) (int, error) {
	index, ok := branchOrder[branchKey]
	if !ok {
		return 0, invalidArgument(fmt.Sprintf("unknown earthly branch key '%s'", branchKey))
	}
	// 宫位自寅起，减去寅的地支序号 2
	return FixIndex12(index - 2), nil
}

// branchOrder 为地支标识到地支序号的映射，子为 0、亥为 11。
var branchOrder = map[string]int{
	BranchZi:   0,
	BranchChou: 1,
	BranchYin:  2,
	BranchMao:  3,
	BranchChen: 4,
	BranchSi:   5,
	BranchWu:   6,
	BranchWei:  7,
	BranchShen: 8,
	BranchYou:  9,
	BranchXu:   10,
	BranchHai:  11,
}

// TimeToIndex 小时（0-23）转时辰索引：0 为早子时，12 为晚子时。
// hour 超出 0-23 时返回错误。
//
// 语义同 wasm 查询 "timeToIndex"；只是整数除法，故在 Go 侧直接算，不往返 wasm。
func TimeToIndex(hour uint8) (uint8, error) {
	switch {
	case hour > 23:
		return 0, invalidArgument(fmt.Sprintf("invalid hour '%d': expected 0-23", hour))
	case hour == 0:
		return 0, nil
	case hour == 23:
		return 12, nil
	default:
		// 每两小时一个时辰，奇数小时进位到下一个时辰
		return (hour + 1) / 2, nil
	}
}

// GetAgeIndex 由生年地支取小限起始宫位索引。
func GetAgeIndex(branchKey string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "getAgeIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetBrightness 取星耀落在指定宫位时的亮度标识；该星没有亮度表时返回空串。
// config 中的自定义亮度表优先于默认表；传 nil 取默认。
func GetBrightness(starKey string, palaceIndex int, config *Config) (string, error) {
	var out *string
	if err := utilQuery(map[string]any{
		"kind":    "getBrightness",
		"starKey": starKey,
		"index":   palaceIndex,
		"config":  config,
	}, &out); err != nil {
		return "", err
	}
	if out == nil {
		return "", nil
	}
	return *out, nil
}

// GetMutagen 取指定天干下某颗星化什么；不在该天干四化表内时返回空串。
// config 中的自定义四化表优先于默认表；传 nil 取默认。
func GetMutagen(starKey string, stemKey string, config *Config) (string, error) {
	var out *string
	if err := utilQuery(map[string]any{
		"kind":    "getMutagen",
		"starKey": starKey,
		"stemKey": stemKey,
		"config":  config,
	}, &out); err != nil {
		return "", err
	}
	if out == nil {
		return "", nil
	}
	return *out, nil
}

// GetMutagensByHeavenlyStem 取指定天干化出的四颗星，顺序为禄、权、科、忌。
// config 中的自定义四化表优先于默认表；传 nil 取默认。
func GetMutagensByHeavenlyStem(stemKey string, config *Config) ([]string, error) {
	var out []string
	return out, utilQuery(map[string]any{
		"kind":    "getMutagensByHeavenlyStem",
		"stemKey": stemKey,
		"config":  config,
	}, &out)
}

// SoulAndBody 为命宫与身宫的推算结果。
type SoulAndBody struct {
	// SoulIndex 为命宫的宫位索引
	SoulIndex int `json:"soulIndex"`
	// BodyIndex 为身宫的宫位索引
	BodyIndex int `json:"bodyIndex"`
	// HeavenlyStemOfSoul 为命宫天干标识
	HeavenlyStemOfSoul string `json:"heavenlyStemOfSoul"`
	// EarthlyBranchOfSoul 为命宫地支标识
	EarthlyBranchOfSoul string `json:"earthlyBranchOfSoul"`
}

// GetSoulAndBody 由农历月索引、时辰索引与年干推命宫与身宫。
// monthIndex 可由 FixLunarMonthIndex 求得。
func GetSoulAndBody(monthIndex int, timeIndex uint8, yearlyStemKey string) (*SoulAndBody, error) {
	var out SoulAndBody
	if err := utilQuery(map[string]any{
		"kind":       "getSoulAndBody",
		"monthIndex": monthIndex,
		"timeIndex":  timeIndex,
		"stemKey":    yearlyStemKey,
	}, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

// GetFiveElementsClass 由命宫干支推五行局标识（Class* 常量）。
func GetFiveElementsClass(stemKey string, branchKey string) (string, error) {
	var out string
	return out, utilQuery(map[string]any{
		"kind":      "getFiveElementsClass",
		"stemKey":   stemKey,
		"branchKey": branchKey,
	}, &out)
}

// GetPalaceNames 由命宫索引推十二宫名标识，按宫位索引排列；
// 第 i 项即 Astrolabe.Palaces[i] 的宫名。
func GetPalaceNames(soulIndex int) ([]string, error) {
	var out []string
	return out, utilQuery(map[string]any{
		"kind":      "getPalaceNames",
		"soulIndex": soulIndex,
	}, &out)
}

// DecadalsAndAges 为十二宫的大限与小限，均按宫位索引排列。
type DecadalsAndAges struct {
	// Decadals 为每宫的大限：岁数区间与该宫干支标识
	Decadals []Decadal `json:"decadals"`
	// Ages 为每宫的小限岁数列表
	Ages [][]int `json:"ages"`
}

// GetDecadalsAndAges 由命宫索引与五行局推十二宫的大限与小限。
//
// 大限起运岁数由五行局决定，顺逆由性别阴阳与年支阴阳决定；小限起宫由年支决定。
func GetDecadalsAndAges(soulIndex int, fiveElementsClass string, gender Gender, yearlyStemKey, yearlyBranchKey string) (DecadalsAndAges, error) {
	var out DecadalsAndAges
	return out, utilQuery(map[string]any{
		"kind":              "getHoroscope",
		"soulIndex":         soulIndex,
		"fiveElementsClass": fiveElementsClass,
		"gender":            gender,
		"stemKey":           yearlyStemKey,
		"branchKey":         yearlyBranchKey,
	}, &out)
}

// FixLunarMonthIndex 取修正后的农历月索引（正月为 0）。
// 修正闰月时，闰月十五日之后按下月算；晚子时（索引 12）不参与修正。
func FixLunarMonthIndex(lunarMonth int, lunarDay int, isLeap bool, timeIndex uint8, fixLeap bool) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":       "fixLunarMonthIndex",
		"lunarMonth": lunarMonth,
		"lunarDay":   lunarDay,
		"isLeap":     isLeap,
		"timeIndex":  timeIndex,
		"fixLeap":    fixLeap,
	}, &out)
}

// FixLunarDayIndex 取修正后的农历日索引；晚子时属次日，因此不减一。
func FixLunarDayIndex(lunarDay int, timeIndex uint8) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "fixLunarDayIndex",
		"lunarDay":  lunarDay,
		"timeIndex": timeIndex,
	}, &out)
}

// TranslateChineseDate 按语言拼接四柱干支展示串（星盘 ChineseDate 字段即由此生成）。
//
// pillars 为四柱标识 [年, 月, 日, 时]，每柱为 [天干, 地支]。
// 词条均为单字符时柱内紧凑相连、柱间空格（如「庚辰 甲申 丁未 庚子」）；
// 任一词条为多字符时柱内空格、柱间「 - 」。
func TranslateChineseDate(pillars [4][2]string, language Language) (string, error) {
	payload := make([][]string, 4)
	for i, p := range pillars {
		payload[i] = []string{p[0], p[1]}
	}
	var out string
	return out, utilQuery(map[string]any{
		"kind":     "translateChineseDate",
		"pillars":  payload,
		"language": language,
	}, &out)
}

// MergeStars 把多组「十二宫星耀」按宫位合并成一组。
//
// 安星是分批进行的（主星、辅星、杂耀各出一组十二宫列表），
// 本函数按宫位索引把它们首尾相接，顺序为传入顺序。
// 某一组长度不为 12 时返回 error。
func MergeStars(groups ...[][]Star) ([][]Star, error) {
	merged := make([][]Star, 12)
	for i := range merged {
		merged[i] = []Star{}
	}
	for gi, group := range groups {
		if len(group) != 12 {
			return nil, invalidArgument(fmt.Sprintf("star group %d must cover 12 palaces, got %d", gi, len(group)))
		}
		for i, stars := range group {
			merged[i] = append(merged[i], stars...)
		}
	}
	return merged, nil
}

// AstrolabeToPrompt 生成本命盘的 AI 分析 prompt：一段结构化文本，
// 按星盘的排盘语言输出，可直接喂给大模型。
func (a *Astrolabe) AstrolabeToPrompt() (string, error) {
	return a.AstrolabeToPromptContext(context.Background())
}

// AstrolabeToPromptContext 为 AstrolabeToPrompt 的 Context 变体；
// ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) AstrolabeToPromptContext(ctx context.Context) (string, error) {
	if a == nil {
		return "", invalidArgument("astrolabeToPrompt: nil astrolabe")
	}
	payload := map[string]any{
		"kind":      "astrolabeToPrompt",
		"solarDate": a.SolarDate,
		"timeIndex": a.TimeIndex,
		"gender":    a.GenderKey,
		"fixLeap":   a.FixLeap,
		"language":  a.Language,
		"config":    a.requestConfig(),
	}
	a.addRearrange(payload)
	var out string
	return out, utilQueryContext(ctx, payload, &out)
}

// HoroscopeToPrompt 生成运限的 AI 分析 prompt；本命信息与运限一并写入。
func (a *Astrolabe) HoroscopeToPrompt(targetDate string, targetTimeIndex uint8) (string, error) {
	return a.HoroscopeToPromptContext(context.Background(), targetDate, targetTimeIndex)
}

// HoroscopeToPromptContext 为 HoroscopeToPrompt 的 Context 变体；
// ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) HoroscopeToPromptContext(ctx context.Context, targetDate string, targetTimeIndex uint8) (string, error) {
	if a == nil {
		return "", invalidArgument("horoscopeToPrompt: nil astrolabe")
	}
	payload := map[string]any{
		"kind":            "horoscopeToPrompt",
		"solarDate":       a.SolarDate,
		"timeIndex":       a.TimeIndex,
		"gender":          a.GenderKey,
		"fixLeap":         a.FixLeap,
		"language":        a.Language,
		"config":          a.requestConfig(),
		"targetDate":      targetDate,
		"targetTimeIndex": targetTimeIndex,
	}
	a.addRearrange(payload)
	var out string
	return out, utilQueryContext(ctx, payload, &out)
}

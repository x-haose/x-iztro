package iztro

// 轻量查询：不需要完整星盘就能拿到的单个结果。
// 五个查询共用 wasm 侧的同一个入口，由 kind 分派。

// translatedResult 为轻量查询的双轨返回：Text 按语言翻译（与 iztro 同名函数
// 的返回一致），Keys 为同一结果的语言无关标识列表。
type translatedResult struct {
	Text string   `json:"text"`
	Keys []string `json:"keys"`
}

// queryTranslated 调用 wasm 的查询入口并取出双轨结果。
func queryTranslated(payload map[string]any) (translatedResult, error) {
	var out translatedResult
	return out, utilQuery(payload, &out)
}

// query 调用 wasm 的查询入口并取出翻译文本。
func query(payload map[string]any) (string, error) {
	out, err := queryTranslated(payload)
	return out.Text, err
}

// GetZodiacBySolarDate 通过阳历日期取生肖。
//
// 生肖由年支决定，换年时点受 Config.YearDivide 影响 ——
// 正月初一与立春之间的生日会随配置得到不同结果。
func GetZodiacBySolarDate(solarDate string, language Language, config *Config) (string, error) {
	return query(map[string]any{
		"kind":      "zodiacBySolar",
		"solarDate": solarDate,
		"language":  language,
		"config":    config,
	})
}

// GetSignBySolarDate 通过阳历日期取星座。星座只由公历日期决定。
func GetSignBySolarDate(solarDate string, language Language) (string, error) {
	return query(map[string]any{
		"kind":      "signBySolar",
		"solarDate": solarDate,
		"language":  language,
	})
}

// GetSignByLunarDate 通过农历日期取星座；isLeapMonth 在该月没有闰月时不生效。
func GetSignByLunarDate(lunarDate string, isLeapMonth bool, language Language) (string, error) {
	return query(map[string]any{
		"kind":        "signByLunar",
		"lunarDate":   lunarDate,
		"isLeapMonth": isLeapMonth,
		"language":    language,
	})
}

// GetMajorStarBySolarDate 通过阳历日期取命宫主星，多颗以逗号分隔。
// 命宫为空宫时借对宫主星。
func GetMajorStarBySolarDate(solarDate string, timeIndex uint8, fixLeap bool, language Language, config *Config) (string, error) {
	return query(map[string]any{
		"kind":      "majorStarBySolar",
		"solarDate": solarDate,
		"timeIndex": timeIndex,
		"fixLeap":   fixLeap,
		"language":  language,
		"config":    config,
	})
}

// GetMajorStarByLunarDate 通过农历日期取命宫主星，多颗以逗号分隔。
// 命宫为空宫时借对宫主星；leap 标为闰月但该月没有闰月时按普通月处理。
func GetMajorStarByLunarDate(lunarDate string, timeIndex uint8, leap LeapMonth, language Language, config *Config) (string, error) {
	isLeapMonth, fixLeap, err := leap.flags()
	if err != nil {
		return "", err
	}
	return query(map[string]any{
		"kind":        "majorStarByLunar",
		"lunarDate":   lunarDate,
		"timeIndex":   timeIndex,
		"isLeapMonth": isLeapMonth,
		"fixLeap":     fixLeap,
		"language":    language,
		"config":      config,
	})
}

// MajorStarKeysBySolarDate 通过阳历日期取命宫主星的语言无关标识列表
// （StarZiweiMaj 等常量取值）。空宫借对宫的口径与 GetMajorStarBySolarDate 一致。
func MajorStarKeysBySolarDate(solarDate string, timeIndex uint8, fixLeap bool, config *Config) ([]string, error) {
	out, err := queryTranslated(map[string]any{
		"kind":      "majorStarBySolar",
		"solarDate": solarDate,
		"timeIndex": timeIndex,
		"fixLeap":   fixLeap,
		"language":  LanguageZhCN,
		"config":    config,
	})
	return out.Keys, err
}

// MajorStarKeysByLunarDate 通过农历日期取命宫主星的语言无关标识列表。
// 空宫借对宫与闰月口径与 GetMajorStarByLunarDate 一致。
func MajorStarKeysByLunarDate(lunarDate string, timeIndex uint8, leap LeapMonth, config *Config) ([]string, error) {
	isLeapMonth, fixLeap, err := leap.flags()
	if err != nil {
		return nil, err
	}
	out, err := queryTranslated(map[string]any{
		"kind":        "majorStarByLunar",
		"lunarDate":   lunarDate,
		"timeIndex":   timeIndex,
		"isLeapMonth": isLeapMonth,
		"fixLeap":     fixLeap,
		"language":    LanguageZhCN,
		"config":      config,
	})
	return out.Keys, err
}

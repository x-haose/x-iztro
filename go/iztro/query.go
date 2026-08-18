package iztro

import (
	"context"
	"encoding/json"
)

// 轻量查询：不需要完整星盘就能拿到的单个结果。
// 五个查询共用 wasm 侧的同一个入口，由 kind 分派。

// queryResult 为 wasm 查询入口的返回体。
type queryResult struct {
	Value string `json:"value"`
}

// query 调用 wasm 的查询入口并取出结果字符串。
func query(payload map[string]any) (string, error) {
	raw, err := callWasm(context.Background(), fnQuery, payload)
	if err != nil {
		return "", err
	}
	var out queryResult
	if err := json.Unmarshal(raw, &out); err != nil {
		return "", internalError("decode query result: " + err.Error())
	}
	return out.Value, nil
}

// GetZodiacBySolarDate 通过阳历日期取生肖。
//
// 生肖由年支决定，换年时点受 Config.YearDivide 影响 ——
// 正月初一与立春之间的生日会随配置得到不同结果。
func GetZodiacBySolarDate(solarDate string, language string, config *Config) (string, error) {
	return query(map[string]any{
		"kind":      "zodiacBySolar",
		"solarDate": solarDate,
		"language":  language,
		"config":    config,
	})
}

// GetSignBySolarDate 通过阳历日期取星座。星座只由公历日期决定。
func GetSignBySolarDate(solarDate string, language string) (string, error) {
	return query(map[string]any{
		"kind":      "signBySolar",
		"solarDate": solarDate,
		"language":  language,
	})
}

// GetSignByLunarDate 通过农历日期取星座；isLeapMonth 在该月没有闰月时不生效。
func GetSignByLunarDate(lunarDate string, isLeapMonth bool, language string) (string, error) {
	return query(map[string]any{
		"kind":        "signByLunar",
		"lunarDate":   lunarDate,
		"isLeapMonth": isLeapMonth,
		"language":    language,
	})
}

// GetMajorStarBySolarDate 通过阳历日期取命宫主星，多颗以逗号分隔。
// 命宫为空宫时借对宫主星。
func GetMajorStarBySolarDate(solarDate string, timeIndex uint8, fixLeap bool, language string, config *Config) (string, error) {
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
// 命宫为空宫时借对宫主星；isLeapMonth 在该月没有闰月时不生效。
func GetMajorStarByLunarDate(lunarDate string, timeIndex uint8, isLeapMonth bool, fixLeap bool, language string, config *Config) (string, error) {
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

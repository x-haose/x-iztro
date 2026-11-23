package iztro

// 历法查询：农历年的月份表、公历与农历互换、农历年的年干支。
//
// 与排盘同一份历法内核，结果与星盘的 RawDates.LunarDate、年柱逐日一致。支持公历与农历
// 1-9999 年；1582-10-15 以前按儒略历，1582-10-05 至 10-14 不存在。
//
// 报错与排盘入口一致：给出完整日期的换算在日期不存在时返回 ErrInvalidDate，
// 只给年份的查询在年份越界时返回 ErrInvalidArgument。

// RawSolarDate 为数字化公历日期；1582-10-15 以前为儒略历日期。
type RawSolarDate struct {
	// SolarYear 为公历年；农历 9999 年最后一两个月落在公历 10000 年
	SolarYear int `json:"solarYear"`
	// SolarMonth 为公历月（1-12）
	SolarMonth int `json:"solarMonth"`
	// SolarDay 为公历日（1-31）
	SolarDay int `json:"solarDay"`
}

// LunarMonthInfo 为农历年里的一个月。
type LunarMonthInfo struct {
	// Month 为月份（1-12）；闰月取所闰的月份，如闰四月为 4
	Month int `json:"month"`
	// IsLeap 表示是否闰月
	IsLeap bool `json:"isLeap"`
	// DayCount 为天数：29（小月）或 30（大月）
	DayCount int `json:"dayCount"`
	// FirstDay 为初一对应的公历日期
	FirstDay RawSolarDate `json:"firstDay"`
}

// LunarMonths 返回农历 year 年的全部月份，按时间先后排列；闰月紧跟在所闰的月之后。
// 一般为 12 个月，有闰月的年为 13 个月；个别古代年份例外，与 lunar-typescript 相同，
// 如农历 237 年只有 11 个月、没有三月。year 不在 1-9999 时返回 ErrInvalidArgument。
func LunarMonths(year int) ([]LunarMonthInfo, error) {
	var out []LunarMonthInfo
	return out, utilQuery(map[string]any{"kind": "lunarMonths", "year": year}, &out)
}

// SolarToLunar 返回公历日期对应的农历日期。公历 1 年 1 月初的几十天属农历 0 年（冬月、腊月），
// 此时 LunarYear 为 0。年不在 1-9999、月日不存在，或落在改历空缺 1582-10-05 至 10-14 时
// 返回 ErrInvalidDate。
func SolarToLunar(year, month, day int) (RawLunarDate, error) {
	var out RawLunarDate
	return out, utilQuery(map[string]any{
		"kind":  "solarToLunar",
		"year":  year,
		"month": month,
		"day":   day,
	}, &out)
}

// LunarToSolar 返回农历日期对应的公历日期。isLeap 只表示「这一天在闰月里」；
// 排盘入口的 fixLeap（闰月十五之后视作次月）管的是星曜按哪个月安，不改变日期本身，这里没有对应参数。
// 年不在 1-9999、该年没有这个月（isLeap 为真而该年不闰此月，或古代年份缺此月）、
// 日超出当月天数时返回 ErrInvalidDate；不会退回同号的常规月。
func LunarToSolar(year, month, day int, isLeap bool) (RawSolarDate, error) {
	var out RawSolarDate
	return out, utilQuery(map[string]any{
		"kind":       "lunarToSolar",
		"year":       year,
		"lunarMonth": month,
		"lunarDay":   day,
		"isLeap":     isLeap,
	}, &out)
}

// LunarYearPillar 返回农历 year 年的年干支标识（公历 4 年为甲子）。
// 年干支按年号循环，与分界口径无关；口径决定这一柱从哪天起算：正月初一分界（默认）为农历
// year 年正月初一至除夕，立春分界为公历 year 年立春当日至次年立春前一日。
// year 不在 1-9999 时返回 ErrInvalidArgument。
func LunarYearPillar(year int) (Pillar, error) {
	var out Pillar
	return out, utilQuery(map[string]any{"kind": "lunarYearPillar", "year": year}, &out)
}

package iztro

import "context"

// 反推：由八字四柱或星盘特征反查候选生辰。
// 计算全部在 wasm 内核完成（剪枝枚举 + 正排终验，与正向排盘零分歧），这里只做类型化封装。
// 星盘布局与性别无关，反推的目标是生辰，故不收性别。

// Pillar 为一柱干支：[天干标识, 地支标识]（StemJia、BranchZi 等常量）。
type Pillar [2]string

// BirthCandidate 为一个候选生辰，可直接交给 BySolar 排盘。
type BirthCandidate struct {
	// SolarDate 为公历日期，YYYY-M-D
	SolarDate string `json:"solarDate"`
	// TimeIndex 为时辰索引 0-12（0 为早子时，12 为晚子时）
	TimeIndex uint8 `json:"timeIndex"`
}

// StarPosition 为一颗星与其落宫地支：星盘特征反推的原子条件。
type StarPosition struct {
	// Star 为星耀标识（须为本命盘星耀，流耀不接受）
	Star string `json:"star"`
	// Branch 为落宫地支标识
	Branch string `json:"branch"`
}

// ReverseCriteria 为星盘特征反推的条件集。
// 全部字段可选（空串/空值即缺省），但至少要给一个条件；
// 条件越具体（尤其命宫地支、生年四化、主星落宫），反推越快。
type ReverseCriteria struct {
	// SoulBranch 为命宫地支标识，空串不限
	SoulBranch string
	// BodyBranch 为身宫地支标识，空串不限
	BodyBranch string
	// FiveElementsClass 为五行局标识，空串不限
	FiveElementsClass string
	// Stars 为星耀落宫条件，全部须同时满足
	Stars []StarPosition
	// Mutagens 为生年四化 [禄, 权, 科, 忌] 各自的星耀标识，空串表示该位不限
	Mutagens [4]string
	// YearRange 为公历年闭区间（含两端），零值取 [1900, 2100]；须落在 1583-9999 内
	YearRange [2]int
	// FixLeap 为是否修正闰月（与排盘入参同义）；nil 取内核默认 true。
	// 值类型 bool 的零值会把「未设置」静默当成 false、与内核默认相反，
	// 故用指针区分（与 YearRange 零值取默认同思路）；显式取值用 Bool 构造
	FixLeap *bool
	// Limit 为候选数上限：达到即停止搜索并置 ReverseResult.Truncated；0 取内核默认（512）
	Limit int
}

// payload 编组为绑定层入参：空串字段转 null（缺省），零值年份范围转默认，
// nil FixLeap 省略键（由内核取默认 true）。
func (c *ReverseCriteria) payload() map[string]any {
	orNil := func(s string) any {
		if s == "" {
			return nil
		}
		return s
	}
	stars := make([]map[string]any, 0, len(c.Stars))
	for _, p := range c.Stars {
		stars = append(stars, map[string]any{"star": p.Star, "branch": p.Branch})
	}
	mutagens := make([]any, 4)
	for i, m := range c.Mutagens {
		mutagens[i] = orNil(m)
	}
	yearRange := c.YearRange
	if yearRange == [2]int{} {
		yearRange = [2]int{1900, 2100}
	}
	p := map[string]any{
		"soulBranch":        orNil(c.SoulBranch),
		"bodyBranch":        orNil(c.BodyBranch),
		"fiveElementsClass": orNil(c.FiveElementsClass),
		"stars":             stars,
		"mutagens":          mutagens,
		"yearRange":         []int{yearRange[0], yearRange[1]},
		"limit":             c.Limit,
	}
	// nil 即未设置：省略键让内核取默认 true
	if c.FixLeap != nil {
		p["fixLeap"] = *c.FixLeap
	}
	return p
}

// ReverseResult 为星盘特征反推的结果。
type ReverseResult struct {
	// Candidates 为满足全部条件的候选生辰，按枚举序排列：农历年升序，
	// 年内依 月→时辰→日；同一年内不保证公历日期升序。
	Candidates []BirthCandidate `json:"candidates"`
	// Truncated 为是否因达到候选数上限而提前截断；截断时枚举序更靠后的
	// 解未被搜索，其中可能包含公历日期更早的解。
	Truncated bool `json:"truncated"`
}

// SolarDatesByBazi 由八字四柱反查公历生辰。
//
// 四柱按 config 的分界口径解释（年柱 YearDivide、月柱 HoroscopeDivide、晚子归属 DayDivide），
// 与星盘 RawDates.ChineseDate 同一套语义。一组四柱在范围内通常每约 60 年出现一次；
// 子时因早晚子之分可能给出两个候选。年份区间含两端，须落在 1583-9999 内。
func SolarDatesByBazi(yearly, monthly, daily, hourly Pillar, startYear, endYear int, config *Config) ([]BirthCandidate, error) {
	return SolarDatesByBaziContext(context.Background(), yearly, monthly, daily, hourly, startYear, endYear, config)
}

// SolarDatesByBaziContext 为 SolarDatesByBazi 的 Context 变体；ctx 用于取消等待 wasm 实例。
func SolarDatesByBaziContext(ctx context.Context, yearly, monthly, daily, hourly Pillar, startYear, endYear int, config *Config) ([]BirthCandidate, error) {
	var out []BirthCandidate
	return out, utilQueryContext(ctx, map[string]any{
		"kind":      "solarDatesByBazi",
		"pillars":   [][2]string{yearly, monthly, daily, hourly},
		"startYear": startYear,
		"endYear":   endYear,
		"config":    config,
	}, &out)
}

// ReverseChart 由星盘特征反查候选生辰。
//
// 排盘配置贯穿判定（四化表、派别、分界都按它算），候选用同一配置排盘必满足全部条件。
// 条件为空、包含流耀、年份范围非法时返回错误。
func ReverseChart(criteria *ReverseCriteria, config *Config) (*ReverseResult, error) {
	return ReverseChartContext(context.Background(), criteria, config)
}

// ReverseChartContext 为 ReverseChart 的 Context 变体；ctx 用于取消等待 wasm 实例。
func ReverseChartContext(ctx context.Context, criteria *ReverseCriteria, config *Config) (*ReverseResult, error) {
	if criteria == nil {
		return nil, invalidArgument("reverseChart: nil criteria")
	}
	var out ReverseResult
	if err := utilQueryContext(ctx, map[string]any{
		"kind":            "reverseChart",
		"reverseCriteria": criteria.payload(),
		"config":          config,
	}, &out); err != nil {
		return nil, err
	}
	return &out, nil
}

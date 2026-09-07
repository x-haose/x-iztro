package iztro

import "context"

// 夹宫与三个运限列表：一次取回整层的运限项，省去逐日期调用 Horoscope 再拼装。
// 三个列表项都内嵌 HoroscopeScope，通用运限字段（宫位索引、层级名、干支、
// 十二宫名、四化、流耀）与列表专属字段在 JSON 里平铺同层。

// FlankingPalaces 为目标宫的夹宫：盘上紧邻其前后的两宫，索引对 12 回绕。
//
// 星耀与四化的判定一律在**两宫合计**的集合上做：Have 要求两宫合起来含全部目标星，
// 不要求同在一宫。
type FlankingPalaces struct {
	// Previous 为前一宫（目标宫索引 -1）
	Previous *Palace
	// Next 为后一宫（目标宫索引 +1）
	Next *Palace
}

// DecadalListItem 为大限列表的一项。
type DecadalListItem struct {
	// HoroscopeScope 为该大限的通用运限字段：所在宫位索引、以该宫为命宫推排的
	// 十二宫名、该限干支与四化、大限流曜
	HoroscopeScope
	// PalaceName 为该大限所在的本命宫名（翻译文本）
	PalaceName string `json:"palaceName"`
	// PalaceNameKey 为该本命宫名的语言无关标识（PalaceSoul 等常量）
	PalaceNameKey string `json:"palaceNameKey"`
	// AgeRange 为起止虚岁 [起, 止]，含两端
	AgeRange [2]int `json:"ageRange"`
	// YearRange 为起止农历年份 [起, 止]，含两端
	YearRange [2]int `json:"yearRange"`
}

// YearlyListItem 为流年列表的一项。
type YearlyListItem struct {
	// HoroscopeScope 为该流年的通用运限字段
	HoroscopeScope
	// Age 为该流年对应的虚岁
	Age int `json:"age"`
	// Year 为农历年份
	Year int `json:"year"`
}

// MonthlyListItem 为流月列表的一项。
type MonthlyListItem struct {
	// HoroscopeScope 为该流月的通用运限字段
	HoroscopeScope
	// Age 为该流月对应的虚岁
	Age int `json:"age"`
	// Year 为农历年份
	Year int `json:"year"`
	// Month 为农历月份，正月为 1；闰月与同号常规月的 Month 相同，由 IsLeapMonth 区分
	Month int `json:"month"`
	// IsLeapMonth 表示该项是否闰月
	IsLeapMonth bool `json:"isLeapMonth"`
	// Part 为分段标识（MonthPart* 常量）：整月、闰月前半或闰月后半
	Part string `json:"part"`
	// DayRange 为该段覆盖的农历日区间 [起, 止]，含两端
	DayRange [2]int `json:"dayRange"`
}

// Have 判断两个夹宫**合计**是否包含指定的所有星耀——不要求同在一宫。
// 星耀接受 keys.go 常量或当前语言的星名。
func (f *FlankingPalaces) Have(stars ...string) bool {
	ids := f.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; !ok {
			return false
		}
	}
	return true
}

// HaveOneOf 判断两个夹宫合计是否包含指定星耀中的至少一颗。
func (f *FlankingPalaces) HaveOneOf(stars ...string) bool {
	ids := f.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return true
		}
	}
	return false
}

// NotHave 判断两个夹宫是否一颗都不包含指定星耀。
func (f *FlankingPalaces) NotHave(stars ...string) bool {
	ids := f.allStarIdentifiers()
	for _, s := range stars {
		if _, ok := ids[s]; ok {
			return false
		}
	}
	return true
}

// HaveMutagen 判断两个夹宫中是否有任一宫带指定四化（传 MutagenLu 等常量）。
func (f *FlankingPalaces) HaveMutagen(mutagenKey string) bool {
	if f == nil {
		return false
	}
	return f.Previous.HasMutagen(mutagenKey) || f.Next.HasMutagen(mutagenKey)
}

// NotHaveMutagen 判断两个夹宫是否都没有指定四化。
func (f *FlankingPalaces) NotHaveMutagen(mutagenKey string) bool {
	return !f.HaveMutagen(mutagenKey)
}

// allStarIdentifiers 汇集两个夹宫内所有星耀的 key 与翻译名。
func (f *FlankingPalaces) allStarIdentifiers() map[string]struct{} {
	if f == nil {
		return map[string]struct{}{}
	}
	return mergeStarIdentifiers(f.Previous, f.Next)
}

// FlankingPalaces 取指定宫位的夹宫；target 按宫名标识或宫位索引寻址。
//
// 返回的两宫是本盘上的宫位，Palace 的飞化、三方四正等关系查询照常可用。
func (a *Astrolabe) FlankingPalaces(target PalaceTarget) (*FlankingPalaces, error) {
	return a.FlankingPalacesContext(context.Background(), target)
}

// FlankingPalacesContext 为 FlankingPalaces 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) FlankingPalacesContext(ctx context.Context, target PalaceTarget) (*FlankingPalaces, error) {
	if a == nil {
		return nil, invalidArgument("flankingPalaces: nil astrolabe")
	}
	payload := a.textPayload("flankingPalaces")
	target.apply(payload)
	// 内核返回两宫的完整宫位数据，这里只取盘上索引再回本盘取宫：本盘的 Palace 带
	// 反向引用，宫位间的关系查询依赖它，重新解出来的宫位对象没有。
	var out struct {
		Previous struct {
			Index int `json:"index"`
		} `json:"previous"`
		Next struct {
			Index int `json:"index"`
		} `json:"next"`
	}
	if err := utilQueryContext(ctx, payload, &out); err != nil {
		return nil, err
	}
	return &FlankingPalaces{
		Previous: a.PalaceByIndex(out.Previous.Index),
		Next:     a.PalaceByIndex(out.Next.Index),
	}, nil
}

// DecadalList 返回本盘十二个大限，按起运先后排列，第 0 项为第一个大限。
func (a *Astrolabe) DecadalList() ([]DecadalListItem, error) {
	return a.DecadalListContext(context.Background())
}

// DecadalListContext 为 DecadalList 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) DecadalListContext(ctx context.Context) ([]DecadalListItem, error) {
	if a == nil {
		return nil, invalidArgument("decadalList: nil astrolabe")
	}
	var out []DecadalListItem
	return out, utilQueryContext(ctx, a.textPayload("decadalList"), &out)
}

// YearlyList 返回指定大限内的全部流年，按虚岁先后排列。
// ordinal 为大限序号，0 是第一个大限；越界时返回错误。
func (a *Astrolabe) YearlyList(ordinal int) ([]YearlyListItem, error) {
	return a.YearlyListContext(context.Background(), ordinal)
}

// YearlyListContext 为 YearlyList 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) YearlyListContext(ctx context.Context, ordinal int) ([]YearlyListItem, error) {
	return a.yearlyList(ctx, "decadalOrdinal", ordinal)
}

// YearlyListByPalace 返回指定本命宫所在大限内的全部流年，按虚岁先后排列。
// nameKey 为本命宫名标识（PalaceSoul 等常量）；空串或未知标识返回错误，
// 不会退回某个默认大限。
func (a *Astrolabe) YearlyListByPalace(nameKey string) ([]YearlyListItem, error) {
	return a.YearlyListByPalaceContext(context.Background(), nameKey)
}

// YearlyListByPalaceContext 为 YearlyListByPalace 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) YearlyListByPalaceContext(ctx context.Context, nameKey string) ([]YearlyListItem, error) {
	return a.yearlyList(ctx, "palaceKey", nameKey)
}

// yearlyList 按单一定位键发起流年列表查询：内核要求 decadalOrdinal 与 palaceKey
// 二选一，只写调用方给的那一个，另一个缺省时由内核报错。
func (a *Astrolabe) yearlyList(ctx context.Context, key string, value any) ([]YearlyListItem, error) {
	if a == nil {
		return nil, invalidArgument("yearlyList: nil astrolabe")
	}
	payload := a.textPayload("yearlyList")
	payload[key] = value
	var out []YearlyListItem
	return out, utilQueryContext(ctx, payload, &out)
}

// MonthlyList 返回指定农历年的全部流月，按月份先后排列。
//
// fixLeap 决定闰月是否拆段，与排盘的 fixLeap 无关（后者管安星，这里管分段）：
// 为真时闰月拆成前后半月两项（该年共 14 项），为假时闰月整月一项（共 13 项），
// 无闰月的年份恒为 12 项。传 nil 取内核默认（真），用 Bool(false) 关掉。
func (a *Astrolabe) MonthlyList(lunarYear int, fixLeap *bool) ([]MonthlyListItem, error) {
	return a.MonthlyListContext(context.Background(), lunarYear, fixLeap)
}

// MonthlyListContext 为 MonthlyList 的 Context 变体；ctx 用于取消等待 wasm 实例。
func (a *Astrolabe) MonthlyListContext(ctx context.Context, lunarYear int, fixLeap *bool) ([]MonthlyListItem, error) {
	if a == nil {
		return nil, invalidArgument("monthlyList: nil astrolabe")
	}
	payload := a.textPayload("monthlyList")
	payload["year"] = lunarYear
	if fixLeap != nil {
		payload["monthFixLeap"] = *fixLeap
	}
	var out []MonthlyListItem
	return out, utilQueryContext(ctx, payload, &out)
}

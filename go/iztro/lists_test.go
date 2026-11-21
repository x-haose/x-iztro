package iztro

import (
	"errors"
	"reflect"
	"strings"
	"testing"
)

// 夹宫、三个运限列表与按农历年、农历月取运限的行为测试：项数、字段自洽、两种大限定位等价、
// 定位缺省报错，以及重排上下文与运限目标是否随查询转发。

// listChart 排出测试用盘：2000-8-16 寅时女命，fixLeap 决定闰月是否拆段。
func listChart(t *testing.T, fixLeap bool) *Astrolabe {
	t.Helper()
	a, err := BySolar("2000-8-16", 2, GenderFemale, fixLeap, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	return a
}

func TestFlankingPalaces(t *testing.T) {
	a := listChart(t, true)
	soul := a.Palace(PalaceSoul)
	if soul == nil {
		t.Fatal("命宫取不到")
	}

	flanking, err := a.FlankingPalaces(PalaceTarget{Key: PalaceSoul})
	if err != nil {
		t.Fatal(err)
	}
	if flanking.Previous != a.PalaceByIndex(soul.Index-1) {
		t.Errorf("前一宫应为索引 %d 的宫位，实际 %d", (soul.Index+11)%12, flanking.Previous.Index)
	}
	if flanking.Next != a.PalaceByIndex(soul.Index+1) {
		t.Errorf("后一宫应为索引 %d 的宫位，实际 %d", (soul.Index+1)%12, flanking.Next.Index)
	}
	// 返回的是本盘宫位，关系查询照常可用
	if flanking.Previous.Astrolabe() != a {
		t.Error("夹宫未挂回本盘，Palace 的关系查询会失效")
	}

	// 按索引寻址与按宫名寻址取到同一对宫位
	byIndex, err := a.FlankingPalaces(PalaceTarget{Index: soul.Index})
	if err != nil {
		t.Fatal(err)
	}
	if *byIndex != *flanking {
		t.Error("按索引与按宫名寻址的夹宫不一致")
	}

	// 索引 0（寅宫）回绕：前一宫是索引 11
	wrap, err := a.FlankingPalaces(PalaceTarget{Index: 0})
	if err != nil {
		t.Fatal(err)
	}
	if wrap.Previous.Index != 11 || wrap.Next.Index != 1 {
		t.Errorf("寅宫夹宫应为 [11, 1]，实际 [%d, %d]", wrap.Previous.Index, wrap.Next.Index)
	}

	if _, err := a.FlankingPalaces(PalaceTarget{Key: "noSuchPalace"}); err == nil {
		t.Error("未知宫位标识应报错")
	}
}

func TestDecadalList(t *testing.T) {
	a := listChart(t, true)
	list, err := a.DecadalList()
	if err != nil {
		t.Fatal(err)
	}
	if len(list) != 12 {
		t.Fatalf("大限应为 12 项，实际 %d", len(list))
	}
	birthLunarYear := list[0].YearRange[0] - list[0].AgeRange[0] + 1
	for i, item := range list {
		if i > 0 && item.AgeRange[0] != list[i-1].AgeRange[1]+1 {
			t.Errorf("大限[%d] 起始虚岁 %d 与上一限止岁 %d 不衔接", i, item.AgeRange[0], list[i-1].AgeRange[1])
		}
		if item.AgeRange[1]-item.AgeRange[0] != 9 {
			t.Errorf("大限[%d] 跨度应为 10 年，实际 %v", i, item.AgeRange)
		}
		// 农历年份与虚岁同一把尺子：虚岁 1 即出生农历年
		if item.YearRange[0] != birthLunarYear+item.AgeRange[0]-1 ||
			item.YearRange[1] != birthLunarYear+item.AgeRange[1]-1 {
			t.Errorf("大限[%d] 年份 %v 与虚岁 %v 不对应", i, item.YearRange, item.AgeRange)
		}
		if a.PalaceByIndex(item.Index).NameKey != item.PalaceNameKey {
			t.Errorf("大限[%d] 本命宫名 %s 与索引 %d 上的宫位不符", i, item.PalaceNameKey, item.Index)
		}
		// 内嵌的通用运限字段跟着解出来了
		if item.NameKey != ScopeDecadal || len(item.PalaceNameKeys) != 12 || len(item.MutagenStarKeys) != 4 {
			t.Errorf("大限[%d] 通用运限字段缺失：nameKey=%q 宫名 %d 项 四化 %d 项",
				i, item.NameKey, len(item.PalaceNameKeys), len(item.MutagenStarKeys))
		}
	}
}

func TestYearlyList(t *testing.T) {
	a := listChart(t, true)
	decadals, err := a.DecadalList()
	if err != nil {
		t.Fatal(err)
	}

	list, err := a.YearlyList(1)
	if err != nil {
		t.Fatal(err)
	}
	if len(list) != 10 {
		t.Fatalf("流年应为 10 项，实际 %d", len(list))
	}
	for i, item := range list {
		if item.Age != decadals[1].AgeRange[0]+i {
			t.Errorf("流年[%d] 虚岁 %d 与大限区间 %v 不符", i, item.Age, decadals[1].AgeRange)
		}
		if item.NameKey != ScopeYearly {
			t.Errorf("流年[%d] 层级标识应为 %s，实际 %s", i, ScopeYearly, item.NameKey)
		}
	}

	// 两种定位方式指向同一个大限时结果必须一致
	byPalace, err := a.YearlyListByPalace(decadals[1].PalaceNameKey)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(byPalace, list) {
		t.Error("按大限序号与按本命宫名定位的流年列表不一致")
	}

	if _, err := a.YearlyListByPalace(""); err == nil {
		t.Error("定位缺省应报错，不应退回某个默认大限")
	}
	if _, err := a.YearlyList(12); err == nil {
		t.Error("大限序号越界应报错")
	}
}

func TestMonthlyList(t *testing.T) {
	a := listChart(t, true)

	// 2020 农历年闰四月：拆段与否只看传入的 fixLeap，与排盘的 fixLeap 无关
	cases := []struct {
		name    string
		fixLeap *bool
		want    int
	}{
		{"缺省取内核默认（拆段）", nil, 14},
		{"显式拆段", Bool(true), 14},
		{"显式不拆段", Bool(false), 13},
	}
	for _, c := range cases {
		t.Run(c.name, func(t *testing.T) {
			list, err := a.MonthlyList(2020, c.fixLeap)
			if err != nil {
				t.Fatal(err)
			}
			if len(list) != c.want {
				t.Fatalf("闰年流月应为 %d 项，实际 %d", c.want, len(list))
			}

			var leap []MonthlyListItem
			for i, item := range list {
				if item.Year != 2020 || item.Month < 1 || item.Month > 12 {
					t.Errorf("流月[%d] 年月越界：%d 年 %d 月", i, item.Year, item.Month)
				}
				if item.NameKey != ScopeMonthly {
					t.Errorf("流月[%d] 层级标识应为 %s，实际 %s", i, ScopeMonthly, item.NameKey)
				}
				if item.IsLeapMonth {
					leap = append(leap, item)
				} else if item.Part != MonthPartNormal || item.DayRange[0] != 1 {
					t.Errorf("流月[%d] 常规月应是整月一段，实际 part=%s days=%v", i, item.Part, item.DayRange)
				}
			}

			if c.want == 14 {
				if len(leap) != 2 || leap[0].Part != MonthPartFirst || leap[1].Part != MonthPartSecond {
					t.Fatalf("拆段时闰月应为 first/second 两项，实际 %d 项", len(leap))
				}
				if leap[0].DayRange != [2]int{1, 15} || leap[1].DayRange[0] != 16 {
					t.Errorf("闰月分段日区间应为 [1 15] 与 [16 月末]，实际 %v 与 %v", leap[0].DayRange, leap[1].DayRange)
				}
			} else if len(leap) != 1 || leap[0].Part != MonthPartNormal {
				t.Errorf("不拆段时闰月应为整月一项，实际 %d 项", len(leap))
			}
		})
	}

	// 无闰月的农历年，拆段开关无从生效
	for _, fixLeap := range []*bool{Bool(true), Bool(false)} {
		plain, err := a.MonthlyList(2021, fixLeap)
		if err != nil {
			t.Fatal(err)
		}
		if len(plain) != 12 {
			t.Errorf("fixLeap=%v 时无闰月的农历年应为 12 项，实际 %d", *fixLeap, len(plain))
		}
	}
}

// TestListsFollowRearrange 守住重排上下文的转发：重排改变五行局与十二宫名，
// 列表查询漏带 fromStem/fromBranch 就会拿原盘的答案冒充重排盘的。
func TestListsFollowRearrange(t *testing.T) {
	a := listChart(t, true)
	b, err := a.Rearranged(StemJia, BranchZi)
	if err != nil {
		t.Fatal(err)
	}
	if b.FiveElementsClassKey == a.FiveElementsClassKey {
		t.Skip("重排后五行局未变，本用例区分不出两张盘")
	}

	origin, err := a.DecadalList()
	if err != nil {
		t.Fatal(err)
	}
	rearranged, err := b.DecadalList()
	if err != nil {
		t.Fatal(err)
	}
	if origin[0].AgeRange == rearranged[0].AgeRange {
		t.Errorf("重排盘的大限起运虚岁应随五行局改变，两盘同为 %v——重排上下文没转发", origin[0].AgeRange)
	}

	// 夹宫按重排后的十二宫名定位：财帛宫在两盘上落在不同格，夹宫应跟着走
	flanking, err := b.FlankingPalaces(PalaceTarget{Key: PalaceWealth})
	if err != nil {
		t.Fatal(err)
	}
	if flanking.Previous.Index != (b.Palace(PalaceWealth).Index+11)%12 {
		t.Errorf("重排盘财帛宫在索引 %d，夹宫的前一宫却是 %d——重排上下文没转发",
			b.Palace(PalaceWealth).Index, flanking.Previous.Index)
	}
}

// TestFlankingPalacesJudgements 钉死「两宫合计」语义：判定在前后两宫合起来的
// 星耀集合上做，写成「某一宫含全部」会在这里红。
func TestFlankingPalacesJudgements(t *testing.T) {
	a := listChart(t, true)

	// 找一格，其前后两宫各自有星——合计语义只有这种格局能与「单宫含全部」区分开
	var f *FlankingPalaces
	var prevStar, nextStar string
	for i := 0; i < 12; i++ {
		got, err := a.FlankingPalaces(PalaceTarget{Index: i})
		if err != nil {
			t.Fatal(err)
		}
		p, n := firstStar(got.Previous), firstStar(got.Next)
		if p != "" && n != "" && p != n {
			f, prevStar, nextStar = got, p, n
			break
		}
	}
	if f == nil {
		t.Fatal("盘上找不到前后两宫各自有星的格，本用例区分不出合计语义")
	}

	if !f.Have(prevStar, nextStar) {
		t.Errorf("Have(%s, %s) 应为真：两星分处前后两宫，合计即算含全", prevStar, nextStar)
	}
	// 反证：任何单独一宫都不同时含这两颗，Have 若按单宫判就会假
	if f.Previous.Has(prevStar, nextStar) || f.Next.Has(prevStar, nextStar) {
		t.Fatalf("%s 与 %s 落在了同一宫，本用例失去区分力", prevStar, nextStar)
	}

	if !f.HaveOneOf(prevStar, "noSuchStar") {
		t.Errorf("HaveOneOf 含 %s 应为真", prevStar)
	}
	if f.HaveOneOf("noSuchStar") {
		t.Error("HaveOneOf 只给不存在的星应为假")
	}
	if f.Have(prevStar, "noSuchStar") {
		t.Error("Have 含不存在的星应为假")
	}
	if !f.NotHave("noSuchStar") {
		t.Error("NotHave 只给不存在的星应为真")
	}
	if f.NotHave(prevStar) || f.NotHave(nextStar) {
		t.Error("NotHave 给在盘上的星应为假")
	}

	// 四化：任一宫带该化即为真，与逐宫判定一致
	for _, m := range []string{MutagenLu, MutagenQuan, MutagenKe, MutagenJi} {
		want := f.Previous.HasMutagen(m) || f.Next.HasMutagen(m)
		if f.HaveMutagen(m) != want {
			t.Errorf("HaveMutagen(%s) 应为 %v", m, want)
		}
		if f.NotHaveMutagen(m) == want {
			t.Errorf("NotHaveMutagen(%s) 应为 %v", m, !want)
		}
	}
}

// TestHoroscopeOfYear 校验按农历年取运限：流年是该农历年的（公历同年 1 月 1 日还在上一个农历年），
// 越界年份报错，返回的运限再发起计算时转发的仍是年份、文本只写到流年；流年一览的区间颠倒或年份越界报错。
func TestHoroscopeOfYear(t *testing.T) {
	a := listChart(t, true)

	ofYear, err := a.HoroscopeOfYear(2027)
	if err != nil {
		t.Fatal(err)
	}
	if ofYear.Yearly.HeavenlyStemKey != StemDing || ofYear.Yearly.EarthlyBranchKey != BranchWei {
		t.Errorf("农历 2027 年应为丁未，实际 %s %s", ofYear.Yearly.HeavenlyStemKey, ofYear.Yearly.EarthlyBranchKey)
	}
	newYearsDay, err := a.Horoscope("2027-1-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	if newYearsDay.Yearly.HeavenlyStemKey != StemBing || newYearsDay.Yearly.EarthlyBranchKey != BranchWu {
		t.Errorf("公历 2027-1-1 应仍在丙午年，实际 %s %s",
			newYearsDay.Yearly.HeavenlyStemKey, newYearsDay.Yearly.EarthlyBranchKey)
	}

	for _, year := range []int{0, 10000} {
		if _, err := a.HoroscopeOfYear(year); !errors.Is(err, ErrInvalidArgument) {
			t.Errorf("年份 %d 越界应报 ErrInvalidArgument，实际 %v", year, err)
		}
	}

	// 查询精度为流年：文本只写到流年，与「六月初一 + 出生时辰（寅时，索引 2）」的运限写到流年同文；
	// 转发 DTO 的 SolarDate 会成为日期目标、写到流时。比流年细的层级报错
	if got := ofYear.Precision(); got != ScopeYearly {
		t.Errorf("年份运限的查询精度应为 %s，实际 %s", ScopeYearly, got)
	}
	text, err := ofYear.ToText()
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(text, "## 流年 · ") || strings.Contains(text, "## 流月 · ") {
		t.Errorf("年份运限的文本应写到流年为止:\n%s", text)
	}
	sameHour, err := a.Horoscope(ofYear.SolarDate, 2)
	if err != nil {
		t.Fatal(err)
	}
	if want, err := sameHour.ToTextWith(TextOptions{Through: ScopeYearly}); err != nil || text != want {
		t.Errorf("年份运限的文本应与六月初一出生时辰的运限写到流年同文（err=%v）", err)
	}
	if _, err := ofYear.ToTextWith(TextOptions{Through: ScopeMonthly}); !errors.Is(err, ErrInvalidArgument) {
		t.Errorf("年份运限写到流月应报 ErrInvalidArgument，实际 %v", err)
	}

	for _, r := range [][2]int{{2012, 2001}, {0, 1}, {9999, 10000}} {
		if _, err := a.YearlyOverviewToText(r[0], r[1]); !errors.Is(err, ErrInvalidArgument) {
			t.Errorf("流年一览 %d-%d 应报 ErrInvalidArgument，实际 %v", r[0], r[1], err)
		}
	}
}

// TestHoroscopeOfMonth 校验按农历月取运限：流月与流月列表里同月号的常规月那项一致，
// 年份、月号越界报错，返回的运限再发起计算时转发的仍是年份与月号。
func TestHoroscopeOfMonth(t *testing.T) {
	a := listChart(t, true)

	list, err := a.MonthlyList(2025, nil)
	if err != nil {
		t.Fatal(err)
	}
	for month := 1; month <= 12; month++ {
		ofMonth, err := a.HoroscopeOfMonth(2025, month)
		if err != nil {
			t.Fatal(err)
		}
		var want *MonthlyListItem
		for i := range list {
			if list[i].Month == month && !list[i].IsLeapMonth {
				want = &list[i]
			}
		}
		if want == nil {
			t.Fatalf("流月列表里没有 %d 月的常规月", month)
		}
		got := ofMonth.Monthly
		if got.Index != want.Index || got.HeavenlyStemKey != want.HeavenlyStemKey ||
			got.EarthlyBranchKey != want.EarthlyBranchKey ||
			!reflect.DeepEqual(got.MutagenStarKeys, want.MutagenStarKeys) {
			t.Errorf("%d 月的流月与流月列表不一致: %+v vs %+v", month, got, want.HoroscopeScope)
		}
	}

	for _, ym := range [][2]int{{2025, 0}, {2025, 13}, {2025, -1}, {0, 1}, {10000, 1}} {
		if _, err := a.HoroscopeOfMonth(ym[0], ym[1]); !errors.Is(err, ErrInvalidArgument) {
			t.Errorf("%d 年 %d 月越界应报 ErrInvalidArgument，实际 %v", ym[0], ym[1], err)
		}
	}

	// 农历 2025 年七月初一是 2025-8-23。再发起的计算取到流月，与「该日 + 出生时辰（寅时，索引 2）」的运限
	// 取到流月相同：只转发年份会退成六月初一、只取到流年，转发 DTO 的 SolarDate 会取到流时
	ofMonth, err := a.HoroscopeOfMonth(2025, 7)
	if err != nil {
		t.Fatal(err)
	}
	if ofMonth.SolarDate != "2025-8-23" {
		t.Fatalf("农历 2025 年七月初一应为 2025-8-23，实际 %s", ofMonth.SolarDate)
	}
	sameHour, err := a.Horoscope("2025-8-23", 2)
	if err != nil {
		t.Fatal(err)
	}
	if got := ofMonth.Precision(); got != ScopeMonthly {
		t.Errorf("月份运限的查询精度应为 %s，实际 %s", ScopeMonthly, got)
	}
	text := func(h *Horoscope, opts TextOptions) string {
		t.Helper()
		out, err := h.ToTextWith(opts)
		if err != nil {
			t.Fatal(err)
		}
		return out
	}

	monthly := text(ofMonth, TextOptions{})
	if monthly != text(sameHour, TextOptions{Through: ScopeMonthly}) {
		t.Error("月份运限的文本应与该月初一出生时辰的运限写到流月同文")
	}
	for i, heading := range []string{"## 流月 · ", "## 流日 · ", "## 流时 · "} {
		if got, want := strings.Contains(monthly, heading), i < 1; got != want {
			t.Errorf("写到流月时 %q 是否出现应为 %v", heading, want)
		}
	}

	gotHits, err := ofMonth.Patterns(ScopeMonthly, nil)
	if err != nil {
		t.Fatal(err)
	}
	wantHits, err := sameHour.Patterns(ScopeMonthly, nil)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(gotHits, wantHits) {
		t.Error("月份运限的流月格局应与该月初一的运限相同")
	}
	gotHitsText, err := ofMonth.PatternsToText(ScopeMonthly, nil)
	if err != nil {
		t.Fatal(err)
	}
	if wantHitsText, err := sameHour.PatternsToText(ScopeMonthly, nil); err != nil || gotHitsText != wantHitsText {
		t.Errorf("月份运限的流月格局文本应与该月初一的运限同文（err=%v）", err)
	}
	gotPack, err := BuiltinKnowledge().ForHoroscope(ofMonth, nil)
	if err != nil {
		t.Fatal(err)
	}
	wantPack, err := BuiltinKnowledge().ForHoroscope(sameHour, nil)
	if err != nil {
		t.Fatal(err)
	}
	// 按盘取材取到流月：有该月的流耀，没有代表日流日的流耀；同一天按日期取材两者都有
	for _, c := range []struct {
		scope HoroscopeScope
		want  bool
	}{{ofMonth.Monthly, true}, {ofMonth.Daily, false}} {
		for _, group := range c.scope.Stars {
			for _, s := range group {
				if _, got := gotPack.Stars[s.Key]; got != c.want {
					t.Errorf("月份运限的子包里 %s 是否出现应为 %v", s.Key, c.want)
				}
				if _, ok := wantPack.Stars[s.Key]; !ok {
					t.Errorf("该月初一按日期取材的子包里应有 %s", s.Key)
				}
			}
		}
	}

	if _, err := a.MonthlyListToText(99999); !errors.Is(err, ErrInvalidArgument) {
		t.Errorf("流月一览的年份越界应报 ErrInvalidArgument，实际 %v", err)
	}
}

// firstStar 取宫内第一颗星的 key；空宫返回空串。
func firstStar(p *Palace) string {
	for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
		if len(group) > 0 {
			return group[0].Key
		}
	}
	return ""
}

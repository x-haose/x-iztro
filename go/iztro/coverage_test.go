package iztro

import "testing"

// 判断方法与工具函数的取值测试。
//
// 断言锚定在固定盘 2000-8-16 寅时 女 上，取值可对照
// tests/golden/prompt_snapshots/astrolabe_zh-CN.txt 逐宫核对。

// coverageChart 提供本组测试共用的本命盘。
func coverageChart(t *testing.T) *Astrolabe {
	t.Helper()
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	return chart
}

// TestPalaceMutagenJudgements 校验宫位与三方四正的四化判断。
func TestPalaceMutagenJudgements(t *testing.T) {
	chart := coverageChart(t)

	// 财帛（索引 0，戊寅）有武曲化权，没有化忌
	wealth := chart.PalaceByIndex(0)
	if wealth.NameKey != PalaceWealth {
		t.Fatalf("索引 0 是 %s，期望财帛", wealth.NameKey)
	}
	if !wealth.HasMutagen(MutagenQuan) {
		t.Error("财帛应有化权")
	}
	if wealth.HasMutagen(MutagenJi) || !wealth.NotHaveMutagen(MutagenJi) {
		t.Error("财帛不应有化忌")
	}

	// 命宫三方四正为命宫、迁移、财帛、官禄，含财帛的化权、不含疾厄的化忌
	sp := chart.SurroundedPalaces(PalaceSoul)
	if !sp.HaveMutagen(MutagenQuan) {
		t.Error("命宫三方四正应有化权")
	}
	if sp.HaveMutagen(MutagenJi) || !sp.NotHaveMutagen(MutagenJi) {
		t.Error("命宫三方四正不应有化忌（天同化忌在疾厄）")
	}
	if !chart.IsSurroundedOneOf(PalaceSoul, StarJumenMaj, StarTianfuMaj) {
		t.Error("命宫三方四正应含天府")
	}
	if chart.IsSurroundedOneOf(PalaceSoul, StarJumenMaj) {
		t.Error("巨门在疾厄，不在命宫三方四正")
	}
}

// TestPalaceSelfMutagen 校验自化判断。
func TestPalaceSelfMutagen(t *testing.T) {
	chart := coverageChart(t)

	// 迁移（索引 10，戊子）坐贪狼，戊干化禄即贪狼，故禄自化
	travel := chart.PalaceByIndex(10)
	if travel.NameKey != PalaceSurface {
		t.Fatalf("索引 10 是 %s，期望迁移", travel.NameKey)
	}
	if !travel.SelfMutaged(MutagenLu) {
		t.Error("迁移应有禄自化")
	}
	if !travel.SelfMutagedOneOf() {
		t.Error("不传四化时应检查全部四化，迁移有禄自化")
	}
	if travel.NotSelfMutaged() {
		t.Error("迁移有自化，NotSelfMutaged 应为 false")
	}

	// 财帛（戊寅）不含戊干的四化四星，无任何自化
	wealth := chart.PalaceByIndex(0)
	if wealth.SelfMutagedOneOf() {
		t.Error("财帛不应有自化")
	}
	if !wealth.NotSelfMutaged() {
		t.Error("财帛的 NotSelfMutaged 应为 true")
	}
}

// TestStarBrightnessJudgement 校验星耀亮度判断。
func TestStarBrightnessJudgement(t *testing.T) {
	chart := coverageChart(t)

	wuqu, _ := chart.Star(StarWuquMaj)
	if wuqu == nil {
		t.Fatal("武曲应在盘上")
	}
	if !wuqu.WithBrightness(BrightnessDe) {
		t.Errorf("武曲亮度 %s，期望得", wuqu.BrightnessKey)
	}
	if wuqu.WithBrightness(BrightnessMiao, BrightnessXian) {
		t.Error("武曲不是庙也不是陷")
	}

	// 杂耀没有亮度，任何亮度判断都为 false
	jieshen, _ := chart.Star(StarJieshen)
	if jieshen == nil {
		t.Fatal("解神应在盘上")
	}
	if jieshen.WithBrightness(BrightnessMiao) || jieshen.WithBrightness(BrightnessWang) {
		t.Error("无亮度的杂耀不应匹配任何亮度")
	}
}

// TestHoroscopeScopeItem 校验按层级取运限项。
func TestHoroscopeScopeItem(t *testing.T) {
	h, err := coverageChart(t).Horoscope("2024-10-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	pairs := []struct {
		scope string
		want  *HoroscopeScope
	}{
		{ScopeDecadal, &h.Decadal},
		{ScopeYearly, &h.Yearly},
		{ScopeMonthly, &h.Monthly},
		{ScopeDaily, &h.Daily},
		{ScopeHourly, &h.Hourly},
	}
	for _, p := range pairs {
		if got := h.ScopeItem(p.scope); got != p.want {
			t.Errorf("ScopeItem(%s) 未取到对应层级", p.scope)
		}
	}
	if h.ScopeItem(ScopeOrigin) != nil {
		t.Error("本命不是运限层级，ScopeItem 应返回 nil")
	}
	if h.ScopeItem("没有这个层级") != nil {
		t.Error("未知层级应返回 nil")
	}

	// 流年层级下按宫名反查索引，与 Palace 的结果一致
	idx := h.Yearly.PalaceIndexByName(PalaceSoul)
	if idx < 0 {
		t.Fatal("流年层级应有命宫")
	}
	if h.Palace(PalaceSoul, ScopeYearly).Index != idx {
		t.Error("PalaceIndexByName 与 Palace 结果不一致")
	}
	if h.Yearly.PalaceIndexByName("没有这个宫") != -1 {
		t.Error("查不到的宫名应返回 -1")
	}
}

// TestAgeIndexMatchesChart 校验小限起宫与盘上小限岁数一致。
func TestAgeIndexMatchesChart(t *testing.T) {
	chart := coverageChart(t)
	yearBranch := chart.RawDates.ChineseDate.YearlyKeys[1]

	got, err := GetAgeIndex(yearBranch)
	if err != nil {
		t.Fatal(err)
	}
	want := -1
	for i := range chart.Palaces {
		if len(chart.Palaces[i].Ages) > 0 && chart.Palaces[i].Ages[0] == 1 {
			want = i
			break
		}
	}
	if want < 0 {
		t.Fatal("盘上应有虚岁 1 的宫位")
	}
	if got != want {
		t.Errorf("GetAgeIndex(%s) = %d，盘上虚岁 1 落在 %d", yearBranch, got, want)
	}
}

// TestFixLunarDayIndex 校验农历日索引修正；晚子时属次日故不减一。
func TestFixLunarDayIndex(t *testing.T) {
	chart := coverageChart(t)
	day := chart.RawDates.LunarDate.LunarDay
	if day != 17 {
		t.Fatalf("农历日 %d，期望 17（二〇〇〇年七月十七）", day)
	}

	got, err := FixLunarDayIndex(day, 2)
	if err != nil {
		t.Fatal(err)
	}
	if got != day-1 {
		t.Errorf("FixLunarDayIndex(%d, 2) = %d，期望 %d", day, got, day-1)
	}
	got, err = FixLunarDayIndex(day, 12)
	if err != nil {
		t.Fatal(err)
	}
	if got != day {
		t.Errorf("晚子时 FixLunarDayIndex(%d, 12) = %d，期望 %d", day, got, day)
	}
}

// TestEarthlyBranchToPalaceIndexMatchesChart 校验地支转宫位索引与盘上宫位地支一致。
func TestEarthlyBranchToPalaceIndexMatchesChart(t *testing.T) {
	chart := coverageChart(t)
	for i := range chart.Palaces {
		p := &chart.Palaces[i]
		got, err := EarthlyBranchToPalaceIndex(p.EarthlyBranchKey)
		if err != nil {
			t.Fatal(err)
		}
		if got != p.Index {
			t.Errorf("%s 的索引为 %d，EarthlyBranchToPalaceIndex 给出 %d", p.EarthlyBranchKey, p.Index, got)
		}
	}
}

// TestTimeToIndexTable 校验小时到时辰索引的完整映射。
func TestTimeToIndexTable(t *testing.T) {
	// 0 点为早子时，23 点为晚子时，其余每两小时一个时辰
	want := []uint8{0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12}
	for hour := 0; hour < 24; hour++ {
		got, err := TimeToIndex(uint8(hour))
		if err != nil {
			t.Fatalf("TimeToIndex(%d): %v", hour, err)
		}
		if got != want[hour] {
			t.Errorf("TimeToIndex(%d) = %d，期望 %d", hour, got, want[hour])
		}
	}
}

// TestFixIndexWrapping 校验索引回绕，含 max 传 0 的默认值语义。
func TestFixIndexWrapping(t *testing.T) {
	cases := []struct{ index, max, want int }{
		{0, 12, 0},
		{12, 12, 0},
		{-1, 12, 11},
		{-12, 12, 0},
		{5, 12, 5},
		{13, 12, 1},
		{-13, 12, 11},
		{5, 3, 2},
		// max 传 0 取默认值 12
		{13, 0, 1},
		{-1, 0, 11},
	}
	for _, c := range cases {
		got, err := FixIndex(c.index, c.max)
		if err != nil {
			t.Fatalf("FixIndex(%d, %d): %v", c.index, c.max, err)
		}
		if got != c.want {
			t.Errorf("FixIndex(%d, %d) = %d，期望 %d", c.index, c.max, got, c.want)
		}
	}
	for _, c := range []struct{ index, want int }{{0, 0}, {12, 0}, {-1, 11}, {25, 1}} {
		if got := FixIndex12(c.index); got != c.want {
			t.Errorf("FixIndex12(%d) = %d，期望 %d", c.index, got, c.want)
		}
	}
}

// TestFiveElementsClassNumber 校验五行局局数取值与常量表一致。
func TestFiveElementsClassNumber(t *testing.T) {
	constants, err := GetConstants()
	if err != nil {
		t.Fatal(err)
	}
	if len(constants.FiveElementsClass) != 5 {
		t.Fatalf("常量表给出 %d 个五行局，期望 5", len(constants.FiveElementsClass))
	}
	// 本地表不得与 wasm 侧的常量表漂移
	for key, n := range constants.FiveElementsClass {
		if got := FiveElementsClassNumber(key); got != n {
			t.Errorf("FiveElementsClassNumber(%s) = %d，常量表为 %d", key, got, n)
		}
	}
	if constants.FiveElementsClass[ClassWood3rd] != 3 {
		t.Errorf("木三局局数 %d，期望 3", constants.FiveElementsClass[ClassWood3rd])
	}
	if got := FiveElementsClassNumber("没有这个局"); got != 0 {
		t.Errorf("未知五行局应返回 0，实际 %d", got)
	}
	// 与盘上的五行局对上：本盘为木三局
	if got := FiveElementsClassNumber(coverageChart(t).FiveElementsClassKey); got != 3 {
		t.Errorf("本盘五行局局数 %d，期望 3", got)
	}
}

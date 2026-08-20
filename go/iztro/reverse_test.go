package iztro

import "testing"

// 反推：八字与星盘特征的往返一致性、入参校验（与 Rust 侧 tests/reverse.rs 同一批断言面）。

func TestSolarDatesByBaziRoundtrip(t *testing.T) {
	a, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	cd := a.RawDates.ChineseDate
	got, err := SolarDatesByBazi(
		Pillar(cd.YearlyKeys), Pillar(cd.MonthlyKeys), Pillar(cd.DailyKeys), Pillar(cd.HourlyKeys),
		1900, 2100, nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(got) != 3 {
		t.Fatalf("60 年周期应有 3 个解，got %v", got)
	}
	found := false
	for _, c := range got {
		if c.SolarDate == "2000-8-16" && c.TimeIndex == 2 {
			found = true
		}
		// 每个候选正排后的四柱必须与目标完全一致（四柱逐柱比对）
		b, err := BySolar(c.SolarDate, c.TimeIndex, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			t.Fatal(err)
		}
		q := b.RawDates.ChineseDate
		if q.YearlyKeys != cd.YearlyKeys || q.MonthlyKeys != cd.MonthlyKeys ||
			q.DailyKeys != cd.DailyKeys || q.HourlyKeys != cd.HourlyKeys {
			t.Fatalf("candidate %v disagrees", c)
		}
	}
	if !found {
		t.Fatalf("original birth missing: %v", got)
	}
	if _, err := SolarDatesByBazi(
		Pillar{StemJia, BranchChou}, Pillar{StemJia, BranchZi}, Pillar{StemJia, BranchZi}, Pillar{StemJia, BranchZi},
		1900, 2100, nil); err == nil {
		t.Fatal("mismatched polarity must error")
	}
}

func TestReverseChartRoundtrip(t *testing.T) {
	a, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	ziwei, palace := a.Star(StarZiweiMaj)
	if ziwei == nil {
		t.Fatal("ziwei missing")
	}
	// 生年四化条件取自该盘年干实际四化（禄位与忌位），身宫地支取自该盘：
	// 条件都是原盘事实，只会收窄候选、不会排除原生辰，借此走通
	// mutagens 与 bodyBranch 两维的绑定转换（非空值路径）
	mutagens, err := GetMutagensByHeavenlyStem(a.RawDates.ChineseDate.YearlyKeys[0], nil)
	if err != nil {
		t.Fatal(err)
	}
	r, err := ReverseChart(&ReverseCriteria{
		SoulBranch:        a.EarthlyBranchOfSoulPalaceKey,
		BodyBranch:        a.EarthlyBranchOfBodyPalaceKey,
		FiveElementsClass: a.FiveElementsClassKey,
		Stars:             []StarPosition{{Star: StarZiweiMaj, Branch: palace.EarthlyBranchKey}},
		Mutagens:          [4]string{mutagens[0], "", "", mutagens[3]},
		YearRange:         [2]int{1999, 2001},
		FixLeap:           Bool(true),
	}, nil)
	if err != nil {
		t.Fatal(err)
	}
	found := false
	for _, c := range r.Candidates {
		if c.SolarDate == "2000-8-16" && c.TimeIndex == 2 {
			found = true
		}
	}
	if !found || r.Truncated {
		t.Fatalf("original birth missing among %d candidates (truncated=%v)", len(r.Candidates), r.Truncated)
	}
}

func TestReverseChartLimitAndErrors(t *testing.T) {
	r, err := ReverseChart(&ReverseCriteria{
		Stars:   []StarPosition{{Star: StarHuoxingMin, Branch: BranchYin}},
		FixLeap: Bool(true),
		Limit:   10,
	}, nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(r.Candidates) != 10 || !r.Truncated {
		t.Fatalf("limit: got %d truncated=%v", len(r.Candidates), r.Truncated)
	}
	if _, err := ReverseChart(&ReverseCriteria{FixLeap: Bool(true)}, nil); err == nil {
		t.Fatal("empty criteria must error")
	}
	if _, err := ReverseChart(nil, nil); err == nil {
		t.Fatal("nil criteria must error")
	}
}

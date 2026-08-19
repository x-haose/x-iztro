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
		// 每个候选正排后的四柱必须与目标一致
		b, err := BySolar(c.SolarDate, c.TimeIndex, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			t.Fatal(err)
		}
		if b.RawDates.ChineseDate.DailyKeys != cd.DailyKeys || b.RawDates.ChineseDate.YearlyKeys != cd.YearlyKeys {
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
	r, err := ReverseChart(&ReverseCriteria{
		SoulBranch:        a.EarthlyBranchOfSoulPalaceKey,
		FiveElementsClass: a.FiveElementsClassKey,
		Stars:             []StarPosition{{Star: StarZiweiMaj, Branch: palace.EarthlyBranchKey}},
		YearRange:         [2]int{1999, 2001},
		FixLeap:           true,
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
		FixLeap: true,
		Limit:   10,
	}, nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(r.Candidates) != 10 || !r.Truncated {
		t.Fatalf("limit: got %d truncated=%v", len(r.Candidates), r.Truncated)
	}
	if _, err := ReverseChart(&ReverseCriteria{FixLeap: true}, nil); err == nil {
		t.Fatal("empty criteria must error")
	}
	if _, err := ReverseChart(nil, nil); err == nil {
		t.Fatal("nil criteria must error")
	}
}

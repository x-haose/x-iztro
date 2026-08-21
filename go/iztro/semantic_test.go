package iztro

import (
	"fmt"
	"strings"
	"testing"
)

// 语义化契约的专项测试：语言无关标识的新增字段、流耀对应表与主星 key 查询。

// TestChildhoodNameKey 校验未起运盘的大限层标识为童限，且运限文本如实报出。
func TestChildhoodNameKey(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	horoscope, err := chart.Horoscope("2001-10-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	if horoscope.Decadal.NameKey != "childhood" {
		t.Fatalf("未起运盘的大限层 NameKey 应为 childhood，实际 %q", horoscope.Decadal.NameKey)
	}
	text, err := horoscope.ToText()
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(text, "童限") {
		t.Errorf("未起运盘的运限文本应含「童限」:\n%s", text)
	}

	// 已起运的目标日期应回到 decadal
	grown, err := chart.Horoscope("2026-8-19", 0)
	if err != nil {
		t.Fatal(err)
	}
	if grown.Decadal.NameKey != "decadal" {
		t.Fatalf("已起运盘的大限层 NameKey 应为 decadal，实际 %q", grown.Decadal.NameKey)
	}
}

// TestSignAndZodiacKeys 校验星盘携带语言无关的星座与生肖标识。
func TestSignAndZodiacKeys(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageEnUS, nil)
	if err != nil {
		t.Fatal(err)
	}
	if chart.SignKey != "leo" {
		t.Errorf("2000-8-16 的 SignKey 应为 leo，实际 %q", chart.SignKey)
	}
	if chart.ZodiacKey != "dragon" {
		t.Errorf("2000 年生的 ZodiacKey 应为 dragon，实际 %q", chart.ZodiacKey)
	}
}

// TestFlowStarCounterparts 校验流耀到本命辅星的对应表完整且取值正确。
func TestFlowStarCounterparts(t *testing.T) {
	m, err := FlowStarCounterparts()
	if err != nil {
		t.Fatal(err)
	}
	if len(m) != 50 {
		t.Fatalf("流耀对应表应有 50 条，实际 %d", len(m))
	}
	if m[StarLiuchang] != StarWenchangMin {
		t.Errorf("流昌应对应文昌，实际 %q", m[StarLiuchang])
	}
}

// TestMajorStarKeys 校验主星 key 查询与同盘命宫主星逐项一致（空宫借对宫的
// 口径由内核统一，非空宫盘上两者必然相同）。
func TestMajorStarKeys(t *testing.T) {
	const (
		date      = "2000-8-16"
		timeIndex = uint8(2)
	)
	chart, err := BySolar(date, timeIndex, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	soul := chart.Palace(PalaceSoul)
	if soul == nil || len(soul.MajorStars) == 0 {
		t.Fatal("该盘命宫应有主星")
	}
	want := make([]string, len(soul.MajorStars))
	for i, s := range soul.MajorStars {
		want[i] = s.Key
	}

	keys, err := MajorStarKeysBySolarDate(date, timeIndex, true, nil)
	if err != nil {
		t.Fatal(err)
	}
	assertKeyList(t, "阳历主星 key", keys, want)

	l := chart.RawDates.LunarDate
	leap := NotLeapMonth
	if l.IsLeap {
		leap = LeapMonthFixed
	}
	lunarKeys, err := MajorStarKeysByLunarDate(
		fmt.Sprintf("%d-%d-%d", l.LunarYear, l.LunarMonth, l.LunarDay), timeIndex, leap, nil)
	if err != nil {
		t.Fatal(err)
	}
	assertKeyList(t, "农历主星 key", lunarKeys, want)
}

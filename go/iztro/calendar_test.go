package iztro

import (
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 历法查询入口对照 lunar-typescript 的金标：读 tests/golden/calendar_api.csv
// （由 generate_calendar_api.mjs 生成，Rust 的 calendar_api.rs、Python 的
// test_calendar_golden.py 与 JS 的 calendar.test.ts 按同一规则重建）。
// 按年重建规范化串，验证 Go 包装的字段映射与闰月入参，并检查每一天公历换回原农历日期。

// calendarStems 为天干标识按序号（甲 = 0）排列，用于写规范化串的干序。
var calendarStems = []string{
	StemJia, StemYi, StemBing, StemDing, StemWu, StemJi, StemGeng, StemXin, StemRen, StemGui,
}

func calendarYearLines(t *testing.T, y int) []string {
	t.Helper()
	ymd := func(d RawSolarDate) string {
		return fmt.Sprintf("%04d-%02d-%02d", d.SolarYear, d.SolarMonth, d.SolarDay)
	}
	flag := func(leap bool) int {
		if leap {
			return 1
		}
		return 0
	}
	months, err := LunarMonths(y)
	if err != nil {
		t.Fatalf("LunarMonths(%d): %v", y, err)
	}
	var lines []string
	for _, m := range months {
		lines = append(lines, fmt.Sprintf("M %d,%d,%d,%s", m.Month, flag(m.IsLeap), m.DayCount, ymd(m.FirstDay)))
	}
	pillar, err := LunarYearPillar(y)
	if err != nil {
		t.Fatalf("LunarYearPillar(%d): %v", y, err)
	}
	stem := -1
	for i, s := range calendarStems {
		if s == pillar[0] {
			stem = i
		}
	}
	lines = append(lines, fmt.Sprintf("P %d,%d", stem, branchOrder[pillar[1]]))
	for _, m := range months {
		for d := 1; d <= m.DayCount; d++ {
			solar, err := LunarToSolar(y, m.Month, d, m.IsLeap)
			if err != nil {
				t.Fatalf("LunarToSolar(%d, %d, %d, %v): %v", y, m.Month, d, m.IsLeap, err)
			}
			if solar.SolarYear <= 9999 {
				back, err := SolarToLunar(solar.SolarYear, solar.SolarMonth, solar.SolarDay)
				want := RawLunarDate{LunarYear: y, LunarMonth: m.Month, LunarDay: d, IsLeap: m.IsLeap}
				if err != nil || back != want {
					t.Fatalf("SolarToLunar(%s) = %+v, %v；应为 %+v", ymd(solar), back, err, want)
				}
			}
			lines = append(lines, fmt.Sprintf("D %d,%d,%d,%s", m.Month, flag(m.IsLeap), d, ymd(solar)))
		}
	}
	return lines
}

func TestCalendarMatchesLunarTypescript(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "golden", "calendar_api.csv"))
	if err != nil {
		t.Fatal(err)
	}
	expected := map[string]string{}
	for _, line := range strings.Split(strings.TrimSpace(string(data)), "\n") {
		year, hash, _ := strings.Cut(line, ",")
		expected[year] = hash
	}
	// 闰月年、无闰月年、缺三月的 237 年、改历的 1582 年、腊月后段落在公历 10000 年的 9999 年
	for _, y := range []int{1, 237, 1582, 2023, 2024, 9999} {
		sum := sha256.Sum256([]byte(strings.Join(calendarYearLines(t, y), "\n")))
		if got := hex.EncodeToString(sum[:])[:32]; got != expected[fmt.Sprint(y)] {
			t.Errorf("农历 %d 年与 lunar-typescript 不一致", y)
		}
	}
}

func TestCalendarRejectsDatesThatDoNotExist(t *testing.T) {
	_, err := SolarToLunar(1582, 10, 10)
	if !errors.Is(err, ErrInvalidDate) {
		t.Errorf("改历空缺：%v", err)
	}
	_, err = LunarToSolar(2024, 2, 1, true)
	if !errors.Is(err, ErrInvalidDate) {
		t.Errorf("2024 年无闰二月：%v", err)
	}
	_, err = LunarToSolar(237, 3, 1, false)
	if !errors.Is(err, ErrInvalidDate) {
		t.Errorf("237 年无三月：%v", err)
	}
	_, err = LunarMonths(10000)
	if !errors.Is(err, ErrInvalidArgument) {
		t.Errorf("年份越界：%v", err)
	}
	_, err = LunarYearPillar(0)
	if !errors.Is(err, ErrInvalidArgument) {
		t.Errorf("年份越界：%v", err)
	}
	if p, err := LunarYearPillar(2024); err != nil || p != (Pillar{StemJia, BranchChen}) {
		t.Errorf("LunarYearPillar(2024) = %v, %v", p, err)
	}
}

package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strconv"
	"strings"
	"testing"
)

// 夹宫与三个运限列表的金标测试：读 tests/golden/chart_lists.json（由 JS iztro 生成，
// Rust 的 golden_chart_lists.rs 与 Python 的 test_chart_lists_golden.py 读同一份），
// 按盘重算后与 JS 的取值逐项比。取样盘含晚子时与闰月出生——列表用的时辰取自时柱地支
// 而非出生入参，晚子时两者差 12，只有这类盘能暴露取错。

// chartListsCase 为金标里的一张盘：出生参数 + 四组期望取值。
type chartListsCase struct {
	Birth struct {
		// D 为出生阳历日期
		D string `json:"d"`
		// T 为出生时辰索引
		T uint8 `json:"t"`
		// G 为性别的 zh-CN 译名（"男"/"女"）
		G string `json:"g"`
	} `json:"birth"`
	// Flanking 为十二宫各自的夹宫，按目标宫索引列出
	Flanking []struct {
		// Index 为目标宫索引
		Index int `json:"index"`
		// Previous 为前一宫的索引与宫名
		Previous chartListsPalace `json:"previous"`
		// Next 为后一宫的索引与宫名
		Next chartListsPalace `json:"next"`
	} `json:"flanking"`
	// Decadals 为十二个大限
	Decadals []struct {
		Index         int      `json:"index"`
		Name          string   `json:"name"`
		PalaceName    string   `json:"palaceName"`
		AgeRange      [2]int   `json:"ageRange"`
		YearRange     [2]int   `json:"yearRange"`
		HeavenlyStem  string   `json:"heavenlyStem"`
		EarthlyBranch string   `json:"earthlyBranch"`
		PalaceNames   []string `json:"palaceNames"`
		Mutagen       []string `json:"mutagen"`
	} `json:"decadals"`
	// Yearly 按大限序号列出该限内的流年
	Yearly map[string][]struct {
		Index         int      `json:"index"`
		Age           int      `json:"age"`
		Year          int      `json:"year"`
		HeavenlyStem  string   `json:"heavenlyStem"`
		EarthlyBranch string   `json:"earthlyBranch"`
		PalaceNames   []string `json:"palaceNames"`
		Mutagen       []string `json:"mutagen"`
	} `json:"yearly"`
	// Monthly 按 "农历年_fixLeap" 列出该年的流月，fixLeap 即传给 MonthlyList 的分段开关
	Monthly map[string][]struct {
		Index         int      `json:"index"`
		Age           int      `json:"age"`
		Year          int      `json:"year"`
		Month         int      `json:"month"`
		IsLeapMonth   bool     `json:"isLeapMonth"`
		Part          string   `json:"part"`
		DayRange      [2]int   `json:"dayRange"`
		HeavenlyStem  string   `json:"heavenlyStem"`
		EarthlyBranch string   `json:"earthlyBranch"`
		Mutagen       []string `json:"mutagen"`
	} `json:"monthly"`
}

// chartListsPalace 为金标里的一个宫位引用。
type chartListsPalace struct {
	// Index 为宫位索引
	Index int `json:"index"`
	// Name 为宫名的 zh-CN 译名
	Name string `json:"name"`
}

// loadChartListsGolden 读入金标；文件缺失即失败，不跳过。
func loadChartListsGolden(t *testing.T) []chartListsCase {
	t.Helper()
	path := filepath.Join("..", "..", "tests", "golden", "chart_lists.json")
	raw, err := os.ReadFile(path)
	if err != nil {
		t.Fatalf("读不到金标 %s（先跑 node tests/golden/generate_chart_lists.mjs）：%v", path, err)
	}
	var cases []chartListsCase
	if err := json.Unmarshal(raw, &cases); err != nil {
		t.Fatal(err)
	}
	if len(cases) == 0 {
		t.Fatal("金标为空")
	}
	return cases
}

// chart 按金标的出生参数排盘；排盘的 fixLeap 恒为真，与生成器一致。
func (c chartListsCase) chart(t *testing.T) *Astrolabe {
	t.Helper()
	gender := GenderFemale
	if c.Birth.G == "男" {
		gender = GenderMale
	}
	a, err := BySolar(c.Birth.D, c.Birth.T, gender, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	return a
}

// label 为失败信息里的盘标识。
func (c chartListsCase) label() string {
	return c.Birth.D + " t=" + strconv.Itoa(int(c.Birth.T)) + " " + c.Birth.G
}

func TestFlankingPalacesMatchGolden(t *testing.T) {
	for _, c := range loadChartListsGolden(t) {
		a := c.chart(t)
		for _, want := range c.Flanking {
			got, err := a.FlankingPalaces(PalaceTarget{Index: want.Index})
			if err != nil {
				t.Fatalf("%s 第 %d 宫: %v", c.label(), want.Index, err)
			}
			for _, side := range []struct {
				name string
				got  *Palace
				want chartListsPalace
			}{
				{"previous", got.Previous, want.Previous},
				{"next", got.Next, want.Next},
			} {
				if side.got.Index != side.want.Index || side.got.Name != side.want.Name {
					t.Errorf("%s 第 %d 宫 %s: go (%d, %s) != js (%d, %s)",
						c.label(), want.Index, side.name,
						side.got.Index, side.got.Name, side.want.Index, side.want.Name)
				}
			}
		}
	}
}

func TestDecadalListMatchesGolden(t *testing.T) {
	for _, c := range loadChartListsGolden(t) {
		a := c.chart(t)
		got, err := a.DecadalList()
		if err != nil {
			t.Fatal(err)
		}
		if len(got) != len(c.Decadals) {
			t.Fatalf("%s: 大限应为 %d 项，实际 %d", c.label(), len(c.Decadals), len(got))
		}
		for n, want := range c.Decadals {
			g := got[n]
			ctx := c.label() + " 大限#" + strconv.Itoa(n)
			diff(t, ctx, map[string][2]any{
				"index":         {g.Index, want.Index},
				"name":          {g.Name, want.Name},
				"palaceName":    {g.PalaceName, want.PalaceName},
				"ageRange":      {g.AgeRange, want.AgeRange},
				"yearRange":     {g.YearRange, want.YearRange},
				"heavenlyStem":  {g.HeavenlyStem, want.HeavenlyStem},
				"earthlyBranch": {g.EarthlyBranch, want.EarthlyBranch},
				"palaceNames":   {g.PalaceNames, want.PalaceNames},
				"mutagen":       {g.Mutagen, want.Mutagen},
			})
		}
	}
}

func TestYearlyListMatchesGolden(t *testing.T) {
	for _, c := range loadChartListsGolden(t) {
		a := c.chart(t)
		for key, wants := range c.Yearly {
			ordinal, err := strconv.Atoi(key)
			if err != nil {
				t.Fatalf("%s: 流年键 %q 不是大限序号", c.label(), key)
			}
			got, err := a.YearlyList(ordinal)
			if err != nil {
				t.Fatal(err)
			}
			if len(got) != len(wants) {
				t.Fatalf("%s 大限#%d: 流年应为 %d 项，实际 %d", c.label(), ordinal, len(wants), len(got))
			}
			for n, want := range wants {
				g := got[n]
				ctx := c.label() + " 大限#" + key + " 虚岁" + strconv.Itoa(want.Age)
				diff(t, ctx, map[string][2]any{
					"index":         {g.Index, want.Index},
					"age":           {g.Age, want.Age},
					"year":          {g.Year, want.Year},
					"heavenlyStem":  {g.HeavenlyStem, want.HeavenlyStem},
					"earthlyBranch": {g.EarthlyBranch, want.EarthlyBranch},
					"palaceNames":   {g.PalaceNames, want.PalaceNames},
					"mutagen":       {g.Mutagen, want.Mutagen},
				})
			}
		}
	}
}

func TestMonthlyListMatchesGolden(t *testing.T) {
	for _, c := range loadChartListsGolden(t) {
		a := c.chart(t)
		for key, wants := range c.Monthly {
			yearText, fixLeapText, ok := strings.Cut(key, "_")
			if !ok {
				t.Fatalf("%s: 流月键 %q 不是「年份_fixLeap」形式", c.label(), key)
			}
			year, err := strconv.Atoi(yearText)
			if err != nil {
				t.Fatal(err)
			}
			fixLeap, err := strconv.ParseBool(fixLeapText)
			if err != nil {
				t.Fatal(err)
			}
			got, err := a.MonthlyList(year, Bool(fixLeap))
			if err != nil {
				t.Fatal(err)
			}
			if len(got) != len(wants) {
				t.Fatalf("%s %d fixLeap=%v: 流月应为 %d 项，实际 %d",
					c.label(), year, fixLeap, len(wants), len(got))
			}
			for n, want := range wants {
				g := got[n]
				ctx := c.label() + " " + key + " 第" + strconv.Itoa(n) + "项"
				diff(t, ctx, map[string][2]any{
					"index":         {g.Index, want.Index},
					"age":           {g.Age, want.Age},
					"year":          {g.Year, want.Year},
					"month":         {g.Month, want.Month},
					"isLeapMonth":   {g.IsLeapMonth, want.IsLeapMonth},
					"part":          {g.Part, want.Part},
					"dayRange":      {g.DayRange, want.DayRange},
					"heavenlyStem":  {g.HeavenlyStem, want.HeavenlyStem},
					"earthlyBranch": {g.EarthlyBranch, want.EarthlyBranch},
					"mutagen":       {g.Mutagen, want.Mutagen},
				})
			}
		}
	}
}

// diff 逐字段比对 [go 取值, js 取值]，不一致的报出来。
func diff(t *testing.T, ctx string, fields map[string][2]any) {
	t.Helper()
	for name, pair := range fields {
		if !reflect.DeepEqual(pair[0], pair[1]) {
			t.Errorf("%s %s: go %v != js %v", ctx, name, pair[0], pair[1])
		}
	}
}

package iztro

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"testing"
)

// star 模块的金标测试：直接读 JS iztro 生成的 tests/golden/star_cases.json，
// 逐个安星入口对照它的第一例（2000-8-16 寅时 女，修正闰月）。
//
// 这些入口不参与整盘输出，整盘金标测不到它们；此处按数据源逐字段比对，
// 任一入口的落宫漂移都会暴露。

// starGolden 为 star_cases.json 中本测试用到的部分。
type starGolden struct {
	Cases []struct {
		Param struct {
			SolarDate string `json:"solarDate"`
			TimeIndex uint8  `json:"timeIndex"`
			Gender    string `json:"gender"`
			FixLeap   bool   `json:"fixLeap"`
		} `json:"param"`
		StartIndex     StartIndex       `json:"startIndex"`
		LuYangTuoMa    LuYangTuoMaIndex `json:"luYangTuoMa"`
		KuiYue         KuiYueIndex      `json:"kuiYue"`
		ChangQu        ChangQuIndex     `json:"changQu"`
		KongJie        KongJieIndex     `json:"kongJie"`
		Timely         TimelyStarIndex  `json:"timely"`
		LuanXi         LuanXiIndex      `json:"luanXi"`
		Daily          DailyStarIndex   `json:"daily"`
		Monthly        MonthlyStarIndex `json:"monthly"`
		Yearly         YearlyStarIndex  `json:"yearly"`
		MajorStars     [][]string       `json:"majorStars"`
		MinorStars     [][]string       `json:"minorStars"`
		AdjectiveStars [][]string       `json:"adjectiveStars"`
		Changsheng12   []string         `json:"changsheng12"`
		Boshi12        []string         `json:"boshi12"`
		Yearly12       Yearly12         `json:"yearly12"`
	} `json:"cases"`
}

// loadStarGolden 读取安星金标数据。
func loadStarGolden(t *testing.T) starGolden {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "tests", "golden", "star_cases.json"))
	if err != nil {
		t.Fatalf("金标数据不可用: %v", err)
	}
	var g starGolden
	if err := json.Unmarshal(raw, &g); err != nil {
		t.Fatalf("解析金标数据: %v", err)
	}
	if len(g.Cases) == 0 {
		t.Fatal("金标数据为空")
	}
	return g
}

// starKeysByPalace 把十二宫星耀取成标识列表，便于同金标比对。
func starKeysByPalace(groups [][]Star) [][]string {
	out := make([][]string, len(groups))
	for i, group := range groups {
		out[i] = make([]string, 0, len(group))
		for _, s := range group {
			out[i] = append(out[i], s.Key)
		}
	}
	return out
}

// TestStarIndexesMatchGolden 对照金标校验各安星入口的落宫索引。
func TestStarIndexesMatchGolden(t *testing.T) {
	want := loadStarGolden(t).Cases[0]
	gender := GenderMale
	if want.Param.Gender == "女" {
		gender = GenderFemale
	}
	birth := StarBirth{
		SolarDate: want.Param.SolarDate,
		TimeIndex: want.Param.TimeIndex,
		Gender:    gender,
		FixLeap:   want.Param.FixLeap,
	}

	// 返回结构体索引的入口：逐个取值与金标整体比对
	indexCases := []struct {
		name string
		got  func() (any, error)
		want any
	}{
		{"GetStartIndex", func() (any, error) { return GetStartIndex(birth) }, want.StartIndex},
		{"GetLuYangTuoMaIndex", func() (any, error) { return GetLuYangTuoMaIndex(birth) }, want.LuYangTuoMa},
		{"GetKuiYueIndex", func() (any, error) { return GetKuiYueIndex(birth) }, want.KuiYue},
		{"GetChangQuIndex", func() (any, error) { return GetChangQuIndex(birth) }, want.ChangQu},
		{"GetKongJieIndex", func() (any, error) { return GetKongJieIndex(birth) }, want.KongJie},
		{"GetTimelyStarIndex", func() (any, error) { return GetTimelyStarIndex(birth) }, want.Timely},
		{"GetLuanXiIndex", func() (any, error) { return GetLuanXiIndex(birth) }, want.LuanXi},
		{"GetDailyStarIndex", func() (any, error) { return GetDailyStarIndex(birth) }, want.Daily},
		{"GetMonthlyStarIndex", func() (any, error) { return GetMonthlyStarIndex(birth) }, want.Monthly},
		{"GetYearlyStarIndex", func() (any, error) { return GetYearlyStarIndex(birth) }, want.Yearly},
	}
	for _, c := range indexCases {
		got, err := c.got()
		if err != nil {
			t.Fatalf("%s: %v", c.name, err)
		}
		if got != c.want {
			t.Errorf("%s:\n got  %+v\n want %+v", c.name, got, c.want)
		}
	}
}

// TestStarLayoutMatchesGolden 对照金标校验各组星耀与十二神的十二宫分布。
func TestStarLayoutMatchesGolden(t *testing.T) {
	want := loadStarGolden(t).Cases[0]
	gender := GenderMale
	if want.Param.Gender == "女" {
		gender = GenderFemale
	}
	birth := StarBirth{
		SolarDate: want.Param.SolarDate,
		TimeIndex: want.Param.TimeIndex,
		Gender:    gender,
		FixLeap:   want.Param.FixLeap,
	}

	starCases := []struct {
		name string
		got  func() ([][]Star, error)
		want [][]string
	}{
		{"GetMajorStar", func() ([][]Star, error) { return GetMajorStar(birth) }, want.MajorStars},
		{"GetMinorStar", func() ([][]Star, error) { return GetMinorStar(birth) }, want.MinorStars},
		{"GetAdjectiveStar", func() ([][]Star, error) { return GetAdjectiveStar(birth) }, want.AdjectiveStars},
	}
	for _, c := range starCases {
		stars, err := c.got()
		if err != nil {
			t.Fatalf("%s: %v", c.name, err)
		}
		assertKeyGrid(t, c.name, starKeysByPalace(stars), c.want)
	}

	changsheng, err := GetChangsheng12(birth)
	if err != nil {
		t.Fatalf("GetChangsheng12: %v", err)
	}
	assertKeyList(t, "GetChangsheng12", changsheng, want.Changsheng12)

	boshi, err := GetBoShi12(birth)
	if err != nil {
		t.Fatalf("GetBoShi12: %v", err)
	}
	assertKeyList(t, "GetBoShi12", boshi, want.Boshi12)

	yearly12, err := GetYearly12(birth)
	if err != nil {
		t.Fatalf("GetYearly12: %v", err)
	}
	assertKeyList(t, "GetYearly12.Suiqian12", yearly12.Suiqian12, want.Yearly12.Suiqian12)
	assertKeyList(t, "GetYearly12.Jiangqian12", yearly12.Jiangqian12, want.Yearly12.Jiangqian12)
}

// assertKeyList 逐项比对标识列表。
func assertKeyList(t *testing.T, name string, got, want []string) {
	t.Helper()
	if len(got) != len(want) {
		t.Errorf("%s: 长度 %d，期望 %d", name, len(got), len(want))
		return
	}
	for i := range want {
		if got[i] != want[i] {
			t.Errorf("%s[%d]: got %s want %s", name, i, got[i], want[i])
		}
	}
}

// assertKeyGrid 逐宫比对标识列表。
func assertKeyGrid(t *testing.T, name string, got, want [][]string) {
	t.Helper()
	if len(got) != len(want) {
		t.Errorf("%s: 宫数 %d，期望 %d", name, len(got), len(want))
		return
	}
	for i := range want {
		assertKeyList(t, fmt.Sprintf("%s[%d]", name, i), got[i], want[i])
	}
}

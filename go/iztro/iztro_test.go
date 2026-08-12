package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"testing"
)

// tier1Case 为 tests/golden/tier1_data.json 的用例结构（取本测试用到的字段）。
type tier1Case struct {
	Params struct {
		SolarDate string `json:"solar_date"`
		TimeIndex uint8  `json:"time_index"`
		Gender    string `json:"gender"`
	} `json:"params"`
	SoulStar          string `json:"soul_star"`
	BodyStar          string `json:"body_star"`
	FiveElementsClass string `json:"five_elements_class"`
	ChineseDate       string `json:"chinese_date"`
	Palaces           []struct {
		Name string `json:"name"`
	} `json:"palaces"`
}

// TestBySolarMatchesGolden 对照 JS 金标数据抽样验证 Go 链路（wasm 往返）。
func TestBySolarMatchesGolden(t *testing.T) {
	raw, err := os.ReadFile(filepath.Join("..", "..", "tests", "golden", "tier1_data.json"))
	if err != nil {
		t.Skipf("golden data unavailable: %v", err)
	}
	var cases []tier1Case
	if err := json.Unmarshal(raw, &cases); err != nil {
		t.Fatalf("parse golden data: %v", err)
	}

	step := len(cases) / 20
	for i := 0; i < len(cases); i += step {
		c := cases[i]
		gender := "male"
		if c.Params.Gender == "女" {
			gender = "female"
		}
		got, err := BySolar(c.Params.SolarDate, c.Params.TimeIndex, gender, true, "zh_cn", nil)
		if err != nil {
			t.Fatalf("BySolar(%s t%d): %v", c.Params.SolarDate, c.Params.TimeIndex, err)
		}
		if got.Soul != c.SoulStar {
			t.Errorf("%s t%d soul: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got.Soul, c.SoulStar)
		}
		if got.Body != c.BodyStar {
			t.Errorf("%s t%d body: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got.Body, c.BodyStar)
		}
		if got.FiveElementsClass != c.FiveElementsClass {
			t.Errorf("%s t%d fiveElementsClass: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got.FiveElementsClass, c.FiveElementsClass)
		}
		if got.ChineseDate != c.ChineseDate {
			t.Errorf("%s t%d chineseDate: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got.ChineseDate, c.ChineseDate)
		}
		for pi, exp := range c.Palaces {
			if got.Palaces[pi].Name != exp.Name {
				t.Errorf("%s t%d palace[%d]: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, pi, got.Palaces[pi].Name, exp.Name)
			}
		}
	}
}

// TestTypedQueries 验证类型化查询方法在任意输出语言下基于 key 正确工作。
func TestTypedQueries(t *testing.T) {
	for _, lang := range []string{"zh_cn", "en_us"} {
		a, err := BySolar("2000-8-16", 2, "female", true, lang, nil)
		if err != nil {
			t.Fatalf("BySolar(%s): %v", lang, err)
		}
		soul := a.Palace(PalaceSoul)
		if soul == nil {
			t.Fatalf("%s: soul palace not found by key", lang)
		}
		if !soul.Has(StarZiweiMaj) {
			t.Errorf("%s: soul palace should contain Ziwei", lang)
		}
		if a.SoulKey != StarPojunMaj {
			t.Errorf("%s: soulKey got %s want %s", lang, a.SoulKey, StarPojunMaj)
		}
		star, palace := a.Star(StarWuquMaj)
		if star == nil || palace == nil {
			t.Fatalf("%s: Wuqu not found", lang)
		}
		if !star.WithMutagen(MutagenQuan) {
			t.Errorf("%s: Wuqu should carry Quan mutagen", lang)
		}
		sp := a.SurroundedPalaces(soul.Index)
		if !sp.Have(StarZiweiMaj) {
			t.Errorf("%s: surrounded palaces of soul should contain Ziwei", lang)
		}
	}
}

// TestHoroscopeAndConfig 验证运限与中州派配置经 wasm 链路生效。
func TestHoroscopeAndConfig(t *testing.T) {
	h, err := GetHoroscope("2000-8-16", 2, "female", true, "zh_cn", nil, "2024-10-1", 0)
	if err != nil {
		t.Fatalf("GetHoroscope: %v", err)
	}
	if h.Yearly.HeavenlyStem != "甲" || h.Yearly.EarthlyBranch != "辰" {
		t.Errorf("yearly ganzhi: got %s%s want 甲辰", h.Yearly.HeavenlyStem, h.Yearly.EarthlyBranch)
	}
	if h.Yearly.YearlyDecStar == nil || len(h.Yearly.YearlyDecStar.Suiqian12Keys) != 12 {
		t.Error("yearly.yearlyDecStar missing or incomplete")
	}

	zz, err := BySolar("1990-11-5", 4, "male", true, "zh_cn", &Config{Algorithm: "zhongzhou"})
	if err != nil {
		t.Fatalf("BySolar zhongzhou: %v", err)
	}
	found := false
	for i := range zz.Palaces {
		if zz.Palaces[i].Suiqian12Key == StarSuipo {
			found = true
		}
	}
	if !found {
		t.Error("zhongzhou suiqian12 should contain Suipo")
	}
}

// TestErrorPropagation 验证非法参数经 wasm 返回错误。
func TestErrorPropagation(t *testing.T) {
	if _, err := BySolar("2000-8-16", 2, "unknown", true, "zh_cn", nil); err == nil {
		t.Error("invalid gender should return an error")
	}
}

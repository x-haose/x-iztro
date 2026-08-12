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
		if got["soul"] != c.SoulStar {
			t.Errorf("%s t%d soul: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got["soul"], c.SoulStar)
		}
		if got["body"] != c.BodyStar {
			t.Errorf("%s t%d body: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got["body"], c.BodyStar)
		}
		if got["fiveElementsClass"] != c.FiveElementsClass {
			t.Errorf("%s t%d fiveElementsClass: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got["fiveElementsClass"], c.FiveElementsClass)
		}
		if got["chineseDate"] != c.ChineseDate {
			t.Errorf("%s t%d chineseDate: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, got["chineseDate"], c.ChineseDate)
		}
		palaces := got["palaces"].([]any)
		for pi, exp := range c.Palaces {
			name := palaces[pi].(map[string]any)["name"]
			if name != exp.Name {
				t.Errorf("%s t%d palace[%d]: got %v want %s", c.Params.SolarDate, c.Params.TimeIndex, pi, name, exp.Name)
			}
		}
	}
}

// TestHoroscopeAndConfig 验证运限与中州派配置经 wasm 链路生效。
func TestHoroscopeAndConfig(t *testing.T) {
	h, err := Horoscope("2000-8-16", 2, "female", true, "zh_cn", nil, "2024-10-1", 0)
	if err != nil {
		t.Fatalf("Horoscope: %v", err)
	}
	yearly := h["yearly"].(map[string]any)
	if yearly["heavenlyStem"] != "甲" || yearly["earthlyBranch"] != "辰" {
		t.Errorf("yearly ganzhi: got %v%v want 甲辰", yearly["heavenlyStem"], yearly["earthlyBranch"])
	}
	if _, ok := yearly["yearlyDecStar"].(map[string]any); !ok {
		t.Error("yearly.yearlyDecStar missing")
	}

	zz, err := BySolar("1990-11-5", 4, "male", true, "zh_cn", &Config{Algorithm: "zhongzhou"})
	if err != nil {
		t.Fatalf("BySolar zhongzhou: %v", err)
	}
	found := false
	for _, p := range zz["palaces"].([]any) {
		if p.(map[string]any)["suiqian12"] == "岁破" {
			found = true
		}
	}
	if !found {
		t.Error("zhongzhou suiqian12 should contain 岁破")
	}
}

// TestErrorPropagation 验证非法参数经 wasm 返回错误。
func TestErrorPropagation(t *testing.T) {
	if _, err := BySolar("2000-8-16", 2, "unknown", true, "zh_cn", nil); err == nil {
		t.Error("invalid gender should return an error")
	}
}

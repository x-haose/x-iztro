package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

// 流派口径开关的四侧共用用例 tests/data/school_option_cases.json（期望由 Rust school_option_cases.rs 生成并核对，
// Python test_school_option_cases.py、JS school-option-cases.test.ts 读同一份）。配置经 Config、PatternConfig 的
// 字段解码后传入，字段漏接即与期望不符。

type schoolCase struct {
	Name  string `json:"name"`
	Chart struct {
		SolarDate string `json:"solarDate"`
		TimeIndex uint8  `json:"timeIndex"`
		Gender    Gender `json:"gender"`
	} `json:"chart"`
	Config        Config          `json:"config"`
	PatternConfig PatternConfig   `json:"patternConfig"`
	Expect        json.RawMessage `json:"expect"`
}

type schoolPalace struct {
	Stars                     []string `json:"stars"`
	Changsheng12Key           string   `json:"changsheng12Key"`
	Suiqian12Key              string   `json:"suiqian12Key"`
	Jiangqian12Key            string   `json:"jiangqian12Key"`
	IsOriginalPalace          bool     `json:"isOriginalPalace"`
	IsSecondaryOriginalPalace bool     `json:"isSecondaryOriginalPalace"`
}

type schoolProjection struct {
	LunarDate    []any          `json:"lunarDate"`
	YearlyKeys   [2]string      `json:"yearlyKeys"`
	Palaces      []schoolPalace `json:"palaces"`
	MonthlyIndex int            `json:"monthlyIndex"`
}

func TestSchoolOptionSharedCases(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "school_option_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		Target       [2]any       `json:"target"`
		Cases        []schoolCase `json:"cases"`
		PatternCases []schoolCase `json:"patternCases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	targetDate := doc.Target[0].(string)
	targetTime := uint8(doc.Target[1].(float64))
	chartOf := func(c schoolCase) *Astrolabe {
		config := c.Config
		chart, err := BySolar(c.Chart.SolarDate, c.Chart.TimeIndex, c.Chart.Gender, true, LanguageZhCN, &config)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		return chart
	}
	for _, c := range doc.Cases {
		chart := chartOf(c)
		h, err := chart.Horoscope(targetDate, targetTime)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		lunar := chart.RawDates.LunarDate
		got := schoolProjection{
			LunarDate:    []any{float64(lunar.LunarYear), float64(lunar.LunarMonth), float64(lunar.LunarDay), lunar.IsLeap},
			YearlyKeys:   chart.RawDates.ChineseDate.YearlyKeys,
			MonthlyIndex: h.Monthly.Index,
		}
		for _, p := range chart.Palaces {
			var stars []string
			for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
				for _, s := range group {
					stars = append(stars, s.Key)
				}
			}
			got.Palaces = append(got.Palaces, schoolPalace{
				Stars:                     stars,
				Changsheng12Key:           p.Changsheng12Key,
				Suiqian12Key:              p.Suiqian12Key,
				Jiangqian12Key:            p.Jiangqian12Key,
				IsOriginalPalace:          p.IsOriginalPalace,
				IsSecondaryOriginalPalace: p.IsSecondaryOriginalPalace,
			})
		}
		var want schoolProjection
		if err := json.Unmarshal(c.Expect, &want); err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("%s:\n got %+v\nwant %+v", c.Name, got, want)
		}
	}
	for _, c := range doc.PatternCases {
		config := c.PatternConfig
		hits, err := chartOf(c).Patterns(&config)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		var got [][2]*string
		for _, h := range hits {
			key := h.Key
			var variant *string
			if h.Variant != "" {
				v := h.Variant
				variant = &v
			}
			got = append(got, [2]*string{&key, variant})
		}
		var want [][2]*string
		if err := json.Unmarshal(c.Expect, &want); err != nil {
			t.Fatal(err)
		}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("%s: 命中不同", c.Name)
		}
	}
}

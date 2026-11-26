package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

// 附加四化星的四侧共用用例 tests/data/extra_mutagen_cases.json（期望由 Rust extra_mutagen_cases.rs 生成并核对，
// Python test_extra_mutagen_cases.py、JS extra-mutagen-cases.test.ts 读同一份）。
func TestExtraMutagenSharedCases(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "extra_mutagen_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		Cases []struct {
			Name  string `json:"name"`
			Chart struct {
				SolarDate string `json:"solarDate"`
				TimeIndex uint8  `json:"timeIndex"`
				Gender    Gender `json:"gender"`
			} `json:"chart"`
			Config  Config `json:"config"`
			Palaces []struct {
				Index           int                 `json:"index"`
				MutagenStarKeys []string            `json:"mutagenStarKeys"`
				MutagenStars    map[string][]string `json:"mutagenStars"`
				MutagedPlaces   []*int              `json:"mutagedPlaces"`
				SelfMutaged     map[string]bool     `json:"selfMutaged"`
			} `json:"palaces"`
		} `json:"cases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	for _, c := range doc.Cases {
		config := c.Config
		chart, err := BySolar(c.Chart.SolarDate, c.Chart.TimeIndex, c.Chart.Gender, true, LanguageZhCN, &config)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		for _, want := range c.Palaces {
			p := &chart.Palaces[want.Index]
			if !reflect.DeepEqual(p.MutagenStarKeys[:], want.MutagenStarKeys) {
				t.Errorf("%s 宫 %d 四槽: %v", c.Name, want.Index, p.MutagenStarKeys)
			}
			for _, m := range []string{MutagenLu, MutagenQuan, MutagenKe, MutagenJi} {
				if got := p.MutagenStars(m); !reflect.DeepEqual(got, want.MutagenStars[m]) {
					t.Errorf("%s 宫 %d %s: %v", c.Name, want.Index, m, got)
				}
				if got := p.SelfMutaged(m); got != want.SelfMutaged[m] {
					t.Errorf("%s 宫 %d %s 自化: %v", c.Name, want.Index, m, got)
				}
			}
			var places []*int
			for _, q := range p.MutagedPlaces() {
				if q == nil {
					places = append(places, nil)
				} else {
					i := q.Index
					places = append(places, &i)
				}
			}
			if len(places) != len(want.MutagedPlaces) {
				t.Fatalf("%s 宫 %d 落宫数: %d", c.Name, want.Index, len(places))
			}
			for k := range places {
				if (places[k] == nil) != (want.MutagedPlaces[k] == nil) || (places[k] != nil && *places[k] != *want.MutagedPlaces[k]) {
					t.Errorf("%s 宫 %d 落宫: 第 %d 项不同", c.Name, want.Index, k)
				}
			}
		}
	}
}

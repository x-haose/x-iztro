package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

// 日期写法与运限省略时辰的金标测试：读 tests/golden/inputs.json（由 JS iztro 生成，
// Rust 的 golden_inputs.rs 与 Python 的 test_inputs_golden.py 读同一份）。
// 这里验 Go 侧自己的那部分：HoroscopeAt 省略时辰时由目标日期的时间换算，
// 运限对象再发起计算时转发原始目标参数；以及各种日期写法的排盘回显。

// inputsGolden 为金标文件的结构。
type inputsGolden struct {
	// Solar 为公历写法用例：Error 非空表示 JS 报错
	Solar []struct {
		Input     string `json:"input"`
		SolarDate string `json:"solarDate"`
		Error     string `json:"error"`
	} `json:"solar"`
	// Horoscope 为运限用例：Tt 为 nil 表示省略时辰
	Horoscope []struct {
		P struct {
			D string `json:"d"`
			T uint8  `json:"t"`
		} `json:"p"`
		Td    string `json:"td"`
		Tt    *uint8 `json:"tt"`
		Sd    string `json:"sd"`
		Error string `json:"error"`
		Da    struct {
			Hs string `json:"hs"`
			Eb string `json:"eb"`
		} `json:"da"`
		Hr struct {
			Hs string `json:"hs"`
			Eb string `json:"eb"`
		} `json:"hr"`
	} `json:"horoscope"`
}

func loadInputsGolden(t *testing.T) inputsGolden {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "tests", "golden", "inputs.json"))
	if err != nil {
		t.Fatalf("读取 inputs.json 失败（cd tests/golden && npm run gen:inputs）: %v", err)
	}
	var data inputsGolden
	if err := json.Unmarshal(raw, &data); err != nil {
		t.Fatalf("解析 inputs.json 失败: %v", err)
	}
	return data
}

func TestSolarDateGrammarMatchesGolden(t *testing.T) {
	for _, c := range loadInputsGolden(t).Solar {
		chart, err := BySolar(c.Input, 2, GenderFemale, true, LanguageZhCN, nil)
		if c.Error != "" {
			if err == nil {
				t.Errorf("%q: JS 报错，Go 却排出了盘", c.Input)
			}
			continue
		}
		if err != nil {
			t.Errorf("%q: %v", c.Input, err)
			continue
		}
		if chart.SolarDate != c.SolarDate {
			t.Errorf("%q: SolarDate %q ≠ %q", c.Input, chart.SolarDate, c.SolarDate)
		}
	}
}

func TestHoroscopeTargetTimeMatchesGolden(t *testing.T) {
	for _, c := range loadInputsGolden(t).Horoscope {
		chart, err := BySolar(c.P.D, c.P.T, GenderFemale, true, LanguageZhCN, nil)
		if err != nil {
			t.Fatalf("%s: %v", c.P.D, err)
		}
		var h *Horoscope
		if c.Tt == nil {
			h, err = chart.HoroscopeAt(c.Td)
		} else {
			h, err = chart.Horoscope(c.Td, *c.Tt)
		}
		if c.Error != "" {
			if err == nil {
				t.Errorf("%q: JS 报错，Go 却算出了运限", c.Td)
			}
			continue
		}
		if err != nil {
			t.Errorf("%q: %v", c.Td, err)
			continue
		}
		label := c.Td
		if h.SolarDate != c.Sd {
			t.Errorf("%q: SolarDate %q ≠ %q", label, h.SolarDate, c.Sd)
		}
		if got := h.Hourly.HeavenlyStem + h.Hourly.EarthlyBranch; got != c.Hr.Hs+c.Hr.Eb {
			t.Errorf("%q: 流时 %s ≠ %s", label, got, c.Hr.Hs+c.Hr.Eb)
		}
		if got := h.Daily.HeavenlyStem + h.Daily.EarthlyBranch; got != c.Da.Hs+c.Da.Eb {
			t.Errorf("%q: 流日 %s ≠ %s", label, got, c.Da.Hs+c.Da.Eb)
		}
		// 再发起计算转发原始目标参数：运限文本里流时一节的干支须与本次运限一致
		text, err := h.ToText()
		if err != nil {
			t.Fatalf("%q: ToText: %v", label, err)
		}
		hourlyLine := ""
		for _, line := range strings.Split(text, "\n") {
			if strings.HasPrefix(line, "## 流时") {
				hourlyLine = line
				break
			}
		}
		if !strings.Contains(hourlyLine, "("+c.Hr.Hs+c.Hr.Eb+")") {
			t.Errorf("%q: 运限文本流时一节 %q 未含 (%s%s)", label, hourlyLine, c.Hr.Hs, c.Hr.Eb)
		}
	}
}

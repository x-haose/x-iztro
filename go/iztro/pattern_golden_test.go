package iztro

import (
	"encoding/json"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

// 格局的端到端金标测试：直接读 tests/golden/pattern_snapshots/*.json
// （由 Rust 的 tests/pattern_snapshot.rs 写出，Python 侧的 test_patterns.py 读同一批文件）。
// 每份快照含一张盘的排盘入参与本命、大限、流年三层的命中 DTO，
// 本测试按入参重新排盘取命中，与文件逐字段比对，三侧因此断言在同一组取值上。

// patternSnapshot 为一份快照文件的结构。
type patternSnapshot struct {
	Params struct {
		SolarDate       string         `json:"solarDate"`
		TimeIndex       uint8          `json:"timeIndex"`
		Gender          Gender         `json:"gender"`
		FixLeap         bool           `json:"fixLeap"`
		Language        Language       `json:"language"`
		Config          *Config        `json:"config"`
		PatternConfig   *PatternConfig `json:"patternConfig"`
		TargetDate      string         `json:"targetDate"`
		TargetTimeIndex uint8          `json:"targetTimeIndex"`
	} `json:"params"`
	Origin  []PatternHit `json:"origin"`
	Decadal []PatternHit `json:"decadal"`
	Yearly  []PatternHit `json:"yearly"`
}

// loadPatternSnapshots 读入全部快照文件，按文件名排序。
func loadPatternSnapshots(t *testing.T) map[string]patternSnapshot {
	t.Helper()
	dir := filepath.Join("..", "..", "tests", "golden", "pattern_snapshots")
	files, err := filepath.Glob(filepath.Join(dir, "*.json"))
	if err != nil {
		t.Fatal(err)
	}
	if len(files) == 0 {
		t.Fatalf("快照目录为空：%s（先跑 cargo test --test pattern_snapshot）", dir)
	}
	out := make(map[string]patternSnapshot, len(files))
	for _, file := range files {
		raw, err := os.ReadFile(file)
		if err != nil {
			t.Fatal(err)
		}
		var snapshot patternSnapshot
		if err := json.Unmarshal(raw, &snapshot); err != nil {
			t.Fatalf("解析 %s：%v", file, err)
		}
		out[strings.TrimSuffix(filepath.Base(file), ".json")] = snapshot
	}
	return out
}

// samePatternHits 比对两层命中；长度为 0 时 nil 与空切片视为相同。
func samePatternHits(got, want []PatternHit) bool {
	if len(got) == 0 && len(want) == 0 {
		return true
	}
	return reflect.DeepEqual(got, want)
}

func TestPatternsMatchSnapshots(t *testing.T) {
	for name, snapshot := range loadPatternSnapshots(t) {
		t.Run(name, func(t *testing.T) {
			p := snapshot.Params
			chart, err := BySolar(p.SolarDate, p.TimeIndex, p.Gender, p.FixLeap, p.Language, p.Config)
			if err != nil {
				t.Fatal(err)
			}
			horoscope, err := chart.Horoscope(p.TargetDate, p.TargetTimeIndex)
			if err != nil {
				t.Fatal(err)
			}

			origin, err := chart.Patterns(p.PatternConfig)
			if err != nil {
				t.Fatal(err)
			}
			decadal, err := horoscope.Patterns(ScopeDecadal, p.PatternConfig)
			if err != nil {
				t.Fatal(err)
			}
			yearly, err := horoscope.Patterns(ScopeYearly, p.PatternConfig)
			if err != nil {
				t.Fatal(err)
			}

			for _, layer := range []struct {
				name string
				got  []PatternHit
				want []PatternHit
			}{
				{"本命", origin, snapshot.Origin},
				{"大限", decadal, snapshot.Decadal},
				{"流年", yearly, snapshot.Yearly},
			} {
				if !samePatternHits(layer.got, layer.want) {
					t.Fatalf("%s 层与快照不一致：\n实际 %+v\n快照 %+v", layer.name, layer.got, layer.want)
				}
			}
		})
	}
}

func TestPatternHitHelpers(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	hits, err := chart.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}

	var hit *PatternHit
	for i := range hits {
		if hits[i].Is(PatternFuXiangChaoYuan) {
			hit = &hits[i]
		}
	}
	if hit == nil {
		t.Fatalf("该盘应命中府相朝垣，实际 %v", hits)
	}
	if hit.Is(PatternZiFuTongGong) {
		t.Fatal("Is 不应对别的格局为真")
	}
	if !hit.InPalace(PalaceSoul) || !hit.InPalace(hit.PalaceName) {
		t.Fatalf("成格宫位应为命宫，实际 %s / %s", hit.PalaceNameKey, hit.PalaceName)
	}
	if hit.Scope != ScopeOrigin {
		t.Fatalf("本命命中的视角应为 %s，实际 %s", ScopeOrigin, hit.Scope)
	}
	if len(hit.Stars) == 0 {
		t.Fatal("命中证据不应为空")
	}
	for _, star := range hit.Stars {
		if star.PalaceIndex < 0 || star.PalaceIndex > 11 {
			t.Fatalf("证据星落宫索引越界：%d", star.PalaceIndex)
		}
	}
	if want := hit.Name + "(" + hit.PalaceName + ")"; hit.String() != want {
		t.Fatalf("String() = %s，期望 %s", hit.String(), want)
	}
}

func TestHoroscopePatternsScope(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	horoscope, err := chart.Horoscope("2026-8-19", 3)
	if err != nil {
		t.Fatal(err)
	}

	decadal, err := horoscope.Patterns(ScopeDecadal, nil)
	if err != nil {
		t.Fatal(err)
	}
	for _, hit := range decadal {
		if hit.Scope != ScopeDecadal {
			t.Fatalf("大限层命中的视角应为 %s，实际 %s", ScopeDecadal, hit.Scope)
		}
	}

	origin, err := horoscope.Patterns(ScopeOrigin, nil)
	if err != nil {
		t.Fatal(err)
	}
	natal, err := chart.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	if !samePatternHits(origin, natal) {
		t.Fatal("运限的本命视角应与本命盘一致")
	}

	if _, err := horoscope.Patterns("nope", nil); err == nil {
		t.Fatal("未知 scope 应报错")
	}
}

func TestPatternConfigReachesKernel(t *testing.T) {
	// 该盘命宫三方四正的太阴在酉：亮度表判「不」不成格，位置法判明成格
	chart, err := BySolar("1985-1-3", 7, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	table, err := chart.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	positional, err := chart.Patterns(&PatternConfig{
		BrightnessSource: BrightnessSourcePositional,
		Borrow:           true,
		FlowStars:        true,
	})
	if err != nil {
		t.Fatal(err)
	}

	if hasPattern(table, PatternRiYueBingMing) {
		t.Fatal("亮度表口径下不应命中日月并明")
	}
	if !hasPattern(positional, PatternRiYueBingMing) || !hasPattern(positional, PatternDanChiGuiChi) {
		t.Fatalf("位置法口径下应命中日月并明与丹墀桂墀，实际 %v", positional)
	}
}

func TestChartConfigReachesPatternQuery(t *testing.T) {
	// 该盘的空亡星安法两派不同，默认派命宫空亡逢破军成「生不逢时」，中州派不成
	const (
		date      = "1985-6-11"
		timeIndex = uint8(11)
	)
	def, err := BySolar(date, timeIndex, GenderMale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	zhongzhou, err := BySolar(date, timeIndex, GenderMale, true, LanguageZhCN, &Config{Algorithm: AlgorithmZhongzhou})
	if err != nil {
		t.Fatal(err)
	}

	defHits, err := def.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	zzHits, err := zhongzhou.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	if !hasPattern(defHits, PatternShengBuFengShi) {
		t.Fatalf("默认派应命中生不逢时，实际 %v", defHits)
	}
	if hasPattern(zzHits, PatternShengBuFengShi) {
		t.Fatalf("中州派不应命中生不逢时，实际 %v", zzHits)
	}
}

// hasPattern 判断命中列表里是否有指定格局。
func hasPattern(hits []PatternHit, key string) bool {
	for i := range hits {
		if hits[i].Is(key) {
			return true
		}
	}
	return false
}

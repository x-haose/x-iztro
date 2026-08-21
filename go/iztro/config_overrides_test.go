package iztro

import (
	"strings"
	"testing"
)

// 自定义四化表与亮度表在 Go 侧的传播测试。
//
// 覆盖表只存在于调用方传入的 Config 里——星盘序列化回来的 Config 只有六个标量键，
// 一旦后续调用（运限、重排、prompt）改用那一份，覆盖就会静默失效。
// 断言取值与 tests/config_overrides.rs 同一组，三侧结论一致。

// overridesBirth 为本组测试固定的出生日期；2000 年为庚辰年。
const overridesBirth = "2000-8-16"

// customGeng 取 iztro 文档给出的另一派庚干四化：太阳、武曲、天同、天相。
func customGeng() *Config {
	return &Config{Mutagens: map[string][]string{
		StemGeng: {StarTaiyangMaj, StarWuquMaj, StarTiantongMaj, StarTianxiangMaj},
	}}
}

// starsWithMutagen 找出盘上带指定四化的星耀标识。
func starsWithMutagen(chart *Astrolabe, mutagenKey string) []string {
	var out []string
	for i := range chart.Palaces {
		p := &chart.Palaces[i]
		for _, group := range [][]Star{p.MajorStars, p.MinorStars} {
			for _, s := range group {
				if s.MutagenKey == mutagenKey {
					out = append(out, s.Key)
				}
			}
		}
	}
	return out
}

// TestCustomMutagensApplyToNatalStars 校验自定义四化改写本命盘的四化标注。
func TestCustomMutagensApplyToNatalStars(t *testing.T) {
	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, customGeng())
	if err != nil {
		t.Fatal(err)
	}

	// 天同由默认的化忌变为化科，天相接替化忌
	assertKeyList(t, "化科", starsWithMutagen(chart, MutagenKe), []string{StarTiantongMaj})
	assertKeyList(t, "化忌", starsWithMutagen(chart, MutagenJi), []string{StarTianxiangMaj})
	// 未改动的禄、权保持原状
	assertKeyList(t, "化禄", starsWithMutagen(chart, MutagenLu), []string{StarTaiyangMaj})
	assertKeyList(t, "化权", starsWithMutagen(chart, MutagenQuan), []string{StarWuquMaj})
}

// TestCustomMutagensApplyToFlyingStars 校验飞星判断读的是随盘下发的四化表。
func TestCustomMutagensApplyToFlyingStars(t *testing.T) {
	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, customGeng())
	if err != nil {
		t.Fatal(err)
	}
	def, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}

	var geng *Palace
	for i := range chart.Palaces {
		if chart.Palaces[i].HeavenlyStemKey == StemGeng {
			geng = &chart.Palaces[i]
			break
		}
	}
	if geng == nil {
		t.Fatal("该盘应有宫干为庚的宫位")
	}

	assertKeyList(t, "庚宫化忌星", geng.MutagenStars(MutagenJi), []string{StarTianxiangMaj})
	// 同一宫在默认配置下化忌为天同
	assertKeyList(t, "默认庚宫化忌星", def.Palaces[geng.Index].MutagenStars(MutagenJi), []string{StarTiantongMaj})

	// 飞星判断随之改变：天相落在财帛，天同落在疾厄
	tianxiang, tianxiangPalace := chart.Star(StarTianxiangMaj)
	if tianxiang == nil {
		t.Fatal("天相应在盘上")
	}
	if !geng.FliesTo(tianxiangPalace, MutagenJi) {
		t.Errorf("庚宫化忌应飞入天相所在的 %s", tianxiangPalace.NameKey)
	}
	if !geng.FliesOneOfTo(tianxiangPalace, MutagenJi) {
		t.Error("FliesOneOfTo 应与 FliesTo 一致")
	}
	if geng.NotFlyTo(tianxiangPalace, MutagenJi) {
		t.Error("NotFlyTo 应为 false")
	}
	_, defaultJiPalace := def.Star(StarTiantongMaj)
	if !def.Palaces[geng.Index].FliesTo(defaultJiPalace, MutagenJi) {
		t.Error("默认配置下庚宫化忌应飞入天同所在宫")
	}
}

// TestCustomMutagensApplyToHoroscope 校验运限沿用原始配置，覆盖表不丢。
func TestCustomMutagensApplyToHoroscope(t *testing.T) {
	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, customGeng())
	if err != nil {
		t.Fatal(err)
	}
	// 2030 年为庚戌年，流年干为庚
	h, err := chart.Horoscope("2030-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	if h.Yearly.HeavenlyStemKey != StemGeng {
		t.Fatalf("流年干 %s，期望庚", h.Yearly.HeavenlyStemKey)
	}
	assertKeyList(t, "流年四化", h.Yearly.MutagenStarKeys, []string{
		StarTaiyangMaj, StarWuquMaj, StarTiantongMaj, StarTianxiangMaj,
	})
}

// TestCustomMutagensApplyToRearrangedAndPrompt 校验重排与 prompt 同样沿用原始配置。
func TestCustomMutagensApplyToRearranged(t *testing.T) {
	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, customGeng())
	if err != nil {
		t.Fatal(err)
	}
	rearranged, err := chart.Rearranged(StemGeng, BranchChen)
	if err != nil {
		t.Fatal(err)
	}
	assertKeyList(t, "重排后化忌", starsWithMutagen(rearranged, MutagenJi), []string{StarTianxiangMaj})

	// 重排结果继续携带原始配置，可再次重排而不丢覆盖表
	again, err := rearranged.Rearranged(StemGeng, BranchChen)
	if err != nil {
		t.Fatal(err)
	}
	assertKeyList(t, "二次重排后化忌", starsWithMutagen(again, MutagenJi), []string{StarTianxiangMaj})
}

// TestCustomMutagensApplyToText 校验语义化文本的生年四化取自自定义表。
func TestCustomMutagensApplyToText(t *testing.T) {
	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, customGeng())
	if err != nil {
		t.Fatal(err)
	}
	text, err := chart.ToText()
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(text, "生年四化: 太阳禄, 武曲权, 天同科, 天相忌") {
		t.Errorf("本命文本的生年四化未反映自定义表:\n%s", text)
	}

	horoscope, err := chart.Horoscope("2030-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	horoText, err := horoscope.ToText()
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(horoText, "流年四化: 太阳禄, 武曲权, 天同科, 天相忌") {
		t.Errorf("运限文本的流年四化未反映自定义表:\n%s", horoText)
	}
}

// TestCustomBrightnessApplies 校验自定义亮度表在排盘与工具函数上一致生效。
func TestCustomBrightnessApplies(t *testing.T) {
	wang := make([]string, 12)
	for i := range wang {
		wang[i] = BrightnessWang
	}
	config := &Config{Brightness: map[string][]string{StarTanlangMaj: wang}}

	chart, err := BySolar(overridesBirth, 2, GenderFemale, true, LanguageZhCN, config)
	if err != nil {
		t.Fatal(err)
	}
	tanlang, _ := chart.Star(StarTanlangMaj)
	if tanlang == nil {
		t.Fatal("贪狼应在盘上")
	}
	if !tanlang.WithBrightness(BrightnessWang) {
		t.Errorf("贪狼亮度 %s，期望旺", tanlang.BrightnessKey)
	}
	for index := 0; index < 12; index++ {
		got, err := GetBrightness(StarTanlangMaj, index, config)
		if err != nil {
			t.Fatal(err)
		}
		if got != BrightnessWang {
			t.Errorf("GetBrightness(贪狼, %d) = %s，期望旺", index, got)
		}
	}

	// 未覆盖的星保持默认：紫微在命宫仍为庙
	ziwei, _ := chart.Star(StarZiweiMaj)
	if ziwei.BrightnessKey != BrightnessMiao {
		t.Errorf("紫微亮度 %s，期望庙", ziwei.BrightnessKey)
	}
}

// TestCustomMutagensDoNotLeakToOtherStems 校验覆盖只作用于指定天干。
func TestCustomMutagensDoNotLeakToOtherStems(t *testing.T) {
	config := customGeng()

	geng, err := GetMutagensByHeavenlyStem(StemGeng, config)
	if err != nil {
		t.Fatal(err)
	}
	if geng[3] != StarTianxiangMaj {
		t.Errorf("庚干化忌 %s，期望天相", geng[3])
	}

	custom, err := GetMutagensByHeavenlyStem(StemRen, config)
	if err != nil {
		t.Fatal(err)
	}
	def, err := GetMutagensByHeavenlyStem(StemRen, nil)
	if err != nil {
		t.Fatal(err)
	}
	assertKeyList(t, "壬干四化", custom, def)

	got, err := GetMutagen(StarTiantongMaj, StemGeng, config)
	if err != nil {
		t.Fatal(err)
	}
	if got != MutagenKe {
		t.Errorf("自定义表下天同在庚干化 %s，期望科", got)
	}
	got, err = GetMutagen(StarTiantongMaj, StemGeng, nil)
	if err != nil {
		t.Fatal(err)
	}
	if got != MutagenJi {
		t.Errorf("默认表下天同在庚干化 %s，期望忌", got)
	}
}

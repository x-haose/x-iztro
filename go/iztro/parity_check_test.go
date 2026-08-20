package iztro

import (
	"encoding/json"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"testing"
	"time"
)

// TestStage1Parity 覆盖本轮为对齐 iztro 而补的对象方法，
// 断言值与 Rust / Python 侧同一张盘的结果一致。
func TestStage1Parity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}

	// 星耀回引
	star, _ := chart.Star(StarZiweiMaj)
	if star.Palace().NameKey != PalaceSoul {
		t.Fatalf("紫微应在命宫，实际 %s", star.Palace().NameKey)
	}
	if got, want := star.OppositePalace().Index, (star.Palace().Index+6)%12; got != want {
		t.Fatalf("对宫索引 %d，期望 %d", got, want)
	}
	if star.SurroundedPalaces().Target.Index != star.Palace().Index {
		t.Fatal("三方四正本宫应为星耀所在宫")
	}

	// 身宫 / 来因宫定位
	body := chart.Palace(PalaceBody)
	if body == nil || !body.IsBodyPalace {
		t.Fatal("身宫定位失败")
	}
	if body.NameKey != PalaceCareer {
		t.Fatalf("身宫应在官禄，实际 %s", body.NameKey)
	}
	if orig := chart.Palace(PalaceOriginal); orig == nil || orig.NameKey != PalaceSpouse {
		t.Fatal("来因宫应在夫妻")
	}

	soul := chart.Palace(PalaceSoul)
	if soul.Astrolabe() != chart {
		t.Fatal("宫位回引星盘失败")
	}

	// 四化表：命宫壬午，壬干四化为天梁、紫微、左辅、武曲
	stars := soul.MutagenStars(MutagenLu, MutagenQuan, MutagenKe, MutagenJi)
	want := []string{StarTianliangMaj, StarZiweiMaj, StarZuofuMin, StarWuquMaj}
	for i := range want {
		if stars[i] != want[i] {
			t.Fatalf("四化星[%d] = %s，期望 %s", i, stars[i], want[i])
		}
	}

	// 紫微在命宫，故命宫自化权
	if !soul.SelfMutaged(MutagenQuan) {
		t.Fatal("命宫应自化权")
	}
	if soul.NotSelfMutaged() {
		t.Fatal("命宫有自化，NotSelfMutaged 应为 false")
	}

	// 空四化列表的语义（复刻 iztro）
	target := chart.PalaceByIndex(0)
	if soul.FliesTo(target) {
		t.Fatal("FliesTo 空四化应为 false")
	}
	if !soul.FliesOneOfTo(target) || !soul.NotFlyTo(target) {
		t.Fatal("FliesOneOfTo / NotFlyTo 空四化应为 true")
	}

	// 四化飞入的宫位
	places := soul.MutagedPlaces()
	if len(places) != 4 {
		t.Fatalf("MutagedPlaces 应返回 4 项，实际 %d", len(places))
	}
	if places[1] == nil || places[1].NameKey != PalaceSoul {
		t.Fatal("壬干化权为紫微，应飞入命宫")
	}

	// 空宫与借星
	var empty *Palace
	for i := range chart.Palaces {
		if chart.Palaces[i].IsEmpty() {
			empty = &chart.Palaces[i]
			break
		}
	}
	if empty == nil {
		t.Fatal("该盘应存在空宫")
	}
	if len(empty.MinorStars) > 0 && empty.IsEmpty(empty.MinorStars[0].Key) {
		t.Fatal("排除星在宫内时不应判为空宫")
	}

	// 三方四正补齐的方法
	sp := chart.SurroundedPalaces(PalaceSoul)
	first := sp.Target.MajorStars[0].Key
	if !sp.HaveOneOf(first) || sp.NotHave(first) {
		t.Fatal("HaveOneOf / NotHave 行为不符")
	}
	if !chart.IsSurrounded(PalaceSoul, first) || chart.NotSurrounded(PalaceSoul, first) {
		t.Fatal("IsSurrounded / NotSurrounded 行为不符")
	}

	// 运限便捷方法
	h, err := chart.Horoscope("2024-10-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	// 运限持有发起它的那张盘，宫位查询不必再传盘
	if h.Astrolabe() != chart {
		t.Fatal("运限未持有发起查询的本命盘")
	}
	if h.AgePalace() == nil {
		t.Fatal("小限宫位取不到")
	}
	if h.Palace(PalaceSoul, ScopeYearly) == nil {
		t.Fatal("流年命宫取不到")
	}
	if h.SurroundPalaces(PalaceSoul, ScopeYearly) == nil {
		t.Fatal("流年命宫三方四正取不到")
	}
	if h.HasHoroscopeMutagen(PalaceSoul, ScopeOrigin, MutagenLu) {
		t.Fatal("ScopeOrigin 下运限四化恒为 false")
	}
	// 三个流耀判断互相自洽
	probe := []string{"流禄"}
	has := h.HasHoroscopeStars(PalaceSoul, ScopeYearly, probe)
	one := h.HasOneOfHoroscopeStars(PalaceSoul, ScopeYearly, probe)
	not := h.NotHaveHoroscopeStars(PalaceSoul, ScopeYearly, probe)
	if has != one || has == not {
		t.Fatalf("流耀判断不自洽: has=%v one=%v not=%v", has, one, not)
	}
}

// TestQueryParity 覆盖阶段 2 的轻量查询，断言值与完整排盘及另两种语言绑定一致。
func TestQueryParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}

	zodiac, err := GetZodiacBySolarDate("2000-8-16", "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	if zodiac != chart.Zodiac {
		t.Fatalf("生肖 %s，排盘字段为 %s", zodiac, chart.Zodiac)
	}

	sign, err := GetSignBySolarDate("2000-8-16", "zh-CN")
	if err != nil {
		t.Fatal(err)
	}
	if sign != chart.Sign {
		t.Fatalf("星座 %s，排盘字段为 %s", sign, chart.Sign)
	}

	// 农历入口应得到同一个星座（2000-7-17 农历即 2000-8-16 阳历）
	lunarSign, err := GetSignByLunarDate("2000-7-17", false, "zh-CN")
	if err != nil {
		t.Fatal(err)
	}
	if lunarSign != sign {
		t.Fatalf("农历星座 %s，阳历星座 %s", lunarSign, sign)
	}

	// 命宫主星：该盘命宫坐紫微
	major, err := GetMajorStarBySolarDate("2000-8-16", 2, true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	if major != "紫微" {
		t.Fatalf("命宫主星 %s，期望 紫微", major)
	}
	lunarMajor, err := GetMajorStarByLunarDate("2000-7-17", 2, NotLeapMonth, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	if lunarMajor != major {
		t.Fatalf("农历入口命宫主星 %s，阳历入口 %s", lunarMajor, major)
	}

	// 多语言：与 Python 侧断言同一组取值
	enZodiac, _ := GetZodiacBySolarDate("2000-8-16", "en-US", nil)
	enSign, _ := GetSignBySolarDate("2000-8-16", "en-US")
	enMajor, _ := GetMajorStarBySolarDate("2000-8-16", 2, true, "en-US", nil)
	if enZodiac != "dragon" || enSign != "leo" || enMajor != "emperor" {
		t.Fatalf("英文输出不符: %s / %s / %s", enZodiac, enSign, enMajor)
	}

	// 非法入参应返回 error 而不是 panic
	if _, err := GetZodiacBySolarDate("2000-2-30", "zh-CN", nil); err == nil {
		t.Fatal("非法日期应返回 error")
	}
	if _, err := GetMajorStarBySolarDate("2000-8-16", 13, true, "zh-CN", nil); err == nil {
		t.Fatal("非法时辰索引应返回 error")
	}
}

// TestUtilParity 覆盖阶段 2 的工具函数，断言与排盘结果及 Python 侧一致。
func TestUtilParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	const yearStem = StemGeng // 2000 年为庚辰

	if v, _ := FixIndex(-1, 0); v != 11 {
		t.Fatalf("FixIndex(-1) = %d，期望 11", v)
	}
	if v, _ := TimeToIndex(23); v != 12 {
		t.Fatalf("TimeToIndex(23) = %d，期望 12", v)
	}
	if v, _ := EarthlyBranchToPalaceIndex(BranchYin); v != 0 {
		t.Fatalf("寅宫索引应为 0，实际 %d", v)
	}

	soul := chart.Palace(PalaceSoul)

	// 五行局由命宫干支推出，应与排盘字段一致
	fec, err := GetFiveElementsClass(soul.HeavenlyStemKey, soul.EarthlyBranchKey)
	if err != nil {
		t.Fatal(err)
	}
	if fec != chart.FiveElementsClassKey {
		t.Fatalf("五行局 %s，排盘字段 %s", fec, chart.FiveElementsClassKey)
	}

	// 十二宫名由命宫索引推出，应与排盘逐项一致
	names, err := GetPalaceNames(soul.Index)
	if err != nil {
		t.Fatal(err)
	}
	for i, name := range names {
		if name != chart.Palaces[i].NameKey {
			t.Fatalf("宫位 %d 名称 %s，排盘为 %s", i, name, chart.Palaces[i].NameKey)
		}
	}

	// 命宫身宫
	monthIndex, err := FixLunarMonthIndex(
		chart.RawDates.LunarDate.LunarMonth, chart.RawDates.LunarDate.LunarDay,
		chart.RawDates.LunarDate.IsLeap, chart.TimeIndex, chart.FixLeap)
	if err != nil {
		t.Fatal(err)
	}
	sb, err := GetSoulAndBody(monthIndex, chart.TimeIndex, yearStem)
	if err != nil {
		t.Fatal(err)
	}
	if sb.SoulIndex != soul.Index {
		t.Fatalf("命宫索引 %d，排盘为 %d", sb.SoulIndex, soul.Index)
	}
	if sb.EarthlyBranchOfSoul != chart.EarthlyBranchOfSoulPalaceKey {
		t.Fatalf("命宫地支 %s，排盘为 %s", sb.EarthlyBranchOfSoul, chart.EarthlyBranchOfSoulPalaceKey)
	}
	if sb.BodyIndex != chart.Palace(PalaceBody).Index {
		t.Fatal("身宫索引与排盘不一致")
	}

	// 亮度与四化查表：与盘上每颗主星、辅星比对
	checked := 0
	for i := range chart.Palaces {
		p := &chart.Palaces[i]
		for _, group := range [][]Star{p.MajorStars, p.MinorStars} {
			for _, s := range group {
				b, err := GetBrightness(s.Key, p.Index, nil)
				if err != nil {
					t.Fatal(err)
				}
				if b != s.BrightnessKey {
					t.Fatalf("%s 在宫位 %d 的亮度 %q，星耀字段 %q", s.Key, p.Index, b, s.BrightnessKey)
				}
				m, err := GetMutagen(s.Key, yearStem, nil)
				if err != nil {
					t.Fatal(err)
				}
				if m != s.MutagenKey {
					t.Fatalf("%s 在年干下的四化 %q，星耀字段 %q", s.Key, m, s.MutagenKey)
				}
				checked++
			}
		}
	}
	if checked < 20 {
		t.Fatalf("覆盖的星耀太少：%d", checked)
	}

	// 壬干四化：与 Rust / Python 侧断言同一组取值
	muts, err := GetMutagensByHeavenlyStem(StemRen, nil)
	if err != nil {
		t.Fatal(err)
	}
	want := []string{StarTianliangMaj, StarZiweiMaj, StarZuofuMin, StarWuquMaj}
	for i := range want {
		if muts[i] != want[i] {
			t.Fatalf("壬干四化[%d] = %s，期望 %s", i, muts[i], want[i])
		}
	}

	// 非法 key 应返回 error
	if _, err := GetBrightness("nope", 0, nil); err == nil {
		t.Fatal("未知星耀标识应返回 error")
	}
	if _, err := TimeToIndex(24); err == nil {
		t.Fatal("非法小时应返回 error")
	}
}

// TestConfigOverridesParity 覆盖自定义四化与亮度表，断言与 Rust / Python 侧一致。
func TestConfigOverridesParity(t *testing.T) {
	// iztro 文档给出的另一派庚干四化：太阳、武曲、天同、天相
	custom := &Config{Mutagens: map[string][]string{
		StemGeng: {StarTaiyangMaj, StarWuquMaj, StarTiantongMaj, StarTianxiangMaj},
	}}

	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", custom)
	if err != nil {
		t.Fatal(err)
	}

	collect := func(c *Astrolabe, mutagenKey string) []string {
		var out []string
		for i := range c.Palaces {
			p := &c.Palaces[i]
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

	if got := collect(chart, MutagenKe); len(got) != 1 || got[0] != StarTiantongMaj {
		t.Fatalf("自定义庚干化科应为天同，实际 %v", got)
	}
	if got := collect(chart, MutagenJi); len(got) != 1 || got[0] != StarTianxiangMaj {
		t.Fatalf("自定义庚干化忌应为天相，实际 %v", got)
	}

	// 默认配置下化科仍是太阴
	def, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	if got := collect(def, MutagenKe); len(got) != 1 || got[0] != StarTaiyinMaj {
		t.Fatalf("默认庚干化科应为太阴，实际 %v", got)
	}

	// 工具函数走同一套自定义表，且不泄漏到其他天干
	muts, err := GetMutagensByHeavenlyStem(StemGeng, custom)
	if err != nil {
		t.Fatal(err)
	}
	if muts[3] != StarTianxiangMaj {
		t.Fatalf("工具函数取到的庚干化忌为 %s", muts[3])
	}
	renMuts, err := GetMutagensByHeavenlyStem(StemRen, custom)
	if err != nil {
		t.Fatal(err)
	}
	if renMuts[3] != StarWuquMaj {
		t.Fatalf("壬干不应受影响，实际化忌为 %s", renMuts[3])
	}

	// 亮度覆盖
	bright := &Config{Brightness: map[string][]string{
		StarTanlangMaj: {
			BrightnessWang, BrightnessWang, BrightnessWang, BrightnessWang,
			BrightnessWang, BrightnessWang, BrightnessWang, BrightnessWang,
			BrightnessWang, BrightnessWang, BrightnessWang, BrightnessWang,
		},
	}}
	c2, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", bright)
	if err != nil {
		t.Fatal(err)
	}
	tanlang, _ := c2.Star(StarTanlangMaj)
	if tanlang.BrightnessKey != BrightnessWang {
		t.Fatalf("贪狼亮度应为旺，实际 %s", tanlang.BrightnessKey)
	}

	// 非法自定义表应返回 error
	if _, err := BySolar("2000-8-16", 2, "female", true, "zh-CN",
		&Config{Mutagens: map[string][]string{StemGeng: {StarTaiyangMaj}}}); err == nil {
		t.Fatal("四化数量不为 4 应返回 error")
	}
	if _, err := BySolar("2000-8-16", 2, "female", true, "zh-CN",
		&Config{Mutagens: map[string][]string{"nope": {StarTaiyangMaj, StarWuquMaj, StarTiantongMaj, StarTianxiangMaj}}}); err == nil {
		t.Fatal("未知天干标识应返回 error")
	}
}

// TestAstroTypeParity 覆盖天盘/地盘/人盘与任意干支重排，
// 断言与 Rust、Python 侧同一盘面的相同结论。
func TestAstroTypeParity(t *testing.T) {
	heaven, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	earth, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", &Config{AstroType: AstroEarth})
	if err != nil {
		t.Fatal(err)
	}
	human, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", &Config{AstroType: AstroHuman})
	if err != nil {
		t.Fatal(err)
	}

	// 地盘以天盘身宫为命宫，人盘以天盘福德宫为命宫
	bodyPalace := heaven.Palace(PalaceBody)
	if earth.Palace(PalaceSoul).Index != bodyPalace.Index {
		t.Fatalf("地盘命宫应落在天盘身宫（索引 %d），实际 %d",
			bodyPalace.Index, earth.Palace(PalaceSoul).Index)
	}
	spiritPalace := heaven.Palace(PalaceSpirit)
	if human.Palace(PalaceSoul).Index != spiritPalace.Index {
		t.Fatalf("人盘命宫应落在天盘福德宫（索引 %d），实际 %d",
			spiritPalace.Index, human.Palace(PalaceSoul).Index)
	}

	// Rearranged 从任意干支起盘；以身宫干支重排即等价于地盘
	manual, err := heaven.Rearranged(bodyPalace.HeavenlyStemKey, bodyPalace.EarthlyBranchKey)
	if err != nil {
		t.Fatal(err)
	}
	if manual.FiveElementsClassKey != earth.FiveElementsClassKey {
		t.Fatalf("以身宫干支重排应等价于地盘：%s vs %s",
			manual.FiveElementsClassKey, earth.FiveElementsClassKey)
	}
	for i := range manual.Palaces {
		if manual.Palaces[i].NameKey != earth.Palaces[i].NameKey {
			t.Fatalf("宫位 %d 宫名不一致：%s vs %s",
				i, manual.Palaces[i].NameKey, earth.Palaces[i].NameKey)
		}
	}

	// 非法干支标识报错而不是静默取默认
	if _, err := heaven.Rearranged("notAStem", BranchZi); err == nil {
		t.Fatal("非法天干标识应报错")
	}
}

// TestRearrangedCarriesIntoHoroscopeAndPrompt 校验重排盘上的运限与 Prompt
// 按重排后的布局计算，而不是静默退回原盘。
func TestRearrangedCarriesIntoHoroscopeAndPrompt(t *testing.T) {
	chart, err := BySolar("1990-4-23", 2, GenderMale, false, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	re, err := chart.Rearranged(StemGeng, BranchChen)
	if err != nil {
		t.Fatal(err)
	}

	plainH, err := chart.Horoscope("2024-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	reH, err := re.Horoscope("2024-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	plainJSON, err := json.Marshal(plainH)
	if err != nil {
		t.Fatal(err)
	}
	reJSON, err := json.Marshal(reH)
	if err != nil {
		t.Fatal(err)
	}
	if string(plainJSON) == string(reJSON) {
		t.Fatal("重排盘的运限应与原盘不同")
	}

	plainPrompt, err := chart.AstrolabeToPrompt()
	if err != nil {
		t.Fatal(err)
	}
	rePrompt, err := re.AstrolabeToPrompt()
	if err != nil {
		t.Fatal(err)
	}
	if plainPrompt == rePrompt {
		t.Fatal("重排盘的本命 Prompt 应与原盘不同")
	}

	plainHP, err := chart.HoroscopeToPrompt("2024-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	reHP, err := re.HoroscopeToPrompt("2024-6-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	if plainHP == reHP {
		t.Fatal("重排盘的运限 Prompt 应与原盘不同")
	}
}

// TestUtilTailParity 覆盖 translateChineseDate 与 mergeStars，
// 断言与排盘字段及另两种语言绑定一致。
func TestUtilTailParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}

	// 四柱展示串应与排盘字段逐字一致
	got, err := TranslateChineseDate(chart.RawDates.ChineseDate.PillarKeys(), "zh-CN")
	if err != nil {
		t.Fatal(err)
	}
	if got != chart.ChineseDate {
		t.Fatalf("四柱展示串 %q，排盘字段 %q", got, chart.ChineseDate)
	}

	// 多字符词条改用「 - 」分隔柱
	en, err := TranslateChineseDate(chart.RawDates.ChineseDate.PillarKeys(), "en-US")
	if err != nil {
		t.Fatal(err)
	}
	if en != "geng chen - jia shen - bing woo - geng yin" {
		t.Fatalf("英文四柱串 %q 不符", en)
	}

	// 非法标识报错而不是静默取默认
	bad := chart.RawDates.ChineseDate.PillarKeys()
	bad[0][0] = "notAStem"
	if _, err := TranslateChineseDate(bad, "zh-CN"); err == nil {
		t.Fatal("非法天干标识应报错")
	}

	// 合并两组十二宫星耀
	majors := make([][]Star, 12)
	minors := make([][]Star, 12)
	for i := range chart.Palaces {
		majors[i] = chart.Palaces[i].MajorStars
		minors[i] = chart.Palaces[i].MinorStars
	}
	merged, err := MergeStars(majors, minors)
	if err != nil {
		t.Fatal(err)
	}
	if len(merged) != 12 {
		t.Fatalf("合并结果应为十二宫，实际 %d", len(merged))
	}
	for i := range merged {
		want := len(majors[i]) + len(minors[i])
		if len(merged[i]) != want {
			t.Fatalf("宫位 %d 合并后 %d 颗，期望 %d", i, len(merged[i]), want)
		}
	}
	if _, err := MergeStars(majors[:11]); err == nil {
		t.Fatal("非十二项的分组应报错")
	}
}

// TestHoroscopeNowParity 断言 HoroscopeNow 与显式传今天等价。
func TestHoroscopeNowParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}

	now := time.Now()
	ti, err := TimeToIndex(uint8(now.Hour()))
	if err != nil {
		t.Fatal(err)
	}
	explicit, err := chart.Horoscope(
		fmt.Sprintf("%d-%d-%d", now.Year(), int(now.Month()), now.Day()), ti)
	if err != nil {
		t.Fatal(err)
	}
	byNow, err := chart.HoroscopeNow()
	if err != nil {
		t.Fatal(err)
	}

	if byNow.SolarDate != explicit.SolarDate {
		t.Fatalf("日期 %s vs %s", byNow.SolarDate, explicit.SolarDate)
	}
	if byNow.Decadal.Index != explicit.Decadal.Index || byNow.Hourly.Index != explicit.Hourly.Index {
		t.Fatal("HoroscopeNow 与显式传今天结果不一致")
	}
}

// promptSnapshot 读取 Rust 侧写下的 prompt 快照。
func promptSnapshot(t *testing.T, name string) string {
	t.Helper()
	raw, err := os.ReadFile(filepath.Join("..", "..", "tests", "golden", "prompt_snapshots", name+".txt"))
	if err != nil {
		t.Fatalf("prompt 快照不可用: %v", err)
	}
	return string(raw)
}

// TestPromptParity 断言 Go 侧 Prompt 与 Rust 侧快照逐字节相同。
//
// 快照由 tests/prompt_snapshot.rs 生成，固定盘 2000-8-16 时辰 2 女命、
// 运限目标 2025-1-1 时辰 0。措辞、字段顺序、段落增删任一处漂移都会暴露；
// 有意改动 prompt 时按 Rust 侧的流程重建快照，两侧一起更新。
func TestPromptParity(t *testing.T) {
	for _, lang := range []Language{LanguageZhCN, LanguageEnUS} {
		chart, err := BySolar("2000-8-16", 2, GenderFemale, true, lang, nil)
		if err != nil {
			t.Fatal(err)
		}

		natal, err := chart.AstrolabeToPrompt()
		if err != nil {
			t.Fatal(err)
		}
		if want := promptSnapshot(t, "astrolabe_"+string(lang)); natal != want {
			t.Errorf("%s 本命 prompt 与快照不一致:\n--- got ---\n%s\n--- want ---\n%s", lang, natal, want)
		}

		horo, err := chart.HoroscopeToPrompt("2025-1-1", 0)
		if err != nil {
			t.Fatal(err)
		}
		if want := promptSnapshot(t, "horoscope_"+string(lang)); horo != want {
			t.Errorf("%s 运限 prompt 与快照不一致:\n--- got ---\n%s\n--- want ---\n%s", lang, horo, want)
		}
	}

	// 非法运限目标应返回 error
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := chart.HoroscopeToPrompt("garbage", 0); !errors.Is(err, ErrInvalidDate) {
		t.Fatalf("非法目标日期应报 ErrInvalidDate，实际 %v", err)
	}
}

// TestStarModuleParity 校验 star 模块：与 Python 侧 test_parity.py 的
// test_star_module_matches_chart 断言同一组取值，任一侧漂移都会暴露。
func TestStarModuleParity(t *testing.T) {
	birth := StarBirth{SolarDate: "2000-8-16", TimeIndex: 2, Gender: "female", FixLeap: true}

	start, err := GetStartIndex(birth)
	if err != nil {
		t.Fatal(err)
	}
	if start.ZiweiIndex != 4 || start.TianfuIndex != 8 {
		t.Fatalf("紫微天府起宫 = %+v", start)
	}

	// 单独安主星的结果必须与整盘一致
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	major, err := GetMajorStar(birth)
	if err != nil {
		t.Fatal(err)
	}
	for i, palace := range chart.Palaces {
		if len(palace.MajorStars) != len(major[i]) {
			t.Fatalf("第 %d 宫主星数不一致: 整盘 %d, 单取 %d", i, len(palace.MajorStars), len(major[i]))
		}
		for j, s := range palace.MajorStars {
			if s.Key != major[i][j].Key {
				t.Fatalf("第 %d 宫第 %d 颗主星: 整盘 %s, 单取 %s", i, j, s.Key, major[i][j].Key)
			}
		}
	}

	cs12, err := GetChangsheng12(birth)
	if err != nil {
		t.Fatal(err)
	}
	for i, palace := range chart.Palaces {
		if palace.Changsheng12Key != cs12[i] {
			t.Fatalf("第 %d 宫长生12神: 整盘 %s, 单取 %s", i, palace.Changsheng12Key, cs12[i])
		}
	}

	// 本命层级流耀：iztro 此处把红鸾误写为 hongluanMin，x-iztro 用正确标识
	origin, err := GetHoroscopeStar(StemJia, BranchZi, "origin", "zh-CN")
	if err != nil {
		t.Fatal(err)
	}
	if origin[1][1].Key != "hongluan" {
		t.Fatalf("本命红鸾标识 = %s，应为 hongluan", origin[1][1].Key)
	}

	if idx, err := GetChangsheng12StartIndex("water2nd"); err != nil || idx != 6 {
		t.Fatalf("水二局长生起点 = %d, %v", idx, err)
	}
	if idx, err := GetJiangqian12StartIndex(BranchZi); err != nil || idx != 10 {
		t.Fatalf("子年将前起点 = %d, %v", idx, err)
	}
}

// TestDataModuleParity 校验 data 模块的四张查表。
func TestDataModuleParity(t *testing.T) {
	info, err := StarsInfo()
	if err != nil {
		t.Fatal(err)
	}
	if len(info) != 20 {
		t.Fatalf("星耀信息条数 = %d，应为 20", len(info))
	}
	ziwei := info[StarZiweiMaj]
	if ziwei.FiveElements != "土" || ziwei.YinYang != "阴" || ziwei.Brightness[0] != "wang" {
		t.Fatalf("紫微信息 = %+v", ziwei)
	}
	// 太阳的五行与阴阳在原表中未填
	if info[StarTaiyangMaj].FiveElements != "" || info[StarTaiyangMaj].YinYang != "" {
		t.Fatalf("太阳五行阴阳应为空: %+v", info[StarTaiyangMaj])
	}

	stems, err := HeavenlyStems()
	if err != nil {
		t.Fatal(err)
	}
	jia := stems[StemJia]
	if jia.YinYang != "阳" || jia.FiveElements != "木" || jia.Crash != StemGeng {
		t.Fatalf("甲干信息 = %+v", jia)
	}
	if jia.Mutagen[0] != StarLianzhenMaj || jia.Mutagen[3] != StarTaiyangMaj {
		t.Fatalf("甲干四化 = %v", jia.Mutagen)
	}
	// 戊己无对冲天干
	if stems[StemWu].Crash != "" {
		t.Fatalf("戊干不应有对冲: %q", stems[StemWu].Crash)
	}

	branches, err := EarthlyBranches()
	if err != nil {
		t.Fatal(err)
	}
	zi := branches[BranchZi]
	if zi.Crash != BranchWu || zi.Soul != StarTanlangMaj || zi.Inside != "胆" {
		t.Fatalf("子支信息 = %+v", zi)
	}

	c, err := GetConstants()
	if err != nil {
		t.Fatal(err)
	}
	if len(c.Languages) != 6 || c.Languages[0] != "en-US" {
		t.Fatalf("语言列表 = %v", c.Languages)
	}
	if len(c.ChineseTime) != 13 || c.ChineseTime[12] != "lateRatHour" {
		t.Fatalf("时辰标识 = %v", c.ChineseTime)
	}
	if c.Gender["male"] != "阳" || c.Gender["female"] != "阴" {
		t.Fatalf("性别阴阳 = %v", c.Gender)
	}
	if c.TigerRule[StemJia] != "bingHeavenly" || c.RatRule[StemJia] != StemJia {
		t.Fatalf("遁干规则 = %v / %v", c.TigerRule, c.RatRule)
	}
}

// TestI18nParity 校验标识与译名的双向查找。
func TestI18nParity(t *testing.T) {
	cases := []struct {
		lang Language
		want string
	}{
		{LanguageZhCN, "紫微"},
		{LanguageEnUS, "emperor"},
		{LanguageJaJP, "紫微"},
		{LanguageKoKR, "자미"},
	}
	for _, c := range cases {
		got, err := Translate(StarZiweiMaj, c.lang)
		if err != nil || got != c.want {
			t.Fatalf("Translate(ziweiMaj, %s) = %q, %v", c.lang, got, err)
		}
		key, err := KeyOf(c.want)
		if err != nil || key != StarZiweiMaj {
			t.Fatalf("KeyOf(%q) = %q, %v", c.want, key, err)
		}
	}

	// 非星耀类目同样可查
	if got, _ := Translate(PalaceSoul, "en-US"); got != "soul" {
		t.Fatalf("命宫英文 = %q", got)
	}
	if got, _ := Translate("bodyPalace", "zh-CN"); got != "身宫" {
		t.Fatalf("身宫中文 = %q", got)
	}
	// 未知标识返回空串
	if got, err := Translate("nosuchkey", "zh-CN"); err != nil || got != "" {
		t.Fatalf("未知标识 = %q, %v", got, err)
	}

	// 同形译名靠限定标识名消歧：en-US 的 horse 既是生肖马也是天马
	for _, c := range []struct{ text, filter, want string }{
		{"horse", "", "horse"},
		{"horse", "Min", StarTianmaMin},
		{"dragon", "", "dragon"},
		{"유시", "", "hourly"},
		{"유시", "Hour", "roosterHour"},
		{"horse", "Palace", ""},
	} {
		var got string
		var err error
		if c.filter == "" {
			got, err = KeyOf(c.text)
		} else {
			got, err = KeyOfIn(c.text, c.filter)
		}
		if err != nil || got != c.want {
			t.Fatalf("KeyOf(%q, %q) = %q, %v，应为 %q", c.text, c.filter, got, err, c.want)
		}
	}

	// 标识表的规模、次序与逐条可译性
	keys, err := AllKeys()
	if err != nil || len(keys) != 260 {
		t.Fatalf("AllKeys 共 %d 条, %v", len(keys), err)
	}
	seen := make(map[string]bool, len(keys))
	for _, k := range keys {
		if seen[k] {
			t.Fatalf("标识 %s 重复", k)
		}
		seen[k] = true
		if v, err := Translate(k, "zh-CN"); err != nil || v == "" {
			t.Fatalf("标识 %s 无中文译名: %q, %v", k, v, err)
		}
	}
	// 次序即反查次序：common.json 打头、性别收尾
	if keys[0] != "decadal" || keys[len(keys)-1] != "female" {
		t.Fatalf("标识次序首尾 = %s ... %s", keys[0], keys[len(keys)-1])
	}
}

// TestLocationIndexParity 校验低层落宫函数与整盘、与年系杂耀合并结果一致。
func TestLocationIndexParity(t *testing.T) {
	birth := StarBirth{SolarDate: "2000-8-16", TimeIndex: 2, Gender: "female", FixLeap: true}
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	yearBranch := chart.RawDates.ChineseDate.YearlyKeys[1]

	soulIndex := -1
	for _, p := range chart.Palaces {
		if p.NameKey == PalaceSoul {
			soulIndex = p.Index
		}
	}
	if soulIndex < 0 {
		t.Fatal("整盘无命宫")
	}

	yearly, err := GetYearlyStarIndex(birth)
	if err != nil {
		t.Fatal(err)
	}

	hx, err := GetHuagaiXianchiIndex(yearBranch)
	if err != nil || hx.HuagaiIndex != yearly.HuagaiIndex || hx.XianchiIndex != yearly.XianchiIndex {
		t.Fatalf("华盖咸池 = %+v，年系为 (%d,%d)", hx, yearly.HuagaiIndex, yearly.XianchiIndex)
	}

	gg, err := GetGuGuaIndex(yearBranch)
	if err != nil || gg.GuchenIndex != yearly.GuchenIndex || gg.GuasuIndex != yearly.GuasuIndex {
		t.Fatalf("孤辰寡宿 = %+v，年系为 (%d,%d)", gg, yearly.GuchenIndex, yearly.GuasuIndex)
	}

	if v, err := GetJieshaAdjIndex(yearBranch); err != nil || v != yearly.JieshaAdjIndex {
		t.Fatalf("劫煞 = %d，年系为 %d", v, yearly.JieshaAdjIndex)
	}
	if v, err := GetDahaoIndex(yearBranch); err != nil || v != yearly.DahaoAdjIndex {
		t.Fatalf("大耗 = %d，年系为 %d", v, yearly.DahaoAdjIndex)
	}
	if v, err := GetNianjieIndex(yearBranch); err != nil || v != yearly.NianjieIndex {
		t.Fatalf("年解 = %d，年系为 %d", v, yearly.NianjieIndex)
	}

	ts, err := GetTianshiTianshangIndex("female", yearBranch, soulIndex, nil)
	if err != nil || ts.TianshangIndex != yearly.TianshangIndex || ts.TianshiIndex != yearly.TianshiIndex {
		t.Fatalf("天伤天使 = %+v，年系为 (%d,%d)", ts, yearly.TianshangIndex, yearly.TianshiIndex)
	}

	// 左辅右弼、火星铃星必须落在整盘对应的宫位上
	palaceOf := func(key string) int {
		for _, p := range chart.Palaces {
			for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
				for _, s := range group {
					if s.Key == key {
						return p.Index
					}
				}
			}
		}
		t.Fatalf("整盘找不到星耀 %s", key)
		return -1
	}

	lunar := chart.RawDates.LunarDate
	monthIndex, err := FixLunarMonthIndex(lunar.LunarMonth, lunar.LunarDay, lunar.IsLeap, 2, true)
	if err != nil {
		t.Fatal(err)
	}
	zy, err := GetZuoYouIndex(monthIndex + 1)
	if err != nil || zy.ZuoIndex != palaceOf(StarZuofuMin) || zy.YouIndex != palaceOf(StarYoubiMin) {
		t.Fatalf("左辅右弼 = %+v，整盘为 (%d,%d)", zy, palaceOf(StarZuofuMin), palaceOf(StarYoubiMin))
	}

	hl, err := GetHuoLingIndex(yearBranch, 2)
	if err != nil || hl.HuoIndex != palaceOf(StarHuoxingMin) || hl.LingIndex != palaceOf(StarLingxingMin) {
		t.Fatalf("火星铃星 = %+v，整盘为 (%d,%d)", hl, palaceOf(StarHuoxingMin), palaceOf(StarLingxingMin))
	}

	// 按天干的昌曲用于运限层级：与流耀分布里的流昌流曲同宫
	decadal, err := GetHoroscopeStar(StemJia, BranchZi, "decadal", "zh-CN")
	if err != nil {
		t.Fatal(err)
	}
	cq, err := GetChangQuIndexByHeavenlyStem(StemJia)
	if err != nil {
		t.Fatal(err)
	}
	hasStar := func(group []Star, key string) bool {
		for _, s := range group {
			if s.Key == key {
				return true
			}
		}
		return false
	}
	if !hasStar(decadal[cq.ChangIndex], "yunchang") || !hasStar(decadal[cq.QuIndex], "yunqu") {
		t.Fatalf("按天干昌曲 = %+v，与流耀分布不符", cq)
	}
}

// TestDecadalsAndAgesParity 校验单取大限小限与整盘一致。
func TestDecadalsAndAgesParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	soulIndex := -1
	for _, p := range chart.Palaces {
		if p.NameKey == PalaceSoul {
			soulIndex = p.Index
		}
	}
	if soulIndex < 0 {
		t.Fatal("未找到命宫")
	}

	got, err := GetDecadalsAndAges(
		soulIndex,
		chart.FiveElementsClassKey,
		chart.GenderKey,
		chart.RawDates.ChineseDate.YearlyKeys[0],
		chart.RawDates.ChineseDate.YearlyKeys[1],
	)
	if err != nil {
		t.Fatal(err)
	}
	for i, p := range chart.Palaces {
		if p.Decadal.Range != got.Decadals[i].Range {
			t.Fatalf("第 %d 宫大限区间: 整盘 %v, 单取 %v", i, p.Decadal.Range, got.Decadals[i].Range)
		}
		// 译名与标识两组字段的含义在整盘与单取之间必须一致
		if p.Decadal.HeavenlyStem != got.Decadals[i].HeavenlyStem ||
			p.Decadal.HeavenlyStemKey != got.Decadals[i].HeavenlyStemKey ||
			p.Decadal.EarthlyBranch != got.Decadals[i].EarthlyBranch ||
			p.Decadal.EarthlyBranchKey != got.Decadals[i].EarthlyBranchKey {
			t.Fatalf("第 %d 宫大限干支: 整盘 %+v, 单取 %+v", i, p.Decadal, got.Decadals[i])
		}
		if len(p.Ages) != len(got.Ages[i]) {
			t.Fatalf("第 %d 宫小限数: 整盘 %d, 单取 %d", i, len(p.Ages), len(got.Ages[i]))
		}
	}
}

// TestPalaceBackReferences 校验宫位自身可取对宫与三方四正，
// 与 Python 侧 test_palace_back_references_to_opposite_and_surrounded 断言同一组取值。
func TestPalaceBackReferences(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	soul := chart.Palace(PalaceSoul)
	if soul == nil {
		t.Fatal("未找到命宫")
	}

	opposite := soul.OppositePalace()
	if opposite.NameKey != PalaceSurface {
		t.Fatalf("命宫对宫 = %s，应为迁移", opposite.NameKey)
	}
	if opposite.Index != (soul.Index+6)%12 {
		t.Fatalf("对宫索引 = %d，应为 %d", opposite.Index, (soul.Index+6)%12)
	}

	fromPalace := soul.SurroundedPalaces()
	fromChart := chart.SurroundedPalaces(PalaceSoul)
	pairs := [][2]*Palace{
		{fromPalace.Target, fromChart.Target},
		{fromPalace.Opposite, fromChart.Opposite},
		{fromPalace.Wealth, fromChart.Wealth},
		{fromPalace.Career, fromChart.Career},
	}
	for i, pair := range pairs {
		if pair[0].Index != pair[1].Index {
			t.Fatalf("三方四正第 %d 项: 宫位取 %d, 星盘取 %d", i, pair[0].Index, pair[1].Index)
		}
	}

	// 十二宫的对宫互为对方
	for i := range chart.Palaces {
		p := &chart.Palaces[i]
		if p.OppositePalace().OppositePalace().Index != p.Index {
			t.Fatalf("第 %d 宫对宫的对宫不是自己", p.Index)
		}
	}
}

// TestConfigConstantsParity 校验六组配置常量齐备且取值正确。
// 与 Python 侧 x_iztro.enums 的同名枚举一一对应。
func TestConfigConstantsParity(t *testing.T) {
	pairs := [][2]string{
		{YearDivideNormal, "normal"}, {YearDivideExact, "exact"},
		{HoroscopeDivideNormal, "normal"}, {HoroscopeDivideExact, "exact"},
		{AgeDivideNormal, "normal"}, {AgeDivideBirthday, "birthday"},
		{DayDivideForward, "forward"}, {DayDivideCurrent, "current"},
		{AlgorithmDefault, "default"}, {AlgorithmZhongzhou, "zhongzhou"},
		{AstroHeaven, "heaven"}, {AstroEarth, "earth"}, {AstroHuman, "human"},
	}
	for _, p := range pairs {
		if p[0] != p[1] {
			t.Fatalf("常量取值 = %q，应为 %q", p[0], p[1])
		}
	}

	// 每组常量都要能实际驱动排盘
	zz, err := BySolar("2000-8-16", 2, "female", true, "zh-CN",
		&Config{Algorithm: AlgorithmZhongzhou, YearDivide: YearDivideExact})
	if err != nil || zz.FiveElementsClass == "" {
		t.Fatalf("中州派排盘失败: %v", err)
	}
	late, err := BySolar("2000-8-16", 12, "female", true, "zh-CN",
		&Config{DayDivide: DayDivideCurrent})
	if err != nil || late.Time == "" {
		t.Fatalf("晚子时归当日排盘失败: %v", err)
	}
}

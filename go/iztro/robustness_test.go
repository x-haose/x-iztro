package iztro

import (
	"errors"
	"strings"
	"testing"
)

// 错误契约与零值安全测试。
//
// 前半段固定每类错误的 Code 与哨兵；后半段保证查不到的宫位、星耀、零值星盘
// 一路传下去不会 panic——查询方法返回 nil 与 false，由调用方自行判空。

// TestErrorClassification 校验各类非法输入落到正确的错误类别。
func TestErrorClassification(t *testing.T) {
	cases := []struct {
		name     string
		call     func() error
		sentinel error
		code     string
		message  string
	}{
		{
			name:     "月份越界",
			call:     func() error { _, err := BySolar("2000-13-1", 2, GenderMale, true, LanguageZhCN, nil); return err },
			sentinel: ErrInvalidDate,
			code:     CodeInvalidDate,
			message:  "invalid solar date '2000-13-1'",
		},
		{
			name:     "公历下限",
			call:     func() error { _, err := BySolar("1500-8-16", 2, GenderMale, true, LanguageZhCN, nil); return err },
			sentinel: ErrInvalidDate,
			code:     CodeInvalidDate,
			message:  "year must be within 1583-9999",
		},
		{
			name:     "时辰越界",
			call:     func() error { _, err := BySolar("2000-8-16", 13, GenderMale, true, LanguageZhCN, nil); return err },
			sentinel: ErrInvalidTimeIndex,
			code:     CodeInvalidTimeIndex,
			message:  "13",
		},
		{
			name:     "未知性别",
			call:     func() error { _, err := BySolar("2000-8-16", 2, "x", true, LanguageZhCN, nil); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "invalid gender 'x': expected 'male' or 'female'",
		},
		{
			name:     "未知语言",
			call:     func() error { _, err := BySolar("2000-8-16", 2, GenderMale, true, "xx", nil); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "invalid language 'xx'",
		},
		{
			name:     "未知星耀标识",
			call:     func() error { _, err := GetBrightness("nope", 0, nil); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "unknown star key 'nope'",
		},
		{
			name:     "未知地支标识（Go 侧校验）",
			call:     func() error { _, err := EarthlyBranchToPalaceIndex("nope"); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "unknown earthly branch key 'nope'",
		},
		{
			name:     "小时越界（Go 侧校验）",
			call:     func() error { _, err := TimeToIndex(24); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "invalid hour '24': expected 0-23",
		},
		{
			name:     "FixIndex 负上限（Go 侧校验）",
			call:     func() error { _, err := FixIndex(1, -3); return err },
			sentinel: ErrInvalidArgument,
			code:     CodeInvalidArgument,
			message:  "invalid max '-3': expected a positive integer",
		},
		{
			name: "农历日越界",
			call: func() error {
				_, err := ByLunar("2000-7-31", 2, GenderMale, false, true, LanguageZhCN, nil)
				return err
			},
			sentinel: ErrInvalidDate,
			code:     CodeInvalidDate,
			message:  "2000-7-31",
		},
	}

	for _, c := range cases {
		err := c.call()
		if err == nil {
			t.Errorf("%s: 应返回错误", c.name)
			continue
		}
		if !errors.Is(err, c.sentinel) {
			t.Errorf("%s: errors.Is 未匹配哨兵，实际 %v", c.name, err)
		}
		var e *Error
		if !errors.As(err, &e) {
			t.Errorf("%s: errors.As 应取到 *Error，实际 %T", c.name, err)
			continue
		}
		if e.Code != c.code {
			t.Errorf("%s: Code = %s，期望 %s", c.name, e.Code, c.code)
		}
		if !strings.Contains(e.Message, c.message) {
			t.Errorf("%s: Message = %q，应含 %q", c.name, e.Message, c.message)
		}
		if !strings.HasPrefix(err.Error(), "iztro: ") {
			t.Errorf("%s: Error() 应带 iztro: 前缀，实际 %q", c.name, err.Error())
		}
	}
}

// TestErrorSentinelsAreDistinct 校验四个哨兵互不相等，按类别判断不会串。
func TestErrorSentinelsAreDistinct(t *testing.T) {
	dateErr := &Error{Code: CodeInvalidDate, Message: "x"}
	if errors.Is(dateErr, ErrInvalidArgument) || errors.Is(dateErr, ErrInternal) {
		t.Error("invalid_date 不应匹配其他哨兵")
	}
	// 无法识别的 Code 归为内部缺陷
	unknown := &Error{Code: "brand_new", Message: "x"}
	if !errors.Is(unknown, ErrInternal) {
		t.Error("未知 Code 应归为 ErrInternal")
	}
}

// TestZeroValueAstrolabeIsSafe 校验零值星盘上的查询返回 nil 而非 panic。
func TestZeroValueAstrolabeIsSafe(t *testing.T) {
	var zero Astrolabe
	if zero.Palace(PalaceSoul) != nil {
		t.Error("零值星盘不应查到宫位")
	}
	if zero.PalaceByIndex(0) != nil {
		t.Error("零值星盘不应查到索引宫位")
	}
	if zero.SurroundedPalacesByIndex(0) != nil {
		t.Error("零值星盘不应给出三方四正")
	}
	if zero.SurroundedPalaces(PalaceSoul) != nil {
		t.Error("零值星盘不应给出三方四正")
	}
	if star, palace := zero.Star(StarZiweiMaj); star != nil || palace != nil {
		t.Error("零值星盘不应查到星耀")
	}
	if zero.IsSurrounded(PalaceSoul, StarZiweiMaj) || zero.IsSurroundedOneOf(PalaceSoul, StarZiweiMaj) {
		t.Error("零值星盘的三方四正判断应为 false")
	}
	if zero.NotSurrounded(PalaceSoul, StarZiweiMaj) {
		t.Error("三方四正取不到时 NotSurrounded 也应为 false")
	}
}

// TestNilPalaceAndStarAreSafe 校验查不到的宫位与星耀继续调用不会 panic。
func TestNilPalaceAndStarAreSafe(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	missing := chart.Palace("没有这个宫")
	if missing != nil {
		t.Fatal("不存在的宫名应返回 nil")
	}
	if missing.Has(StarZiweiMaj) || missing.HasOneOf(StarZiweiMaj) || missing.HasMutagen(MutagenLu) {
		t.Error("nil 宫位的包含判断应为 false")
	}
	if !missing.NotHave(StarZiweiMaj) || !missing.NotHaveMutagen(MutagenLu) || !missing.IsEmpty() {
		t.Error("nil 宫位的否定判断应为 true")
	}
	if missing.Astrolabe() != nil || missing.OppositePalace() != nil || missing.SurroundedPalaces() != nil {
		t.Error("nil 宫位的关系查询应返回 nil")
	}
	if missing.MutagenStars(MutagenLu) != nil || missing.MutagedPlaces() != nil {
		t.Error("nil 宫位的四化查询应返回 nil")
	}
	if missing.FliesTo(chart.Palace(PalaceSoul), MutagenLu) {
		t.Error("nil 宫位不应飞化")
	}
	if chart.Palace(PalaceSoul).FliesTo(missing, MutagenLu) {
		t.Error("飞入 nil 宫位应为 false")
	}

	star, palace := chart.Star("没有这颗星")
	if star != nil || palace != nil {
		t.Fatal("不存在的星名应返回 nil")
	}
	if star.WithMutagen(MutagenLu) || star.WithBrightness(BrightnessMiao) {
		t.Error("nil 星耀的判断应为 false")
	}
	if star.Palace() != nil || star.OppositePalace() != nil || star.SurroundedPalaces() != nil {
		t.Error("nil 星耀的关系查询应返回 nil")
	}
}

// TestNilHoroscopeIsSafe 校验 nil 运限上的查询返回 nil/false。
func TestNilHoroscopeIsSafe(t *testing.T) {
	var h *Horoscope
	if h.Astrolabe() != nil || h.ScopeItem(ScopeYearly) != nil || h.AgePalace() != nil {
		t.Error("nil 运限的查询应返回 nil")
	}
	if h.Palace(PalaceSoul, ScopeYearly) != nil || h.SurroundPalaces(PalaceSoul, ScopeYearly) != nil {
		t.Error("nil 运限的宫位查询应返回 nil")
	}
	if h.HasHoroscopeMutagen(PalaceSoul, ScopeYearly, MutagenLu) {
		t.Error("nil 运限的四化判断应为 false")
	}
	var item *HoroscopeScope
	if item.PalaceIndexByName(PalaceSoul) != -1 {
		t.Error("nil 层级的宫位索引应为 -1")
	}
}

// TestPalaceMethodsOnUnlinkedPalace 校验未挂盘的宫位仍可做星耀包含判断。
func TestPalaceMethodsOnUnlinkedPalace(t *testing.T) {
	p := &Palace{MajorStars: []Star{{Key: StarZiweiMaj, Name: "紫微"}}}
	if !p.Has(StarZiweiMaj) || !p.Has("紫微") {
		t.Error("未挂盘的宫位应能按 key 与译名判断包含")
	}
	if p.IsEmpty() {
		t.Error("有主星的宫位不是空宫")
	}
	if p.Astrolabe() != nil {
		t.Error("未挂盘的宫位没有星盘")
	}
}

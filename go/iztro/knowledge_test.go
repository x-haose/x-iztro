package iztro

import (
	"errors"
	"reflect"
	"strings"
	"testing"
)

// TestTextWithKnowledgeParity 断言五类带释义文本与 Rust 侧快照逐字节相同
// （固定盘 2000-8-16 时辰 2 女命 zh-CN，运限 2025-1-1 时辰 0，palace/surrounded 取命宫，
// patterns 默认口径），并确认零值 Knowledge 的输出仍等于不带释义的原快照。
func TestTextWithKnowledgeParity(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	horoscope, err := chart.Horoscope("2025-1-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	soul := PalaceTarget{Key: PalaceSoul}
	cases := map[string]func(Knowledge) (string, error){
		"astrolabe": chart.ToTextWith,
		"horoscope": horoscope.ToTextWith,
		"patterns":  func(k Knowledge) (string, error) { return chart.PatternsToTextWith(nil, k) },
		"palace":    func(k Knowledge) (string, error) { return chart.PalaceToTextWith(soul, k) },
		"surrounded": func(k Knowledge) (string, error) {
			return chart.SurroundedPalacesToTextWith(soul, k)
		},
	}
	for name, get := range cases {
		got, err := get(BuiltinKnowledge())
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		if want := textSnapshot(t, name+"_knowledge_zh-CN"); got != want {
			t.Errorf("%s 带释义文本与快照不一致:\n--- got ---\n%s\n--- want ---\n%s", name, got, want)
		}
		plain, err := get(Knowledge{})
		if err != nil {
			t.Fatalf("%s: %v", name, err)
		}
		if want := textSnapshot(t, name+"_zh-CN"); plain != want {
			t.Errorf("%s 零值 Knowledge 应等于无释义快照", name)
		}
	}

	// 英文盘没有内嵌包：报错而非静默回退
	en, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageEnUS, nil)
	if err != nil {
		t.Fatal(err)
	}
	if _, err := en.ToTextWith(BuiltinKnowledge()); !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("en-US builtin knowledge should be ErrInvalidArgument, got %v", err)
	}

	// 自定义包：覆盖紫微解读后释义节应含覆盖文本
	base, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	overlay, err := ParseKnowledgePack([]byte(`{"schema":1,"id":"mine","version":"1","language":"zh-CN","extends":"iztro-docs",
		"stars":{"ziweiMaj":{"intro":"我的紫微释义"}}}`))
	if err != nil {
		t.Fatal(err)
	}
	merged, err := base.Merged(overlay)
	if err != nil {
		t.Fatal(err)
	}
	custom, err := chart.ToTextWith(KnowledgeFrom(merged))
	if err != nil {
		t.Fatal(err)
	}
	if !strings.Contains(custom, "我的紫微释义") {
		t.Fatal("custom pack intro should appear in text")
	}
	if strings.Contains(custom, base.StarIntro(StarZiweiMaj)) {
		t.Fatal("overridden builtin intro should not appear")
	}
}

// TestKnowledgeForChart 校验按盘取材的子包：stars 恰为盘上出现的星，mutagens 4 条，
// palaces/concepts 为空；合并后的自定义包与就地改过条目的内嵌包都以当前内容取材。
func TestKnowledgeForChart(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	builtin, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	onChart := map[string]struct{}{}
	for i := range chart.Palaces {
		p := &chart.Palaces[i]
		for _, group := range [][]Star{p.MajorStars, p.MinorStars, p.AdjectiveStars} {
			for _, s := range group {
				onChart[s.Key] = struct{}{}
			}
		}
		for _, k := range []string{p.Changsheng12Key, p.Boshi12Key, p.Jiangqian12Key, p.Suiqian12Key} {
			onChart[k] = struct{}{}
		}
	}

	sub, err := builtin.ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	for key := range sub.Stars {
		if _, ok := onChart[key]; !ok {
			t.Errorf("sub pack has star %s not on chart", key)
		}
	}
	for key := range onChart {
		if builtin.Star(key) != nil && sub.Star(key) == nil {
			t.Errorf("chart star %s missing from sub pack", key)
		}
	}
	if len(sub.Mutagens) != 4 || len(sub.Palaces) != 0 || len(sub.Concepts) != 0 {
		t.Fatalf("mutagens %d palaces %d concepts %d", len(sub.Mutagens), len(sub.Palaces), len(sub.Concepts))
	}
	hits, err := chart.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	if len(sub.Patterns) != len(hits) {
		t.Fatalf("patterns %d, hits %d", len(sub.Patterns), len(hits))
	}
	if sub.ID != builtin.ID || sub.Source.License != builtin.Source.License {
		t.Fatal("sub pack should keep source metadata")
	}

	// 自定义包走整包发送路径，覆盖文本应进入子包
	overlay, err := ParseKnowledgePack([]byte(`{"schema":1,"id":"mine","version":"1","language":"zh-CN","extends":"iztro-docs",
		"stars":{"ziweiMaj":{"intro":"我的紫微释义"}}}`))
	if err != nil {
		t.Fatal(err)
	}
	merged, err := builtin.Merged(overlay)
	if err != nil {
		t.Fatal(err)
	}
	customSub, err := merged.ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	if customSub.StarIntro(StarZiweiMaj) != "我的紫微释义" || len(customSub.Stars) != len(sub.Stars) {
		t.Fatalf("custom sub pack: intro %q stars %d", customSub.StarIntro(StarZiweiMaj), len(customSub.Stars))
	}

	// 就地改内嵌包的条目后取材，子包里是改后的文本
	edited := builtin.Stars[StarZiweiMaj]
	edited.Intro = "就地改的紫微"
	builtin.Stars[StarZiweiMaj] = edited
	editedSub, err := builtin.ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	if editedSub.StarIntro(StarZiweiMaj) != "就地改的紫微" {
		t.Fatalf("in-place edit should reach sub pack, got %q", editedSub.StarIntro(StarZiweiMaj))
	}

	// 运限取材是本命取材的超集
	horoscope, err := chart.Horoscope("2025-1-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	hsub, err := builtin.ForHoroscope(horoscope, nil)
	if err != nil {
		t.Fatal(err)
	}
	for key := range sub.Stars {
		if hsub.Star(key) == nil {
			t.Errorf("horoscope sub pack missing natal star %s", key)
		}
	}
	if len(hsub.Stars) <= len(sub.Stars) {
		t.Fatalf("horoscope sub pack should add flow stars: %d vs %d", len(hsub.Stars), len(sub.Stars))
	}

	var nilPack *KnowledgePack
	if _, err := nilPack.ForAstrolabe(chart, nil); !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("nil pack should be ErrInvalidArgument, got %v", err)
	}
	if _, err := builtin.ForAstrolabe(nil, nil); !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("nil chart should be ErrInvalidArgument, got %v", err)
	}
}

// TestKnowledgeForChartSources 校验 BuiltinKnowledge 哨兵与整包发送的内嵌包取材结果逐字段相同，
// 零值 Knowledge 取材报 ErrInvalidArgument。
func TestKnowledgeForChartSources(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	pack, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	fromSentinel, err := BuiltinKnowledge().ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	fromPack, err := pack.ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(fromSentinel, fromPack) {
		t.Fatalf("builtin sentinel and builtin pack should draw the same sub pack:\n%+v\n%+v", fromSentinel, fromPack)
	}
	horoscope, err := chart.Horoscope("2025-1-1", 0)
	if err != nil {
		t.Fatal(err)
	}
	hFromSentinel, err := BuiltinKnowledge().ForHoroscope(horoscope, nil)
	if err != nil {
		t.Fatal(err)
	}
	hFromPack, err := pack.ForHoroscope(horoscope, nil)
	if err != nil {
		t.Fatal(err)
	}
	if !reflect.DeepEqual(hFromSentinel, hFromPack) {
		t.Fatal("builtin sentinel and builtin pack should draw the same horoscope sub pack")
	}

	if _, err := (Knowledge{}).ForAstrolabe(chart, nil); !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("zero Knowledge should be ErrInvalidArgument, got %v", err)
	}
	if _, err := (Knowledge{}).ForHoroscope(horoscope, nil); !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("zero Knowledge should be ErrInvalidArgument, got %v", err)
	}
}

// TestKnowledgeForChartPatternConfig 校验取材时的格局口径随 config 传给内核：
// 子包 patterns 键集合恰等于同口径 Patterns 命中 key 的去重集合。
func TestKnowledgeForChartPatternConfig(t *testing.T) {
	// 1990-1-10 子时男：默认亮度表口径无格，位置口径命中日月并明
	chart, err := BySolar("1990-1-10", 0, GenderMale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	cfg := &PatternConfig{BrightnessSource: BrightnessSourcePositional}
	hitKeys := func(config *PatternConfig) map[string]struct{} {
		hits, err := chart.Patterns(config)
		if err != nil {
			t.Fatal(err)
		}
		keys := map[string]struct{}{}
		for _, h := range hits {
			keys[h.Key] = struct{}{}
		}
		return keys
	}
	packKeys := func(p *KnowledgePack) map[string]struct{} {
		keys := map[string]struct{}{}
		for k := range p.Patterns {
			keys[k] = struct{}{}
		}
		return keys
	}
	defaultHits, positionalHits := hitKeys(nil), hitKeys(cfg)
	if reflect.DeepEqual(defaultHits, positionalHits) {
		t.Fatalf("fixture chart must distinguish the two criteria: %v", defaultHits)
	}
	defaultSub, err := BuiltinKnowledge().ForAstrolabe(chart, nil)
	if err != nil {
		t.Fatal(err)
	}
	positionalSub, err := BuiltinKnowledge().ForAstrolabe(chart, cfg)
	if err != nil {
		t.Fatal(err)
	}
	if got := packKeys(defaultSub); !reflect.DeepEqual(got, defaultHits) {
		t.Fatalf("default sub pack patterns %v, hits %v", got, defaultHits)
	}
	if got := packKeys(positionalSub); !reflect.DeepEqual(got, positionalHits) {
		t.Fatalf("positional sub pack patterns %v, hits %v", got, positionalHits)
	}
	if _, ok := positionalSub.Patterns[PatternRiYueBingMing]; !ok {
		t.Fatalf("positional sub pack should carry 日月并明, got %v", packKeys(positionalSub))
	}
}

func TestBuiltinKnowledgePack(t *testing.T) {
	p, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	if p.Schema != 1 || p.ID != "iztro-docs" || p.Language != "zh-CN" || p.Source.License != "MIT" {
		t.Fatalf("metadata: %+v", *p)
	}
	zi := p.Star(StarZiweiMaj)
	if zi == nil || zi.Name != "紫微" || zi.Attributes.YinYang != "yin" || zi.Attributes.FiveElements != "earth" {
		t.Fatalf("ziwei entry: %+v", zi)
	}
	if _, ok := zi.Combinations[StarTianfuMaj]; !ok {
		t.Fatal("ziwei should have tianfu combination")
	}
	if p.Pattern(PatternZiFuTongGong) == nil || p.PatternIntro(PatternZiFuTongGong) == "" {
		t.Fatal("pattern entry")
	}
	if p.Palace(PalaceSoul) == nil || p.Mutagen(MutagenJi) == nil || p.Concept("tong-gong") == nil {
		t.Fatal("palace/mutagen/concept entries")
	}
	if len(p.Patterns) != 64 {
		t.Fatalf("patterns: %d", len(p.Patterns))
	}
	// 概念条目数钉死：绑定层若丢字段或默认包被误删条目，这里立即报
	if len(p.Concepts) != 49 {
		t.Fatalf("concepts: %d", len(p.Concepts))
	}
	if p.Star("nope") != nil || p.StarIntro("nope") != "" {
		t.Fatal("unknown key should be nil/empty")
	}
	if _, err := BuiltinKnowledgePack(LanguageEnUS); err == nil {
		t.Fatal("en-US has no builtin pack")
	}
	a, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	hits, err := a.Patterns(nil)
	if err != nil {
		t.Fatal(err)
	}
	for _, h := range hits {
		if p.Pattern(h.Key) == nil {
			t.Fatalf("no knowledge for pattern %s", h.Key)
		}
	}
}

func TestKnowledgePackMerge(t *testing.T) {
	base, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	overlay, err := ParseKnowledgePack([]byte(`{"schema":1,"id":"mine","version":"1","language":"zh-CN","extends":"iztro-docs",
		"stars":{"ziweiMaj":{"intro":"我的紫微","attributes":{"aliases":["我的别号"]}}},
		"patterns":{"zi_fu_tong_gong":{"intro":"我的紫府同宫"}}}`))
	if err != nil {
		t.Fatal(err)
	}
	m, err := base.Merged(overlay)
	if err != nil {
		t.Fatal(err)
	}
	zi := m.Star(StarZiweiMaj)
	if m.ID != "mine" || zi.Intro != "我的紫微" || zi.Name != "紫微" || len(zi.Attributes.Aliases) != 1 || zi.Attributes.Chemistry != base.Star(StarZiweiMaj).Attributes.Chemistry {
		t.Fatalf("merge: %+v", zi)
	}
	if m.PatternIntro(PatternZiFuTongGong) != "我的紫府同宫" || len(m.Pattern(PatternZiFuTongGong).Quotes) != len(base.Pattern(PatternZiFuTongGong).Quotes) {
		t.Fatal("pattern merge")
	}
	if base.StarIntro(StarZiweiMaj) == "我的紫微" {
		t.Fatal("base must stay unchanged")
	}
	if _, err := base.Merged(nil); err == nil {
		t.Fatal("nil overlay must error")
	}
	bad, _ := ParseKnowledgePack([]byte(`{"schema":99}`))
	if _, err := base.Merged(bad); err == nil {
		t.Fatal("newer schema must error")
	}
	if _, err := ParseKnowledgePack([]byte(`nope`)); err == nil {
		t.Fatal("bad json must error")
	}
}

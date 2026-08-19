package iztro

import "testing"

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

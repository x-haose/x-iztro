package iztro

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"testing"
)

// 知识包校验：四侧共用用例 tests/data/knowledge_check_cases.json（Rust knowledge_check.rs、
// Python test_knowledge_check.py、JS knowledge-check.test.ts 读同一份），默认包无问题，
// ParseKnowledgePack 按 error 级问题拒绝。

type checkCase struct {
	Name     string           `json:"name"`
	Pack     *json.RawMessage `json:"pack"`
	PackJSON *string          `json:"packJson"`
	Base     string           `json:"base"`
	Expect   []map[string]any `json:"expect"`
}

func loadCheckCases(t *testing.T) []checkCase {
	t.Helper()
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "knowledge_check_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		Cases []checkCase `json:"cases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	return doc.Cases
}

// caseText 为用例的 JSON 原文：packJson 原样，pack 取其 JSON（含 null）。
func caseText(c checkCase) []byte {
	if c.PackJSON != nil {
		return []byte(*c.PackJSON)
	}
	if c.Pack == nil {
		return []byte("null")
	}
	return *c.Pack
}

// issueShape 把问题落成与用例同形的值，经 JSON 往返使数字与 any 的类型一致。
func issueShape(t *testing.T, issue KnowledgeIssue, withParams bool) map[string]any {
	t.Helper()
	v := map[string]any{"level": issue.Level, "path": []any(issue.Path), "code": issue.Code}
	if withParams {
		v["params"] = issue.Params
	}
	data, err := json.Marshal(v)
	if err != nil {
		t.Fatal(err)
	}
	var out map[string]any
	if err := json.Unmarshal(data, &out); err != nil {
		t.Fatal(err)
	}
	return out
}

func TestKnowledgeCheckSharedCases(t *testing.T) {
	builtin, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	for _, c := range loadCheckCases(t) {
		var base *KnowledgePack
		if c.Base == "builtin" {
			base = builtin
		}
		issues, err := CheckKnowledgePack(caseText(c), base)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		got := make([]map[string]any, 0, len(issues))
		for i, issue := range issues {
			withParams := false
			if i < len(c.Expect) {
				_, withParams = c.Expect[i]["params"]
			}
			got = append(got, issueShape(t, issue, withParams))
		}
		want := c.Expect
		if want == nil {
			want = []map[string]any{}
		}
		if !reflect.DeepEqual(got, want) {
			t.Errorf("%s\n  expect %v\n  got    %v", c.Name, want, got)
		}
	}
}

func TestKnowledgeCheckBuiltinHasNoIssues(t *testing.T) {
	builtin, err := BuiltinKnowledgePack(LanguageZhCN)
	if err != nil {
		t.Fatal(err)
	}
	data, err := json.Marshal(builtin)
	if err != nil {
		t.Fatal(err)
	}
	issues, err := CheckKnowledgePack(data, nil)
	if err != nil || len(issues) != 0 {
		t.Fatalf("默认包应无问题：%v %v", issues, err)
	}
}

func TestParseKnowledgePackRejectsErrors(t *testing.T) {
	for _, c := range loadCheckCases(t) {
		if c.Base != "" {
			continue
		}
		hasError := false
		for _, e := range c.Expect {
			if e["level"] == "error" {
				hasError = true
			}
		}
		_, err := ParseKnowledgePack(caseText(c))
		if (err != nil) != hasError {
			t.Errorf("%s: err = %v", c.Name, err)
			continue
		}
		if err == nil {
			continue
		}
		var e *Error
		if !errors.As(err, &e) || !errors.Is(err, ErrInvalidArgument) || len(e.Issues) != len(c.Expect) {
			t.Errorf("%s: %v", c.Name, err)
		}
	}
}

// TestParseKnowledgePackSharedParseCases 由原文构造的包与其他语言是同一份：键区分大小写、
// 映射里的 null 值等同省略（encoding/json 直接解原文会不分大小写收键、把 null 解成空串）。
func TestParseKnowledgePackSharedParseCases(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "knowledge_check_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		ParseCases []struct {
			Name         string            `json:"name"`
			PackJSON     string            `json:"packJson"`
			Star         string            `json:"star"`
			Intro        *string           `json:"intro"`
			Combinations map[string]string `json:"combinations"`
		} `json:"parseCases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	for _, c := range doc.ParseCases {
		pack, err := ParseKnowledgePack([]byte(c.PackJSON))
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		star := pack.Star(c.Star)
		want := ""
		if c.Intro != nil {
			want = *c.Intro
		}
		got := star.Combinations
		if got == nil {
			got = map[string]string{}
		}
		if star.Intro != want || !reflect.DeepEqual(got, c.Combinations) {
			t.Errorf("%s: intro %q combinations %v", c.Name, star.Intro, star.Combinations)
		}
	}
}

// TestSharedTopicOrderCases 第一个包为底、其余依次合并，主题的显示顺序与其他语言一致。
func TestSharedTopicOrderCases(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "knowledge_check_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		TopicOrderCases []struct {
			Name   string            `json:"name"`
			Packs  []json.RawMessage `json:"packs"`
			Topics []string          `json:"topics"`
		} `json:"topicOrderCases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	for _, c := range doc.TopicOrderCases {
		packs := make([]*KnowledgePack, len(c.Packs))
		for i, raw := range c.Packs {
			if packs[i], err = ParseKnowledgePack(raw); err != nil {
				t.Fatalf("%s: %v", c.Name, err)
			}
		}
		merged, err := packs[0].Merged(packs[1:]...)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		if got := merged.TopicSlugs(); !reflect.DeepEqual(got, c.Topics) {
			t.Errorf("%s: %v", c.Name, got)
		}
	}
}

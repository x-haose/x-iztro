package iztro

import (
	"encoding/json"
	"errors"
	"os"
	"path/filepath"
	"reflect"
	"strings"
	"testing"
)

// 按主题选宫：四侧共用用例 tests/data/topic_palaces_cases.json（Rust topic_palaces.rs 按各层宫名另行推得并
// 核对期望，Python test_topic.py、JS topic.test.ts 读同一份），以及层级比查询精度细时报错。

type topicCase struct {
	Name  string `json:"name"`
	Chart struct {
		SolarDate string `json:"solarDate"`
		TimeIndex uint8  `json:"timeIndex"`
		Gender    Gender `json:"gender"`
	} `json:"chart"`
	Target *struct {
		TargetDate      string `json:"targetDate"`
		TargetTimeIndex uint8  `json:"targetTimeIndex"`
		TargetYear      int    `json:"targetYear"`
		TargetMonth     int    `json:"targetMonth"`
	} `json:"target"`
	Scope     string          `json:"scope"`
	Topic     string          `json:"topic"`
	Knowledge json.RawMessage `json:"knowledge"`
	Expect    TopicSelection  `json:"expect"`
}

func TestTopicPalacesSharedCases(t *testing.T) {
	data, err := os.ReadFile(filepath.Join("..", "..", "tests", "data", "topic_palaces_cases.json"))
	if err != nil {
		t.Fatal(err)
	}
	var doc struct {
		Cases []topicCase `json:"cases"`
	}
	if err := json.Unmarshal(data, &doc); err != nil {
		t.Fatal(err)
	}
	for _, c := range doc.Cases {
		chart, err := BySolar(c.Chart.SolarDate, c.Chart.TimeIndex, c.Chart.Gender, true, LanguageZhCN, nil)
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		knowledge := BuiltinKnowledge()
		if c.Knowledge != nil {
			pack, err := ParseKnowledgePack(c.Knowledge)
			if err != nil {
				t.Fatalf("%s: %v", c.Name, err)
			}
			knowledge = KnowledgeFrom(pack)
		}
		var got *TopicSelection
		switch {
		case c.Target == nil:
			got, err = chart.TopicPalaces(c.Topic, knowledge)
		default:
			var h *Horoscope
			switch {
			case c.Target.TargetDate != "":
				h, err = chart.Horoscope(c.Target.TargetDate, c.Target.TargetTimeIndex)
			case c.Target.TargetMonth != 0:
				h, err = chart.HoroscopeOfMonth(c.Target.TargetYear, c.Target.TargetMonth)
			default:
				h, err = chart.HoroscopeOfYear(c.Target.TargetYear)
			}
			if err != nil {
				t.Fatalf("%s: %v", c.Name, err)
			}
			got, err = h.TopicPalaces(c.Topic, c.Scope, knowledge)
		}
		if err != nil {
			t.Fatalf("%s: %v", c.Name, err)
		}
		if !reflect.DeepEqual(*got, c.Expect) {
			t.Errorf("%s:\n got %+v\nwant %+v", c.Name, *got, c.Expect)
		}
	}
}

func TestTopicPalacesScopeFinerThanPrecision(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, GenderFemale, true, LanguageZhCN, nil)
	if err != nil {
		t.Fatal(err)
	}
	h, err := chart.HoroscopeOfYear(2025)
	if err != nil {
		t.Fatal(err)
	}
	_, err = h.TopicPalaces("romance", ScopeMonthly, BuiltinKnowledge())
	if !errors.Is(err, ErrInvalidArgument) || !strings.Contains(err.Error(), "only specifies layers down to 'yearly'") {
		t.Fatalf("got %v", err)
	}
	_, err = h.TopicToText("romance", ScopeMonthly, TextOptions{Knowledge: BuiltinKnowledge()})
	if !errors.Is(err, ErrInvalidArgument) {
		t.Fatalf("got %v", err)
	}
}

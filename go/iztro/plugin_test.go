package iztro

import "testing"

// 插件机制测试。
//
// Go 不允许给其他包的类型加方法，因此扩展点是**嵌入**：
// 把 *Astrolabe 嵌进自己的结构体，新方法与内置方法一样用点号调用，
// 且是编译期检查的。
//
// 与 tests/extension.rs、python/tests/test_plugin.py 实现同一个插件，
// 断言同一组取值。

// MyChart 是插件作者定义的类型：嵌入星盘，补自己的分析方法。
type MyChart struct {
	*Astrolabe
}

// MajorStar 返回命宫主星名（空宫借对宫），多颗以逗号分隔。
func (c MyChart) MajorStar() string {
	soul := c.Palace(PalaceSoul)
	source := soul
	if soul.IsEmpty() {
		source = c.PalaceByIndex((soul.Index + 6) % 12)
	}
	names := make([]string, 0, len(source.MajorStars))
	for _, s := range source.MajorStars {
		if s.Type == StarTypeMajor {
			names = append(names, s.Name)
		}
	}
	out := ""
	for i, n := range names {
		if i > 0 {
			out += ","
		}
		out += n
	}
	return out
}

// FiveElementsValue 返回五行局的局数。
func (c MyChart) FiveElementsValue() int {
	return map[string]int{
		ClassWater2nd: 2,
		ClassWood3rd:  3,
		ClassMetal4th: 4,
		ClassEarth5th: 5,
		ClassFire6th:  6,
	}[c.FiveElementsClassKey]
}

func TestPluginViaEmbedding(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "zh-CN", nil)
	if err != nil {
		t.Fatal(err)
	}
	my := MyChart{chart}

	if got := my.MajorStar(); got != "紫微" {
		t.Fatalf("命宫主星 %q，期望 紫微", got)
	}
	if got := my.FiveElementsValue(); got != 3 {
		t.Fatalf("五行局局数 %d，期望 3", got)
	}

	// 内置方法照常可用
	if my.Palace(PalaceBody).NameKey != PalaceCareer {
		t.Fatal("嵌入后内置方法应照常可用")
	}
}

func TestPluginOutputFollowsChartLanguage(t *testing.T) {
	chart, err := BySolar("2000-8-16", 2, "female", true, "en-US", nil)
	if err != nil {
		t.Fatal(err)
	}
	my := MyChart{chart}

	if got := my.MajorStar(); got != "emperor" {
		t.Fatalf("英文命宫主星 %q，期望 emperor", got)
	}
	if got := my.FiveElementsValue(); got != 3 {
		t.Fatalf("五行局局数 %d，期望 3", got)
	}
}

package iztro

// 标识与译名的双向查找。
//
// 星盘对象的每个字段都同时给出译名与 *Key 标识，通常不必再手工翻译；
// 这两个函数用于手上只有标识（或只有某种语言的译名）、需要换算的场合。
//
// 覆盖星耀、宫位（含身宫、来因宫）、天干、地支、亮度、四化、五行局、性别、
// 生肖、时辰、星座、运限层级共十二类，合计 260 个标识。

// Translate 把标识译成指定语言的文本；未知标识返回空串。
//
// key 为语言无关标识（keys.go 常量），如 StarZiweiMaj、PalaceSoul、StemJia。
func Translate(key string, language Language) (string, error) {
	var out string
	return out, utilQuery(map[string]any{
		"kind":     "translate",
		"key":      key,
		"language": language,
	}, &out)
}

// KeyOf 由任意语言的译名反查标识；查不到返回空串。
//
// 逐语言、每种语言内逐标识比对取先命中者，顺序与 iztro 的 kot 一致。
// 同一译名在多个类目下同形时用 KeyOfIn 消歧。
func KeyOf(text string) (string, error) {
	var out string
	return out, utilQuery(map[string]any{
		"kind": "keyOf",
		"text": text,
	}, &out)
}

// KeyOfIn 限定标识名后反查标识；查不到返回空串。
//
// keyFilter 为标识名须含的子串，用于消歧同形译名（en-US 的 "horse" 既是生肖马
// 也是天马）："Maj" 只看十四主星、"Min" 只看辅星、"Heavenly" / "Earthly" 只看
// 干支、"Palace" 只看宫位、"Hour" 只看时辰。限定后无匹配返回空串，不退回
// 未限定的结果。
func KeyOfIn(text, keyFilter string) (string, error) {
	var out string
	return out, utilQuery(map[string]any{
		"kind":      "keyOf",
		"text":      text,
		"keyFilter": keyFilter,
	}, &out)
}

// AllKeys 返回全部 260 个可翻译标识。
//
// 顺序即 KeyOf 的反查次序，与 iztro 各语言翻译文件的合并次序一致：
// 运限层级、生肖、时辰、星座、五行局、天干、地支、亮度、四化、星耀、宫位、性别。
func AllKeys() ([]string, error) {
	var out []string
	return out, utilQuery(map[string]any{"kind": "allKeys"}, &out)
}

// rs-iztro Go 示例
//
// 紫微斗数 Rust 核心库的 Go FFI 绑定用法演示。
//
// 前置步骤：
//
//	cd rs-iztro          # 项目根目录
//	cargo build --release
//
// 运行方式：
//
//	cd examples/go
//	go run .
package main

import (
	"encoding/json"
	"fmt"
	"log"
	"strings"

	"rs-iztro/go/iztro"
)

func main() {
	// ============================================================
	// 1. 阳历排盘
	// ============================================================
	fmt.Println("===== 1. 阳历排盘 =====")
	fmt.Println()

	// BySolar(solarDate, timeIndex, gender, fixLeap, language, algorithm)
	//   solarDate:  "YYYY-M-D"
	//   timeIndex:  0=早子时, 1=丑, 2=寅, ..., 12=晚子时
	//   gender:     "male" | "female"
	//   fixLeap:    是否修正闰月
	//   language:   "zh_cn" | "zh_tw" | "en_us" | "ja_jp" | "ko_kr" | "vi_vn"
	//   algorithm:  "default" | "zhongzhou"
	astrolabe, err := iztro.BySolar("2000-8-16", 2, "female", true, "zh_cn", "default")
	if err != nil {
		log.Fatalf("BySolar failed: %v", err)
	}

	fmt.Printf("阳历：%v\n", astrolabe["solarDate"])
	fmt.Printf("农历：%v\n", astrolabe["lunarDate"])
	fmt.Printf("干支：%v\n", astrolabe["chineseDate"])
	fmt.Printf("时辰：%v (%v)\n", astrolabe["time"], astrolabe["timeRange"])
	fmt.Printf("星座：%v\n", astrolabe["sign"])
	fmt.Printf("生肖：%v\n", astrolabe["zodiac"])
	fmt.Printf("命主：%v\n", astrolabe["soul"])
	fmt.Printf("身主：%v\n", astrolabe["body"])
	fmt.Printf("五行局：%v\n", astrolabe["fiveElementsClass"])
	fmt.Println()

	// ============================================================
	// 2. 农历排盘
	// ============================================================
	fmt.Println("===== 2. 农历排盘 =====")
	fmt.Println()

	// ByLunar 多了一个 isLeapMonth 参数
	lunar, err := iztro.ByLunar("2000-7-17", 2, "female", false, true, "zh_cn", "default")
	if err != nil {
		log.Fatalf("ByLunar failed: %v", err)
	}
	fmt.Printf("农历排盘 - 阳历：%v\n", lunar["solarDate"])
	fmt.Printf("农历排盘 - 农历：%v\n", lunar["lunarDate"])
	fmt.Println()

	// ============================================================
	// 3. 十二宫概览
	// ============================================================
	fmt.Println("===== 3. 十二宫概览 =====")
	fmt.Println()

	palaces := astrolabe["palaces"].([]any)
	for _, p := range palaces {
		palace := p.(map[string]any)
		index := palace["index"].(float64)
		name := palace["name"].(string)
		hs := palace["heavenlyStem"].(string)
		eb := palace["earthlyBranch"].(string)
		isBody := palace["isBodyPalace"].(bool)

		majorStars := palace["majorStars"].([]any)
		var names []string
		for _, s := range majorStars {
			names = append(names, s.(map[string]any)["name"].(string))
		}

		bodyMarker := ""
		if isBody {
			bodyMarker = " [身]"
		}
		starsStr := "（空宫）"
		if len(names) > 0 {
			starsStr = strings.Join(names, "、")
		}
		fmt.Printf("  [%2.0f] %s%s (%s%s) - 主星：%s\n",
			index, name, bodyMarker, hs, eb, starsStr)
	}
	fmt.Println()

	// ============================================================
	// 4. 命宫星耀详情
	// ============================================================
	fmt.Println("===== 4. 命宫星耀详情 =====")
	fmt.Println()

	var soulPalace map[string]any
	for _, p := range palaces {
		palace := p.(map[string]any)
		if palace["name"].(string) == "命宫" {
			soulPalace = palace
			break
		}
	}

	if soulPalace != nil {
		fmt.Printf("命宫：%s%s\n", soulPalace["heavenlyStem"], soulPalace["earthlyBranch"])
		fmt.Printf("是否身宫：%v\n", soulPalace["isBodyPalace"])

		decadal := soulPalace["decadal"].(map[string]any)
		fmt.Printf("大限范围：%v\n", decadal["range"])

		fmt.Println("\n主星：")
		for _, s := range soulPalace["majorStars"].([]any) {
			star := s.(map[string]any)
			info := fmt.Sprintf("  %s", star["name"])
			if b, ok := star["brightness"].(string); ok && b != "" {
				info += fmt.Sprintf(" (%s)", b)
			}
			if m, ok := star["mutagen"].(string); ok && m != "" {
				info += fmt.Sprintf(" [%s]", m)
			}
			fmt.Println(info)
		}

		fmt.Println("\n辅星：")
		for _, s := range soulPalace["minorStars"].([]any) {
			star := s.(map[string]any)
			info := fmt.Sprintf("  %s", star["name"])
			if b, ok := star["brightness"].(string); ok && b != "" {
				info += fmt.Sprintf(" (%s)", b)
			}
			if m, ok := star["mutagen"].(string); ok && m != "" {
				info += fmt.Sprintf(" [%s]", m)
			}
			fmt.Println(info)
		}

		fmt.Println("\n杂曜：")
		for _, s := range soulPalace["adjectiveStars"].([]any) {
			star := s.(map[string]any)
			fmt.Printf("  %s\n", star["name"])
		}
	}
	fmt.Println()

	// ============================================================
	// 5. 运限（大限、流年、流月等）
	// ============================================================
	fmt.Println("===== 5. 运限 =====")
	fmt.Println()

	// GetHoroscope 需要传入 astrolabe 的 JSON 字符串
	astrolabeJSON, _ := json.Marshal(astrolabe)

	horoscope, err := iztro.GetHoroscope(string(astrolabeJSON), "2024-10-1", 0, "zh_cn")
	if err != nil {
		log.Fatalf("GetHoroscope failed: %v", err)
	}

	fmt.Printf("目标日期：%v / %v\n", horoscope["solarDate"], horoscope["lunarDate"])

	// 大限
	hDecadal := horoscope["decadal"].(map[string]any)
	fmt.Printf("\n大限：%v (%v%v)\n", hDecadal["name"], hDecadal["heavenlyStem"], hDecadal["earthlyBranch"])
	fmt.Printf("  宫位：%s\n", joinAny(hDecadal["palaceNames"].([]any), "、"))
	fmt.Printf("  四化：%s\n", joinAny(hDecadal["mutagen"].([]any), "、"))

	// 小限
	age := horoscope["age"].(map[string]any)
	fmt.Printf("\n小限：%v (虚岁 %v)\n", age["name"], age["nominalAge"])

	// 流年
	yearly := horoscope["yearly"].(map[string]any)
	fmt.Printf("\n流年：%v (%v%v)\n", yearly["name"], yearly["heavenlyStem"], yearly["earthlyBranch"])
	fmt.Printf("  四化：%s\n", joinAny(yearly["mutagen"].([]any), "、"))

	// 流月、流日、流时
	fmt.Printf("\n流月：%v\n", horoscope["monthly"].(map[string]any)["name"])
	fmt.Printf("流日：%v\n", horoscope["daily"].(map[string]any)["name"])
	fmt.Printf("流时：%v\n", horoscope["hourly"].(map[string]any)["name"])
	fmt.Println()

	// ============================================================
	// 6. JSON 输出预览
	// ============================================================
	fmt.Println("===== 6. JSON 输出预览 =====")
	fmt.Println()

	prettyJSON, _ := json.MarshalIndent(astrolabe, "", "  ")
	preview := string(prettyJSON)
	if len(preview) > 500 {
		preview = preview[:500] + "..."
	}
	fmt.Println(preview)
	fmt.Printf("\n（JSON 总长度：%d 字符）\n", len(astrolabeJSON))

	fmt.Println("\n===== 示例完毕 =====")
}

func joinAny(items []any, sep string) string {
	parts := make([]string, len(items))
	for i, item := range items {
		parts[i] = fmt.Sprintf("%v", item)
	}
	return strings.Join(parts, sep)
}

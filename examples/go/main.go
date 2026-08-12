// rs-iztro Go 示例
//
// 通过内嵌 wasm + wazero（纯 Go，无 cgo）调用紫微斗数核心库。
//
// 运行方式：
//
//	cd examples/go
//	go run .
package main

import (
	"fmt"
	"log"

	"rs-iztro/go/iztro"
)

func main() {
	// ============================================================
	// 1. 阳历排盘
	// ============================================================
	fmt.Println("===== 1. 阳历排盘 =====")
	fmt.Println()

	astrolabe, err := iztro.BySolar("2000-8-16", 2, "female", true, "zh_cn", nil)
	if err != nil {
		log.Fatalf("BySolar failed: %v", err)
	}

	fmt.Printf("阳历：%v\n", astrolabe["solarDate"])
	fmt.Printf("农历：%v\n", astrolabe["lunarDate"])
	fmt.Printf("干支：%v\n", astrolabe["chineseDate"])
	fmt.Printf("时辰：%v (%v)\n", astrolabe["time"], astrolabe["timeRange"])
	fmt.Printf("星座：%v    生肖：%v\n", astrolabe["sign"], astrolabe["zodiac"])
	fmt.Printf("命主：%v    身主：%v\n", astrolabe["soul"], astrolabe["body"])
	fmt.Printf("五行局：%v\n", astrolabe["fiveElementsClass"])
	fmt.Println()

	// ============================================================
	// 2. 十二宫概览
	// ============================================================
	fmt.Println("===== 2. 十二宫概览 =====")
	fmt.Println()

	palaces := astrolabe["palaces"].([]any)
	for _, p := range palaces {
		palace := p.(map[string]any)
		var stars []string
		for _, s := range palace["majorStars"].([]any) {
			star := s.(map[string]any)
			label := star["name"].(string)
			if b, _ := star["brightness"].(string); b != "" {
				label += "(" + b + ")"
			}
			if m, _ := star["mutagen"].(string); m != "" {
				label += "[" + m + "]"
			}
			stars = append(stars, label)
		}
		display := "空宫"
		if len(stars) > 0 {
			display = fmt.Sprint(stars)
		}
		body := ""
		if palace["isBodyPalace"].(bool) {
			body = " [身]"
		}
		fmt.Printf("  %v%s (%v%v) %s\n",
			palace["name"], body, palace["heavenlyStem"], palace["earthlyBranch"], display)
	}
	fmt.Println()

	// ============================================================
	// 3. 农历排盘
	// ============================================================
	fmt.Println("===== 3. 农历排盘 =====")
	fmt.Println()

	lunar, err := iztro.ByLunar("2000-7-17", 2, "female", false, true, "zh_cn", nil)
	if err != nil {
		log.Fatalf("ByLunar failed: %v", err)
	}
	fmt.Printf("农历 2000-7-17 → 阳历 %v\n", lunar["solarDate"])
	fmt.Println()

	// ============================================================
	// 4. 运限（无状态）
	// ============================================================
	fmt.Println("===== 4. 运限 (2024-10-1) =====")
	fmt.Println()

	horoscope, err := iztro.Horoscope("2000-8-16", 2, "female", true, "zh_cn", nil, "2024-10-1", 0)
	if err != nil {
		log.Fatalf("Horoscope failed: %v", err)
	}
	yearly := horoscope["yearly"].(map[string]any)
	fmt.Printf("目标日期：%v / %v\n", horoscope["solarDate"], horoscope["lunarDate"])
	fmt.Printf("%v：%v%v\n", yearly["name"], yearly["heavenlyStem"], yearly["earthlyBranch"])
	fmt.Printf("流年四化：%v\n", yearly["mutagen"])
	age := horoscope["age"].(map[string]any)
	fmt.Printf("%v：虚岁 %v\n", age["name"], age["nominalAge"])
	fmt.Println()

	// ============================================================
	// 5. 中州派配置
	// ============================================================
	fmt.Println("===== 5. 中州派 =====")
	fmt.Println()

	zz, err := iztro.BySolar("1990-11-5", 4, "male", true, "zh_cn", &iztro.Config{Algorithm: "zhongzhou"})
	if err != nil {
		log.Fatalf("BySolar zhongzhou failed: %v", err)
	}
	fmt.Printf("中州派命主：%v\n", zz["soul"])

	fmt.Println()
	fmt.Println("===== 完毕 =====")
}

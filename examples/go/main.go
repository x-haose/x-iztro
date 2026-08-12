// rs-iztro Go 示例
//
// 通过内嵌 wasm + wazero（纯 Go，无 cgo）调用紫微斗数核心库，
// 返回类型化结构体并以语言无关的 key 常量做判断。
//
// 运行方式：
//
//	cd examples/go
//	go run .
package main

import (
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

	astrolabe, err := iztro.BySolar("2000-8-16", 2, "female", true, "zh_cn", nil)
	if err != nil {
		log.Fatalf("BySolar failed: %v", err)
	}

	fmt.Printf("阳历：%s\n", astrolabe.SolarDate)
	fmt.Printf("农历：%s\n", astrolabe.LunarDate)
	fmt.Printf("干支：%s\n", astrolabe.ChineseDate)
	fmt.Printf("时辰：%s (%s)\n", astrolabe.Time, astrolabe.TimeRange)
	fmt.Printf("星座：%s    生肖：%s\n", astrolabe.Sign, astrolabe.Zodiac)
	fmt.Printf("命主：%s    身主：%s\n", astrolabe.Soul, astrolabe.Body)
	fmt.Printf("五行局：%s\n", astrolabe.FiveElementsClass)
	fmt.Println()

	// ============================================================
	// 2. 十二宫概览
	// ============================================================
	fmt.Println("===== 2. 十二宫概览 =====")
	fmt.Println()

	for i := range astrolabe.Palaces {
		p := &astrolabe.Palaces[i]
		var stars []string
		for _, s := range p.MajorStars {
			label := s.Name
			if s.Brightness != "" {
				label += "(" + s.Brightness + ")"
			}
			if s.Mutagen != "" {
				label += "[" + s.Mutagen + "]"
			}
			stars = append(stars, label)
		}
		display := "空宫"
		if len(stars) > 0 {
			display = strings.Join(stars, " ")
		}
		body := ""
		if p.IsBodyPalace {
			body = " [身]"
		}
		fmt.Printf("  %s%s (%s%s) %s\n", p.Name, body, p.HeavenlyStem, p.EarthlyBranch, display)
	}
	fmt.Println()

	// ============================================================
	// 3. 类型化查询（key 常量在任何输出语言下都有效）
	// ============================================================
	fmt.Println("===== 3. 类型化查询 =====")
	fmt.Println()

	soul := astrolabe.Palace(iztro.PalaceSoul)
	fmt.Printf("命宫有紫微：%v\n", soul.Has(iztro.StarZiweiMaj))
	fmt.Printf("命宫化禄：%v    化忌：%v\n", soul.HasMutagen(iztro.MutagenLu), soul.HasMutagen(iztro.MutagenJi))

	star, palace := astrolabe.Star(iztro.StarWuquMaj)
	fmt.Printf("武曲落宫：%s，化权：%v\n", palace.Name, star.WithMutagen(iztro.MutagenQuan))

	sp := astrolabe.SurroundedPalaces(soul.Index)
	fmt.Printf("命宫三方四正有天府：%v\n", sp.Have(iztro.StarTianfuMaj))
	fmt.Println()

	// ============================================================
	// 4. 运限（无状态）
	// ============================================================
	fmt.Println("===== 4. 运限 (2024-10-1) =====")
	fmt.Println()

	h, err := iztro.GetHoroscope("2000-8-16", 2, "female", true, "zh_cn", nil, "2024-10-1", 0)
	if err != nil {
		log.Fatalf("GetHoroscope failed: %v", err)
	}
	fmt.Printf("目标日期：%s / %s\n", h.SolarDate, h.LunarDate)
	fmt.Printf("%s：%s%s，四化：%v\n", h.Yearly.Name, h.Yearly.HeavenlyStem, h.Yearly.EarthlyBranch, h.Yearly.Mutagen)
	fmt.Printf("%s：虚岁 %d\n", h.Age.Name, h.Age.NominalAge)
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
	fmt.Printf("中州派命主：%s\n", zz.Soul)

	fmt.Println()
	fmt.Println("===== 完毕 =====")
}

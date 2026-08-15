package iztro

// 天干四化表：宫干或运限干决定哪四颗星分别化禄、权、科、忌。
// 键为天干标识（keys.go 的 Stem* 常量），值按 [禄, 权, 科, 忌] 排列。
// 与 Rust 核心 `data::heavenly_stems` 的表一一对应。
var mutagenTable = map[string][4]string{
	StemJia:  {StarLianzhenMaj, StarPojunMaj, StarWuquMaj, StarTaiyangMaj},
	StemYi:   {StarTianjiMaj, StarTianliangMaj, StarZiweiMaj, StarTaiyinMaj},
	StemBing: {StarTiantongMaj, StarTianjiMaj, StarWenchangMin, StarLianzhenMaj},
	StemDing: {StarTaiyinMaj, StarTiantongMaj, StarTianjiMaj, StarJumenMaj},
	StemWu:   {StarTanlangMaj, StarTaiyinMaj, StarYoubiMin, StarTianjiMaj},
	StemJi:   {StarWuquMaj, StarTanlangMaj, StarTianliangMaj, StarWenquMin},
	StemGeng: {StarTaiyangMaj, StarWuquMaj, StarTaiyinMaj, StarTiantongMaj},
	StemXin:  {StarJumenMaj, StarTaiyangMaj, StarWenquMin, StarWenchangMin},
	StemRen:  {StarTianliangMaj, StarZiweiMaj, StarZuofuMin, StarWuquMaj},
	StemGui:  {StarPojunMaj, StarJumenMaj, StarTaiyinMaj, StarTanlangMaj},
}

// mutagenIndex 把四化标识映射到四化表中的位置。
var mutagenIndex = map[string]int{
	MutagenLu:   0,
	MutagenQuan: 1,
	MutagenKe:   2,
	MutagenJi:   3,
}

// allMutagens 为全部四化，顺序为禄、权、科、忌。
var allMutagens = [4]string{MutagenLu, MutagenQuan, MutagenKe, MutagenJi}

// orAllMutagens 在未指定四化时回退为全部四化。
func orAllMutagens(mutagenKeys []string) []string {
	if len(mutagenKeys) == 0 {
		return allMutagens[:]
	}
	return mutagenKeys
}

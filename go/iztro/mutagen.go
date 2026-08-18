package iztro

// 四化的顺序与标识。
//
// 天干化出哪四颗星由排盘时的配置决定（Config.Mutagens 可整表替换某个天干），
// 因此结果由 wasm 侧随盘给出，落在 Palace.MutagenStarKeys 上，本地不留副本。

// mutagenIndex 把四化标识映射到 Palace.MutagenStarKeys 中的位置。
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

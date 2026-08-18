package iztro

// 按出生数据安星。
//
// 不排整盘、只取某一组星耀时用这些函数。返回的星耀标识与索引都与语言无关，
// 可直接同星盘对象的 *Key 字段比对。
//
// 索引一律为宫位索引：0 为寅宫，11 为丑宫。

// StarBirth 为安星的出生数据。
//
// 对应 iztro 的 AstrolabeParam，另收 iztro 挂在全局单例上的语言与配置。
type StarBirth struct {
	// SolarDate 为公历日期，格式 "YYYY-M-D"
	SolarDate string
	// TimeIndex 为时辰索引 0-12（0 为早子时，12 为晚子时）
	TimeIndex uint8
	// Gender 为性别，"male" 或 "female"
	Gender string
	// FixLeap 表示是否调整农历闰月（该月非闰月则不生效）
	FixLeap bool
	// Language 为星耀名称的输出语言，留空取 zh-CN
	Language string
	// Config 为排盘配置，nil 取默认
	Config *Config
	// FromStem 为起五行局的天干；与 FromBranch 同时给出时改用该干支起局（中州派地盘、人盘）
	FromStem string
	// FromBranch 为起五行局的地支；与 FromStem 同时给出时才生效
	FromBranch string
}

// payload 把出生数据落成 wasm 查询入参。
func (b StarBirth) payload(kind string) map[string]any {
	p := map[string]any{
		"kind":      kind,
		"solarDate": b.SolarDate,
		"timeIndex": b.TimeIndex,
		"gender":    b.Gender,
		"fixLeap":   b.FixLeap,
		"language":  b.Language,
		"config":    b.Config,
	}
	if b.FromStem != "" && b.FromBranch != "" {
		p["fromStem"] = b.FromStem
		p["fromBranch"] = b.FromBranch
	}
	return p
}

// StartIndex 为紫微、天府的起始宫位索引。
type StartIndex struct {
	// ZiweiIndex 为紫微所在宫位索引
	ZiweiIndex int `json:"ziweiIndex"`
	// TianfuIndex 为天府所在宫位索引
	TianfuIndex int `json:"tianfuIndex"`
}

// LuYangTuoMaIndex 为禄存、擎羊、陀罗、天马的宫位索引。
type LuYangTuoMaIndex struct {
	// LuIndex 为禄存所在宫位索引
	LuIndex int `json:"luIndex"`
	// YangIndex 为擎羊所在宫位索引
	YangIndex int `json:"yangIndex"`
	// TuoIndex 为陀罗所在宫位索引
	TuoIndex int `json:"tuoIndex"`
	// MaIndex 为天马所在宫位索引
	MaIndex int `json:"maIndex"`
}

// KuiYueIndex 为天魁、天钺的宫位索引。
type KuiYueIndex struct {
	// KuiIndex 为天魁所在宫位索引
	KuiIndex int `json:"kuiIndex"`
	// YueIndex 为天钺所在宫位索引
	YueIndex int `json:"yueIndex"`
}

// ChangQuIndex 为文昌、文曲的宫位索引。
type ChangQuIndex struct {
	// ChangIndex 为文昌所在宫位索引
	ChangIndex int `json:"changIndex"`
	// QuIndex 为文曲所在宫位索引
	QuIndex int `json:"quIndex"`
}

// KongJieIndex 为地空、地劫的宫位索引。
type KongJieIndex struct {
	// KongIndex 为地空所在宫位索引
	KongIndex int `json:"kongIndex"`
	// JieIndex 为地劫所在宫位索引
	JieIndex int `json:"jieIndex"`
}

// TimelyStarIndex 为台辅、封诰的宫位索引。
type TimelyStarIndex struct {
	// TaifuIndex 为台辅所在宫位索引
	TaifuIndex int `json:"taifuIndex"`
	// FenggaoIndex 为封诰所在宫位索引
	FenggaoIndex int `json:"fenggaoIndex"`
}

// LuanXiIndex 为红鸾、天喜的宫位索引。
type LuanXiIndex struct {
	// HongluanIndex 为红鸾所在宫位索引
	HongluanIndex int `json:"hongluanIndex"`
	// TianxiIndex 为天喜所在宫位索引
	TianxiIndex int `json:"tianxiIndex"`
}

// DailyStarIndex 为日系星的宫位索引。
type DailyStarIndex struct {
	// SantaiIndex 为三台所在宫位索引
	SantaiIndex int `json:"santaiIndex"`
	// BazuoIndex 为八座所在宫位索引
	BazuoIndex int `json:"bazuoIndex"`
	// EnguangIndex 为恩光所在宫位索引
	EnguangIndex int `json:"enguangIndex"`
	// TianguiIndex 为天贵所在宫位索引
	TianguiIndex int `json:"tianguiIndex"`
}

// MonthlyStarIndex 为月系星的宫位索引。
type MonthlyStarIndex struct {
	// YuejieIndex 为解神所在宫位索引
	YuejieIndex int `json:"yuejieIndex"`
	// TianyaoIndex 为天姚所在宫位索引
	TianyaoIndex int `json:"tianyaoIndex"`
	// TianxingIndex 为天刑所在宫位索引
	TianxingIndex int `json:"tianxingIndex"`
	// YinshaIndex 为阴煞所在宫位索引
	YinshaIndex int `json:"yinshaIndex"`
	// TianyueIndex 为天月所在宫位索引
	TianyueIndex int `json:"tianyueIndex"`
	// TianwuIndex 为天巫所在宫位索引
	TianwuIndex int `json:"tianwuIndex"`
}

// YearlyStarIndex 为年系杂耀的宫位索引。
type YearlyStarIndex struct {
	// XianchiIndex 为咸池所在宫位索引
	XianchiIndex int `json:"xianchiIndex"`
	// HuagaiIndex 为华盖所在宫位索引
	HuagaiIndex int `json:"huagaiIndex"`
	// GuchenIndex 为孤辰所在宫位索引
	GuchenIndex int `json:"guchenIndex"`
	// GuasuIndex 为寡宿所在宫位索引
	GuasuIndex int `json:"guasuIndex"`
	// TiancaiIndex 为天才所在宫位索引
	TiancaiIndex int `json:"tiancaiIndex"`
	// TianshouIndex 为天寿所在宫位索引
	TianshouIndex int `json:"tianshouIndex"`
	// TianchuIndex 为天厨所在宫位索引
	TianchuIndex int `json:"tianchuIndex"`
	// PosuiIndex 为破碎所在宫位索引
	PosuiIndex int `json:"posuiIndex"`
	// FeilianIndex 为飞廉所在宫位索引
	FeilianIndex int `json:"feilianIndex"`
	// LongchiIndex 为龙池所在宫位索引
	LongchiIndex int `json:"longchiIndex"`
	// FenggeIndex 为凤阁所在宫位索引
	FenggeIndex int `json:"fenggeIndex"`
	// TiankuIndex 为天哭所在宫位索引
	TiankuIndex int `json:"tiankuIndex"`
	// TianxuIndex 为天虚所在宫位索引
	TianxuIndex int `json:"tianxuIndex"`
	// TianguanIndex 为天官所在宫位索引
	TianguanIndex int `json:"tianguanIndex"`
	// TianfuIndex 为天福所在宫位索引
	TianfuIndex int `json:"tianfuIndex"`
	// TiandeIndex 为天德所在宫位索引
	TiandeIndex int `json:"tiandeIndex"`
	// YuedeIndex 为月德所在宫位索引
	YuedeIndex int `json:"yuedeIndex"`
	// TiankongIndex 为天空所在宫位索引
	TiankongIndex int `json:"tiankongIndex"`
	// JieluIndex 为截路所在宫位索引
	JieluIndex int `json:"jieluIndex"`
	// KongwangIndex 为空亡所在宫位索引
	KongwangIndex int `json:"kongwangIndex"`
	// XunkongIndex 为旬空所在宫位索引
	XunkongIndex int `json:"xunkongIndex"`
	// TianshangIndex 为天伤所在宫位索引
	TianshangIndex int `json:"tianshangIndex"`
	// TianshiIndex 为天使所在宫位索引
	TianshiIndex int `json:"tianshiIndex"`
	// JiekongIndex 为截空所在宫位索引
	JiekongIndex int `json:"jiekongIndex"`
	// JieshaAdjIndex 为劫煞所在宫位索引
	JieshaAdjIndex int `json:"jieshaAdjIndex"`
	// NianjieIndex 为年解所在宫位索引
	NianjieIndex int `json:"nianjieIndex"`
	// DahaoAdjIndex 为大耗所在宫位索引
	DahaoAdjIndex int `json:"dahaoAdjIndex"`
}

// Yearly12 为岁前12神与将前12神，从寅宫起按宫位索引排列。
type Yearly12 struct {
	// Suiqian12 为岁前12神标识
	Suiqian12 []string `json:"suiqian12"`
	// Jiangqian12 为将前12神标识
	Jiangqian12 []string `json:"jiangqian12"`
}

// GetStartIndex 取紫微、天府的起始宫位索引。
func GetStartIndex(birth StarBirth) (StartIndex, error) {
	var out StartIndex
	return out, utilQuery(birth.payload("getStartIndex"), &out)
}

// GetLuYangTuoMaIndex 取禄存、擎羊、陀罗、天马的宫位索引（按年干支）。
func GetLuYangTuoMaIndex(birth StarBirth) (LuYangTuoMaIndex, error) {
	var out LuYangTuoMaIndex
	return out, utilQuery(birth.payload("getLuYangTuoMaIndex"), &out)
}

// GetKuiYueIndex 取天魁、天钺的宫位索引（按年干）。
func GetKuiYueIndex(birth StarBirth) (KuiYueIndex, error) {
	var out KuiYueIndex
	return out, utilQuery(birth.payload("getKuiYueIndex"), &out)
}

// GetChangQuIndex 取文昌、文曲的宫位索引（按时支）。
func GetChangQuIndex(birth StarBirth) (ChangQuIndex, error) {
	var out ChangQuIndex
	return out, utilQuery(birth.payload("getChangQuIndex"), &out)
}

// GetKongJieIndex 取地空、地劫的宫位索引（按时支）。
func GetKongJieIndex(birth StarBirth) (KongJieIndex, error) {
	var out KongJieIndex
	return out, utilQuery(birth.payload("getKongJieIndex"), &out)
}

// GetTimelyStarIndex 取台辅、封诰的宫位索引（按时支）。
func GetTimelyStarIndex(birth StarBirth) (TimelyStarIndex, error) {
	var out TimelyStarIndex
	return out, utilQuery(birth.payload("getTimelyStarIndex"), &out)
}

// GetLuanXiIndex 取红鸾、天喜的宫位索引（按年支）。
func GetLuanXiIndex(birth StarBirth) (LuanXiIndex, error) {
	var out LuanXiIndex
	return out, utilQuery(birth.payload("getLuanXiIndex"), &out)
}

// GetDailyStarIndex 取日系星索引：三台、八座、恩光、天贵。
func GetDailyStarIndex(birth StarBirth) (DailyStarIndex, error) {
	var out DailyStarIndex
	return out, utilQuery(birth.payload("getDailyStarIndex"), &out)
}

// GetMonthlyStarIndex 取月系星索引：解神、天姚、天刑、阴煞、天月、天巫。
func GetMonthlyStarIndex(birth StarBirth) (MonthlyStarIndex, error) {
	var out MonthlyStarIndex
	return out, utilQuery(birth.payload("getMonthlyStarIndex"), &out)
}

// GetYearlyStarIndex 取年系杂耀的宫位索引；年支按 Config.HoroscopeDivide 分界。
func GetYearlyStarIndex(birth StarBirth) (YearlyStarIndex, error) {
	var out YearlyStarIndex
	return out, utilQuery(birth.payload("getYearlyStarIndex"), &out)
}

// GetMajorStar 取十四主星在十二宫的分布，按宫位索引排列。
func GetMajorStar(birth StarBirth) ([][]Star, error) {
	var out [][]Star
	return out, utilQuery(birth.payload("getMajorStar"), &out)
}

// GetMinorStar 取十四辅星在十二宫的分布，按宫位索引排列。
func GetMinorStar(birth StarBirth) ([][]Star, error) {
	var out [][]Star
	return out, utilQuery(birth.payload("getMinorStar"), &out)
}

// GetAdjectiveStar 取杂耀在十二宫的分布，按宫位索引排列。
func GetAdjectiveStar(birth StarBirth) ([][]Star, error) {
	var out [][]Star
	return out, utilQuery(birth.payload("getAdjectiveStar"), &out)
}

// GetChangsheng12 取长生12神标识，从寅宫起按宫位索引排列。
func GetChangsheng12(birth StarBirth) ([]string, error) {
	var out []string
	return out, utilQuery(birth.payload("getChangsheng12"), &out)
}

// GetBoShi12 取博士12神标识，从寅宫起按宫位索引排列。
func GetBoShi12(birth StarBirth) ([]string, error) {
	var out []string
	return out, utilQuery(birth.payload("getBoShi12"), &out)
}

// GetYearly12 取岁前12神与将前12神标识，从寅宫起按宫位索引排列。
// 流年神煞按 Config.HoroscopeDivide 分界取年支。
func GetYearly12(birth StarBirth) (Yearly12, error) {
	var out Yearly12
	return out, utilQuery(birth.payload("getYearly12"), &out)
}

// GetHoroscopeStar 取流耀在十二宫的分布：魁钺昌曲禄羊陀马鸾喜，
// 流年层级另加年解。scope 决定星名，如大限为运魁、流年为流魁。
func GetHoroscopeStar(stemKey, branchKey, scope, language string) ([][]Star, error) {
	var out [][]Star
	return out, utilQuery(map[string]any{
		"kind":      "getHoroscopeStar",
		"stemKey":   stemKey,
		"branchKey": branchKey,
		"scope":     scope,
		"language":  language,
	}, &out)
}

// GetChangsheng12StartIndex 取长生12神的起始宫位索引：
// 水二局在申、木三局在亥、金四局在巳、土五局在申、火六局在寅。
func GetChangsheng12StartIndex(fiveElementsClass string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":              "getChangesheng12StartIndex",
		"fiveElementsClass": fiveElementsClass,
	}, &out)
}

// GetJiangqian12StartIndex 取将前12神的起始宫位索引：
// 寅午戌年在午、申子辰年在子、巳酉丑年在酉、亥卯未年在卯。
func GetJiangqian12StartIndex(branchKey string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "getJiangqian12StartIndex",
		"branchKey": branchKey,
	}, &out)
}

// ZuoYouIndex 为左辅、右弼的宫位索引。
type ZuoYouIndex struct {
	// ZuoIndex 为左辅所在宫位索引
	ZuoIndex int `json:"zuoIndex"`
	// YouIndex 为右弼所在宫位索引
	YouIndex int `json:"youIndex"`
}

// HuoLingIndex 为火星、铃星的宫位索引。
type HuoLingIndex struct {
	// HuoIndex 为火星所在宫位索引
	HuoIndex int `json:"huoIndex"`
	// LingIndex 为铃星所在宫位索引
	LingIndex int `json:"lingIndex"`
}

// HuagaiXianchiIndex 为华盖、咸池的宫位索引。
type HuagaiXianchiIndex struct {
	// HuagaiIndex 为华盖所在宫位索引
	HuagaiIndex int `json:"huagaiIndex"`
	// XianchiIndex 为咸池所在宫位索引
	XianchiIndex int `json:"xianchiIndex"`
}

// GuGuaIndex 为孤辰、寡宿的宫位索引。
type GuGuaIndex struct {
	// GuchenIndex 为孤辰所在宫位索引
	GuchenIndex int `json:"guchenIndex"`
	// GuasuIndex 为寡宿所在宫位索引
	GuasuIndex int `json:"guasuIndex"`
}

// TianshiTianshangIndex 为天伤、天使的宫位索引。
type TianshiTianshangIndex struct {
	// TianshangIndex 为天伤所在宫位索引
	TianshangIndex int `json:"tianshangIndex"`
	// TianshiIndex 为天使所在宫位索引
	TianshiIndex int `json:"tianshiIndex"`
}

// GetZuoYouIndex 取左辅、右弼的宫位索引（按农历月份）。
//
// lunarMonth 为经闰月修正后的农历月份 1-12，即 FixLunarMonthIndex 结果加一。
func GetZuoYouIndex(lunarMonth int) (ZuoYouIndex, error) {
	var out ZuoYouIndex
	return out, utilQuery(map[string]any{
		"kind":       "getZuoYouIndex",
		"lunarMonth": lunarMonth,
	}, &out)
}

// GetHuoLingIndex 取火星、铃星的宫位索引（按年支与时辰）。
func GetHuoLingIndex(branchKey string, timeIndex uint8) (HuoLingIndex, error) {
	var out HuoLingIndex
	return out, utilQuery(map[string]any{
		"kind":      "getHuoLingIndex",
		"branchKey": branchKey,
		"timeIndex": timeIndex,
	}, &out)
}

// GetHuagaiXianchiIndex 取华盖、咸池的宫位索引（按年支）。
func GetHuagaiXianchiIndex(branchKey string) (HuagaiXianchiIndex, error) {
	var out HuagaiXianchiIndex
	return out, utilQuery(map[string]any{
		"kind":      "getHuagaiXianchiIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetGuGuaIndex 取孤辰、寡宿的宫位索引（按年支）。
func GetGuGuaIndex(branchKey string) (GuGuaIndex, error) {
	var out GuGuaIndex
	return out, utilQuery(map[string]any{
		"kind":      "getGuGuaIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetJieshaAdjIndex 取劫煞（杂耀）的宫位索引（按年支）。取值只有 0、3、6、9 四种。
func GetJieshaAdjIndex(branchKey string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "getJieshaAdjIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetDahaoIndex 取大耗（杂耀）的宫位索引（按年支）。
func GetDahaoIndex(branchKey string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "getDahaoIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetNianjieIndex 取年解的宫位索引（按年支）。
func GetNianjieIndex(branchKey string) (int, error) {
	var out int
	return out, utilQuery(map[string]any{
		"kind":      "getNianjieIndex",
		"branchKey": branchKey,
	}, &out)
}

// GetTianshiTianshangIndex 取天伤、天使的宫位索引（按性别、年支与命宫位置）。
//
// 二者夹迁移宫：通行派天伤居仆役位、天使居疾厄位；中州派在阴男阳女
// （生年地支阴阳与性别阴阳不同）时二者对调，由 config.Algorithm 决定走哪一派。
// config 为 nil 时取默认配置。
func GetTianshiTianshangIndex(gender, branchKey string, soulIndex int, config *Config) (TianshiTianshangIndex, error) {
	var out TianshiTianshangIndex
	return out, utilQuery(map[string]any{
		"kind":      "getTianshiTianshangIndex",
		"gender":    gender,
		"branchKey": branchKey,
		"soulIndex": soulIndex,
		"config":    config,
	}, &out)
}

// GetChangQuIndexByHeavenlyStem 取文昌、文曲的宫位索引（按天干）。
//
// 运限层级的流昌流曲用这一支，本命盘的昌曲按时支走 GetChangQuIndex。
func GetChangQuIndexByHeavenlyStem(stemKey string) (ChangQuIndex, error) {
	var out ChangQuIndex
	return out, utilQuery(map[string]any{
		"kind":    "getChangQuIndexByHeavenlyStem",
		"stemKey": stemKey,
	}, &out)
}

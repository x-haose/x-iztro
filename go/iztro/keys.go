package iztro

// 本文件为语言无关标识常量（iztro i18n key），与数据对象上的
// Key/NameKey 等标识字段直接比较，在任何输出语言的星盘上都能正确判断。

// 十二宫标识（Palace.NameKey）。
const (
	// PalaceSoul 为命宫
	PalaceSoul = "soulPalace"
	// PalaceSiblings 为兄弟
	PalaceSiblings = "siblingsPalace"
	// PalaceSpouse 为夫妻
	PalaceSpouse = "spousePalace"
	// PalaceChildren 为子女
	PalaceChildren = "childrenPalace"
	// PalaceWealth 为财帛
	PalaceWealth = "wealthPalace"
	// PalaceHealth 为疾厄
	PalaceHealth = "healthPalace"
	// PalaceSurface 为迁移
	PalaceSurface = "surfacePalace"
	// PalaceFriends 为仆役（又名交友）
	PalaceFriends = "friendsPalace"
	// PalaceCareer 为官禄
	PalaceCareer = "careerPalace"
	// PalaceProperty 为田宅
	PalaceProperty = "propertyPalace"
	// PalaceSpirit 为福德
	PalaceSpirit = "spiritPalace"
	// PalaceParents 为父母
	PalaceParents = "parentsPalace"
)

// 四化标识（Star.MutagenKey 与 HoroscopeScope.MutagenKeys）。
const (
	// MutagenLu 为化禄
	MutagenLu = "sihuaLu"
	// MutagenQuan 为化权
	MutagenQuan = "sihuaQuan"
	// MutagenKe 为化科
	MutagenKe = "sihuaKe"
	// MutagenJi 为化忌
	MutagenJi = "sihuaJi"
)

// 亮度标识（Star.BrightnessKey）。
const (
	// BrightnessMiao 为庙
	BrightnessMiao = "miao"
	// BrightnessWang 为旺
	BrightnessWang = "wang"
	// BrightnessDe 为得
	BrightnessDe = "de"
	// BrightnessLi 为利
	BrightnessLi = "li"
	// BrightnessPing 为平
	BrightnessPing = "ping"
	// BrightnessBu 为不
	BrightnessBu = "bu"
	// BrightnessXian 为陷
	BrightnessXian = "xian"
)

// 星耀标识（Star.Key，含主星、辅星、杂耀、四组十二神与运限流耀）。
const (
	// StarZiweiMaj 为紫微
	StarZiweiMaj = "ziweiMaj"
	// StarTianjiMaj 为天机
	StarTianjiMaj = "tianjiMaj"
	// StarTaiyangMaj 为太阳
	StarTaiyangMaj = "taiyangMaj"
	// StarWuquMaj 为武曲
	StarWuquMaj = "wuquMaj"
	// StarTiantongMaj 为天同
	StarTiantongMaj = "tiantongMaj"
	// StarLianzhenMaj 为廉贞
	StarLianzhenMaj = "lianzhenMaj"
	// StarTianfuMaj 为天府
	StarTianfuMaj = "tianfuMaj"
	// StarTaiyinMaj 为太阴
	StarTaiyinMaj = "taiyinMaj"
	// StarTanlangMaj 为贪狼
	StarTanlangMaj = "tanlangMaj"
	// StarJumenMaj 为巨门
	StarJumenMaj = "jumenMaj"
	// StarTianxiangMaj 为天相
	StarTianxiangMaj = "tianxiangMaj"
	// StarTianliangMaj 为天梁
	StarTianliangMaj = "tianliangMaj"
	// StarQishaMaj 为七杀
	StarQishaMaj = "qishaMaj"
	// StarPojunMaj 为破军
	StarPojunMaj = "pojunMaj"
	// StarZuofuMin 为左辅
	StarZuofuMin = "zuofuMin"
	// StarYoubiMin 为右弼
	StarYoubiMin = "youbiMin"
	// StarWenchangMin 为文昌
	StarWenchangMin = "wenchangMin"
	// StarWenquMin 为文曲
	StarWenquMin = "wenquMin"
	// StarLucunMin 为禄存
	StarLucunMin = "lucunMin"
	// StarTianmaMin 为天马
	StarTianmaMin = "tianmaMin"
	// StarQingyangMin 为擎羊
	StarQingyangMin = "qingyangMin"
	// StarTuoluoMin 为陀罗
	StarTuoluoMin = "tuoluoMin"
	// StarHuoxingMin 为火星
	StarHuoxingMin = "huoxingMin"
	// StarLingxingMin 为铃星
	StarLingxingMin = "lingxingMin"
	// StarTiankuiMin 为天魁
	StarTiankuiMin = "tiankuiMin"
	// StarTianyueMin 为天钺
	StarTianyueMin = "tianyueMin"
	// StarDikongMin 为地空
	StarDikongMin = "dikongMin"
	// StarDijieMin 为地劫
	StarDijieMin = "dijieMin"
	// StarJieshaAdj 为劫杀
	StarJieshaAdj = "jieshaAdj"
	// StarTiankong 为天空
	StarTiankong = "tiankong"
	// StarTianxing 为天刑
	StarTianxing = "tianxing"
	// StarTianyao 为天姚
	StarTianyao = "tianyao"
	// StarJieshen 为解神
	StarJieshen = "jieshen"
	// StarYinsha 为阴煞
	StarYinsha = "yinsha"
	// StarTianxi 为天喜
	StarTianxi = "tianxi"
	// StarTianguan 为天官
	StarTianguan = "tianguan"
	// StarTianfu 为天福
	StarTianfu = "tianfu"
	// StarTianku 为天哭
	StarTianku = "tianku"
	// StarTianxu 为天虚
	StarTianxu = "tianxu"
	// StarLongchi 为龙池
	StarLongchi = "longchi"
	// StarFengge 为凤阁
	StarFengge = "fengge"
	// StarHongluan 为红鸾
	StarHongluan = "hongluan"
	// StarGuchen 为孤辰
	StarGuchen = "guchen"
	// StarGuasu 为寡宿
	StarGuasu = "guasu"
	// StarFeilian 为蜚廉
	StarFeilian = "feilian"
	// StarPosui 为破碎
	StarPosui = "posui"
	// StarTaifu 为台辅
	StarTaifu = "taifu"
	// StarFenggao 为封诰
	StarFenggao = "fenggao"
	// StarTianwu 为天巫
	StarTianwu = "tianwu"
	// StarTianyue2 为天月
	StarTianyue2 = "tianyue"
	// StarSantai 为三台
	StarSantai = "santai"
	// StarBazuo 为八座
	StarBazuo = "bazuo"
	// StarEngguang 为恩光
	StarEngguang = "engguang"
	// StarTiangui 为天贵
	StarTiangui = "tiangui"
	// StarTiancai 为天才
	StarTiancai = "tiancai"
	// StarTianshou 为天寿
	StarTianshou = "tianshou"
	// StarJiekong 为截空
	StarJiekong = "jiekong"
	// StarXunzhong 为旬中
	StarXunzhong = "xunzhong"
	// StarXunkong 为旬空
	StarXunkong = "xunkong"
	// StarKongwang 为空亡
	StarKongwang = "kongwang"
	// StarJielu 为截路
	StarJielu = "jielu"
	// StarYuede 为月德
	StarYuede = "yuede"
	// StarTianshang 为天伤
	StarTianshang = "tianshang"
	// StarTianshi 为天使
	StarTianshi = "tianshi"
	// StarTianchu 为天厨
	StarTianchu = "tianchu"
	// StarChangsheng 为长生
	StarChangsheng = "changsheng"
	// StarMuyu 为沐浴
	StarMuyu = "muyu"
	// StarGuandai 为冠带
	StarGuandai = "guandai"
	// StarLinguan 为临官
	StarLinguan = "linguan"
	// StarDiwang 为帝旺
	StarDiwang = "diwang"
	// StarShuai 为衰
	StarShuai = "shuai"
	// StarBing 为病
	StarBing = "bing"
	// StarSi 为死
	StarSi = "si"
	// StarMu 为墓
	StarMu = "mu"
	// StarJue 为绝
	StarJue = "jue"
	// StarTai 为胎
	StarTai = "tai"
	// StarYang 为养
	StarYang = "yang"
	// StarBoshi 为博士
	StarBoshi = "boshi"
	// StarLishi 为力士
	StarLishi = "lishi"
	// StarQinglong 为青龙
	StarQinglong = "qinglong"
	// StarXiaohao 为小耗
	StarXiaohao = "xiaohao"
	// StarJiangjun 为将军
	StarJiangjun = "jiangjun"
	// StarZhoushu 为奏书
	StarZhoushu = "zhoushu"
	// StarFaylian 为飞廉
	StarFaylian = "faylian"
	// StarXishen 为喜神
	StarXishen = "xishen"
	// StarBingfu 为病符
	StarBingfu = "bingfu"
	// StarDahao 为大耗
	StarDahao = "dahao"
	// StarSuipo 为岁破
	StarSuipo = "suipo"
	// StarFubing 为伏兵
	StarFubing = "fubing"
	// StarGuanfu 为官府
	StarGuanfu = "guanfu"
	// StarSuijian 为岁建
	StarSuijian = "suijian"
	// StarHuiqi 为晦气
	StarHuiqi = "huiqi"
	// StarSangmen 为丧门
	StarSangmen = "sangmen"
	// StarGuansuo 为贯索
	StarGuansuo = "guansuo"
	// StarGwanfu 为官符
	StarGwanfu = "gwanfu"
	// StarLongde 为龙德
	StarLongde = "longde"
	// StarBaihu 为白虎
	StarBaihu = "baihu"
	// StarTiande 为天德
	StarTiande = "tiande"
	// StarDiaoke 为吊客
	StarDiaoke = "diaoke"
	// StarJiangxing 为将星
	StarJiangxing = "jiangxing"
	// StarPanan 为攀鞍
	StarPanan = "panan"
	// StarSuiyi 为岁驿
	StarSuiyi = "suiyi"
	// StarXiishen 为息神
	StarXiishen = "xiishen"
	// StarHuagai 为华盖
	StarHuagai = "huagai"
	// StarJiesha 为劫煞
	StarJiesha = "jiesha"
	// StarZhaisha 为灾煞
	StarZhaisha = "zhaisha"
	// StarTiansha 为天煞
	StarTiansha = "tiansha"
	// StarZhibei 为指背
	StarZhibei = "zhibei"
	// StarXianchi 为咸池
	StarXianchi = "xianchi"
	// StarYuesha 为月煞
	StarYuesha = "yuesha"
	// StarWangshen 为亡神
	StarWangshen = "wangshen"
	// StarYunkui 为运魁
	StarYunkui = "yunkui"
	// StarYunyue 为运钺
	StarYunyue = "yunyue"
	// StarYunchang 为运昌
	StarYunchang = "yunchang"
	// StarYunqu 为运曲
	StarYunqu = "yunqu"
	// StarYunluan 为运鸾
	StarYunluan = "yunluan"
	// StarYunxi 为运喜
	StarYunxi = "yunxi"
	// StarYunlu 为运禄
	StarYunlu = "yunlu"
	// StarYunyang 为运羊
	StarYunyang = "yunyang"
	// StarYuntuo 为运陀
	StarYuntuo = "yuntuo"
	// StarYunma 为运马
	StarYunma = "yunma"
	// StarLiukui 为流魁
	StarLiukui = "liukui"
	// StarLiuyue 为流钺
	StarLiuyue = "liuyue"
	// StarLiuchang 为流昌
	StarLiuchang = "liuchang"
	// StarLiuqu 为流曲
	StarLiuqu = "liuqu"
	// StarLiuluan 为流鸾
	StarLiuluan = "liuluan"
	// StarLiuxi 为流喜
	StarLiuxi = "liuxi"
	// StarLiulu 为流禄
	StarLiulu = "liulu"
	// StarLiuyang 为流羊
	StarLiuyang = "liuyang"
	// StarLiutuo 为流陀
	StarLiutuo = "liutuo"
	// StarLiuma 为流马
	StarLiuma = "liuma"
	// StarNianjie 为年解
	StarNianjie = "nianjie"
	// StarYuekui 为月魁
	StarYuekui = "yuekui"
	// StarYueyue 为月钺
	StarYueyue = "yueyue"
	// StarYuechang 为月昌
	StarYuechang = "yuechang"
	// StarYuequ 为月曲
	StarYuequ = "yuequ"
	// StarYueluan 为月鸾
	StarYueluan = "yueluan"
	// StarYuexi 为月喜
	StarYuexi = "yuexi"
	// StarYuelu 为月禄
	StarYuelu = "yuelu"
	// StarYueyang 为月羊
	StarYueyang = "yueyang"
	// StarYuetuo 为月陀
	StarYuetuo = "yuetuo"
	// StarYuema 为月马
	StarYuema = "yuema"
	// StarRikui 为日魁
	StarRikui = "rikui"
	// StarRiyue 为日钺
	StarRiyue = "riyue"
	// StarRichang 为日昌
	StarRichang = "richang"
	// StarRiqu 为日曲
	StarRiqu = "riqu"
	// StarRiluan 为日鸾
	StarRiluan = "riluan"
	// StarRixi 为日喜
	StarRixi = "rixi"
	// StarRilu 为日禄
	StarRilu = "rilu"
	// StarRiyang 为日羊
	StarRiyang = "riyang"
	// StarRituo 为日陀
	StarRituo = "rituo"
	// StarRima 为日马
	StarRima = "rima"
	// StarShikui 为时魁
	StarShikui = "shikui"
	// StarShiyue 为时钺
	StarShiyue = "shiyue"
	// StarShichang 为时昌
	StarShichang = "shichang"
	// StarShiqu 为时曲
	StarShiqu = "shiqu"
	// StarShiluan 为时鸾
	StarShiluan = "shiluan"
	// StarShixi 为时喜
	StarShixi = "shixi"
	// StarShilu 为时禄
	StarShilu = "shilu"
	// StarShiyang 为时羊
	StarShiyang = "shiyang"
	// StarShituo 为时陀
	StarShituo = "shituo"
	// StarShima 为时马
	StarShima = "shima"
)

// 天干标识（Palace.HeavenlyStemKey、Decadal.HeavenlyStemKey、
// HoroscopeScope.HeavenlyStemKey，也是 Config.Mutagens 的键）。
const (
	// StemJia 为甲
	StemJia = "jiaHeavenly"
	// StemYi 为乙
	StemYi = "yiHeavenly"
	// StemBing 为丙
	StemBing = "bingHeavenly"
	// StemDing 为丁
	StemDing = "dingHeavenly"
	// StemWu 为戊
	StemWu = "wuHeavenly"
	// StemJi 为己
	StemJi = "jiHeavenly"
	// StemGeng 为庚
	StemGeng = "gengHeavenly"
	// StemXin 为辛
	StemXin = "xinHeavenly"
	// StemRen 为壬
	StemRen = "renHeavenly"
	// StemGui 为癸
	StemGui = "guiHeavenly"
)

// 地支标识（Palace.EarthlyBranchKey、Decadal.EarthlyBranchKey、
// HoroscopeScope.EarthlyBranchKey、Astrolabe.EarthlyBranchOfSoulPalaceKey 等）。
const (
	// BranchZi 为子
	BranchZi = "ziEarthly"
	// BranchChou 为丑
	BranchChou = "chouEarthly"
	// BranchYin 为寅
	BranchYin = "yinEarthly"
	// BranchMao 为卯
	BranchMao = "maoEarthly"
	// BranchChen 为辰
	BranchChen = "chenEarthly"
	// BranchSi 为巳
	BranchSi = "siEarthly"
	// BranchWu 为午
	BranchWu = "wuEarthly"
	// BranchWei 为未
	BranchWei = "weiEarthly"
	// BranchShen 为申
	BranchShen = "shenEarthly"
	// BranchYou 为酉
	BranchYou = "youEarthly"
	// BranchXu 为戌
	BranchXu = "xuEarthly"
	// BranchHai 为亥
	BranchHai = "haiEarthly"
)

// 五行局标识（Astrolabe.FiveElementsClassKey）。
const (
	// ClassWater2nd 为水二局
	ClassWater2nd = "water2nd"
	// ClassWood3rd 为木三局
	ClassWood3rd = "wood3rd"
	// ClassMetal4th 为金四局
	ClassMetal4th = "metal4th"
	// ClassEarth5th 为土五局
	ClassEarth5th = "earth5th"
	// ClassFire6th 为火六局
	ClassFire6th = "fire6th"
)

// fiveElementsClassNumbers 为五行局标识到局数的映射，
// 与 constants 查询的 FIVE_ELEMENTS_CLASS 一致。
var fiveElementsClassNumbers = map[string]int{
	ClassWater2nd: 2,
	ClassWood3rd:  3,
	ClassMetal4th: 4,
	ClassEarth5th: 5,
	ClassFire6th:  6,
}

// FiveElementsClassNumber 取五行局的局数（水二局为 2，火六局为 6）；
// 不是五行局标识时返回 0。
//
// 局数即大限每步的年数，也是紫微星起盘的除数。
func FiveElementsClassNumber(fiveElementsClassKey string) int {
	return fiveElementsClassNumbers[fiveElementsClassKey]
}

// Gender 是性别标识（排盘入口的 gender 参数与 Astrolabe.GenderKey 的取值类型）。
//
// 具名类型让编译器挡住把语言码、宫位 key 等别的字符串误传成性别的错误；
// 字面量 "male" / "female" 仍可直接传入。
type Gender string

// 性别标识的全部取值。
const (
	// GenderMale 为男
	GenderMale Gender = "male"
	// GenderFemale 为女
	GenderFemale Gender = "female"
)

// Language 是盘面语言标识（各入口的 language 参数与 Astrolabe.Language 的取值类型）。
//
// 取值与 iztro 一致（"zh-CN" 等，大小写不敏感，下划线写法 "zh_cn" 也接受）。
type Language string

// 盘面语言的全部取值。
const (
	// LanguageZhCN 为简体中文
	LanguageZhCN Language = "zh-CN"
	// LanguageZhTW 为繁体中文
	LanguageZhTW Language = "zh-TW"
	// LanguageEnUS 为英文
	LanguageEnUS Language = "en-US"
	// LanguageJaJP 为日文
	LanguageJaJP Language = "ja-JP"
	// LanguageKoKR 为韩文
	LanguageKoKR Language = "ko-KR"
	// LanguageViVN 为越南文
	LanguageViVN Language = "vi-VN"
)

// LeapMonth 是农历输入的闰月处理方式（ByLunar 等农历入口的 leap 参数）。
//
// 它把 iztro byLunar 的 isLeapMonth 与 fixLeap 两个布尔合成一个三态值：
// 两个布尔相邻传参极易写反且不报错，而 fixLeap 只在输入是闰月时才有意义。
// 阳历入口的 fixLeap 仍是单个布尔。
type LeapMonth string

// 闰月处理方式的全部取值。
const (
	// NotLeapMonth 表示输入的农历月不是闰月
	NotLeapMonth LeapMonth = "notLeap"
	// LeapMonthKeep 表示输入的农历月是闰月，按闰月本身排盘
	LeapMonthKeep LeapMonth = "leap"
	// LeapMonthFixed 表示输入的农历月是闰月，且十五之后视作次月（iztro fixLeap）
	LeapMonthFixed LeapMonth = "leapFixed"
)

// flags 把闰月处理方式拆回 wasm 线协议使用的 isLeapMonth / fixLeap 两个布尔；
// 未知取值返回错误。
func (m LeapMonth) flags() (isLeapMonth bool, fixLeap bool, err error) {
	switch m {
	case NotLeapMonth:
		return false, false, nil
	case LeapMonthKeep:
		return true, false, nil
	case LeapMonthFixed:
		return true, true, nil
	default:
		return false, false, invalidArgument("invalid leap month '" + string(m) + "': expected 'notLeap', 'leap' or 'leapFixed'")
	}
}

// 宫位定位的特殊标识：不是十二宫之一，而是定位「被标记为该角色的那一宫」，
// 可传给 Astrolabe.Palace 与 Astrolabe.SurroundedPalaces。
const (
	// PalaceBody 定位身宫
	PalaceBody = "bodyPalace"
	// PalaceOriginal 定位来因宫（一张盘可能没有）
	PalaceOriginal = "originalPalace"
)

// 运限层级标识（Star.Scope，以及 Horoscope 各查询方法的 scope 参数）。
const (
	// ScopeOrigin 为本命
	ScopeOrigin = "origin"
	// ScopeDecadal 为大限
	ScopeDecadal = "decadal"
	// ScopeYearly 为流年
	ScopeYearly = "yearly"
	// ScopeMonthly 为流月
	ScopeMonthly = "monthly"
	// ScopeDaily 为流日
	ScopeDaily = "daily"
	// ScopeHourly 为流时
	ScopeHourly = "hourly"
)

// 年分界点（Config.YearDivide）：年干支按哪一天换年。
const (
	// YearDivideNormal 以正月初一换年
	YearDivideNormal = "normal"
	// YearDivideExact 以立春换年
	YearDivideExact = "exact"
)

// 运限分界点（Config.HoroscopeDivide）：流年神煞取年支时按哪一天换年。
const (
	// HoroscopeDivideNormal 以正月初一换年
	HoroscopeDivideNormal = "normal"
	// HoroscopeDivideExact 以立春换年
	HoroscopeDivideExact = "exact"
)

// 虚岁分界点（Config.AgeDivide）：虚岁在哪一天增长。
const (
	// AgeDivideNormal 以农历年换岁
	AgeDivideNormal = "normal"
	// AgeDivideBirthday 以生日换岁
	AgeDivideBirthday = "birthday"
)

// 晚子时归属（Config.DayDivide）：23:00-24:00 出生算哪一天。
const (
	// DayDivideForward 晚子时归次日
	DayDivideForward = "forward"
	// DayDivideCurrent 晚子时归当日
	DayDivideCurrent = "current"
)

// 算法派别（Config.Algorithm）。
const (
	// AlgorithmDefault 为默认派别
	AlgorithmDefault = "default"
	// AlgorithmZhongzhou 为中州派
	AlgorithmZhongzhou = "zhongzhou"
)

// 排盘视角（Config.AstroType）：中州派的天盘、地盘、人盘。
const (
	// AstroHeaven 天盘：以命宫干支起五行局，即常规排盘结果
	AstroHeaven = "heaven"
	// AstroEarth 地盘：以身宫干支起五行局，身宫即为新盘的命宫
	AstroEarth = "earth"
	// AstroHuman 人盘：以福德宫干支起五行局，福德宫即为新盘的命宫
	AstroHuman = "human"
)

// 星耀类型标识（Star.Type）。
const (
	// StarTypeMajor 主星
	StarTypeMajor = "major"
	// StarTypeSoft 吉星
	StarTypeSoft = "soft"
	// StarTypeTough 煞星
	StarTypeTough = "tough"
	// StarTypeAdjective 杂耀
	StarTypeAdjective = "adjective"
	// StarTypeFlower 桃花星
	StarTypeFlower = "flower"
	// StarTypeHelper 解神
	StarTypeHelper = "helper"
	// StarTypeLucun 禄存
	StarTypeLucun = "lucun"
	// StarTypeTianma 天马
	StarTypeTianma = "tianma"
)

// 格局标识（PatternHit.Key），与 iztro-docs《格局》页条目一一对应。
const (
	// PatternJunChenQingHui 为君臣庆会
	PatternJunChenQingHui = "jun_chen_qing_hui"
	// PatternZiFuTongGong 为紫府同宫
	PatternZiFuTongGong = "zi_fu_tong_gong"
	// PatternJinYuFuJia 为金舆扶驾
	PatternJinYuFuJia = "jin_yu_fu_jia"
	// PatternZiFuJiaMing 为紫府夹命
	PatternZiFuJiaMing = "zi_fu_jia_ming"
	// PatternJiXiangLiMing 为极向离明
	PatternJiXiangLiMing = "ji_xiang_li_ming"
	// PatternJiJuMaoYou 为极居卯酉
	PatternJiJuMaoYou = "ji_ju_mao_you"
	// PatternJiYueTongLiang 为机月同梁
	PatternJiYueTongLiang = "ji_yue_tong_liang"
	// PatternShanYinChaoGang 为善荫朝纲
	PatternShanYinChaoGang = "shan_yin_chao_gang"
	// PatternJiJuTongLin 为机巨同临
	PatternJiJuTongLin = "ji_ju_tong_lin"
	// PatternJiJuJuMao 为机巨居卯
	PatternJiJuJuMao = "ji_ju_ju_mao"
	// PatternRiYueTongGong 为日月同宫
	PatternRiYueTongGong = "ri_yue_tong_gong"
	// PatternJuRiTongGong 为巨日同宫
	PatternJuRiTongGong = "ju_ri_tong_gong"
	// PatternRiZhaoLeiMen 为日照雷门
	PatternRiZhaoLeiMen = "ri_zhao_lei_men"
	// PatternRiYueBingMing 为日月并明
	PatternRiYueBingMing = "ri_yue_bing_ming"
	// PatternRiYueFanBei 为日月反背
	PatternRiYueFanBei = "ri_yue_fan_bei"
	// PatternRiYueZhaoBi 为日月照璧
	PatternRiYueZhaoBi = "ri_yue_zhao_bi"
	// PatternJinCanGuangHui 为金灿光辉
	PatternJinCanGuangHui = "jin_can_guang_hui"
	// PatternRiYueCangHui 为日月藏辉
	PatternRiYueCangHui = "ri_yue_cang_hui"
	// PatternDanChiGuiChi 为丹墀桂墀
	PatternDanChiGuiChi = "dan_chi_gui_chi"
	// PatternRiYueJiaMing 为日月夹命
	PatternRiYueJiaMing = "ri_yue_jia_ming"
	// PatternRiYueJiaCai 为日月夹财
	PatternRiYueJiaCai = "ri_yue_jia_cai"
	// PatternYueLangTianMen 为月朗天门
	PatternYueLangTianMen = "yue_lang_tian_men"
	// PatternYueShengCangHai 为月生沧海
	PatternYueShengCangHai = "yue_sheng_cang_hai"
	// PatternMingZhuChuHai 为明珠出海
	PatternMingZhuChuHai = "ming_zhu_chu_hai"
	// PatternWuTanTongXing 为武贪同行
	PatternWuTanTongXing = "wu_tan_tong_xing"
	// PatternLingChangTuoWu 为铃昌陀武
	PatternLingChangTuoWu = "ling_chang_tuo_wu"
	// PatternXingQiuJiaYin 为刑囚夹印
	PatternXingQiuJiaYin = "xing_qiu_jia_yin"
	// PatternShengBuFengShi 为生不逢时
	PatternShengBuFengShi = "sheng_bu_feng_shi"
	// PatternXiongSuChaoYuan 为雄宿朝元
	PatternXiongSuChaoYuan = "xiong_su_chao_yuan"
	// PatternFuXiangChaoYuan 为府相朝垣
	PatternFuXiangChaoYuan = "fu_xiang_chao_yuan"
	// PatternHuoTan 为火贪
	PatternHuoTan = "huo_tan"
	// PatternLingTan 为铃贪
	PatternLingTan = "ling_tan"
	// PatternShiZhongYinYu 为石中隐玉
	PatternShiZhongYinYu = "shi_zhong_yin_yu"
	// PatternLiangMaPiaoDang 为梁马飘荡
	PatternLiangMaPiaoDang = "liang_ma_piao_dang"
	// PatternYangLiangChangLu 为阳梁昌禄
	PatternYangLiangChangLu = "yang_liang_chang_lu"
	// PatternShaPoLang 为杀破狼
	PatternShaPoLang = "sha_po_lang"
	// PatternQiShaChaoDou 为七杀朝斗
	PatternQiShaChaoDou = "qi_sha_chao_dou"
	// PatternLuShuaiMaKun 为禄衰马困
	PatternLuShuaiMaKun = "lu_shuai_ma_kun"
	// PatternYingXingRuMiao 为英星入庙
	PatternYingXingRuMiao = "ying_xing_ru_miao"
	// PatternZhongShuiChaoDong 为众水朝东
	PatternZhongShuiChaoDong = "zhong_shui_chao_dong"
	// PatternSanQiJiaHui 为三奇加会
	PatternSanQiJiaHui = "san_qi_jia_hui"
	// PatternLuMaJiaoChi 为禄马交驰
	PatternLuMaJiaoChi = "lu_ma_jiao_chi"
	// PatternLuHeYuanYang 为禄合鸳鸯
	PatternLuHeYuanYang = "lu_he_yuan_yang"
	// PatternMingLuAnLu 为明禄暗禄
	PatternMingLuAnLu = "ming_lu_an_lu"
	// PatternLuMaPeiYin 为禄马佩印
	PatternLuMaPeiYin = "lu_ma_pei_yin"
	// PatternLiangChongHuaGai 为两重华盖
	PatternLiangChongHuaGai = "liang_chong_hua_gai"
	// PatternFengYunJiHui 为风云际会
	PatternFengYunJiHui = "feng_yun_ji_hui"
	// PatternYangTuoJiaMing 为羊陀夹命
	PatternYangTuoJiaMing = "yang_tuo_jia_ming"
	// PatternMaTouDaiJian 为马头带箭
	PatternMaTouDaiJian = "ma_tou_dai_jian"
	// PatternZuoYouTongGong 为左右同宫
	PatternZuoYouTongGong = "zuo_you_tong_gong"
	// PatternZuoYouJiaMing 为左右夹命
	PatternZuoYouJiaMing = "zuo_you_jia_ming"
	// PatternFuBiGongZhu 为辅弼拱主
	PatternFuBiGongZhu = "fu_bi_gong_zhu"
	// PatternKuiYueJiaMing 为魁钺夹命
	PatternKuiYueJiaMing = "kui_yue_jia_ming"
	// PatternZuoGuiXiangGui 为坐贵向贵
	PatternZuoGuiXiangGui = "zuo_gui_xiang_gui"
	// PatternJieKongJiaMing 为劫空夹命
	PatternJieKongJiaMing = "jie_kong_jia_ming"
	// PatternLuFengLiangSha 为禄逢两杀
	PatternLuFengLiangSha = "lu_feng_liang_sha"
	// PatternWenGuiWenHua 为文贵文华
	PatternWenGuiWenHua = "wen_gui_wen_hua"
	// PatternWenXingChaoMing 为文星朝命
	PatternWenXingChaoMing = "wen_xing_chao_ming"
	// PatternChangQuJiaMing 为昌曲夹命
	PatternChangQuJiaMing = "chang_qu_jia_ming"
	// PatternWenXingAnGong 为文星暗拱
	PatternWenXingAnGong = "wen_xing_an_gong"
	// PatternQuanLuShengFeng 为权禄生逢
	PatternQuanLuShengFeng = "quan_lu_sheng_feng"
	// PatternKeMingAnLu 为科明暗禄
	PatternKeMingAnLu = "ke_ming_an_lu"
	// PatternKeQuanLuJia 为科权禄夹
	PatternKeQuanLuJia = "ke_quan_lu_jia"
	// PatternJiaDiDengYong 为甲第登庸
	PatternJiaDiDengYong = "jia_di_deng_yong"
)

// 格局判定中日月明暗的依据（PatternConfig.BrightnessSource）。
const (
	// BrightnessSourceTable 按 iztro 亮度表：庙旺为明，陷与「不」为暗
	BrightnessSourceTable = "table"
	// BrightnessSourcePositional 按传统位置：太阳寅至午明、酉至丑暗；太阴酉至丑明、卯至未暗
	BrightnessSourcePositional = "positional"
)

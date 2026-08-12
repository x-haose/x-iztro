"""
rs-iztro 枚举与常量。

全部枚举取语言无关的标识值（iztro i18n key），与数据对象上的
`key`/`*_key` 字段直接比较——因此在任何输出语言的星盘上都能正确判断。
展示文本请读取数据对象上对应的翻译字段（如 `Star.name`、`Palace.name`）。
"""

from __future__ import annotations

from enum import StrEnum


class Gender(StrEnum):
    """性别（排盘入参与 `Astrolabe.gender_key`）"""

    MALE = "male"
    FEMALE = "female"


class Language(StrEnum):
    """输出语言（排盘入参与 `Astrolabe.language`）"""

    ZH_CN = "zh_cn"
    ZH_TW = "zh_tw"
    EN_US = "en_us"
    JA_JP = "ja_jp"
    KO_KR = "ko_kr"
    VI_VN = "vi_vn"


class Algorithm(StrEnum):
    """算法派别（`ChartConfig.algorithm`）"""

    DEFAULT = "default"
    ZHONGZHOU = "zhongzhou"


class YearDivide(StrEnum):
    """年分界点（`ChartConfig.year_divide`）：安星年干支按正月初一或立春换年"""

    NORMAL = "normal"
    EXACT = "exact"


class HoroscopeDivide(StrEnum):
    """运限分界点（`ChartConfig.horoscope_divide`）：运限干支按初一或节气推算"""

    NORMAL = "normal"
    EXACT = "exact"


class AgeDivide(StrEnum):
    """虚岁分界点（`ChartConfig.age_divide`）：跨农历年即加一岁或过生日才加"""

    NORMAL = "normal"
    BIRTHDAY = "birthday"


class DayDivide(StrEnum):
    """晚子时归属（`ChartConfig.day_divide`）：归次日或归当天"""

    FORWARD = "forward"
    CURRENT = "current"


class HeavenlyStem(StrEnum):
    """天干（对应 `heavenly_stem_key` 字段）"""

    JIA = "jiaHeavenly"
    YI = "yiHeavenly"
    BING = "bingHeavenly"
    DING = "dingHeavenly"
    WU = "wuHeavenly"
    JI = "jiHeavenly"
    GENG = "gengHeavenly"
    XIN = "xinHeavenly"
    REN = "renHeavenly"
    GUI = "guiHeavenly"


class EarthlyBranch(StrEnum):
    """地支（对应 `earthly_branch_key` 字段）"""

    ZI = "ziEarthly"
    CHOU = "chouEarthly"
    YIN = "yinEarthly"
    MAO = "maoEarthly"
    CHEN = "chenEarthly"
    SI = "siEarthly"
    WU = "wuEarthly"
    WEI = "weiEarthly"
    SHEN = "shenEarthly"
    YOU = "youEarthly"
    XU = "xuEarthly"
    HAI = "haiEarthly"


class PalaceName(StrEnum):
    """十二宫（对应 `Palace.name_key`，可直接传入 `Astrolabe.palace()` 等查询方法）"""

    SOUL = "soulPalace"
    """命宫"""
    SIBLINGS = "siblingsPalace"
    """兄弟"""
    SPOUSE = "spousePalace"
    """夫妻"""
    CHILDREN = "childrenPalace"
    """子女"""
    WEALTH = "wealthPalace"
    """财帛"""
    HEALTH = "healthPalace"
    """疾厄"""
    SURFACE = "surfacePalace"
    """迁移"""
    FRIENDS = "friendsPalace"
    """仆役（又名交友）"""
    CAREER = "careerPalace"
    """官禄"""
    PROPERTY = "propertyPalace"
    """田宅"""
    SPIRIT = "spiritPalace"
    """福德"""
    PARENTS = "parentsPalace"
    """父母"""


class FiveElementsClass(StrEnum):
    """五行局（对应 `Astrolabe.five_elements_class_key`）"""

    WATER_2 = "water2nd"
    """水二局"""
    WOOD_3 = "wood3rd"
    """木三局"""
    METAL_4 = "metal4th"
    """金四局"""
    EARTH_5 = "earth5th"
    """土五局"""
    FIRE_6 = "fire6th"
    """火六局"""


class Mutagen(StrEnum):
    """四化（对应 `mutagen_key` 字段，可传入 `Palace.has_mutagen()` 等方法）"""

    LU = "sihuaLu"
    """化禄"""
    QUAN = "sihuaQuan"
    """化权"""
    KE = "sihuaKe"
    """化科"""
    JI = "sihuaJi"
    """化忌"""


class Brightness(StrEnum):
    """星耀亮度（对应 `Star.brightness_key`）"""

    MIAO = "miao"
    """庙"""
    WANG = "wang"
    """旺"""
    DE = "de"
    """得"""
    LI = "li"
    """利"""
    PING = "ping"
    """平"""
    BU = "bu"
    """不"""
    XIAN = "xian"
    """陷"""


class StarType(StrEnum):
    """星耀类型（`Star.type`）"""

    MAJOR = "major"
    """主星"""
    SOFT = "soft"
    """吉星"""
    TOUGH = "tough"
    """煞星"""
    ADJECTIVE = "adjective"
    """杂耀"""
    FLOWER = "flower"
    """桃花星"""
    HELPER = "helper"
    """解神类"""
    LUCUN = "lucun"
    """禄存"""
    TIANMA = "tianma"
    """天马"""


class Scope(StrEnum):
    """作用范围（`Star.scope` 与运限查询方法的 scope 参数）"""

    ORIGIN = "origin"
    """本命"""
    DECADAL = "decadal"
    """大限"""
    YEARLY = "yearly"
    """流年"""
    MONTHLY = "monthly"
    """流月"""
    DAILY = "daily"
    """流日"""
    HOURLY = "hourly"
    """流时"""


class MajorStar(StrEnum):
    """十四主星（对应 `Star.key`）"""

    ZIWEI = "ziweiMaj"
    """紫微"""
    TIANJI = "tianjiMaj"
    """天机"""
    TAIYANG = "taiyangMaj"
    """太阳"""
    WUQU = "wuquMaj"
    """武曲"""
    TIANTONG = "tiantongMaj"
    """天同"""
    LIANZHEN = "lianzhenMaj"
    """廉贞"""
    TIANFU = "tianfuMaj"
    """天府"""
    TAIYIN = "taiyinMaj"
    """太阴"""
    TANLANG = "tanlangMaj"
    """贪狼"""
    JUMEN = "jumenMaj"
    """巨门"""
    TIANXIANG = "tianxiangMaj"
    """天相"""
    TIANLIANG = "tianliangMaj"
    """天梁"""
    QISHA = "qishaMaj"
    """七杀"""
    POJUN = "pojunMaj"
    """破军"""


class MinorStar(StrEnum):
    """十四辅星（对应 `Star.key`）"""

    ZUOFU = "zuofuMin"
    """左辅"""
    YOUBI = "youbiMin"
    """右弼"""
    WENCHANG = "wenchangMin"
    """文昌"""
    WENQU = "wenquMin"
    """文曲"""
    LUCUN = "lucunMin"
    """禄存"""
    TIANMA = "tianmaMin"
    """天马"""
    QINGYANG = "qingyangMin"
    """擎羊"""
    TUOLUO = "tuoluoMin"
    """陀罗"""
    HUOXING = "huoxingMin"
    """火星"""
    LINGXING = "lingxingMin"
    """铃星"""
    TIANKUI = "tiankuiMin"
    """天魁"""
    TIANYUE = "tianyueMin"
    """天钺"""
    DIKONG = "dikongMin"
    """地空"""
    DIJIE = "dijieMin"
    """地劫"""


class AdjectiveStar(StrEnum):
    """杂耀（对应 `Star.key`）"""

    JIESHA = "jieshaAdj"
    """劫杀（中州派）"""
    TIANKONG = "tiankong"
    """天空"""
    TIANXING = "tianxing"
    """天刑"""
    TIANYAO = "tianyao"
    """天姚"""
    JIESHEN = "jieshen"
    """解神"""
    YINSHA = "yinsha"
    """阴煞"""
    TIANXI = "tianxi"
    """天喜"""
    TIANGUAN = "tianguan"
    """天官"""
    TIANFU = "tianfu"
    """天福"""
    TIANKU = "tianku"
    """天哭"""
    TIANXU = "tianxu"
    """天虚"""
    LONGCHI = "longchi"
    """龙池"""
    FENGGE = "fengge"
    """凤阁"""
    HONGLUAN = "hongluan"
    """红鸾"""
    GUCHEN = "guchen"
    """孤辰"""
    GUASU = "guasu"
    """寡宿"""
    FEILIAN = "feilian"
    """蜚廉"""
    POSUI = "posui"
    """破碎"""
    TAIFU = "taifu"
    """台辅"""
    FENGGAO = "fenggao"
    """封诰"""
    TIANWU = "tianwu"
    """天巫"""
    TIANYUE = "tianyue"
    """天月"""
    SANTAI = "santai"
    """三台"""
    BAZUO = "bazuo"
    """八座"""
    ENGGUANG = "engguang"
    """恩光"""
    TIANGUI = "tiangui"
    """天贵"""
    TIANCAI = "tiancai"
    """天才"""
    TIANSHOU = "tianshou"
    """天寿"""
    JIEKONG = "jiekong"
    """截空（中州派）"""
    XUNKONG = "xunkong"
    """旬空"""
    KONGWANG = "kongwang"
    """空亡"""
    JIELU = "jielu"
    """截路"""
    YUEDE = "yuede"
    """月德"""
    TIANSHANG = "tianshang"
    """天伤"""
    TIANSHI = "tianshi"
    """天使"""
    TIANCHU = "tianchu"
    """天厨"""
    NIANJIE = "nianjie"
    """年解"""
    XIANCHI = "xianchi"
    """咸池"""
    HUAGAI = "huagai"
    """华盖"""
    TIANDE = "tiande"
    """天德"""
    LONGDE = "longde"
    """龙德（中州派）"""
    DAHAO = "dahao"
    """大耗（中州派杂耀）"""


class Changsheng12(StrEnum):
    """长生十二神（对应 `Palace.changsheng12_key`）"""

    CHANGSHENG = "changsheng"
    """长生"""
    MUYU = "muyu"
    """沐浴"""
    GUANDAI = "guandai"
    """冠带"""
    LINGUAN = "linguan"
    """临官"""
    DIWANG = "diwang"
    """帝旺"""
    SHUAI = "shuai"
    """衰"""
    BING = "bing"
    """病"""
    SI = "si"
    """死"""
    MU = "mu"
    """墓"""
    JUE = "jue"
    """绝"""
    TAI = "tai"
    """胎"""
    YANG = "yang"
    """养"""


class Boshi12(StrEnum):
    """博士十二神（对应 `Palace.boshi12_key`）"""

    BOSHI = "boshi"
    """博士"""
    LISHI = "lishi"
    """力士"""
    QINGLONG = "qinglong"
    """青龙"""
    XIAOHAO = "xiaohao"
    """小耗"""
    JIANGJUN = "jiangjun"
    """将军"""
    ZHOUSHU = "zhoushu"
    """奏书"""
    FEILIAN = "faylian"
    """飞廉"""
    XISHEN = "xishen"
    """喜神"""
    BINGFU = "bingfu"
    """病符"""
    DAHAO = "dahao"
    """大耗"""
    FUBING = "fubing"
    """伏兵"""
    GUANFU = "guanfu"
    """官府"""


class Suiqian12(StrEnum):
    """岁前十二神（对应 `Palace.suiqian12_key`）"""

    SUIJIAN = "suijian"
    """岁建"""
    HUIQI = "huiqi"
    """晦气"""
    SANGMEN = "sangmen"
    """丧门"""
    GUANSUO = "guansuo"
    """贯索"""
    GWANFU = "gwanfu"
    """官符"""
    XIAOHAO = "xiaohao"
    """小耗"""
    DAHAO = "dahao"
    """大耗"""
    SUIPO = "suipo"
    """岁破（中州派）"""
    LONGDE = "longde"
    """龙德"""
    BAIHU = "baihu"
    """白虎"""
    TIANDE = "tiande"
    """天德"""
    DIAOKE = "diaoke"
    """吊客"""
    BINGFU = "bingfu"
    """病符"""


class Jiangqian12(StrEnum):
    """将前十二神（对应 `Palace.jiangqian12_key`）"""

    JIANGXING = "jiangxing"
    """将星"""
    PANAN = "panan"
    """攀鞍"""
    SUIYI = "suiyi"
    """岁驿"""
    XISHEN = "xiishen"
    """息神"""
    HUAGAI = "huagai"
    """华盖"""
    JIESHA = "jiesha"
    """劫煞"""
    ZHAISHA = "zhaisha"
    """灾煞"""
    TIANSHA = "tiansha"
    """天煞"""
    ZHIBEI = "zhibei"
    """指背"""
    XIANCHI = "xianchi"
    """咸池"""
    YUESHA = "yuesha"
    """月煞"""
    WANGSHEN = "wangshen"
    """亡神"""

"""
x-iztro 枚举与常量。

全部枚举取语言无关的标识值（iztro i18n key），与数据对象上的
`key`/`*_key` 字段直接比较——因此在任何输出语言的星盘上都能正确判断。
展示文本请读取数据对象上对应的翻译字段（如 `Star.name`、`Palace.name`）。

排盘入参的类型别名与四化位次也在这里：它们同样是「合法取值」这一件事的表达，
别名让 IDE 直接提示可选值，四化位次是「禄权科忌」固定顺序在各处的唯一来源。
"""

from __future__ import annotations

from typing import Literal

try:
    from enum import StrEnum
except ImportError:  # Python 3.10 没有 StrEnum
    from enum import Enum

    class StrEnum(str, Enum):  # type: ignore[no-redef]
        """`enum.StrEnum` 在 Python 3.10 上的等价实现。

        成员即其字符串值：`str(member)`、`f"{member}"` 与直接同字面量比较的
        结果都与 3.11+ 的 `StrEnum` 一致，因此枚举可以直接同星盘的
        `*_key` 字段比较。
        """

        __str__ = str.__str__  # type: ignore[assignment]
        __format__ = str.__format__  # type: ignore[assignment]


class Gender(StrEnum):
    """性别（排盘入参与 `Astrolabe.gender_key`）"""

    MALE = "male"
    FEMALE = "female"


class Language(StrEnum):
    """输出语言（排盘入参与 `Astrolabe.language`）"""

    ZH_CN = "zh-CN"
    ZH_TW = "zh-TW"
    EN_US = "en-US"
    JA_JP = "ja-JP"
    KO_KR = "ko-KR"
    VI_VN = "vi-VN"


class Algorithm(StrEnum):
    """算法派别（`ChartConfig.algorithm`）"""

    DEFAULT = "default"
    ZHONGZHOU = "zhongzhou"


class AstroType(StrEnum):
    """排盘视角（`ChartConfig.astro_type`）：中州派的天盘、地盘、人盘"""

    HEAVEN = "heaven"
    """天盘：以命宫干支起五行局，即常规排盘结果"""

    EARTH = "earth"
    """地盘：以身宫干支起五行局，身宫即为新盘的命宫"""

    HUMAN = "human"
    """人盘：以福德宫干支起五行局，福德宫即为新盘的命宫"""


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

    BODY = "bodyPalace"
    """身宫。不是十二宫之一，而是「被标记为身宫的那一宫」，供查询方法定位使用"""
    ORIGINAL = "originalPalace"
    """来因宫。同上，定位被标记为来因宫的那一宫；一张盘可能没有"""


class FiveElementsClass(StrEnum):
    """五行局（对应 `Astrolabe.five_elements_class_key`）。

    成员的 `number` 属性即局数（水二局为 2，火六局为 6），
    取值与数据表 `x_iztro.data.constants().five_elements_class` 一致；
    局数决定紫微起宫与大限起运岁数。
    """

    def __new__(cls, value: str, number: int) -> FiveElementsClass:
        """成员值仍是标识字符串，局数作为附加属性挂在成员上。"""
        member = str.__new__(cls, value)
        member._value_ = value
        member.number = number
        return member

    number: int
    """局数 2-6"""

    WATER_2 = ("water2nd", 2)
    """水二局"""
    WOOD_3 = ("wood3rd", 3)
    """木三局"""
    METAL_4 = ("metal4th", 4)
    """金四局"""
    EARTH_5 = ("earth5th", 5)
    """土五局"""
    FIRE_6 = ("fire6th", 6)
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
    XUNZHONG = "xunzhong"
    """旬中"""
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


class HoroscopeStar(StrEnum):
    """流耀（对应运限对象 `stars` 里的 `Star.key`）。

    同一颗星在每个运限层级各有一个标识与名称，前缀标出层级：
    `yun*` 大限、`liu*` 流年、`yue*` 流月、`ri*` 流日、`shi*` 流时。
    本命层级的对应星耀在 `MinorStar` 与 `AdjectiveStar` 里。
    """

    # 大限
    YUN_KUI = "yunkui"
    """运魁（大限层级的天魁）"""
    YUN_YUE = "yunyue"
    """运钺（大限层级的天钺）"""
    YUN_CHANG = "yunchang"
    """运昌（大限层级的文昌）"""
    YUN_QU = "yunqu"
    """运曲（大限层级的文曲）"""
    YUN_LUAN = "yunluan"
    """运鸾（大限层级的红鸾）"""
    YUN_XI = "yunxi"
    """运喜（大限层级的天喜）"""
    YUN_LU = "yunlu"
    """运禄（大限层级的禄存）"""
    YUN_YANG = "yunyang"
    """运羊（大限层级的擎羊）"""
    YUN_TUO = "yuntuo"
    """运陀（大限层级的陀罗）"""
    YUN_MA = "yunma"
    """运马（大限层级的天马）"""
    # 流年
    LIU_KUI = "liukui"
    """流魁（流年层级的天魁）"""
    LIU_YUE = "liuyue"
    """流钺（流年层级的天钺）"""
    LIU_CHANG = "liuchang"
    """流昌（流年层级的文昌）"""
    LIU_QU = "liuqu"
    """流曲（流年层级的文曲）"""
    LIU_LUAN = "liuluan"
    """流鸾（流年层级的红鸾）"""
    LIU_XI = "liuxi"
    """流喜（流年层级的天喜）"""
    LIU_LU = "liulu"
    """流禄（流年层级的禄存）"""
    LIU_YANG = "liuyang"
    """流羊（流年层级的擎羊）"""
    LIU_TUO = "liutuo"
    """流陀（流年层级的陀罗）"""
    LIU_MA = "liuma"
    """流马（流年层级的天马）"""
    # 流月
    YUE_KUI = "yuekui"
    """月魁（流月层级的天魁）"""
    YUE_YUE = "yueyue"
    """月钺（流月层级的天钺）"""
    YUE_CHANG = "yuechang"
    """月昌（流月层级的文昌）"""
    YUE_QU = "yuequ"
    """月曲（流月层级的文曲）"""
    YUE_LUAN = "yueluan"
    """月鸾（流月层级的红鸾）"""
    YUE_XI = "yuexi"
    """月喜（流月层级的天喜）"""
    YUE_LU = "yuelu"
    """月禄（流月层级的禄存）"""
    YUE_YANG = "yueyang"
    """月羊（流月层级的擎羊）"""
    YUE_TUO = "yuetuo"
    """月陀（流月层级的陀罗）"""
    YUE_MA = "yuema"
    """月马（流月层级的天马）"""
    # 流日
    RI_KUI = "rikui"
    """日魁（流日层级的天魁）"""
    RI_YUE = "riyue"
    """日钺（流日层级的天钺）"""
    RI_CHANG = "richang"
    """日昌（流日层级的文昌）"""
    RI_QU = "riqu"
    """日曲（流日层级的文曲）"""
    RI_LUAN = "riluan"
    """日鸾（流日层级的红鸾）"""
    RI_XI = "rixi"
    """日喜（流日层级的天喜）"""
    RI_LU = "rilu"
    """日禄（流日层级的禄存）"""
    RI_YANG = "riyang"
    """日羊（流日层级的擎羊）"""
    RI_TUO = "rituo"
    """日陀（流日层级的陀罗）"""
    RI_MA = "rima"
    """日马（流日层级的天马）"""
    # 流时
    SHI_KUI = "shikui"
    """时魁（流时层级的天魁）"""
    SHI_YUE = "shiyue"
    """时钺（流时层级的天钺）"""
    SHI_CHANG = "shichang"
    """时昌（流时层级的文昌）"""
    SHI_QU = "shiqu"
    """时曲（流时层级的文曲）"""
    SHI_LUAN = "shiluan"
    """时鸾（流时层级的红鸾）"""
    SHI_XI = "shixi"
    """时喜（流时层级的天喜）"""
    SHI_LU = "shilu"
    """时禄（流时层级的禄存）"""
    SHI_YANG = "shiyang"
    """时羊（流时层级的擎羊）"""
    SHI_TUO = "shituo"
    """时陀（流时层级的陀罗）"""
    SHI_MA = "shima"
    """时马（流时层级的天马）"""


TimeIndexType = Literal[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
GenderType = Literal["male", "female"]
LanguageType = Literal["zh-CN", "zh-TW", "en-US", "ja-JP", "ko-KR", "vi-VN"]
StarTypeLiteral = Literal[
    "major", "soft", "tough", "adjective", "flower", "helper", "lucun", "tianma"
]
ScopeLiteral = Literal["origin", "decadal", "yearly", "monthly", "daily", "hourly"]


# ============================================================
# 四化位次（四化标识 → 在 [禄, 权, 科, 忌] 中的下标）
# ============================================================

_MUTAGEN_INDEX: dict[str, int] = {
    Mutagen.LU: 0,
    Mutagen.QUAN: 1,
    Mutagen.KE: 2,
    Mutagen.JI: 3,
}


def _as_mutagen_list(mutagens: Mutagen | list[Mutagen]) -> list[Mutagen]:
    """把单个四化或四化列表统一成列表。"""
    if isinstance(mutagens, list):
        return mutagens
    return [mutagens]


def _or_all_mutagens(
    mutagens: Mutagen | list[Mutagen] | None,
) -> list[Mutagen]:
    """空值回退为全部四化，顺序为禄、权、科、忌。"""
    if mutagens is None:
        return list(Mutagen)
    out = _as_mutagen_list(mutagens)
    return out if out else list(Mutagen)

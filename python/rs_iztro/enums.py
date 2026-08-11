"""
rs-iztro 枚举与常量

与 Rust 侧一一对应，用于 IDE 自动补全和值比较。
所有值均为 zh_cn 下的翻译字符串（默认语言）。
"""

from __future__ import annotations

from enum import StrEnum


# ============================================================
# 性别
# ============================================================

class Gender(StrEnum):
    MALE = "male"
    FEMALE = "female"


# ============================================================
# 语言
# ============================================================

class Language(StrEnum):
    ZH_CN = "zh_cn"
    ZH_TW = "zh_tw"
    EN_US = "en_us"
    JA_JP = "ja_jp"
    KO_KR = "ko_kr"
    VI_VN = "vi_vn"


# ============================================================
# 算法
# ============================================================

class Algorithm(StrEnum):
    DEFAULT = "default"
    ZHONGZHOU = "zhongzhou"


# ============================================================
# 天干
# ============================================================

class HeavenlyStem(StrEnum):
    JIA = "甲"
    YI = "乙"
    BING = "丙"
    DING = "丁"
    WU = "戊"
    JI = "己"
    GENG = "庚"
    XIN = "辛"
    REN = "壬"
    GUI = "癸"


# ============================================================
# 地支
# ============================================================

class EarthlyBranch(StrEnum):
    ZI = "子"
    CHOU = "丑"
    YIN = "寅"
    MAO = "卯"
    CHEN = "辰"
    SI = "巳"
    WU = "午"
    WEI = "未"
    SHEN = "申"
    YOU = "酉"
    XU = "戌"
    HAI = "亥"


# ============================================================
# 十二宫
# ============================================================

class PalaceName(StrEnum):
    SOUL = "命宫"         # 命宫
    SIBLINGS = "兄弟"     # 兄弟
    SPOUSE = "夫妻"       # 夫妻
    CHILDREN = "子女"     # 子女
    WEALTH = "财帛"       # 财帛
    HEALTH = "疾厄"       # 疾厄
    SURFACE = "迁移"      # 迁移
    FRIENDS = "交友"      # 交友（又名仆役）
    CAREER = "官禄"       # 官禄
    PROPERTY = "田宅"     # 田宅
    SPIRIT = "福德"       # 福德
    PARENTS = "父母"      # 父母


# ============================================================
# 五行局
# ============================================================

class FiveElementsClass(StrEnum):
    WATER_2 = "水二局"
    WOOD_3 = "木三局"
    METAL_4 = "金四局"
    EARTH_5 = "土五局"
    FIRE_6 = "火六局"


# ============================================================
# 四化
# ============================================================

class Mutagen(StrEnum):
    LU = "禄"    # 化禄
    QUAN = "权"  # 化权
    KE = "科"    # 化科
    JI = "忌"    # 化忌


# ============================================================
# 亮度
# ============================================================

class Brightness(StrEnum):
    MIAO = "庙"   # 庙
    WANG = "旺"   # 旺
    DE = "得"     # 得
    LI = "利"     # 利
    PING = "平"   # 平
    BU = "不"     # 不
    XIAN = "陷"   # 陷


# ============================================================
# 星耀类型
# ============================================================

class StarType(StrEnum):
    MAJOR = "major"         # 主星
    SOFT = "soft"           # 吉星
    TOUGH = "tough"         # 煞星
    ADJECTIVE = "adjective" # 杂耀
    FLOWER = "flower"       # 桃花星
    HELPER = "helper"       # 解神
    LUCUN = "lucun"         # 禄存
    TIANMA = "tianma"       # 天马


# ============================================================
# 作用范围
# ============================================================

class Scope(StrEnum):
    ORIGIN = "origin"     # 本命
    DECADAL = "decadal"   # 大限
    YEARLY = "yearly"     # 流年
    MONTHLY = "monthly"   # 流月
    DAILY = "daily"       # 流日
    HOURLY = "hourly"     # 流时


# ============================================================
# 主星名（14 颗）
# ============================================================

class MajorStar(StrEnum):
    ZIWEI = "紫微"
    TIANJI = "天机"
    TAIYANG = "太阳"
    WUQU = "武曲"
    TIANTONG = "天同"
    LIANZHEN = "廉贞"
    TIANFU = "天府"
    TAIYIN = "太阴"
    TANLANG = "贪狼"
    JUMEN = "巨门"
    TIANXIANG = "天相"
    TIANLIANG = "天梁"
    QISHA = "七杀"
    POJUN = "破军"


# ============================================================
# 辅星名（14 颗）
# ============================================================

class MinorStar(StrEnum):
    ZUOFU = "左辅"
    YOUBI = "右弼"
    WENCHANG = "文昌"
    WENQU = "文曲"
    LUCUN = "禄存"
    TIANMA = "天马"
    QINGYANG = "擎羊"
    TUOLUO = "陀罗"
    HUOXING = "火星"
    LINGXING = "铃星"
    TIANKUI = "天魁"
    TIANYUE = "天钺"
    DIKONG = "地空"
    DIJIE = "地劫"


# ============================================================
# 杂耀名
# ============================================================

class AdjectiveStar(StrEnum):
    JIESHA = "劫煞"
    TIANKONG = "天空"
    TIANXING = "天刑"
    TIANYAO = "天姚"
    JIESHEN = "解神"
    YINSHA = "阴煞"
    TIANXI = "天喜"
    TIANGUAN = "天官"
    TIANFU_ADJ = "天福"
    TIANKU = "天哭"
    TIANXU = "天虚"
    LONGCHI = "龙池"
    FENGGE = "凤阁"
    HONGLUAN = "红鸾"
    GUCHEN = "孤辰"
    GUASU = "寡宿"
    FEILIAN = "蜚廉"
    POSUI = "破碎"
    TAIFU = "台辅"
    FENGGAO = "封诰"
    TIANWU = "天巫"
    TIANYUE2 = "天月"
    SANTAI = "三台"
    BAZUO = "八座"
    ENGGUANG = "恩光"
    TIANGUI = "天贵"
    TIANCAI = "天才"
    TIANSHOU = "天寿"
    JIEKONG = "截空"
    XUNZHONG = "旬中"
    XUNKONG = "旬空"
    KONGWANG = "空亡"
    JIELU = "截路"
    YUEDE = "月德"
    TIANSHANG = "天伤"
    TIANSHI = "天使"
    TIANCHU = "天厨"


# ============================================================
# 长生十二神
# ============================================================

class Changsheng12(StrEnum):
    CHANGSHENG = "长生"
    MUYU = "沐浴"
    GUANDAI = "冠带"
    LINGUAN = "临官"
    DIWANG = "帝旺"
    SHUAI = "衰"
    BING = "病"
    SI = "死"
    MU = "墓"
    JUE = "绝"
    TAI = "胎"
    YANG = "养"


# ============================================================
# 博士十二神
# ============================================================

class Boshi12(StrEnum):
    BOSHI = "博士"
    LISHI = "力士"
    QINGLONG = "青龙"
    XIAOHAO = "小耗"
    JIANGJUN = "将军"
    ZHOUSHU = "奏书"
    FEILIAN = "飞廉"
    XISHEN = "喜神"
    BINGFU = "病符"
    DAHAO = "大耗"
    SUIPO = "伏兵"
    FUBING = "官府"
    GUANFU = "官符"


# ============================================================
# 岁前十二神
# ============================================================

class Suiqian12(StrEnum):
    SUIJIAN = "岁建"
    HUIQI = "晦气"
    SANGMEN = "丧门"
    GUANSUO = "贯索"
    GWANFU = "官符"
    LONGDE = "龙德"
    BAIHU = "白虎"
    TIANDE = "天德"
    DIAOKE = "吊客"
    BINGFU = "病符"
    XIAOHAO = "小耗"
    DAHAO = "大耗"


# ============================================================
# 将前十二神
# ============================================================

class Jiangqian12(StrEnum):
    JIANGXING = "将星"
    PANAN = "攀鞍"
    SUIYI = "岁驿"
    XISHEN = "息神"
    HUAGAI = "华盖"
    JIESHA = "劫煞"
    ZHAISHA = "灾煞"
    TIANSHA = "天煞"
    ZHIBEI = "指背"
    XIANCHI = "咸池"
    YUESHA = "月煞"
    WANGSHEN = "亡神"

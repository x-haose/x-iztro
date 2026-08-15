"""对齐 iztro 的对象方法测试。

断言值与 Rust / Go 侧同一张盘（2000-8-16 寅时女命）的结果一致，
三侧任一处行为漂移都会在这里暴露。
"""

from __future__ import annotations

import pytest

from x_iztro import Astro, Astrolabe, ChartConfig
from x_iztro.enums import (
    AstroType,
    EarthlyBranch,
    HeavenlyStem,
    MajorStar,
    Mutagen,
    PalaceName,
)


@pytest.fixture(scope="module")
def chart():
    return Astro().by_solar("2000-8-16", 2, "female")


def test_star_back_references(chart):
    star = chart.star(MajorStar.ZIWEI)

    assert star.palace().name_key == PalaceName.SOUL
    assert star.opposite_palace().index == (star.palace().index + 6) % 12
    assert star.surrounded_palaces().target.index == star.palace().index


def test_palace_back_reference(chart):
    assert chart.palace(PalaceName.SOUL).astrolabe() is chart


def test_locate_body_and_original_palace(chart):
    body = chart.palace(PalaceName.BODY)
    assert body.is_body_palace
    assert body.name_key == PalaceName.CAREER

    original = chart.palace(PalaceName.ORIGINAL)
    assert original.is_original_palace
    assert original.name_key == PalaceName.SPOUSE


def test_mutagen_table_of_soul_palace(chart):
    """命宫壬午，壬干四化依次为天梁、紫微、左辅、武曲。"""
    soul = chart.palace(PalaceName.SOUL)

    assert soul.mutagen_stars(list(Mutagen)) == [
        "tianliangMaj",
        "ziweiMaj",
        "zuofuMin",
        "wuquMaj",
    ]


def test_self_mutagen(chart):
    """紫微坐命宫，而壬干化权为紫微，故命宫自化权。"""
    soul = chart.palace(PalaceName.SOUL)

    assert soul.self_mutaged(Mutagen.QUAN)
    assert soul.self_mutaged_one_of()
    assert not soul.not_self_mutaged()


def test_empty_mutagen_list_semantics(chart):
    """复刻 iztro：flies_to 空列表为假，flies_one_of_to / not_fly_to 为真。"""
    soul = chart.palace(PalaceName.SOUL)

    assert soul.flies_to(0, []) is False
    assert soul.flies_one_of_to(0, []) is True
    assert soul.not_fly_to(0, []) is True


def test_flies_to_accepts_palace_index_and_name(chart):
    soul = chart.palace(PalaceName.SOUL)
    target = chart.palaces[0]

    by_object = soul.flies_to(target, Mutagen.LU)
    by_index = soul.flies_to(0, Mutagen.LU)
    by_name = soul.flies_to(target.name_key, Mutagen.LU)

    assert by_object == by_index == by_name


def test_mutaged_places_without_argument(chart):
    soul = chart.palace(PalaceName.SOUL)
    places = soul.mutaged_places()

    assert len(places) == 4
    assert places == soul.mutaged_places(chart.palaces)
    # 壬干化权为紫微，紫微坐命宫
    assert places[1].name_key == PalaceName.SOUL


def test_is_empty_with_exclude_stars(chart):
    empty = next(p for p in chart.palaces if p.is_empty())
    assert empty.minor_stars, "该盘的空宫应仍有辅星，否则本用例失去意义"

    assert empty.is_empty() is True
    assert empty.is_empty([empty.minor_stars[0].key]) is False


def test_surrounded_palaces_by_name_and_predicates(chart):
    sp = chart.surrounded_palaces(PalaceName.SOUL)
    assert sp.target.name_key == PalaceName.SOUL

    first = sp.target.major_stars[0].key
    assert chart.is_surrounded(PalaceName.SOUL, [first]) is True
    assert chart.is_surrounded_one_of(PalaceName.SOUL, [first]) is True
    assert chart.not_surrounded(PalaceName.SOUL, [first]) is False


def test_horoscope_star_predicates_are_consistent(chart):
    h = chart.horoscope("2024-10-1", 0)
    probe = ["流禄"]

    has = h.has_horoscope_stars(PalaceName.SOUL, "yearly", probe)
    one = h.has_one_of_horoscope_stars(PalaceName.SOUL, "yearly", probe)
    not_have = h.not_have_horoscope_stars(PalaceName.SOUL, "yearly", probe)

    assert has == one
    assert has != not_have


def test_horoscope_holds_astrolabe(chart):
    h = chart.horoscope("2024-10-1", 0)

    # 运限持有发起它的那张盘
    assert h.astrolabe() is chart
    # 免传星盘与显式传盘的结果一致
    assert h.palace(PalaceName.SOUL, "yearly") is h.palace(
        PalaceName.SOUL, "yearly", chart
    )
    assert h.age_palace() is h.age_palace(chart)
    assert (
        h.surround_palaces(PalaceName.SOUL, "yearly").target.index
        == h.palace(PalaceName.SOUL, "yearly").index
    )
    # 顶层入口返回的运限同样持有传入的盘
    assert Astro().get_horoscope(chart, "2024-10-1", 0).astrolabe() is chart


# ============================================================
# 轻量查询（阶段 2）
# ============================================================


def test_zodiac_and_sign_match_full_chart(chart):
    from x_iztro import get_sign_by_solar_date, get_zodiac_by_solar_date

    assert get_zodiac_by_solar_date("2000-8-16") == chart.zodiac
    assert get_sign_by_solar_date("2000-8-16") == chart.sign


def test_sign_by_lunar_equals_solar():
    from x_iztro import get_sign_by_lunar_date, get_sign_by_solar_date

    # 农历 2000-7-17 即公历 2000-8-16
    assert get_sign_by_lunar_date("2000-7-17") == get_sign_by_solar_date("2000-8-16")


def test_major_star_of_soul_palace():
    from x_iztro import get_major_star_by_lunar_date, get_major_star_by_solar_date

    assert get_major_star_by_solar_date("2000-8-16", 2) == "紫微"
    assert get_major_star_by_lunar_date("2000-7-17", 2) == "紫微"


def test_query_respects_language():
    """与 Go 侧 TestQueryParity 断言同一组取值。"""
    from x_iztro import (
        get_major_star_by_solar_date,
        get_sign_by_solar_date,
        get_zodiac_by_solar_date,
    )

    assert get_zodiac_by_solar_date("2000-8-16", "en-US") == "dragon"
    assert get_sign_by_solar_date("2000-8-16", "en-US") == "leo"
    assert get_major_star_by_solar_date("2000-8-16", 2, language="en-US") == "emperor"


def test_query_rejects_invalid_input():
    from x_iztro import get_major_star_by_solar_date, get_zodiac_by_solar_date

    with pytest.raises(ValueError):
        get_zodiac_by_solar_date("2000-2-30")
    with pytest.raises(ValueError):
        get_major_star_by_solar_date("2000-8-16", 13)


# ============================================================
# 工具函数（阶段 2）
# ============================================================

# 2000 年为庚辰年；四柱在 raw_dates 里是中文原文，这里直接用 key 常量
YEAR_STEM = HeavenlyStem.GENG


def test_index_helpers():
    from x_iztro.utils import (
        earthly_branch_to_palace_index,
        fix_index,
        fix_lunar_day_index,
        time_to_index,
    )

    assert fix_index(-1) == 11
    assert fix_index(12) == 0
    assert fix_index(5, 6) == 5
    assert time_to_index(0) == 0
    assert time_to_index(3) == 2
    assert time_to_index(23) == 12
    assert earthly_branch_to_palace_index(EarthlyBranch.YIN) == 0
    # 晚子时属次日，日索引不减一
    assert fix_lunar_day_index(15, 12) == 15


def test_five_elements_class_matches_chart(chart):
    from x_iztro.utils import get_five_elements_class

    soul = chart.palace(PalaceName.SOUL)
    assert (
        get_five_elements_class(soul.heavenly_stem_key, soul.earthly_branch_key)
        == chart.five_elements_class_key
    )


def test_palace_names_match_chart(chart):
    from x_iztro.utils import get_palace_names

    soul = chart.palace(PalaceName.SOUL)
    assert get_palace_names(soul.index) == [p.name_key for p in chart.palaces]


def test_soul_and_body_matches_chart(chart):
    from x_iztro.utils import fix_lunar_month_index, get_soul_and_body

    month_index = fix_lunar_month_index(
        chart.raw_dates.lunar_date.lunar_month,
        chart.raw_dates.lunar_date.lunar_day,
        chart.raw_dates.lunar_date.is_leap,
        chart.time_index,
        chart.fix_leap,
    )
    result = get_soul_and_body(month_index, chart.time_index, YEAR_STEM)

    assert result["soulIndex"] == chart.palace(PalaceName.SOUL).index
    assert result["bodyIndex"] == chart.palace(PalaceName.BODY).index
    assert result["earthlyBranchOfSoul"] == chart.earthly_branch_of_soul_palace_key


def test_brightness_and_mutagen_lookup_match_chart(chart):
    """查表结果必须与排盘写入每颗星的字段一致。"""
    from x_iztro.utils import get_brightness, get_mutagen

    checked = 0
    for palace in chart.palaces:
        for star in palace.major_stars + palace.minor_stars:
            assert get_brightness(star.key, palace.index) == star.brightness_key
            assert get_mutagen(star.key, YEAR_STEM) == star.mutagen_key
            checked += 1
    assert checked >= 20


def test_mutagens_by_heavenly_stem():
    """与 Go 侧 TestUtilParity 断言同一组取值。"""
    from x_iztro.utils import get_mutagens_by_heavenly_stem

    assert get_mutagens_by_heavenly_stem(HeavenlyStem.REN) == [
        "tianliangMaj",
        "ziweiMaj",
        "zuofuMin",
        "wuquMaj",
    ]


def test_utils_reject_unknown_keys():
    from x_iztro.utils import get_brightness, get_five_elements_class, time_to_index

    with pytest.raises(ValueError):
        get_brightness("nope", 0)
    with pytest.raises(ValueError):
        get_five_elements_class("x", "y")
    with pytest.raises(ValueError):
        time_to_index(24)


# ============================================================
# 自定义四化与亮度表（阶段 3）
# ============================================================

# iztro 文档给出的另一派庚干四化：太阳、武曲、天同、天相
CUSTOM_GENG = {
    HeavenlyStem.GENG: [
        MajorStar.TAIYANG,
        MajorStar.WUQU,
        MajorStar.TIANTONG,
        MajorStar.TIANXIANG,
    ]
}


def _stars_with_mutagen(chart, mutagen):
    return [
        s.key
        for p in chart.palaces
        for s in p.major_stars + p.minor_stars
        if s.mutagen_key == mutagen
    ]


def test_custom_mutagens_change_natal_chart():
    """与 Rust / Go 侧断言同一组结果：天同接替化科、天相接替化忌。"""
    from x_iztro import Astro, ChartConfig

    chart = Astro().by_solar(
        "2000-8-16", 2, "female", config=ChartConfig(mutagens=CUSTOM_GENG)
    )

    assert _stars_with_mutagen(chart, Mutagen.KE) == [MajorStar.TIANTONG]
    assert _stars_with_mutagen(chart, Mutagen.JI) == [MajorStar.TIANXIANG]
    # 未改动的禄权保持原状
    assert _stars_with_mutagen(chart, Mutagen.LU) == [MajorStar.TAIYANG]


def test_default_chart_keeps_builtin_mutagens(chart):
    """同一张盘在默认配置下，庚干化科仍是太阴。"""
    assert _stars_with_mutagen(chart, Mutagen.KE) == [MajorStar.TAIYIN]


def test_custom_mutagens_do_not_leak_to_other_stems():
    from x_iztro import ChartConfig
    from x_iztro.utils import get_mutagens_by_heavenly_stem

    config = ChartConfig(mutagens=CUSTOM_GENG)

    assert get_mutagens_by_heavenly_stem(HeavenlyStem.GENG, config)[3] == MajorStar.TIANXIANG
    # 壬干不受影响
    assert get_mutagens_by_heavenly_stem(HeavenlyStem.REN, config) == [
        "tianliangMaj",
        "ziweiMaj",
        "zuofuMin",
        "wuquMaj",
    ]


def test_custom_brightness_applies():
    from x_iztro import Astro, ChartConfig
    from x_iztro.enums import Brightness
    from x_iztro.utils import get_brightness

    config = ChartConfig(brightness={MajorStar.TANLANG: [Brightness.WANG] * 12})
    chart = Astro().by_solar("2000-8-16", 2, "female", config=config)

    assert chart.star(MajorStar.TANLANG).brightness_key == Brightness.WANG
    assert get_brightness(MajorStar.TANLANG, 5, config) == Brightness.WANG


def test_invalid_override_tables_are_rejected():
    from x_iztro import Astro, ChartConfig

    astro = Astro()
    bad_configs = [
        ChartConfig(mutagens={"nope": ["a", "b", "c", "d"]}),
        ChartConfig(mutagens={HeavenlyStem.GENG: [MajorStar.TAIYANG]}),
        ChartConfig(brightness={MajorStar.TANLANG: ["wang"]}),
    ]
    for config in bad_configs:
        with pytest.raises(ValueError):
            astro.by_solar("2000-8-16", 2, "female", config=config)


# ============================================================
# 天盘 / 地盘 / 人盘（阶段 4）
# ============================================================


def _chart(astro_type: str) -> Astrolabe:
    return Astro().by_solar(
        "2000-8-16", 2, "female", config=ChartConfig(astro_type=astro_type)
    )


def test_earth_chart_takes_body_palace_as_soul(chart):
    earth = _chart(AstroType.EARTH)
    body = chart.palace(PalaceName.BODY)

    assert earth.palace(PalaceName.SOUL).index == body.index
    assert earth.config.astro_type == AstroType.EARTH


def test_human_chart_takes_spirit_palace_as_soul(chart):
    human = _chart(AstroType.HUMAN)
    spirit = chart.palace(PalaceName.SPIRIT)

    assert human.palace(PalaceName.SOUL).index == spirit.index


def test_rearranged_from_body_palace_equals_earth_chart(chart):
    body = chart.palace(PalaceName.BODY)
    manual = chart.rearranged(body.heavenly_stem_key, body.earthly_branch_key)
    earth = _chart(AstroType.EARTH)

    assert manual.five_elements_class_key == earth.five_elements_class_key
    assert [p.name_key for p in manual.palaces] == [p.name_key for p in earth.palaces]
    assert [
        [s.key for s in p.major_stars] for p in manual.palaces
    ] == [[s.key for s in p.major_stars] for p in earth.palaces]


def test_rearranged_rejects_unknown_stem(chart):
    with pytest.raises(ValueError):
        chart.rearranged("notAStem", EarthlyBranch.ZI)


# ============================================================
# 工具函数收尾与 horoscope 默认参数（阶段 5）
# ============================================================


def test_translate_chinese_date_matches_chart_field(chart):
    from x_iztro.utils import translate_chinese_date

    pillars = chart.raw_dates.chinese_date.pillar_keys()
    assert translate_chinese_date(pillars) == chart.chinese_date
    # 词条为多字符时改用「 - 」分隔柱
    assert (
        translate_chinese_date(pillars, "en-US")
        == "geng chen - jia shen - bing woo - geng yin"
    )


def test_translate_chinese_date_rejects_bad_input(chart):
    from x_iztro.utils import translate_chinese_date

    pillars = chart.raw_dates.chinese_date.pillar_keys()
    with pytest.raises(ValueError):
        translate_chinese_date(pillars[:3])
    with pytest.raises(ValueError):
        translate_chinese_date([("notAStem", "ziEarthly")] + pillars[1:])


def test_merge_stars_concatenates_by_palace(chart):
    from x_iztro.utils import merge_stars

    majors = [p.major_stars for p in chart.palaces]
    minors = [p.minor_stars for p in chart.palaces]
    merged = merge_stars(majors, minors)

    assert len(merged) == 12
    for i in range(12):
        assert len(merged[i]) == len(majors[i]) + len(minors[i])
        # 顺序为传入顺序：主星在前，辅星在后
        assert merged[i][: len(majors[i])] == majors[i]

    assert merge_stars() == [[] for _ in range(12)]
    with pytest.raises(ValueError):
        merge_stars(majors[:11])


def test_horoscope_defaults_to_now(chart):
    """两个参数都可省略，取本地时钟的当前日期与时辰。"""
    from datetime import datetime

    from x_iztro.utils import time_to_index

    now = datetime.now()
    explicit = chart.horoscope(
        f"{now.year}-{now.month}-{now.day}", time_to_index(now.hour)
    )
    by_now = chart.horoscope()

    assert by_now.solar_date == explicit.solar_date
    assert by_now.decadal.index == explicit.decadal.index
    assert by_now.hourly.index == explicit.hourly.index


# ---------------------------------------------------------------------------
# star / data / i18n 三个模块：与 Go 侧 parity_check_test.go 断言同一组取值
# ---------------------------------------------------------------------------


def test_star_module_matches_chart():
    """单独安星的结果必须与整盘对应字段一致。"""
    from x_iztro import star

    birth = dict(solar_date="2000-8-16", time_index=2, gender="female", fix_leap=True)

    start = star.get_start_index(**birth)
    assert start["ziweiIndex"] == 4
    assert start["tianfuIndex"] == 8

    chart = Astro().by_solar("2000-8-16", 2, "female")

    major = star.get_major_star(**birth)
    for i, palace in enumerate(chart.palaces):
        assert [s.key for s in palace.major_stars] == [s.key for s in major[i]]

    minor = star.get_minor_star(**birth)
    for i, palace in enumerate(chart.palaces):
        assert [s.key for s in palace.minor_stars] == [s.key for s in minor[i]]

    cs12 = star.get_changsheng12(**birth)
    assert [p.changsheng12_key for p in chart.palaces] == cs12

    boshi12 = star.get_boshi12(**birth)
    assert [p.boshi12_key for p in chart.palaces] == boshi12

    yearly12 = star.get_yearly12(**birth)
    assert [p.suiqian12_key for p in chart.palaces] == yearly12["suiqian12"]
    assert [p.jiangqian12_key for p in chart.palaces] == yearly12["jiangqian12"]


def test_horoscope_star_uses_correct_hongluan_key():
    """iztro 在本命层级把红鸾误写为 hongluanMin，x-iztro 安放正确标识。"""
    from x_iztro import star

    origin = star.get_horoscope_star("jiaHeavenly", "ziEarthly", "origin")
    assert origin[1][1].key == "hongluan"

    decadal = star.get_horoscope_star("jiaHeavenly", "ziEarthly", "decadal")
    assert decadal[1][1].key == "yunluan"


def test_star_start_indexes():
    from x_iztro import star

    assert star.get_changsheng12_start_index("water2nd") == 6
    assert star.get_jiangqian12_start_index("ziEarthly") == 10


def test_location_indices_match_yearly_composite():
    """低层落宫函数与年系杂耀的合并结果必须一致。"""
    from x_iztro import star

    birth = dict(solar_date="2000-8-16", time_index=2, gender="female", fix_leap=True)
    chart = Astro().by_solar("2000-8-16", 2, "female")
    year_branch = chart.raw_dates.chinese_date.yearly_keys[1]
    soul_index = next(p.index for p in chart.palaces if p.name_key == PalaceName.SOUL)

    yearly = star.get_yearly_star_index(**birth)

    assert star.get_huagai_xianchi_index(year_branch) == {
        "huagaiIndex": yearly["huagaiIndex"],
        "xianchiIndex": yearly["xianchiIndex"],
    }
    assert star.get_gu_gua_index(year_branch) == {
        "guchenIndex": yearly["guchenIndex"],
        "guasuIndex": yearly["guasuIndex"],
    }
    assert star.get_jiesha_adj_index(year_branch) == yearly["jieshaAdjIndex"]
    assert star.get_dahao_index(year_branch) == yearly["dahaoAdjIndex"]
    assert star.get_nianjie_index(year_branch) == yearly["nianjieIndex"]
    assert star.get_tianshi_tianshang_index("female", year_branch, soul_index) == {
        "tianshangIndex": yearly["tianshangIndex"],
        "tianshiIndex": yearly["tianshiIndex"],
    }


def test_location_indices_match_chart_star_positions():
    """左辅右弼、火星铃星、按天干的昌曲必须落在整盘对应的宫位上。"""
    from x_iztro import star
    from x_iztro.utils import fix_lunar_month_index

    chart = Astro().by_solar("2000-8-16", 2, "female")
    year_branch = chart.raw_dates.chinese_date.yearly_keys[1]

    def palace_of(star_key: str) -> int:
        return next(
            p.index
            for p in chart.palaces
            for s in (*p.major_stars, *p.minor_stars, *p.adjective_stars)
            if s.key == star_key
        )

    lunar = chart.raw_dates.lunar_date
    lunar_month = (
        fix_lunar_month_index(lunar.lunar_month, lunar.lunar_day, lunar.is_leap, 2, True) + 1
    )
    zuo_you = star.get_zuo_you_index(lunar_month)
    assert zuo_you["zuoIndex"] == palace_of("zuofuMin")
    assert zuo_you["youIndex"] == palace_of("youbiMin")

    huo_ling = star.get_huo_ling_index(year_branch, 2)
    assert huo_ling["huoIndex"] == palace_of("huoxingMin")
    assert huo_ling["lingIndex"] == palace_of("lingxingMin")

    # 按天干的昌曲用于运限层级：与流耀分布里的流昌流曲同宫
    decadal = star.get_horoscope_star("jiaHeavenly", "ziEarthly", "decadal")
    chang_qu = star.get_chang_qu_index_by_heavenly_stem("jiaHeavenly")
    assert any(s.key == "yunchang" for s in decadal[chang_qu["changIndex"]])
    assert any(s.key == "yunqu" for s in decadal[chang_qu["quIndex"]])


def test_key_of_filter_disambiguates_homonyms():
    """限定标识名后的反查对应 iztro kot 的第二个参数。"""
    from x_iztro.i18n import key_of

    assert key_of("horse") == "horse"
    assert key_of("horse", "Min") == "tianmaMin"
    assert key_of("dragon") == "dragon"
    assert key_of("유시") == "hourly"
    assert key_of("유시", "Hour") == "roosterHour"
    assert key_of("horse", "Palace") is None


def test_all_keys_covers_every_category():
    """标识表的规模、次序与逐条可译性。"""
    from x_iztro.i18n import all_keys, translate

    keys = all_keys()
    assert len(keys) == 260
    assert len(set(keys)) == 260
    # 次序即反查次序：common.json 打头、性别收尾
    assert keys[:2] == ["decadal", "childhood"]
    assert keys[-2:] == ["male", "female"]
    assert all(translate(k) is not None for k in keys)


def test_data_tables():
    from x_iztro import data

    info = data.stars_info()
    assert len(info) == 20
    assert info["ziweiMaj"]["fiveElements"] == "土"
    assert info["ziweiMaj"]["yinYang"] == "阴"
    assert info["ziweiMaj"]["brightness"][0] == "wang"
    # 太阳的五行与阴阳在原表中未填
    assert info["taiyangMaj"]["fiveElements"] is None
    assert info["taiyangMaj"]["yinYang"] is None

    stems = data.heavenly_stems()
    jia = stems["jiaHeavenly"]
    assert (jia["yinYang"], jia["fiveElements"], jia["crash"]) == ("阳", "木", "gengHeavenly")
    assert jia["mutagen"] == ["lianzhenMaj", "pojunMaj", "wuquMaj", "taiyangMaj"]
    # 戊己无对冲天干
    assert stems["wuHeavenly"]["crash"] is None

    branches = data.earthly_branches()
    zi = branches["ziEarthly"]
    assert zi["crash"] == "wuEarthly"
    assert zi["soul"] == "tanlangMaj"
    assert zi["inside"] == "胆"

    c = data.constants()
    assert c["LANGUAGES"] == ["en-US", "ja-JP", "ko-KR", "zh-CN", "zh-TW", "vi-VN"]
    assert len(c["CHINESE_TIME"]) == 13
    assert c["CHINESE_TIME"][12] == "lateRatHour"
    assert c["GENDER"] == {"male": "阳", "female": "阴"}
    assert c["TIGER_RULE"]["jiaHeavenly"] == "bingHeavenly"
    assert c["RAT_RULE"]["jiaHeavenly"] == "jiaHeavenly"


def test_i18n_round_trip():
    from x_iztro import i18n

    for lang, want in [("zh-CN", "紫微"), ("en-US", "emperor"), ("ko-KR", "자미")]:
        assert i18n.translate("ziweiMaj", lang) == want
        assert i18n.key_of(want) == "ziweiMaj"

    # 非星耀类目同样可查
    assert i18n.translate("soulPalace", "en-US") == "soul"
    assert i18n.translate("bodyPalace") == "身宫"
    assert i18n.translate("nosuchkey") is None
    assert i18n.key_of("查无此名") is None


def test_decadals_and_ages_match_chart():
    from x_iztro.utils import get_decadals_and_ages

    chart = Astro().by_solar("2000-8-16", 2, "female")
    soul_index = next(p.index for p in chart.palaces if p.name_key == "soulPalace")

    got = get_decadals_and_ages(
        soul_index,
        chart.five_elements_class_key,
        chart.gender_key,
        chart.raw_dates.chinese_date.yearly_keys[0],
        chart.raw_dates.chinese_date.yearly_keys[1],
    )
    for i, palace in enumerate(chart.palaces):
        assert list(palace.decadal.range) == got["decadals"][i]["range"]
        assert palace.ages == got["ages"][i]
        # 译名与标识两组字段的含义在整盘与单取之间必须一致
        assert palace.decadal.heavenly_stem == got["decadals"][i]["heavenlyStem"]
        assert palace.decadal.heavenly_stem_key == got["decadals"][i]["heavenlyStemKey"]
        assert palace.decadal.earthly_branch == got["decadals"][i]["earthlyBranch"]
        assert palace.decadal.earthly_branch_key == got["decadals"][i]["earthlyBranchKey"]


def test_palace_back_references_to_opposite_and_surrounded():
    """宫位自身可取对宫与三方四正，结果与从星盘入口取的一致。"""
    chart = Astro().by_solar("2000-8-16", 2, "female")
    soul = chart.palace("soulPalace")

    assert soul.opposite_palace().name_key == "surfacePalace"
    assert soul.opposite_palace().index == (soul.index + 6) % 12

    from_palace = soul.surrounded_palaces()
    from_chart = chart.surrounded_palaces("soulPalace")
    assert [p.index for p in (from_palace.target, from_palace.opposite, from_palace.wealth, from_palace.career)] == [
        p.index for p in (from_chart.target, from_chart.opposite, from_chart.wealth, from_chart.career)
    ]

    # 十二宫的对宫互为对方
    for p in chart.palaces:
        assert p.opposite_palace().opposite_palace().index == p.index


def test_config_enums_cover_all_switches():
    """六组配置开关的枚举取值，与 Go 侧 TestConfigConstantsParity 断言同一组。"""
    from x_iztro import (
        AgeDivide,
        Algorithm,
        AstroType,
        DayDivide,
        HoroscopeDivide,
        YearDivide,
    )

    assert (YearDivide.NORMAL, YearDivide.EXACT) == ("normal", "exact")
    assert (HoroscopeDivide.NORMAL, HoroscopeDivide.EXACT) == ("normal", "exact")
    assert (AgeDivide.NORMAL, AgeDivide.BIRTHDAY) == ("normal", "birthday")
    assert (DayDivide.FORWARD, DayDivide.CURRENT) == ("forward", "current")
    assert (Algorithm.DEFAULT, Algorithm.ZHONGZHOU) == ("default", "zhongzhou")
    assert (AstroType.HEAVEN, AstroType.EARTH, AstroType.HUMAN) == ("heaven", "earth", "human")

    zz = Astro().by_solar(
        "2000-8-16", 2, "female",
        config=ChartConfig(algorithm=Algorithm.ZHONGZHOU, year_divide=YearDivide.EXACT),
    )
    assert zz.five_elements_class

    late = Astro().by_solar(
        "2000-8-16", 12, "female", config=ChartConfig(day_divide=DayDivide.CURRENT)
    )
    assert late.time

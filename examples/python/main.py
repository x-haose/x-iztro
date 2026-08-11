#!/usr/bin/env python3
"""
rs-iztro Python 示例

前置步骤：
    cd rs-iztro
    pip install maturin
    PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

运行方式：
    python examples/python/main.py
"""

from rs_iztro import Astro, Astrolabe, Palace, Star


def main():
    astro = Astro()

    # ============================================================
    # 1. 阳历排盘 — 返回 Astrolabe dataclass
    # ============================================================
    print("===== 1. 阳历排盘 =====\n")

    result = astro.by_solar("2000-8-16", 2, "female")

    # 属性访问，IDE 自动补全
    print(f"阳历：{result.solar_date}")
    print(f"农历：{result.lunar_date}")
    print(f"干支：{result.chinese_date}")
    print(f"时辰：{result.time} ({result.time_range})")
    print(f"星座：{result.sign}")
    print(f"生肖：{result.zodiac}")
    print(f"命主：{result.soul}")
    print(f"身主：{result.body}")
    print(f"五行局：{result.five_elements_class}")
    print()

    # ============================================================
    # 2. 农历排盘
    # ============================================================
    print("===== 2. 农历排盘 =====\n")

    lunar = astro.by_lunar("2000-7-17", 2, "female")
    print(f"阳历：{lunar.solar_date}")
    print(f"农历：{lunar.lunar_date}")
    print()

    # ============================================================
    # 3. 十二宫 — Palace dataclass
    # ============================================================
    print("===== 3. 十二宫概览 =====\n")

    for palace in result.palaces:
        names = [s.name for s in palace.major_stars]
        body = " [身]" if palace.is_body_palace else ""
        stars = "、".join(names) if names else "（空宫）"
        print(f"  [{palace.index:>2}] {palace.name}{body} "
              f"({palace.heavenly_stem}{palace.earthly_branch}) - {stars}")
    print()

    # ============================================================
    # 4. 命宫详情 — 嵌套 dataclass
    # ============================================================
    print("===== 4. 命宫星耀 =====\n")

    soul = result.palace("命宫")
    if soul:
        print(f"天干地支：{soul.heavenly_stem}{soul.earthly_branch}")
        print(f"大限：{soul.decadal.range[0]}-{soul.decadal.range[1]} 岁"
              f" ({soul.decadal.heavenly_stem}{soul.decadal.earthly_branch})")

        for label, stars in [
            ("主星", soul.major_stars),
            ("辅星", soul.minor_stars),
        ]:
            print(f"\n{label}：")
            for s in stars:
                info = f"  {s.name}"
                if s.brightness:
                    info += f" ({s.brightness})"
                if s.mutagen:
                    info += f" [{s.mutagen}]"
                print(info)
    print()

    # ============================================================
    # 5. 宫位方法
    # ============================================================
    print("===== 5. 宫位判断 =====\n")

    if soul:
        print(f"有紫微：{soul.has(['紫微'])}")
        print(f"有紫微或天府：{soul.has_one_of(['紫微', '天府'])}")
        print(f"空宫：{soul.is_empty()}")
        print(f"化禄：{soul.has_mutagen('禄')}")
        print(f"化忌：{soul.has_mutagen('忌')}")
    print()

    # ============================================================
    # 6. 查找星耀
    # ============================================================
    print("===== 6. 查找星耀 =====\n")

    for name in ["紫微", "天机", "太阳", "太阴", "禄存"]:
        s = result.star(name)
        if s:
            info = f"  {s.name}"
            if s.brightness:
                info += f" ({s.brightness})"
            if s.mutagen:
                info += f" [{s.mutagen}]"
            # Star 是 frozen dataclass，类型安全
            print(f"{info}  type={s.type} scope={s.scope}")
    print()

    # ============================================================
    # 7. 运限 — Horoscope dataclass
    # ============================================================
    print("===== 7. 运限 =====\n")

    h = astro.get_horoscope(result, "2024-10-1", 0)

    print(f"日期：{h.solar_date} / {h.lunar_date}")
    print(f"\n大限：{h.decadal.name} ({h.decadal.heavenly_stem}{h.decadal.earthly_branch})")
    print(f"  四化：{'、'.join(h.decadal.mutagen)}")
    print(f"\n小限：{h.age.name} (虚岁 {h.age.nominal_age})")
    print(f"\n流年：{h.yearly.name} ({h.yearly.heavenly_stem}{h.yearly.earthly_branch})")
    print(f"  四化：{'、'.join(h.yearly.mutagen)}")
    print(f"\n流月：{h.monthly.name}")
    print(f"流日：{h.daily.name}")
    print(f"流时：{h.hourly.name}")
    print()

    # ============================================================
    # 8. AI Prompt
    # ============================================================
    print("===== 8. AI Prompt =====\n")

    prompt = astro.astrolabe_to_prompt(result)
    print(prompt[:300] + "...\n")

    print("===== 示例完毕 =====")


if __name__ == "__main__":
    main()

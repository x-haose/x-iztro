#!/usr/bin/env python3
"""
x-iztro Python 示例

前置步骤：
    cd x-iztro
    pip install maturin
    PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1 maturin develop --features python

运行方式：
    python examples/python/main.py
"""

from x_iztro import Astro, IztroError
from x_iztro.enums import MajorStar, Mutagen, PalaceName


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

    soul = result.palace(PalaceName.SOUL)
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
        print(f"有紫微：{soul.has([MajorStar.ZIWEI])}")
        print(f"有紫微或天府：{soul.has_one_of([MajorStar.ZIWEI, MajorStar.TIANFU])}")
        print(f"空宫：{soul.is_empty()}")
        print(f"化禄：{soul.has_mutagen(Mutagen.LU)}")
        print(f"化忌：{soul.has_mutagen(Mutagen.JI)}")
    print()

    # ============================================================
    # 6. 查找星耀
    # ============================================================
    print("===== 6. 查找星耀 =====\n")

    for name in [MajorStar.ZIWEI, MajorStar.TIANJI, MajorStar.TAIYANG, MajorStar.TAIYIN, "禄存"]:
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

    h = result.horoscope("2024-10-1", 0)

    print(f"日期：{h.solar_date} / {h.lunar_date}")
    print(f"\n{h.decadal.name}：{h.decadal.heavenly_stem}{h.decadal.earthly_branch}")
    print(f"  四化：{'、'.join(h.decadal.mutagen)}")
    print(f"\n{h.age.name}：虚岁 {h.age.nominal_age}")
    print(f"\n{h.yearly.name}：{h.yearly.heavenly_stem}{h.yearly.earthly_branch}")
    print(f"  四化：{'、'.join(h.yearly.mutagen)}")
    print(f"  岁前十二神[命宫位]：{h.yearly.yearly_dec_star.suiqian12[0]}")
    print(f"\n{h.monthly.name}：{h.monthly.heavenly_stem}{h.monthly.earthly_branch}")
    print(f"{h.daily.name}：{h.daily.heavenly_stem}{h.daily.earthly_branch}")
    print(f"{h.hourly.name}：{h.hourly.heavenly_stem}{h.hourly.earthly_branch}")
    print()

    # ============================================================
    # 8. 语义化文本 — str() 即面向语言模型与人的完整描述
    # ============================================================
    print("===== 8. 语义化文本 =====\n")

    text = result.to_text()  # 等价于 str(result)
    print(text[:300] + "...\n")

    fortune_text = h.to_text()  # 等价于 str(h)
    print(fortune_text[:200] + "...\n")

    print(result.palace(PalaceName.SOUL).to_text()[:200] + "...\n")

    # knowledge=True：事实节之后追加内嵌知识包的释义（星耀/格局/四化），
    # 也可传 KnowledgePack 实例（自定义或合并后的包）
    explained = result.to_text(knowledge=True)
    print(explained[explained.index("=== 星耀释义 ==="):][:300] + "...\n")

    # ============================================================
    # 9. JSON 导出与错误处理
    # ============================================================
    print("===== 9. JSON 导出与错误处理 =====\n")

    # 与 JS iztro 的 JSON.stringify 逐键逐值一致
    print(f"JSON 长度：{len(result.to_json())} 字符，首个宫位："
          f"{result.to_dict()['palaces'][0]['name']}")

    # 入参非法时抛 IztroError（继承 ValueError），code 为机器可读分类
    try:
        astro.by_solar("not-a-date", 2, "female")
    except IztroError as e:
        print(f"错误分类：{e.code}，描述：{e}")
    print()

    print("===== 示例完毕 =====")


if __name__ == "__main__":
    main()

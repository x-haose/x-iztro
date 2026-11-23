//! 寿星天文历的定朔定气：合朔与二十四节气的时刻。
//!
//! 逐函数移植自 lunar-typescript 的 `ShouXingUtil`，运算顺序与原式一致（浮点加法不满足结合律，
//! 改写算式会让个别节气的秒数或合朔的日期在边界上分叉）。记号：`t` 是 J2000 起算的儒略世纪数，
//! `w` 是黄经角（弧度），返回的时刻是 J2000 起算的日数（北京时间）。三角函数取 `libm`，
//! 各平台结果一致。

use std::f64::consts::PI;

use super::tables::{DT_AT, NUT_B, QB, QI_KB, SB, SHUO_KB, XL0, XL1_0, XL1_1, XL1_2, XL1_3};

/// 每日秒数。
const SECOND_PER_DAY: f64 = 86400.0;
/// 每弧度的角秒数。
const SECOND_PER_RAD: f64 = 648000.0 / PI;
/// 北京时间相对世界时的偏移（8 小时，以日计）。
const ONE_THIRD: f64 = 1.0 / 3.0;
/// J2000.0（2000-01-01 12:00 TT）的儒略日。
pub(super) const J2000: f64 = 2451545.0;
/// 定朔定气改用高精度算法的起点（儒略日，1959 年末）。
const HIGH_PRECISION_FROM: f64 = 2436935.0;

/// 正弦（`libm` 实现）。
fn sin(x: f64) -> f64 {
    libm::sin(x)
}

/// 余弦（`libm` 实现）。
fn cos(x: f64) -> f64 {
    libm::cos(x)
}

/// 黄经章动（弧度），取 [`NUT_B`] 全部项。
fn nutation_lon2(t: f64) -> f64 {
    let mut a = -1.742 * t;
    let t2 = t * t;
    let mut dl = 0.0;
    for k in NUT_B.as_chunks::<5>().0 {
        dl += (k[3] + a) * sin(k[0] + k[1] * t + k[2] * t2);
        a = 0.0;
    }
    dl / 100.0 / SECOND_PER_RAD
}

/// 地球日心黄经（弧度）；`n` 为各阶取用的项数规模，负数取全部项。
fn e_lon(t: f64, n: i32) -> f64 {
    let t = t / 10.0;
    let mut v = 0.0;
    let mut tn = 1.0;
    let pn = 1;
    let m0 = XL0[pn + 1] - XL0[pn];
    for i in 0..6 {
        let n1 = XL0[pn + i].floor();
        let n2 = XL0[pn + 1 + i].floor();
        let n0 = n2 - n1;
        if n0 != 0.0 {
            let m = if n < 0 {
                n2
            } else {
                let mut m = (3.0 * f64::from(n) * n0 / m0 + 0.5 + n1).floor();
                if i != 0 {
                    m += 3.0;
                }
                m.min(n2)
            };
            let mut c = 0.0;
            for j in (n1 as usize..m as usize).step_by(3) {
                c += XL0[j] * cos(XL0[j + 1] + t * XL0[j + 2]);
            }
            v += c * tn;
        }
        tn *= t;
    }
    v /= XL0[0];
    let t2 = t * t;
    v += (-0.0728 - 2.7702 * t - 1.1019 * t2 - 0.0996 * t2 * t) / SECOND_PER_RAD;
    v
}

/// 月球地心黄经（弧度）；`n` 为各阶取用的项数规模，负数取全部项。
fn m_lon(t: f64, n: i32) -> f64 {
    let series: [&[f64]; 4] = [&XL1_0, &XL1_1, &XL1_2, &XL1_3];
    let obl = XL1_0.len();
    let mut tn = 1.0;
    let mut v = 0.0;
    let mut t2 = t * t;
    let mut t3 = t2 * t;
    let mut t4 = t3 * t;
    let t5 = t4 * t;
    let tx = t - 10.0;
    v += (3.81034409 + 8399.684730072 * t - 3319e-8 * t2 + 311e-10 * t3 - 2033e-13 * t4)
        * SECOND_PER_RAD;
    v += 5028.792262 * t + 1.1124406 * t2 + 7699e-8 * t3 - 23479e-9 * t4 - 178e-10 * t5;
    if tx > 0.0 {
        v += -0.866 + 1.43 * tx + 0.054 * tx * tx;
    }
    t2 /= 1e4;
    t3 /= 1e8;
    t4 /= 1e8;
    let n = if n < 0 { obl as f64 } else { f64::from(n * 6) };
    for (i, f) in series.iter().enumerate() {
        let l = f.len();
        let mut m = (n * l as f64 / obl as f64 + 0.5).floor() as usize;
        if i > 0 {
            m += 6;
        }
        let m = m.min(l);
        let mut c = 0.0;
        for j in (0..m).step_by(6) {
            c +=
                f[j] * cos(f[j + 1] + t * f[j + 2] + t2 * f[j + 3] + t3 * f[j + 4] + t4 * f[j + 5]);
        }
        v += c * tn;
        tn *= t;
    }
    v / SECOND_PER_RAD
}

/// 太阳光行差（弧度）。
fn gxc_sun_lon(t: f64) -> f64 {
    let t2 = t * t;
    let v = -0.043126 + 628.301955 * t - 2732e-9 * t2;
    let e = 0.016708634 - 42037e-9 * t - 1267e-10 * t2;
    -20.49552 * (1.0 + e * cos(v)) / SECOND_PER_RAD
}

/// 地球公转角速度（弧度/儒略世纪）。
fn ev(t: f64) -> f64 {
    let f = 628.307585 * t;
    628.332
        + 21.0 * sin(1.527 + f)
        + 0.44 * sin(1.48 + f * 2.0)
        + 0.129 * sin(5.82 + f) * t
        + 55e-5 * sin(4.21 + f) * t * t
}

/// 太阳视黄经（弧度）。
fn sa_lon(t: f64, n: i32) -> f64 {
    e_lon(t, n) + nutation_lon2(t) + gxc_sun_lon(t) + PI
}

/// 超出 [`DT_AT`] 表尾后的 ΔT 外推（秒），`jsd` 为加速度系数。
fn dt_ext(y: f64, jsd: f64) -> f64 {
    let dy = (y - 1820.0) / 100.0;
    -20.0 + jsd * dy * dy
}

/// 年份 `y`（可带小数）的 ΔT（秒）。
fn dt_calc(y: f64) -> f64 {
    let size = DT_AT.len();
    let y0 = DT_AT[size - 2];
    let t0 = DT_AT[size - 1];
    if y >= y0 {
        let jsd = 31.0;
        if y > y0 + 100.0 {
            return dt_ext(y, jsd);
        }
        return dt_ext(y, jsd) - (dt_ext(y0, jsd) - t0) * (y0 + 100.0 - y) / 100.0;
    }
    // y 早于表尾年份 DT_AT[size-2]，最迟在末段停下
    let mut i = 0;
    while y >= DT_AT[i + 5] {
        i += 5;
    }
    let t1 = (y - DT_AT[i]) / (DT_AT[i + 5] - DT_AT[i]) * 10.0;
    let t2 = t1 * t1;
    let t3 = t2 * t1;
    DT_AT[i + 1] + DT_AT[i + 2] * t1 + DT_AT[i + 3] * t2 + DT_AT[i + 4] * t3
}

/// J2000 起算第 `t` 日的 ΔT（日）。
fn dt_t(t: f64) -> f64 {
    dt_calc(t / 365.2425 + 2e3) / SECOND_PER_DAY
}

/// 月球公转角速度（弧度/儒略世纪）。
fn mv(t: f64) -> f64 {
    let mut v = 8399.71 - 914.0 * sin(0.7848 + 8328.691425 * t + 1523e-7 * t * t);
    v -= 179.0 * sin(2.543 + 15542.7543 * t)
        + 160.0 * sin(0.1874 + 7214.0629 * t)
        + 62.0 * sin(3.14 + 16657.3828 * t)
        + 34.0 * sin(4.827 + 16866.9323 * t)
        + 22.0 * sin(4.9 + 23871.4457 * t)
        + 12.0 * sin(2.59 + 14914.4523 * t)
        + 7.0 * sin(0.23 + 6585.7609 * t)
        + 5.0 * sin(0.9 + 25195.624 * t)
        + 5.0 * sin(2.32 - 7700.3895 * t)
        + 5.0 * sin(3.88 + 8956.9934 * t)
        + 5.0 * sin(0.49 + 7771.3771 * t);
    v
}

/// 太阳视黄经达到 `w` 的时刻（儒略世纪），高精度迭代。
fn sa_lon_t(w: f64) -> f64 {
    let mut v = 628.3319653318;
    let mut t = (w - 1.75347 - PI) / v;
    v = ev(t);
    t += (w - sa_lon(t, 10)) / v;
    v = ev(t);
    t += (w - sa_lon(t, -1)) / v;
    t
}

/// 月日视黄经差（弧度）。
fn msa_lon(t: f64, mn: i32, sn: i32) -> f64 {
    m_lon(t, mn) + -34e-7 - (e_lon(t, sn) + gxc_sun_lon(t) + PI)
}

/// 月日视黄经差达到 `w` 的时刻（儒略世纪），高精度迭代。
fn msa_lon_t(w: f64) -> f64 {
    let mut v = 7771.37714500204;
    let mut t = (w + 1.08472) / v;
    t += (w - msa_lon(t, 3, 3)) / v;
    v = mv(t) - ev(t);
    t += (w - msa_lon(t, 20, 10)) / v;
    t += (w - msa_lon(t, -1, 60)) / v;
    t
}

/// 太阳视黄经达到 `w` 的时刻（儒略世纪），快速算法。
fn sa_lon_t2(w: f64) -> f64 {
    let v = 628.3319653318;
    let mut t = (w - 1.75347 - PI) / v;
    t -= (5297e-9 * t * t
        + 0.0334166 * cos(4.669257 + 628.307585 * t)
        + 2061e-7 * cos(2.67823 + 628.307585 * t) * t)
        / v;
    t += (w - e_lon(t, 8) - PI + (20.5 + 17.2 * sin(2.1824 - 33.75705 * t)) / SECOND_PER_RAD) / v;
    t
}

/// 月日视黄经差达到 `w` 的时刻（儒略世纪），快速算法。
fn msa_lon_t2(w: f64) -> f64 {
    let mut v = 7771.37714500204;
    let mut t = (w + 1.08472) / v;
    let mut t2 = t * t;
    t -= (-3309e-8 * t2
        + 0.10976 * cos(0.784758 + 8328.6914246 * t + 152292e-9 * t2)
        + 0.02224 * cos(0.1874 + 7214.0628654 * t - 21848e-8 * t2)
        - 0.03342 * cos(4.669257 + 628.307585 * t))
        / v;
    t2 = t * t;
    let l = m_lon(t, 20)
        - (4.8950632
            + 628.3319653318 * t
            + 5297e-9 * t2
            + 0.0334166 * cos(4.669257 + 628.307585 * t)
            + 2061e-7 * cos(2.67823 + 628.307585 * t) * t
            + 349e-6 * cos(4.6261 + 1256.61517 * t)
            - 20.5 / SECOND_PER_RAD);
    v = 7771.38
        - 914.0 * sin(0.7848 + 8328.691425 * t + 1523e-7 * t2)
        - 179.0 * sin(2.543 + 15542.7543 * t)
        - 160.0 * sin(0.1874 + 7214.0629 * t);
    t += (w - l) / v;
    t
}

/// 太阳视黄经为 `w` 的节气时刻（J2000 起算日数），高精度。
fn qi_high(w: f64) -> f64 {
    let mut t = sa_lon_t2(w) * 36525.0;
    t = t - dt_t(t) + ONE_THIRD;
    let v = (t + 0.5) % 1.0 * SECOND_PER_DAY;
    if v < 1200.0 || v > SECOND_PER_DAY - 1200.0 {
        t = sa_lon_t(w) * 36525.0 - dt_t(t) + ONE_THIRD;
    }
    t
}

/// 月日视黄经差为 `w` 的合朔时刻（J2000 起算日数），高精度。
fn shuo_high(w: f64) -> f64 {
    let mut t = msa_lon_t2(w) * 36525.0;
    t = t - dt_t(t) + ONE_THIRD;
    let v = (t + 0.5) % 1.0 * SECOND_PER_DAY;
    if v < 1800.0 || v > SECOND_PER_DAY - 1800.0 {
        t = msa_lon_t(w) * 36525.0 - dt_t(t) + ONE_THIRD;
    }
    t
}

/// 太阳视黄经为 `w` 的节气时刻（J2000 起算日数），低精度。
fn qi_low(w: f64) -> f64 {
    let v = 628.3319653318;
    let mut t = (w - 4.895062166) / v;
    t -= (53.0 * t * t
        + 334116.0 * cos(4.67 + 628.307585 * t)
        + 2061.0 * cos(2.678 + 628.3076 * t) * t)
        / v
        / 1e7;
    let n = 4895062166e-2
        + 6283319653318e-3 * t
        + 53.0 * t * t
        + 334166.0 * cos(4.669257 + 628.307585 * t)
        + 3489.0 * cos(4.6261 + 1256.61517 * t)
        + 2060.6 * cos(2.67823 + 628.307585 * t) * t
        - 994.0
        - 834.0 * sin(2.1824 - 33.75705 * t);
    t -= (n / 1e7 - w) / 628.332 + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
    t * 36525.0 + ONE_THIRD
}

/// 月日视黄经差为 `w` 的合朔时刻（J2000 起算日数），低精度。
fn shuo_low(w: f64) -> f64 {
    let v = 7771.37714500204;
    let mut t = (w + 1.08472) / v;
    t -= (-331e-7 * t * t
        + 0.10976 * cos(0.785 + 8328.6914 * t)
        + 0.02224 * cos(0.187 + 7214.0629 * t)
        - 0.03342 * cos(4.669 + 628.3076 * t))
        / v
        + (32.0 * (t + 1.8) * (t + 1.8) - 20.0) / SECOND_PER_DAY / 36525.0;
    t * 36525.0 + ONE_THIRD
}

/// 修正串在第 `index` 项上的日数修正（b'1' 加一日、b'2' 减一日，越界或 b'0' 不修正）。
fn correction(table: &[u8], index: f64) -> f64 {
    match table.get(index as usize) {
        Some(b'1') => 1.0,
        Some(b'2') => -1.0,
        _ => 0.0,
    }
}

/// 古历平朔 / 平气：在参数表 `kb` 里找到 `jd + pc` 所在段，按该段首日与周期推出整日儒略日。
/// 调用方保证 `jd + pc` 早于表尾，查找最迟停在末段。
fn from_kb(kb: &[f64], jd: f64, pc: f64) -> f64 {
    let mut i = 0;
    while jd + pc >= kb[i + 2] {
        i += 2;
    }
    let mut d = kb[i] + kb[i + 1] * ((jd + pc - kb[i]) / kb[i + 1]).floor();
    d = (d + 0.5).floor();
    if d == 1683460.0 {
        d += 1.0;
    }
    d - J2000
}

/// 距 J2000 第 `jd` 日附近的合朔日（J2000 起算的整日数）。
///
/// 按年代分三段取算法：古历平朔（[`SHUO_KB`]）、低精度定朔加 [`SB`] 修正、高精度定朔。
pub(super) fn calc_shuo(jd: f64) -> f64 {
    let pc = 14.0;
    let jd = jd + J2000;
    let f1 = SHUO_KB[0] - pc;
    let f2 = SHUO_KB[SHUO_KB.len() - 1] - pc;
    if jd < f1 || jd >= HIGH_PRECISION_FROM {
        (shuo_high(((jd + pc - 2451551.0) / 29.5306).floor() * PI * 2.0) + 0.5).floor()
    } else if jd < f2 {
        from_kb(&SHUO_KB, jd, pc)
    } else {
        let d = (shuo_low(((jd + pc - 2451551.0) / 29.5306).floor() * PI * 2.0) + 0.5).floor();
        d + correction(SB, ((jd - f2) / 29.5306).floor())
    }
}

/// 距 J2000 第 `jd` 日附近的节气日（J2000 起算的整日数）。
///
/// 按年代分三段取算法：古历平气（[`QI_KB`]）、低精度定气加 [`QB`] 修正、高精度定气。
pub(super) fn calc_qi(jd: f64) -> f64 {
    let pc = 7.0;
    let jd = jd + J2000;
    let f1 = QI_KB[0] - pc;
    let f2 = QI_KB[QI_KB.len() - 1] - pc;
    if jd < f1 || jd >= HIGH_PRECISION_FROM {
        (qi_high(((jd + pc - 2451259.0) / 365.2422 * 24.0).floor() * PI / 12.0) + 0.5).floor()
    } else if jd < f2 {
        from_kb(&QI_KB, jd, pc)
    } else {
        let d =
            (qi_low(((jd + pc - 2451259.0) / 365.2422 * 24.0).floor() * PI / 12.0) + 0.5).floor();
        d + correction(QB, ((jd - f2) / 365.2422 * 24.0).floor())
    }
}

/// 太阳视黄经为 `w` 的精确节气时刻（J2000 起算日数）。
fn qi_accurate(w: f64) -> f64 {
    let t = sa_lon_t(w) * 36525.0;
    t - dt_t(t) + ONE_THIRD
}

/// 距 J2000 第 `jd` 日附近（±5 日内）那个节气的精确时刻（J2000 起算日数）。
pub(super) fn qi_accurate2(jd: f64) -> f64 {
    let d = PI / 12.0;
    let w = ((jd + 293.0) / 365.2422 * 24.0).floor() * d;
    let a = qi_accurate(w);
    if a - jd > 5.0 {
        return qi_accurate(w - d);
    }
    if a - jd < -5.0 {
        return qi_accurate(w + d);
    }
    a
}

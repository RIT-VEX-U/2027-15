// SPDX-License-Identifier: SunPro

#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_log.hpp"
#include "cevalm_sqrt.hpp"
#include "cevalm_trig.hpp"

/*
 * Bessel approximations derived from fdlibm.
 * Copyright (C) 1993 Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */

namespace cevalm {

inline constexpr double bessel_p0_r[4][6] = {
    {0.0, -7.03124999999900357484e-02, -8.08167041275349795626e+00, -2.57063105679704847262e+02,
     -2.48521641009428822144e+03, -5.25304380490729545272e+03},
    {-1.14125464691894502584e-11, -7.03124940873599280078e-02, -4.15961064470587782438e+00,
     -6.76747652265167261021e+01, -3.31231299649172967747e+02, -3.46433388365604912451e+02},
    {-2.54704601771951915620e-09, -7.03119616381481654654e-02, -2.40903221549529611423e+00,
     -2.19659774734883086467e+01, -5.80791704701737572236e+01, -3.14479470594888503854e+01},
    {-8.87534333032526411254e-08, -7.03030995483624743247e-02, -1.45073846780952986357e+00,
     -7.63569613823527770791e+00, -1.11931668860356747786e+01, -3.23364579351335335033e+00}};
inline constexpr double bessel_p0_s[4][5] = {
    {1.16534364619668181717e+02, 3.83374475364121826715e+03, 4.05978572648472545552e+04,
     1.16752972564375915681e+05, 4.76277284146730962675e+04},
    {6.07539382692300335975e+01, 1.05125230595704579173e+03, 5.97897094333855784498e+03,
     9.62544514357774460223e+03, 2.40605815922939109441e+03},
    {3.58560338055209726349e+01, 3.61513983050303863820e+02, 1.19360783792111533330e+03,
     1.12799679856907414432e+03, 1.73580930813335754692e+02},
    {2.22202997532088808441e+01, 1.36206794218215208048e+02, 2.70470278658083486789e+02,
     1.53875394208320329881e+02, 1.46576176948256193810e+01}};
inline constexpr double bessel_q0_r[4][6] = {
    {0.0, 7.32421874999935051953e-02, 1.17682064682252693899e+01, 5.57673380256401856059e+02,
     8.85919720756468632317e+03, 3.70146267776887834771e+04},
    {1.84085963594515531381e-11, 7.32421766612684765896e-02, 5.83563508962056953777e+00,
     1.35111577286449829671e+02, 1.02724376596164097464e+03, 1.98997785864605384631e+03},
    {4.37741014089738620906e-09, 7.32411180042911447163e-02, 3.34423137516170720929e+00,
     4.26218440745412650017e+01, 1.70808091340565596283e+02, 1.66733948696651168575e+02},
    {1.50444444886983272379e-07, 7.32234265963079278272e-02, 1.99819174093815998816e+00,
     1.44956029347885735348e+01, 3.16662317504781540833e+01, 1.62527075710929267416e+01}};
inline constexpr double bessel_q0_s[4][6] = {
    {1.63776026895689824414e+02, 8.09834494656449805916e+03, 1.42538291419120476348e+05,
     8.03309257119514397345e+05, 8.40501579819060512818e+05, -3.43899293537866615225e+05},
    {8.27766102236537761883e+01, 2.07781416421392987104e+03, 1.88472887785718085070e+04,
     5.67511122894947329769e+04, 3.59767538425114471465e+04, -5.35434275601944773371e+03},
    {4.87588729724587182091e+01, 7.09689221056606015736e+02, 3.70414822620111362994e+03,
     6.46042516752568917582e+03, 2.51633368920368957333e+03, -1.49247451836156386662e+02},
    {3.03655848355219184498e+01, 2.69348118608049844624e+02, 8.44783757595320139444e+02,
     8.82935845112488550512e+02, 2.12666388511798828631e+02, -5.31095493882666946917e+00}};

inline constexpr double bessel_p1_r[4][6] = {
    {0.0, 1.17187499999988647970e-01, 1.32394806593073575129e+01, 4.12051854307378562225e+02,
     3.87474538913960532227e+03, 7.91447954031891731574e+03},
    {1.31990519556243522749e-11, 1.17187493190614097638e-01, 6.80275127868432871736e+00,
     1.08308182990189109773e+02, 5.17636139533199752805e+02, 5.28715201363337541807e+02},
    {3.02503916137373618024e-09, 1.17186865567253592491e-01, 3.93297750033315640650e+00,
     3.51194035591636932736e+01, 9.10550110750781271918e+01, 4.85590685197364919645e+01},
    {1.07710830106873743082e-07, 1.17176219462683348094e-01, 2.36851496667608785174e+00,
     1.22426109148261232917e+01, 1.76939711271687727390e+01, 5.07352312588818499250e+00}};
inline constexpr double bessel_p1_s[4][5] = {
    {1.14207370375678408436e+02, 3.65093083420853463394e+03, 3.69562060269033463555e+04,
     9.76027935934950801311e+04, 3.08042720627888811578e+04},
    {5.92805987221131331921e+01, 9.91401418733614377743e+02, 5.35326695291487976647e+03,
     7.84469031749551231769e+03, 1.50404688810361062679e+03},
    {3.47913095001251519989e+01, 3.36762458747825746741e+02, 1.04687139975775130551e+03,
     8.90811346398256432622e+02, 1.03787932439639277504e+02},
    {2.14364859363821409488e+01, 1.25290227168402751090e+02, 2.32276469057162813669e+02,
     1.17679373287147100768e+02, 8.36463893371618283368e+00}};
inline constexpr double bessel_q1_r[4][6] = {
    {0.0, -1.02539062499992714161e-01, -1.62717534544589987888e+01, -7.59601722513950107896e+02,
     -1.18498066702429587167e+04, -4.84385124285750353010e+04},
    {-2.08979931141764104297e-11, -1.02539050241375426231e-01, -8.05644828123936029840e+00,
     -1.83669607474888380239e+02, -1.37319376065508163265e+03, -2.61244440453215656817e+03},
    {-5.07831226461766561369e-09, -1.02537829820837089745e-01, -4.61011581139473403113e+00,
     -5.78472216562783643212e+01, -2.28244540737631695038e+02, -2.19210128478909325622e+02},
    {-1.78381727510958865572e-07, -1.02517042607985553460e-01, -2.75220568278187460720e+00,
     -1.96636162643703720221e+01, -4.23253133372830490089e+01, -2.13719211703704061733e+01}};
inline constexpr double bessel_q1_s[4][6] = {
    {1.61395369700722909556e+02, 7.82538599923348465381e+03, 1.33875336287249578163e+05,
     7.19657723683240939863e+05, 6.66601232617776375264e+05, -2.94490264303834643215e+05},
    {8.12765501384335777857e+01, 1.99179873460485964642e+03, 1.74684851924908907677e+04,
     4.98514270910352279316e+04, 2.79480751638918118260e+04, -4.71918354795128470869e+03},
    {4.76651550323729509273e+01, 6.73865112676699709482e+02, 3.38015286679526343505e+03,
     5.54772909720722782367e+03, 1.90311919338810798763e+03, -1.35201191444307340817e+02},
    {2.95333629060523854548e+01, 2.52981549982190529136e+02, 7.57502834868645436472e+02,
     7.39393205320467245656e+02, 1.55949003336666123687e+02, -4.95949898822628210127e+00}};

constexpr unsigned bessel_interval(uint32 high) {
    return high >= 0x40200000U ? 0U : (high >= 0x40122e8bU ? 1U : (high >= 0x4006db6dU ? 2U : 3U));
}

constexpr double bessel_asymptotic_p(double x, const double (&rcoef)[4][6],
                                     const double (&scoef)[4][5]) {
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (high >= 0x41b00000U)
        return 1.0;
    const unsigned k = bessel_interval(high);
    const double z = 1.0 / (x * x), z2 = z * z, z4 = z2 * z2;
    const double r = (rcoef[k][0] + z * rcoef[k][1]) + z2 * (rcoef[k][2] + z * rcoef[k][3]) +
                     z4 * (rcoef[k][4] + z * rcoef[k][5]);
    const double s = (1.0 + z * scoef[k][0]) + z2 * (scoef[k][1] + z * scoef[k][2]) +
                     z4 * (scoef[k][3] + z * scoef[k][4]);
    return 1.0 + r / s;
}

constexpr double bessel_asymptotic_q(double x, const double (&rcoef)[4][6],
                                     const double (&scoef)[4][6], double leading) {
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (high >= 0x41b00000U)
        return leading / x;
    const unsigned k = bessel_interval(high);
    const double z = 1.0 / (x * x), z2 = z * z, z4 = z2 * z2, z6 = z4 * z2;
    const double r = (rcoef[k][0] + z * rcoef[k][1]) + z2 * (rcoef[k][2] + z * rcoef[k][3]) +
                     z4 * (rcoef[k][4] + z * rcoef[k][5]);
    const double s = (1.0 + z * scoef[k][0]) + z2 * (scoef[k][1] + z * scoef[k][2]) +
                     z4 * (scoef[k][3] + z * scoef[k][4]) + z6 * scoef[k][5];
    return (leading + r / s) / x;
}

constexpr double bessel_pzero(double x) {
    return bessel_asymptotic_p(x, bessel_p0_r, bessel_p0_s);
}
constexpr double bessel_qzero(double x) {
    return bessel_asymptotic_q(x, bessel_q0_r, bessel_q0_s, -0.125);
}
constexpr double bessel_pone(double x) {
    return bessel_asymptotic_p(x, bessel_p1_r, bessel_p1_s);
}
constexpr double bessel_qone(double x) {
    return bessel_asymptotic_q(x, bessel_q1_r, bessel_q1_s, 0.375);
}

constexpr double j0(double x);
constexpr double j1(double x);

constexpr double j0(double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_bessel_j(0.0, x);
    }
#endif

    constexpr double invsqrtpi = 5.64189583547756279280e-01;
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (high >= 0x7ff00000U)
        return isnan(x) ? binary64_quiet_nan(x) : 0.0;
    x = cevalm::fabs(x);
    if (high >= 0x40000000U) {
        double s, c;
        cevalm::sincos(x, &s, &c);
        double ss = s - c, cc = s + c;
        if (high < 0x7fe00000U) {
            const double z = -cevalm::cos(x + x);
            if (s * c < 0.0)
                cc = z / ss;
            else
                ss = z / cc;
        }
        if (high > 0x48000000U)
            return invsqrtpi * cc / cevalm::sqrt(x);
        return invsqrtpi * (bessel_pzero(x) * cc - bessel_qzero(x) * ss) / cevalm::sqrt(x);
    }
    if (high < 0x3f200000U)
        return high < 0x3e400000U ? 1.0 : 1.0 - 0.25 * x * x;
    constexpr double R02 = 1.56249999999999947958e-02, R03 = -1.89979294238854721751e-04,
                     R04 = 1.82954049532700665670e-06, R05 = -4.61832688532103189199e-09;
    constexpr double S01 = 1.56191029464890010492e-02, S02 = 1.16926784663337450260e-04,
                     S03 = 5.13546550207318111446e-07, S04 = 1.16614003333790000205e-09;
    const double z = x * x, z2 = z * z, z4 = z2 * z2;
    const double r = z * R02 + z2 * (R03 + z * R04) + z4 * R05;
    const double s = 1.0 + z * S01 + z2 * (S02 + z * S03) + z4 * S04;
    if (high < 0x3ff00000U)
        return 1.0 + z * (-0.25 + r / s);
    const double u = 0.5 * x;
    return (1.0 + u) * (1.0 - u) + z * (r / s);
}

constexpr double j1(double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_bessel_j(1.0, x);
    }
#endif

    constexpr double invsqrtpi = 5.64189583547756279280e-01;
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (high >= 0x7ff00000U)
        return isnan(x) ? binary64_quiet_nan(x) : cevalm::copysign(0.0, x);
    const double y = cevalm::fabs(x);
    if (high >= 0x40000000U) {
        double s, c;
        cevalm::sincos(y, &s, &c);
        double ss = -s - c, cc = s - c;
        if (high < 0x7fe00000U) {
            const double z = cevalm::cos(y + y);
            if (s * c > 0.0)
                cc = z / ss;
            else
                ss = z / cc;
        }
        double z = high > 0x48000000U
                       ? invsqrtpi * cc / cevalm::sqrt(y)
                       : invsqrtpi * (bessel_pone(y) * cc - bessel_qone(y) * ss) / cevalm::sqrt(y);
        return signbit(x) ? -z : z;
    }
    if (high < 0x3e400000U)
        return 0.5 * x;
    constexpr double r00 = -6.25000000000000000000e-02, r01 = 1.40705666955189706048e-03,
                     r02 = -1.59955631084035597520e-05, r03 = 4.96727999609584448412e-08;
    constexpr double s01 = 1.91537599538363460805e-02, s02 = 1.85946785588630915560e-04,
                     s03 = 1.17718464042623683263e-06, s04 = 5.04636257076217042715e-09,
                     s05 = 1.23542274426137913908e-11;
    const double z = x * x, z2 = z * z, z4 = z2 * z2;
    double r = z * r00 + z2 * (r01 + z * r02) + z4 * r03;
    r *= x;
    const double s = 1.0 + z * s01 + z2 * (s02 + z * s03) + z4 * (s04 + z * s05);
    return x * 0.5 + r / s;
}

constexpr double y0(double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_neumann(0.0, x);
    }
#endif

    constexpr double invsqrtpi = 5.64189583547756279280e-01, tpi = 6.36619772367581382433e-01;
    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return signbit(x) ? binary64_from_bits(0xfff8000000000000ULL) : 0.0;
    if (magnitude == 0)
        return binary64_infinity(true);
    if (signbit(x))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (high >= 0x40000000U) {
        double s, c;
        cevalm::sincos(x, &s, &c);
        double ss = s - c, cc = s + c;
        if (high < 0x7fe00000U) {
            const double z = -cevalm::cos(x + x);
            if (s * c < 0.0)
                cc = z / ss;
            else
                ss = z / cc;
        }
        if (high > 0x48000000U)
            return invsqrtpi * ss / cevalm::sqrt(x);
        return invsqrtpi * (bessel_pzero(x) * ss + bessel_qzero(x) * cc) / cevalm::sqrt(x);
    }
    constexpr double u00 = -7.38042951086872317523e-02, u01 = 1.76666452509181115538e-01,
                     u02 = -1.38185671945596898896e-02, u03 = 3.47453432093683650238e-04,
                     u04 = -3.81407053724364161125e-06, u05 = 1.95590137035022920206e-08,
                     u06 = -3.98205194132103398453e-11;
    if (high <= 0x3e400000U)
        return u00 + tpi * cevalm::log(x);
    constexpr double v01 = 1.27304834834123699328e-02, v02 = 7.60068627350353253702e-05,
                     v03 = 2.59150851840457805467e-07, v04 = 4.41110311332675467403e-10;
    const double z = x * x, z2 = z * z, z4 = z2 * z2, z6 = z4 * z2;
    const double u = u00 + z * u01 + z2 * (u02 + z * u03) + z4 * (u04 + z * u05) + z6 * u06;
    const double v = 1.0 + z * v01 + z2 * (v02 + z * v03) + z4 * v04;
    return u / v + tpi * (cevalm::j0(x) * cevalm::log(x));
}

constexpr double y1(double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_neumann(1.0, x);
    }
#endif

    constexpr double invsqrtpi = 5.64189583547756279280e-01, tpi = 6.36619772367581382433e-01;
    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return signbit(x) ? binary64_from_bits(0xfff8000000000000ULL) : 0.0;
    if (magnitude == 0)
        return binary64_infinity(true);
    if (signbit(x))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (high >= 0x40000000U) {
        double s, c;
        cevalm::sincos(x, &s, &c);
        double ss = -s - c, cc = s - c;
        if (high < 0x7fe00000U) {
            const double z = cevalm::cos(x + x);
            if (s * c > 0.0)
                cc = z / ss;
            else
                ss = z / cc;
        }
        if (high > 0x48000000U)
            return invsqrtpi * ss / cevalm::sqrt(x);
        return invsqrtpi * (bessel_pone(x) * ss + bessel_qone(x) * cc) / cevalm::sqrt(x);
    }
    if (high <= 0x3c900000U)
        return -tpi / x;
    constexpr double U[5] = {-1.96057090646238940668e-01, 5.04438716639811282616e-02,
                             -1.91256895875763547298e-03, 2.35252600561610495928e-05,
                             -9.19099158039878874504e-08};
    constexpr double V[5] = {1.99167318236649903973e-02, 2.02552581025135171496e-04,
                             1.35608801097516229404e-06, 6.22741452364621501295e-09,
                             1.66559246207992079114e-11};
    const double z = x * x, z2 = z * z, z4 = z2 * z2;
    const double u = U[0] + z * U[1] + z2 * (U[2] + z * U[3]) + z4 * U[4];
    const double v = 1.0 + z * V[0] + z2 * (V[1] + z * V[2]) + z4 * (V[3] + z * V[4]);
    return x * (u / v) + tpi * (cevalm::j1(x) * cevalm::log(x) - 1.0 / x);
}

constexpr double jn(int n, double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_bessel_j(n, x);
    }
#endif

    const uint32 original_high = binary64_high_word(x);
    const uint32 high = original_high & 0x7fffffffU;
    if (isnan(x))
        return binary64_quiet_nan(x);
    const bool negative_order = n < 0;
    const unsigned order =
        negative_order ? 0U - static_cast<unsigned>(n) : static_cast<unsigned>(n);
    if (negative_order)
        x = -x;
    if (order == 0U)
        return cevalm::j0(x);
    if (order == 1U)
        return cevalm::j1(x);
    const bool negate = (order & 1U) != 0U && signbit(x);
    x = cevalm::fabs(x);
    double b = 0.0;
    if (binary64_magnitude_bits(x) == 0 || isinf(x))
        b = 0.0;
    else if (static_cast<double>(order) <= x) {
        if (high >= 0x52d00000U) {
            double s, c;
            cevalm::sincos(x, &s, &c);
            double temp = 0.0;
            switch (order & 3U) {
            case 0:
                temp = c + s;
                break;
            case 1:
                temp = -c + s;
                break;
            case 2:
                temp = -c - s;
                break;
            default:
                temp = c - s;
            }
            b = 5.64189583547756279280e-01 * temp / cevalm::sqrt(x);
        } else {
            double a = cevalm::j0(x);
            b = cevalm::j1(x);
            for (unsigned i = 1; i < order; ++i) {
                const double temp = b;
                b = b * (static_cast<double>(i + i) / x) - a;
                a = temp;
            }
        }
    } else if (high < 0x3e100000U) {
        if (order > 33U)
            b = 0.0;
        else {
            const double temp = x * 0.5;
            b = temp;
            double a = 1.0;
            for (unsigned i = 2; i <= order; ++i) {
                a *= static_cast<double>(i);
                b *= temp;
            }
            b /= a;
        }
    } else {
        const double h = 2.0 / x;
        double w = static_cast<double>(order + order) / x, q0 = w, z = w + h, q1 = w * z - 1.0;
        unsigned k = 1;
        while (q1 < 1.0e9) {
            ++k;
            z += h;
            const double tmp = z * q1 - q0;
            q0 = q1;
            q1 = tmp;
        }
        double t = 0.0;
        for (unsigned i = 2U * (order + k), m = order + order; i >= m; i -= 2U)
            t = 1.0 / (static_cast<double>(i) / x - t);
        double a = t;
        b = 1.0;
        double tmp = static_cast<double>(order);
        tmp *= cevalm::log(cevalm::fabs((2.0 / x) * tmp));
        double di = static_cast<double>((order - 1U) + (order - 1U));
        for (unsigned i = order - 1U; i > 0U; --i) {
            const double temp = b;
            b *= di;
            b = b / x - a;
            a = temp;
            di -= 2.0;
            if (tmp >= 7.09782712893383973096e+02 && b > 1e100) {
                a /= b;
                t /= b;
                b = 1.0;
            }
        }
        z = cevalm::j0(x);
        w = cevalm::j1(x);
        b = cevalm::fabs(z) >= cevalm::fabs(w) ? t * z / b : t * w / a;
    }
    return negate ? -b : b;
}

constexpr double yn(int n, double x) {
#if CEVALM_HAS_STD_RUNTIME && defined(__cpp_lib_math_special_functions)
    if !consteval {
        return std::cyl_neumann(n, x);
    }
#endif

    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (binary64_magnitude_bits(x) == 0)
        return binary64_infinity(true);
    if (signbit(x))
        return binary64_from_bits(0xfff8000000000000ULL);
    const bool negative_order = n < 0;
    const unsigned order =
        negative_order ? 0U - static_cast<unsigned>(n) : static_cast<unsigned>(n);
    const bool negate = negative_order && (order & 1U) != 0U;
    if (order == 0U)
        return cevalm::y0(x);
    if (order == 1U) {
        const double r = cevalm::y1(x);
        return negate ? -r : r;
    }
    if (isinf(x))
        return 0.0;
    double b;
    if (high >= 0x52d00000U) {
        double s, c;
        cevalm::sincos(x, &s, &c);
        double temp = 0.0;
        switch (order & 3U) {
        case 0:
            temp = s - c;
            break;
        case 1:
            temp = -s - c;
            break;
        case 2:
            temp = -s + c;
            break;
        default:
            temp = s + c;
        }
        b = 5.64189583547756279280e-01 * temp / cevalm::sqrt(x);
    } else {
        double a = cevalm::y0(x);
        b = cevalm::y1(x);
        for (unsigned i = 1; i < order && binary64_bits(b) != 0xfff0000000000000ULL; ++i) {
            const double temp = b;
            b = (static_cast<double>(i + i) / x) * b - a;
            a = temp;
        }
    }
    return negate ? -b : b;
}

} // namespace cevalm

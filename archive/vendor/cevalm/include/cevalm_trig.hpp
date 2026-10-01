// SPDX-License-Identifier: SunPro

/*
 * Binary64 trigonometric functions derived from fdlibm.
 * Copyright (C) 1993, 2004 by Sun Microsystems, Inc.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_rem_pio2.hpp"

namespace cevalm {

constexpr double kernel_sin(double x, double y, int tail) {
    constexpr double s1 = -1.66666666666666324348e-01;
    constexpr double s2 = 8.33333333332248946124e-03;
    constexpr double s3 = -1.98412698298579493134e-04;
    constexpr double s4 = 2.75573137070700676789e-06;
    constexpr double s5 = -2.50507602534068634195e-08;
    constexpr double s6 = 1.58969099521155010221e-10;
    if ((binary64_high_word(x) & 0x7fffffffU) < 0x3e400000U)
        return x;
    const double z = x * x;
    const double v = z * x;
    const double r = s2 + z * (s3 + z * (s4 + z * (s5 + z * s6)));
    return tail == 0 ? x + v * (s1 + z * r) : x - ((z * (0.5 * y - v * r) - y) - v * s1);
}

constexpr double kernel_cos(double x, double y) {
    constexpr double c1 = 4.16666666666666019037e-02;
    constexpr double c2 = -1.38888888888741095749e-03;
    constexpr double c3 = 2.48015872894767294178e-05;
    constexpr double c4 = -2.75573143513906633035e-07;
    constexpr double c5 = 2.08757232129817482790e-09;
    constexpr double c6 = -1.13596475577881948265e-11;
    const uint32 high = binary64_high_word(x) & 0x7fffffffU;
    if (high < 0x3e400000U)
        return 1.0;
    const double z = x * x;
    const double r = z * (c1 + z * (c2 + z * (c3 + z * (c4 + z * (c5 + z * c6)))));
    if (high < 0x3fd33333U)
        return 1.0 - (0.5 * z - (z * r - x * y));
    const double qx = high > 0x3fe90000U
                          ? 0.28125
                          : binary64_from_bits(static_cast<uint64>(high - 0x00200000U) << 32);
    const double hz = 0.5 * z - qx;
    return (1.0 - qx) - (hz - (z * r - x * y));
}

constexpr double kernel_tan(double x, double y, int reciprocal) {
    constexpr double pio4 = 7.85398163397448278999e-01;
    constexpr double pio4_low = 3.06161699786838301793e-17;
    constexpr double coefficients[13] = {
        3.33333333333334091986e-01, 1.33333333333201242699e-01, 5.39682539762260521377e-02,
        2.18694882948595424599e-02, 8.86323982359930005737e-03, 3.59207910759131235356e-03,
        1.45620945432529025516e-03, 5.88041240820264096874e-04, 2.46463134818469906812e-04,
        7.81794442939557092300e-05, 7.14072491382608190305e-05, -1.85586374855275456654e-05,
        2.59073051863633712884e-05};

    const int32 signed_high = static_cast<int32>(binary64_high_word(x));
    const uint32 high = static_cast<uint32>(signed_high) & 0x7fffffffU;
    if (high < 0x3e300000U) {
        if (reciprocal == 1)
            return x;
        if ((binary64_bits(x) & ~binary64_sign_mask) == 0)
            return binary64_infinity(false);
        return -1.0 / x;
    }
    if (high >= 0x3fe59428U) {
        if (signed_high < 0) {
            x = -x;
            y = -y;
        }
        const double z = pio4 - x;
        const double w = pio4_low - y;
        x = z + w;
        y = 0.0;
    }
    const double z = x * x;
    const double square = z * z;
    double r =
        coefficients[1] +
        square * (coefficients[3] +
                  square * (coefficients[5] +
                            square * (coefficients[7] +
                                      square * (coefficients[9] + square * coefficients[11]))));
    const double v =
        z *
        (coefficients[2] +
         square * (coefficients[4] +
                   square * (coefficients[6] +
                             square * (coefficients[8] +
                                       square * (coefficients[10] + square * coefficients[12])))));
    const double cube = z * x;
    r = y + z * (cube * (r + v) + y);
    r += coefficients[0] * cube;
    const double w = x + r;
    if (high >= 0x3fe59428U) {
        const double direction = static_cast<double>(reciprocal);
        const double sign = static_cast<double>(1 - ((signed_high >> 30) & 2));
        return sign * (direction - 2.0 * (x - (w * w / (w + direction) - r)));
    }
    if (reciprocal == 1)
        return w;
    const double high_w = binary64_from_bits(binary64_bits(w) & 0xffffffff00000000ULL);
    const double correction = r - (high_w - x);
    const double inverse = -1.0 / w;
    const double high_inverse = binary64_from_bits(binary64_bits(inverse) & 0xffffffff00000000ULL);
    const double residual = 1.0 + high_inverse * high_w;
    return high_inverse + inverse * (residual + high_inverse * correction);
}

constexpr double trig_nan(double x) {
    return isnan(x) ? binary64_quiet_nan(x) : binary64_from_bits(0xfff8000000000000ULL);
}

constexpr double sin(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::sin(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high <= 0x3fe921fbU)
        return kernel_sin(x, 0.0, 0);
    if (high >= 0x7ff00000U)
        return trig_nan(x);
    double remainder[2];
    const int quadrant = rem_pio2(x, remainder) & 3;
    switch (quadrant) {
    case 0:
        return kernel_sin(remainder[0], remainder[1], 1);
    case 1:
        return kernel_cos(remainder[0], remainder[1]);
    case 2:
        return -kernel_sin(remainder[0], remainder[1], 1);
    default:
        return -kernel_cos(remainder[0], remainder[1]);
    }
}

constexpr double cos(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::cos(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high <= 0x3fe921fbU)
        return kernel_cos(x, 0.0);
    if (high >= 0x7ff00000U)
        return trig_nan(x);
    double remainder[2];
    const int quadrant = rem_pio2(x, remainder) & 3;
    switch (quadrant) {
    case 0:
        return kernel_cos(remainder[0], remainder[1]);
    case 1:
        return -kernel_sin(remainder[0], remainder[1], 1);
    case 2:
        return -kernel_cos(remainder[0], remainder[1]);
    default:
        return kernel_sin(remainder[0], remainder[1], 1);
    }
}

constexpr double tan(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::tan(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high <= 0x3fe921fbU)
        return kernel_tan(x, 0.0, 1);
    if (high >= 0x7ff00000U)
        return trig_nan(x);
    double remainder[2];
    const int quadrant = rem_pio2(x, remainder);
    return kernel_tan(remainder[0], remainder[1], 1 - ((quadrant & 1) << 1));
}

constexpr void sincos(double x, double* sine, double* cosine) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        *sine = std::sin(x);
        *cosine = std::cos(x);
        return;
    }
#endif

    *sine = cevalm::sin(x);
    *cosine = cevalm::cos(x);
}

} // namespace cevalm

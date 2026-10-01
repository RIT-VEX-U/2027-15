// SPDX-License-Identifier: SunPro

/*
 * Binary64 exponential functions derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_scale.hpp"

namespace cevalm {

constexpr double exp(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::exp(x);
    }
#endif

    constexpr double one = 1.0;
    constexpr double half[2] = {0.5, -0.5};
    constexpr double overflow_threshold = 7.09782712893383973096e+02;
    constexpr double underflow_threshold = -7.45133219101941108420e+02;
    constexpr double ln2_high[2] = {6.93147180369123816490e-01, -6.93147180369123816490e-01};
    constexpr double ln2_low[2] = {1.90821492927058770002e-10, -1.90821492927058770002e-10};
    constexpr double inverse_ln2 = 1.44269504088896338700e+00;
    constexpr double p1 = 1.66666666666666019037e-01;
    constexpr double p2 = -2.77777777770155933842e-03;
    constexpr double p3 = 6.61375632143793436117e-05;
    constexpr double p4 = -1.65339022054652515390e-06;
    constexpr double p5 = 4.13813679705723846039e-08;

    const uint64 bits = binary64_bits(x);
    const uint32 sign = static_cast<uint32>(bits >> 63);
    const uint32 high = static_cast<uint32>((bits >> 32) & 0x7fffffffU);
    if (high >= 0x7ff00000U) {
        if (binary64_is_nan_bits(bits))
            return x;
        return sign == 0 ? x : 0.0;
    }
    if (x > overflow_threshold)
        return binary64_infinity(false);
    if (x < underflow_threshold)
        return 0.0;

    int k = 0;
    double high_part = 0.0;
    double low_part = 0.0;
    if (high > 0x3fd62e42U) {
        if (high < 0x3ff0a2b2U) {
            high_part = x - ln2_high[sign];
            low_part = ln2_low[sign];
            k = 1 - static_cast<int>(sign) - static_cast<int>(sign);
        } else {
            k = static_cast<int>(inverse_ln2 * x + half[sign]);
            const double multiple = static_cast<double>(k);
            high_part = x - multiple * ln2_high[0];
            low_part = multiple * ln2_low[0];
        }
        x = high_part - low_part;
    } else if (high < 0x3e300000U) {
        return one + x;
    }

    const double square = x * x;
    const double correction =
        x - square * (p1 + square * (p2 + square * (p3 + square * (p4 + square * p5))));
    if (k == 0)
        return one - ((x * correction) / (correction - 2.0) - x);
    const double reduced = one - ((low_part - (x * correction) / (2.0 - correction)) - high_part);
    return binary64_scale(reduced, k);
}

constexpr double expm1(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::expm1(x);
    }
#endif

    constexpr double one = 1.0;
    constexpr double overflow_threshold = 7.09782712893383973096e+02;
    constexpr double ln2_high = 6.93147180369123816490e-01;
    constexpr double ln2_low = 1.90821492927058770002e-10;
    constexpr double inverse_ln2 = 1.44269504088896338700e+00;
    constexpr double q1 = -3.33333333333331316428e-02;
    constexpr double q2 = 1.58730158725481460165e-03;
    constexpr double q3 = -7.93650757867487942473e-05;
    constexpr double q4 = 4.00821782732936239552e-06;
    constexpr double q5 = -2.01099218183624371326e-07;

    const uint64 bits = binary64_bits(x);
    const bool negative = (bits & binary64_sign_mask) != 0;
    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high >= 0x7ff00000U) {
        if (binary64_is_nan_bits(bits))
            return binary64_quiet_nan(x);
        return negative ? -one : x;
    }
    if (x > overflow_threshold)
        return binary64_infinity(false);
    if (negative && high >= 0x4043687aU)
        return -one;

    int k;
    double correction;
    if (high > 0x3fd62e42U) {
        double high_part;
        double low_part;
        if (high < 0x3ff0a2b2U) {
            if (!negative) {
                high_part = x - ln2_high;
                low_part = ln2_low;
                k = 1;
            } else {
                high_part = x + ln2_high;
                low_part = -ln2_low;
                k = -1;
            }
        } else {
            k = static_cast<int>(inverse_ln2 * x + (negative ? -0.5 : 0.5));
            const double multiple = static_cast<double>(k);
            high_part = x - multiple * ln2_high;
            low_part = multiple * ln2_low;
        }
        x = high_part - low_part;
        correction = (high_part - x) - low_part;
    } else if (high < 0x3c900000U) {
        return x;
    } else {
        k = 0;
        correction = 0.0;
    }

    const double half_x = 0.5 * x;
    const double half_square = x * half_x;
    const double r1_part = one + half_square * q1;
    const double square2 = half_square * half_square;
    const double r2_part = q2 + half_square * q3;
    const double square4 = square2 * square2;
    const double r3_part = q4 + half_square * q5;
    const double r1 = r1_part + square2 * r2_part + square4 * r3_part;
    const double t = 3.0 - r1 * half_x;
    double error = half_square * ((r1 - t) / (6.0 - x * t));
    if (k == 0)
        return x - (x * error - half_square);

    error = x * (error - correction) - correction;
    error -= half_square;
    if (k == -1)
        return 0.5 * (x - error) - 0.5;
    if (k == 1)
        return x < -0.25 ? -2.0 * (error - (x + 0.5)) : one + 2.0 * (x - error);

    if (k <= -2 || k > 56)
        return binary64_scale(one - (error - x), k) - one;

    double result;
    if (k < 20) {
        const uint32 t_high = 0x3ff00000U - (0x00200000U >> k);
        const double t_value = binary64_with_high_word(one, t_high);
        result = t_value - (error - x);
    } else {
        const double t_value = binary64_from_bits(static_cast<uint64>(0x3ff - k) << 52);
        result = one + (x - (error + t_value));
    }
    return binary64_scale(result, k);
}

} // namespace cevalm

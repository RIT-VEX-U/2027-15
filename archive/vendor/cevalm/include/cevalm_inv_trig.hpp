// SPDX-License-Identifier: SunPro

/*
 * Binary64 inverse trigonometric functions derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_sqrt.hpp"

namespace cevalm {

constexpr double atan(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::atan(x);
    }
#endif

    constexpr double high_parts[4] = {4.63647609000806093515e-01, 7.85398163397448278999e-01,
                                      9.82793723247329054082e-01, 1.57079632679489655800e+00};
    constexpr double low_parts[4] = {2.26987774529616870924e-17, 3.06161699786838301793e-17,
                                     1.39033110312309984516e-17, 6.12323399573676603587e-17};
    constexpr double coefficients[11] = {
        3.33333333333329318027e-01,  -1.99999999998764832476e-01, 1.42857142725034663711e-01,
        -1.11111104054623557880e-01, 9.09088713343650656196e-02,  -7.69187620504482999495e-02,
        6.66107313738753120669e-02,  -5.83357013379057348645e-02, 4.97687799461593236017e-02,
        -3.65315727442169155270e-02, 1.62858201153657823623e-02};
    const bool negative = signbit(x);
    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high >= 0x44100000U) {
        if (isnan(x))
            return binary64_quiet_nan(x);
        const double result = high_parts[3] + low_parts[3];
        return negative ? -result : result;
    }
    int interval = -1;
    if (high < 0x3fdc0000U) {
        if (high < 0x3e200000U)
            return x;
    } else {
        x = cevalm::fabs(x);
        if (high < 0x3ff30000U) {
            if (high < 0x3fe60000U) {
                interval = 0;
                x = (2.0 * x - 1.0) / (2.0 + x);
            } else {
                interval = 1;
                x = (x - 1.0) / (x + 1.0);
            }
        } else if (high < 0x40038000U) {
            interval = 2;
            x = (x - 1.5) / (1.0 + 1.5 * x);
        } else {
            interval = 3;
            x = -1.0 / x;
        }
    }
    const double square = x * x;
    const double fourth = square * square;
    const double odd =
        square *
        (coefficients[0] +
         fourth * (coefficients[2] +
                   fourth * (coefficients[4] +
                             fourth * (coefficients[6] +
                                       fourth * (coefficients[8] + fourth * coefficients[10])))));
    const double even =
        fourth * (coefficients[1] +
                  fourth * (coefficients[3] +
                            fourth * (coefficients[5] +
                                      fourth * (coefficients[7] + fourth * coefficients[9]))));
    if (interval < 0)
        return x - x * (odd + even);
    const double result = high_parts[interval] - ((x * (odd + even) - low_parts[interval]) - x);
    return negative ? -result : result;
}

constexpr double atan2(double y, double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::atan2(y, x);
    }
#endif

    constexpr double tiny = 1.0e-300, quarter_pi = 7.8539816339744827900e-01,
                     half_pi = 1.5707963267948965580, pi = 3.1415926535897931160,
                     pi_low = 1.2246467991473531772e-16;
    if (isnan(x) || isnan(y))
        return binary64_propagate_nan(x, y);
    if (binary64_bits(x) == 0x3ff0000000000000ULL)
        return cevalm::atan(y);
    const bool y_negative = signbit(y), x_negative = signbit(x);
    const int quadrant = (y_negative ? 1 : 0) | (x_negative ? 2 : 0);
    const uint64 xmag = binary64_magnitude_bits(x), ymag = binary64_magnitude_bits(y);
    if (ymag == 0) {
        if (quadrant < 2)
            return y;
        return quadrant == 2 ? pi + tiny : -pi - tiny;
    }
    if (xmag == 0)
        return y_negative ? -half_pi - tiny : half_pi + tiny;
    if (xmag == binary64_infinity_bits) {
        if (ymag == binary64_infinity_bits) {
            switch (quadrant) {
            case 0:
                return quarter_pi + tiny;
            case 1:
                return -quarter_pi - tiny;
            case 2:
                return 3.0 * quarter_pi + tiny;
            default:
                return -3.0 * quarter_pi - tiny;
            }
        }
        switch (quadrant) {
        case 0:
            return 0.0;
        case 1:
            return -0.0;
        case 2:
            return pi + tiny;
        default:
            return -pi - tiny;
        }
    }
    if (ymag == binary64_infinity_bits)
        return y_negative ? -half_pi - tiny : half_pi + tiny;
    const int32 high_difference = static_cast<int32>(ymag >> 32) - static_cast<int32>(xmag >> 32);
    const int32 exponent_difference = high_difference >= 0
                                          ? high_difference / (1 << 20)
                                          : -((-high_difference + (1 << 20) - 1) / (1 << 20));
    double angle;
    if (exponent_difference > 60)
        angle = half_pi + 0.5 * pi_low;
    else if (x_negative && exponent_difference < -60)
        angle = 0.0;
    else
        angle = cevalm::atan(cevalm::fabs(y / x));
    switch (quadrant) {
    case 0:
        return angle;
    case 1:
        return -angle;
    case 2:
        return pi - (angle - pi_low);
    default:
        return (angle - pi_low) - pi;
    }
}

constexpr double asin(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::asin(x);
    }
#endif

    constexpr double pio2_high = 1.57079632679489655800, pio2_low = 6.12323399573676603587e-17,
                     pio4_high = 7.85398163397448278999e-01;
    constexpr double p0 = 1.66666666666666657415e-01, p1 = -3.25565818622400915405e-01,
                     p2 = 2.01212532134862925881e-01, p3 = -4.00555345006794114027e-02,
                     p4 = 7.91534994289814532176e-04, p5 = 3.47933107596021167570e-05;
    constexpr double q1 = -2.40339491173441421878, q2 = 2.02094576023350569471,
                     q3 = -6.88283971605453293030e-01, q4 = 7.70381505559019352791e-02;
    const uint64 bits = binary64_bits(x);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    if (high >= 0x3ff00000U) {
        if (magnitude == 0x3ff0000000000000ULL)
            return x * pio2_high + x * pio2_low;
        if (isnan(x))
            return binary64_quiet_nan(x);
        return binary64_from_bits(0xfff8000000000000ULL);
    }
    if (high < 0x3fe00000U) {
        if (high < 0x3e400000U)
            return x;
        const double t = x * x;
        const double numerator = t * (p0 + t * (p1 + t * (p2 + t * (p3 + t * (p4 + t * p5)))));
        const double denominator = 1.0 + t * (q1 + t * (q2 + t * (q3 + t * q4)));
        return x + x * (numerator / denominator);
    }
    double w = 1.0 - cevalm::fabs(x);
    double t = 0.5 * w;
    const double numerator = t * (p0 + t * (p1 + t * (p2 + t * (p3 + t * (p4 + t * p5)))));
    const double denominator = 1.0 + t * (q1 + t * (q2 + t * (q3 + t * q4)));
    const double root = cevalm::sqrt(t);
    if (high >= 0x3fef3333U) {
        w = numerator / denominator;
        t = pio2_high - (2.0 * (root + root * w) - pio2_low);
    } else {
        const double truncated = binary64_from_bits(binary64_bits(root) & 0xffffffff00000000ULL);
        const double correction = (t - truncated * truncated) / (root + truncated);
        const double ratio = numerator / denominator;
        const double adjustment = 2.0 * root * ratio - (pio2_low - 2.0 * correction);
        const double main = pio4_high - 2.0 * truncated;
        t = pio4_high - (adjustment - main);
    }
    return signbit(x) ? -t : t;
}

constexpr double acos(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::acos(x);
    }
#endif

    constexpr double pi = 3.14159265358979311600, pio2_high = 1.57079632679489655800,
                     pio2_low = 6.12323399573676603587e-17;
    constexpr double p0 = 1.66666666666666657415e-01, p1 = -3.25565818622400915405e-01,
                     p2 = 2.01212532134862925881e-01, p3 = -4.00555345006794114027e-02,
                     p4 = 7.91534994289814532176e-04, p5 = 3.47933107596021167570e-05;
    constexpr double q1 = -2.40339491173441421878, q2 = 2.02094576023350569471,
                     q3 = -6.88283971605453293030e-01, q4 = 7.70381505559019352791e-02;
    const uint64 bits = binary64_bits(x);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    if (high >= 0x3ff00000U) {
        if (magnitude == 0x3ff0000000000000ULL)
            return signbit(x) ? pi + 2.0 * pio2_low : 0.0;
        if (isnan(x))
            return binary64_quiet_nan(x);
        return binary64_from_bits(0xfff8000000000000ULL);
    }
    if (high < 0x3fe00000U) {
        if (high <= 0x3c600000U)
            return pio2_high + pio2_low;
        const double z = x * x;
        const double numerator = z * (p0 + z * (p1 + z * (p2 + z * (p3 + z * (p4 + z * p5)))));
        const double denominator = 1.0 + z * (q1 + z * (q2 + z * (q3 + z * q4)));
        return pio2_high - (x - (pio2_low - x * (numerator / denominator)));
    }
    if (signbit(x)) {
        const double z = 0.5 * (1.0 + x);
        const double numerator = z * (p0 + z * (p1 + z * (p2 + z * (p3 + z * (p4 + z * p5)))));
        const double denominator = 1.0 + z * (q1 + z * (q2 + z * (q3 + z * q4)));
        const double root = cevalm::sqrt(z);
        const double correction = (numerator / denominator) * root - pio2_low;
        return pi - 2.0 * (root + correction);
    }
    const double z = 0.5 * (1.0 - x);
    const double root = cevalm::sqrt(z);
    const double truncated = binary64_from_bits(binary64_bits(root) & 0xffffffff00000000ULL);
    const double correction = (z - truncated * truncated) / (root + truncated);
    const double numerator = z * (p0 + z * (p1 + z * (p2 + z * (p3 + z * (p4 + z * p5)))));
    const double denominator = 1.0 + z * (q1 + z * (q2 + z * (q3 + z * q4)));
    return 2.0 * (truncated + ((numerator / denominator) * root + correction));
}

} // namespace cevalm

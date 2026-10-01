// SPDX-License-Identifier: SunPro AND LGPL-2.1-or-later

/*
 * Binary64 powers and roots derived from fdlibm and glibc fdlibm sources.
 *
 * The exp2 and cbrt implementations are Copyright (C) 1997-2013 Free Software
 * Foundation, Inc. and is distributed under the GNU Lesser General Public
 * License, version 2.1 or (at your option) any later version.
 *
 * Other routines retain the Sun fdlibm notice reproduced in
 * LICENSES/SunPro.txt.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_exp.hpp"
#include "cevalm_exp2_table.hpp"
#include "cevalm_scale.hpp"
#include "cevalm_sqrt.hpp"

namespace cevalm {

constexpr int32 power_floor_divide(int32 value, int32 divisor) {
    return value >= 0 ? value / divisor : -((-value + divisor - 1) / divisor);
}

constexpr double exp2(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::exp2(x);
    }
#endif

    if (isnan(x))
        return binary64_quiet_nan(x);
    if (x >= 1024.0)
        return binary64_infinity(false);
    if (x < -1075.0)
        return 0.0;
    if (cevalm::fabs(x) < 0x1p-54)
        return 1.0 + x;
    constexpr double rounding = 13194139533312.0;
    double reduced = x + rounding;
    reduced -= rounding;
    double residual = x - reduced;
    int32 table_value = static_cast<int32>(reduced * 512.0 + 256.0);
    const uint32 index = static_cast<uint32>(table_value) & 511U;
    residual -= exp2_deltatable[index];
    double table = exp2_accuratetable[index];
    table_value = power_floor_divide(table_value, 512);
    const bool unsafe = (table_value < 0 ? -table_value : table_value) >= 965;
    const int32 table_exponent = unsafe ? power_floor_divide(table_value, 2) : table_value;
    const uint32 table_high = binary64_high_word(table);
    const int32 exponent_bits =
        static_cast<int32>(table_high & 0x7ff00000U) + (table_exponent * (1 << 20));
    table = binary64_with_high_word(table, static_cast<uint32>(exponent_bits) |
                                               (table_high & 0x800fffffU));
    const int32 scale_exponent = 1023 + table_value - table_exponent;
    const double scale =
        binary64_from_bits(static_cast<uint64>(static_cast<uint32>(scale_exponent)) << 52);
    const double polynomial =
        (((0.0096181293647031180 * residual + 0.055504110254308625) * residual +
          0.240226506959100583) *
             residual +
         0.69314718055994495) *
        table;
    const double result = polynomial * residual + table;
    return unsafe ? result * scale : result;
}

constexpr double exp10(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::pow(10.0, x);
    }
#endif

    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return signbit(x) ? 0.0 : x;
    if (x < -347.0)
        return 0.0;
    return cevalm::exp(2.30258509299404568402 * x);
}

constexpr double cbrt(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::cbrt(x);
    }
#endif

    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x) || binary64_magnitude_bits(x) == 0)
        return x;
    constexpr double factors[5] = {0.62996052494743658238361, 0.79370052598409973737585, 1.0,
                                   1.2599210498948731647672, 1.5874010519681994747517};
    int exponent = 0;
    const double fraction = cevalm::frexp(cevalm::fabs(x), &exponent);
    const double approximation =
        0.354895765043919860 +
        (1.50819193781584896 +
         (-2.11499494167371287 +
          (2.44693122563534430 +
           (-1.83469277483613086 +
            (0.784932344976639262 - 0.145263899385486377 * fraction) * fraction) *
               fraction) *
              fraction) *
             fraction) *
            fraction;
    const double cube = approximation * approximation * approximation;
    const double refined = approximation * (cube + 2.0 * fraction) / (2.0 * cube + fraction) *
                           factors[2 + exponent % 3];
    return cevalm::ldexp(signbit(x) ? -refined : refined, exponent / 3);
}

constexpr double hypot(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::hypot(x, y);
    }
#endif

    uint64 a_bits = binary64_magnitude_bits(x), b_bits = binary64_magnitude_bits(y);
    uint32 a_high = static_cast<uint32>(a_bits >> 32), b_high = static_cast<uint32>(b_bits >> 32);
    double a = cevalm::fabs(x), b = cevalm::fabs(y);
    if (b_high > a_high) {
        const double value = a;
        a = b;
        b = value;
        const uint32 high = a_high;
        a_high = b_high;
        b_high = high;
        const uint64 bits = a_bits;
        a_bits = b_bits;
        b_bits = bits;
    }
    if (a_high >= 0x7ff00000U) {
        if (a_bits == binary64_infinity_bits || b_bits == binary64_infinity_bits)
            return binary64_infinity(false);
        return binary64_is_nan_bits(a_bits) ? binary64_from_bits(a_bits | binary64_quiet_nan_mask)
                                            : binary64_from_bits(b_bits | binary64_quiet_nan_mask);
    }
    if (a_high - b_high > 0x03c00000U)
        return a + b;
    int scale = 0;
    if (a_high > 0x5f300000U) {
        a_high -= 0x25800000U;
        b_high -= 0x25800000U;
        scale += 600;
        a = binary64_with_high_word(a, a_high);
        b = binary64_with_high_word(b, b_high);
    }
    if (b_high < 0x20b00000U) {
        if (b_high <= 0x000fffffU) {
            if (b_bits == 0)
                return a;
            const double multiplier = binary64_from_bits(0x7fd0000000000000ULL);
            b *= multiplier;
            a *= multiplier;
            scale -= 1022;
            a_high = binary64_high_word(a);
            b_high = binary64_high_word(b);
        } else {
            a_high += 0x25800000U;
            b_high += 0x25800000U;
            scale -= 600;
            a = binary64_with_high_word(a, a_high);
            b = binary64_with_high_word(b, b_high);
        }
    }
    double difference = a - b;
    double result;
    if (difference > b) {
        const double high_part = binary64_from_bits(static_cast<uint64>(a_high) << 32);
        const double low_part = a - high_part;
        result = cevalm::sqrt(high_part * high_part - (b * (-b) - low_part * (a + high_part)));
    } else {
        a = a + a;
        const double b_high_part = binary64_from_bits(static_cast<uint64>(b_high) << 32);
        const double b_low_part = b - b_high_part;
        const double a_high_part =
            binary64_from_bits(static_cast<uint64>(a_high + 0x00100000U) << 32);
        const double a_low_part = a - a_high_part;
        result =
            cevalm::sqrt(a_high_part * b_high_part - (difference * (-difference) -
                                                      (a_high_part * b_low_part + a_low_part * b)));
    }
    return scale == 0 ? result : cevalm::scalbn(result, scale);
}

constexpr int power_integer_kind(double value) {
    const uint64 magnitude = binary64_magnitude_bits(value);
    const int exponent = static_cast<int>((magnitude >> 52) & 0x7ff) - 1023;
    if (exponent < 0)
        return 0;
    if (exponent > 52)
        return 2;
    const uint64 significand = (magnitude & binary64_fraction_mask) | 0x0010000000000000ULL;
    const unsigned shift = static_cast<unsigned>(52 - exponent);
    if (shift != 0 && (significand & ((1ULL << shift) - 1)) != 0)
        return 0;
    return ((significand >> shift) & 1) != 0 ? 1 : 2;
}

constexpr double powi(double x, int exponent) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::pow(x, exponent);
    }
#endif

    const bool odd = (static_cast<unsigned>(exponent) & 1U) != 0;
    if (exponent == 0 || x == 1.0)
        return 1.0;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (binary64_magnitude_bits(x) == 0) {
        if (exponent > 0)
            return odd && signbit(x) ? -0.0 : 0.0;
        return binary64_infinity(odd && signbit(x));
    }
    if (isinf(x)) {
        if (exponent < 0)
            return odd && signbit(x) ? -0.0 : 0.0;
        return binary64_infinity(odd && signbit(x));
    }
    double factor = cevalm::fabs(x);
    unsigned magnitude;
    if (exponent < 0) {
        factor = 1.0 / factor;
        magnitude = 0U - static_cast<unsigned>(exponent);
    } else
        magnitude = static_cast<unsigned>(exponent);
    double result = (magnitude & 1U) != 0 ? factor : 1.0;
    magnitude >>= 1;
    while (magnitude != 0) {
        factor *= factor;
        if ((magnitude & 1U) != 0)
            result *= factor;
        magnitude >>= 1;
    }
    return odd && signbit(x) ? -result : result;
}

constexpr double pow(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::pow(x, y);
    }
#endif

    constexpr double bp[2] = {1.0, 1.5}, dp_high[2] = {0.0, 5.84962487220764160156e-01},
                     dp_low[2] = {0.0, 1.35003920212974897128e-08};
    constexpr double l1 = 5.99999999999994648725e-01, l2 = 4.28571428578550184252e-01,
                     l3 = 3.33333329818377432918e-01, l4 = 2.72728123808534006489e-01,
                     l5 = 2.30660745775561754067e-01, l6 = 2.06975017800338417784e-01;
    constexpr double p1 = 1.66666666666666019037e-01, p2 = -2.77777777770155933842e-03,
                     p3 = 6.61375632143793436117e-05, p4 = -1.65339022054652515390e-06,
                     p5 = 4.13813679705723846039e-08;
    constexpr double log_two = 6.93147180559945286227e-01,
                     log_two_high = 6.93147182464599609375e-01,
                     log_two_low = -1.90465429995776804525e-09,
                     overflow_tail = 8.0085662595372944372e-17;
    constexpr double cp = 9.61796693925975554329e-01, cp_high = 9.61796700954437255859e-01,
                     cp_low = -7.02846165095275826516e-09,
                     inverse_log_two = 1.44269504088896338700e+00,
                     inverse_log_two_high = 1.44269502162933349609e+00,
                     inverse_log_two_low = 1.92596299112661746887e-08;
    const uint64 x_bits = binary64_bits(x), y_bits = binary64_bits(y),
                 x_magnitude = x_bits & ~binary64_sign_mask,
                 y_magnitude = y_bits & ~binary64_sign_mask;
    const uint32 x_high = static_cast<uint32>(x_magnitude >> 32),
                 y_high = static_cast<uint32>(y_magnitude >> 32);
    const bool x_negative = signbit(x), y_negative = signbit(y);
    if (y_magnitude == 0)
        return issignaling(x) ? binary64_from_bits(0x7ff8000000000000ULL) : 1.0;
    if (isnan(x) || isnan(y)) {
        if (x_bits == 0x3ff0000000000000ULL)
            return 1.0;
        return binary64_propagate_nan(x, y);
    }
    const int y_integer = x_negative ? power_integer_kind(y) : 0;
    if (isinf(y)) {
        if (x_magnitude == 0x3ff0000000000000ULL)
            return 1.0;
        if (x_magnitude > 0x3ff0000000000000ULL)
            return y_negative ? 0.0 : binary64_infinity(false);
        return y_negative ? binary64_infinity(false) : 0.0;
    }
    if (y_bits == 0x3ff0000000000000ULL)
        return x;
    if (y_bits == 0xbff0000000000000ULL && x_high >= 0x00100000U)
        return 1.0 / x;
    if (y_bits == 0x4000000000000000ULL && x_high < 0x5ff00000U)
        return x * x;
    if (y_bits == 0x3fe0000000000000ULL && !x_negative)
        return cevalm::sqrt(x);
    const double absolute = cevalm::fabs(x);
    if (x_magnitude == 0 || isinf(x) || x_magnitude == 0x3ff0000000000000ULL) {
        double result = x_magnitude == 0 ? 0.0 : (isinf(x) ? binary64_infinity(false) : 1.0);
        if (y_negative)
            result = result == 0.0 ? binary64_infinity(false) : 0.0;
        if (x_negative) {
            if (x_magnitude == 0x3ff0000000000000ULL && y_integer == 0)
                return binary64_from_bits(0xfff8000000000000ULL);
            if (y_integer == 1)
                result = cevalm::copysign(result, -1.0);
        }
        return result;
    }
    if (x_negative && y_integer == 0)
        return binary64_from_bits(0xfff8000000000000ULL);
    double first_log, second_log;
    if (y_high > 0x41e00000U) {
        if (y_high > 0x43f00000U) {
            if (x_high <= 0x3fefffffU)
                return y_negative ? binary64_infinity(false) : 0.0;
            if (x_high >= 0x3ff00000U)
                return y_negative ? 0.0 : binary64_infinity(false);
        }
        if (x_high < 0x3fefffffU)
            return y_negative ? binary64_infinity(false) : 0.0;
        if (x_high > 0x3ff00000U)
            return y_negative ? 0.0 : binary64_infinity(false);
        const double difference = absolute - 1.0;
        const double correction =
            (difference * difference) *
            (0.5 - difference * (0.3333333333333333333333 - difference * 0.25));
        const double high = inverse_log_two_high * difference;
        const double low = difference * inverse_log_two_low - correction * inverse_log_two;
        first_log = high + low;
        first_log = binary64_from_bits(binary64_bits(first_log) & 0xffffffff00000000ULL);
        second_log = low - (first_log - high);
    } else {
        double normalized = absolute;
        int n = 0;
        uint32 normalized_high = x_high;
        if (normalized_high < 0x00100000U) {
            normalized *= 9007199254740992.0;
            n -= 53;
            normalized_high = binary64_high_word(normalized);
        }
        n += static_cast<int>(normalized_high >> 20) - 1023;
        const uint32 fraction = normalized_high & 0x000fffffU;
        int interval;
        uint32 word = fraction | 0x3ff00000U;
        if (fraction <= 0x3988eU)
            interval = 0;
        else if (fraction < 0xbb67aU)
            interval = 1;
        else {
            interval = 0;
            ++n;
            word -= 0x00100000U;
        }
        normalized = binary64_with_high_word(normalized, word);
        const double difference = normalized - bp[interval],
                     reciprocal = 1.0 / (normalized + bp[interval]),
                     ratio = difference * reciprocal;
        const double ratio_high = binary64_from_bits(binary64_bits(ratio) & 0xffffffff00000000ULL);
        const double sum_high =
            binary64_from_bits(static_cast<uint64>(((word >> 1) | 0x20000000U) + 0x00080000U +
                                                   (static_cast<uint32>(interval) << 18))
                               << 32);
        const double sum_low = normalized - (sum_high - bp[interval]);
        const double ratio_low =
            reciprocal * ((difference - ratio_high * sum_high) - ratio_high * sum_low);
        const double square = ratio * ratio;
        double remainder =
            square * square *
            (l1 + square * (l2 + square * (l3 + square * (l4 + square * (l5 + square * l6)))));
        remainder += ratio_low * (ratio_high + ratio);
        const double high_sum = binary64_from_bits(
            binary64_bits(3.0 + ratio_high * ratio_high + remainder) & 0xffffffff00000000ULL);
        const double low_sum = remainder - ((high_sum - 3.0) - ratio_high * ratio_high);
        const double product_high = ratio_high * high_sum,
                     product_low = ratio_low * high_sum + low_sum * ratio;
        const double combined = product_high + product_low;
        const double combined_high =
            binary64_from_bits(binary64_bits(combined) & 0xffffffff00000000ULL);
        const double combined_low = product_low - (combined_high - product_high);
        const double log_high = cp_high * combined_high,
                     log_low = cp_low * combined_high + combined_low * cp + dp_low[interval];
        const double integer = static_cast<double>(n);
        first_log = ((log_high + log_low) + dp_high[interval]) + integer;
        first_log = binary64_from_bits(binary64_bits(first_log) & 0xffffffff00000000ULL);
        second_log = log_low - (((first_log - integer) - dp_high[interval]) - log_high);
    }
    const double sign = x_negative && y_integer == 1 ? -1.0 : 1.0;
    const double y_high_part = binary64_from_bits(binary64_bits(y) & 0xffffffff00000000ULL);
    double exponent_low = (y - y_high_part) * first_log + y * second_log;
    double exponent_high = y_high_part * first_log;
    double exponent = exponent_low + exponent_high;
    const uint64 exponent_bits = binary64_bits(exponent);
    int32 exponent_word = static_cast<int32>(exponent_bits >> 32);
    const uint32 exponent_low_word = static_cast<uint32>(exponent_bits);
    if (exponent_word >= static_cast<int32>(0x40900000U)) {
        if (exponent_word != static_cast<int32>(0x40900000U) || exponent_low_word != 0 ||
            exponent_low + overflow_tail > exponent - exponent_high)
            return binary64_infinity(signbit(sign));
    } else if ((static_cast<uint32>(exponent_word) & 0x7fffffffU) >= 0x4090cc00U) {
        if (exponent_word != static_cast<int32>(0xc090cc00U) || exponent_low_word != 0 ||
            exponent_low <= exponent - exponent_high)
            return binary64_signed_zero(signbit(sign));
    }
    const uint32 absolute_exponent_word = static_cast<uint32>(exponent_word) & 0x7fffffffU;
    int binary_exponent = static_cast<int>(absolute_exponent_word >> 20) - 1023;
    int adjustment = 0;
    if (absolute_exponent_word > 0x3fe00000U) {
        const int32 rounded_word =
            exponent_word +
            static_cast<int32>(0x00100000U >> static_cast<unsigned>(binary_exponent + 1));
        binary_exponent =
            static_cast<int>((static_cast<uint32>(rounded_word) & 0x7fffffffU) >> 20) - 1023;
        const uint32 truncation_mask = ~(0x000fffffU >> static_cast<unsigned>(binary_exponent));
        const double integral = binary64_from_bits(
            static_cast<uint64>(static_cast<uint32>(rounded_word) & truncation_mask) << 32);
        adjustment =
            static_cast<int>(((static_cast<uint32>(rounded_word) & 0x000fffffU) | 0x00100000U) >>
                             static_cast<unsigned>(20 - binary_exponent));
        if (exponent_word < 0)
            adjustment = -adjustment;
        exponent_high -= integral;
    }
    const double reduced = exponent_low + exponent_high;
    const double reduced_high = binary64_from_bits(binary64_bits(reduced) & 0xffffffff00000000ULL);
    const double first = reduced_high * log_two_high,
                 second = (exponent_low - (reduced_high - exponent_high)) * log_two +
                          reduced_high * log_two_low;
    double value = first + second;
    const double tail = second - (value - first), square = value * value,
                 polynomial =
                     value -
                     square * (p1 + square * (p2 + square * (p3 + square * (p4 + square * p5))));
    const double correction = (value * polynomial) / (polynomial - 2.0) - (tail + value * tail);
    value = 1.0 - (correction - value);
    int32 value_word = static_cast<int32>(binary64_high_word(value)) + adjustment * (1 << 20);
    if ((value_word >> 20) <= 0)
        value = cevalm::scalbn(value, adjustment);
    else
        value = binary64_with_high_word(value, static_cast<uint32>(value_word));
    return sign * value;
}

} // namespace cevalm

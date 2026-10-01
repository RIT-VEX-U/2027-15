// SPDX-License-Identifier: SunPro

/*
 * Binary64 decomposition, scaling, and adjacent-value operations.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_round.hpp"

namespace cevalm {

constexpr uint64 binary64_round_shift_right(uint64 value, int shift) {
    if (shift <= 0)
        return value;
    if (shift >= 54)
        return 0;
    const uint64 quotient = value >> shift;
    const uint64 remainder_mask = (uint64{1} << shift) - 1;
    const uint64 remainder = value & remainder_mask;
    const uint64 halfway = uint64{1} << (shift - 1);
    return quotient + (remainder > halfway || (remainder == halfway && (quotient & 1)));
}

constexpr double binary64_scale(double value, long long power) {
    const uint64 bits = binary64_bits(value);
    const uint64 sign = bits & binary64_sign_mask;
    uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    uint64 significand = bits & binary64_fraction_mask;

    if (exponent_field == 0x7ff)
        return significand == 0 ? value : binary64_quiet_nan(value);
    if (exponent_field == 0 && significand == 0)
        return value;
    if (power > 50000)
        return binary64_infinity(sign != 0);
    if (power < -50000)
        return binary64_signed_zero(sign != 0);

    int exponent;
    if (exponent_field == 0) {
        exponent = -1022;
        while ((significand & (uint64{1} << 52)) == 0) {
            significand <<= 1;
            --exponent;
        }
    } else {
        exponent = static_cast<int>(exponent_field) - 1023;
        significand |= uint64{1} << 52;
    }

    const long long scaled_exponent = static_cast<long long>(exponent) + power;
    if (scaled_exponent > 1023)
        return binary64_infinity(sign != 0);
    if (scaled_exponent >= -1022) {
        exponent_field = static_cast<uint64>(scaled_exponent + 1023);
        return binary64_from_bits(sign | (exponent_field << 52) |
                                  (significand & binary64_fraction_mask));
    }

    const long long wide_shift = -1022 - scaled_exponent;
    if (wide_shift >= 54)
        return binary64_signed_zero(sign != 0);
    const uint64 subnormal = binary64_round_shift_right(significand, static_cast<int>(wide_shift));
    return binary64_from_bits(sign | subnormal);
}

constexpr double scalbn(double value, int power) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::scalbn(value, power);
    }
#endif

    return binary64_scale(value, power);
}

constexpr double scalbln(double value, long power) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::scalbln(value, power);
    }
#endif

    return binary64_scale(value, static_cast<long long>(power));
}

constexpr double ldexp(double value, int power) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::ldexp(value, power);
    }
#endif

    return binary64_scale(value, power);
}

constexpr double frexp(double value, int* exponent) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::frexp(value, exponent);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    uint64 fraction = bits & binary64_fraction_mask;
    *exponent = 0;
    if (magnitude == 0 || exponent_field == 0x7ff)
        return value;

    int normalized_exponent;
    if (exponent_field == 0) {
        normalized_exponent = -1022;
        while ((fraction & (uint64{1} << 52)) == 0) {
            fraction <<= 1;
            --normalized_exponent;
        }
    } else {
        normalized_exponent = static_cast<int>(exponent_field) - 1023;
        fraction |= uint64{1} << 52;
    }
    *exponent = normalized_exponent + 1;
    return binary64_from_bits((bits & binary64_sign_mask) | 0x3fe0000000000000ULL |
                              (fraction & binary64_fraction_mask));
}

constexpr double modf(double value, double* integral) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::modf(value, integral);
    }
#endif

    if (isnan(value)) {
        const double quiet = binary64_quiet_nan(value);
        *integral = quiet;
        return quiet;
    }
    if (isinf(value)) {
        *integral = value;
        return binary64_signed_zero(signbit(value));
    }
    *integral = trunc(value);
    if (*integral == value)
        return binary64_signed_zero(signbit(value));
    return value - *integral;
}

constexpr double nextafter(double from, double toward) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::nextafter(from, toward);
    }
#endif

    if (isnan(from) || isnan(toward))
        return binary64_propagate_nan(from, toward);
    if (from == toward)
        return from;

    uint64 bits = binary64_bits(from);
    if ((bits & ~binary64_sign_mask) == 0)
        return binary64_from_bits((binary64_bits(toward) & binary64_sign_mask) | 1);

    if ((bits & binary64_sign_mask) == 0)
        bits += from < toward ? 1 : uint64(-1);
    else
        bits += from < toward ? uint64(-1) : 1;
    return binary64_from_bits(bits);
}

constexpr double nexttoward(double from, double toward) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::nexttoward(from, toward);
    }
#endif

    return nextafter(from, toward);
}

} // namespace cevalm

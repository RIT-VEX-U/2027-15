// SPDX-License-Identifier: SunPro AND LGPL-2.1-or-later

/*
 * Binary64 remainder functions derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 *
 * remquo is derived from a glibc fdlibm source distributed under the GNU
 * Lesser General Public License, version 2.1 or (at your option) any later.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_scale.hpp"

namespace cevalm {

constexpr int remainder_ilogb(uint64 magnitude) {
    const int encoded = static_cast<int>((magnitude >> 52) & 0x7ff);
    if (encoded != 0)
        return encoded - 1023;
    int position = 0;
    uint64 value = magnitude;
    while ((value >> 1) != 0) {
        value >>= 1;
        ++position;
    }
    return position - 1074;
}

constexpr uint64 remainder_significand(uint64 magnitude, int exponent) {
    if ((magnitude & binary64_exponent_mask) != 0)
        return (magnitude & binary64_fraction_mask) | 0x0010000000000000ULL;
    return magnitude << static_cast<unsigned>(-1022 - exponent);
}

constexpr double fmod(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fmod(x, y);
    }
#endif

    const uint64 x_bits = binary64_bits(x), y_bits = binary64_bits(y),
                 x_magnitude = x_bits & ~binary64_sign_mask,
                 y_magnitude = y_bits & ~binary64_sign_mask;
    const bool negative = (x_bits & binary64_sign_mask) != 0;
    if (y_magnitude == 0 || binary64_is_infinite_bits(x_bits) || binary64_is_nan_bits(x_bits) ||
        binary64_is_nan_bits(y_bits))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (x_magnitude < y_magnitude)
        return x;
    if (x_magnitude == y_magnitude)
        return binary64_signed_zero(negative);
    int x_exponent = remainder_ilogb(x_magnitude), y_exponent = remainder_ilogb(y_magnitude);
    uint64 x_significand = remainder_significand(x_magnitude, x_exponent),
           y_significand = remainder_significand(y_magnitude, y_exponent);
    int shifts = x_exponent - y_exponent;
    while (shifts-- != 0) {
        if (x_significand >= y_significand) {
            x_significand -= y_significand;
            if (x_significand == 0)
                return binary64_signed_zero(negative);
        }
        x_significand <<= 1;
    }
    if (x_significand >= y_significand)
        x_significand -= y_significand;
    if (x_significand == 0)
        return binary64_signed_zero(negative);
    while (x_significand < 0x0010000000000000ULL) {
        x_significand <<= 1;
        --y_exponent;
    }
    uint64 result;
    if (y_exponent >= -1022)
        result = (static_cast<uint64>(y_exponent + 1023) << 52) |
                 (x_significand & binary64_fraction_mask);
    else
        result = x_significand >> static_cast<unsigned>(-1022 - y_exponent);
    return binary64_from_bits(result | (negative ? binary64_sign_mask : 0));
}

constexpr double remainder(double x, double divisor) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::remainder(x, divisor);
    }
#endif

    const uint64 x_bits = binary64_bits(x), divisor_bits = binary64_bits(divisor),
                 x_magnitude = x_bits & ~binary64_sign_mask,
                 divisor_magnitude = divisor_bits & ~binary64_sign_mask;
    const bool negative = signbit(x);
    if (divisor_magnitude == 0 || !isfinite(x) || isnan(divisor))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (x_magnitude == divisor_magnitude)
        return binary64_signed_zero(negative);
    if (static_cast<uint32>(divisor_magnitude >> 32) <= 0x7fdfffffU)
        x = cevalm::fmod(x, divisor + divisor);
    x = cevalm::fabs(x);
    divisor = cevalm::fabs(divisor);
    if (static_cast<uint32>(divisor_magnitude >> 32) < 0x00200000U) {
        if (x + x > divisor) {
            x -= divisor;
            if (x + x >= divisor)
                x -= divisor;
        }
    } else {
        const double half = 0.5 * divisor;
        if (x > half) {
            x -= divisor;
            if (x >= half)
                x -= divisor;
        }
    }
    return negative ? -x : x;
}

constexpr double remquo(double x, double divisor, int* quotient) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::remquo(x, divisor, quotient);
    }
#endif

    const uint64 x_bits = binary64_bits(x), divisor_bits = binary64_bits(divisor),
                 x_magnitude = x_bits & ~binary64_sign_mask,
                 divisor_magnitude = divisor_bits & ~binary64_sign_mask;
    const bool x_negative = signbit(x), quotient_negative = x_negative != signbit(divisor);
    if (divisor_magnitude == 0 || !isfinite(x) || isnan(divisor))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (x_magnitude == divisor_magnitude) {
        *quotient = quotient_negative ? -1 : 1;
        return binary64_signed_zero(x_negative);
    }
    if (static_cast<uint32>(divisor_magnitude >> 32) <= 0x7fbfffffU)
        x = cevalm::fmod(x, cevalm::scalbn(divisor, 3));
    x = cevalm::fabs(x);
    divisor = cevalm::fabs(divisor);
    int result_quotient = 0;
    const double four = cevalm::scalbn(divisor, 2);
    if (x >= four) {
        x -= four;
        result_quotient += 4;
    }
    const double two = divisor + divisor;
    if (x >= two) {
        x -= two;
        result_quotient += 2;
    }
    if (static_cast<uint32>(divisor_magnitude >> 32) < 0x00200000U) {
        if (x + x > divisor) {
            x -= divisor;
            ++result_quotient;
            if (x + x >= divisor) {
                x -= divisor;
                ++result_quotient;
            }
        }
    } else {
        const double half = 0.5 * divisor;
        if (x > half) {
            x -= divisor;
            ++result_quotient;
            if (x >= half) {
                x -= divisor;
                ++result_quotient;
            }
        }
    }
    *quotient = quotient_negative ? -result_quotient : result_quotient;
    return x_negative ? -x : x;
}

} // namespace cevalm

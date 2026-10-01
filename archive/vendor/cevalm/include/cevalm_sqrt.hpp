// SPDX-License-Identifier: SunPro

/*
 * Correctly rounded square root derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"

namespace cevalm {

constexpr double sqrt(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::sqrt(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    if (binary64_is_nan_bits(bits))
        return binary64_quiet_nan(value);
    if (magnitude == binary64_infinity_bits)
        return (bits & binary64_sign_mask) == 0 ? value : binary64_from_bits(0xfff8000000000000ULL);
    if (magnitude == 0)
        return value;
    if ((bits & binary64_sign_mask) != 0)
        return binary64_from_bits(0xfff8000000000000ULL);

    constexpr uint32 sign = 0x80000000U;
    uint32 high = static_cast<uint32>(bits >> 32);
    uint32 low = static_cast<uint32>(bits);
    int exponent = static_cast<int>(high >> 20);

    if (exponent == 0) {
        while (high == 0) {
            exponent -= 21;
            high |= low >> 11;
            low <<= 21;
        }
        int shifts = 0;
        while ((high & 0x00100000U) == 0) {
            high <<= 1;
            ++shifts;
        }
        exponent -= shifts - 1;
        if (shifts != 0) {
            high |= low >> (32 - shifts);
            low <<= shifts;
        }
    }

    exponent -= 1023;
    high = (high & 0x000fffffU) | 0x00100000U;
    if ((exponent & 1) != 0) {
        high += high + ((low & sign) >> 31);
        low += low;
    }
    exponent >>= 1;

    high += high + ((low & sign) >> 31);
    low += low;

    uint32 quotient_high = 0;
    uint32 quotient_low = 0;
    uint32 sum_high = 0;
    uint32 sum_low = 0;

    for (uint32 moving = 0x00200000U; moving != 0; moving >>= 1) {
        const uint32 trial = sum_high + moving;
        if (trial <= high) {
            sum_high = trial + moving;
            high -= trial;
            quotient_high += moving;
        }
        high += high + ((low & sign) >> 31);
        low += low;
    }

    for (uint32 moving = sign; moving != 0; moving >>= 1) {
        const uint32 trial_low = sum_low + moving;
        const uint32 trial_high = sum_high;
        if (trial_high < high || (trial_high == high && trial_low <= low)) {
            sum_low = trial_low + moving;
            if ((trial_low & sign) != 0 && (sum_low & sign) == 0)
                ++sum_high;
            high -= trial_high;
            if (low < trial_low)
                --high;
            low -= trial_low;
            quotient_low += moving;
        }
        high += high + ((low & sign) >> 31);
        low += low;
    }

    if ((high | low) != 0)
        quotient_low += quotient_low & 1U;

    uint32 result_high = (quotient_high >> 1) + 0x3fe00000U;
    uint32 result_low = quotient_low >> 1;
    if ((quotient_high & 1U) != 0)
        result_low |= sign;
    result_high = static_cast<uint32>(static_cast<int64>(result_high) +
                                      static_cast<int64>(exponent) * 0x00100000LL);
    return binary64_from_bits((static_cast<uint64>(result_high) << 32) | result_low);
}

} // namespace cevalm

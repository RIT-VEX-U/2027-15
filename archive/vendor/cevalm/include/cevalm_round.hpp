// SPDX-License-Identifier: SunPro AND LGPL-2.1-or-later

/*
 * Bit-level rounding derived from fdlibm's binary64 routines.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 * lrint, llrint, lround, and llround include material derived from GNU C
 * Library sources distributed under LGPL-2.1-or-later.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"

namespace cevalm {

constexpr int binary64_unbiased_exponent(uint64 bits) {
    return static_cast<int>((bits & binary64_exponent_mask) >> 52) - 1023;
}

constexpr double trunc(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::trunc(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    if (exponent_field == 0x7ff)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(value) : value;

    const int exponent = static_cast<int>(exponent_field) - 1023;
    if (exponent < 0)
        return binary64_signed_zero((bits & binary64_sign_mask) != 0);
    if (exponent >= 52)
        return value;

    const uint64 fractional_mask = (uint64{1} << (52 - exponent)) - 1;
    if ((magnitude & fractional_mask) == 0)
        return value;
    return binary64_from_bits(bits & ~fractional_mask);
}

constexpr double floor(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::floor(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const bool negative = (bits & binary64_sign_mask) != 0;
    const uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    if (exponent_field == 0x7ff)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(value) : value;

    const int exponent = static_cast<int>(exponent_field) - 1023;
    if (exponent < 0) {
        if (magnitude == 0)
            return value;
        return negative ? -1.0 : 0.0;
    }
    if (exponent >= 52)
        return value;

    const uint64 unit = uint64{1} << (52 - exponent);
    const uint64 fractional_mask = unit - 1;
    if ((bits & fractional_mask) == 0)
        return value;
    uint64 result = bits & ~fractional_mask;
    if (negative)
        result += unit;
    return binary64_from_bits(result);
}

constexpr double ceil(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::ceil(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const bool negative = (bits & binary64_sign_mask) != 0;
    const uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    if (exponent_field == 0x7ff)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(value) : value;

    const int exponent = static_cast<int>(exponent_field) - 1023;
    if (exponent < 0) {
        if (magnitude == 0)
            return value;
        return negative ? binary64_signed_zero(true) : 1.0;
    }
    if (exponent >= 52)
        return value;

    const uint64 unit = uint64{1} << (52 - exponent);
    const uint64 fractional_mask = unit - 1;
    if ((bits & fractional_mask) == 0)
        return value;
    uint64 result = bits & ~fractional_mask;
    if (!negative)
        result += unit;
    return binary64_from_bits(result);
}

constexpr double round(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::round(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const uint64 exponent_field = (bits & binary64_exponent_mask) >> 52;
    if (exponent_field == 0x7ff)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(value) : value;

    const int exponent = static_cast<int>(exponent_field) - 1023;
    if (exponent < -1)
        return binary64_signed_zero((bits & binary64_sign_mask) != 0);
    if (exponent == -1)
        return binary64_from_bits((bits & binary64_sign_mask) | 0x3ff0000000000000ULL);
    if (exponent >= 52)
        return value;

    const uint64 unit = uint64{1} << (52 - exponent);
    const uint64 fractional_mask = unit - 1;
    if ((magnitude & fractional_mask) == 0)
        return value;
    const uint64 half = unit >> 1;
    uint64 result = bits & ~fractional_mask;
    if ((magnitude & fractional_mask) >= half)
        result += unit;
    return binary64_from_bits(result);
}

constexpr double rint(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::rint(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 magnitude = bits & ~binary64_sign_mask;
    const uint64 exponent_field = (magnitude & binary64_exponent_mask) >> 52;
    if (exponent_field == 0x7ff)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(value) : value;
    const int exponent = static_cast<int>(exponent_field) - 1023;
    if (exponent < -1)
        return binary64_signed_zero(signbit(value));
    if (exponent == -1) {
        if (magnitude == 0x3fe0000000000000ULL)
            return binary64_signed_zero(signbit(value));
        return binary64_from_bits((bits & binary64_sign_mask) | 0x3ff0000000000000ULL);
    }
    if (exponent >= 52)
        return value;
    const uint64 unit = uint64{1} << (52 - exponent);
    const uint64 mask = unit - 1;
    const uint64 fraction = magnitude & mask;
    if (fraction == 0)
        return value;
    uint64 result = bits & ~mask;
    const uint64 half = unit >> 1;
    if (fraction > half || (fraction == half && (magnitude & unit) != 0))
        result += unit;
    return binary64_from_bits(result);
}

constexpr double nearbyint(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::nearbyint(value);
    }
#endif

    return cevalm::rint(value);
}

constexpr long lrint(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::lrint(value);
    }
#endif

    const double rounded = cevalm::rint(value);
    if (!isfinite(rounded) || rounded >= 0x1p31 || rounded < -0x1p31)
        return (-2147483647L - 1L);
    return static_cast<long>(rounded);
}

constexpr long long llrint(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::llrint(value);
    }
#endif

    const double rounded = cevalm::rint(value);
    if (!isfinite(rounded) || rounded >= 0x1p63 || rounded < -0x1p63)
        return (-9223372036854775807LL - 1LL);
    return static_cast<long long>(rounded);
}

constexpr long lround(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::lround(value);
    }
#endif

    const double rounded = cevalm::round(value);
    if (!isfinite(rounded) || rounded >= 0x1p31 || rounded < -0x1p31)
        return (-2147483647L - 1L);
    return static_cast<long>(rounded);
}

constexpr long long llround(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::llround(value);
    }
#endif

    const double rounded = cevalm::round(value);
    if (!isfinite(rounded) || rounded >= 0x1p63 || rounded < -0x1p63)
        return (-9223372036854775807LL - 1LL);
    return static_cast<long long>(rounded);
}

} // namespace cevalm

// SPDX-License-Identifier: LicenseRef-Red-Hat-FDLIBM AND LGPL-2.1-or-later

/*
 * Copyright (C) 2002, 2007 by Red Hat, Incorporated. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"

namespace cevalm {

inline constexpr int fp_nan = 0;
inline constexpr int fp_infinite = 1;
inline constexpr int fp_zero = 2;
inline constexpr int fp_subnormal = 3;
inline constexpr int fp_normal = 4;

constexpr int fpclassify(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fpclassify(value);
    }
#endif

    const uint64 bits = binary64_bits(value);
    const uint64 exponent = bits & binary64_exponent_mask;
    const uint64 fraction = bits & binary64_fraction_mask;
    if (exponent == binary64_exponent_mask)
        return fraction == 0 ? fp_infinite : fp_nan;
    if (exponent == 0)
        return fraction == 0 ? fp_zero : fp_subnormal;
    return fp_normal;
}

constexpr bool isfinite(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isfinite(value);
    }
#endif

    return (binary64_bits(value) & binary64_exponent_mask) != binary64_exponent_mask;
}

constexpr bool isinf(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isinf(value);
    }
#endif

    return binary64_is_infinite_bits(binary64_bits(value));
}

constexpr bool isnan(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isnan(value);
    }
#endif

    return binary64_is_nan_bits(binary64_bits(value));
}

constexpr bool isnormal(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isnormal(value);
    }
#endif

    return fpclassify(value) == fp_normal;
}

constexpr bool signbit(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::signbit(value);
    }
#endif

    return (binary64_bits(value) & binary64_sign_mask) != 0;
}

constexpr bool isgreater(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isgreater(x, y);
    }
#endif

    return !isnan(x) && !isnan(y) && x > y;
}

constexpr bool isgreaterequal(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isgreaterequal(x, y);
    }
#endif

    return !isnan(x) && !isnan(y) && x >= y;
}

constexpr bool isless(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isless(x, y);
    }
#endif

    return !isnan(x) && !isnan(y) && x < y;
}

constexpr bool islessequal(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::islessequal(x, y);
    }
#endif

    return !isnan(x) && !isnan(y) && x <= y;
}

constexpr bool islessgreater(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::islessgreater(x, y);
    }
#endif

    return !isnan(x) && !isnan(y) && x != y;
}

constexpr bool isunordered(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isunordered(x, y);
    }
#endif

    return isnan(x) || isnan(y);
}

constexpr bool issignaling(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::isnan(value) && (binary64_bits(value) & binary64_quiet_nan_mask) == 0;
    }
#endif

    const uint64 bits = binary64_bits(value);
    return binary64_is_nan_bits(bits) && (bits & binary64_quiet_nan_mask) == 0;
}

} // namespace cevalm

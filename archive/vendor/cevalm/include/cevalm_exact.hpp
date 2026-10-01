// SPDX-License-Identifier: SunPro AND LGPL-2.1-or-later

/*
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 * fdim, fmin, and fmax include material derived from GNU C Library sources
 * distributed under LGPL-2.1-or-later.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"

namespace cevalm {

constexpr double fabs(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fabs(value);
    }
#endif

    return binary64_from_bits(binary64_bits(value) & ~binary64_sign_mask);
}

constexpr double abs(double value) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::abs(value);
    }
#endif

    return cevalm::fabs(value);
}

constexpr double copysign(double magnitude, double sign) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::copysign(magnitude, sign);
    }
#endif

    return binary64_from_bits((binary64_bits(magnitude) & ~binary64_sign_mask) |
                              (binary64_bits(sign) & binary64_sign_mask));
}

constexpr double fmin(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fmin(x, y);
    }
#endif

    if (isnan(y))
        return x;
    if (isnan(x))
        return y;
    return x <= y ? x : y;
}

constexpr double fmax(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fmax(x, y);
    }
#endif

    if (isnan(y))
        return x;
    if (isnan(x))
        return y;
    return x >= y ? x : y;
}

constexpr double fdim(double x, double y) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fdim(x, y);
    }
#endif

    if (isnan(x) || isnan(y))
        return binary64_propagate_nan(x, y);
    return x <= y ? 0.0 : x - y;
}

} // namespace cevalm

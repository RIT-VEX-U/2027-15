// SPDX-License-Identifier: SunPro

/*
 * Binary64 hyperbolic functions derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_exp.hpp"
#include "cevalm_log.hpp"
#include "cevalm_sqrt.hpp"

namespace cevalm {

constexpr double sinh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::sinh(x);
    }
#endif

    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    const bool negative = signbit(x);
    const double absolute = cevalm::fabs(x);
    const double half = negative ? -0.5 : 0.5;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return x;
    if (high < 0x40360000U) {
        if (high < 0x3e300000U)
            return x;
        const double t = cevalm::expm1(absolute);
        if (high < 0x3ff00000U)
            return half * (2.0 * t - t * t / (t + 1.0));
        return half * (t + t / (t + 1.0));
    }
    if (high < 0x40862e42U)
        return half * cevalm::exp(absolute);
    if (high < 0x408633ceU ||
        (high == 0x408633ceU && static_cast<uint32>(magnitude) <= 0x8fb9f87dU)) {
        const double w = cevalm::exp(0.5 * absolute);
        const double t = half * w;
        return t * w;
    }
    return binary64_infinity(negative);
}

constexpr double cosh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::cosh(x);
    }
#endif

    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    const double absolute = cevalm::fabs(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return binary64_infinity(false);
    if (high < 0x40360000U) {
        if (high < 0x3fd62e43U) {
            const double t = cevalm::expm1(absolute);
            const double w = 1.0 + t;
            if (high < 0x3c800000U)
                return w;
            return 1.0 + t * t / (w + w);
        }
        const double t = cevalm::exp(absolute);
        return 0.5 * t + 0.5 / t;
    }
    if (high < 0x40862e42U)
        return 0.5 * cevalm::exp(absolute);
    if (high < 0x408633ceU ||
        (high == 0x408633ceU && static_cast<uint32>(magnitude) <= 0x8fb9f87dU)) {
        const double w = cevalm::exp(0.5 * absolute);
        const double t = 0.5 * w;
        return t * w;
    }
    return binary64_infinity(false);
}

constexpr double tanh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::tanh(x);
    }
#endif

    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    const bool negative = signbit(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return negative ? -1.0 : 1.0;
    if (magnitude == 0)
        return x;
    double result;
    if (high < 0x40360000U) {
        if (high < 0x3c800000U)
            return x * (1.0 + x);
        if (high >= 0x3ff00000U) {
            const double t = cevalm::expm1(2.0 * cevalm::fabs(x));
            result = 1.0 - 2.0 / (t + 2.0);
        } else {
            const double t = cevalm::expm1(-2.0 * cevalm::fabs(x));
            result = -t / (t + 2.0);
        }
    } else
        result = 1.0 - 1.0e-300;
    return negative ? -result : result;
}

constexpr double asinh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::asinh(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    const bool negative = signbit(x);
    const double absolute = cevalm::fabs(x);
    double result;
    if (high < 0x3e300000U)
        return x;
    if (high > 0x41b00000U) {
        if (isnan(x))
            return binary64_quiet_nan(x);
        if (isinf(x))
            return x;
        result = cevalm::log(absolute) + 6.93147180559945286227e-01;
    } else if (high > 0x40000000U)
        result = cevalm::log(2.0 * absolute +
                             1.0 / (cevalm::sqrt(absolute * absolute + 1.0) + absolute));
    else {
        const double square = absolute * absolute;
        result = cevalm::log1p(absolute + square / (1.0 + cevalm::sqrt(1.0 + square)));
    }
    return negative ? -result : result;
}

constexpr double acosh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::acosh(x);
    }
#endif

    const uint64 bits = binary64_bits(x);
    const uint32 high = binary64_high_word(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (signbit(x) || high < 0x3ff00000U)
        return binary64_from_bits(0xfff8000000000000ULL);
    if (high >= 0x41b00000U) {
        if (isinf(x))
            return x;
        return cevalm::log(x) + 6.93147180559945286227e-01;
    }
    if (bits == 0x3ff0000000000000ULL)
        return 0.0;
    if (high > 0x40000000U) {
        const double square = x * x;
        return cevalm::log(2.0 * x - 1.0 / (x + cevalm::sqrt(square - 1.0)));
    }
    const double difference = x - 1.0;
    return cevalm::log1p(difference + cevalm::sqrt(2.0 * difference + difference * difference));
}

constexpr double atanh(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::atanh(x);
    }
#endif

    const uint64 magnitude = binary64_magnitude_bits(x);
    const uint32 high = static_cast<uint32>(magnitude >> 32);
    const bool negative = signbit(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (magnitude > 0x3ff0000000000000ULL)
        return binary64_from_bits(0xfff8000000000000ULL);
    if (magnitude == 0x3ff0000000000000ULL)
        return binary64_infinity(negative);
    if (high < 0x3e300000U)
        return x;
    const double absolute = cevalm::fabs(x);
    double result;
    if (high < 0x3fe00000U) {
        const double twice = absolute + absolute;
        result = 0.5 * cevalm::log1p(twice + twice * absolute / (1.0 - absolute));
    } else
        result = 0.5 * cevalm::log1p((absolute + absolute) / (1.0 - absolute));
    return negative ? -result : result;
}

} // namespace cevalm

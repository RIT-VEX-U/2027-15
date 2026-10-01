// SPDX-License-Identifier: SunPro

/*
 * Binary64 logarithms derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_scale.hpp"

namespace cevalm {

constexpr double log(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::log(x);
    }
#endif

    constexpr double ln2_high = 6.93147180369123816490e-01;
    constexpr double ln2_low = 1.90821492927058770002e-10;
    constexpr double lg1 = 6.666666666666735130e-01;
    constexpr double lg2 = 3.999999999940941908e-01;
    constexpr double lg3 = 2.857142874366239149e-01;
    constexpr double lg4 = 2.222219843214978396e-01;
    constexpr double lg5 = 1.818357216161805012e-01;
    constexpr double lg6 = 1.531383769920937332e-01;
    constexpr double lg7 = 1.479819860511658591e-01;

    const uint64 original_bits = binary64_bits(x);
    const uint64 magnitude = original_bits & ~binary64_sign_mask;
    if (magnitude == 0)
        return binary64_infinity(true);
    if ((original_bits & binary64_sign_mask) != 0)
        return binary64_is_nan_bits(original_bits) ? binary64_quiet_nan(x)
                                                   : binary64_from_bits(0xfff8000000000000ULL);
    if (binary64_is_nan_bits(original_bits))
        return binary64_quiet_nan(x);
    if (binary64_is_infinite_bits(original_bits))
        return x;

    int k = 0;
    uint32 high = binary64_high_word(x);
    if (high < 0x00100000U) {
        k -= 54;
        x = binary64_scale(x, 54);
        high = binary64_high_word(x);
    }

    k += static_cast<int>(high >> 20) - 1023;
    high &= 0x000fffffU;
    const uint32 adjustment = (high + 0x00095f64U) & 0x00100000U;
    x = binary64_with_high_word(x, high | (adjustment ^ 0x3ff00000U));
    k += static_cast<int>(adjustment >> 20);
    const double f = x - 1.0;
    if ((0x000fffffU & (2U + high)) < 3U) {
        if (f == 0.0) {
            if (k == 0)
                return 0.0;
            const double dk = static_cast<double>(k);
            return dk * ln2_high + dk * ln2_low;
        }
        const double remainder = f * f * (0.5 - 0.33333333333333333 * f);
        if (k == 0)
            return f - remainder;
        const double dk = static_cast<double>(k);
        return dk * ln2_high - ((remainder - dk * ln2_low) - f);
    }

    const double s = f / (2.0 + f);
    const double dk = static_cast<double>(k);
    const double z = s * s;
    int32 indicator = static_cast<int32>(high) - 0x0006147a;
    const double w = z * z;
    const int32 other = 0x0006b851 - static_cast<int32>(high);
    const double t1 = w * (lg2 + w * (lg4 + w * lg6));
    const double t2 = z * (lg1 + w * (lg3 + w * (lg5 + w * lg7)));
    indicator |= other;
    const double remainder = t2 + t1;
    if (indicator > 0) {
        const double half_square = 0.5 * f * f;
        if (k == 0)
            return f - (half_square - s * (half_square + remainder));
        return dk * ln2_high - ((half_square - (s * (half_square + remainder) + dk * ln2_low)) - f);
    }
    if (k == 0)
        return f - s * (f - remainder);
    return dk * ln2_high - ((s * (f - remainder) - dk * ln2_low) - f);
}

constexpr double log1p(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::log1p(x);
    }
#endif

    constexpr double ln2_high = 6.93147180369123816490e-01;
    constexpr double ln2_low = 1.90821492927058770002e-10;
    constexpr double lp1 = 6.666666666666735130e-01;
    constexpr double lp2 = 3.999999999940941908e-01;
    constexpr double lp3 = 2.857142874366239149e-01;
    constexpr double lp4 = 2.222219843214978396e-01;
    constexpr double lp5 = 1.818357216161805012e-01;
    constexpr double lp6 = 1.531383769920937332e-01;
    constexpr double lp7 = 1.479819860511658591e-01;

    const uint64 bits = binary64_bits(x);
    if (binary64_is_nan_bits(bits))
        return binary64_quiet_nan(x);
    if (binary64_is_infinite_bits(bits))
        return signbit(x) ? binary64_from_bits(0xfff8000000000000ULL) : x;
    if (x == -1.0)
        return binary64_infinity(true);
    if (x < -1.0)
        return binary64_from_bits(0xfff8000000000000ULL);

    const int32 signed_high = static_cast<int32>(binary64_high_word(x));
    const uint32 absolute_high = binary64_high_word(cevalm::fabs(x));
    int k = 1;
    int32 normalized_high = 0;
    double f = 0.0;
    double correction = 0.0;
    if (signed_high < 0x3fda827a) {
        if (absolute_high < 0x3e200000U) {
            if (absolute_high < 0x3c900000U)
                return x;
            return x - x * x * 0.5;
        }
        if (signed_high > 0 || signed_high <= static_cast<int32>(0xbfd2bec3U)) {
            k = 0;
            f = x;
            normalized_high = 1;
        }
    }

    if (k != 0) {
        double normalized;
        if (absolute_high < 0x43400000U) {
            normalized = 1.0 + x;
            normalized_high = static_cast<int32>(binary64_high_word(normalized));
            k = (normalized_high >> 20) - 1023;
            correction = k > 0 ? 1.0 - (normalized - x) : x - (normalized - 1.0);
            correction /= normalized;
        } else {
            normalized = x;
            normalized_high = static_cast<int32>(binary64_high_word(normalized));
            k = (normalized_high >> 20) - 1023;
        }
        normalized_high &= 0x000fffff;
        if (normalized_high < 0x0006a09e) {
            normalized = binary64_with_high_word(normalized, static_cast<uint32>(normalized_high) |
                                                                 0x3ff00000U);
        } else {
            ++k;
            normalized = binary64_with_high_word(normalized, static_cast<uint32>(normalized_high) |
                                                                 0x3fe00000U);
            normalized_high = (0x00100000 - normalized_high) >> 2;
        }
        f = normalized - 1.0;
    }

    const double half_square = 0.5 * f * f;
    if (normalized_high == 0) {
        if (f == 0.0) {
            if (k == 0)
                return 0.0;
            correction += static_cast<double>(k) * ln2_low;
            return static_cast<double>(k) * ln2_high + correction;
        }
        const double remainder = half_square * (1.0 - 0.66666666666666666 * f);
        if (k == 0)
            return f - remainder;
        return static_cast<double>(k) * ln2_high -
               ((remainder - (static_cast<double>(k) * ln2_low + correction)) - f);
    }

    const double s = f / (2.0 + f);
    const double z = s * s;
    const double r1 = z * lp1;
    const double z2 = z * z;
    const double r2 = lp2 + z * lp3;
    const double z4 = z2 * z2;
    const double r3 = lp4 + z * lp5;
    const double z6 = z4 * z2;
    const double r4 = lp6 + z * lp7;
    const double remainder = r1 + z2 * r2 + z4 * r3 + z6 * r4;
    if (k == 0)
        return f - (half_square - s * (half_square + remainder));
    return static_cast<double>(k) * ln2_high -
           ((half_square -
             (s * (half_square + remainder) + (static_cast<double>(k) * ln2_low + correction))) -
            f);
}

constexpr double log2(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::log2(x);
    }
#endif

    constexpr double ln2 = 0.69314718055994530942;
    constexpr double lg1 = 6.666666666666735130e-01;
    constexpr double lg2 = 3.999999999940941908e-01;
    constexpr double lg3 = 2.857142874366239149e-01;
    constexpr double lg4 = 2.222219843214978396e-01;
    constexpr double lg5 = 1.818357216161805012e-01;
    constexpr double lg6 = 1.531383769920937332e-01;
    constexpr double lg7 = 1.479819860511658591e-01;
    const uint64 bits = binary64_bits(x);
    if ((bits & ~binary64_sign_mask) == 0)
        return binary64_infinity(true);
    if ((bits & binary64_sign_mask) != 0)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(x)
                                          : binary64_from_bits(0xfff8000000000000ULL);
    if (binary64_is_nan_bits(bits))
        return binary64_quiet_nan(x);
    if (binary64_is_infinite_bits(bits))
        return x;

    int k = 0;
    uint32 high = binary64_high_word(x);
    if (high < 0x00100000U) {
        k -= 54;
        x = binary64_scale(x, 54);
        high = binary64_high_word(x);
    }
    k += static_cast<int>(high >> 20) - 1023;
    high &= 0x000fffffU;
    const uint32 adjustment = (high + 0x00095f64U) & 0x00100000U;
    x = binary64_with_high_word(x, high | (adjustment ^ 0x3ff00000U));
    k += static_cast<int>(adjustment >> 20);
    const double dk = static_cast<double>(k);
    const double f = x - 1.0;
    if ((0x000fffffU & (2U + high)) < 3U) {
        if (f == 0.0)
            return dk;
        const double remainder = f * f * (0.5 - 0.33333333333333333 * f);
        return dk - (remainder - f) / ln2;
    }
    const double s = f / (2.0 + f);
    const double z = s * s;
    int32 indicator = static_cast<int32>(high) - 0x0006147a;
    const double w = z * z;
    const int32 other = 0x0006b851 - static_cast<int32>(high);
    const double t1 = w * (lg2 + w * (lg4 + w * lg6));
    const double t2 = z * (lg1 + w * (lg3 + w * (lg5 + w * lg7)));
    indicator |= other;
    const double remainder = t2 + t1;
    if (indicator > 0) {
        const double half_square = 0.5 * f * f;
        return dk - ((half_square - s * (half_square + remainder)) - f) / ln2;
    }
    return dk - (s * (f - remainder) - f) / ln2;
}

constexpr double log10(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::log10(x);
    }
#endif

    constexpr double inverse_ln10 = 4.34294481903251816668e-01;
    constexpr double log10_2_high = 3.01029995663611771306e-01;
    constexpr double log10_2_low = 3.69423907715893078616e-13;

    const uint64 bits = binary64_bits(x);
    if ((bits & ~binary64_sign_mask) == 0)
        return binary64_infinity(true);
    if ((bits & binary64_sign_mask) != 0)
        return binary64_is_nan_bits(bits) ? binary64_quiet_nan(x)
                                          : binary64_from_bits(0xfff8000000000000ULL);
    if (binary64_is_nan_bits(bits))
        return binary64_quiet_nan(x);
    if (binary64_is_infinite_bits(bits))
        return x;

    int k = 0;
    uint32 high = binary64_high_word(x);
    if (high < 0x00100000U) {
        k -= 54;
        x = binary64_scale(x, 54);
        high = binary64_high_word(x);
    }
    k += static_cast<int>(high >> 20) - 1023;
    const int adjustment = k < 0 ? 1 : 0;
    high = (high & 0x000fffffU) | (static_cast<uint32>(0x3ff - adjustment) << 20);
    const double y = static_cast<double>(k + adjustment);
    x = binary64_with_high_word(x, high);
    const double z = y * log10_2_low + inverse_ln10 * log(x);
    return z + y * log10_2_high;
}

} // namespace cevalm

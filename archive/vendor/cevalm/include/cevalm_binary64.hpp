// SPDX-License-Identifier: LGPL-2.1-or-later

/*
 * Binary64 support for cevalm.
 *
 * cevalm intentionally targets Clang on ARMv7-A EABI and has no runtime or
 * standard-library dependency.
 */
#pragma once

#include "cevalm_config.hpp"

#ifndef __clang__
#error "cevalm requires Clang"
#endif

static_assert(__clang_major__ >= 22, "cevalm requires Clang 22 or newer");
static_assert(sizeof(double) == 8, "cevalm requires 64-bit double");
static_assert(__FLT_RADIX__ == 2, "cevalm requires radix-2 floating point");
static_assert(__DBL_MANT_DIG__ == 53, "cevalm requires IEEE-754 binary64");
static_assert(__DBL_MAX_EXP__ == 1024, "cevalm requires IEEE-754 binary64");
static_assert(sizeof(long double) == 8, "cevalm targets the ARM EABI binary64 long double ABI");
static_assert(__LDBL_MANT_DIG__ == 53, "cevalm targets the ARM EABI binary64 long double ABI");
static_assert(sizeof(int) == 4 && sizeof(long) == 4,
              "cevalm targets the 32-bit ARM EABI integer model");
static_assert(sizeof(long long) == 8, "cevalm requires a 64-bit long long");

namespace cevalm {

using uint64 = __UINT64_TYPE__;
using int64 = __INT64_TYPE__;
using uint32 = __UINT32_TYPE__;
using int32 = __INT32_TYPE__;

inline constexpr uint64 binary64_sign_mask = 0x8000000000000000ULL;
inline constexpr uint64 binary64_exponent_mask = 0x7ff0000000000000ULL;
inline constexpr uint64 binary64_fraction_mask = 0x000fffffffffffffULL;
inline constexpr uint64 binary64_quiet_nan_mask = 0x0008000000000000ULL;
inline constexpr uint64 binary64_infinity_bits = 0x7ff0000000000000ULL;

constexpr uint64 binary64_bits(double value) {
    return __builtin_bit_cast(uint64, value);
}

constexpr double binary64_from_bits(uint64 bits) {
    return __builtin_bit_cast(double, bits);
}

constexpr uint32 binary64_high_word(double value) {
    return static_cast<uint32>(binary64_bits(value) >> 32);
}

constexpr uint32 binary64_low_word(double value) {
    return static_cast<uint32>(binary64_bits(value));
}

constexpr double binary64_with_high_word(double value, uint32 high) {
    return binary64_from_bits((static_cast<uint64>(high) << 32) | binary64_low_word(value));
}

constexpr uint64 binary64_magnitude_bits(double value) {
    return binary64_bits(value) & ~binary64_sign_mask;
}

constexpr bool binary64_is_nan_bits(uint64 bits) {
    return (bits & binary64_exponent_mask) == binary64_exponent_mask &&
           (bits & binary64_fraction_mask) != 0;
}

constexpr bool binary64_is_infinite_bits(uint64 bits) {
    return (bits & ~binary64_sign_mask) == binary64_infinity_bits;
}

constexpr double binary64_quiet_nan(double value) {
    return binary64_from_bits(binary64_bits(value) | binary64_quiet_nan_mask);
}

constexpr double binary64_propagate_nan(double first, double second) {
    const uint64 first_bits = binary64_bits(first);
    return binary64_is_nan_bits(first_bits)
               ? binary64_from_bits(first_bits | binary64_quiet_nan_mask)
               : binary64_from_bits(binary64_bits(second) | binary64_quiet_nan_mask);
}

constexpr double binary64_signed_zero(bool negative) {
    return binary64_from_bits(negative ? binary64_sign_mask : 0);
}

constexpr double binary64_infinity(bool negative) {
    return binary64_from_bits(binary64_infinity_bits | (negative ? binary64_sign_mask : 0));
}

constexpr double nan(const char* tag) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::nan(tag);
    }
#endif

    if (tag == nullptr || *tag == '\0')
        return binary64_from_bits(0x7ff8000000000000ULL);
    unsigned base = 10;
    if (tag[0] == '0' && (tag[1] == 'x' || tag[1] == 'X')) {
        base = 16;
        tag += 2;
        if (*tag == '\0')
            return binary64_from_bits(0x7ff8000000000000ULL);
    }
    uint64 payload = 0;
    for (; *tag != '\0'; ++tag) {
        unsigned digit;
        if (*tag >= '0' && *tag <= '9')
            digit = static_cast<unsigned>(*tag - '0');
        else if (*tag >= 'a' && *tag <= 'f')
            digit = static_cast<unsigned>(*tag - 'a') + 10U;
        else if (*tag >= 'A' && *tag <= 'F')
            digit = static_cast<unsigned>(*tag - 'A') + 10U;
        else
            return binary64_from_bits(0x7ff8000000000000ULL);
        if (digit >= base)
            return binary64_from_bits(0x7ff8000000000000ULL);
        payload = (payload * base + digit) & binary64_fraction_mask;
    }
    return binary64_from_bits(binary64_infinity_bits | binary64_quiet_nan_mask | payload);
}

} // namespace cevalm

// SPDX-License-Identifier: LGPL-2.1-or-later

#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"

namespace cevalm {

struct fma_integer {
    uint64 limb[67];
};

constexpr fma_integer fma_zero_integer() {
    fma_integer result{};
    return result;
}

constexpr void fma_add_word(fma_integer* value, unsigned index, uint64 word) {
    while (word != 0 && index < 67) {
        const uint64 previous = value->limb[index];
        value->limb[index] += word;
        word = value->limb[index] < previous ? 1 : 0;
        ++index;
    }
}

constexpr void fma_add_shifted(fma_integer* value, uint64 significand, unsigned shift) {
    const unsigned index = shift >> 6;
    const unsigned offset = shift & 63U;
    if (offset == 0) {
        fma_add_word(value, index, significand);
        return;
    }
    fma_add_word(value, index, significand << offset);
    fma_add_word(value, index + 1, significand >> (64U - offset));
}

constexpr int fma_compare(const fma_integer& first, const fma_integer& second) {
    for (int i = 66; i >= 0; --i) {
        if (first.limb[i] != second.limb[i])
            return first.limb[i] > second.limb[i] ? 1 : -1;
    }
    return 0;
}

constexpr fma_integer fma_subtract(const fma_integer& larger, const fma_integer& smaller) {
    fma_integer result{};
    uint64 borrow = 0;
    for (unsigned i = 0; i < 67; ++i) {
        const uint64 difference = larger.limb[i] - smaller.limb[i];
        const uint64 first_borrow = larger.limb[i] < smaller.limb[i] ? 1 : 0;
        result.limb[i] = difference - borrow;
        const uint64 second_borrow = difference < borrow ? 1 : 0;
        borrow = first_borrow | second_borrow;
    }
    return result;
}

constexpr int fma_high_bit(const fma_integer& value) {
    for (int i = 66; i >= 0; --i) {
        if (value.limb[i] != 0) {
            uint64 word = value.limb[i];
            int bit = 0;
            while ((word >>= 1) != 0)
                ++bit;
            return i * 64 + bit;
        }
    }
    return -1;
}

constexpr bool fma_bit(const fma_integer& value, unsigned bit) {
    return ((value.limb[bit >> 6] >> (bit & 63U)) & 1U) != 0;
}

constexpr bool fma_any_below(const fma_integer& value, unsigned bit) {
    const unsigned index = bit >> 6;
    for (unsigned i = 0; i < index; ++i) {
        if (value.limb[i] != 0)
            return true;
    }
    const unsigned offset = bit & 63U;
    if (offset == 0)
        return false;
    return (value.limb[index] & ((uint64{1} << offset) - 1U)) != 0;
}

constexpr uint64 fma_shift_to_word(const fma_integer& value, unsigned shift) {
    const unsigned index = shift >> 6;
    const unsigned offset = shift & 63U;
    uint64 result = value.limb[index] >> offset;
    if (offset != 0 && index + 1 < 67)
        result |= value.limb[index + 1] << (64U - offset);
    return result;
}

constexpr double fma_round(const fma_integer& magnitude, bool negative) {
    constexpr int storage_exponent = -2148;
    int high = fma_high_bit(magnitude);
    if (high < 0)
        return binary64_signed_zero(negative);
    if (high + storage_exponent > 1023)
        return binary64_infinity(negative);

    unsigned shift = high + storage_exponent >= -1022 ? static_cast<unsigned>(high - 52) : 1074U;
    uint64 rounded = fma_shift_to_word(magnitude, shift);
    const bool halfway = fma_bit(magnitude, shift - 1U);
    if (halfway && (fma_any_below(magnitude, shift - 1U) || (rounded & 1U) != 0))
        ++rounded;

    if (high + storage_exponent < -1022)
        return binary64_from_bits((negative ? binary64_sign_mask : 0) + rounded);

    if (rounded == (uint64{1} << 53)) {
        rounded >>= 1;
        ++high;
        if (high + storage_exponent > 1023)
            return binary64_infinity(negative);
    }
    const uint64 exponent = static_cast<uint64>(high + storage_exponent + 1023);
    return binary64_from_bits((negative ? binary64_sign_mask : 0) | (exponent << 52) |
                              (rounded & binary64_fraction_mask));
}

constexpr uint64 fma_significand(uint64 bits) {
    const uint64 fraction = bits & binary64_fraction_mask;
    return (bits & binary64_exponent_mask) == 0 ? fraction : fraction | (uint64{1} << 52);
}

constexpr int fma_exponent(uint64 bits) {
    const int encoded = static_cast<int>((bits & binary64_exponent_mask) >> 52);
    return encoded == 0 ? -1074 : encoded - 1023 - 52;
}

constexpr double fma(double x, double y, double z) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::fma(x, y, z);
    }
#endif

    const uint64 x_bits = binary64_bits(x);
    const uint64 y_bits = binary64_bits(y);
    const uint64 z_bits = binary64_bits(z);
    if (isnan(x) || isnan(y))
        return binary64_propagate_nan(x, y);
    if (isnan(z))
        return binary64_quiet_nan(z);

    const bool product_negative = signbit(x) != signbit(y);
    const bool z_negative = signbit(z);
    const bool x_infinite = isinf(x);
    const bool y_infinite = isinf(y);
    const bool z_infinite = isinf(z);
    const bool x_zero = binary64_magnitude_bits(x) == 0;
    const bool y_zero = binary64_magnitude_bits(y) == 0;
    if ((x_infinite && y_zero) || (y_infinite && x_zero))
        return binary64_from_bits(0xfff8000000000000ULL);
    if (x_infinite || y_infinite) {
        if (z_infinite && product_negative != z_negative)
            return binary64_from_bits(0xfff8000000000000ULL);
        return binary64_infinity(product_negative);
    }
    if (z_infinite)
        return z;

    const uint64 x_significand = fma_significand(x_bits);
    const uint64 y_significand = fma_significand(y_bits);
    const uint64 z_significand = fma_significand(z_bits);
    if ((x_significand == 0 || y_significand == 0) && z_significand == 0)
        return binary64_signed_zero(product_negative == z_negative && product_negative);

    fma_integer product = fma_zero_integer();
    const unsigned product_shift =
        static_cast<unsigned>(fma_exponent(x_bits) + fma_exponent(y_bits) + 2148);
    for (unsigned bit = 0; bit < 53; ++bit) {
        if (((x_significand >> bit) & 1U) != 0)
            fma_add_shifted(&product, y_significand, product_shift + bit);
    }
    fma_integer addend = fma_zero_integer();
    fma_add_shifted(&addend, z_significand, static_cast<unsigned>(fma_exponent(z_bits) + 2148));

    if (product_negative == z_negative) {
        for (unsigned i = 0; i < 67; ++i)
            fma_add_word(&product, i, addend.limb[i]);
        return fma_round(product, product_negative);
    }
    const int comparison = fma_compare(product, addend);
    if (comparison == 0)
        return 0.0;
    return comparison > 0 ? fma_round(fma_subtract(product, addend), product_negative)
                          : fma_round(fma_subtract(addend, product), z_negative);
}

} // namespace cevalm

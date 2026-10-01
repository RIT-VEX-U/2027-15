// SPDX-License-Identifier: SunPro AND LGPL-2.1-or-later

/*
 * Binary64 special functions derived from fdlibm.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 * The tgamma implementation is derived from a glibc fdlibm source distributed
 * under the GNU Lesser General Public License, version 2.1 or later.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_exp.hpp"
#include "cevalm_log.hpp"
#include "cevalm_power.hpp"
#include "cevalm_round.hpp"
#include "cevalm_scale.hpp"
#include "cevalm_sqrt.hpp"
#include "cevalm_trig.hpp"

namespace cevalm {

inline constexpr double erf_pp[5] = {1.28379167095512558561e-01, -3.25042107247001499370e-01,
                                     -2.84817495755985104766e-02, -5.77027029648944159157e-03,
                                     -2.37630166566501626084e-05};
inline constexpr double erf_qq[5] = {3.97917223959155352819e-01, 6.50222499887672944485e-02,
                                     5.08130628187576562776e-03, 1.32494738004321644526e-04,
                                     -3.96022827877536812320e-06};
inline constexpr double erf_pa[7] = {-2.36211856075265944077e-03, 4.14856118683748331666e-01,
                                     -3.72207876035701323847e-01, 3.18346619901161753674e-01,
                                     -1.10894694282396677476e-01, 3.54783043256182359371e-02,
                                     -2.16637559486879084300e-03};
inline constexpr double erf_qa[6] = {1.06420880400844228286e-01, 5.40397917702171048937e-01,
                                     7.18286544141962662868e-02, 1.26171219808761642112e-01,
                                     1.36370839120290507362e-02, 1.19844998467991074170e-02};
inline constexpr double erf_ra[8] = {-9.86494403484714822705e-03, -6.93858572707181764372e-01,
                                     -1.05586262253232909814e+01, -6.23753324503260060396e+01,
                                     -1.62396669462573470355e+02, -1.84605092906711035994e+02,
                                     -8.12874355063065934246e+01, -9.81432934416914548592e+00};
inline constexpr double erf_sa[8] = {1.96512716674392571292e+01, 1.37657754143519042600e+02,
                                     4.34565877475229228821e+02, 6.45387271733267880336e+02,
                                     4.29008140027567833386e+02, 1.08635005541779435134e+02,
                                     6.57024977031928170135e+00, -6.04244152148580987438e-02};
inline constexpr double erf_rb[7] = {-9.86494292470009928597e-03, -7.99283237680523006574e-01,
                                     -1.77579549177547519889e+01, -1.60636384855821916062e+02,
                                     -6.37566443368389627722e+02, -1.02509513161107724954e+03,
                                     -4.83519191608651397019e+02};
inline constexpr double erf_sb[7] = {3.03380607434824582924e+01, 3.25792512996573918826e+02,
                                     1.53672958608443695994e+03, 3.19985821950859553908e+03,
                                     2.55305040643316442583e+03, 4.74528541206955367215e+02,
                                     -2.24409524465858183362e+01};

constexpr double erf_small_ratio(double square) {
    const double square2 = square * square, square4 = square2 * square2;
    const double numerator = (erf_pp[0] + square * erf_pp[1]) +
                             square2 * (erf_pp[2] + square * erf_pp[3]) + square4 * erf_pp[4];
    const double denominator = (1.0 + square * erf_qq[0]) +
                               square2 * (erf_qq[1] + square * erf_qq[2]) +
                               square4 * (erf_qq[3] + square * erf_qq[4]);
    return numerator / denominator;
}
constexpr double erf_middle_ratio(double offset) {
    const double square = offset * offset, fourth = square * square, sixth = fourth * square;
    const double numerator = (erf_pa[0] + offset * erf_pa[1]) +
                             square * (erf_pa[2] + offset * erf_pa[3]) +
                             fourth * (erf_pa[4] + offset * erf_pa[5]) + sixth * erf_pa[6];
    const double denominator = (1.0 + offset * erf_qa[0]) +
                               square * (erf_qa[1] + offset * erf_qa[2]) +
                               fourth * (erf_qa[3] + offset * erf_qa[4]) + sixth * erf_qa[5];
    return numerator / denominator;
}
constexpr double erf_tail_ratio(double reciprocal_square, bool first) {
    const double square = reciprocal_square * reciprocal_square, fourth = square * square,
                 sixth = fourth * square;
    if (first) {
        const double eighth = fourth * fourth;
        const double numerator = (erf_ra[0] + reciprocal_square * erf_ra[1]) +
                                 square * (erf_ra[2] + reciprocal_square * erf_ra[3]) +
                                 fourth * (erf_ra[4] + reciprocal_square * erf_ra[5]) +
                                 sixth * (erf_ra[6] + reciprocal_square * erf_ra[7]);
        const double denominator = (1.0 + reciprocal_square * erf_sa[0]) +
                                   square * (erf_sa[1] + reciprocal_square * erf_sa[2]) +
                                   fourth * (erf_sa[3] + reciprocal_square * erf_sa[4]) +
                                   sixth * (erf_sa[5] + reciprocal_square * erf_sa[6]) +
                                   eighth * erf_sa[7];
        return numerator / denominator;
    }
    const double numerator = (erf_rb[0] + reciprocal_square * erf_rb[1]) +
                             square * (erf_rb[2] + reciprocal_square * erf_rb[3]) +
                             fourth * (erf_rb[4] + reciprocal_square * erf_rb[5]) +
                             sixth * erf_rb[6];
    const double denominator = (1.0 + reciprocal_square * erf_sb[0]) +
                               square * (erf_sb[1] + reciprocal_square * erf_sb[2]) +
                               fourth * (erf_sb[3] + reciprocal_square * erf_sb[4]) +
                               sixth * (erf_sb[5] + reciprocal_square * erf_sb[6]);
    return numerator / denominator;
}

constexpr double erf(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::erf(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    const bool negative = signbit(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return negative ? -1.0 : 1.0;
    if (high < 0x3feb0000U) {
        if (high < 0x3e300000U) {
            if (high < 0x00800000U)
                return 0.125 * (8.0 * x + 1.02703333676410069053 * x);
            return x + 1.28379167095512586316e-01 * x;
        }
        const double ratio = erf_small_ratio(x * x);
        return x + x * ratio;
    }
    if (high < 0x3ff40000U) {
        const double ratio = erf_middle_ratio(cevalm::fabs(x) - 1.0);
        return negative ? -8.45062911510467529297e-01 - ratio : 8.45062911510467529297e-01 + ratio;
    }
    if (high >= 0x40180000U)
        return negative ? -1.0 : 1.0;
    const double absolute = cevalm::fabs(x), inverse_square = 1.0 / (absolute * absolute),
                 ratio = erf_tail_ratio(inverse_square, high < 0x4006db6eU);
    const double truncated = binary64_from_bits(binary64_bits(absolute) & 0xffffffff00000000ULL);
    const double tail = cevalm::exp(-truncated * truncated - 0.5625) *
                        cevalm::exp((truncated - absolute) * (truncated + absolute) + ratio);
    return negative ? tail / absolute - 1.0 : 1.0 - tail / absolute;
}

constexpr double erfc(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::erfc(x);
    }
#endif

    const uint32 high = binary64_high_word(cevalm::fabs(x));
    const bool negative = signbit(x);
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return negative ? 2.0 : 0.0;
    if (high < 0x3feb0000U) {
        if (high < 0x3c700000U)
            return 1.0 - x;
        const double ratio = erf_small_ratio(x * x);
        if (static_cast<int32>(binary64_high_word(x)) < static_cast<int32>(0x3fd00000U))
            return 1.0 - (x + x * ratio);
        double correction = x * ratio;
        correction += x - 0.5;
        return 0.5 - correction;
    }
    if (high < 0x3ff40000U) {
        const double ratio = erf_middle_ratio(cevalm::fabs(x) - 1.0);
        if (!negative)
            return (1.0 - 8.45062911510467529297e-01) - ratio;
        return 1.0 + (8.45062911510467529297e-01 + ratio);
    }
    if (high >= 0x403c0000U)
        return negative ? 2.0 : 0.0;
    if (negative && high >= 0x40180000U)
        return 2.0;
    const double absolute = cevalm::fabs(x), inverse_square = 1.0 / (absolute * absolute),
                 ratio = erf_tail_ratio(inverse_square, high < 0x4006db6dU);
    const double truncated = binary64_from_bits(binary64_bits(absolute) & 0xffffffff00000000ULL);
    const double tail = cevalm::exp(-truncated * truncated - 0.5625) *
                        cevalm::exp((truncated - absolute) * (truncated + absolute) + ratio);
    return negative ? 2.0 - tail / absolute : tail / absolute;
}

inline constexpr double lgamma_a[12] = {
    7.72156649015328655494e-02, 3.22467033424113591611e-01, 6.73523010531292681824e-02,
    2.05808084325167332806e-02, 7.38555086081402883957e-03, 2.89051383673415629091e-03,
    1.19270763183362067845e-03, 5.10069792153511336608e-04, 2.20862790713908385557e-04,
    1.08011567247583939954e-04, 2.52144565451257326939e-05, 4.48640949618915160150e-05};
inline constexpr double lgamma_t[15] = {
    4.83836122723810047042e-01,  -1.47587722994593911752e-01, 6.46249402391333854778e-02,
    -3.27885410759859649565e-02, 1.79706750811820387126e-02,  -1.03142241298341437450e-02,
    6.10053870246291332635e-03,  -3.68452016781138256760e-03, 2.25964780900612472250e-03,
    -1.40346469989232843813e-03, 8.81081882437654011382e-04,  -5.38595305356740546715e-04,
    3.15632070903625950361e-04,  -3.12754168375120860518e-04, 3.35529192635519073543e-04};
inline constexpr double lgamma_u[6] = {-7.72156649015328655494e-02, 6.32827064025093366517e-01,
                                       1.45492250137234768737e+00,  9.77717527963372745603e-01,
                                       2.28963728064692451092e-01,  1.33810918536787660377e-02};
inline constexpr double lgamma_v[5] = {2.45597793713041134822e+00, 2.12848976379893395361e+00,
                                       7.69285150456672783825e-01, 1.04222645593369134254e-01,
                                       3.21709242282423911810e-03};
inline constexpr double lgamma_s[7] = {-7.72156649015328655494e-02, 2.14982415960608852501e-01,
                                       3.25778796408930981787e-01,  1.46350472652464452805e-01,
                                       2.66422703033638609560e-02,  1.84028451407337715652e-03,
                                       3.19475326584100867617e-05};
inline constexpr double lgamma_den[6] = {1.39200533467621045958e+00, 7.21935547567138069525e-01,
                                         1.71933865632803078993e-01, 1.86459191715652901344e-02,
                                         7.77942496381893596434e-04, 7.32668430744625636189e-06};
inline constexpr double lgamma_w[7] = {4.18938533204672725052e-01,  8.33333333333329678849e-02,
                                       -2.77777777728775536470e-03, 7.93650558643019558500e-04,
                                       -5.95187557450339963135e-04, 8.36339918996282139126e-04,
                                       -1.63092934096575273989e-03};

constexpr double lgamma_sin_pi(double x) {
    constexpr double pi = 3.14159265358979311600e+00;
    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high < 0x3fd00000U)
        return cevalm::sin(pi * x);
    double y = -x;
    int n;
    const double integral = cevalm::floor(y);
    if (integral != y) {
        y *= 0.5;
        y = 2.0 * (y - cevalm::floor(y));
        n = static_cast<int>(y * 4.0);
    } else if (high >= 0x43400000U) {
        y = 0.0;
        n = 0;
    } else {
        double parity_source = y;
        if (high < 0x43300000U)
            parity_source = y + 4503599627370496.0;
        n = static_cast<int>(binary64_low_word(parity_source) & 1U);
        y = static_cast<double>(n);
        n <<= 2;
    }
    double result;
    switch (n) {
    case 0:
        result = kernel_sin(pi * y, 0.0, 0);
        break;
    case 1:
    case 2:
        result = kernel_cos(pi * (0.5 - y), 0.0);
        break;
    case 3:
    case 4:
        result = kernel_sin(pi * (1.0 - y), 0.0, 0);
        break;
    case 5:
    case 6:
        result = -kernel_cos(pi * (y - 1.5), 0.0);
        break;
    default:
        result = kernel_sin(pi * (y - 2.0), 0.0, 0);
        break;
    }
    return -result;
}

constexpr double lgamma_r_value(double x, int* sign_gamma) {
    constexpr double pi = 3.14159265358979311600e+00, tc = 1.46163214496836224576,
                     tf = -1.21486290535849611461e-01, tt = -3.63867699703950536541e-18;
    const uint64 original_bits = binary64_bits(x), magnitude = original_bits & ~binary64_sign_mask;
    const uint32 high = static_cast<uint32>(magnitude >> 32), low = static_cast<uint32>(magnitude);
    const bool negative = signbit(x);
    *sign_gamma = 1;
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (isinf(x))
        return binary64_infinity(false);
    if (magnitude == 0) {
        if (negative)
            *sign_gamma = -1;
        return binary64_infinity(false);
    }
    if (high < 0x3b900000U) {
        if (negative) {
            *sign_gamma = -1;
            return -cevalm::log(-x);
        }
        return -cevalm::log(x);
    }
    double adjustment = 0.0;
    if (negative) {
        if (high >= 0x43300000U)
            return binary64_infinity(true);
        const double sine = lgamma_sin_pi(x);
        if (sine == 0.0)
            return binary64_infinity(false);
        adjustment = cevalm::log(pi / cevalm::fabs(sine * x));
        if (sine < 0.0)
            *sign_gamma = -1;
        x = -x;
    }
    double result;
    if ((high == 0x3ff00000U && low == 0) || (high == 0x40000000U && low == 0))
        result = 0.0;
    else if (high < 0x40000000U) {
        double y;
        int interval;
        if (high <= 0x3fecccccU) {
            result = -cevalm::log(x);
            if (high >= 0x3fe76944U) {
                y = 1.0 - x;
                interval = 0;
            } else if (high >= 0x3fcda661U) {
                y = x - (tc - 1.0);
                interval = 1;
            } else {
                y = x;
                interval = 2;
            }
        } else {
            result = 0.0;
            if (high >= 0x3ffbb4c3U) {
                y = 2.0 - x;
                interval = 0;
            } else if (high >= 0x3ff3b4c4U) {
                y = x - tc;
                interval = 1;
            } else {
                y = x - 1.0;
                interval = 2;
            }
        }
        if (interval == 0) {
            const double square = y * y;
            const double first =
                lgamma_a[0] +
                square * (lgamma_a[2] +
                          square * (lgamma_a[4] +
                                    square * (lgamma_a[6] +
                                              square * (lgamma_a[8] + square * lgamma_a[10]))));
            const double second =
                square *
                (lgamma_a[1] +
                 square * (lgamma_a[3] +
                           square * (lgamma_a[5] +
                                     square * (lgamma_a[7] +
                                               square * (lgamma_a[9] + square * lgamma_a[11])))));
            result += (y * first + second) - 0.5 * y;
        } else if (interval == 1) {
            const double square = y * y, cube = square * y;
            const double first =
                lgamma_t[0] +
                cube * (lgamma_t[3] +
                        cube * (lgamma_t[6] + cube * (lgamma_t[9] + cube * lgamma_t[12])));
            const double second =
                lgamma_t[1] +
                cube * (lgamma_t[4] +
                        cube * (lgamma_t[7] + cube * (lgamma_t[10] + cube * lgamma_t[13])));
            const double third =
                lgamma_t[2] +
                cube * (lgamma_t[5] +
                        cube * (lgamma_t[8] + cube * (lgamma_t[11] + cube * lgamma_t[14])));
            const double polynomial = square * first - (tt - cube * (second + y * third));
            result += tf + polynomial;
        } else {
            const double numerator =
                y *
                (lgamma_u[0] +
                 y * (lgamma_u[1] +
                      y * (lgamma_u[2] + y * (lgamma_u[3] + y * (lgamma_u[4] + y * lgamma_u[5])))));
            const double denominator =
                1.0 +
                y * (lgamma_v[0] +
                     y * (lgamma_v[1] + y * (lgamma_v[2] + y * (lgamma_v[3] + y * lgamma_v[4]))));
            result += -0.5 * y + numerator / denominator;
        }
    } else if (high < 0x40200000U) {
        const int integer = static_cast<int>(x);
        const double y = x - static_cast<double>(integer);
        const double numerator =
            y * (lgamma_s[0] +
                 y * (lgamma_s[1] +
                      y * (lgamma_s[2] +
                           y * (lgamma_s[3] +
                                y * (lgamma_s[4] + y * (lgamma_s[5] + y * lgamma_s[6]))))));
        const double denominator =
            1.0 + y * (lgamma_den[0] +
                       y * (lgamma_den[1] +
                            y * (lgamma_den[2] +
                                 y * (lgamma_den[3] + y * (lgamma_den[4] + y * lgamma_den[5])))));
        result = 0.5 * y + numerator / denominator;
        double product = 1.0;
        for (int factor = 2; factor <= integer - 1; ++factor)
            product *= y + static_cast<double>(factor);
        if (integer >= 3)
            result += cevalm::log(product);
    } else if (high < 0x43900000U) {
        const double logarithm = cevalm::log(x), inverse = 1.0 / x, square = inverse * inverse;
        const double correction =
            lgamma_w[0] +
            inverse *
                (lgamma_w[1] +
                 square * (lgamma_w[2] +
                           square * (lgamma_w[3] +
                                     square * (lgamma_w[4] +
                                               square * (lgamma_w[5] + square * lgamma_w[6])))));
        result = (x - 0.5) * (logarithm - 1.0) + correction;
    } else
        result = x * (cevalm::log(x) - 1.0);
    return negative ? adjustment - result : result;
}

constexpr double lgamma_r(double x, int* sign_gamma) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        *sign_gamma = std::signbit(x) ? -1 : 1;
        return std::lgamma(x);
    }
#endif

    return lgamma_r_value(x, sign_gamma);
}
constexpr double lgamma(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::lgamma(x);
    }
#endif

    int sign_gamma = 1;
    return lgamma_r_value(x, &sign_gamma);
}

inline constexpr double gamma_coefficients[6] = {0x1.5555555555555p-4,  -0xb.60b60b60b60b8p-12,
                                                 0x3.4034034034034p-12, -0x2.7027027027028p-12,
                                                 0x3.72a3c5631fe46p-12, -0x7.daac36664f1f4p-12};

constexpr void gamma_multiply_split(double* high, double* low, double x, double y) {
    *high = x * y;
    constexpr double splitter = 134217729.0;
    const double x_product = x * splitter, y_product = y * splitter;
    const double x_high = (x - x_product) + x_product;
    const double y_high = (y - y_product) + y_product;
    const double x_low = x - x_high, y_low = y - y_high;
    *low = ((x_high * y_high - *high) + x_high * y_low + x_low * y_high) + x_low * y_low;
}
constexpr double gamma_product(double x, double x_error, int count, double* error) {
    double result = x;
    *error = x_error / x;
    for (int i = 1; i < count; ++i) {
        const double factor = x + static_cast<double>(i);
        *error += x_error / factor;
        double high, low;
        gamma_multiply_split(&high, &low, result, factor);
        result = high;
        *error += low / result;
    }
    return result;
}
constexpr double gamma_positive(double x, int* exponent_adjustment) {
    int ignored_sign = 1;
    if (x < 0.5) {
        *exponent_adjustment = 0;
        return cevalm::exp(lgamma_r_value(x + 1.0, &ignored_sign)) / x;
    }
    if (x <= 1.5) {
        *exponent_adjustment = 0;
        return cevalm::exp(lgamma_r_value(x, &ignored_sign));
    }
    if (x < 6.5) {
        const int count = static_cast<int>(cevalm::ceil(x - 1.5));
        const double adjusted = x - static_cast<double>(count);
        double error;
        const double product = gamma_product(adjusted, 0.0, count, &error);
        *exponent_adjustment = 0;
        return cevalm::exp(lgamma_r_value(adjusted, &ignored_sign)) * product * (1.0 + error);
    }
    double error = 0.0, x_error = 0.0, adjusted = x, product = 1.0;
    if (x < 12.0) {
        const int count = static_cast<int>(cevalm::ceil(12.0 - x));
        adjusted = x + static_cast<double>(count);
        x_error = x - (adjusted - static_cast<double>(count));
        product = gamma_product(adjusted - static_cast<double>(count), x_error, count, &error);
    }
    double exponent_error = -error;
    const double adjusted_integer = cevalm::round(adjusted),
                 adjusted_fraction = adjusted - adjusted_integer;
    int adjusted_log2;
    double adjusted_mantissa = cevalm::frexp(adjusted, &adjusted_log2);
    if (adjusted_mantissa < 7.07106781186547524401e-01) {
        --adjusted_log2;
        adjusted_mantissa *= 2.0;
    }
    *exponent_adjustment = adjusted_log2 * static_cast<int>(adjusted_integer);
    double result = cevalm::pow(adjusted_mantissa, adjusted) *
                    cevalm::exp2(static_cast<double>(adjusted_log2) * adjusted_fraction) *
                    cevalm::exp(-adjusted) * cevalm::sqrt(6.28318530717958647692 / adjusted) /
                    product;
    exponent_error += x_error * cevalm::log(adjusted);
    double sum = gamma_coefficients[5], square = adjusted * adjusted;
    for (int i = 1; i <= 5; ++i)
        sum = sum / square + gamma_coefficients[5 - i];
    exponent_error += sum / adjusted;
    result += result * cevalm::expm1(exponent_error);
    return result;
}

constexpr double tgamma_r(double x, int* sign_gamma) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        *sign_gamma = std::signbit(x) ? -1 : 1;
        return std::tgamma(x);
    }
#endif

    const uint64 magnitude = binary64_magnitude_bits(x);
    *sign_gamma = 0;
    if (magnitude == 0)
        return binary64_infinity(signbit(x));
    if (signbit(x) && isfinite(x) && cevalm::rint(x) == x)
        return binary64_from_bits(0xfff8000000000000ULL);
    if (isinf(x)) {
        if (signbit(x))
            return binary64_from_bits(0xfff8000000000000ULL);
        return x;
    }
    if (isnan(x))
        return binary64_quiet_nan(x);
    if (x >= 172.0)
        return binary64_infinity(false);
    if (x > 0.0) {
        int adjustment;
        const double result = gamma_positive(x, &adjustment);
        return cevalm::scalbn(result, adjustment);
    }
    if (x >= -0x1p-54)
        return 1.0 / x;
    const double integral = cevalm::trunc(x);
    *sign_gamma = integral == 2.0 * cevalm::trunc(integral / 2.0) ? -1 : 1;
    if (x <= -184.0)
        return 0.0;
    double fraction = integral - x;
    if (fraction > 0.5)
        fraction = 1.0 - fraction;
    const double sine = fraction <= 0.25 ? cevalm::sin(3.14159265358979323846 * fraction)
                                         : cevalm::cos(3.14159265358979323846 * (0.5 - fraction));
    int adjustment;
    const double positive = gamma_positive(-x, &adjustment);
    const double result = 3.14159265358979323846 / (-x * sine * positive);
    return cevalm::scalbn(result, -adjustment);
}
constexpr double tgamma(double x) {
#if CEVALM_HAS_STD_RUNTIME
    if !consteval {
        return std::tgamma(x);
    }
#endif

    int sign_gamma = 0;
    const double result = tgamma_r(x, &sign_gamma);
    return sign_gamma < 0 ? -result : result;
}

} // namespace cevalm

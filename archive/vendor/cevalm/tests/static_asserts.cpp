// SPDX-License-Identifier: LGPL-2.1-or-later

#include <cevalm.hpp>

using cevalm::binary64_bits;
using cevalm::binary64_from_bits;

static_assert(binary64_bits(1.0) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(-0.0) == 0x8000000000000000ULL);
static_assert(binary64_bits(binary64_from_bits(1)) == 1);

static_assert(cevalm::fpclassify(0.0) == cevalm::fp_zero);
static_assert(cevalm::fpclassify(binary64_from_bits(1)) == cevalm::fp_subnormal);
static_assert(cevalm::fpclassify(1.0) == cevalm::fp_normal);
static_assert(cevalm::fpclassify(binary64_from_bits(0x7ff0000000000000ULL)) == cevalm::fp_infinite);
static_assert(cevalm::fpclassify(binary64_from_bits(0x7ff8000000000001ULL)) == cevalm::fp_nan);
static_assert(cevalm::isfinite(1.0));
static_assert(cevalm::isinf(binary64_from_bits(0xfff0000000000000ULL)));
static_assert(cevalm::isnan(binary64_from_bits(0x7ff8000000001234ULL)));
static_assert(cevalm::issignaling(binary64_from_bits(0x7ff0000000001234ULL)));
static_assert(cevalm::signbit(-0.0));

static_assert(binary64_bits(cevalm::fabs(binary64_from_bits(0xfff8000000001234ULL))) ==
              0x7ff8000000001234ULL);
static_assert(binary64_bits(cevalm::copysign(1.0, -0.0)) == 0xbff0000000000000ULL);
static_assert(binary64_bits(cevalm::fmin(-0.0, 0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::fmax(0.0, -0.0)) == 0x0000000000000000ULL);
static_assert(cevalm::fdim(5.0, 2.0) == 3.0);
static_assert(cevalm::fdim(2.0, 5.0) == 0.0);

static_assert(cevalm::trunc(1.75) == 1.0);
static_assert(cevalm::trunc(-1.75) == -1.0);
static_assert(binary64_bits(cevalm::trunc(-0.25)) == 0x8000000000000000ULL);
static_assert(cevalm::floor(1.75) == 1.0);
static_assert(cevalm::floor(-1.25) == -2.0);
static_assert(cevalm::ceil(1.25) == 2.0);
static_assert(cevalm::ceil(-1.75) == -1.0);
static_assert(binary64_bits(cevalm::ceil(-0.25)) == 0x8000000000000000ULL);
static_assert(cevalm::round(0.5) == 1.0);
static_assert(cevalm::round(-0.5) == -1.0);
static_assert(cevalm::round(2.5) == 3.0);
static_assert(cevalm::round(-2.5) == -3.0);

static_assert(binary64_bits(cevalm::scalbn(1.0, -1074)) == 1);
static_assert(binary64_bits(cevalm::scalbn(binary64_from_bits(1), -1)) == 0);
static_assert(binary64_bits(cevalm::scalbn(binary64_from_bits(3), -1)) == 2);
static_assert(binary64_bits(cevalm::scalbn(binary64_from_bits(0x0010000000000000ULL), -1)) ==
              0x0008000000000000ULL);
static_assert(binary64_bits(cevalm::scalbn(binary64_from_bits(0x7fefffffffffffffULL), 1)) ==
              0x7ff0000000000000ULL);
static_assert(cevalm::ldexp(0.75, 2) == 3.0);

consteval bool frexp_cases() {
    int exponent = 99;
    double fraction = cevalm::frexp(12.0, &exponent);
    if (fraction != 0.75 || exponent != 4)
        return false;
    fraction = cevalm::frexp(binary64_from_bits(1), &exponent);
    return fraction == 0.5 && exponent == -1073;
}
static_assert(frexp_cases());

consteval bool modf_cases() {
    double integral = 0.0;
    const double fraction = cevalm::modf(-3.25, &integral);
    if (integral != -3.0 || fraction != -0.25)
        return false;
    const double zero = cevalm::modf(-3.0, &integral);
    return integral == -3.0 && binary64_bits(zero) == 0x8000000000000000ULL;
}
static_assert(modf_cases());

static_assert(binary64_bits(cevalm::nextafter(0.0, 1.0)) == 1);
static_assert(binary64_bits(cevalm::nextafter(0.0, -1.0)) == 0x8000000000000001ULL);
static_assert(binary64_bits(cevalm::nextafter(1.0, 2.0)) == 0x3ff0000000000001ULL);
static_assert(binary64_bits(cevalm::nextafter(1.0, 0.0)) == 0x3fefffffffffffffULL);
static_assert(binary64_bits(cevalm::nextafter(-1.0, 0.0)) == 0xbfefffffffffffffULL);

static_assert(cevalm::sqrt(0.0) == 0.0);
static_assert(binary64_bits(cevalm::sqrt(-0.0)) == 0x8000000000000000ULL);
static_assert(cevalm::sqrt(4.0) == 2.0);
static_assert(binary64_bits(cevalm::sqrt(2.0)) == 0x3ff6a09e667f3bcdULL);
static_assert(binary64_bits(cevalm::sqrt(-1.0)) == 0xfff8000000000000ULL);

static_assert(binary64_bits(cevalm::exp(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp(-0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp(1.0)) == 0x4005bf0a8b14576aULL);
static_assert(binary64_bits(cevalm::exp(-1.0)) == 0x3fd78b56362cef38ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x7ff0000000000000ULL))) ==
              0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0xfff0000000000000ULL))) == 0);
static_assert(binary64_bits(cevalm::expm1(0.0)) == 0);
static_assert(binary64_bits(cevalm::expm1(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::expm1(1.0)) == 0x3ffb7e151628aed2ULL);
static_assert(binary64_bits(cevalm::expm1(-1.0)) == 0xbfe43a54e4e98864ULL);

static_assert(binary64_bits(cevalm::log(1.0)) == 0);
static_assert(binary64_bits(cevalm::log(2.0)) == 0x3fe62e42fefa39efULL);
static_assert(binary64_bits(cevalm::log(0.0)) == 0xfff0000000000000ULL);
static_assert(binary64_bits(cevalm::log(-1.0)) == 0xfff8000000000000ULL);
static_assert(binary64_bits(cevalm::log1p(0.0)) == 0);
static_assert(binary64_bits(cevalm::log1p(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::log1p(1.0)) == 0x3fe62e42fefa39efULL);
static_assert(binary64_bits(cevalm::log1p(-1.0)) == 0xfff0000000000000ULL);
static_assert(cevalm::log2(1.0) == 0.0);
static_assert(cevalm::log2(2.0) == 1.0);
static_assert(cevalm::log2(0.5) == -1.0);
static_assert(cevalm::log10(1.0) == 0.0);
static_assert(cevalm::log10(10.0) == 1.0);

// fdlibm argument-reduction and range boundaries.
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x3fd62e4200000000ULL))) ==
              0x3ff6a09e0c594ff3ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x3fd62e42ffffffffULL))) ==
              0x3ff6a09e66dbc8d9ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x3ff0a2b1ffffffffULL))) ==
              0x4006a09e0d126a08ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x3ff0a2b200000000ULL))) ==
              0x4006a09e0d126a0aULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x40862e42fefa39efULL))) ==
              0x7fefffffffffff2aULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0x40862e42fefa39f0ULL))) ==
              0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0xc0874910d52d3051ULL))) ==
              0x0000000000000001ULL);
static_assert(binary64_bits(cevalm::exp(binary64_from_bits(0xc0874910d52d3052ULL))) ==
              0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::expm1(binary64_from_bits(0x3c8fffffffffffffULL))) ==
              0x3c8fffffffffffffULL);
static_assert(binary64_bits(cevalm::expm1(binary64_from_bits(0x3c90000000000000ULL))) ==
              0x3c90000000000000ULL);
static_assert(binary64_bits(cevalm::expm1(binary64_from_bits(0x40436879ffffffffULL))) ==
              0x436fffd8395c0c01ULL);
static_assert(binary64_bits(cevalm::expm1(binary64_from_bits(0x4043687a00000000ULL))) ==
              0x436fffd8395c0c41ULL);

// Subnormal, normalization, and domain boundaries for logarithms.
static_assert(binary64_bits(cevalm::log(binary64_from_bits(0x0000000000000001ULL))) ==
              0xc0874385446d71c3ULL);
static_assert(binary64_bits(cevalm::log(binary64_from_bits(0x000fffffffffffffULL))) ==
              0xc086232bdd7abcd2ULL);
static_assert(binary64_bits(cevalm::log(binary64_from_bits(0x0010000000000000ULL))) ==
              0xc086232bdd7abcd2ULL);
static_assert(binary64_bits(cevalm::log(binary64_from_bits(0x3ff0000000000001ULL))) ==
              0x3cafffffffffffffULL);
static_assert(binary64_bits(cevalm::log(binary64_from_bits(0x3fefffffffffffffULL))) ==
              0xbca0000000000000ULL);
static_assert(binary64_bits(cevalm::log1p(binary64_from_bits(0xbff0000000000001ULL))) ==
              0xfff8000000000000ULL);
static_assert(binary64_bits(cevalm::log1p(binary64_from_bits(0xbfefffffffffffffULL))) ==
              0xc0425e4f7b2737faULL);
static_assert(binary64_bits(cevalm::log1p(binary64_from_bits(0x3fda8279ffffffffULL))) ==
              0x3fd62e43471c5d9dULL);
static_assert(binary64_bits(cevalm::log1p(binary64_from_bits(0x3fda827a00000000ULL))) ==
              0x3fd62e43471c5d9eULL);
static_assert(binary64_bits(cevalm::log1p(binary64_from_bits(0xbfd2bec300000000ULL))) ==
              0xbfd62e42b6d81640ULL);
static_assert(binary64_bits(cevalm::log2(binary64_from_bits(0x0000000000000001ULL))) ==
              0xc090c80000000000ULL);
static_assert(binary64_bits(cevalm::log2(binary64_from_bits(0x0010000000000000ULL))) ==
              0xc08ff00000000000ULL);
static_assert(binary64_bits(cevalm::log10(binary64_from_bits(0x0000000000000001ULL))) ==
              0xc07434e6420f4374ULL);
static_assert(binary64_bits(cevalm::log10(binary64_from_bits(0x0010000000000000ULL))) ==
              0xc0733a7146f72a42ULL);

// Trigonometric kernel and argument-reduction boundaries.
static_assert(binary64_bits(cevalm::sin(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::sin(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::sin(1.0)) == 0x3feaed548f090ceeULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x3fe921fb54442d18ULL))) ==
              0x3fe6a09e667f3bccULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x3fe921fb54442d19ULL))) ==
              0x3fe6a09e667f3bcdULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x3ff921fb54442d18ULL))) ==
              0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x413921fb00000000ULL))) ==
              0xbfd4b02e5be91e9eULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x413921fc00000000ULL))) ==
              0x3fe3e479a61836ebULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x7e37e43c8800759cULL))) ==
              0xbfea2c16b010e385ULL);
static_assert(binary64_bits(cevalm::sin(binary64_from_bits(0x7fefffffffffffffULL))) ==
              0x3f7452fc98b34e97ULL);

static_assert(binary64_bits(cevalm::cos(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::cos(-1.0)) == 0x3fe14a280fb5068cULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x3fe921fb54442d18ULL))) ==
              0x3fe6a09e667f3bcdULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x3ff921fb54442d18ULL))) ==
              0x3c91a62633145c07ULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x400921fb54442d18ULL))) ==
              0xbff0000000000000ULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x413921fc00000000ULL))) ==
              0x3fe910d18d26c0d4ULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x7e37e43c8800759cULL))) ==
              0xbfe2699022adc4c1ULL);
static_assert(binary64_bits(cevalm::cos(binary64_from_bits(0x7fefffffffffffffULL))) ==
              0xbfefffe62ecfab75ULL);

static_assert(binary64_bits(cevalm::tan(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::tan(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::tan(1.0)) == 0x3ff8eb245cbee3a6ULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x3fe921fb54442d18ULL))) ==
              0x3fefffffffffffffULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x3fe921fb54442d19ULL))) ==
              0x3ff0000000000001ULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x3ff921fb54442d18ULL))) ==
              0x434d02967c31cdb5ULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x413921fc00000000ULL))) ==
              0x3fe9654894d5652aULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x7e37e43c8800759cULL))) ==
              0x3ff6be411f37ac77ULL);
static_assert(binary64_bits(cevalm::tan(binary64_from_bits(0x7fefffffffffffffULL))) ==
              0xbf74530cfe729484ULL);

consteval bool sincos_case() {
    double sine = 0.0;
    double cosine = 0.0;
    cevalm::sincos(1.0, &sine, &cosine);
    return binary64_bits(sine) == 0x3feaed548f090ceeULL &&
           binary64_bits(cosine) == 0x3fe14a280fb5068cULL;
}
static_assert(sincos_case());

// Inverse-trigonometric reduction, approximation, and special-case boundaries.
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3e1fffffffffffffULL))) ==
              0x3e1fffffffffffffULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3e20000000000000ULL))) ==
              0x3e20000000000000ULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3fdbffffffffffffULL))) ==
              0x3fda64eec3cc23fcULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3fdc000000000000ULL))) ==
              0x3fda64eec3cc23fdULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3fe5ffffffffffffULL))) ==
              0x3fe345f01cce37baULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3fe6000000000000ULL))) ==
              0x3fe345f01cce37bbULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3ff2ffffffffffffULL))) ==
              0x3febde70ed439fe6ULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x3ff3000000000000ULL))) ==
              0x3febde70ed439fe7ULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x40037fffffffffffULL))) ==
              0x3ff2e75728833a54ULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x4003800000000000ULL))) ==
              0x3ff2e75728833a54ULL);
static_assert(binary64_bits(cevalm::atan(binary64_from_bits(0x4410000000000000ULL))) ==
              0x3ff921fb54442d18ULL);

static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3e3fffffffffffffULL))) ==
              0x3e3fffffffffffffULL);
static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3e40000000000000ULL))) ==
              0x3e40000000000000ULL);
static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3fdfffffffffffffULL))) ==
              0x3fe0c152382d7365ULL);
static_assert(binary64_bits(cevalm::asin(0.5)) == 0x3fe0c152382d7366ULL);
static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3fef3332ffffffffULL))) ==
              0x3ff58c2ae9ab49e8ULL);
static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3fef333300000000ULL))) ==
              0x3ff58c2ae9ab49eaULL);
static_assert(binary64_bits(cevalm::asin(binary64_from_bits(0x3fefffffffffffffULL))) ==
              0x3ff921fb50442d18ULL);
static_assert(binary64_bits(cevalm::asin(1.0)) == 0x3ff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::asin(-1.0)) == 0xbff921fb54442d18ULL);

static_assert(binary64_bits(cevalm::acos(binary64_from_bits(0x3c600000ffffffffULL))) ==
              0x3ff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::acos(binary64_from_bits(0x3c60000100000000ULL))) ==
              0x3ff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::acos(binary64_from_bits(0x3fdfffffffffffffULL))) ==
              0x3ff0c152382d7366ULL);
static_assert(binary64_bits(cevalm::acos(0.5)) == 0x3ff0c152382d7366ULL);
static_assert(binary64_bits(cevalm::acos(binary64_from_bits(0x3fefffffffffffffULL))) ==
              0x3e50000000000000ULL);
static_assert(binary64_bits(cevalm::acos(1.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::acos(-1.0)) == 0x400921fb54442d18ULL);

static_assert(binary64_bits(cevalm::atan2(0.0, 1.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::atan2(-0.0, 1.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::atan2(0.0, -1.0)) == 0x400921fb54442d18ULL);
static_assert(binary64_bits(cevalm::atan2(-0.0, -1.0)) == 0xc00921fb54442d18ULL);
static_assert(binary64_bits(cevalm::atan2(1.0, 0.0)) == 0x3ff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::atan2(-1.0, 0.0)) == 0xbff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::atan2(binary64_from_bits(0x7ff0000000000000ULL),
                                          binary64_from_bits(0xfff0000000000000ULL))) ==
              0x4002d97c7f3321d2ULL);
static_assert(binary64_bits(cevalm::atan2(binary64_from_bits(0xfff0000000000000ULL),
                                          binary64_from_bits(0xfff0000000000000ULL))) ==
              0xc002d97c7f3321d2ULL);
static_assert(binary64_bits(cevalm::atan2(binary64_from_bits(0x43c0000000000000ULL),
                                          binary64_from_bits(0x0000000000000001ULL))) ==
              0x3ff921fb54442d18ULL);
static_assert(binary64_bits(cevalm::atan2(binary64_from_bits(0x0000000000000001ULL),
                                          binary64_from_bits(0xc3c0000000000000ULL))) ==
              0x400921fb54442d18ULL);

// Hyperbolic range, approximation, domain, and overflow boundaries.
static_assert(binary64_bits(cevalm::sinh(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::sinh(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::sinh(1.0)) == 0x3ff2cd9fc44eb982ULL);
static_assert(binary64_bits(cevalm::sinh(binary64_from_bits(0x3e2fffffffffffffULL))) ==
              0x3e2fffffffffffffULL);
static_assert(binary64_bits(cevalm::sinh(binary64_from_bits(0x3e30000000000000ULL))) ==
              0x3e30000000000000ULL);
static_assert(binary64_bits(cevalm::sinh(binary64_from_bits(0x408633ce8fb9f87dULL))) ==
              0x7feffffffffffd3bULL);
static_assert(binary64_bits(cevalm::sinh(binary64_from_bits(0x408633ce8fb9f87eULL))) ==
              0x7ff0000000000000ULL);

static_assert(binary64_bits(cevalm::cosh(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::cosh(-1.0)) == 0x3ff8b07551d9f551ULL);
static_assert(binary64_bits(cevalm::cosh(binary64_from_bits(0x3c7fffffffffffffULL))) ==
              0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::cosh(binary64_from_bits(0x408633ce8fb9f87dULL))) ==
              0x7feffffffffffd3bULL);
static_assert(binary64_bits(cevalm::cosh(binary64_from_bits(0x408633ce8fb9f87eULL))) ==
              0x7ff0000000000000ULL);

static_assert(binary64_bits(cevalm::tanh(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::tanh(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::tanh(1.0)) == 0x3fe85efab514f394ULL);
static_assert(binary64_bits(cevalm::tanh(22.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::tanh(-22.0)) == 0xbff0000000000000ULL);

static_assert(binary64_bits(cevalm::asinh(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::asinh(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::asinh(1.0)) == 0x3fec34366179d427ULL);
static_assert(binary64_bits(cevalm::asinh(binary64_from_bits(0x4000000000000000ULL))) ==
              0x3ff719218313d087ULL);
static_assert(binary64_bits(cevalm::asinh(binary64_from_bits(0x4000000100000000ULL))) ==
              0x3ff71922680cf8fcULL);
static_assert(binary64_bits(cevalm::asinh(binary64_from_bits(0x41b0000100000000ULL))) ==
              0x403419ecc712c400ULL);

static_assert(binary64_bits(cevalm::acosh(1.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::acosh(binary64_from_bits(0x3ff0000000000001ULL))) ==
              0x3e56a09e667f3bcdULL);
static_assert(binary64_bits(cevalm::acosh(2.0)) == 0x3ff5124271980434ULL);
static_assert(binary64_bits(cevalm::acosh(binary64_from_bits(0x4000000100000000ULL))) ==
              0x3ff5124399326c3dULL);

static_assert(binary64_bits(cevalm::atanh(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::atanh(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::atanh(0.5)) == 0x3fe193ea7aad030aULL);
static_assert(binary64_bits(cevalm::atanh(-0.5)) == 0xbfe193ea7aad030aULL);
static_assert(binary64_bits(cevalm::atanh(binary64_from_bits(0x3fefffffffffffffULL))) ==
              0x4032b708872320e2ULL);
static_assert(binary64_bits(cevalm::atanh(1.0)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::atanh(-1.0)) == 0xfff0000000000000ULL);

// Power, cube-root, and norm boundaries.
static_assert(binary64_bits(cevalm::exp2(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp2(-0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp2(1.0)) == 0x4000000000000000ULL);
static_assert(binary64_bits(cevalm::exp2(-1075.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::exp2(-1074.0)) == 0x0000000000000001ULL);
static_assert(binary64_bits(cevalm::exp2(binary64_from_bits(0x408fffffffffffffULL))) ==
              0x7feffffffffffd3aULL);
static_assert(binary64_bits(cevalm::exp2(1024.0)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::exp10(1.0)) == 0x4024000000000001ULL);
static_assert(binary64_bits(cevalm::exp10(binary64_from_bits(0xc075b00000000000ULL))) ==
              0x0000000000000000ULL);

static_assert(binary64_bits(cevalm::cbrt(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::cbrt(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::cbrt(27.0)) == 0x4008000000000001ULL);
static_assert(binary64_bits(cevalm::cbrt(-27.0)) == 0xc008000000000001ULL);
static_assert(binary64_bits(cevalm::cbrt(binary64_from_bits(0x0000000000000001ULL))) ==
              0x2990000000000000ULL);
static_assert(binary64_bits(cevalm::cbrt(binary64_from_bits(0x7fefffffffffffffULL))) ==
              0x554428a2f98d728bULL);

static_assert(binary64_bits(cevalm::hypot(3.0, 4.0)) == 0x4014000000000000ULL);
static_assert(binary64_bits(cevalm::hypot(binary64_from_bits(1), binary64_from_bits(1))) ==
              0x0000000000000001ULL);
static_assert(binary64_bits(cevalm::hypot(binary64_from_bits(0x7fefffffffffffffULL),
                                          binary64_from_bits(0x7fefffffffffffffULL))) ==
              0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::hypot(binary64_from_bits(0x7ff0000000000000ULL),
                                          binary64_from_bits(0x7ff8000000001234ULL))) ==
              0x7ff0000000000000ULL);

static_assert(binary64_bits(cevalm::pow(2.0, 3.0)) == 0x4020000000000000ULL);
static_assert(binary64_bits(cevalm::pow(-2.0, 3.0)) == 0xc020000000000000ULL);
static_assert(binary64_bits(cevalm::pow(-2.0, 4.0)) == 0x4030000000000000ULL);
static_assert(binary64_bits(cevalm::pow(-0.0, -3.0)) == 0xfff0000000000000ULL);
static_assert(binary64_bits(cevalm::pow(-0.0, 3.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::pow(-0.0, 4.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::pow(2.0, 1024.0)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::pow(2.0, -1075.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::pow(binary64_from_bits(0x3ff0000000000001ULL),
                                        binary64_from_bits(0x43f0000000000000ULL))) ==
              0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::pow(binary64_from_bits(0x3fefffffffffffffULL),
                                        binary64_from_bits(0x43f0000000000000ULL))) ==
              0x0000000000000000ULL);

static_assert(binary64_bits(cevalm::powi(2.0, 10)) == 0x4090000000000000ULL);
static_assert(binary64_bits(cevalm::powi(-2.0, 11)) == 0xc0a0000000000000ULL);
static_assert(binary64_bits(cevalm::powi(-2.0, -11)) == 0xbf40000000000000ULL);
static_assert(binary64_bits(cevalm::powi(2.0, 1024)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::powi(2.0, -1075)) == 0x0000000000000000ULL);

// Exact remainder reduction, halfway ties, and quotient-bit signs.
static_assert(binary64_bits(cevalm::fmod(binary64_from_bits(0x4015333333333333ULL), 2.0)) ==
              0x3ff4ccccccccccccULL);
static_assert(binary64_bits(cevalm::fmod(binary64_from_bits(0xc015333333333333ULL), 2.0)) ==
              0xbff4ccccccccccccULL);
static_assert(binary64_bits(cevalm::fmod(1.0, 2.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::fmod(2.0, 2.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::fmod(binary64_from_bits(0x8000000000000001ULL),
                                         binary64_from_bits(0x0000000000000001ULL))) ==
              0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::fmod(binary64_from_bits(0x000fffffffffffffULL),
                                         binary64_from_bits(0x0000000000000003ULL))) ==
              0x0000000000000000ULL);

static_assert(binary64_bits(cevalm::remainder(3.0, 2.0)) == 0xbff0000000000000ULL);
static_assert(binary64_bits(cevalm::remainder(5.0, 2.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::remainder(7.0, 2.0)) == 0xbff0000000000000ULL);
static_assert(binary64_bits(cevalm::remainder(-3.0, 2.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::remainder(1.0, binary64_from_bits(0x7ff0000000000000ULL))) ==
              0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::remainder(binary64_from_bits(0x8000000000000001ULL),
                                              binary64_from_bits(0x0000000000000003ULL))) ==
              0x8000000000000001ULL);

consteval bool remquo_cases() {
    int quotient = 0;
    double result = cevalm::remquo(3.0, 2.0, &quotient);
    if (binary64_bits(result) != 0xbff0000000000000ULL || quotient != 2)
        return false;
    result = cevalm::remquo(5.0, 2.0, &quotient);
    if (binary64_bits(result) != 0x3ff0000000000000ULL || quotient != 2)
        return false;
    result = cevalm::remquo(7.0, 2.0, &quotient);
    if (binary64_bits(result) != 0xbff0000000000000ULL || quotient != 4)
        return false;
    result = cevalm::remquo(-5.0, 2.0, &quotient);
    if (binary64_bits(result) != 0xbff0000000000000ULL || quotient != -2)
        return false;
    result = cevalm::remquo(29.0, 3.0, &quotient);
    if (binary64_bits(result) != 0xbff0000000000000ULL || quotient != 2)
        return false;
    result = cevalm::remquo(binary64_from_bits(0x8000000000000001ULL),
                            binary64_from_bits(0x0000000000000003ULL), &quotient);
    return binary64_bits(result) == 0x8000000000000001ULL && quotient == 0;
}
static_assert(remquo_cases());

// Nearest-even floating and integer rounding under cevalm's fixed mode.
static_assert(binary64_bits(cevalm::rint(0.5)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::rint(-0.5)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::rint(1.5)) == 0x4000000000000000ULL);
static_assert(binary64_bits(cevalm::rint(2.5)) == 0x4000000000000000ULL);
static_assert(binary64_bits(cevalm::rint(3.5)) == 0x4010000000000000ULL);
static_assert(binary64_bits(cevalm::rint(-2.5)) == 0xc000000000000000ULL);
static_assert(binary64_bits(cevalm::nearbyint(2.5)) == 0x4000000000000000ULL);

static_assert(cevalm::lrint(1.5) == 2L);
static_assert(cevalm::lrint(2.5) == 2L);
static_assert(cevalm::lrint(-2.5) == -2L);
static_assert(cevalm::lrint(2147483647.0) == 2147483647L);
static_assert(cevalm::lrint(2147483648.0) == (-2147483647L - 1L));
static_assert(cevalm::lround(0.5) == 1L);
static_assert(cevalm::lround(-0.5) == -1L);
static_assert(cevalm::lround(2.5) == 3L);
static_assert(cevalm::lround(-2.5) == -3L);

static_assert(cevalm::llrint(2.5) == 2LL);
static_assert(cevalm::llrint(-2.5) == -2LL);
static_assert(cevalm::llrint(binary64_from_bits(0x43dfffffffffffffULL)) == 0x7ffffffffffffc00LL);
static_assert(cevalm::llrint(binary64_from_bits(0xc3e0000000000000ULL)) ==
              (-9223372036854775807LL - 1LL));
static_assert(cevalm::llround(2.5) == 3LL);
static_assert(cevalm::llround(-2.5) == -3LL);

// Binary exponent extraction, normalization, and legacy scaling.
static_assert(cevalm::ilogb(0.0) == cevalm::fp_ilogb_zero);
static_assert(cevalm::ilogb(binary64_from_bits(1)) == -1074);
static_assert(cevalm::ilogb(binary64_from_bits(0x0010000000000000ULL)) == -1022);
static_assert(cevalm::ilogb(binary64_from_bits(0x7fefffffffffffffULL)) == 1023);
static_assert(cevalm::ilogb(binary64_from_bits(0x7ff0000000000000ULL)) == cevalm::fp_ilogb_nan);
static_assert(binary64_bits(cevalm::logb(0.0)) == 0xfff0000000000000ULL);
static_assert(binary64_bits(cevalm::logb(binary64_from_bits(1))) == 0xc090c80000000000ULL);
static_assert(binary64_bits(cevalm::logb(binary64_from_bits(0x0010000000000000ULL))) ==
              0xc08ff00000000000ULL);
static_assert(binary64_bits(cevalm::logb(binary64_from_bits(0x7fefffffffffffffULL))) ==
              0x408ff80000000000ULL);
static_assert(binary64_bits(cevalm::significand(binary64_from_bits(1))) == 1);
static_assert(binary64_bits(cevalm::significand(3.0)) == 0x3ff8000000000000ULL);
static_assert(binary64_bits(cevalm::significand(-3.0)) == 0xbff8000000000000ULL);

static_assert(binary64_bits(cevalm::scalb(1.0, 2.0)) == 0x4010000000000000ULL);
static_assert(binary64_bits(cevalm::scalb(1.0, 0.5)) == 0x7ff8000000000000ULL);
static_assert(binary64_bits(cevalm::scalb(0.0, binary64_from_bits(0x7ff0000000000000ULL))) ==
              0xfff8000000000000ULL);
static_assert(binary64_bits(cevalm::scalb(binary64_from_bits(0x7ff0000000000000ULL),
                                          binary64_from_bits(0xfff0000000000000ULL))) ==
              0xfff8000000000000ULL);

// Error-function approximation intervals and asymptotic tails.
static_assert(binary64_bits(cevalm::erf(0.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::erf(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::erf(1.0)) == 0x3feaf767a741088bULL);
static_assert(binary64_bits(cevalm::erf(binary64_from_bits(0x3e2fffffffffffffULL))) ==
              0x3e320dd750429b6cULL);
static_assert(binary64_bits(cevalm::erf(binary64_from_bits(0x3e30000000000000ULL))) ==
              0x3e320dd750429b6dULL);
static_assert(binary64_bits(cevalm::erf(binary64_from_bits(0x3feaffffffffffffULL))) ==
              0x3fe88d1cd474a2e0ULL);
static_assert(binary64_bits(cevalm::erf(binary64_from_bits(0x3feb000000000000ULL))) ==
              0x3fe88d1cd474a2e0ULL);
static_assert(binary64_bits(cevalm::erf(6.0)) == 0x3ff0000000000000ULL);

static_assert(binary64_bits(cevalm::erfc(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::erfc(-0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::erfc(1.0)) == 0x3fc4226162fbddd5ULL);
static_assert(binary64_bits(cevalm::erfc(binary64_from_bits(0x3feaffffffffffffULL))) ==
              0x3fcdcb8cae2d7480ULL);
static_assert(binary64_bits(cevalm::erfc(binary64_from_bits(0x3feb000000000000ULL))) ==
              0x3fcdcb8cae2d747eULL);
static_assert(binary64_bits(cevalm::erfc(28.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::erfc(-6.0)) == 0x4000000000000000ULL);

consteval bool gamma_sign_cases() {
    int sign = 0;
    double result = cevalm::lgamma_r(-0.0, &sign);
    if (binary64_bits(result) != 0x7ff0000000000000ULL || sign != -1)
        return false;
    result = cevalm::lgamma_r(-0.5, &sign);
    if (binary64_bits(result) != 0x3ff43f89a3f0edd6ULL || sign != -1)
        return false;
    result = cevalm::lgamma_r(-1.5, &sign);
    if (binary64_bits(result) != 0x3feb858151820f85ULL || sign != 1)
        return false;
    result = cevalm::tgamma_r(-0.5, &sign);
    return binary64_bits(result) == 0x400c5bf891b4ef6aULL && sign == -1;
}
static_assert(gamma_sign_cases());

static_assert(binary64_bits(cevalm::lgamma(0.5)) == 0x3fe250d048e7a1bdULL);
static_assert(binary64_bits(cevalm::lgamma(1.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::lgamma(2.0)) == 0x0000000000000000ULL);
static_assert(binary64_bits(cevalm::lgamma(3.0)) == 0x3fe62e42fefa39efULL);
static_assert(binary64_bits(cevalm::lgamma(8.0)) == 0x40210ce1f32dcc2fULL);

static_assert(binary64_bits(cevalm::tgamma(0.0)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::tgamma(-0.0)) == 0xfff0000000000000ULL);

// fdlibm Bessel rational, asymptotic, recurrence, and domain cases.
static_assert(binary64_bits(cevalm::j0(0.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::j1(-0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::y0(0.0)) == 0xfff0000000000000ULL);
static_assert(binary64_bits(cevalm::y1(-0.0)) == 0xfff0000000000000ULL);
static_assert(binary64_bits(cevalm::j0(1.0)) == 0x3fe87c7fdbd7b8f0ULL);
static_assert(binary64_bits(cevalm::j1(1.0)) == 0x3fdc29c9ee970c6cULL);
static_assert(binary64_bits(cevalm::y0(1.0)) == 0x3fb6980226f358e1ULL);
static_assert(binary64_bits(cevalm::y1(1.0)) == 0xbfe8ffb207d66b94ULL);
static_assert(binary64_bits(cevalm::j0(binary64_from_bits(0x4006db6cffffffffULL))) ==
              0xbfca9c7088e444b7ULL);
static_assert(binary64_bits(cevalm::j0(binary64_from_bits(0x4006db6d00000000ULL))) ==
              0xbfca9c7088e444bdULL);
static_assert(binary64_bits(cevalm::y1(binary64_from_bits(0x40122e8affffffffULL))) ==
              0x3fd27cf2cb44266bULL);
static_assert(binary64_bits(cevalm::y1(binary64_from_bits(0x40122e8b00000000ULL))) ==
              0x3fd27cf2cb442665ULL);
static_assert(binary64_bits(cevalm::j1(binary64_from_bits(0x401fffffffffffffULL))) ==
              0x3fce08905618617dULL);
static_assert(binary64_bits(cevalm::j1(8.0)) == 0x3fce089056186182ULL);
static_assert(binary64_bits(cevalm::jn(-7, 0.125)) == 0xbd69fe6014389cb3ULL);
static_assert(binary64_bits(cevalm::yn(-7, 0.125)) == 0x422caaa2e402d046ULL);
static_assert(binary64_bits(cevalm::jn(2, 3.0)) == 0x3fdf1c1e84c59ec7ULL);
static_assert(binary64_bits(cevalm::yn(2, 3.0)) == 0xbfc488000624427cULL);
static_assert(binary64_bits(cevalm::jn(7, 20.0)) == 0xbfc7949117377868ULL);
static_assert(binary64_bits(cevalm::yn(7, 20.0)) == 0xbf722674753218b0ULL);

// Fused multiply-add rounds only once, including at subnormal boundaries.
static_assert(binary64_bits(cevalm::fma(0x1.0000000000001p0, 0x1.fffffffffffffp0, -2.0)) ==
              0x3caffffffffffffeULL);
static_assert(binary64_bits(cevalm::fma(binary64_from_bits(1), 0.5, 0.0)) == 0);
static_assert(binary64_bits(cevalm::fma(binary64_from_bits(3), 0.5, 0.0)) == 2);
static_assert(binary64_bits(cevalm::fma(0.0, -1.0, -0.0)) == 0x8000000000000000ULL);
static_assert(binary64_bits(cevalm::fma(0.0, 1.0, -0.0)) == 0);
static_assert(binary64_bits(cevalm::fma(binary64_from_bits(0x7ff0000000000000ULL), 0.0, 1.0)) ==
              0xfff8000000000000ULL);

static_assert(binary64_bits(cevalm::nan("")) == 0x7ff8000000000000ULL);
static_assert(binary64_bits(cevalm::nan("123")) == 0x7ff800000000007bULL);
static_assert(binary64_bits(cevalm::nan("0x123")) == 0x7ff8000000000123ULL);
static_assert(binary64_bits(cevalm::nan("xyz")) == 0x7ff8000000000000ULL);

static_assert(cevalm::isgreater(2.0, 1.0));
static_assert(cevalm::isgreaterequal(2.0, 2.0));
static_assert(cevalm::isless(1.0, 2.0));
static_assert(cevalm::islessequal(2.0, 2.0));
static_assert(cevalm::islessgreater(1.0, 2.0));
static_assert(!cevalm::islessgreater(-0.0, 0.0));
static_assert(cevalm::isunordered(binary64_from_bits(0x7ff8000000000001ULL), 1.0));
static_assert(!cevalm::isgreater(binary64_from_bits(0x7ff8000000000001ULL), 1.0));
static_assert(binary64_bits(cevalm::tgamma(0.5)) == 0x3ffc5bf891b4ef6bULL);
static_assert(binary64_bits(cevalm::tgamma(-0.5)) == 0xc00c5bf891b4ef6aULL);
static_assert(binary64_bits(cevalm::tgamma(1.0)) == 0x3ff0000000000000ULL);
static_assert(binary64_bits(cevalm::tgamma(4.0)) == 0x4018000000000000ULL);
static_assert(binary64_bits(cevalm::tgamma(10.0)) == 0x411625ffffffffffULL);
static_assert(binary64_bits(cevalm::tgamma(171.0)) == 0x7fa4ab7864418639ULL);
static_assert(binary64_bits(cevalm::tgamma(172.0)) == 0x7ff0000000000000ULL);
static_assert(binary64_bits(cevalm::tgamma(-2.0)) == 0xfff8000000000000ULL);

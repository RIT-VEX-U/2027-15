// SPDX-License-Identifier: SunPro

/*
 * fdlibm binary64 pi/2 argument reduction.
 * Copyright (C) 1993 by Sun Microsystems, Inc. All rights reserved.
 * Permission to use, copy, modify, and distribute this software is freely
 * granted, provided that this notice is preserved.
 */
#pragma once

#include "cevalm_config.hpp"

#include "cevalm_binary64.hpp"
#include "cevalm_classify.hpp"
#include "cevalm_exact.hpp"
#include "cevalm_round.hpp"
#include "cevalm_scale.hpp"

namespace cevalm {

constexpr int kernel_rem_pio2(double* x, double* y, int e0, int nx) {
    constexpr int32 two_over_pi[] = {
        0xa2f983, 0x6e4e44, 0x1529fc, 0x2757d1, 0xf534dd, 0xc0db62, 0x95993c, 0x439041, 0xfe5163,
        0xabdebb, 0xc561b7, 0x246e3a, 0x424dd2, 0xe00649, 0x2eea09, 0xd1921c, 0xfe1deb, 0x1cb129,
        0xa73ee8, 0x8235f5, 0x2ebb44, 0x84e99c, 0x7026b4, 0x5f7e41, 0x3991d6, 0x398353, 0x39f49c,
        0x845f8b, 0xbdf928, 0x3b1ff8, 0x97ffde, 0x05980f, 0xef2f11, 0x8b5a0a, 0x6d1f6d, 0x367ecf,
        0x27cb09, 0xb74f46, 0x3f669e, 0x5fea2d, 0x7527ba, 0xc7ebe5, 0xf17b3d, 0x0739f7, 0x8a5292,
        0xea6bfb, 0x5fb11f, 0x8d5d08, 0x560330, 0x46fc7b, 0x6babf0, 0xcfbc20, 0x9af436, 0x1da9e3,
        0x91615e, 0xe61b08, 0x659985, 0x5f14a0, 0x68408d, 0xffd880, 0x4d7327, 0x310606, 0x1556ca,
        0x73a8c9, 0x60e27b, 0xc08c6b};
    constexpr double pio2[] = {1.57079625129699707031e+00, 7.54978941586159635335e-08,
                               5.39030252995776476554e-15, 3.28200341580791294123e-22,
                               1.27065575308067607349e-29, 1.22933308981111328932e-36,
                               2.73370053816464559624e-44, 2.16741683877804819444e-51};
    constexpr double two24 = 1.67772160000000000000e+07;
    constexpr double two_minus24 = 5.96046447753906250000e-08;

    int iq[20]{};
    double f[20]{}, fq[20]{}, q[20]{};
    constexpr int jk = 4;
    constexpr int jp = 4;
    const int jx = nx - 1;
    int jv = (e0 - 3) / 24;
    if (jv < 0)
        jv = 0;
    int q0 = e0 - 24 * (jv + 1);
    int table_index = jv - jx;
    const int initial_terms = jx + jk;
    for (int i = 0; i <= initial_terms; ++i, ++table_index)
        f[i] = table_index < 0 ? 0.0 : static_cast<double>(two_over_pi[table_index]);
    for (int i = 0; i <= jk; ++i) {
        double sum = 0.0;
        for (int j = 0; j <= jx; ++j)
            sum += x[j] * f[jx + i - j];
        q[i] = sum;
    }

    int jz = jk;
    int n = 0;
    int half_indicator = 0;
    double z = 0.0;
    for (;;) {
        int output_index = 0;
        int q_index = jz;
        z = q[jz];
        for (; q_index > 0; ++output_index, --q_index) {
            const double carry_value = static_cast<double>(static_cast<int>(two_minus24 * z));
            iq[output_index] = static_cast<int>(z - two24 * carry_value);
            z = q[q_index - 1] + carry_value;
        }
        z = binary64_scale(z, q0);
        z -= 8.0 * cevalm::floor(z * 0.125);
        n = static_cast<int>(z);
        z -= static_cast<double>(n);
        half_indicator = 0;
        if (q0 > 0) {
            const int top = iq[jz - 1] >> (24 - q0);
            n += top;
            iq[jz - 1] -= top << (24 - q0);
            half_indicator = iq[jz - 1] >> (23 - q0);
        } else if (q0 == 0) {
            half_indicator = iq[jz - 1] >> 23;
        } else if (z >= 0.5) {
            half_indicator = 2;
        }
        if (half_indicator > 0) {
            ++n;
            int carry = 0;
            for (int i = 0; i < jz; ++i) {
                const int current = iq[i];
                if (carry == 0) {
                    if (current != 0) {
                        carry = 1;
                        iq[i] = 0x1000000 - current;
                    }
                } else
                    iq[i] = 0xffffff - current;
            }
            if (q0 == 1)
                iq[jz - 1] &= 0x7fffff;
            else if (q0 == 2)
                iq[jz - 1] &= 0x3fffff;
            if (half_indicator == 2) {
                z = 1.0 - z;
                if (carry != 0)
                    z -= binary64_scale(1.0, q0);
            }
        }
        bool recompute = false;
        if (z == 0.0) {
            int combined = 0;
            for (int i = jz - 1; i >= jk; --i)
                combined |= iq[i];
            if (combined == 0) {
                int more = 1;
                while (iq[jk - more] == 0)
                    ++more;
                for (int i = jz + 1; i <= jz + more; ++i) {
                    f[jx + i] = static_cast<double>(two_over_pi[jv + i]);
                    double sum = 0.0;
                    for (int j = 0; j <= jx; ++j)
                        sum += x[j] * f[jx + i - j];
                    q[i] = sum;
                }
                jz += more;
                recompute = true;
            }
        }
        if (!recompute)
            break;
    }

    if (z == 0.0) {
        --jz;
        q0 -= 24;
        while (iq[jz] == 0) {
            --jz;
            q0 -= 24;
        }
    } else {
        z = binary64_scale(z, -q0);
        if (z >= two24) {
            const double carry_value = static_cast<double>(static_cast<int>(two_minus24 * z));
            iq[jz] = static_cast<int>(z - two24 * carry_value);
            ++jz;
            q0 += 24;
            iq[jz] = static_cast<int>(carry_value);
        } else
            iq[jz] = static_cast<int>(z);
    }
    double weight = binary64_scale(1.0, q0);
    for (int i = jz; i >= 0; --i) {
        q[i] = weight * static_cast<double>(iq[i]);
        weight *= two_minus24;
    }
    for (int i = jz; i >= 0; --i) {
        double sum = 0.0;
        for (int k = 0; k <= jp && k <= jz - i; ++k)
            sum += pio2[k] * q[i + k];
        fq[jz - i] = sum;
    }
    double sum = 0.0;
    for (int i = jz; i >= 0; --i)
        sum += fq[i];
    y[0] = half_indicator == 0 ? sum : -sum;
    sum = fq[0] - sum;
    for (int i = 1; i <= jz; ++i)
        sum += fq[i];
    y[1] = half_indicator == 0 ? sum : -sum;
    return n & 7;
}

constexpr int rem_pio2(double x, double* y) {
    constexpr uint32 multiples[] = {
        0x3ff921fb, 0x400921fb, 0x4012d97c, 0x401921fb, 0x401f6a7a, 0x4022d97c, 0x4025fdbb,
        0x402921fb, 0x402c463a, 0x402f6a7a, 0x4031475c, 0x4032d97c, 0x40346b9c, 0x4035fdbb,
        0x40378fdb, 0x403921fb, 0x403ab41b, 0x403c463a, 0x403dd85a, 0x403f6a7a, 0x40407e4c,
        0x4041475c, 0x4042106c, 0x4042d97c, 0x4043a28c, 0x40446b9c, 0x404534ac, 0x4045fdbb,
        0x4046c6cb, 0x40478fdb, 0x404858eb, 0x404921fb};
    constexpr double invpio2 = 6.36619772367581382433e-01, p1 = 1.57079632673412561417,
                     p1t = 6.07710050650619224932e-11, p2 = 6.07710050630396597660e-11,
                     p2t = 2.02226624879595063154e-21, p3 = 2.02226624871116645580e-21,
                     p3t = 8.47842766036889956997e-32, two24 = 16777216.0;
    const uint64 bits = binary64_bits(x);
    const bool neg = signbit(x);
    const uint32 high = binary64_high_word(cevalm::fabs(x));
    if (high <= 0x3fe921fbU) {
        y[0] = x;
        y[1] = 0;
        return 0;
    }
    if (high < 0x4002d97cU) {
        double z = x + (neg ? p1 : -p1);
        if (high != 0x3ff921fbU) {
            y[0] = z + (neg ? p1t : -p1t);
            y[1] = (z - y[0]) + (neg ? p1t : -p1t);
        } else {
            z += neg ? p2 : -p2;
            y[0] = z + (neg ? p2t : -p2t);
            y[1] = (z - y[0]) + (neg ? p2t : -p2t);
        }
        return neg ? -1 : 1;
    }
    if (high <= 0x413921fbU) {
        const double magnitude = cevalm::fabs(x);
        const int n = static_cast<int>(magnitude * invpio2 + 0.5);
        const double fn = static_cast<double>(n);
        double r = magnitude - fn * p1;
        double w = fn * p1t;
        if (!(n < 32 && high != multiples[n - 1])) {
            const int exponent = static_cast<int>(high >> 20);
            y[0] = r - w;
            int cancellation =
                exponent -
                static_cast<int>((binary64_high_word(cevalm::fabs(y[0])) >> 20) & 0x7ffU);
            if (cancellation > 16) {
                double t = r;
                w = fn * p2;
                r = t - w;
                w = fn * p2t - ((t - r) - w);
                y[0] = r - w;
                cancellation =
                    exponent -
                    static_cast<int>((binary64_high_word(cevalm::fabs(y[0])) >> 20) & 0x7ffU);
                if (cancellation > 49) {
                    t = r;
                    w = fn * p3;
                    r = t - w;
                    w = fn * p3t - ((t - r) - w);
                    y[0] = r - w;
                }
            }
        } else
            y[0] = r - w;
        y[1] = (r - y[0]) - w;
        if (neg) {
            y[0] = -y[0];
            y[1] = -y[1];
            return -n;
        }
        return n;
    }
    if (high >= 0x7ff00000U) {
        const double nan = binary64_is_nan_bits(bits) ? binary64_quiet_nan(x)
                                                      : binary64_from_bits(0xfff8000000000000ULL);
        y[0] = nan;
        y[1] = nan;
        return 0;
    }
    const int e0 = static_cast<int>(high >> 20) - (1023 + 23);
    double z = binary64_from_bits(
        (static_cast<uint64>(high - static_cast<uint32>(e0 * 0x00100000)) << 32) |
        binary64_low_word(x));
    double pieces[3];
    for (int i = 0; i < 2; ++i) {
        pieces[i] = static_cast<double>(static_cast<int>(z));
        z = (z - pieces[i]) * two24;
    }
    pieces[2] = z;
    int nx = 3;
    while (pieces[nx - 1] == 0.0)
        --nx;
    const int n = kernel_rem_pio2(pieces, y, e0, nx);
    if (neg) {
        y[0] = -y[0];
        y[1] = -y[1];
        return -n;
    }
    return n;
}

} // namespace cevalm

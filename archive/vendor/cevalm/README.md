# cevalm

`cevalm` is a binary64-only, constexpr mathematics
library for Clang 22+ and the Zynq-7000 Cortex-A9 (`armv7a-none-eabi`).
Its public functions use familiar `<cmath>` names under `namespace cevalm`:

```cpp
#include <cevalm.hpp>

constexpr double root_two = cevalm::sqrt(2.0);
```

At compile time, operations use the cevalm implementations. At runtime, the
public functions dispatch to the corresponding `<cmath>` operation.
Implementations derived from fdlibm preserve its binary64 arithmetic and
returned bit patterns for constant evaluation; runtime behavior follows the
standard library.

## Build the compile-time tests

```sh
cmake -S . -B build -G Ninja -DCMAKE_CXX_COMPILER=clang++
cmake --build build
```

The library headers do not require libc++, libstdc++, or a target runtime.

For development, the `dev` preset consistently creates `build/` and its
`compile_commands.json` for clangd:

```sh
cmake --preset dev
cmake --build --preset dev
```

The checked-in `.clangd` reads that compilation database. Source formatting is
configured by `.clang-format`; use `cmake --build build --target format` to
apply it or `cmake --build build --target format-check` to check it without
changing files.

## Implemented functions

The current binary64 surface includes classification and sign operations,
rounding, decomposition and scaling, adjacent-value operations, `sqrt`,
`exp`, `expm1`, `log`, `log1p`, `log2`, and `log10`.
The trigonometric stack provides `sin`, `cos`, `tan`, and `sincos` with full
fdlibm large-argument reduction. The inverse-trigonometric stack provides
`asin`, `acos`, `atan`, and `atan2`. The hyperbolic stack provides `sinh`,
`cosh`, `tanh`, `asinh`, `acosh`, and `atanh`. Powers and roots include
`exp2`, `exp10`, `pow`, `powi`, `sqrt`, `cbrt`, and `hypot`. Exact remainder
operations include `fmod`, `remainder`, and `remquo`.
The remaining `<cmath>` binary64 operations include fused `fma`, `nan`, and
the unordered-aware comparison predicates.

Nearest-even rounding is provided by `rint`, `nearbyint`, `lrint`, and
`llrint`; `lround` and `llround` use halfway-away-from-zero semantics.
Exponent operations include `ilogb`, `logb`, `significand`, and legacy
`scalb`, alongside the existing `frexp`, `ldexp`, `scalbn`, and `scalbln`.
Special functions include `erf`, `erfc`, `lgamma`, `lgamma_r`, `tgamma`, and
`tgamma_r`, including fdlibm reflection and sign reporting. The Bessel family
provides `j0`, `j1`, `jn`, `y0`, `y1`, and `yn`, with fdlibm's rational and
asymptotic approximations and integer-order recurrences.

## License

The combined cevalm distribution is licensed under LGPL-2.1-or-later. Portions
derived from Sun fdlibm retain the permissive SunPro license, and the
`fpclassify` implementation retains its Red Hat permission notice. See
`LICENSE`, `NOTICE`, and the texts under `LICENSES/` for the complete terms and
attribution.

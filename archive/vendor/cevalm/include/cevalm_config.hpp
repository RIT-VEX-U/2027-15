// SPDX-License-Identifier: LGPL-2.1-or-later

#pragma once

#if !defined(CEVALM_DISABLE_STD_RUNTIME) && __has_include(<cmath>)
#include <cmath>
#define CEVALM_HAS_STD_RUNTIME 1
#else
#define CEVALM_HAS_STD_RUNTIME 0
#endif

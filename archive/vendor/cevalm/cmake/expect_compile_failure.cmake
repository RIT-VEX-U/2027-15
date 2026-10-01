# SPDX-License-Identifier: LGPL-2.1-or-later

execute_process(
    COMMAND "${CEVALM_COMPILER}"
            --target=armv7a-none-eabi
            -mcpu=cortex-a9
            -std=gnu++23
            -ffreestanding
            -fno-fast-math
            -ffp-contract=off
            -nostdinc++
            -I"${CEVALM_INCLUDE}"
            -fsyntax-only
            "${CEVALM_SOURCE}"
    RESULT_VARIABLE compile_result
    OUTPUT_QUIET
    ERROR_QUIET
)
if(compile_result EQUAL 0)
    message(FATAL_ERROR "A runtime call to a cevalm consteval function compiled successfully")
endif()

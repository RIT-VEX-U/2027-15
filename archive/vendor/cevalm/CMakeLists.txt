# SPDX-License-Identifier: LGPL-2.1-or-later

cmake_minimum_required(VERSION 3.20)
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)
set(CMAKE_EXPORT_COMPILE_COMMANDS ON)
project(cevalm VERSION 0.1.0 LANGUAGES CXX)

if(NOT CMAKE_CXX_COMPILER_ID STREQUAL "Clang")
    message(FATAL_ERROR "cevalm requires Clang")
endif()
if(CMAKE_CXX_COMPILER_VERSION VERSION_LESS 22)
    message(FATAL_ERROR "cevalm requires Clang 22 or newer")
endif()

set(CEVALM_FORMAT_SOURCES
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_bessel.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_binary64.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_classify.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_config.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_exact.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_exp.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_exp2_table.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_exponent.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_fma.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_hyperbolic.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_inv_trig.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_log.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_power.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_rem_pio2.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_remainder.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_round.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_scale.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_special.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_sqrt.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/include/cevalm_trig.hpp
    ${CMAKE_CURRENT_SOURCE_DIR}/tests/runtime_call_must_fail.cpp
    ${CMAKE_CURRENT_SOURCE_DIR}/tests/static_asserts.cpp)

find_program(CEVALM_CLANG_FORMAT NAMES clang-format-22 clang-format)
if(CEVALM_CLANG_FORMAT)
    add_custom_target(format
        COMMAND ${CEVALM_CLANG_FORMAT} -i ${CEVALM_FORMAT_SOURCES}
        COMMENT "Formatting cevalm sources"
        VERBATIM)
    add_custom_target(format-check
        COMMAND ${CEVALM_CLANG_FORMAT} --dry-run --Werror ${CEVALM_FORMAT_SOURCES}
        COMMENT "Checking cevalm source formatting"
        VERBATIM)
endif()

add_library(cevalm INTERFACE)
target_include_directories(cevalm INTERFACE
    $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}/include>
    $<INSTALL_INTERFACE:include>)
target_compile_features(cevalm INTERFACE cxx_std_23)
target_compile_options(cevalm INTERFACE
    --target=armv7a-none-eabi
    -mcpu=cortex-a9
    -ffreestanding
    -fno-fast-math
    -ffp-contract=off)

option(CEVALM_BUILD_TESTS "Build cevalm compile-time tests" ON)
if(CEVALM_BUILD_TESTS)
    add_library(cevalm_static_asserts OBJECT tests/static_asserts.cpp)
    target_link_libraries(cevalm_static_asserts PRIVATE cevalm)
    target_compile_options(cevalm_static_asserts PRIVATE
        -Wall
        -Wextra
        -Wconversion
        -Wsign-conversion
        -Werror)

    add_library(cevalm_runtime_calls OBJECT tests/runtime_call_must_fail.cpp)
    target_link_libraries(cevalm_runtime_calls PRIVATE cevalm)
endif()

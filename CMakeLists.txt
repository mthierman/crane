cmake_minimum_required(VERSION 3.30)

project(Crane VERSION 0.0.0)

list(
    APPEND
    CMAKE_MODULE_PATH
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake"
    )

include(FetchContent)
include(mthierman.glow)
include(nlohmann.json)

add_executable(${PROJECT_NAME})

target_compile_options(
    ${PROJECT_NAME}
    PRIVATE $<$<CXX_COMPILER_ID:MSVC>:
            /MP
            /utf-8
            /bigobj
            /diagnostics:caret
            /Zc:__cplusplus
            >
            $<$<CXX_COMPILER_FRONTEND_VARIANT:MSVC>:
            /W4
            /WX
            >
            $<$<CXX_COMPILER_FRONTEND_VARIANT:GNU>:
            -Wall
            -Werror
            >
    PUBLIC $<$<CXX_COMPILER_ID:Clang>:
           -Wno-braced-scalar-init
           -Wno-nonportable-include-path
           >
    )

target_link_options(
    ${PROJECT_NAME}
    PRIVATE
    $<$<CXX_COMPILER_FRONTEND_VARIANT:MSVC>:
    /WX
    >
    $<$<CXX_COMPILER_FRONTEND_VARIANT:GNU>:
    -Wl,/WX
    >
    )

target_sources(${PROJECT_NAME} PRIVATE "src/main.cxx")

target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE glow::glow
            nlohmann::json
    )

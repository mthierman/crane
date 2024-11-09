cmake_minimum_required(VERSION 3.30)

project(
    Crane
    VERSION 0.0.0
)

list(
    APPEND
    CMAKE_MODULE_PATH
    "${CMAKE_CURRENT_SOURCE_DIR}/cmake"
)

add_executable(${PROJECT_NAME})

target_compile_features(
    ${PROJECT_NAME}
    PRIVATE c_std_17
            cxx_std_23
)

target_compile_definitions(
    ${PROJECT_NAME}
    PRIVATE UNICODE
            WIN32_LEAN_AND_MEAN
            NOMINMAX
            GDIPVER=0x0110
)

target_compile_options(
    ${PROJECT_NAME}
    PRIVATE $<$<CXX_COMPILER_ID:MSVC>:
            /W4
            /WX
            /MP
            /utf-8
            /bigobj
            /diagnostics:caret
            /Zc:__cplusplus
            >
            $<$<AND:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_FRONTEND_VARIANT:MSVC>>:
            /W4
            /WX
            >
            $<$<AND:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_FRONTEND_VARIANT:GNU>>:
            -Wall
            -Werror
            >
)

target_link_options(
    ${PROJECT_NAME}
    PRIVATE
    $<$<CXX_COMPILER_ID:MSVC>:
    /WX
    >
    $<$<AND:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_FRONTEND_VARIANT:MSVC>>:
    /WX
    >
    $<$<AND:$<CXX_COMPILER_ID:Clang>,$<CXX_COMPILER_FRONTEND_VARIANT:GNU>>:
    -Wl,/WX
    >
)

target_sources(
    ${PROJECT_NAME}
    PRIVATE "src/main.cxx"
)

target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE Urlmon.Lib
)

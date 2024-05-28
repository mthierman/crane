cmake_minimum_required(VERSION 3.28)

project(crane)

include(libs/mthierman/cmake-modules-main/common.cmake)

add_executable(${PROJECT_NAME})

target_sources(
    ${PROJECT_NAME}
    PRIVATE libcurl/main.cxx
)

target_link_libraries(
    ${PROJECT_NAME}
    PRIVATE common::flags
            common::definitions
            common::features
)

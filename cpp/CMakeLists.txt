# CMakeLists.txt for Advent of Code problems
# Including also GTEST Dependancy resolutions

cmake_minimum_required(VERSION 3.14)

project(adventOfCode LANGUAGES C CXX)

# Setting C++ version 20
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED True)

# Setting up Google Test
include(FetchContent)
FetchContent_Declare(
    googletest
    GIT_REPOSITORY https://github.com/google/googletest.git
    GIT_TAG        release-1.12.1
)

# For Windows ...
set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

add_library(
    utils_libs
    STATIC utils/src/utils.cpp
)

target_include_directories(
    utils_libs 
    INTERFACE utils/interface/
    PRIVATE utils/inc/
)

add_executable(
    day1_exec
    day1/main.cpp
    day1/src/problem1.cpp
)


target_include_directories(
    day1_exec
    PRIVATE day1/inc/
)

target_link_libraries(day1_exec PUBLIC utils_libs)

enable_testing()
add_executable(
    adventOfCode_tests
    day1/tests/testMain.cpp
    day1/src/problem1.cpp
)

target_include_directories(
    adventOfCode_tests
    PRIVATE day1/inc/
)

target_link_libraries(
    adventOfCode_tests
    GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(adventOfCode_tests)
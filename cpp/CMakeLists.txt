# CMakeLists.txt for Advent of Code problems
# Including also GTEST Dependancy resolutions

cmake_minimum_required(VERSION 3.14)

project(adventOfCode LANGUAGES CXX)

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

enable_testing()
add_executable(
    adventOfCode
    tutorial/main.cpp
)

target_include_directories(
    adventOfCode 
    PUBLIC tutorial/inc/
)

add_executable(
    tutorialtests
    tutorial/tests/tutorialtest.cpp
    tutorial/src/tutorial.cpp
)

target_include_directories(
    tutorialtests
    PUBLIC tutorial/inc/
)

target_link_libraries(
    tutorialtests
    GTest::gtest_main
)

include(GoogleTest)
gtest_discover_tests(tutorialtests)
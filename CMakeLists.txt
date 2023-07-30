# reference: http://google.github.io/googletest/quickstart-cmake.html
cmake_minimum_required(VERSION 3.24)
project(ray_tracing_in_series_tests)

# Standard: C++17
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# use FetchContent module in order to use googletest
include(FetchContent)
FetchContent_Declare(
  googletest
  URL https://github.com/google/googletest/archive/03597a01ee50ed33e9dfd640b249b4be3799d395.zip
)

set(gtest_force_shared_crt ON CACHE BOOL "" FORCE)
FetchContent_MakeAvailable(googletest)

enable_testing()

include_directories("ray_tracer/utils")

add_executable(
  project_test
  tests/utils/json_parser_test.cpp
  ray_tracer/utils/json_parser.cpp
)

target_link_libraries(project_test GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(project_test)

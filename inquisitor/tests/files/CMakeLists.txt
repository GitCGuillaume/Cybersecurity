cmake_minimum_required(VERSION 3.16)

project(baz LANGUAGES CXX VERSION 0.0.1)

find_package(Catch2 REQUIRED)
add_executable(tests test.cpp)
target_link_libraries(tests PRIVATE Catch2::Catch2)

include(CTest)
include(Catch)
catch_discover_tests(tests)

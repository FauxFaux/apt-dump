cmake_minimum_required(VERSION 3.8.2)
project(apt-dump)

set(CMAKE_CXX_STANDARD 14)

# sanitise
set (CMAKE_CXX_FLAGS_DEBUG "${CMAKE_CXX_FLAGS_DEBUG} -fno-omit-frame-pointer -fsanitize=address")
set (CMAKE_LINKER_FLAGS_DEBUG "${CMAKE_STATIC_LINKER_FLAGS_DEBUG} -fno-omit-frame-pointer -fsanitize=address")

set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -pedantic")

find_library(APT_PKG REQUIRED NAMES apt-pkg)

find_package(CapnProto REQUIRED)
include_directories(${CAPNP_INCLUDE_DIRS})
add_definitions(${CAPNP_DEFINITIONS})
set(CAPNPC_SRC_PREFIX "..")
set(CAPNPC_OUTPUT_DIR ${CMAKE_CURRENT_BINARY_DIR})
include_directories(${CAPNPC_OUTPUT_DIR})
capnp_generate_cpp(CAPNP_SRCS CAPNP_HDRS ../apt.capnp)

find_library(CAPNP_A_CAPNP NAMES libcapnp.a HINTS "/usr/lib/x86_64-linux-gnu")
find_library(CAPNP_A_KJ NAMES libkj.a HINTS "/usr/lib/x86_64-linux-gnu")
set(LINKER_FLAGS "-Wl,--gc-sections")

set(SOURCE_FILES main.cpp)
add_executable(apt-dump ${SOURCE_FILES} ${CAPNP_SRCS} ${CAPNP_HDRS})
target_link_libraries(apt-dump ${APT_PKG} ${CAPNP_A_CAPNP} ${CAPNP_A_KJ})
set_target_properties(apt-dump PROPERTIES INTERPROCEDURAL_OPTIMIZATION TRUE)

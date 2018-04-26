cmake_minimum_required(VERSION 3.9)
project(empire-examples)

include_directories(${CMAKE_SOURCE_DIR}/../libempire/include)
link_directories(${CMAKE_SOURCE_DIR}/../target/debug)

link_libraries(empire advapi32 ws2_32 userenv shell32)

add_executable(hello_mpi hello_mpi.c)
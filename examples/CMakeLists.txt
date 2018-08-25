cmake_minimum_required(VERSION 3.9)
project(empire-examples C CXX)

include_directories(${CMAKE_SOURCE_DIR}/../libempire/include)
link_directories(${CMAKE_SOURCE_DIR}/../target/x86_64-pc-windows-msvc/debug)

if (WIN32)
    link_libraries(empire advapi32 ws2_32 userenv shell32)
else()
    link_libraries(empire)
endif()

add_executable(hello_mpi hello_mpi.c)
add_executable(open_port open_port.c)
add_executable(spawn_self spawn_self.cpp)
add_executable(spawn_self_w spawn_self_w.cpp)
add_executable(info info.cpp)
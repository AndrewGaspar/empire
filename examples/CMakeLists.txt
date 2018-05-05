cmake_minimum_required(VERSION 3.9)
project(empire-examples C)

include_directories(${CMAKE_SOURCE_DIR}/../libempire/include)
link_directories(${CMAKE_SOURCE_DIR}/../target/debug)

if (WIN32)
    link_libraries(empire.dll.lib advapi32 ws2_32 userenv shell32)
else()
    link_libraries(empire)
endif()

add_executable(hello_mpi hello_mpi.c)
add_executable(open_port open_port.c)
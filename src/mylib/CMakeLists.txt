cmake_minimum_required(VERSION 3.14)
project(mylib)

add_library(mylib lib.c)

install(
    TARGETS mylib
    ARCHIVE DESTINATION ${CMAKE_INSTALL_PREFIX}
)
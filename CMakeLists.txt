cmake_minimum_required(VERSION 3.10)

project(AlphaShershMurderer)

add_executable(main src/main.c)     # main file

add_executable(test1 test/test1.c)  # test 1

enable_testing()

add_test(TEST1 test1)

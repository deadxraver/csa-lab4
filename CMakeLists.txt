cmake_minimum_required(VERSION 3.10)

project(AlphaShershMurderer)

# executables          vvvvvvvvvvvvvv

add_executable(asm-compile src/translator.c)            # translator binary

add_executable(test1 test/translator_argparser.c)       # test 1

enable_testing()

add_test(TEST1 test1)

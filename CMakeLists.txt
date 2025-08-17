cmake_minimum_required(VERSION 3.10)

project(AlphaShershMurderer)

# executables          vvvvvvvvvvvvvv

add_executable(asm-compile
    src/translator.c 
    src/translator/argparser.c src/translator/tokenizer.c)        # translator binary

add_executable(test1 test/translator_argparser_success.c src/translator/argparser.c)       # test 1

add_executable(test2 test/translator_argparser_error.c src/translator/argparser.c)

enable_testing()

add_test(TEST1 test1)

add_test(TEST2 test2)

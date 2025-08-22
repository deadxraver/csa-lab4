cmake_minimum_required(VERSION 3.10)

project(AlphaShershMurderer)

# executables          vvvvvvvvvvvvvv

add_executable(asm-compile
        src/translator.c
        src/translator/argparser.c src/translator/tokenizer.c
        src/preprocessor/preprocessor.c
        src/preprocessor/preprocessor.h)        # translator binary

add_executable(test1 test/translator_argparser_success.c src/translator/argparser.c)       # test 1

add_executable(test2 test/translator_argparser_error.c src/translator/argparser.c)         # test 2

add_executable(test3 test/translator_tokenizer_free.c src/translator/tokenizer.c)

enable_testing()

add_test(TEST1 test1)

add_test(TEST2 test2)

add_test(TEST3 test3)
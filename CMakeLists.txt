cmake_minimum_required(VERSION 3.10)

project(AlphaShershMurderer)

add_executable(
    asm-compile
    src/translator.c
    src/translator/argparser.c
    src/translator/tokenizer.c
    src/preprocessor/preprocessor.c
)

add_executable(
    test1
    test/translator_argparser_success.c
    src/translator/argparser.c
)

add_executable(
    test2
    test/translator_argparser_error.c
    src/translator/argparser.c
)

#[[
add_executable(
    test3
    test/preprocessor_error.c
    src/preprocessor/preprocessor.c
)
]]

add_executable(
    test4
    test/preprocessor_success.c
    src/preprocessor/preprocessor.c
)

enable_testing()

add_test(ARGPARSER_OK test1)

add_test(ARGPARSER_ERR test2)

# add_test(PREPROC_ERR test3)

add_test(PREPROC_OK test4)

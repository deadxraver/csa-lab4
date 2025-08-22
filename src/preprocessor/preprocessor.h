#ifndef PREPROCESSOR_H
#define PREPROCESSOR_H

#include <malloc.h>

enum PreprocessorError {
  PREPROCESS_SUCCESS = 0,
  UNKNOWN_MACRO,
  UNKNOWN_ERROR,
  // TODO: other codes
};

enum PreprocessorError preprocess_code(char* source);

#endif  // PREPROCESSOR_H

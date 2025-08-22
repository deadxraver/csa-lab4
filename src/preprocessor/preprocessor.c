#include "preprocessor.h"

enum PreprocessorError preprocess_code(char* source) {
  // TODO: function body
  //  * actually I am not sure how to handle strings in C of this type
  //  * so I need to change its size but dont know how much I need
  //  * e.g. `#define CONST 10` becomes nothing and `print(CONST);` becomes `print(10);`
  return PREPROCESS_SUCCESS;
}
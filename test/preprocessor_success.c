#include "../src/preprocessor/preprocessor.h"
#include <string.h>
#include <assert.h>
#include <malloc.h>

char initial_code[] = "%define ONE_CONST 1\nONE_CONST";
char preprocessed_code[] = "1";

int main() {
  char* result_code = preprocess_code(initial_code);
  if (result_code == NULL) {
    fprintf(stderr, "return value is NULL, expected: %s\n", preprocessed_code);
    return -1;
  }
  assert(strcmp(result_code, preprocessed_code) == 0);
  free(result_code);
  result_code = NULL;
  return 0;
}

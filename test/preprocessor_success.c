#include "../src/preprocessor/preprocessor.h"
#include <string.h>
#include <assert.h>
#include <malloc.h>

char initial_code[] = "%define ONE_CONST 1\nONE_CONST";
char preprocessed_code[] = "1\n";

int main() {
  int err_code = 0;
  char* result_code = preprocess_code(initial_code);
  if (result_code == NULL) {
    fprintf(stderr, "return value is NULL, expected:\n```\n%s\n```\n", preprocessed_code);
    err_code = -1;
    goto end;
  }
  if (strcmp(result_code, preprocessed_code) != 0) {
    fprintf(stderr, "expected:\n```\n%s\n```\n", preprocessed_code);
    fprintf(stderr, "got:\n```\n%s\n```\n", result_code);
    err_code = -1;
  }
end:
  free(result_code);
  result_code = NULL;
  return err_code;
}

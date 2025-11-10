#include "../src/preprocessor/preprocessor.h"
#include <stdio.h>

char source_code[MAX_STR] = "%unknown 1\nunknown\n";

int main() {
  char* preprocessed = preprocess_code(source_code);
  if (preprocessed != NULL) {
    fprintf(stderr, "Expected NULL, but instead got:\n```\n%s\n```\n", preprocessed);
    return -1;
  }
  return 0;
}

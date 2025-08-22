#include <assert.h>

#include "../src/translator/argparser.h"

int main() {
  int argc = 3;
  char* argv[3] = {
      "asm-compile",
      "--verbose",
      "--preprocess-only",
  };
  struct ParseResults expect = DEFAULT_PARSE_RESULTS;
  expect.error_code = NO_FILE_ERROR;
  struct ParseResults result = parse_args(argc, argv);
  assert(expect.error_code == result.error_code);
  return 0;
}

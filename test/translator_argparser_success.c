#include <assert.h>
#include "../src/translator/argparser.h"

int main() {
  int argc = 3;
  char* argv[3] = {
    "asm-compile",
    "--verbose",
    "main.asm",
  };
  struct ParseResults expect = DEFAULT_PARSE_RESULTS;
  expect.error_code = NO_ERROR;
  expect.verbose = true;
  expect.preprocess_only = false;
  expect.help_message_only = false;
  expect.filename = "main.asm";
  struct ParseResults result = parse_args(argc, argv);
  assert(expect.error_code == result.error_code);
  assert(expect.verbose == result.verbose);
  assert(expect.preprocess_only == result.preprocess_only);
  assert(expect.help_message_only == result.help_message_only);
  assert(expect.filename == result.filename);
  return 0;
}

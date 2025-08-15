#include <assert.h>
#include "../src/translator/argparser.h"

int main() {
  int argc = 3;
  char* argv[3] = {
    "translator",
    "--verbose",
    "main.asm",
  };
  struct ParseResults expect = {
    .error_code = NO_ERROR,
    .verbose = true,
    .preprocess_only = false,
    .help_message_only = false,
  };
  struct ParseResults result = parse_args(argc, argv);
  assert(expect.error_code == result.error_code);
  assert(expect.verbose == result.verbose);
  assert(expect.preprocess_only == result.preprocess_only);
  assert(expect.help_message_only == result.help_message_only);
  return 0;
}

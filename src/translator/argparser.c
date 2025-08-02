#include "translator.h"

void print_message(struct ParseResults parse_results, char* argv) {
  if (parse_results.error_code == NO_ERROR) {
    return ;
  }
  if (parse_results.error_code == NO_ARGS_ERROR) {
    fprintf(stderr, "No args specified! Run `%s --help` to see help message\n", argv[0]);
    return ;
  }
  if (parse_results.error_code == UNKNOWN_ARG_ERROR) {
    fprintf(stderr, "Unknown arg: %s\n", argv[parse_results.pos]);
    return ;
  }
  // TODO: other cases
}

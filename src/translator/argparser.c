#include "argparser.h"

void print_message(struct ParseResults parse_results, char* argv[]) {
  if (parse_results.error_code == NO_ERROR) {
    return ;
  }
  if (parse_results.error_code == NO_FILE_ERROR) {
    fprintf(stderr, "No input file specified! Run `%s --help` to see help message\n", argv[0]);
    return ;
  }
  if (parse_results.error_code == UNKNOWN_ARG_ERROR) {
    fprintf(stderr, "Unknown arg: %s\n", argv[parse_results.pos]);
    return ;
  }
  if (parse_results.error_code == NO_SUCH_FILE) {
    fprintf(stderr, "No such file: %s\n", argv[parse_results.pos]);
    return ;
  }
  fprintf(stderr, "Unknown error: %d\n", parse_results.error_code);
}

struct ParseResults parse_args(int argc, char* argv[]) {
  if (argc == 1) {
    return (struct ParseResults) { .error_code = NO_FILE_ERROR };
  }
  struct ParseResults result = DEFAULT_PARSE_RESULTS;
  for (size_t i = 1; i < argc; ++i) {
    if (!strcmp("--help", argv[i])) {
      result.help_message_only = true;
    }
    else if (!strcmp("--verbose", argv[i])) {
      result.verbose = true;
    }
    else if (!strcmp("--preprocess-only", argv[i])) {
      result.preprocess_only = true;
    }
    else if (argv[i][0] == '-' || result.filename != NULL) {
      result.error_code = UNKNOWN_ARG_ERROR;
      result.pos = i;
      break;
    }
    else {
      result.filename = argv[i];
    }
  }
  return result;
}

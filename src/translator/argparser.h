#include <stdio.h>
#include <stdbool.h>

enum ErrorCode {
  NO_ERROR = 0,
  NO_ARGS_ERROR,
  UNKNOWN_ARG_ERROR,
};

struct ParseResults {
  enum ErrorCode error_code;
  size_t pos;
  bool verbose;
};

void print_message(struct ParseResults parse_results, char* argv[]);


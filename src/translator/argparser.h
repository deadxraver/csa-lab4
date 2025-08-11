#include <stdio.h>
#include <stdbool.h>

enum ErrorCode {
  NO_ERROR = 0,
  NO_ARGS_ERROR,
  UNKNOWN_ARG_ERROR,
  NO_SUCH_FILE,
};

struct ParseResults {
  enum ErrorCode error_code;
  size_t pos;
  bool verbose;
  bool preprocess_only;
};

void print_message(struct ParseResults parse_results, char* argv[]);

struct ParseResults parse_args(int argc, char* argv[]);

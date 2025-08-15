#ifndef TRANSLATOR_ARGPARSER_H

#define TRANSLATOR_ARGPARSER_H

#include <stdio.h>
#include <stdbool.h>
#include <string.h>

enum ErrorCode {
  NO_ERROR = 0,
  NO_FILE_ERROR,
  UNKNOWN_ARG_ERROR,
  NO_SUCH_FILE_ERROR,
};

struct ParseResults {
  enum ErrorCode error_code;
  size_t pos;
  bool verbose;
  bool preprocess_only;
  bool help_message_only;
  char* filename;
};

#define DEFAULT_PARSE_RESULTS (struct ParseResults) {.error_code = NO_ERROR,.pos = 0,.verbose = false,.preprocess_only = false,.help_message_only = false,.filename = NULL,}

void print_message(struct ParseResults parse_results, char* argv[]);

struct ParseResults parse_args(int argc, char* argv[]);

#endif // !TRANSLATOR_ARGPARSER_H

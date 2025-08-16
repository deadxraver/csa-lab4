#include <stdlib.h>

#include "translator/argparser.h"

int main(int argc, char* argv[]) {
  struct ParseResults parse_results = parse_args(argc, argv);
check_err:
  print_message(parse_results, argv);
  if (parse_results.error_code) {
    return parse_results.error_code;
  }
  if (parse_results.help_message_only)
    return 0;
  FILE* input_file = fopen(parse_results.filename, "r");
  if (input_file == NULL) {
    parse_results.error_code = NO_SUCH_FILE_ERROR;
    goto check_err;
  }
  fseek(input_file, 0L, SEEK_END);
  size_t sz = ftell(input_file);
  rewind(input_file);
  char* s = (char*)malloc(sizeof(char) * sz);
  if (fgets(s, sz, input_file) == NULL) {
    fprintf(stderr, "Error when reading from file\n");
    return READ_ERROR;
  }
  // free everything
  fclose(input_file);
  free(s);
  return 0;
}

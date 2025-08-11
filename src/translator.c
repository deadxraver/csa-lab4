#include "translator/argparser.h"

int main(int argc, char* argv[]) {
  struct ParseResults parse_results = parse_args(argc, argv);
  print_message(parse_results, argv);
  if (parse_results.error_code) {
    return parse_results.error_code;
  }
  // TODO:
  return 0;
}

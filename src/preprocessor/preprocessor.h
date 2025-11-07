#ifndef PREPROCESSOR_H

#define PREPROCESSOR_H

#define MAX_STR 64

#include <malloc.h>

struct StringNode {
  char str[MAX_STR];
  struct StringNode* next;
  struct StringNode* prev;
};

/**
 * Returns pointer to nul-terminated string.
 * Note that str should be cleaned after usage.
 */
char* preprocess_code(char* code);

#endif // !PREPROCESSOR_H

#ifndef PREPROCESSOR_PREPROCESSOR_H

#define PREPROCESSOR_PREPROCESSOR_H

#define MAX_STR 64

struct StringNode {
  char str[MAX_STR];
  struct StringNode* next;
  struct StringNode* prev;
};

char* preprocess_code(char* code);

#endif // !PREPROCESSOR_PREPROCESSOR_H

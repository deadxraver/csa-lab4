#include "tokenizer.h"

struct SectionCollection* tokenize_code(char* text) {
  return NULL; /* заглушка */
}

void free_sections(struct SectionCollection* sections) {
  if (sections) {
    if (sections->sections) {
      for (size_t i = 0; i < sections->sz; ++i) {
        if (sections->sections[i].cells) {
          free(sections->sections[i].cells);
          sections->sections[i].cells = NULL;
        }
      }
      free(sections->sections);
      sections->sections = NULL;
    }
    free(sections);
    sections = NULL;
  }
}

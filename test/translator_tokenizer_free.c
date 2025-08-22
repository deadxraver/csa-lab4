#include "../src/translator/tokenizer.h"

int main() {
  free_sections(NULL);  // should not fail
  struct SectionCollection* section_collection =
      (struct SectionCollection*)malloc(sizeof(struct SectionCollection));
  section_collection->sections = (struct Section*)malloc(sizeof(struct Section) * 3);
  section_collection->sz = 3;
  section_collection->sections[0].cells = NULL;
  section_collection->sections[1].cells = (struct Cell*) malloc(sizeof(struct Cell) * 10);
  section_collection->sections[2].cells = NULL;
  free_sections(section_collection);  // same, should not segfault
  return 0;
}

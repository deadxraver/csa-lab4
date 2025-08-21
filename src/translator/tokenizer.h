#include <malloc.h>
#include <stdint.h>

enum SectionType {
  DATA,
  TEXT,
};

enum MachineWord {
  PUSH_IMM = 0x1,
  PUSH_A,
  PUSH_B,
  POP_A,
  POP_B,
  FETCH_A,
  FETCH_B,
  FETCH,
  STORE_A,
  STORE_B,
  DUP,
  D2R,
  R2D,
  SWP,
  ADD,
  SUB,
  MUL,
  DIV,
  AND,
  OR,
  XOR,
  NOT,
  CMP,
  CALL,
  CALL_TOP,
  BEQZ,
  BNZ,
  BN,
  BP,
  RET,
  PUSH_RAND,
  HALT,
};

enum CellType {
  MACHINE_WORD,
  DATA_BYTE,
  DATA_WORD,
};

union MachineCommand {
  enum MachineWord machine_word;
  uint8_t data_byte;
  uint32_t data_word;
};

struct Cell {
  enum CellType cell_type;
  union MachineCommand machine_command;
};

struct Section {
  uint32_t section_start;
  enum SectionType section_type;
  struct Cell* cells;
  size_t cells_length;
};

struct SectionCollection {
  struct Section* sections;
  size_t sz;
};

struct SectionCollection* tokenize_code(char* text);

void free_sections(struct SectionCollection* sections);

#include <string.h>
#include <malloc.h>
#include <stdbool.h>

#include "preprocessor.h"

/**
 * Split given string into linked nodes and return head.
 * Break char is \n.
*/
struct StringNode* split_code(char* code) {
  struct StringNode* list = (struct StringNode*) malloc(sizeof(struct StringNode));
  size_t big_i = 0;
  size_t i;
  memset(list, 0, MAX_STR);
  if (list == NULL) return NULL;
  for (struct StringNode* l = list; ; l = l->next) {
    for (i = 0; code[i + big_i] != '\n'; ++i) {
      l->str[i] = code[i + big_i];
      if (code[i + big_i] == 0) {
        l->next = NULL;
        goto end;
      }
    }
    big_i += i + 1;
    l->next = (struct StringNode*) malloc(sizeof(struct StringNode));
    l->next->prev = l;
  }
end:
  return list;
}

/**
 * Combine nodes to single string separated by \n's.
 * Returns allocated string or NULL if nodes are empty.
*/
char* join_nodes(struct StringNode* node) {
  size_t sz = 0;
  char* code;
  for (struct StringNode* n = node; n; n = n->next) {
    sz += strlen(n->str) + 1;
  }
  if (sz == 0) return NULL;
  code = (char*) malloc(sizeof(char) * (sz + 1));
  code[0] = 0;
  for (struct StringNode* n = node; n; n = n->next) {
    strcat(code, n->str);
    strcat(code, "\n");
  }
  return code;
}

/**
 * Free nodes one by one.
*/
void free_nodes(struct StringNode* sn) {
  for (; sn; sn = sn->next) {
    struct StringNode* next = sn->next;
    free(sn);
    sn = next;
  }
}

/**
 * Replace str first word by rep.
*/
void str_replace(char* str, char* rep) {
  char* cc;
  char buf[MAX_STR];
  for (cc = str; *cc != ' ' && *cc != 0 && *cc != '\t' && *cc != '\n'; ++cc); // skip word to be replaced
  strcpy(cc, buf);
  strcpy(rep, str);
  size_t sz = strlen(str);
  str[sz] = ' ';
  str[sz+1] = 0;
  strcat(str, buf);
}

/**
 * Replace all name's by body's in node's.
*/
void replace_all(struct StringNode* node, char name[MAX_STR], char body[MAX_STR]) {
  size_t name_sz = strlen(name);
  for (; node; node = node->next) {
    for (size_t i = 0; i < MAX_STR && node->str[i]; ++i) {
      if (strncmp(name, node->str + i, name_sz) == 0)
        str_replace(node->str + i, body);
    }
  }
}

/**
 * Preprocess given code and return allocated string.
 * Note that returned value should be freed.
*/
char* preprocess_code(char* code) {
  struct StringNode* node = split_code(code);
  bool comment_section = false;
  bool string = false;
  char* preprocessed = NULL;
  for (struct StringNode* n = node; n; n = n->next) {
    for (size_t i = 0; n->str[i]; ++i) {
      if (n->str[i] == '"' && !comment_section)
        string = !string;
      if (n->str[i] == ';' && !string)
        comment_section = true;
      if (n->str[i] == '\n') {
        string = false;
        comment_section = false;
      }
      if (comment_section || string)
        continue;
      if (n->str[i] == '#' && strncmp(n->str + i + 1, "define", strlen("define")) == 0) {
        struct StringNode* np = n->prev; // remove line from code
        n->prev->next = n->next;
        n->next->prev = n->prev; // detach node
        i += strlen("define"); // skip macro word
        for (; n->str[i] == ' ' || n->str[i] == '\t'; ++i); // skip whitespaces
        char* macro_name_p = n->str + i; // name pointer
        for (; n->str[i] != ' ' && n->str[i] != '\t'; ++i); // skip macro name
        size_t sz = n->str + i - macro_name_p;
        char macro_name[MAX_STR];
        memcpy(macro_name_p, macro_name, sz);
        macro_name[sz] = 0;
        sz = 0;
        char* macro_body_p = n->str + i;
        for (; n->str[i] != '\n' && n->str[i] != 0; ++i, ++sz);
        char macro_body[MAX_STR];
        memcpy(macro_body_p, macro_body, sz);
        macro_body[sz] = 0;
        replace_all(n->next, macro_name, macro_body);
        free(n);
        n = np;
        break;
      } else if (n->str[i] == '#' && strncmp(n->str + i + 1, "include", strlen("include")) == 0) {
        struct StringNode* np = n->prev;
        n->prev->next = n->next;
        n->next->prev = n->prev;
        // TODO: process include
        free(n);
        n = np;
      } else if (n->str[i] == '#') {
        // Unknown macro
        fprintf(stderr, "Unknown macro: ");
        for (; n->str[i] != '\n' && n->str[i] != ' ' && n->str[i] != '\t'; ++i)
          fprintf(stderr, "%c", n->str[i]);
        fprintf(stderr, "\n");
        preprocessed = NULL;
        goto end;
      }
    }
  }
  preprocessed = join_nodes(node);
end:
  free_nodes(node);
  node = NULL;
  return preprocessed;
}

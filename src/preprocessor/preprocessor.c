#include <string.h>
#include <malloc.h>
#include <stdbool.h>

#include "preprocessor.h"

void print_nodes(struct StringNode* node) {
  int i = 0;
  for (; node; node = node->next) {
    printf("%d: %s\n", ++i, node->str);
  }
}

/**
 * Remove node from list and return pointer to previous element.
*/
static struct StringNode* detach_node(struct StringNode* node) {
  if (node == NULL)
    return NULL;
  struct StringNode* prev = node->prev;
  if (prev) {
    prev->next = node->next;
  }
  if (node->next) {
    node->next->prev = prev;
  }
  return prev;
}

/**
 * Remove all trailing and leading spaces from str
*/
void trim(char str[MAX_STR]) {
  size_t i;
  for (i = 0; (str[i] == ' ' || str[i] == '\t') && i < MAX_STR; ++i); // skip leading whitespaces
  if (i == MAX_STR - 1) // smth's wrong
    return ;
  if (i != 0)
    strcpy(str, str + i);
  for (size_t j = 0; j < MAX_STR && str[j]; ++j) {
    if (str[j] != ' ' && str[j] != '\t')
      i = j + 1;
  }
  str[i] = 0;
}

/**
 * Split given string into linked nodes and return head.
 * Break char is \n.
*/
struct StringNode* split_code(char* code) {
  struct StringNode* list = (struct StringNode*) malloc(sizeof(struct StringNode));
  size_t big_i = 0;
  size_t i;
  memset(list->str, 0, MAX_STR); // first node is empty so that head is never pointing to invalid memory segment
  list->next = (struct StringNode*) malloc(sizeof(struct StringNode));
  memset(list->next->str, 0, MAX_STR);
  list->next->prev = list;
  list->next->next = NULL;
  list->prev = NULL;
  if (list == NULL) return NULL;
  for (struct StringNode* l = list->next; ; l = l->next) {
    for (i = 0; code[i + big_i] != '\n'; ++i) {
      l->str[i] = code[i + big_i];
      if (code[i + big_i] == 0) {
        l->next = NULL;
        goto end;
      }
    }
    big_i += i + 1;
    l->next = (struct StringNode*) malloc(sizeof(struct StringNode));
    l->next->next = NULL;
    memset(l->next->str, 0, MAX_STR);
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
    trim(n->str);
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
  strcpy(buf, cc);
  strcpy(str, rep);
  size_t sz = strlen(rep);
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
      if (strlen(node->str + i) < strlen(name))
        break;
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
      if (strlen(n->str + i + 1) >= strlen("%define") && n->str[i] == '%' && strncmp(n->str + i + 1, "define", strlen("define")) == 0) {
        struct StringNode* np = detach_node(n); // remove node from list
        i += strlen("%define"); // skip macro word
        for (; n->str[i] == ' ' || n->str[i] == '\t'; ++i); // skip whitespaces
        char* macro_name_p = n->str + i; // name pointer
        for (; n->str[i] != ' ' && n->str[i] != '\t'; ++i); // skip macro name
        size_t sz = n->str + i - macro_name_p;
        char macro_name[MAX_STR];
        memcpy(macro_name, macro_name_p, sz);
        macro_name[sz] = 0;
        sz = 0;
        for (; n->str[i] == ' ' || n->str[i] == '\t'; ++i); // skip whitespaces
        char* macro_body_p = n->str + i;
        for (; n->str[i] != '\n' && n->str[i] != 0; ++i, ++sz);
        char macro_body[MAX_STR];
        memcpy(macro_body, macro_body_p, sz);
        macro_body[sz] = 0;
        replace_all(n->next, macro_name, macro_body);
        free(n);
        n = np;
        break;
      } else if (strlen(n->str + i + 1) >= strlen("%include") && n->str[i] == '%' && strncmp(n->str + i + 1, "include", strlen("include")) == 0) {
        struct StringNode* np = detach_node(n);
        // TODO: process include
        free(n);
        n = np;
      } else if (n->str[i] == '%') {
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
  preprocessed = join_nodes(node->next);
end:
  free_nodes(node);
  node = NULL;
  return preprocessed;
}

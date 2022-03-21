#ifndef XC_H_DEFINED
#define XC_H

#define VERSION "0.0.1"

typedef struct wFile { 
  int lines;
  int words;
  int chars;
} wFile;

enum {
  LINE = 1,
  CHAR,
  WORD,
  ALL,
};

#endif

#ifndef XC_H_DEFINED
#define XC_H

#define VERSION "0.0.1"
#define USAGE "Usage:\n\txc [FILES] [OPTIONS]\n\n-m \t Print characters in file\n-l \t Print lines in file\n-w \t Print words in file\n"

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

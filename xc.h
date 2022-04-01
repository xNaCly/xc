#ifndef XC_H_DEFINED
#define XC_H

#define VERSION "0.0.1"
#define USAGE "Usage: \
  \n\txc [FILES] [OPTIONS]\
  \n \
  \n-m  --chars \n\t Print characters in file\n \
  \n-l  --lines \n\t Print lines in file\n \
  \n-w  --words \n\t Print words in file\n"

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

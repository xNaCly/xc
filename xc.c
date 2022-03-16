#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>

#include "xc.h"

typedef struct wFile { 
  int lines;
  int words;
  int chars;
} wFile;

static void throw_error(const char* text){
  printf("xc: %s\n", text);
  exit(EXIT_FAILURE);
} 
static int isFile(const char* target){
  struct stat statbuf;
  stat(target, &statbuf);
  return S_ISDIR(statbuf.st_mode);
}

wFile* work_file(const char* filename){
  wFile *f = malloc( sizeof(int) * 3 );
  f->lines = 0;
  f->words = 0;
  f->chars = 0;

  if(isFile(filename))
    return NULL;

  char ch;
  FILE *file = fopen(filename, "r");

  if(file == NULL){
    char temp[559];
    sprintf(temp, "xc: %s", filename);
    perror(temp);
    exit(EXIT_FAILURE);
  }
  
  while((ch=fgetc(file))!=EOF){
    f->chars++;
    if(ch == '\0' || ch=='\n'){
      f->lines++;
    }
    if(ch == ' ')
      f->words++;
  }

  return f;
}

int main(int argc, const char *argv[]){
  int lines = 0;
  int words = 0;
  int chars = 0;

  char filename[10][555];
  int amount_of_files = 0;
  int mode = 0;

  if(argc == 1)
    throw_error("Not enough arguments.");

  for(int i = 1; i < argc; i++){
    if(argv[i][0] == '-'){
      if(strcmp("-v", argv[i]) == 0 || strcmp("--version", argv[i]) == 0){
        printf("xc-%s\n", VERSION);
        return EXIT_SUCCESS;
      } else if(strcmp("-l", argv[i]) == 0 || strcmp("--lines", argv[i]) == 0){
        mode = LINE;
      } else if(strcmp("-m", argv[i]) == 0 || strcmp("--chars", argv[i]) == 0){
        mode = CHAR;
      } else if(strcmp("-w", argv[i]) == 0 || strcmp("--words", argv[i]) == 0){
        mode = WORD;
      } else if(strcmp("--help", argv[i]) == 0 || strcmp("-h", argv[i]) == 0 ){
        printf("Usage:"
            "\n\txc [FILES] [OPTIONS]"
            "\n\n -m \t Print characters in file\n"
            " -l \t Print lines in file\n"
            " -w \t Print words in file\n"
            "\nExample:"
            "\n Counting words in main.c and test.c:"
            "\n\n\t xc main.c test.c -w"
            "\n\n Counting lines for all files in current directory:"
            "\n\n\t xc * -l"
            "\n\nVersion: \n\tv%s\n", VERSION
            );
        exit(EXIT_SUCCESS);
      }
      else {
        mode = ALL;
      }
    }else if(argv[i][0] != '-'){
      strcpy(filename[i-1], argv[i]);
      amount_of_files++;
    } 
  }

  if(amount_of_files < 1)
    throw_error("No file specified");


  for(int i = 0; i < amount_of_files; i++){
    wFile *f = work_file(filename[i]);
    if(f == NULL)
      continue;

    switch(mode){
      case(CHAR):
        printf(" %5d %-5s\n", f->chars, filename[i]);
        break;
      case(LINE):
        printf(" %5d %-5s\n", f->lines, filename[i]);
        break;
      case (WORD):
        printf(" %5d %-5s\n", f->words, filename[i]);
        break;
      default:
        printf(" %5d %5d %5d %-5s\n", f->lines, f->words, f->chars, filename[i]);
    }

    chars += f->chars;
    lines += f->lines;
    words += f->words;
  }

  if(amount_of_files > 1){
    switch(mode){
      case(CHAR):
        printf(" %5d total\n", chars);
        break;
      case(LINE):
        printf(" %5d total\n", lines);
        break;
      case (WORD):
        printf(" %5d total\n", words);
        break;
      default:
        printf(" %5d %5d %5d total\n", lines, words, chars);
    }
   return EXIT_SUCCESS;
  }else {
    wFile *f = work_file(filename[0]);
    chars = f->chars;
    lines = f->lines;
    words = f->words;
  }

  return EXIT_SUCCESS;
}

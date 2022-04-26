# xc
xc - implementation of wc in various languages

## Languages:
- [C](/C)
- [Java](/Java)

## Implementation details:
The wc shell utility counts words, lines and characters by default for the specified file.
The following features must be implemented:
- support cli arguments:
  - `-m` / `--chars`: print only char count
  - `-l` / `--lines`: print only line count
  - `-w` / `--words`: print only word count
  - `-v` / `--version`: print the version and exit
  - `-h` / `--help`: print the usage screen and exit
- support multiple files, add a total sum after reading all files

[wc wiki page](https://en.wikipedia.org/wiki/Wc_(Unix))

## Principle:
- Use only standard lib methods shipped with the language
- Use as few libraries and library calls as possible
- Write readable, correct and simple code (**K**eep **I**t **S**imple and **S**tupid)
- Comment incomprehensible
- Comply with the Conventional Commit Messages [Standard](https://www.conventionalcommits.org/en/v1.0.0/)
- Provide build instructions / build scripts

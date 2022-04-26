# xc
xc - implementation of wc in various languages

## Languages:
- [C](/C)
- [Java](/Java)

## Build Requirements:
All documentation in this repo assumes a \*nix operating system, create a virtual machine for the least hussle [here](https://www.google.com/url?sa=t&rct=j&q=&esrc=s&source=web&cd=&cad=rja&uact=8&ved=2ahUKEwitx8m6tLH3AhVGnKQKHQABA8YQwqsBegQIAxAB&url=https%3A%2F%2Fwww.youtube.com%2Fwatch%3Fv%3DsB_5fqiysi4&usg=AOvVaw2NcyjuXi_VnBI2CAYt-b_W) `[Youtube]`.

Most build scripts are written using `make` and a `Makefile` and well documented.
Dependencies and more information on compiling / running implementations can be found in `<lang>/Readme`.

If something isn't clear, feel free to create an Issue with your Question.


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

[wc](https://en.wikipedia.org/wiki/Wc_(Unix)) `[Wikipedia]`

## Principle & Contribution standard:
- Use only standard lib methods shipped with the language
- Use as few libraries and library calls as possible
- Write readable, correct and simple code (**K**eep **I**t **S**imple and **S**tupid)
- Comment incomprehensible code
- Comply with the Conventional Commit Messages [Standard](https://www.conventionalcommits.org/en/v1.0.0/)
- Provide build instructions / build scripts

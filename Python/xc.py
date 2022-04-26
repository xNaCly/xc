from ast import arg

from click import echo
from Parser import Parser
import sys

MODE = {
    "all": 0,
    "lines": 1,
    "chars": 2,
    "words": 3
}


def work_file(filename: str):
    words: int = 0
    chars: int = 0
    lines: int = 0

    try:
        with open(filename, "r") as file:
            lines_ = file.readlines()
            lines = len(lines_)
            for line in lines_:
                chars += len(line)
                words += len(line.split())
    except:
        pass

    return {
        "name": filename,
        "words": words,
        "chars": chars,
        "lines": lines,
    }


def main():
    words: int = 0
    chars: int = 0
    lines: int = 0
    mode: int = MODE["all"]

    parser = Parser(sys.argv)
    arguments = parser.get_args()
    if len(arguments["args"]) < 1:
        raise Exception("No file specified!")

    flag = ""
    if len(arguments["flags"]):
        flag = arguments["flags"][0]
    if flag == "l" or flag == "lines":
        mode = MODE["lines"]
    elif flag == "w" or flag == "words":
        mode = MODE["words"]
    elif flag == "m" or flag == "chars":
        mode = MODE["chars"]

    for file in arguments["args"]:
        file = work_file(file)
        words += file['words']
        chars += file['chars']
        lines += file['lines']
        if mode == MODE["words"]:
            print(
                f"{file['words']} {file['name']}")
        elif mode == MODE["chars"]:
            print(
                f"{file['chars']} {file['name']}")
        elif mode == MODE["lines"]:
            print(
                f"{file['lines']} {file['name']}")
        else:
            print(
                f"{file['lines']} {file['words']} {file['chars']} {file['name']}")

    if len(arguments["args"]) > 1:
        if mode == MODE["words"]:
            print(f"{words} total")
        elif mode == MODE["chars"]:
            print(f"{chars} total")
        elif mode == MODE["lines"]:
            print(f"{lines} total")
        else:
            print(f"{lines} {words} {chars} total")


if __name__ == "__main__":
    main()

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
                chars += len(line.replace("\n", ""))
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

    flag = ""
    if len(arguments["flags"]):
        flag = arguments["flags"][0]

    if flag == "v" or flag == "version":
        print("xc - version: 1")
        return
    elif flag == "h" or flag == "help":
        print("usage")
        return
    elif flag == "l" or flag == "lines":
        mode = MODE["lines"]
    elif flag == "w" or flag == "words":
        mode = MODE["words"]
    elif flag == "m" or flag == "chars":
        mode = MODE["chars"]

    if len(arguments["args"]) < 1:
        raise Exception("No file specified!")

    for file in arguments["args"]:
        file = work_file(file)
        words += file['words']
        chars += file['chars']
        lines += file['lines']
        if mode == MODE["words"]:
            print(
                f"{file['words']:<3} {file['name']:<2}")
        elif mode == MODE["chars"]:
            print(
                f"{file['words']:<3} {file['name']:<2}")
        elif mode == MODE["lines"]:
            print(
                f"{file['words']:<3} {file['name']:<2}")
        else:
            print(
                f"{file['lines']:<3} {file['words']:<2} {file['chars']:<2} {file['name']:<2}")

    if len(arguments["args"]) > 1:
        if mode == MODE["words"]:
            print(f"{words:<3} total")
        elif mode == MODE["chars"]:
            print(f"{chars:<3} total")
        elif mode == MODE["lines"]:
            print(f"{lines:<3} total")
        else:
            print(f"{lines:<3} {words:<2} {chars:<2} total")


if __name__ == "__main__":
    main()

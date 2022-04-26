class Parser:
    def __init__(self, args: list):
        args.pop(0)
        self.args = args

    def get_args(self) -> str:
        flags = []
        args = []
        if not self.args:
            return {
                "args": args,
                "flags": flags
            }
        for item in self.args:
            if item.startswith("--") or item.startswith("-"):
                flags.append(item.replace("-", ""))
            else:
                args.append(item)
        return {
            "args": args,
            "flags": flags
        }

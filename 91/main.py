import inspect
import json
from functools import lru_cache
from pathlib import Path


class Solution:
    @lru_cache(maxsize=1_000_000)
    def numDecodings(self, s: str) -> int:
        if s == "":
            return 1
        if s[0] == "0":
            return 0
        if len(s) == 1:
            return 1
        if int(s[:2]) <= 26:
            return self.numDecodings(s[1:]) + self.numDecodings(s[2:])
        return self.numDecodings(s[1:])


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = list(filter(
            lambda x: not x[0].startswith("_"),
            inspect.getmembers(Solution(), predicate=inspect.ismethod)
        ))[0]
        out = fn(**input_)
        assert out == res, (out, res)


if __name__ == "__main__":
    main()


import inspect
import json
from functools import lru_cache
from pathlib import Path


class Solution:
    @lru_cache(maxsize=1_000)
    def _is_palindrome(self, s: str) -> bool:
        if len(s) <= 1:
            return True
        return s[0] == s[-1] and self._is_palindrome(s[1:][:-1])

    def partition(self, s: str) -> list[list[str]]:
        if len(s) == 0:
            return [[]]

        res = []
        for idx in range(1, len(s)+1):
            pre, post = s[:idx], s[idx:]
            if self._is_palindrome(pre):
                res += [
                    [pre] + part
                    for part in self.partition(s[idx:])
                ]

        return res


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


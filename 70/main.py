import inspect
import json
from functools import lru_cache
from pathlib import Path


class Solution:
    @lru_cache(maxsize=1_000_000)
    def climbStairs(self, n: int) -> int:
        if n == 0 or n == 1:
            return 1
        return self.climbStairs(n-1) + self.climbStairs(n-2)


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
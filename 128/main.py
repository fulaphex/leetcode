import inspect
import json
from collections import defaultdict
from pathlib import Path


class Solution:
    repr_: dict[int, int] = {}

    def _find(self, x: int) -> int:
        if x not in self.repr_:
            self.repr_[x] = x
        if self.repr_[x] != x:
            self.repr_[x] = self._find(self.repr_[x])
        return self.repr_[x]

    def _union(self, x, y):
        y_rep = self._find(y)
        self.repr_[y] = self.repr_[y_rep] = self._find(x)

    def longestConsecutive(self, nums: list[int]) -> int:
        self.repr_: dict[int, int] = {}

        for num in nums:
            if num + 1 in self.repr_:
                self._union(num+1, num)

            if num - 1 in self.repr_:
                self._union(num-1, num)

            self._find(num)

        res = defaultdict(set)
        for num in nums:
            res[self._find(num)].add(num)
        return max(map(len, res.values()))



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


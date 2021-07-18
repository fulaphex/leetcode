import inspect
import json
from collections import Counter, defaultdict
from pathlib import Path


class Solution:
    def repr_(self, str_) -> tuple[tuple[int, int], ...]:
        cnt = Counter(str_)
        return tuple(sorted(cnt.items()))

    def groupAnagrams(self, strs: list[str]) -> list[list[str]]:
        dd = defaultdict(list)
        for str_ in strs:
            dd[self.repr_(str_)].append(str_)
        return list(dd.values())


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = inspect.getmembers(Solution(), predicate=inspect.ismethod)[0]
        out = fn(**input_)
        assert sorted(out) == sorted(res), (out, res)


if __name__ == "__main__":
    main()


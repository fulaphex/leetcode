import inspect
import json
from pathlib import Path


class Solution:
    def singleNumber(self, nums: list[int]) -> int:
        s = set()
        for n in nums:
            if n not in s:
                s.add(n)
            else:
                s.remove(n)
        return list(s)[0]

        

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


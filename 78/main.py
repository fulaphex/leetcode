import inspect
import json
from pathlib import Path


class Solution:
    def subsets(self, nums: list[int]) -> list[list[int]]:
        if len(nums) == 0:
            return [[]]
        subs = self.subsets(nums[1:])
        return [
            [nums[0]] + ss for ss in subs
        ] + subs


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
        assert sorted(out) == sorted(res), (out, res)


if __name__ == "__main__":
    main()
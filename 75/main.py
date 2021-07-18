import inspect
import json
from collections import Counter
from pathlib import Path


class Solution:
    def sortColors(self, nums: list[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        counter = Counter(nums)
        el = 0
        for idx in range(len(nums)):
            while not counter.get(el):
                el += 1
            nums[idx] = el
            counter[el] -= 1


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
        assert list(input_.values())[0] == res, (out, res)


if __name__ == "__main__":
    main()
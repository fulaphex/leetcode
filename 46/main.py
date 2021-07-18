import inspect
import json
from pathlib import Path


class Solution:
    def permute(self, nums: list[int]) -> list[list[int]]:
        if len(nums) == 0:
            return [[]]
        return [
            [el] + suff
            for idx, el in enumerate(nums)
            for suff in self.permute(nums[:idx] + nums[idx+1:])
        ]


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = inspect.getmembers(Solution(), predicate=inspect.ismethod)[0]
        out = fn(**input_)
        assert out == res, (out, res)


if __name__ == "__main__":
    main()

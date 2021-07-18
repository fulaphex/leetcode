import inspect
import json
from pathlib import Path


class Solution:
    def permuteUnique(self, nums: list[int]) -> list[list[int]]:
        if len(nums) == 0:
            return [[]]
        perms = {
            tuple([el] + suff)
            for idx, el in enumerate(nums)
            for suff in self.permuteUnique(nums[:idx] + nums[idx+1:])
        }
        return [list(perm) for perm in perms]


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

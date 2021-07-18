import inspect
import json
from pathlib import Path


class Solution:
    def canJump(self, nums: list[int]) -> bool:
        curr_end, furthest = 0, 0
        for idx, jump in enumerate(nums):
            if curr_end == len(nums)-1:
                return True
            furthest = max(furthest, idx + jump)
            if idx == curr_end:
                if furthest == curr_end:
                    return False
                curr_end = furthest

        return True


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


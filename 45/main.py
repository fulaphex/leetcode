import json
from pathlib import Path


class Solution:
    def jump(self, nums: list[int]) -> int:
        max_dist, curr_jumps, curr_end = 0, 0, 0
        for idx, jump in enumerate(nums[:-1]):
            max_dist = max(max_dist, idx+jump)
            if idx == curr_end:
                # I need to make a jump here
                curr_jumps += 1
                curr_end = max_dist

        return curr_jumps


def main():
    files = Path(".").glob("*.json")
    for fname in files:
        input_ = json.load(open(fname))
        nums = input_["nums"]
        print(nums)
        # nums = nums*10000
        out = Solution().jump(nums)
        res = input_["out"]
        print(out)
        assert out == res, (out, res)


if __name__ == "__main__":
    main()

import inspect
import json
from pathlib import Path
import random


class Solution:
    def findKthLargest(self, nums: list[int], k: int) -> int:
        return self.find_kth_largest(nums, k-1)

    def find_kth_largest(self, nums: list[int], k: int) -> int:
        if len(nums) == 1:
            assert k == 0
            return nums[0]
        r_idx = random.randint(0, len(nums)-1)
        pivot = nums[r_idx]
        leq = 0
        for idx, el in enumerate(nums):
            if idx == r_idx:
                continue
            if el >= pivot:
                leq += 1
        pivot_idx = leq
        
        # put pivot into the correct location
        if r_idx != pivot_idx:
            nums[r_idx], nums[pivot_idx] = nums[pivot_idx], nums[r_idx]
        
        x, y = 0, len(nums)-1
        while x < pivot_idx < y:
            if nums[x] >= pivot:
                x += 1
            elif nums[y] < pivot:
                y -= 1
            else:  # nums[x] < pivot <= nums[y]
                nums[x], nums[y] = nums[y], nums[x]

        if k < pivot_idx:
            return self.find_kth_largest(nums[:pivot_idx], k)

        return self.find_kth_largest(nums[pivot_idx:], k-pivot_idx)


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


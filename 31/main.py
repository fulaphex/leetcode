import json
from pathlib import Path


class Solution:
    def nextPermutation(self, nums: list[int]) -> None:
        """
        Do not return anything, modify nums in-place instead.
        """
        if len(nums) == 0:
            return
        # if all items equal do nothing
        el = nums[0]
        for num in nums:
            if num != el:
                break
        else:
            return
        # if non-ascending then flip
        for idx in range(len(nums)-1):
            if nums[idx] < nums[idx+1]:
                break
        else:
            nums.reverse()
            return

        # find longest decreasing suffix, swap last element with the element before the suffix and
        # sort the suffix. that suffix won't be the entire sequence, because we did that case above
        for idx in range(len(nums)-1, 0, -1):
            if nums[idx-1] < nums[idx]:
                break
        else:
            raise ValueError("suffix not found")

        prev = idx - 1
        limit = idx
        # find the smallest element in the suffix bigger than nums[idx-1]
        for idx in range(len(nums)-1, 0, -1):
            if nums[idx] > nums[prev]:
                # swapping the elements
                nums[prev], nums[idx] = nums[idx], nums[prev]
                break
        else:
            raise ValueError("smallest bigger than nums[prev] not found")

        # swapped_idx = idx
        # swapped element in place
        while prev < idx < len(nums)-1:
            if nums[idx-1] < nums[idx]:
                nums[idx-1], nums[idx] = nums[idx], nums[idx-1]
                idx -= 1
            elif nums[idx] < nums[idx+1]:
                nums[idx], nums[idx+1] = nums[idx+1], nums[idx]
                idx += 1
            else:
                break

        # need to reverse the suffix
        x, y = limit, len(nums) - 1
        while x < y:
            nums[x], nums[y] = nums[y], nums[x]
            x += 1
            y -= 1


def main():
    files = Path(".").glob("*.json")
    for fname in files:
        # print(fname)
        input_ = json.load(open(fname))
        nums = input_["nums"]
        Solution().nextPermutation(nums)
        res = input_["res"]
        try:
            assert nums == res, (nums, res)
        except:
            print("failed:", nums, res)
            pass


if __name__ == "__main__":
    main()


# def all_permutations(elems: list):
#     if len(elems) == 0:
#         return [[]]
#     perms = []
#     for el in elems:
#         for subperm in all_permutations(list(set(elems) - {el})):
#             perms.append([el] + subperm)
#     return perms
#
#
# perms = ["".join(p) for p in all_permutations(['1', '2', '3', '4', '5'])]
# perms = sorted(perms)
# pprint(perms)
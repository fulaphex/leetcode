import json
from pathlib import Path


class Solution:
    __nums = list[int]
    def fixup_position(self, idx):
        while self.__nums[idx] != idx:
            # print(self.__nums, idx)
            if self.__nums[idx] >= len(self.__nums) or self.__nums[idx] < 0:
                # number bigger than size of array or negative, cannot be placed in it's position
                break
            else:
                # place number in its position
                x = self.__nums[idx]
                y = self.__nums[x]
                if x == y:
                    break
                # swapping numbers
                # putting x in it's place and putting y in idx
                self.__nums[x], self.__nums[idx] = x, y

    def firstMissingPositive(self, nums: list[int]) -> int:
        self.__nums = [n-1 for n in nums]
        # print(nums)
        for idx, num in enumerate(self.__nums):
            # if number is not in place and can be put in place
            if num != idx and num < len(self.__nums):
                # we have to swap it to its place
                self.fixup_position(idx)
                # assert False

        for idx, num in enumerate(self.__nums):
            if num != idx:
                return idx + 1
                pass

        return len(self.__nums) + 1


def main():
    files = Path(".").glob("*.json")
    for fname in files:
        input_ = json.load(open(fname))
        arr = input_["arr"]
        # print(arr)
        out = Solution().firstMissingPositive(arr)
        res = input_["res"]
        assert out == res, (out, res)


if __name__ == "__main__":
    main()

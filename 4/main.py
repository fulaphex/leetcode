import inspect
import json
from pathlib import Path
import random


class Solution:
    def binsrch(self, nums1: list[int], nums2: list[int], a, b) -> float:
        print(a, b)
        l1, l2 = len(nums1), len(nums2)

        mid = (a+b)//2 # how many elements from nums1 to take
        print(f"{mid = }")
        idx = (l1+l2) // 2 - (mid)
        x1, y1 = nums1[mid - 1], nums1[mid]
        x2, y2 = nums2[idx - 1], nums2[idx]
        print(f"{x1 = }, {y1 = }")
        print(f"{x2 = }, {y2 = }")
        if x1 <= y2 and x2 <= y1:
            if (l1+l2) % 2:
                return min(y1, y2)
            return (max(x1, x2) + min(y1, y2)) / 2
        if y2 < x1:
            return self.binsrch(nums1, nums2, a, mid)
        return self.binsrch(nums1, nums2, mid, b)

    def findMedianSortedArrays(self, nums1: list[int], nums2: list[int]) -> float:
        l1, l2 = len(nums1), len(nums2)
        if l1 > l2:
            print("changing argument order")
            return self.findMedianSortedArrays(nums2, nums1)

        if l1 == 0:
            return (nums2[(l2-1)//2] + nums2[l2 // 2]) / 2

        if nums2[-1] <= nums1[0]:
            if l1 == l2:
                return (nums2[-1] + nums1[0]) / 2
            idx, idx2 = (l1 + l2 - 1) // 2, (l1 + l2) // 2
            return nums2[idx2]

        if nums1[-1] <= nums2[0]:
            if l1 == l2:
                return (nums1[-1] + nums2[0]) / 2
            idx, idx2 = (l1 + l2 - 1) // 2, (l1 + l2) // 2
            return nums2[idx2-l1]

        # check before first and after last
        # before first
        idx = (l1+l2) // 2
        if nums2[idx-1] <= nums1[0]:
            print("before first")
            print(idx-1, nums2[idx-1])
            if (l1+l2) % 2:
                print("odd")
                return min(nums1[0], nums2[idx])
            print("even")
            return (nums2[idx-1] + min(nums1[0], nums2[idx])) / 2
        # after last (still TODO)
        idx = (l1+l2) // 2 - l1
        if nums1[-1] <= nums2[idx]:
            print("after last")
            print(idx-1, nums2[idx-1])
            print(idx, nums2[idx])
            if (l1+l2) % 2:
                print("odd")
                return nums2[idx]
            print("even")
            return (nums2[idx] + max(nums1[-1], nums2[idx-1])) / 2

        return self.binsrch(nums1, nums2, 0, l1)


def split_arr(arr: list[int]) -> tuple[list[int], list[int]]:
    random.shuffle(arr)
    split_idx = len(arr) // 2
    split_idx = random.randint(0, len(arr)-1)
    return sorted(arr[:split_idx]), sorted(arr[split_idx:])


for _ in range(100):
    l = random.randint(1, 50)
    # l = 20
    arr = list(range(l))
    # arr = list(range(20))
    x, y = split_arr(arr)
    # x, y = [11, 17], [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 18, 19]
    # x, y = [1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 17, 18, 19], [0, 3, 16]
    # x, y = [0, 9], [1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]
    # x, y = [0, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 14, 15, 16, 17, 18, 19], [2, 12]
    # x, y = [2], [0, 1, 3, 4, 5]
    # x, y = [2, 3], [0, 1]
    # x, y = [1], [0, 2]
    x, y = [1, 2], [1, 1]
    print(f"{x}, {y}")
    out = Solution().findMedianSortedArrays(x, y)
    print(out)
    assert out == (len(x) + len(y) - 1)/2
    break

# def main():
#     files = Path(".").glob("*.json")
#
#     for fname in files:
#         input_ = json.load(open(fname))
#         res = input_.pop("out")
#
#         _, fn = inspect.getmembers(Solution(), predicate=inspect.ismethod)[0]
#         out = fn(**input_)
#         assert out == res, (out, res)
#
#
# if __name__ == "__main__":
#     main()

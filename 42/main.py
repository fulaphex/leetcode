import json
from pathlib import Path


class Solution:
    __pref_sum: list[int] = []

    def trap_partial(self, heights: list[int]) -> int:
        pref_sum: list[int] = [0] * (len(heights)+1)
        # pref_sum[idx] = sum(h[0], ..., h[idx-1])
        for idx, h in enumerate(heights):
            pref_sum[idx+1] = pref_sum[idx] + h

        res = 0

        max_idx, max_h = 0, 0
        for idx, h in enumerate(heights):
            if h >= max_h:
                # higher or equal than previous max
                if idx == max_idx + 1:
                    # consecutive elements, update max and continue
                    max_h, max_idx = h, idx
                    continue
                else:
                    # there is a gap that we need to consider
                    # water_level = max_h
                    # gap_length = idx - (max_idx+1)
                    area = max_h * (idx - max_idx - 1)
                    hills = (pref_sum[idx] - pref_sum[max_idx+1])
                    res += max(area - hills, 0)
                    max_h, max_idx = h, idx

        return res

    def trap(self, height: list[int]) -> int:
        heights = [0] + height + [0]
        max_h, max_idx = 0, 0
        for idx, h in enumerate(heights):
            if h > max_h:
                max_h, max_idx = h, idx
        first_half = heights[:max_idx+1]
        second_half = heights[max_idx:][::-1]
        res = 0
        res += self.trap_partial(first_half)
        res += self.trap_partial(second_half)
        return res


def main():
    files = Path(".").glob("*.json")
    for fname in files:
        # print(fname)
        input_ = json.load(open(fname))
        height = input_["height"]
        out = Solution().trap(height)
        res = input_["res"]
        try:
            assert out == res, (out, res)
        except:
            print("failed:", out, res)
            pass


if __name__ == "__main__":
    main()

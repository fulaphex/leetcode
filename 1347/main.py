class Solution:
    def minSteps(self, s: str, t: str) -> int:
        cnt1, cnt2 = Counter(s), Counter(t)
        res = 0
        for k in cnt1:
            if (diff := cnt1[k] - cnt2.get(k, 0)) > 0:
                res += diff
        return res

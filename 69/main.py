import inspect
import json
from pathlib import Path


class Solution:
    def _binsrch(self, a, b, target):
        # print(a, b, target)
        if a+1 == b:
            return a
        mid = (a+b)//2
        # print(mid)
        if mid*mid <= target:
            return self._binsrch(mid, b, target)
        return self._binsrch(a, mid, target)

    def mySqrt(self, x: int) -> int:
        return self._binsrch(0, x+1, x)


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

sol = Solution()
for i in range(1000):
    out = sol.mySqrt(i)
    assert out*out <= i < (out+1)*(out+1)
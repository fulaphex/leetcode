import inspect
import json
from pathlib import Path


import inspect
import json
from pathlib import Path


class Solution:
    def merge(self, intervals: list[list[int]]) -> list[list[int]]:
        intervals = sorted(intervals)
        if len(intervals) == 0:
            return []
        res = []
        prev_x, prev_y = intervals[0]
        for x, y in intervals[1:]:
            if x <= prev_y:
                prev_y = max(prev_y, y)
            else:
                res.append([prev_x, prev_y])
                prev_x, prev_y = x, y
        res.append([prev_x, prev_y])
        return res


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


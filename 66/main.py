import inspect
import json
from pathlib import Path


class Solution:
    def plusOne(self, digits: list[int]) -> list[int]:
        digits[-1] += 1
        for idx in range(len(digits)-1, 0, -1):
            if digits[idx] < 10:
                break
            digits[idx-1] += 1
            digits[idx] %= 10
        if digits[0] >= 10:
            digits[0] %= 10
            return [1] + digits
        return digits


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


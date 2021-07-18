import inspect
import json
from pathlib import Path
from pprint import pprint


class Solution:
    def rotate_points(self, x: int, y: int, n: int) -> tuple[int, int]:
        return y, n - 1 - x

    def rotate(self, matrix: list[list[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        n = len(matrix)
        for i in range((n+1)//2):
            for j in range(n//2):
                x1, y1 = i, j
                x2, y2 = self.rotate_points(x1, y1, n)
                x3, y3 = self.rotate_points(x2, y2, n)
                x4, y4 = self.rotate_points(x3, y3, n)

                matrix[x2][y2], matrix[x3][y3], matrix[x4][y4], matrix[x1][y1] = (
                    matrix[x1][y1], matrix[x2][y2], matrix[x3][y3], matrix[x4][y4]
                )


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = inspect.getmembers(Solution(), predicate=inspect.ismethod)[0]
        out = fn(**input_)
        assert input_["matrix"] == res, (input_["matrix"], res)


if __name__ == "__main__":
    main()


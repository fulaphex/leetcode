import inspect
import json
from functools import lru_cache
from pathlib import Path


class Solution:
    def setZeroes(self, matrix: list[list[int]]) -> None:
        """
        Do not return anything, modify matrix in-place instead.
        """
        col_ids, row_ids = set(), set()
        for row_idx, row in enumerate(matrix):
            for col_idx, el in enumerate(row):
                if el == 0:
                    col_ids.add(col_idx)
                    row_ids.add(row_idx)

        for row_idx, row in enumerate(matrix):
            for col_idx, el in enumerate(row):
                if row_idx in row_ids or col_idx in col_ids:
                    matrix[row_idx][col_idx] = 0


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
        assert list(input_.values())[0] == res, (out, res)


if __name__ == "__main__":
    main()
import inspect
import json
from pathlib import Path
from pprint import pprint
from time import sleep

DIRECTIONS = [(0, 1), (1, 0), (0, -1), (-1, 0)]


class Solution:
    def spiralOrder(self, matrix: list[list[int]]) -> list[int]:
        n = len(matrix)
        if n == 0:
            return []
        m = len(matrix[0])
        x, y = 0, 0
        vis: set[tuple[int, int]] = set()
        vis.add((x,y))
        res = [matrix[x][y]]
        dx, dy = 0, 1
        next_dir = {
            DIRECTIONS[idx]: DIRECTIONS[(idx+1) % 4] for idx in range(4)
        }

        while True:
            if 0 <= x+dx < n and 0 <= y+dy < m and (x+dx, y+dy) not in vis:
                x, y = x+dx, y+dy
                vis.add((x, y))
                res.append(matrix[x][y])
            else:
                dx, dy = next_dir[(dx, dy)]
                if 0 <= x+dx < n and 0 <= y+dy < m and (x+dx, y+dy) not in vis:
                    x, y = x+dx, y+dy
                    vis.add((x, y))
                    res.append(matrix[x][y])
                else:
                    break

        return res


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = inspect.getmembers(Solution(), predicate=inspect.ismethod)[0]
        out = fn(**input_)
        assert out == res, (out, res)
        # print("------")


if __name__ == "__main__":
    main()


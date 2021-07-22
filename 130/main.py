import inspect
import json
from pathlib import Path
from pprint import pprint

DIRECTIONS: list[tuple[int, int]] = [
    (1, 0), (-1, 0), (0, 1), (0, -1)
]
class Solution:
    _vis = []

    def _dfs(self, board: list[list[str]], x: int, y: int) -> tuple[bool, list[tuple[int, int]]]:
        self._vis[x][y] = True
        res = False
        if x == 0 or x == (len(board)-1) or y == 0 or y == (len(board[0])-1):
            res = True

        pts = [(x, y)]
        for dx, dy in DIRECTIONS:
            nx, ny = x+dx, y+dy
            if (
                    0 <= nx < len(board) and
                    0 <= ny < len(board[0]) and
                    board[nx][ny] == "O" and
                    not self._vis[nx][ny]
            ):
                new_res, new_pts = self._dfs(board, nx, ny)
                res = res or new_res
                pts.extend(new_pts)

        return res, pts

    def solve(self, board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        if len(board) == 0:
            return
        self._vis = [
            [False]*len(row)
            for row in board
        ]
        for x in range(len(board)):
            for y in range(len(board[0])):
                if not self._vis[x][y] and board[x][y] == "O":
                    border, pts = self._dfs(board, x, y)
                    if not border:
                        for nx, ny in pts:
                            board[nx][ny] = "X"


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
        assert list(input_.values())[0] == res, (list(input_.values())[0], res)


if __name__ == "__main__":
    main()


import inspect
import json
from pathlib import Path

Point = tuple[int, int]
DIRECTIONS: list[Point] = [(1, 0), (0, 1), (-1, 0), (0, -1)]


class Solution:
    def _dfs(self, board: list[list[str]], word: str, x: int, y: int, path: set[Point]) -> bool:
        # print(x, y)
        # print(word)
        # time.sleep(0.25)

        if word == "":
            return True

        for dx, dy in DIRECTIONS:
            nx, ny = x+dx, y+dy

            if not (0 <= nx < len(board)):
                continue
            if not (0 <= ny < len(board[0])):
                continue

            if board[nx][ny] == word[0] and (nx, ny) not in path:
                path.add((nx, ny))
                if self._dfs(board, word[1:], nx, ny, path):
                    return True
                path.remove((nx, ny))

        return False

    def _exist_xy(self, board: list[list[str]], word: str, x: int, y: int) -> bool:
        if board[x][y] != word[0]:
            return False
        path: set[Point] = {(x, y)}
        return self._dfs(board, word[1:], x, y, path)

    def exist(self, board: list[list[str]], word: str) -> bool:
        # pprint(board)
        # print(word)
        board_let_set = {l for row in board for l in row}
        for l in word:
            if l not in board_let_set:
                return False
        if word == "":
            return True
        lx = len(board)
        if lx == 0:
            return False
        ly = len(board[0])

        for x in range(lx):
            for y in range(ly):
                # print(x, y)
                if self._exist_xy(board, word, x, y):
                    return True
                # print("====")

        return False


def main():
    files = Path(".").glob("*.json")

    for fname in files:
        # if fname.name == "4.json":
        #     continue
        input_ = json.load(open(fname))
        res = input_.pop("out")

        _, fn = list(filter(
            lambda x: not x[0].startswith("_"),
            inspect.getmembers(Solution(), predicate=inspect.ismethod)
        ))[0]
        out = fn(**input_)
        assert out == res, (out, res)
        print("--------")


if __name__ == "__main__":
    main()


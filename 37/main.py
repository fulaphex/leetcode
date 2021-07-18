import json
from pprint import pformat, pprint
from typing import Optional

IncompleteCell = Optional[int]
IncompleteRow = list[IncompleteCell]
IncompleteBoard = list[IncompleteRow]
Move = tuple[int, int, int]  # x, y, value


class Board:
    __cells: IncompleteBoard
    __moves: list[Move]

    def __init__(self, raw_cells: list[list[str]]) -> None:
        self.__cells = [
            [int(cell) if cell != "." else None for cell in row]
            for row in raw_cells
        ]
        self.__moves = []

    def get_col(self, idx: int) -> IncompleteRow:
        assert 0 <= idx < 9
        return [row[idx] for row in self.__cells]

    def get_row(self, idx: int) -> IncompleteRow:
        assert 0 <= idx < 9
        return self.__cells[idx]

    def get_sq(self, x: int, y: int) -> IncompleteRow:
        assert 0 <= x < 9
        assert 0 <= y < 9
        x, y = x - x % 3, y - y % 3
        return [self.__cells[x + dx][y + dy] for dx in range(3) for dy in range(3)]

    def validate(self, row: IncompleteRow) -> bool:
        els = set()
        for x in row:
            if x and x in els:
                return False
            els.add(x)
        return True

    def next_field(self, x: int, y: int) -> tuple[int, int]:
        return (x, y + 1) if y != 8 else (x + 1, 0)

    def solve(self, x, y) -> bool:
        # finished filling up all fields
        if x == 9:
            return True

        # finding first empty field
        while self.__cells[x][y] is not None:
            x, y = self.next_field(x, y)
            if x == 9:
                return True
            continue

        for field_val in range(1, 10):
            self.__cells[x][y] = field_val

            if (
                    self.validate(self.get_row(x)) and
                    self.validate(self.get_col(y)) and
                    self.validate(self.get_sq(x, y))
            ):
                self.__moves.append((x, y, field_val))
                res = self.solve(*self.next_field(x, y))
                if res:
                    return True
                # print("backtracking")
                self.__cells[x][y] = None
                self.__moves.pop(-1)
            else:
                self.__cells[x][y] = None
        return False

    def get_cells(self) -> IncompleteBoard:
        return self.__cells

    def get_str_cells(self) -> list[list[str]]:
        return [
            [str(c) for c in row]
            for row in self.__cells
        ]

    def __str__(self) -> str:
        return pformat(self.__cells)


class Solution:
    def solveSudoku(self, og_board: list[list[str]]) -> None:
        """
        Do not return anything, modify board in-place instead.
        """
        board = Board(og_board)
        board.solve(0, 0)
        str_cells = board.get_str_cells()
        for x in range(9):
            for y in range(9):
                og_board[x][y] = str_cells[x][y]


def main() -> None:
    input_ = json.load(open("1.json"))
    og_board = input_["board"]
    Solution().solveSudoku(og_board)

    assert og_board == input_["solution"]


if __name__ == "__main__":
    main()
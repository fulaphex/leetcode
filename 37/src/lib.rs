const N: usize = 9;

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        fn get_box_idx(x: usize, y: usize) -> usize {
            return 3 * (x / 3) + (y / 3);
        }

        fn inner(
            board: &mut Vec<Vec<char>>,
            empties: &[(usize, usize)],
            row_vals: &mut Vec<Vec<bool>>,
            col_vals: &mut Vec<Vec<bool>>,
            box_vals: &mut Vec<Vec<bool>>,
        ) -> bool {
            if empties.len() == 0 {
                return true;
            }
            let (row, col) = empties[0];
            let boxx = get_box_idx(row, col);

            for (idx, f) in ('1'..='9').enumerate() {
                if row_vals[row][idx] || col_vals[col][idx] || box_vals[boxx][idx] {
                    continue;
                }
                board[row][col] = f;
                row_vals[row][idx] = true;
                col_vals[col][idx] = true;
                box_vals[boxx][idx] = true;

                if inner(board, &empties[1..], row_vals, col_vals, box_vals) {
                    return true;
                }

                row_vals[row][idx] = false;
                col_vals[col][idx] = false;
                box_vals[boxx][idx] = false;
            }
            board[row][col] = '.';
            return false;
        }

        let ub1 = b'1' as usize;
        let (mut row_vals, mut col_vals, mut box_vals) = (
            vec![vec![false; 9]; 9],
            vec![vec![false; 9]; 9],
            vec![vec![false; 9]; 9],
        );
        let mut empties = vec![];

        // initialise the row/col/box_vals
        for (x, row) in board.iter().enumerate() {
            for (y, &f) in row.iter().enumerate() {
                if f == '.' {
                    empties.push((x, y));
                    continue;
                }
                let fv = f as usize - ub1;
                let boxx = get_box_idx(x, y);
                row_vals[x][fv] = true;
                col_vals[y][fv] = true;
                box_vals[boxx][fv] = true;
            }
        }

        inner(
            board,
            empties.as_slice(),
            &mut row_vals,
            &mut col_vals,
            &mut box_vals,
        );
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut board = [
            [".", "3", ".", ".", "7", ".", ".", ".", "."],
            ["6", ".", ".", "1", "9", "5", ".", ".", "."],
            [".", "9", "8", ".", ".", ".", ".", "6", "."],
            ["8", ".", ".", ".", "6", ".", ".", ".", "3"],
            ["4", ".", ".", "8", ".", "3", ".", ".", "1"],
            ["7", ".", ".", ".", "2", ".", ".", ".", "6"],
            [".", "6", ".", ".", ".", ".", "2", "8", "."],
            [".", ".", ".", "4", "1", "9", ".", ".", "5"],
            [".", ".", ".", ".", "8", ".", ".", "7", "9"],
        ]
        .into_iter()
        .map(|x| Vec::from_iter(x.into_iter().map(|y| y.chars().next().unwrap())))
        .collect();
        let res: Vec<Vec<char>> = [
            ["5", "3", "4", "6", "7", "8", "9", "1", "2"],
            ["6", "7", "2", "1", "9", "5", "3", "4", "8"],
            ["1", "9", "8", "3", "4", "2", "5", "6", "7"],
            ["8", "5", "9", "7", "6", "1", "4", "2", "3"],
            ["4", "2", "6", "8", "5", "3", "7", "9", "1"],
            ["7", "1", "3", "9", "2", "4", "8", "5", "6"],
            ["9", "6", "1", "5", "3", "7", "2", "8", "4"],
            ["2", "8", "7", "4", "1", "9", "6", "3", "5"],
            ["3", "4", "5", "2", "8", "6", "1", "7", "9"],
        ]
        .into_iter()
        .map(|x| Vec::from_iter(x.into_iter().map(|y| y.chars().next().unwrap())))
        .collect();
        Solution::solve_sudoku(&mut board);
        assert_eq!(board, res);
    }
}

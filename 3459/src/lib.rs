impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        let grid_rot = Self::rot(&grid);
        let grid_rot2 = Self::rot(&grid_rot);
        let grid_rot3 = Self::rot(&grid_rot2);
        return Self::sub_solve(&grid)
            .max(Self::sub_solve(&grid_rot))
            .max(Self::sub_solve(&grid_rot2))
            .max(Self::sub_solve(&grid_rot3));
    }
    fn rot(grid: &Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (row_count, col_count) = (grid.len(), grid[0].len());
        let mut res = vec![vec![-1; row_count]; col_count];

        for ridx in 0..row_count {
            for cidx in 0..col_count {
                res[cidx][row_count - 1 - ridx] = grid[ridx][cidx];
            }
        }

        return res;
    }
    fn sub_solve(grid: &Vec<Vec<i32>>) -> i32 {
        let (row_count, col_count) = (grid.len(), grid[0].len());

        let mut up_right = vec![vec![-1; col_count]; row_count];
        let mut up_left = vec![vec![-1; col_count]; row_count];

        let mut res = 0;

        for (col_idx, &field) in grid[0].iter().enumerate() {
            if field != 1 {
                up_right[0][col_idx] = 0;
                up_left[0][col_idx] = 0;
            } else {
                res = res.max(1);
            }
        }

        for (row_idx, row) in grid.iter().enumerate().skip(1) {
            for (col_idx, &field) in row.iter().enumerate() {
                if field == 1 {
                    res = res.max(1);
                    continue;
                }
                if col_idx > 0 {
                    let prev = grid[row_idx - 1][col_idx - 1];
                    if (prev + field) == 2 {
                        up_left[row_idx][col_idx] = up_left[row_idx - 1][col_idx - 1] + 1;
                    } else {
                        up_left[row_idx][col_idx] = 0;
                    }
                } else {
                    up_left[row_idx][col_idx] = 0;
                }
                if col_idx < col_count - 1 {
                    let prev = grid[row_idx - 1][col_idx + 1];
                    if (prev + field) == 2 {
                        up_right[row_idx][col_idx] = up_right[row_idx - 1][col_idx + 1] + 1;
                    } else {
                        up_right[row_idx][col_idx] = 0;
                    }
                } else {
                    up_right[row_idx][col_idx] = 0;
                }
            }
        }

        for (row_idx, row) in grid.iter().enumerate().skip(1) {
            for (col_idx, &field) in row.iter().enumerate() {
                if field == 1 {
                    if (col_idx > 0) && (grid[row_idx - 1][col_idx - 1] == 2) {
                        let curr = up_left[row_idx - 1][col_idx - 1];
                        for up in 0..=(curr as usize) {
                            res = res.max(
                                1 + // for the first 1
                                1 + (up as i32) + // 1 + number of tiles to go up_left
                                up_right[row_idx - 1 - up][col_idx - 1 - up], // number of tiles to go up_right from turn
                            )
                        }
                    }
                }
            }
        }

        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [
            [2, 2, 1, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 5;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [
            [2, 2, 2, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 4;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test3() {
        let grid = [
            [1, 2, 2, 2, 2],
            [2, 2, 2, 2, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 2, 2, 2],
            [2, 0, 0, 2, 0],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 5;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test4() {
        let grid = [[1]].iter().map(|x| Vec::from(x)).collect();
        let res = 1;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test5() {
        let grid = [[1]].iter().map(|x| Vec::from(x)).collect();
        let res = 1;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test6() {
        let grid = [[2, 2, 0, 2, 0, 2, 0], [1, 2, 2, 1, 0, 2, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test7() {
        let grid = [[0, 0, 1, 0], [0, 2, 2, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 3;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test8() {
        let grid = [[2, 2, 2, 2, 2, 2, 1, 0, 2], [0, 1, 2, 2, 0, 1, 0, 0, 0]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }

    #[test]
    fn test9() {
        let grid = [
            [0, 2, 0, 2, 0, 2, 2, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2],
            [0, 1, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2],
            [0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2],
            [0, 2, 0, 2, 0, 2, 0, 2, 0, 0, 0, 2, 0, 2, 0, 2, 0, 2],
            [0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2, 0, 2],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 1;
        assert_eq!(Solution::len_of_v_diagonal(grid), res);
    }
}

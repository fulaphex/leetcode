use std::collections::HashSet;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (m as usize, n as usize);
        let mut guarded = vec![vec![false; cols]; rows];
        let (mut guard_grid, mut wall_grid) =
            (vec![vec![false; cols]; rows], vec![vec![false; cols]; rows]);

        for g in guards {
            let (x, y) = (g[0] as usize, g[1] as usize);
            guard_grid[x][y] = true;
        }

        for w in walls {
            let (x, y) = (w[0] as usize, w[1] as usize);
            wall_grid[x][y] = true;
        }

        for row in 0..rows {
            let mut curr_guarded = false;
            for col in 0..cols {
                guarded[row][col] |= wall_grid[row][col] || guard_grid[row][col];
                if wall_grid[row][col] {
                    curr_guarded = false;
                } else if guard_grid[row][col] {
                    curr_guarded = true;
                } else {
                    guarded[row][col] |= curr_guarded;
                }
            }

            let mut curr_guarded = false;
            for col in (0..cols).rev() {
                guarded[row][col] |= wall_grid[row][col] || guard_grid[row][col];
                if wall_grid[row][col] {
                    curr_guarded = false;
                } else if guard_grid[row][col] {
                    curr_guarded = true;
                } else {
                    guarded[row][col] |= curr_guarded;
                }
            }
        }

        for col in 0..cols {
            let mut curr_guarded = false;
            for row in 0..rows {
                guarded[row][col] |= wall_grid[row][col] || guard_grid[row][col];
                if wall_grid[row][col] {
                    curr_guarded = false;
                } else if guard_grid[row][col] {
                    curr_guarded = true;
                } else {
                    guarded[row][col] |= curr_guarded;
                }
            }

            let mut curr_guarded = false;
            for row in (0..rows).rev() {
                guarded[row][col] |= wall_grid[row][col] || guard_grid[row][col];
                if wall_grid[row][col] {
                    curr_guarded = false;
                } else if guard_grid[row][col] {
                    curr_guarded = true;
                } else {
                    guarded[row][col] |= curr_guarded;
                }
            }
        }

        let mut res = 0;

        for row in guarded {
            for g in row {
                if !g {
                    res += 1;
                }
            }
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (m, n) = (4, 6);
        let guards: Vec<_> = [[0, 0], [1, 1], [2, 3]]
            .into_iter()
            .map(Vec::from)
            .collect();
        let walls: Vec<_> = [[0, 1], [2, 2], [1, 4]]
            .into_iter()
            .map(Vec::from)
            .collect();
        let res = 7;

        assert_eq!(Solution::count_unguarded(m, n, guards, walls), res);
    }

    #[test]
    fn test2() {
        let (m, n) = (3, 3);
        let guards: Vec<_> = [[1, 1]].into_iter().map(Vec::from).collect();
        let walls: Vec<_> = [[0, 1], [1, 0], [2, 1], [1, 2]]
            .into_iter()
            .map(Vec::from)
            .collect();

        let res = 4;

        assert_eq!(Solution::count_unguarded(m, n, guards, walls), res);
    }
}

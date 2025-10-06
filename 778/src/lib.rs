use std::cmp::Reverse;
use std::collections::BinaryHeap;

impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

    fn is_valid(row: i32, col: i32, rows: i32, cols: i32) -> bool {
        return 0 <= row && row < rows && 0 <= col && col < cols;
    }

    fn get_neighbours(row: usize, col: usize, n: usize) -> Vec<(usize, usize)> {
        return Self::DIRECTIONS
            .iter()
            .map(|&(dc, dr)| (row as i32 + dr, col as i32 + dc))
            .filter(|&(row, col)| Self::is_valid(row, col, n as i32, n as i32))
            .map(|(row, col)| (row as usize, col as usize))
            .collect();
    }

    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut res = 0;

        let mut que = BinaryHeap::new();
        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;
        que.push((Reverse(grid[0][0]), 0, 0));

        while !que.is_empty() {
            let (rev_height, row, col) = que.pop().unwrap();
            res = res.max(rev_height.0);

            if row == (n - 1) && col == (n - 1) {
                return res;
            }
            for (nrow, ncol) in Self::get_neighbours(row, col, n) {
                if visited[nrow][ncol] {
                    continue;
                }
                visited[nrow][ncol] = true;
                que.push((Reverse(grid[nrow][ncol]), nrow, ncol));
            }
        }
        unreachable!("we can always reach (n-1, n-1) so we should never reach here");
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid: Vec<Vec<i32>> = [[0, 2], [1, 3]].into_iter().map(|x| Vec::from(x)).collect();
        let res = 3;
        assert_eq!(Solution::swim_in_water(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 16;
        assert_eq!(Solution::swim_in_water(grid), res);
    }
}

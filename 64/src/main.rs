use std::cmp;

struct Solution {}

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let MAX = i32::MAX;
        let row_count = grid.len();
        if row_count == 0 {
            return 0;
        }
        let col_count = grid[0].len();
        if col_count == 0 {
            return 0;
        }
        println!("row_count: {}", row_count);
        println!("col_count: {}", col_count);
        let mut min_sum = vec![vec![MAX; col_count]; row_count];
        min_sum[0][0] = grid[0][0];
        for x in 0..col_count {
            for y in 0..row_count {
                if x > 0 {
                    min_sum[y][x] = cmp::min(min_sum[y][x - 1] + grid[y][x], min_sum[y][x]);
                }
                if y > 0 {
                    min_sum[y][x] = cmp::min(min_sum[y - 1][x] + grid[y][x], min_sum[y][x]);
                }
            }
        }
        for row in &min_sum {
            println!("{:?}", row);
        }
        return min_sum[row_count - 1][col_count - 1];
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([
            Vec::from([1, 3, 1]),
            Vec::from([1, 5, 1]),
            Vec::from([4, 2, 1]),
        ]);
        assert_eq!(Solution::min_path_sum(obstacle_grid), 7);
    }

    #[test]
    fn test_2() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([1, 2, 3]), Vec::from([4, 5, 6])]);
        assert_eq!(Solution::min_path_sum(obstacle_grid), 12);
    }

    #[test]
    fn test_empty() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([]);
        assert_eq!(Solution::min_path_sum(obstacle_grid), 0);
    }

    #[test]
    fn test_empty2() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([])]);
        assert_eq!(Solution::min_path_sum(obstacle_grid), 0);
    }
}

struct Solution {}
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid[0][0] != 0 {
            return 0;
        }
        let row_count = obstacle_grid.len();
        if row_count == 0 {
            return 0;
        }
        let col_count = obstacle_grid[0].len();
        println!("row_count: {}", row_count);
        println!("col_count: {}", col_count);
        let mut path_count = vec![vec![0; col_count]; row_count];
        path_count[0][0] = 1;
        for x in 0..col_count {
            for y in 0..row_count {
                println!("{} {}", x, y);
                if obstacle_grid[y][x] != 0 {
                    continue;
                }
                if x > 0 {
                    path_count[y][x] += path_count[y][x - 1];
                }
                if y > 0 {
                    path_count[y][x] += path_count[y - 1][x];
                }
            }
        }
        for row in &path_count {
            println!("{:?}", row);
        }
        return path_count[row_count - 1][col_count - 1];
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_1() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([
            Vec::from([0, 0, 0]),
            Vec::from([0, 1, 0]),
            Vec::from([0, 0, 0]),
        ]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 2);
    }

    #[test]
    fn test_2() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([0, 0])]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 1);
    }

    #[test]
    fn test_no_paths() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([0, 1]), Vec::from([1, 0])]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }

    #[test]
    fn test_rectangle() {
        let obstacle_grid: Vec<Vec<i32>> =
            Vec::from([Vec::from([0, 1]), Vec::from([0, 0]), Vec::from([0, 0])]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 2);
    }

    #[test]
    fn test_bad_source() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([1, 0]), Vec::from([0, 0])]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }

    #[test]
    fn test_bad_dest() {
        let obstacle_grid: Vec<Vec<i32>> = Vec::from([Vec::from([0, 0]), Vec::from([0, 1])]);
        assert_eq!(Solution::unique_paths_with_obstacles(obstacle_grid), 0);
    }
}

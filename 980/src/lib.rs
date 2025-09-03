impl Solution {
    const BLOCKED: i32 = -1;
    const FREE: i32 = 0;
    const START: i32 = 1;
    const END: i32 = 2;
    const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    fn check(grid: &Vec<Vec<i32>>, x: i32, y: i32) -> bool {
        return (0 <= x) && (x < grid.len() as i32) && (0 <= y) && (y < grid[0].len() as i32);
    }

    fn grid_neighs(grid: &Vec<Vec<i32>>, curr: (usize, usize)) -> Vec<(usize, usize)> {
        let mut res = vec![];
        let (cx, cy) = curr;

        for (dx, dy) in Self::DIRS {
            if Self::check(grid, cx as i32 + dx, cy as i32 + dy) {
                res.push(((cx as i32 + dx) as usize, (cy as i32 + dy) as usize));
            }
        }

        return res;
    }

    fn backtrack(grid: &mut Vec<Vec<i32>>, left_unvisited: i32, curr: (usize, usize)) -> i32 {
        if left_unvisited == 0 {
            if grid[curr.0][curr.1] == Self::END {
                return 1;
            }
            return 0;
        }
        let mut res = 0;

        // marking the current field as blocked to avoid descending there again
        let orig_val = grid[curr.0][curr.1];
        grid[curr.0][curr.1] = Self::BLOCKED;

        for (nx, ny) in Self::grid_neighs(grid, curr) {
            // if nx, ny was already visited - don't descend there
            if grid[nx][ny] == Self::BLOCKED {
                continue;
            }

            res += Self::backtrack(grid, left_unvisited - 1, (nx, ny));
        }

        grid[curr.0][curr.1] = orig_val;
        return res;
    }

    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let (mut left_unvisited, mut start) = (0, (0, 0));

        for (ridx, row) in grid.iter().enumerate() {
            for (cidx, &field) in row.iter().enumerate() {
                if field == Self::START {
                    start = (ridx, cidx);
                } else if field == Self::FREE {
                    left_unvisited += 1;
                } else if field == Self::END {
                    left_unvisited += 1;
                }
            }
        }

        return Self::backtrack(&mut grid, left_unvisited, start);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 2, -1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 2;
        assert_eq!(Solution::unique_paths_iii(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [[1, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 2]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 4;
        assert_eq!(Solution::unique_paths_iii(grid), res);
    }

    #[test]
    fn test3() {
        let grid = [[0, 1], [2, 0]].iter().map(|x| Vec::from(x)).collect();
        let res = 0;
        assert_eq!(Solution::unique_paths_iii(grid), res);
    }
}

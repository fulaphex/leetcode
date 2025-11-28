use std::collections::VecDeque;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; cols]; rows];
        let mut que = VecDeque::new();
        let mut orange_count = 0;
        for (row_idx, row) in grid.iter().enumerate() {
            for (col_idx, &el) in row.iter().enumerate() {
                if el == 2 {
                    que.push_back((0, (row_idx, col_idx)));
                    visited[row_idx][col_idx] = true;
                    orange_count += 1;
                } else if el == 1 {
                    orange_count += 1;
                }
            }
        }

        let mut res = 0;

        const DIRS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
        let get_neighs = |row: usize, col: usize| {
            DIRS.iter()
                .map(|(dx, dy)| (row as i32 + dx, col as i32 + dy))
                .filter(|&(x, y)| (x >= 0) && (x < rows as i32) && (y >= 0) && (y < cols as i32))
                .map(|(x, y)| (x as usize, y as usize))
                .collect::<Vec<_>>()
        };

        while !que.is_empty() {
            let (dist, (row, col)) = que.pop_front().unwrap();
            orange_count -= 1;
            res = res.max(dist);
            for (x, y) in get_neighs(row, col) {
                if grid[x][y] == 1 && !visited[x][y] {
                    visited[x][y] = true;
                    que.push_back((dist + 1, (x, y)));
                }
            }
        }
        if orange_count > 0 { -1 } else { res }
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(vec![vec![2,1,1],vec![1,1,0],vec![0,1,1]], 4)]
    fn test(#[case] grid: Vec<Vec<i32>>, #[case] res: i32) {
        assert_eq!(Solution::oranges_rotting(grid), res)
    }
}

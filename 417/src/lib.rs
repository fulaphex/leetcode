use std::collections::VecDeque;

impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    fn is_valid(row: i32, col: i32, rows: usize, cols: usize) -> bool {
        return 0 <= row && (row as usize) < rows && 0 <= col && (col as usize) < cols;
    }

    fn get_neighs(row: usize, col: usize, rows: usize, cols: usize) -> Vec<(usize, usize)> {
        Self::DIRECTIONS
            .iter()
            .map(|(dr, dc)| (row as i32 + dr, col as i32 + dc))
            .filter(|&(row, col)| Self::is_valid(row, col, rows, cols))
            .map(|(row, col)| (row as usize, col as usize))
            .collect()
    }

    fn get_reachable(
        initial_fields: Vec<(usize, usize)>,
        heights: &Vec<Vec<i32>>,
    ) -> Vec<Vec<bool>> {
        let (rows, cols) = (heights.len(), heights[0].len());
        let mut visited = vec![vec![false; cols]; rows];
        for &(r, c) in &initial_fields {
            visited[r][c] = true;
        }
        let mut que = VecDeque::from(initial_fields);

        while !que.is_empty() {
            let (row, col) = que.pop_back().unwrap();
            let curr_height = heights[row][col];
            for (nr, nc) in Self::get_neighs(row, col, rows, cols) {
                if visited[nr][nc] {
                    continue;
                }
                let neigh_height = heights[nr][nc];
                if curr_height <= neigh_height {
                    visited[nr][nc] = true;
                    que.push_back((nr, nc));
                }
            }
        }
        return visited;
    }

    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (rows, cols) = (heights.len(), heights[0].len());
        let pacific_fields: Vec<_> = (0..rows)
            .map(|row| (row, 0))
            .chain((0..cols).map(|col| (0, col)))
            .collect();
        let atlantic_fields: Vec<_> = (0..rows)
            .map(|row| (row, cols - 1))
            .chain((0..cols).map(|col| (rows - 1, col)))
            .collect();

        let pacific_visited = Self::get_reachable(pacific_fields, &heights);
        let atlantic_visited = Self::get_reachable(atlantic_fields, &heights);

        let mut res = vec![];
        for r in 0..rows {
            for c in 0..cols {
                if pacific_visited[r][c] && atlantic_visited[r][c] {
                    res.push(vec![r as i32, c as i32]);
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
        let heights: Vec<Vec<i32>> = [
            [1, 2, 2, 3, 5],
            [3, 2, 3, 4, 4],
            [2, 4, 5, 3, 1],
            [6, 7, 1, 4, 5],
            [5, 1, 1, 2, 4],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res: Vec<Vec<i32>> = [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let mut out = Solution::pacific_atlantic(heights);
        out.sort();
        assert_eq!(out, res);
    }
}

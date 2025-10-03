use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct BoundaryPoint {
    row: usize,
    col: usize,
    height: i32,
}

impl PartialOrd for BoundaryPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl Ord for BoundaryPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.height.cmp(&other.height).reverse()
    }
}

impl Solution {
    const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

    fn calculate(heights: Vec<Vec<i32>>) -> i32 {
        let (rows, cols) = (heights.len(), heights[0].len());
        let directions = Self::DIRECTIONS;

        let is_valid = |row: i32, col: i32| {
            return 0 <= row && row < rows as i32 && 0 <= col && col < cols as i32;
        };

        let get_neighbours = |row: usize, col: usize| {
            directions
                .iter()
                .map(|(dr, dc)| (row as i32 + dr, col as i32 + dc))
                .filter(|&(r, c)| is_valid(r, c))
                .map(|(r, c)| (r as usize, c as usize))
                .collect::<Vec<_>>()
        };

        let mut res = 0;
        let mut visit_map = vec![vec![false; cols]; rows];
        let mut boundary = BinaryHeap::new();

        for r in 0..rows {
            let c = 0;
            boundary.push(BoundaryPoint {
                row: r,
                col: c,
                height: heights[r][c],
            });
            visit_map[r][c] = true;

            let c = cols - 1;
            boundary.push(BoundaryPoint {
                row: r,
                col: c,
                height: heights[r][c],
            });
            visit_map[r][c] = true;
        }

        for c in 1..cols - 1 {
            let r = 0;
            boundary.push(BoundaryPoint {
                row: r,
                col: c,
                height: heights[r][c],
            });
            visit_map[r][c] = true;

            let r = rows - 1;
            boundary.push(BoundaryPoint {
                row: r,
                col: c,
                height: heights[r][c],
            });
            visit_map[r][c] = true;
        }

        while !boundary.is_empty() {
            let point = boundary.pop().unwrap();
            let (boundary_height, row, col) = (point.height, point.row, point.col);

            for (r, c) in get_neighbours(row, col) {
                // if the node is already visited - skip
                if visit_map[r][c] {
                    continue;
                }

                // add the node to que
                let neigh_height = heights[r][c];
                if neigh_height < boundary_height {
                    res += boundary_height - neigh_height;
                }
                boundary.push(BoundaryPoint {
                    row: r,
                    col: c,
                    height: boundary_height.max(neigh_height),
                });

                visit_map[r][c] = true;
            }
        }
        return res;
    }

    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        Self::calculate(height_map)
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let height_map = [[1, 4, 3, 1, 3, 2], [3, 2, 1, 3, 2, 4], [2, 3, 3, 2, 3, 1]]
            .into_iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 4;
        assert_eq!(Solution::trap_rain_water(height_map), res);
    }

    #[test]
    fn test2() {
        let height_map = [
            [12, 10, 12, 10, 12, 9],
            [10, 6, 3, 2, 8, 7],
            [9, 13, 15, 12, 10, 8],
            [8, 12, 11, 8, 12, 7],
        ]
        .into_iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 13;
        assert_eq!(Solution::trap_rain_water(height_map), res);
    }
}

struct Solution {}

impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = grid.len();
        let (mut diag, mut diag2) = (vec![i32::MAX; n], vec![i32::MIN; n]);

        for row in 0..n {
            for idx in 0..n {
                if idx < (n - row) {
                    diag[idx] = grid[row + idx][idx];
                    diag2[idx] = grid[idx][row + idx];
                } else {
                    diag[idx] = i32::MIN + 10;
                    diag2[idx] = i32::MAX - 10;
                };
            }
            diag.sort_by_key(|&x| -x);
            diag2.sort();

            for (idx, (&x, &y)) in std::iter::zip(&diag, &diag2).take(n - row).enumerate() {
                grid[idx][row + idx] = y;
                grid[row + idx][idx] = x;
            }
        }

        return grid;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[1, 7, 3], [9, 8, 2], [4, 5, 6]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res: Vec<Vec<i32>> = [[8, 2, 3], [9, 6, 7], [4, 5, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        assert_eq!(Solution::sort_matrix(grid), res);
    }
}

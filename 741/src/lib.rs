impl Solution {
    fn prevs(x: usize, y: usize) -> Vec<(usize, usize)> {
        if x > 0 && y > 0 {
            return vec![(x - 1, y), (x, y - 1)];
        } else if x > 0 {
            return vec![(x - 1, y)];
        } else if y > 0 {
            return vec![(x, y - 1)];
        } else {
            return vec![];
        }
    }

    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, columns) = (grid.len(), grid[0].len());
        if (grid[0][0] == -1) || (grid[rows - 1][columns - 1] == -1) {
            return -1;
        }

        let mut dp = vec![vec![vec![i32::MIN; columns]; columns]; rows];
        dp[0][0][0] = grid[0][0];

        for r1 in 0..rows {
            for c1 in 0..columns {
                if ((r1 == 0) && (c1 == 0)) || grid[r1][c1] == -1 {
                    continue;
                }
                for c2 in 0..c1 + 1 {
                    let r2 = r1 + c1 - c2;
                    if r2 >= rows || grid[r2][c2] == -1 {
                        continue;
                    }

                    let mut best = i32::MIN;
                    for (nr1, nc1) in Self::prevs(r1, c1) {
                        for (_nr2, nc2) in Self::prevs(r2, c2) {
                            best = best.max(dp[nr1][nc1][nc2]);
                        }
                    }

                    dp[r1][c1][c2] = grid[r1][c1] + best;
                    if c1 != c2 {
                        dp[r1][c1][c2] += grid[r2][c2];
                    }
                }
            }
        }
        return dp[rows - 1][columns - 1][columns - 1].max(0);
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[0, 1, -1], [1, 0, -1], [1, 1, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 5;
        assert_eq!(Solution::cherry_pickup(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [[1, 1, -1], [1, -1, 1], [-1, 1, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 0;
        assert_eq!(Solution::cherry_pickup(grid), res);
    }
}

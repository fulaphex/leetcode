impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let (rows, columns) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![vec![i32::MIN; columns]; columns]; rows];
        dp[0][0][columns - 1] = grid[0][0] + grid[0][columns - 1];

        for ridx in 1..rows {
            for cidx1 in 0..(columns as i32) {
                for cidx2 in 0..(columns as i32) {
                    let mut best_prev = i32::MIN;
                    for prevcol1 in (cidx1 - 1).max(0)..(cidx1 + 2).min(columns as i32) {
                        for prevcol2 in (cidx2 - 1).max(0)..(cidx2 + 2).min(columns as i32) {
                            best_prev =
                                best_prev.max(dp[ridx - 1][prevcol1 as usize][prevcol2 as usize]);
                        }
                    }
                    if cidx1 == cidx2 {
                        // two robots on the same field - only one can pick berries
                        dp[ridx][cidx1 as usize][cidx2 as usize] =
                            best_prev + grid[ridx][cidx2 as usize];
                    } else {
                        dp[ridx][cidx1 as usize][cidx2 as usize] =
                            best_prev + grid[ridx][cidx1 as usize] + grid[ridx][cidx2 as usize];
                    }
                }
            }
        }
        return *dp[rows - 1]
            .iter()
            .map(|x| x.iter().max().unwrap())
            .max()
            .unwrap();
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let grid = [[3, 1, 1], [2, 5, 1], [1, 5, 5], [2, 1, 1]]
            .iter()
            .map(|x| Vec::from(x))
            .collect();
        let res = 24;
        assert_eq!(Solution::cherry_pickup(grid), res);
    }

    #[test]
    fn test2() {
        let grid = [
            [1, 0, 0, 0, 0, 0, 1],
            [2, 0, 0, 0, 0, 3, 0],
            [2, 0, 9, 0, 0, 0, 0],
            [0, 3, 0, 5, 4, 0, 0],
            [1, 0, 2, 3, 0, 0, 6],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 28;
        assert_eq!(Solution::cherry_pickup(grid), res);
    }

    #[test]
    fn test3() {
        let grid = [
            [0, 8, 7, 10, 9, 10, 0, 9, 6],
            [8, 7, 10, 8, 7, 4, 9, 6, 10],
            [8, 1, 1, 5, 1, 5, 5, 1, 2],
            [9, 4, 10, 8, 8, 1, 9, 5, 0],
            [4, 3, 6, 10, 9, 2, 4, 8, 10],
            [7, 3, 2, 8, 3, 3, 5, 9, 8],
            [1, 2, 6, 5, 6, 2, 0, 10, 0],
        ]
        .iter()
        .map(|x| Vec::from(x))
        .collect();
        let res = 96;
        assert_eq!(Solution::cherry_pickup(grid), res);
    }
}

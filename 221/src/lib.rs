impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (row_count, col_count) = (matrix.len(), matrix[0].len());
        let mut res = 0;
        let mut dp = vec![vec![0; col_count]; row_count];
        for (idx, &x) in matrix[0].iter().enumerate() {
            if x == '1' {
                dp[0][idx] = 1;
                res = 1;
            }
        }
        for (row_idx, row) in matrix.iter().enumerate().skip(1) {
            if matrix[row_idx][0] == '1' {
                dp[row_idx][0] = 1;
                res = res.max(1);
            };
            for (col_idx, &val) in row.iter().enumerate().skip(1) {
                if val == '1' {
                    dp[row_idx][col_idx] = dp[row_idx - 1][col_idx]
                        .min(dp[row_idx][col_idx - 1])
                        .min(dp[row_idx - 1][col_idx - 1])
                        + 1;
                    res = res.max(dp[row_idx][col_idx]);
                }
            }
        }
        return res * res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let matrix = [
            ["1", "0", "1", "0", "0"],
            ["1", "0", "1", "1", "1"],
            ["1", "1", "1", "1", "1"],
            ["1", "0", "0", "1", "0"],
        ]
        .into_iter()
        .map(|x| Vec::from_iter(x.into_iter().map(|y| y.chars().next().unwrap())))
        .collect();
        let res = 4;
        assert_eq!(Solution::maximal_square(matrix), res);
    }
}

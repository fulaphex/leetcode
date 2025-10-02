impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let col_count = matrix[0].len();
        let mut res = 0;
        let mut dp = vec![vec![0; col_count]; 2];
        for (idx, &x) in matrix[0].iter().enumerate() {
            if x == '1' {
                dp[0][idx] = 1;
                res = 1;
            }
        }
        for (row_idx, row) in matrix.iter().enumerate().skip(1) {
            dp[row_idx % 2][0] = if matrix[row_idx][0] == '1' { 1 } else { 0 };
            res = res.max(dp[row_idx % 2][0]);
            for (col_idx, &val) in row.iter().enumerate().skip(1) {
                dp[row_idx % 2][col_idx] = if val == '1' {
                    dp[(row_idx - 1) % 2][col_idx]
                        .min(dp[row_idx % 2][col_idx - 1])
                        .min(dp[(row_idx - 1) % 2][col_idx - 1])
                        + 1
                } else {
                    0
                };
                res = res.max(dp[row_idx % 2][col_idx]);
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

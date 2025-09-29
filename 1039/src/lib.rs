impl Solution {
    pub fn min_score_triangulation(values: Vec<i32>) -> i32 {
        let n = values.len();
        let mut dp = vec![vec![i32::MAX; n]; n];
        for x in 0..n - 1 {
            dp[x][x + 1] = 0;
        }
        for window_size in 1..n {
            for x in 0..n - window_size {
                let y = x + window_size;
                for z in x + 1..y {
                    println!("{:?}", (x, z, y));
                    dp[x][y] =
                        dp[x][y].min(values[x] * values[y] * values[z] + dp[x][z] + dp[z][y]);
                }
            }
        }
        return dp[0][n - 1];
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let values = vec![3, 7, 4, 5];
        let res = 144;
        assert_eq!(Solution::min_score_triangulation(values), res);
    }
}

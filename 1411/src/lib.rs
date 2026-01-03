impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let n = n as usize;
        const MOD: i64 = 1_000_000_007;
        let triples = {
            let mut arr = vec![];
            for i in 0..3 {
                for j in 0..3 {
                    if i == j {
                        continue;
                    }
                    for k in 0..3 {
                        if k == j {
                            continue;
                        }
                        arr.push((i, j, k));
                    }
                }
            }
            arr
        };
        let mut dp = vec![vec![vec![vec![0; 3]; 3]; 3]; n + 1];
        for &(i, j, k) in &triples {
            dp[0][i][j][k] = 1;
        }
        for x in 1..n {
            for &(i, j, k) in &triples {
                for &(i2, j2, k2) in &triples {
                    if i == i2 || j == j2 || k == k2 {
                        continue;
                    }
                    dp[x][i2][j2][k2] += dp[x - 1][i][j][k];
                    dp[x][i2][j2][k2] %= MOD;
                }
            }
        }
        let mut res = 0;
        for &(i, j, k) in &triples {
            res += dp[n - 1][i][j][k];
            res %= MOD;
        }
        res as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let n = 1;
        let res = 12;
        assert_eq!(Solution::num_of_ways(n), res);
    }

    #[test]
    fn test2() {
        let n = 2;
        let res = 54;
        assert_eq!(Solution::num_of_ways(n), res);
    }

    #[test]
    fn test3() {
        let n = 5_000;
        let res = 30_228_214;
        assert_eq!(Solution::num_of_ways(n), res);
    }
}

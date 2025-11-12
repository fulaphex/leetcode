impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        const DEF_VAL: i32 = i32::MIN;
        let (m, n) = (m as usize, n as usize);
        let mut dp = vec![vec![DEF_VAL; n + 1]; m + 1];
        dp[0][0] = 0;

        let mut res = 0;

        let mut nonzero_idxs = vec![(0, 0)];
        let mut new_vals = Vec::with_capacity(n * m);

        for s in strs {
            let x = s.chars().filter(|&c| c == '0').count();
            let y = s.len() - x;

            for &(i, j) in &nonzero_idxs {
                if i + x > m || j + y > n {
                    continue;
                }
                new_vals.push((i + x, j + y, dp[i][j] + 1));
            }
            for &(i, j, val) in &new_vals {
                if dp[i][j] == DEF_VAL {
                    nonzero_idxs.push((i, j));
                }
                dp[i][j] = dp[i][j].max(val);
                res = res.max(dp[i][j]);
            }
            new_vals.clear();
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let strs: Vec<String> = ["10", "0001", "111001", "1", "0"]
            .iter()
            .map(|&x| String::from(x))
            .collect();
        let (m, n) = (5, 3);
        let res = 4;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }

    #[test]
    fn test2() {
        let strs: Vec<String> = ["10", "1", "0"].iter().map(|&x| String::from(x)).collect();
        let (m, n) = (1, 1);
        let res = 2;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }

    #[test]
    fn test3() {
        let strs: Vec<String> = [
            "11", "11", "0", "0", "10", "1", "1", "0", "11", "1", "0", "111", "11111000", "0",
            "11", "000", "1", "1", "0", "00", "1", "101", "001", "000", "0", "00", "0011", "0",
            "10000",
        ]
        .iter()
        .map(|&x| String::from(x))
        .collect();
        let (m, n) = (90, 66);
        let res = 29;

        assert_eq!(Solution::find_max_form(strs, m, n), res);
    }
}

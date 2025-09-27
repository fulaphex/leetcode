struct Solution {}

impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        let un = n as usize;
        let mut dp = vec![0; un + 2];
        let mut i: i32 = 1;
        dp[0] = 1;

        loop {
            let ipow = i.pow(x as u32) as usize;
            if ipow > un {
                break;
            }
            for idx in (0..un - ipow + 1).rev() {
                dp[idx + ipow] += dp[idx];
                dp[idx + ipow] %= 1_000_000_007;
            }
            i += 1;
        }

        return dp[un];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let (n, x) = (10, 2);
        let res = 1;
        assert_eq!(Solution::number_of_ways(n, x), res);
    }

    #[test]
    fn test2() {
        let (n, x) = (4, 1);
        let res = 2;
        assert_eq!(Solution::number_of_ways(n, x), res);
    }
}

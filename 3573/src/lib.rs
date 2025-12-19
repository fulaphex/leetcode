impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut dp = vec![vec![0; prices.len() + 1]; k + 1];

        for idx in 1..prices.len() {
            for buy_day in 0..idx {
                for transactions in 1..=k {
                    dp[transactions][idx + 1] = dp[transactions][idx + 1].max(
                        prices[idx] as i64 - prices[buy_day] as i64 + dp[transactions - 1][buy_day],
                    );
                    dp[transactions][idx + 1] = dp[transactions][idx + 1].max(
                        prices[buy_day] as i64 - prices[idx] as i64 + dp[transactions - 1][buy_day],
                    );
                }
            }
        }

        dp[k][prices.len()]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![1, 7, 9, 8, 2];
        let k = 2;
        let res = 14;

        assert_eq!(Solution::maximum_profit(prices, k), res);
    }
}

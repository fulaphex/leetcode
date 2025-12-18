impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;

        let mut prefsum = vec![0; prices.len() + 1];
        let mut prefsum_strategy = vec![0; prices.len() + 1];

        for idx in 0..prices.len() {
            prefsum[idx + 1] = prefsum[idx] + prices[idx] as i64;
            prefsum_strategy[idx + 1] =
                prefsum_strategy[idx] + (prices[idx] as i64 * strategy[idx] as i64);
        }

        let mut res = prefsum_strategy[prices.len()];
        for i in 0..=(prices.len() - k) {
            res = res.max(
                prefsum_strategy[i] + prefsum[i + k] - prefsum[i + k / 2]
                    + prefsum_strategy[prices.len()]
                    - prefsum_strategy[i + k],
            )
        }

        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![4, 2, 8];
        let strategy = vec![-1, 0, 1];
        let k = 2;
        let res = 10;

        assert_eq!(Solution::max_profit(prices, strategy, k), res);
    }
}

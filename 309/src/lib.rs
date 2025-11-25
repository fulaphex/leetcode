impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        const UNVISITED: i32 = i32::MIN;
        // dp[y][x] = best balance with x shares, before yth price, last action being z
        let mut dp = vec![vec![UNVISITED; prices.len() + 1]; 2];

        dp[0][0] = 0;
        // setting the first buy
        dp[1][1] = -prices[0];

        for idx in 1..=prices.len() {
            // do nothing
            {
                for share_count in [0, 1] {
                    dp[share_count][idx] = dp[share_count][idx].max(dp[share_count][idx - 1]);
                }
            }

            // buy one share
            {
                let share_count = 1;
                // looking back two days ago - to have the one day wait before buying
                if idx > 1 && dp[share_count - 1][idx - 2] != UNVISITED {
                    dp[share_count][idx] =
                        dp[share_count][idx].max(dp[share_count - 1][idx - 2] - prices[idx - 1]);
                }
            }

            // sell one share
            {
                let share_count = 0;
                if dp[share_count + 1][idx - 1] != UNVISITED {
                    dp[share_count][idx] =
                        dp[share_count][idx].max(dp[share_count + 1][idx - 1] + prices[idx - 1])
                }
            }
        }

        dp[0][prices.len()]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let prices = vec![1, 2, 3, 0, 2];
        let res = 3;
        assert_eq!(Solution::max_profit(prices), res);
    }
}

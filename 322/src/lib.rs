impl Solution {
    pub fn coin_change(mut coins: Vec<i32>, amount: i32) -> i32 {
        coins.sort_unstable();
        let max = i32::MAX;
        let mut dp = vec![max; (amount + 2) as usize];
        dp[0] = 0;

        for i in 0..(amount as usize) {
            if dp[i] == max {
                continue;
            }
            for &c in &coins {
                let new_idx = i + c as usize;
                if new_idx < dp.len() {
                    dp[new_idx] = dp[new_idx].min(dp[i] + 1);
                } else {
                    break;
                }
            }
        }

        return if dp[amount as usize] != max {
            dp[amount as usize]
        } else {
            -1
        };
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let coins = vec![1, 2, 5];
        let amount = 11;
        let res = 3;
        assert_eq!(Solution::coin_change(coins, amount), res);
    }

    #[test]
    fn test2() {
        let coins = vec![2];
        let amount = 3;
        let res = -1;
        assert_eq!(Solution::coin_change(coins, amount), res);
    }

    #[test]
    fn test3() {
        let coins = vec![1];
        let amount = 0;
        let res = 0;
        assert_eq!(Solution::coin_change(coins, amount), res);
    }

    #[test]
    fn test4() {
        let coins = vec![456, 117, 5, 145];
        let amount = 1459;
        let res = 23;
        assert_eq!(Solution::coin_change(coins, amount), res);
    }
}

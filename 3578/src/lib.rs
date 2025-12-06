use std::collections::BTreeMap;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut dp = vec![0; nums.len() + 2];
        let mut prefsum = vec![-1; nums.len() + 2];
        dp[0] = 1;
        prefsum[0] = dp[0];
        let mut cnt = BTreeMap::new();
        let mut window_start = 0;

        for i in 0..nums.len() {
            *cnt.entry(&nums[i]).or_default() += 1;
            while let Some((&&minval, _)) = cnt.first_key_value()
                && let Some((&&maxval, _)) = cnt.last_key_value()
                && (maxval - minval) > k
            {
                let entry: &mut usize = cnt.entry(&nums[window_start]).or_default();
                *entry -= 1;

                if entry == &0 {
                    cnt.remove(&nums[window_start]);
                }
                window_start += 1;
            }

            dp[i + 1] = prefsum[i];
            if window_start > 0 {
                dp[i + 1] += MOD - prefsum[window_start - 1];
            }
            dp[i + 1] %= MOD;
            prefsum[i + 1] = (prefsum[i] + dp[i + 1]) % MOD;
        }
        dp[nums.len()]
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![9, 4, 1, 3, 7];
        let k = 4;
        let res = 6;
        assert_eq!(Solution::count_partitions(nums, k), res);
    }
}

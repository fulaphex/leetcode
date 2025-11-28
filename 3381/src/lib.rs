impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut dp = vec![0; k];
        let mut curr_sum = nums.iter().take(k).fold(0_i64, |acc, &x| acc + x as i64);
        dp[k - 1] = curr_sum;
        let mut res = curr_sum;

        for i in k..nums.len() {
            curr_sum += (nums[i] - nums[i - k]) as i64;
            dp[i % k] = curr_sum + dp[i % k].max(0);
            res = res.max(dp[i % k]);
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
        let nums = vec![1, 2];
        let k = 1;
        let res = 3;

        assert_eq!(Solution::max_subarray_sum(nums, k), res);
    }

    #[test]
    fn test2() {
        let nums = vec![-1, -2, -3, -4, -5];
        let k = 4;
        let res = -10;

        assert_eq!(Solution::max_subarray_sum(nums, k), res);
    }

    #[test]
    fn test3() {
        let nums = vec![-5, 1, 2, -3, 4];
        let k = 2;
        let res = 4;

        assert_eq!(Solution::max_subarray_sum(nums, k), res);
    }
}

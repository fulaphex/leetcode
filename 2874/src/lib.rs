impl Solution {
    pub fn maximum_triplet_value(nums: Vec<i32>) -> i64 {
        let mut res = 0;
        let n = nums.len();

        let mut suff_max = vec![-1; n];

        suff_max[n - 1] = nums[n - 1];
        for (idx, &x) in nums.iter().enumerate().rev().skip(1) {
            suff_max[idx] = suff_max[idx + 1].max(x);
        }

        let mut pref_max = nums[0];

        for j in 1..n - 1 {
            res = res.max((pref_max - nums[j]) as i64 * suff_max[j + 1] as i64);
            pref_max = pref_max.max(nums[j]);
        }
        return res;
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = [12, 6, 1, 2, 7].into_iter().collect();
        let res = 77;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }

    #[test]
    fn test2() {
        let nums = [1, 10, 3, 4, 19].into_iter().collect();
        let res = 133;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }

    #[test]
    fn test3() {
        let nums = [1, 2, 3].into_iter().collect();
        let res = 0;
        assert_eq!(Solution::maximum_triplet_value(nums), res);
    }
}

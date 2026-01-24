impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut res = i32::MIN;
        for i in 0..(nums.len() / 2) {
            res = res.max(nums[i] + nums[nums.len() - 1 - i]);
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
        let nums = vec![3, 5, 2, 3];
        let res = 7;

        assert_eq!(Solution::min_pair_sum(nums), res);
    }
}

use std::{cmp, collections};

struct Solution {}
impl Solution {
    pub fn maximum_unique_subarray(nums: Vec<i32>) -> i32 {
        let mut window_nums = collections::HashSet::<i32>::new();
        let mut window_start = 0;
        let mut res = -1;
        let mut window_sum = 0;

        for &num in &nums {
            while window_nums.contains(&num) {
                window_sum -= nums[window_start];
                window_nums.remove(&nums[window_start]);
                window_start += 1;
            }
            window_sum += num;
            window_nums.insert(num);
            res = cmp::max(res, window_sum);
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![4, 2, 4, 5, 6];
        let res = 17;
        assert_eq!(Solution::maximum_unique_subarray(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![5, 2, 1, 2, 5, 2, 1, 2, 5];
        let res = 8;
        assert_eq!(Solution::maximum_unique_subarray(nums), res);
    }
}

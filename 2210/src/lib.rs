struct Solution {}

impl Solution {
    pub fn count_hill_valley(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        let mut prev = nums[0];
        for window in nums.windows(3) {
            if window[0] != window[1] {
                prev = window[0];
            }
            if (prev > window[1]) && (window[1] < window[2]) {
                res += 1;
            } else if (prev < window[1]) && (window[1] > window[2]) {
                res += 1;
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![2, 4, 1, 1, 6, 5];
        let res = 3;
        assert_eq!(Solution::count_hill_valley(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4];
        let res = 0;
        assert_eq!(Solution::count_hill_valley(nums), res);
    }
}

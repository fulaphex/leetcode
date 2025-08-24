struct Solution {}

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut res, mut curr) = (0, 0);
        for &n in nums.iter().chain([&0]) {
            if n == 1 {
                curr += 1;
            } else {
                res = res.max(curr);
                curr = 0;
            }
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 0, 1, 1, 1];
        let res = 3;
        assert_eq!(Solution::find_max_consecutive_ones(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 0, 1, 1, 0, 1];
        let res = 2;
        assert_eq!(Solution::find_max_consecutive_ones(nums), res);
    }
}

struct Solution {}

impl Solution {
    pub fn longest_subarray(mut nums: Vec<i32>) -> i32 {
        let (mut res, mut curr, mut prev) = (0, 0, 0);
        for &x in nums.iter().chain([&0]) {
            if x == 1 {
                curr += 1;
            }
            if x == 0 {
                res = res.max(curr + prev);
                (prev, curr) = (curr, 0);
            }
        }
        return res.min(nums.len() - 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 1, 0, 1];
        let res = 3;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 1, 1, 0, 1, 1, 0, 1];
        let res = 5;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 1, 1];
        let res = 2;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test4() {
        let nums = vec![0; 3];
        let res = 0;

        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test5() {
        let nums = vec![
            0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1,
            1, 0, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 0, 1,
            1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
            1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1,
            0, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 0, 1, 1, 1, 1, 0,
            1, 1, 1, 1, 1, 0, 0, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1, 1, 1, 1,
        ];
        let res = 60;

        assert_eq!(Solution::longest_subarray(nums), res);
    }
}

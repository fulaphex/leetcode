struct Solution {}

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max = nums.iter().fold(0, |acc, &x| acc.max(x));
        let (mut res, mut count) = (0, 0);
        for c in nums {
            if c == max {
                count += 1;
            } else {
                count = 0;
            }
            res = res.max(count);
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 3, 2, 2];
        let res = 2;
        assert_eq!(Solution::longest_subarray(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4];
        let res = 1;
        assert_eq!(Solution::longest_subarray(nums), res);
    }
}

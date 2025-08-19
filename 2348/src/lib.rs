struct Solution {}

impl Solution {
    pub fn zero_filled_subarray(nums: Vec<i32>) -> i64 {
        let (mut count, mut res) = (0, 0);
        for n in nums.iter().chain([&1]) {
            if *n == 0 {
                count += 1;
            } else {
                count = 0;
            }
            res += count;
        }
        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 3, 0, 0, 2, 0, 0, 4];
        let res = 6;
        assert_eq!(Solution::zero_filled_subarray(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 0, 0, 2, 0, 0];
        let res = 9;
        assert_eq!(Solution::zero_filled_subarray(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![2, 10, 2019];
        let res = 0;
        assert_eq!(Solution::zero_filled_subarray(nums), res);
    }
}

use core::cmp;

struct Solution {}

impl Solution {
    fn _same(nums: &Vec<i32>) -> i32 {
        let mut c = [0, 0];
        for n in nums {
            c[(n % 2) as usize] += 1;
        }
        return *c.iter().max().unwrap();
    }
    fn _diff(nums: &Vec<i32>) -> i32 {
        let mut x = nums[0];
        let mut res = 1;
        for &n in nums.iter().skip(1) {
            if (n % 2) != (x % 2) {
                res += 1;
            }
            x = n;
        }
        return res;
    }
    pub fn maximum_length(nums: Vec<i32>) -> i32 {
        return cmp::max(Self::_same(&nums), Self::_diff(&nums));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4];
        let res = 4;
        assert_eq!(Solution::maximum_length(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 1, 1, 2, 1, 2];
        let res = 6;
        assert_eq!(Solution::maximum_length(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 3];
        let res = 2;
        assert_eq!(Solution::maximum_length(nums), res);
    }
}

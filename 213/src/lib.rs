impl Solution {
    fn solve(nums: &[i32]) -> i32 {
        let (mut dp0, mut dp1) = (nums[0], nums[0].max(nums[1]));
        for x in nums.iter().skip(2) {
            (dp0, dp1) = (dp1, (dp0 + x).max(dp1));
        }
        return dp1;
    }
    pub fn rob(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 3 {
            return *nums.iter().max().unwrap();
        }
        return Self::solve(&nums.as_slice()[1..]).max(Self::solve(&nums.as_slice()[..n - 1]));
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 3, 2];
        let res = 3;
        assert_eq!(Solution::rob(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 1];
        let res = 4;
        assert_eq!(Solution::rob(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, 3];
        let res = 3;
        assert_eq!(Solution::rob(nums), res);
    }
}

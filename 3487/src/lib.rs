use std::collections;

struct Solution {}

impl Solution {
    pub fn max_sum(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut set = collections::HashSet::<i32>::new();
        for num in nums {
            max = max.max(num);
            if num >= 0 {
                set.insert(num);
            }
        }
        return if max < 0 { max } else { set.iter().sum() };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let nums = vec![1, 2, 3, 4, 5];
        assert_eq!(Solution::max_sum(nums), 15);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 1, 0, 1, 1];
        assert_eq!(Solution::max_sum(nums), 1);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 2, -1, -2, 1, 0, -1];
        assert_eq!(Solution::max_sum(nums), 3);
    }

    #[test]
    fn test4() {
        let nums = vec![-1];
        assert_eq!(Solution::max_sum(nums), -1);
    }
}

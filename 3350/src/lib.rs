impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let (mut curr, mut prev) = (1, 0);
        let mut res = 1;

        for idx in 0..nums.len() - 1 {
            if nums[idx] < nums[idx + 1] {
                curr += 1;
            } else {
                (curr, prev) = (1, curr);
            }
            res = res.max(curr / 2).max(curr.min(prev));
        }
        res
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![2, 5, 7, 8, 9, 2, 3, 4, 3, 1];
        let res = 3;
        assert_eq!(Solution::max_increasing_subarrays(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![1, 2, 3, 4, 4, 4, 4, 5, 6, 7];
        let res = 2;
        assert_eq!(Solution::max_increasing_subarrays(nums), res);
    }

    #[test]
    fn test3() {
        let nums = vec![-15, 19];
        let res = 1;
        assert_eq!(Solution::max_increasing_subarrays(nums), res);
    }
}

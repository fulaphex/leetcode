impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        nums.into_iter().sum::<i32>() % k
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![3, 9, 7];
        let k = 5;
        let res = 4;
        assert_eq!(Solution::min_operations(nums, k), res);
    }
}

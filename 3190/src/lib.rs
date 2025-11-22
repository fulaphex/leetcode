impl Solution {
    pub fn minimum_operations(nums: Vec<i32>) -> i32 {
        nums.iter().filter(|&x| x % 3 != 0).count() as i32
    }
}

struct Solution {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3, 4];
        let res = 3;
        assert_eq!(Solution::minimum_operations(nums), res);
    }

    #[test]
    fn test2() {
        let nums = vec![3, 6, 9];
        let res = 0;
        assert_eq!(Solution::minimum_operations(nums), res);
    }
}
